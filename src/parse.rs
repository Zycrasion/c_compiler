use std::{iter::Peekable, slice::Iter};

use crate::tokenise::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    VOID,
    INT,
    FLOAT,
}

impl From<&String> for Type
{
    fn from(value: &String) -> Self {
        match value.as_str()
        {
            "void" => Self::VOID,
            "int" => Self::INT,
            "float" => Self::FLOAT,
            _ => 
            {
                eprintln!("Error: {value} is not a valid type");
                panic!()
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
    FunctionDeclaration(Type, String, Vec<ASTNode>),
    Return(Box<ASTNode>),
    IntValue(i32),
    FloatValue(f32)
}

fn _parse(token : &Token, tokens : &mut Peekable<Iter<Token>>) -> Option<ASTNode>
{
    match token {
        Token::StringLiteral(string) => todo!(),
        Token::Int(value) => Some(ASTNode::IntValue(*value)),
        Token::Float(value) => Some(ASTNode::FloatValue(*value)),
        Token::Keyword(keyword) => match keyword.as_str() {
            "int" | "float" | "void" => {
                // Assume its going to be a function declaration for the time being

                let ty = Type::from(keyword);
                let name = if let Some(Token::StringLiteral(name)) = tokens.next()
                {
                    name
                }  else
                {
                    eprintln!("Expected Function Name");
                    return None;
                };

                assert_eq!(*tokens.next().unwrap(), Token::Punctuation('('));
                assert_eq!(*tokens.next().unwrap(), Token::Punctuation(')'));
                assert_eq!(*tokens.next().unwrap(), Token::Punctuation('{'));

                let mut internal_nodes = vec![];

                while let Some(tk) = tokens.next()
                {
                    if *tk == Token::Punctuation('}')
                    {
                        break;
                    }

                    internal_nodes.push(_parse(tk, tokens).unwrap())
                }

                Some(ASTNode::FunctionDeclaration(ty, name.clone(), internal_nodes))
            },
            "return" => {
                let value = _parse(if let Some(val) = tokens.next()
                {
                    val
                } else
                {
                    eprintln!("Error: Unexpected EOF");
                    panic!()
                }, tokens).unwrap();
                
                assert_eq!(*tokens.next().unwrap(), Token::Punctuation(';'));

                Some(ASTNode::Return(Box::new(value)))
            },
            _ => {
                eprintln!("Error: Unrecognised Keyword {}", keyword);
                None
            }
        },
        Token::Punctuation(punc) => {eprintln!("Unexpected Punctuation: {punc}"); None},
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
