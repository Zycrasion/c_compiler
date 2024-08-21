use std::{cell::Ref, collections::HashMap, iter::Peekable, slice::Iter, u32};

use crate::tokenise::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    VOID,
    INT,
    FLOAT,
}

impl Type {
    pub fn bytes(&self) -> u32 {
        match self {
            Type::VOID => 0,
            Type::INT | Type::FLOAT => 4,
        }
    }

    pub fn size_name(&self) -> &str {
        match self {
            Type::VOID => panic!(),
            Type::INT | Type::FLOAT => "DWORD",
        }
    }
}

impl From<&String> for Type {
    fn from(value: &String) -> Self {
        match value.as_str() {
            "void" => Self::VOID,
            "int" => Self::INT,
            "float" => Self::FLOAT,
            _ => {
                eprintln!("Error: {value} is not a valid type");
                panic!()
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
    FunctionDeclaration(Type, String, Vec<ASTNode>),
    VariableDeclaration(Type, String, Box<ASTNode>, u32),
    VariableReference(u32),
    StringLiteral(String),
    Return(Box<ASTNode>),
    IntValue(i32),
    FloatValue(f32),
}

fn _parse(
    token: &Token,
    tokens: &mut Peekable<Iter<Token>>,
) -> Option<ASTNode> {
    match token {
        Token::StringLiteral(string) => {
            Some(ASTNode::StringLiteral(string.clone()))
        }
        Token::Int(value) => Some(ASTNode::IntValue(*value)),
        Token::Float(value) => Some(ASTNode::FloatValue(*value)),
        Token::Keyword(keyword) => match keyword.as_str() {
            "int" | "float" | "void" => {
                // Assume its going to be a function declaration for the time being

                let ty = Type::from(keyword);
                let name = if let Some(Token::StringLiteral(name)) = tokens.next() {
                    name
                } else {
                    eprintln!("Expected Function Name");
                    return None;
                };

                let token = tokens.next().unwrap();
                if Token::Punctuation('=') == *token {
                    // Variable Declaration
                    let value =
                        _parse(tokens.next().unwrap(), tokens).unwrap();

                    assert_eq!(*tokens.next().unwrap(), Token::Punctuation(';'));

                    Some(ASTNode::VariableDeclaration(ty, name.clone(), Box::new(value), u32::MAX))
                } else if Token::Punctuation('(') == *token {
                    // Function Decleration
                    assert_eq!(*tokens.next().unwrap(), Token::Punctuation(')'));
                    assert_eq!(*tokens.next().unwrap(), Token::Punctuation('{'));

                    let mut internal_nodes = vec![];

                    while let Some(tk) = tokens.next() {
                        if *tk == Token::Punctuation('}') {
                            break;
                        }

                        internal_nodes.push(_parse(tk, tokens).unwrap())
                    }

                    Some(ASTNode::FunctionDeclaration(
                        ty,
                        name.clone(),
                        internal_nodes,
                    ))
                } else {
                    None
                }
            }
            "return" => {
                let value = _parse(
                    if let Some(val) = tokens.next() {
                        val
                    } else {
                        eprintln!("Error: Unexpected EOF");
                        panic!()
                    },
                    tokens,
                )
                .unwrap();

                assert_eq!(*tokens.next().unwrap(), Token::Punctuation(';'));

                Some(ASTNode::Return(Box::new(value)))
            }
            _ => {
                eprintln!("Error: Unrecognised Keyword {}", keyword);
                None
            }
        },
        Token::Punctuation(punc) => {
            eprintln!("Unexpected Punctuation: {punc}");
            None
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Vec<ASTNode> {
    let mut nodes = vec![];

    let mut tokens = tokens.iter().peekable();

    while let Some(token) = tokens.next() {
        nodes.push(_parse(token, &mut tokens).unwrap())
    }

    nodes
}
