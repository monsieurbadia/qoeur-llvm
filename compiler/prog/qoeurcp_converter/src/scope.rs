use qoeurcp_tokenizer::ast::*;

use std::collections::HashMap;
use std::collections::LinkedList;

pub type ScopeError = String;
pub type ScopeLinked<T> = LinkedList<T>;
pub type ScopeResult<T> = Result<T, ScopeError>;

#[derive(Clone, Debug, PartialEq)]
pub struct Scope {
  functions: HashMap<String, Fun>,
  variables: HashMap<String, Local>,
}

impl Scope {
  pub fn new() -> Scope {
    Self {
      functions: HashMap::new(),
      variables: HashMap::new(),
    }
  }

  pub fn add_function(&mut self, fun: Fun) -> ScopeResult<()> {
    match self.get_function(&fun.name()) {
      Some(_) => Err(format!(
        "scope:fn:add_function:error: function already exist"
      )),
      None => Ok({
        self.functions.insert(fun.name(), fun);
      }),
    }
  }

  pub fn add_variable(&mut self, local: Local) -> ScopeResult<()> {
    match self.get_variable(&local.name()) {
      Some(_) => Err(format!(
        "scope:fn:add_variable:error: variable already exist"
      )),
      None => Ok({
        self.variables.insert(local.name(), local);
      }),
    }
  }

  pub fn get_function(&self, name: &str) -> Option<&Fun> {
    self.functions.get(name)
  }

  pub fn get_variable(&self, name: &str) -> Option<&Local> {
    self.variables.get(name)
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ScopeStack {
  scopes: ScopeLinked<Scope>,
}

impl ScopeStack {
  pub fn new() -> ScopeStack {
    Self {
      scopes: ScopeLinked::new(),
    }
  }

  pub fn add_variable(&mut self, local: Local) -> Result<(), String> {
    match self.scopes.front_mut() {
      Some(head) => head.add_variable(local),
      None => Err(format!("scope_stack:add_variable:error")),
    }
  }

  pub fn add_function(&mut self, fun: Fun) -> Result<(), String> {
    match self.scopes.front_mut() {
      Some(head) => head.add_function(fun),
      None => Err(format!("scope_stack:add_function:error")),
    }
  }

  pub fn get_function(&self, name: &str) -> Option<&Fun> {
    for scope in self.scopes.iter() {
      match scope.get_function(name) {
        Some(fun) => return Some(fun),
        None => continue,
      };
    }

    None
  }

  pub fn get_variable(&self, name: &str) -> Option<&Local> {
    for scope in self.scopes.iter() {
      match scope.get_variable(name) {
        Some(local) => return Some(local),
        None => continue,
      };
    }

    None
  }

  pub fn scope_enter(&mut self) {
    self.scopes.push_front(Scope::new());
  }

  pub fn scope_exit(&mut self) {
    self.scopes.pop_front().unwrap();
  }
}
