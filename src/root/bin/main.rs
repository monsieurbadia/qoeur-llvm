use qoeurcp::BackendKind;

pub enum CompileMode {
  Tokens,
  Ast,
  Jit,
}

fn main() {
  let pathname = "data/code/add.q5";
  let path = std::path::Path::new(&pathname);

  let file_name = format!("{}", path.file_name().unwrap().to_str().unwrap())
    .replace(".q5", "");

  let f = match std::fs::read_to_string(&path) {
    Err(_) => None,
    Ok(file) => Some(file),
  };

  let f = f.unwrap();

  // TODO: tmp
  let compile_mode = CompileMode::Jit;

  match compile_mode {
    CompileMode::Tokens => {
      qoeurcp::tokenize(&f);
    }
    CompileMode::Ast => {
      qoeurcp::parse(&f);
    }
    CompileMode::Jit => {
      qoeurcp::compile(&file_name, &f, &BackendKind::Llvm);
    }
  }
}
