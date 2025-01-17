use std::mem;
use std::ptr;

use crate::boots::data::InstructionSet;
use crate::boots::serializer::allocate_encoded_compilation_info;
use crate::cannon::CompilationFlags;
use crate::compiler::codegen::CompilationData;
use crate::gc::Address;
use crate::handle::create_handle;
use crate::masm::CodeDescriptor;
use crate::object::{Ref, UInt8Array};
use crate::threads::current_thread;
use crate::vm::VM;

mod data;
mod serializer;

pub fn compile(
    vm: &VM,
    compilation_data: CompilationData,
    _flags: CompilationFlags,
) -> CodeDescriptor {
    let compile_fct_id = vm.known.boots_compile_fct_id();
    let compile_address = vm.ensure_compiled(compile_fct_id);

    let encoded_compilation_info = create_handle(allocate_encoded_compilation_info(
        vm,
        compilation_data.bytecode_fct,
        &compilation_data.type_params,
        get_architecture(),
    ));

    let tld_address = current_thread().tld_address();

    let dora_stub_address = vm.stubs.dora_entry();
    let compile_fct_ptr: extern "C" fn(Address, Address, Address) -> Ref<UInt8Array> =
        unsafe { mem::transmute(dora_stub_address) };

    let machine_code = create_handle(compile_fct_ptr(
        tld_address,
        compile_address,
        encoded_compilation_info.direct_ptr(),
    ));
    let mut code = vec![0; machine_code.len()];

    unsafe {
        ptr::copy_nonoverlapping(
            machine_code.data() as *mut u8,
            code.as_mut_ptr(),
            machine_code.len(),
        );
    }

    CodeDescriptor::from_buffer(code)
}

fn get_architecture() -> InstructionSet {
    if cfg!(target_arch = "x86_64") {
        InstructionSet::X64
    } else if cfg!(target_arch = "aarch64") {
        InstructionSet::Arm64
    } else {
        panic!("unsupported architecture")
    }
}
