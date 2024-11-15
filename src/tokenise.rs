use std::{fmt::{format, Display}, iter::Peekable, str::Chars};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Int(i32),
    Float(f32),
    CharValue(char),
    StringValue(String),
    
    StringLiteral(String),
    Keyword(String),
    Punctuation(char),
    MathSymbol(String),
}

impl Display for Token
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{self:#?}"))
    }
}

impl Token
{
    pub fn extract_string_literal(&self) -> Option<String>
    {
        match self
        {
            Self::StringLiteral(a) => Some(a.clone()),
            _ => None
        }
    }
}

pub fn is_keyword(s: &str) -> bool {
    ["int", "void", "char", "return", "if"].contains(&s)
}

pub fn is_str_literal_char(c: char) -> bool {
    "qwertyuiopasdfghjklzxcvbnm_".contains(c)
}

pub fn is_punc_char(c: char) -> bool {
    // TODO: Detect if * is being used as a dereference or a multiply
    "();,[]{}=&*".contains(c)
}

pub fn is_math_char(c: char) -> bool {
    "+-<>".contains(c)
}

/// TODO: structure better
pub fn tokenise<S>(contents: S) -> Vec<Token>
where
    S: AsRef<str>,
{
    let mut tokens = vec![];

    let contents = contents.as_ref().to_string();

    let mut iter = contents.chars().peekable();

    let mut buffer = String::new();

    let mut is_negative = false;

    'token_loop:
    while let Some(c) = iter.next() {
        if is_punc_char(c) {
            // extra checks for ==, treat is as math symbol
            if iter.peek().is_some() && *iter.peek().unwrap() == '=' 
            {
                iter.next();
                tokens.push(Token::MathSymbol("==".to_string()));
                let len = tokens.len();
                tokens.swap(len - 2, len - 1);
            } else
            {
                tokens.push(Token::Punctuation(c))
            }

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
            if is_negative
            {
                buffer.push('-');
                is_negative = false;
            }
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
        } else if is_math_char(c) {
            // But, make an exception for numbers
            if iter.peek().unwrap().is_ascii_digit()
            {
                is_negative = true;
                continue 'token_loop;
            }

            if ['>', '<'].contains(&c) && iter.peek().is_some() && *iter.peek().unwrap() == '='
            {
                iter.next();
                tokens.push(
                    Token::MathSymbol(format!("{c}="))
                );
            } else
            {
                tokens.push(
                    Token::MathSymbol(c.to_string())
                );
            }

            let len = tokens.len();
            tokens.swap(len - 2, len - 1);
        } else if c.is_whitespace() {
            // recognise it but dont do anything
        } else if c == '\''
        {
            tokens.push(Token::CharValue(iter.next().unwrap()));
            assert!(iter.next().unwrap() == '\'');
        } else if c == '\"'
        {
            while let Some(c2) = iter.peek() {
                if *c2 == '\"' {break;}
                buffer.push(iter.next().unwrap())
            }
            assert!(iter.next().unwrap() == '\"');
            tokens.push(Token::StringValue(buffer));
            buffer = String::new();
        } else if c == '/'
        {
            // Comment
            if iter.peek().is_some() && *iter.peek().unwrap() == '/'
            {
                while iter.next().unwrap() != '\n'
                {

                }                
            } else
            {
                eprintln!("Unexpected /");
                panic!()
            }
        } else {
            eprintln!("Error: Unrecognised character: {}", c);
            panic!()
        }
    }

    tokens
}
