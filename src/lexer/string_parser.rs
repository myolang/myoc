use super::*;

/// This function assumes the `quote_exclusive_string_src` argument
/// does not have any double quotes.
/// If `quote_exclusive_string_src` has double quotes,
/// this function may produce an incorrect result.
///
/// This function returns `Err(span)` if it encounters
/// an invalid escape sequence or an unterminated escape sequence.
/// - If the escape sequence is invalid,
///   `span` is the span of the invalid escape sequence,
///   **excluding** the enclosing curly braces.
/// - If the escape sequence is unterminated,
///   the `span` is the span of the unterminated escape sequence,
///   **including** the left curly brace.
///   By definition, there is no right curly brace
///   (otherwise the escape sequence would be terminated).
pub fn get_string_value(
    quote_exclusive_string_src: &str,
) -> Result<String, (ByteIndex, ByteIndex)> {
    StringParser::new(quote_exclusive_string_src).parse()
}

struct StringParser<'a> {
    quote_exclusive_string_src: &'a str,
    out: String,
    state: State,
}

#[derive(Clone, Copy)]
enum State {
    Main,
    Escape { start: ByteIndex, byte_len: usize },
}

impl<'a> StringParser<'a> {
    fn new(quote_exclusive_string_src: &'a str) -> Self {
        Self {
            quote_exclusive_string_src,
            out: String::new(),
            state: State::Main,
        }
    }
}

