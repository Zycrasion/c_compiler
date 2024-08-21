use std::{cell::Ref, collections::HashMap, iter::Peekable, slice::Iter};

use crate::tokenise::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    VOID,
    INT,
    FLOAT,
}

impl Type {
    pub fn number_of_bytes(&self) -> u8 {
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
    VariableDeclaration(Type, Box<ASTNode>),
    VariableReference(u8),
    Return(Box<ASTNode>),
    IntValue(i32),
    FloatValue(f32),
}

fn _parse(
    token: &Token,
    tokens: &mut Peekable<Iter<Token>>,
    variable_references: &mut HashMap<String, u8>,
) -> Option<ASTNode> {
    match token {
        Token::StringLiteral(string) => {
            if let Some(offset) = variable_references.get(string) {
                return Some(ASTNode::VariableReference(*offset));
            }

            eprintln!("Error: Wasn't expecting string literal {string}");
            None
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
                        _parse(tokens.next().unwrap(), tokens, variable_references).unwrap();

                    assert_eq!(*tokens.next().unwrap(), Token::Punctuation(';'));

                    if variable_references.contains_key(name) {
                        eprintln!("Error: variable already set");
                    } else {
                        *variable_references.get_mut(&"&stack".to_string()).unwrap() += ty.number_of_bytes();
                        variable_references.insert(
                            name.clone(),
                            *variable_references.get(&"&stack".to_string()).unwrap(),
                        );
                    }

                    Some(ASTNode::VariableDeclaration(ty, Box::new(value)))
                } else if Token::Punctuation('(') == *token {
                    // Function Decleration
                    assert_eq!(*tokens.next().unwrap(), Token::Punctuation(')'));
                    assert_eq!(*tokens.next().unwrap(), Token::Punctuation('{'));

                    let mut internal_nodes = vec![];

                    while let Some(tk) = tokens.next() {
                        if *tk == Token::Punctuation('}') {
                            break;
                        }

                        internal_nodes.push(_parse(tk, tokens, variable_references).unwrap())
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
                    variable_references,
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
    let mut variable_references = HashMap::new();
    variable_references.insert("&stack".to_string(), 0);

    while let Some(token) = tokens.next() {
        nodes.push(_parse(token, &mut tokens, &mut variable_references).unwrap())
    }

    nodes
}
