use std::mem::size_of;

use crate::baseline;
use crate::baseline::fct::{BailoutInfo, JitBaselineFct, JitDescriptor, JitFct};
use crate::baseline::map::CodeDescriptor;
use crate::cpu::{
    Mem, FREG_PARAMS, REG_FP, REG_PARAMS, REG_RESULT, REG_SP, REG_THREAD, REG_TMP1, REG_TMP_CALLEE,
};
use crate::exception::DoraToNativeInfo;
use crate::gc::Address;
use crate::masm::MacroAssembler;
use crate::mem;
use crate::object::Obj;
use crate::threads::ThreadLocalData;
use crate::ty::{MachineMode, TypeList};
use crate::vm::FctId;
use crate::vm::{get_vm, VM};

// This code generates the compiler thunk, there should only be one instance
// of this function be used in Dora. It is necessary for lazy compilation, where
// functions are only compiled on their first invocation. The compiler can use
// the address of this thunk for invocations of functions that have not been compiled
// yet. The thunk compiles the function and patches the call site to invoke the
// now-compiled function directly on the next invocation. In the end the function is
// executed.

pub fn generate<'a, 'ast: 'a>(vm: &'a VM<'ast>) -> Address {
    let ngen = DoraCompileGen {
        vm,
        masm: MacroAssembler::new(),
        dbg: vm.args.flag_emit_debug_compile,
    };

    let jit_fct = ngen.generate();
    let addr = jit_fct.fct_ptr();
    vm.insert_code_map(
        jit_fct.ptr_start(),
        jit_fct.ptr_end(),
        CodeDescriptor::CompilerThunk,
    );
    vm.jit_fcts.push(JitFct::Base(jit_fct));

    addr
}

struct DoraCompileGen<'a, 'ast: 'a> {
    vm: &'a VM<'ast>,
    masm: MacroAssembler,
    dbg: bool,
}

impl<'a, 'ast> DoraCompileGen<'a, 'ast>
where
    'ast: 'a,
{
    pub fn generate(mut self) -> JitBaselineFct {
        let offset_dtn = 0;
        let offset_params = offset_dtn + size_of::<DoraToNativeInfo>() as i32;
        let offset_tmp =
            offset_params + (FREG_PARAMS.len() + REG_PARAMS.len()) as i32 * mem::ptr_width();
        let offset_thread = offset_tmp + mem::ptr_width();
        let framesize = mem::align_i32(offset_thread + mem::ptr_width(), 16) as i32;

        if self.dbg {
            self.masm.debug();
        }

        // the return address is the call-site we need to patch
        self.masm.prolog_size(framesize);

        // store params passed in registers on the stack
        self.store_params(offset_params);

        // prepare the native call
        self.masm.load_mem(
            MachineMode::Ptr,
            REG_TMP_CALLEE.into(),
            Mem::Base(REG_THREAD, ThreadLocalData::dtn_offset()),
        );

        self.masm.store_mem(
            MachineMode::Ptr,
            Mem::Base(REG_SP, offset_dtn + DoraToNativeInfo::last_offset()),
            REG_TMP_CALLEE.into(),
        );

        self.masm.store_mem(
            MachineMode::Ptr,
            Mem::Base(REG_SP, offset_dtn + DoraToNativeInfo::fp_offset()),
            REG_FP.into(),
        );

        self.masm.copy_pc(REG_TMP1);

        self.masm.store_mem(
            MachineMode::Ptr,
            Mem::Base(REG_SP, offset_dtn + DoraToNativeInfo::pc_offset()),
            REG_TMP1.into(),
        );

        self.masm.store_mem(
            MachineMode::Ptr,
            Mem::Base(REG_THREAD, ThreadLocalData::dtn_offset()),
            REG_SP.into(),
        );

        // invoke the compiler for the call site
        self.masm.load_mem(
            MachineMode::Ptr,
            REG_PARAMS[0].into(),
            Mem::Base(REG_FP, mem::ptr_width()),
        );
        self.masm.load_mem(
            MachineMode::Ptr,
            REG_PARAMS[1].into(),
            Mem::Base(REG_SP, offset_params),
        );
        self.masm.raw_call(compile_request as *const u8);

        self.masm.store_mem(
            MachineMode::Ptr,
            Mem::Base(REG_THREAD, ThreadLocalData::dtn_offset()),
            REG_TMP_CALLEE.into(),
        );

        // restore argument registers from the stack
        self.load_params(offset_params);

        // remove the stack frame
        self.masm.epilog_without_return();

        // jump to compiled function
        self.masm.jump_reg(REG_RESULT);

        self.masm
            .jit(self.vm, framesize, JitDescriptor::CompilerThunk, false)
    }

    fn store_params(&mut self, mut offset: i32) {
        for reg in &REG_PARAMS {
            self.masm
                .store_mem(MachineMode::Ptr, Mem::Base(REG_SP, offset), (*reg).into());
            offset += mem::ptr_width();
        }

        for reg in &FREG_PARAMS {
            self.masm.store_mem(
                MachineMode::Float64,
                Mem::Base(REG_SP, offset),
                (*reg).into(),
            );
            offset += mem::ptr_width();
        }
    }

    fn load_params(&mut self, mut offset: i32) {
        for reg in &REG_PARAMS {
            self.masm
                .load_mem(MachineMode::Ptr, (*reg).into(), Mem::Base(REG_SP, offset));
            offset += mem::ptr_width();
        }

        for reg in &FREG_PARAMS {
            self.masm.load_mem(
                MachineMode::Float64,
                (*reg).into(),
                Mem::Base(REG_SP, offset),
            );
            offset += mem::ptr_width();
        }
    }
}

