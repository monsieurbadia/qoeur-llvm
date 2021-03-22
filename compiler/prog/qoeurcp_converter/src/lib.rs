// TODO: tmp do
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
// end
#![feature(box_patterns)]
#![feature(box_syntax)]
#![feature(decl_macro)]
#![recursion_limit = "256"]

mod cranelift;
mod interface;
mod llvm;
mod scope;

#[cfg(test)]
mod test;

pub use self::interface::BackendKind::{self, *};

pub fn compile(file_name: &str, input: &str, mode: &BackendKind) {
  match mode {
    Cranelift => cranelift::compile(file_name, input),
    Llvm => llvm::compile(file_name, input),
  };
}
