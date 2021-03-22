use super::loc::Loc;

use std::cmp::{max, min};
use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Span {
  pub start: Loc,
  pub end: Loc,
}

// TODO: remove me from
impl Default for Span {
  fn default() -> Span {
    Self::from_start(Loc::zero())
  }
}

impl fmt::Display for Span {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl Span {
  pub fn new(start: Loc, end: Loc) -> Span {
    Self { start, end }
  }

  pub fn from_start(start: Loc) -> Span {
    Self::new(start, start)
  }

  pub fn expand(&self, end: Loc) -> Span {
    Self::new(self.start, end)
  }

  pub fn merge(a: &Span, b: &Span) -> Span {
    let start = min(a.start, b.start);
    let end = max(a.end, b.end);

    Self::new(start, end)
  }

  pub fn text(&self) -> String {
    format!("{}:{}", self.start, self.end)
  }

  pub fn zero() -> Span {
    Self::from_start(Loc::zero())
  }
}