fn compile_request(ra: usize, receiver: Address) -> Address {
    let vm = get_vm();

    let bailout = {
        let data = {
            let code_map = vm.code_map.lock();
            code_map.get(ra.into()).expect("return address not found")
        };

        let fct_id = match data {
            CodeDescriptor::DoraFct(fct_id) => fct_id,
            _ => panic!("expected function for code"),
        };

        let jit_fct = vm.jit_fcts.idx(fct_id);

        let offset = ra - jit_fct.fct_ptr().to_usize();
        let jit_fct = jit_fct.to_base().expect("baseline expected");
        jit_fct
            .bailouts
            .get(offset as i32)
            .expect("bailout info not found")
            .clone()
    };

    match bailout {
        BailoutInfo::Compile(fct_id, disp, ref cls_tps, ref fct_tps) => {
            patch_fct_call(vm, ra, fct_id, cls_tps, fct_tps, disp)
        }

        BailoutInfo::VirtCompile(vtable_index, ref fct_tps) => {
            patch_vtable_call(vm, receiver, vtable_index, fct_tps)
        }
    }
}

fn patch_vtable_call(vm: &VM, receiver: Address, vtable_index: u32, fct_tps: &TypeList) -> Address {
    let obj = unsafe { &mut *receiver.to_mut_ptr::<Obj>() };
    let vtable = obj.header().vtbl();
    let cls_id = vtable.class().cls_id.expect("no corresponding class");
    let cls = vm.classes.idx(cls_id);
    let cls = cls.read();

    let empty = TypeList::empty();
    let fct_id = cls.virtual_fcts[vtable_index as usize];
    let fct_ptr = baseline::generate(vm, fct_id, &empty, fct_tps);

    let methodtable = vtable.table_mut();
    methodtable[vtable_index as usize] = fct_ptr.to_usize();

    fct_ptr
}

fn patch_fct_call(
    vm: &VM,
    ra: usize,
    fct_id: FctId,
    cls_tps: &TypeList,
    fct_tps: &TypeList,
    disp: i32,
) -> Address {
    let fct_ptr = baseline::generate(vm, fct_id, cls_tps, fct_tps);
    let fct_addr: *mut usize = (ra as isize - disp as isize) as *mut _;

    // update function pointer in data segment
    unsafe {
        *fct_addr = fct_ptr.to_usize();
    }

    fct_ptr
}