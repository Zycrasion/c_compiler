use std::{cell::Ref, collections::HashMap, iter::Peekable, slice::Iter, u32};

use low_level_ir::{OperandType, Size};

use crate::tokenise::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    VOID,
    INT,
}

impl Type
{
    pub fn size(&self) -> Size
    {
        match self
        {
            Type::VOID => panic!(),
            Type::INT => Size::DoubleWord,
        }
    }

    pub fn into_ir(&self) -> OperandType
    {
        match self
        {
            Type::VOID => OperandType::Undefined,
            Type::INT => OperandType::Int(self.size()),
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
pub enum ASTValue
{
    StringLiteral(String),
    IntValue(i32),
    FunctionCall(String) 
}

#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
    FunctionDeclaration(Type, String, Vec<ASTNode>),
    FunctionCall(String),
    VariableDeclaration(Type, String, Box<ASTNode>),
    InlineAssembly(String),
    Return(Box<ASTNode>),
    Value(ASTValue)
}

fn _parse(
    token: &Token,
    tokens: &mut Peekable<Iter<Token>>,
    as_value : bool
) -> Option<ASTNode> {
    match token {
        Token::StringLiteral(string) =>  {
            if **tokens.peek().unwrap() == Token::Punctuation('(')
            {
                assert_eq!(*tokens.next().unwrap(), Token::Punctuation('('));
                assert_eq!(*tokens.next().unwrap(), Token::Punctuation(')'));
                if as_value
                {
                    return Some(ASTNode::Value(ASTValue::FunctionCall(string.clone())))
                }
                assert_eq!(*tokens.next().unwrap(), Token::Punctuation(';'));
                return Some(ASTNode::FunctionCall(string.clone()));
            }

            Some(ASTNode::Value(ASTValue::StringLiteral(string.clone())))
        },
        Token::Int(value) => Some(ASTNode::Value(ASTValue::IntValue(*value))),
        Token::Keyword(keyword) => match keyword.as_str() {
            "int" | "void" => {
                // Assume its going to be a function declaration for the time being
                if as_value {panic!()}

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
                        _parse(tokens.next().unwrap(), tokens, true).unwrap();

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

                        internal_nodes.push(_parse(tk, tokens, false).unwrap())
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
                    true
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
            if *punc == '['
            {
                if **tokens.peek().unwrap() == Token::Punctuation('[')
                {
                    tokens.next();
                    let mut buffer = String::new();

                    while tokens.peek().is_some() && **tokens.peek().unwrap() != Token::Punctuation(']')
                    {
                        let curr = tokens.next().unwrap();
                        let curr = match curr
                        {
                            Token::StringLiteral(a) |
                            Token::Keyword(a) => a.clone(),
                            Token::Int(a) => a.to_string(),
                            Token::Float(a) => a.to_string(),
                            Token::Punctuation(a) |
                            Token::MathSymbol(a) => a.to_string()
                        };
                        buffer.push_str(&curr);
                        buffer.push(' ');
                    }
                    assert_eq!(*tokens.next().unwrap(), Token::Punctuation(']'));
                    assert_eq!(*tokens.next().unwrap(), Token::Punctuation(']'));
                    assert_eq!(*tokens.next().unwrap(), Token::Punctuation(';'));

                    return Some(ASTNode::InlineAssembly(buffer));
                }
            }
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
        nodes.push(_parse(token, &mut tokens, false).unwrap())
    }

    nodes
}
