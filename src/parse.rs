use std::{cell::Ref, collections::HashMap, iter::Peekable, slice::Iter, u32};

use low_level_ir::{OperandType, Size, Value};

use crate::tokenise::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    VOID,
    INT,
    PTR(Box<Type>),
}

impl Type {
    pub fn size(&self) -> Size {
        match self {
            Type::VOID => panic!(),
            Type::INT => Size::DoubleWord,
            Type::PTR(_) => Size::QuadWord,
        }
    }

    pub fn into_ir(&self) -> OperandType {
        match self {
            Type::VOID => OperandType::Undefined,
            Type::INT => OperandType::Int(self.size()),
            Type::PTR(a) => OperandType::Pointer(Box::new(a.into_ir())),

        }
    }
}

impl From<&String> for Type {
    fn from(value: &String) -> Self {
        match value.as_str() {
            "void" => Self::VOID,
            "int" => Self::INT,
            _ => {
                eprintln!("Error: {value} is not a valid type");
                panic!()
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ASTValue {
    Deref(String),
    Ref(String),
    StringLiteral(String),
    IntValue(i32),
    FunctionCall(String, Vec<ASTNode>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
    FunctionDeclaration(Type, String, Vec<ASTNode>, Vec<(String, Type)>),
    FunctionCall(String, Vec<ASTNode>),
    Add(Box<ASTNode>, Box<ASTNode>),
    Sub(Box<ASTNode>, Box<ASTNode>),
    VariableDeclaration(Type, String, Box<ASTNode>),
    InlineAssembly(String),
    Return(Option<Box<ASTNode>>),
    Value(ASTValue),
}

fn _parse(token: &Token, tokens: &mut Peekable<Iter<Token>>, as_value: bool) -> Option<ASTNode> {
    match token {
        Token::StringLiteral(string) => {
            if **tokens.peek().unwrap() == Token::Punctuation('(') {
                assert_eq!(*tokens.next().unwrap(), Token::Punctuation('('));

                let mut parameters = vec![];
                while **tokens.peek().expect("UNEXPECTED EOF") != Token::Punctuation(')') {
                    let value = _parse(tokens.next().unwrap(), tokens, true);
                    parameters.push(value.unwrap());
                }

                assert_eq!(*tokens.next().unwrap(), Token::Punctuation(')'));
                if as_value {
                    return Some(ASTNode::Value(ASTValue::FunctionCall(
                        string.clone(),
                        parameters,
                    )));
                }
                assert_eq!(*tokens.next().unwrap(), Token::Punctuation(';'));
                return Some(ASTNode::FunctionCall(string.clone(), parameters));
            }

            Some(ASTNode::Value(ASTValue::StringLiteral(string.clone())))
        }
        Token::Int(value) => Some(ASTNode::Value(ASTValue::IntValue(*value))),
        Token::Keyword(keyword) => match keyword.as_str() {
            "int" | "void" => {
                // Assume its going to be a function declaration for the time being
                if as_value {
                    panic!()
                }

                let ty = Type::from(keyword);
                let is_ptr = if **tokens.peek().unwrap() == Token::Punctuation('*') {
                    tokens.next();
                    true
                } else {
                    false
                };
                let name = if let Some(Token::StringLiteral(name)) = tokens.next() {
                    name
                } else {
                    eprintln!("Expected Function Name");
                    return None;
                };

                let token = tokens.next().unwrap();
                if Token::Punctuation('=') == *token {
                    // Variable Declaration
                    let value = _parse(tokens.next().unwrap(), tokens, true).unwrap();

                    assert_eq!(*tokens.next().unwrap(), Token::Punctuation(';'));
                    Some(ASTNode::VariableDeclaration(
                        if is_ptr
                        {
                            Type::PTR(Box::new(ty))
                        } else
                        {
                            ty
                        },
                        name.clone(),
                        Box::new(value),
                    ))
                } else if Token::Punctuation('(') == *token {
                    // Function Declaration

                    // Paramters
                    let mut parameters = vec![];
                    while **tokens.peek().expect("UNEXPECTED EOF") != Token::Punctuation(')') {
                        let _type = if let Some(Token::Keyword(_type)) = tokens.next() {
                            Type::from(_type)
                        } else {
                            eprintln!("Expected Function Name");
                            return None;
                        };

                        let name = if let Some(Token::StringLiteral(name)) = tokens.next() {
                            name
                        } else {
                            eprintln!("Expected Function Name");
                            return None;
                        };

                        parameters.push((name.clone(), _type));
                    }

                    assert_eq!(*tokens.next().unwrap(), Token::Punctuation(')'));
                    assert_eq!(*tokens.next().unwrap(), Token::Punctuation('{'));

                    let mut internal_nodes = vec![];

                    while let Some(tk) = tokens.next() {
                        if *tk == Token::Punctuation('}') {
                            break;
                        }

                        internal_nodes.push(_parse(tk, tokens, false).unwrap())
                    }

                    Some(ASTNode::FunctionDeclaration(
                        ty,
                        name.clone(),
                        internal_nodes,
                        parameters,
                    ))
                } else {
                    None
                }
            }
            "return" => {
                if **tokens.peek().unwrap() == Token::Punctuation(';') {
                    tokens.next();
                    return Some(ASTNode::Return(None));
                }

                let value = _parse(
                    if let Some(val) = tokens.next() {
                        val
                    } else {
                        eprintln!("Error: Unexpected EOF");
                        panic!()
                    },
                    tokens,
                    true,
                )
                .unwrap();

                assert_eq!(*tokens.next().unwrap(), Token::Punctuation(';'));

                Some(ASTNode::Return(Some(Box::new(value))))
            }
            _ => {
                eprintln!("Error: Unrecognised Keyword {}", keyword);
                None
            }
        },
        Token::Punctuation(punc) => match *punc {
            '*' => Some(ASTNode::Value(ASTValue::Deref(
                tokens
                    .next()
                    .unwrap()
                    .extract_string_literal()
                    .expect("Expected String Literal"),
            ))),
            '&' => Some(ASTNode::Value(ASTValue::Ref(
                tokens
                    .next()
                    .unwrap()
                    .extract_string_literal()
                    .expect("Expected String Literal"),
            ))),
            '[' => {
                if **tokens.peek().unwrap() == Token::Punctuation('[') {
                    tokens.next();
                    let mut buffer = String::new();

                    while tokens.peek().is_some()
                        && **tokens.peek().unwrap() != Token::Punctuation(']')
                    {
                        let curr = tokens.next().unwrap();
                        let curr = match curr {
                            Token::StringLiteral(a) | Token::Keyword(a) => a.clone(),
                            Token::Int(a) => a.to_string(),
                            Token::Float(a) => a.to_string(),
                            Token::Punctuation(a) | Token::MathSymbol(a) => a.to_string(),
                        };
                        buffer.push_str(&curr);
                        buffer.push(' ');
                    }
                    assert_eq!(*tokens.next().unwrap(), Token::Punctuation(']'));
                    assert_eq!(*tokens.next().unwrap(), Token::Punctuation(']'));
                    assert_eq!(*tokens.next().unwrap(), Token::Punctuation(';'));

                    return Some(ASTNode::InlineAssembly(buffer));
                } else {
                    None
                }
            }
            _ => {
                eprintln!("Unexpected Punctuation: {punc}");
                None
            }
        },
        Token::MathSymbol('+') => {
            let lhs = _parse(tokens.next().unwrap(), tokens, true).unwrap();
            let rhs = _parse(tokens.next().unwrap(), tokens, true).unwrap();

            Some(ASTNode::Add(Box::new(lhs), Box::new(rhs)))
        }
        Token::MathSymbol('-') => {
            let lhs = _parse(tokens.next().unwrap(), tokens, true).unwrap();
            let rhs = _parse(tokens.next().unwrap(), tokens, true).unwrap();

            Some(ASTNode::Sub(Box::new(lhs), Box::new(rhs)))
        }
        Token::MathSymbol(_) | Token::Float(_) => panic!(),
    }
}

pub fn parse(tokens: Vec<Token>) -> Vec<ASTNode> {
    let mut nodes = vec![];

    let mut tokens = tokens.iter().peekable();

    while let Some(token) = tokens.next() {
        nodes.push(_parse(token, &mut tokens, false).unwrap())
    }

    nodes
}
