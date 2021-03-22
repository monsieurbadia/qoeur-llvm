mod interface;
mod jit;
mod util;

pub use self::interface::*;
pub use self::jit::Jit;

use std::process::Command;

// just for testing to see if it's working
fn make_exe() {
  Command::new("rm")
    .args(&["-rf", "test"])
    .output()
    .expect("failed to execute process");

  Command::new("gcc")
    .args(&["out/test.ll", "-o", "out/test"])
    .output()
    .expect("failed to execute process");
}

pub fn compile(file_name: &str, input: &str) {
  let tree = qoeurcp_tokenizer::parse(input);
  let mut compiler = Jit::new();
  let _ = compiler.codegen(tree.ast.nodes);

  make_exe()
}
