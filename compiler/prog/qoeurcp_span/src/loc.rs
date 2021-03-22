use super::interface::{ColumnIndex, LineIndex};

use std::cmp;
use std::fmt;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Loc {
  pub line: LineIndex,
  pub column: ColumnIndex,
}

impl fmt::Display for Loc {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl cmp::PartialOrd for Loc {
  fn partial_cmp(&self, rhs: &Loc) -> Option<cmp::Ordering> {
    Some(self.cmp(rhs))
  }
}

impl cmp::Ord for Loc {
  fn cmp(&self, rhs: &Self) -> cmp::Ordering {
    let ord = self.line.cmp(&rhs.line);

    if ord == cmp::Ordering::Equal {
      return self.column.cmp(&rhs.column);
    }

    ord
  }
}

impl Loc {
  pub fn new(line: LineIndex, column: ColumnIndex) -> Loc {
    Self { line, column }
  }

  pub fn zero() -> Loc {
    Self::new(LineIndex(0), ColumnIndex(0))
  }

  pub fn text(&self) -> String {
    format!("{}:{}", self.line, self.column)
  }
}
