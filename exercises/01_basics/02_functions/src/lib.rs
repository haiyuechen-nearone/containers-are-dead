use std::slice;

// This module has *several* issues:
// 1. First we're missing a parser implementation!
//   This parser should accept input as a `&str` and parse the tokens into a `Vec`. See the `Token` enum below and the test cases for details.
// 2. The function declaration below is NOT valid! `&str` and `Vec<Token>` cannot be passed across an FFI boundary! You will need to fix this as well!
// If you get stuck, ask a neighbor or grab a trainer!

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut out = Vec::with_capacity(input.len());
    let n = unsafe { parse_expression(input.as_ptr(), input.len(), out.as_mut_ptr(), out.capacity()) };
    unsafe {
        out.set_len(n);
    }
    out
}

#[unsafe(no_mangle)]
unsafe extern "C" fn parse_expression<'a>(
    in_ptr: *const u8,
    in_len: usize,
    out_ptr: *mut Token,
    out_len: usize,
) -> usize {
    let mut written = 0;
    let bytes = unsafe { slice::from_raw_parts(in_ptr, in_len) };
    let input_string = str::from_utf8(bytes).expect("Failed to parse input string");
    let mut chars = input_string.char_indices().peekable();
    while let Some((_, c)) = chars.next() {
        let token = match c {
            c if c.is_whitespace() => continue,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Multiply,
            '/' => Token::Divide,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            c if c.is_numeric() => {
                let mut str = c.to_string();
                while let Some((_, c)) = chars.next_if(|(_, c)| c.is_numeric() || *c == '.') {
                    str.push(c);
                }
                Token::Number(str.parse::<f64>().unwrap())
            }
            _ => return written,
        };
        if written == out_len {
            return written;
        }
        unsafe { out_ptr.add(written).write(token) };
        written += 1;
    }

    written
}

// The token that our parse should recognize
#[derive(Debug, PartialEq)]
pub enum Token {
    /// Numbers: 1, or 4.5, or 100000000. Must fit within an f64.
    Number(f64),
    /// The `+` symbol
    Plus,
    /// The `-` symbol
    Minus,
    /// The `*` symbol
    Multiply,
    /// The `/` divide symbol
    Divide,
    /// The `(` symbol
    LeftParen,
    /// The `)` symbol
    RightParen,
}
