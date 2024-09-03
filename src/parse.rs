use std::{cell::Ref, collections::HashMap, iter::Peekable, slice::Iter, u32};

use crate::tokenise::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    VOID,
    INT,
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
pub enum ASTValue
{
    StringLiteral(String),
    IntValue(i32),    
}

#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
    FunctionDeclaration(Type, String, Vec<ASTNode>),
    VariableDeclaration(Type, String, Box<ASTNode>),
    Return(Box<ASTNode>),
    Value(ASTValue)
}

fn _parse(
    token: &Token,
    tokens: &mut Peekable<Iter<Token>>,
) -> Option<ASTNode> {
    match token {
        Token::StringLiteral(string) =>  Some(ASTNode::Value(ASTValue::StringLiteral(string.clone()))),
        Token::Int(value) => Some(ASTNode::Value(ASTValue::IntValue(*value))),
        Token::Keyword(keyword) => match keyword.as_str() {
            "int" | "void" => {
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

                    Some(ASTNode::VariableDeclaration(ty, name.clone(), Box::new(value)))
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
        },
        Token::MathSymbol(_) | Token::Float(_) => panic!()
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
