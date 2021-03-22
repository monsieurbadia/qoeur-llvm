mod jit;

pub use self::jit::Jit;

pub fn compile(file_name: &str, input: &str) {
  let tree = qoeurcp_tokenizer::parse(input);
  let mut compiler = Jit::new();
  let _ = compiler.compile(tree.ast.nodes);
}
