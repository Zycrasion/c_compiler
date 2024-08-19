use std::{iter::Peekable, str::Chars};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    StringLiteral(String),
    Int(i32),
    Float(f32),
    Keyword(String),
    Punctuation(char),
}

pub fn is_keyword(s: &str) -> bool {
    ["int", "float", "return"].contains(&s)
}

pub fn is_str_literal_char(c: char) -> bool {
    "qwertyuiopasdfghjklzxcvbnm_".contains(c)
}

pub fn is_punc_char(c: char) -> bool {
    "();,[]{}".contains(c)
}

pub fn tokenise<S>(contents: S) -> Vec<Token>
where
    S: AsRef<str>,
{
    let mut tokens = vec![];

    let contents = contents.as_ref().to_string();

    let mut iter = contents.chars().peekable();

    let mut buffer = String::new();

    while let Some(c) = iter.next() {
        if is_punc_char(c) {
            tokens.push(Token::Punctuation(c))
        } else if is_str_literal_char(c) {
            buffer.push(c);

            while let Some(c2) = iter.peek() {
                if !is_str_literal_char(*c2) {
                    break;
                }
                buffer.push(iter.next().unwrap())
            }

            if is_keyword(&buffer) {
                tokens.push(Token::Keyword(buffer.clone()));
            } else {
                tokens.push(Token::StringLiteral(buffer.clone()));
            }

            buffer.clear();
        } else if c.is_ascii_digit() {
            buffer.push(c);

            while let Some(c2) = iter.peek() {
                if !(c2.is_ascii_digit() || *c2 == '.') {
                    break;
                }
                buffer.push(iter.next().unwrap())
            }

            let dot_count = buffer.matches(".").collect::<Vec<&str>>().len();
            if dot_count == 1 {
                tokens.push(Token::Float(
                    buffer
                        .parse::<f32>()
                        .expect(&format!("Error: Could not parse {} as a float", buffer)),
                ))
            } else if dot_count > 1 {
                // Not a valid number
                eprintln!("Error: {} is not a valid number", buffer);
                return vec![];
            } else {
                tokens.push(Token::Int(
                    buffer
                        .parse::<i32>()
                        .expect(&format!("Error: Could not parse {} as an int", buffer)),
                ))
            }

            buffer.clear();
        }
    }

    tokens
}