impl StringParser<'_> {
    fn parse(mut self) -> Result<String, (ByteIndex, ByteIndex)> {
        for (current_index, current) in self.quote_exclusive_string_src.char_indices() {
            self.handle_char(ByteIndex(current_index), current)?;
        }

        match self.state {
            State::Escape { start, .. } => {
                Err((start, ByteIndex(self.quote_exclusive_string_src.len())))
            }
            State::Main => Ok(self.out),
        }
    }

    fn handle_char(
        &mut self,
        current_index: ByteIndex,
        current: char,
    ) -> Result<(), (ByteIndex, ByteIndex)> {
        match self.state {
            State::Main => self.handle_char_assuming_state_is_main(current_index, current),
            State::Escape { start, byte_len } => {
                self.handle_char_assuming_state_is_escape(current_index, current, start, byte_len)
            }
        }
    }

    fn handle_char_assuming_state_is_main(
        &mut self,
        current_index: ByteIndex,
        current: char,
    ) -> Result<(), (ByteIndex, ByteIndex)> {
        match current {
            '{' => {
                self.state = State::Escape {
                    start: current_index,
                    byte_len: '{'.len_utf8(),
                };
                Ok(())
            }

            '}' => Err((current_index, ByteIndex(current_index.0 + '}'.len_utf8()))),

            _ => {
                self.out.push(current);
                Ok(())
            }
        }
    }

    fn handle_char_assuming_state_is_escape(
        &mut self,
        current_index: ByteIndex,
        current: char,
        start: ByteIndex,
        byte_len: usize,
    ) -> Result<(), (ByteIndex, ByteIndex)> {
        if current == '}' {
            let brace_exclusive_start = ByteIndex(start.0 + '{'.len_utf8());
            let brace_exclusive_end = current_index;
            self.push_escape_sequence(brace_exclusive_start, brace_exclusive_end)?;
            self.state = State::Main;
            return Ok(());
        }

        self.state = State::Escape {
            start,
            byte_len: byte_len + current.len_utf8(),
        };
        Ok(())
    }

    fn push_escape_sequence(
        &mut self,
        brace_exclusive_start: ByteIndex,
        brace_exclusive_end: ByteIndex,
    ) -> Result<(), (ByteIndex, ByteIndex)> {
        let invalid_escape_sequence_err = Err((brace_exclusive_start, brace_exclusive_end));

        let byte_len = brace_exclusive_end.0 - brace_exclusive_start.0;
        if byte_len < 3 {
            return invalid_escape_sequence_err;
        }

        let brace_exclusive_src =
            &self.quote_exclusive_string_src[brace_exclusive_start.0..brace_exclusive_end.0];
        if !brace_exclusive_src.starts_with("0x") {
            return invalid_escape_sequence_err;
        }

        let hex_start = ByteIndex(brace_exclusive_start.0 + "0x".len());
        let hex_src = &self.quote_exclusive_string_src[hex_start.0..brace_exclusive_end.0];
        let Ok(val) = u32::from_str_radix(hex_src, 16) else {
            return invalid_escape_sequence_err;
        };
        let Ok(val) = char::try_from(val) else {
            return invalid_escape_sequence_err;
        };
        self.out.push(val);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn empty() {
        let actual = get_string_value("");
        let expected = Ok("".to_owned());
        assert_eq!(expected, actual);
    }

    #[test]
    fn hello_world() {
        let actual = get_string_value("hello world");
        let expected = Ok("hello world".to_owned());
        assert_eq!(expected, actual);
    }

    #[test]
    fn hello_unescaped_newline_world() {
        let actual = get_string_value("hello\nworld");
        let expected = Ok("hello\nworld".to_owned());
        assert_eq!(expected, actual);
    }

    #[test]
    fn hello_escaped_newline_world() {
        let actual = get_string_value("hello{0xA}world");
        let expected = Ok("hello\nworld".to_owned());
        assert_eq!(expected, actual);
    }

    #[test]
    fn hello_lcurly_world_rcurly() {
        let actual = get_string_value("hello{0x7B}world{0x7D}");
        let expected = Ok("hello{world}".to_owned());
        assert_eq!(expected, actual);
    }

    #[test]
    fn hello_double_quote_world_double_quote() {
        let actual = get_string_value("hello {0x22}world{0x22}");
        let expected = Ok("hello \"world\"".to_owned());
        assert_eq!(expected, actual);
    }

    #[test]
    fn unterminated_invalid_escape() {
        let actual = get_string_value("hello {world");
        let expected = Err((ByteIndex(6), ByteIndex(12)));
        assert_eq!(expected, actual);
    }

    #[test]
    fn unterminated_but_otherwise_valid_escape() {
        let actual = get_string_value("hello {0x22");
        let expected = Err((ByteIndex(6), ByteIndex(11)));
        assert_eq!(expected, actual);
    }

    #[test]
    fn empty_escape() {
        let actual = get_string_value("hello {} world");
        let expected = Err((ByteIndex(7), ByteIndex(7)));
        assert_eq!(expected, actual);
    }

    #[test]
    fn too_short_escape_1_char() {
        let actual = get_string_value("hello {0} world");
        let expected = Err((ByteIndex(7), ByteIndex(8)));
        assert_eq!(expected, actual);
    }

    #[test]
    fn too_short_escape_2_char() {
        let actual = get_string_value("hello {0x} world");
        let expected = Err((ByteIndex(7), ByteIndex(9)));
        assert_eq!(expected, actual);
    }

    #[test]
    fn bad_prefix_escape() {
        let actual = get_string_value("hello {BEEF} world");
        let expected = Err((ByteIndex(7), ByteIndex(11)));
        assert_eq!(expected, actual);
    }

    #[test]
    fn curly_in_escape() {
        let actual = get_string_value("hello {0x{A}} world");
        let expected = Err((ByteIndex(7), ByteIndex(11)));
        assert_eq!(expected, actual);
    }

    #[test]
    fn non_hex_escape() {
        let actual = get_string_value("hello {0xG} world");
        let expected = Err((ByteIndex(7), ByteIndex(10)));
        assert_eq!(expected, actual);
    }

    #[test]
    fn capital_x_escape() {
        let actual = get_string_value("hello {0XA} world");
        let expected = Err((ByteIndex(7), ByteIndex(10)));
        assert_eq!(expected, actual);
    }

    #[test]
    fn unescaped_rcurly() {
        let actual = get_string_value("hello } world");
        let expected = Err((ByteIndex(6), ByteIndex(7)));
        assert_eq!(expected, actual);
    }
}
