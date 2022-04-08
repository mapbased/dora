#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![feature(allocator_api)]
#![feature(new_uninit)]
#![recursion_limit = "256"]

extern crate alloc;

#[cfg(target_os = "windows")]
extern crate winapi;

#[macro_use]
extern crate memoffset;

mod boots;
mod bytecode;
mod cannon;
mod compiler;
mod constpool;
mod cpu;
mod disassembler;
mod driver;
mod gc;
mod handle;
mod language;
mod masm;
mod mem;
mod mode;
mod object;
mod os;
mod safepoint;
mod size;
mod stack;
mod stdlib;
mod threads;
mod timer;
mod utils;
mod vm;
mod vtable;

#[cfg(not(test))]
pub fn run() -> i32 {
    driver::start()
}
