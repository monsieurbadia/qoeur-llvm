pub fn is_carriage_return(ascii: char) -> bool {
  ascii == '\u{000D}' // \r
}

pub fn is_colon(ascii: char) -> bool {
  ascii == '\u{003A}' // ':'
}

pub fn is_comment(ascii: char) -> bool {
  ascii == '\u{0023}' // #
}

pub fn is_dollar(ascii: char) -> bool {
  ascii == '\u{0024}' // $
}

pub fn is_double_quote(ascii: char) -> bool {
  ascii == '\u{0022}' // "
}

pub fn is_end_of_file(ascii: char) -> bool {
  ascii == '\u{0}'
}

pub fn is_end_of_line(ascii: char) -> bool {
  ascii == '\u{000A}' // \n
}

pub fn is_group(ascii: char) -> bool {
  match ascii {
    '[' | ']' | '(' | ')' | '{' | '}' => true,
    _ => false,
  }
}

pub fn is_form_feed(ascii: char) -> bool {
  ascii == '\u{000C}' // \f
}

pub fn is_horizontal_tabulation(ascii: char) -> bool {
  ascii == '\u{0009}' // \t
}

pub fn is_identifier(ascii: char) -> bool {
  ascii.is_alphabetic() || is_underscore(ascii)
}

pub fn is_id_continue(ascii: char) -> bool {
  is_identifier(ascii)
    || is_number(ascii)
    || (ascii > '\x7f' && is_xid_continue(ascii))
}

pub fn is_id_start(ascii: char) -> bool {
  is_identifier(ascii) || (ascii > '\x7f' && is_xid_start(ascii))
}

pub fn is_number(ascii: char) -> bool {
  ascii.is_digit(10)
}

pub fn is_operator(ascii: char) -> bool {
  match ascii {
    '+' | '-' | '*' | '/' | '%' | '>' | '<' | '=' | '!' | '.' => true,
    _ => false,
  }
}

pub fn is_punctuation(ascii: char) -> bool {
  ascii.is_ascii_punctuation()
}

pub fn is_quote(ascii: char) -> bool {
  is_double_quote(ascii) || is_single_quote(ascii)
}

pub fn is_shebang(ascii: char) -> bool {
  ascii == '\u{0021}'
}

pub fn is_single_quote(ascii: char) -> bool {
  ascii == '\u{0027}' // '
}

pub fn is_space(ascii: char) -> bool {
  ascii == '\u{0020}' // " "
}

pub fn is_symbol(ascii: char) -> bool {
  match ascii {
    ':' | ',' | ';' | '.' | '!' | '?' | '$' | '@' | '#' => true,
    _ => false,
  }
}

pub fn is_underscore(ascii: char) -> bool {
  ascii == '\u{005F}' // _
}

pub fn is_vertical_tab(ascii: char) -> bool {
  ascii == '\u{000B}' // \v
}

pub fn is_whitespace(ascii: char) -> bool {
  match ascii {
    | '\u{0009}' // \t
    | '\u{000A}' // \n
    | '\u{000B}' // vertical tab
    | '\u{000C}' // form feed
    | '\u{000D}' // \r
    | '\u{0020}' // space
    | '\u{0085}' // NEXT LINE from latin1
    | '\u{200E}' // LEFT-TO-RIGHT MARK
    | '\u{200F}' // RIGHT-TO-LEFT MARK
    | '\u{2028}' // LINE SEPARATOR
    | '\u{2029}' // PARAGRAPH SEPARATOR
    => true,
    _ => false,
  }
}

pub fn is_xid_start(ascii: char) -> bool {
  unicode_xid::UnicodeXID::is_xid_start(ascii)
}

pub fn is_xid_continue(ascii: char) -> bool {
  unicode_xid::UnicodeXID::is_xid_continue(ascii)
}
