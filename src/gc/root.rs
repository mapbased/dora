use std::convert::From;

use baseline::map::CodeData;
use ctxt::{SemContext, FctKind};
use object::Obj;
use stacktrace::DoraToNativeInfo;

pub fn get_rootset(ctxt: &SemContext) -> Vec<IndirectObj> {
    let mut rootset = Vec::new();

    determine_rootset_from_stack(&mut rootset, ctxt);
    determine_rootset_from_globals(&mut rootset, ctxt);

    rootset
}

fn determine_rootset_from_globals(rootset: &mut Vec<IndirectObj>, ctxt: &SemContext) {
    for glob in ctxt.globals.iter() {
        let glob = glob.borrow();

        if !glob.ty.reference_type() {
            continue;
        }

        rootset.push((glob.address_value as usize).into());
    }
}

fn determine_rootset_from_stack(rootset: &mut Vec<IndirectObj>, ctxt: &SemContext) {
    assert!(!ctxt.sfi.borrow().is_null());

    let mut sfi = *ctxt.sfi.borrow();

    while !sfi.is_null() {
        sfi = from_dora_to_native_info(rootset, ctxt, sfi);
    }
}

fn from_dora_to_native_info(rootset: &mut Vec<IndirectObj>,
                            ctxt: &SemContext,
                            sfi: *const DoraToNativeInfo)
                            -> *const DoraToNativeInfo {
    let sfi = unsafe { &*sfi };

    let mut pc: usize = sfi.ra;
    let mut fp: usize = sfi.fp;

    while fp != 0 {
        if !determine_rootset(rootset, ctxt, fp, pc) {
            break;
        }

        pc = unsafe { *((fp + 8) as *const usize) };
        fp = unsafe { *(fp as *const usize) };
    }

    sfi.last
}

fn determine_rootset(rootset: &mut Vec<IndirectObj>,
                     ctxt: &SemContext,
                     fp: usize,
                     pc: usize)
                     -> bool {
    let code_map = ctxt.code_map.lock().unwrap();
    let data = code_map.get(pc as *const u8);

    if data.is_none() {
        return false;
    }

    if let CodeData::Fct(fct_id) = data.unwrap() {
        let fct = ctxt.fcts[fct_id].borrow();

        if let FctKind::Source(ref src) = fct.kind {
            let src = src.borrow();
            let jit_fct = src.jit_fct.read().unwrap();
            let jit_fct = jit_fct.as_ref().expect("no jit information");
            let offset = pc - (jit_fct.fct_ptr() as usize);
            let gcpoint = jit_fct
                .gcpoint_for_offset(offset as i32)
                .expect("no gcpoint");

            for &offset in &gcpoint.offsets {
                let addr = (fp as isize + offset as isize) as usize;
                rootset.push(addr.into());
            }
        } else {
            panic!("should be FctKind::Source");
        }

        true
    } else {
        false
    }
}

#[derive(Copy, Clone)]
pub struct IndirectObj(*mut *mut Obj);

impl IndirectObj {
    pub fn get(self) -> *mut Obj {
        unsafe { *self.0 }
    }

    pub fn set(self, obj: *mut Obj) {
        unsafe {
            *self.0 = obj;
        }
    }
}

impl From<usize> for IndirectObj {
    fn from(ptr: usize) -> IndirectObj {
        IndirectObj(ptr as *mut *mut Obj)
    }
}
