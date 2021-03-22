use std::collections::HashSet;

pub macro small_char_set { ($($e:expr)+) => (
  crate::util::smallcharset::SmallCharSet {
    set: vec![$($e,)*].into_iter().collect::<HashSet<char>>(),
  }
)}

pub struct SmallCharSet {
  set: HashSet<char>,
}

impl SmallCharSet {
  pub fn new(chars: Vec<char>) -> SmallCharSet {
    let set = chars.iter().cloned().collect::<HashSet<char>>();

    Self { set }
  }

  pub fn contains(&self, c: char) -> bool {
    self.set.contains(&c)
  }

  pub fn nonmember_prefix_len(&self, buf: &str) -> u32 {
    let mut n = 0;

    buf
      .chars()
      .into_iter()
      .filter(|c| !self.contains(*c))
      .for_each(|_| n += 1);

    n
  }
}
