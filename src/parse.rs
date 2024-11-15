use std::{cell::Ref, collections::HashMap, iter::Peekable, slice::Iter, u32};

use low_level_ir::{CompareOperation, ComparePredicate, OperandType, Size, Value};

use crate::tokenise::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    VOID,
    INT,
    CHAR,
    PTR(Box<Type>),
}

impl Type {
    pub fn size(&self) -> Size {
        match self {
            Type::VOID => panic!(),
            Type::INT => Size::DoubleWord,
            Type::CHAR => Size::Byte,
            Type::PTR(_) => Size::QuadWord,
        }
    }

    pub fn into_ir(&self) -> OperandType {
        match self {
            Type::VOID => OperandType::Undefined,
            Type::CHAR => OperandType::Char,
            Type::INT => OperandType::Int(self.size()),
            Type::PTR(a) => OperandType::Pointer(Box::new(a.into_ir())),

        }
    }
    
    pub fn read_type(token: &Token, tokens: &mut Peekable<Iter<Token>>) -> Type
    {
        let base_type = if let Token::Keyword(value) = token
        {
            Self::from(value)
        } else {
            eprintln!("Expected a type, got {token} instead");
            panic!()
        };

        if let Token::Punctuation(punc) = **tokens.peek().unwrap()
        {
            if punc == '*'
            {
                tokens.next();
                return Type::PTR(Box::new(base_type));
            }
        }

        base_type
    }
}

impl From<&String> for Type {
    fn from(value: &String) -> Self {
        match value.as_str() {
            "void" => Self::VOID,
            "int" => Self::INT,
            "char" => Self::CHAR,
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
    StringValue(String),
    IntValue(i32),
    CharValue(char),
    FunctionCall(String, Vec<ASTNode>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Comparison
{
    pub lhs : Box<ASTNode>,
    pub rhs : Box<ASTNode>,
    pub operation : CompareOperation
}

#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
    FunctionDeclaration(Type, String, Vec<ASTNode>, Vec<(String, Type)>),
    FunctionCall(String, Vec<ASTNode>),
    Add(Box<ASTNode>, Box<ASTNode>),
    Sub(Box<ASTNode>, Box<ASTNode>),
    VariableDeclaration(Type, String, Box<ASTNode>),
    SetVariable(ASTValue, Box<ASTNode>),
    InlineAssembly(String),
    Return(Option<Box<ASTNode>>),
    Value(ASTValue),
    If { predicate : Comparison, main_body : Vec<ASTNode>, else_body : Option<Vec<ASTNode>> }
}

fn _try_set_value(lhs : &ASTValue, token: &Token, tokens: &mut Peekable<Iter<Token>>) -> Option<ASTNode>
{
    if **tokens.peek().unwrap() == Token::Punctuation('=') {
        assert_eq!(*tokens.next().unwrap(), Token::Punctuation('='));
        let value = _parse(tokens.next().unwrap(), tokens, true);
        assert_eq!(*tokens.next().unwrap(), Token::Punctuation(';'));

        return Some(ASTNode::SetVariable(lhs.clone(), Box::new(value.unwrap())));
    }

    None
}

fn parse_comparison(token: &Token, tokens: &mut Peekable<Iter<Token>>) -> Option<Comparison>
{
    match token
    {
        Token::MathSymbol(x) if matches!(x.as_str(), ">" | "<" | "==" | ">=" | "<=" | "!=`") => {
            let lhs = Box::new(_parse(tokens.next().unwrap(), tokens, true).unwrap());
            let rhs = Box::new(_parse(tokens.next().unwrap(), tokens, true).unwrap());
            let operation = match x.as_str()
            {
                "==" => CompareOperation::EQ,
                ">" => CompareOperation::GT,
                "<" => CompareOperation::LT,
                "<=" => CompareOperation::LTE,
                ">=" => CompareOperation::GTE,
                "!=" => CompareOperation::NEQ,
                _ => panic!()
            };

            Some(Comparison { lhs, rhs, operation })
        },
        _ => None
    }
}

fn _parse(token: &Token, tokens: &mut Peekable<Iter<Token>>, as_value: bool) -> Option<ASTNode> {
    match token {
        Token::CharValue(val) => {
            Some(ASTNode::Value(ASTValue::CharValue(*val)))
        }
        Token::StringValue(string) => {
            Some(ASTNode::Value(ASTValue::StringValue(string.clone())))
        },
        Token::StringLiteral(string) => {
            if **tokens.peek().unwrap() == Token::Punctuation('=') && !as_value {
                return _try_set_value(&ASTValue::StringLiteral(string.clone()), token, tokens)
            }
            if **tokens.peek().unwrap() == Token::Punctuation('(') {
                assert_eq!(*tokens.next().unwrap(), Token::Punctuation('('));

                let mut parameters = vec![];
                while **tokens.peek().expect("UNEXPECTED EOF") != Token::Punctuation(')') {
                    if **tokens.peek().unwrap() == Token::Punctuation(',') {
                        tokens.next();
                    }
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
            "int" | "void" | "char" => {
                // Assume its going to be a function declaration for the time being
                if as_value {
                    panic!()
                }

                let ty = Type::read_type(token, tokens);

                let name = tokens.next().unwrap().extract_string_literal().unwrap();

                let function_or_variable = tokens.next().unwrap();
                if Token::Punctuation('=') == *function_or_variable {
                    // Variable Declaration
                    let value = _parse(tokens.next().unwrap(), tokens, true).unwrap();

                    assert_eq!(*tokens.next().unwrap(), Token::Punctuation(';'));
                    Some(ASTNode::VariableDeclaration(
                        ty,
                        name.clone(),
                        Box::new(value),
                    ))
                } else if Token::Punctuation('(') == *function_or_variable {
                    // Function Declaration

                    // Paramters
                    let mut parameters = vec![];
                    while **tokens.peek().expect("UNEXPECTED EOF") != Token::Punctuation(')') {
                        let parameter_type = Type::read_type(tokens.next().unwrap(), tokens);

                        let parameter_name = tokens.next().unwrap().extract_string_literal().unwrap();

                        parameters.push((parameter_name, parameter_type));

                        if **tokens.peek().unwrap() != Token::Punctuation(',')
                        {
                            break;
                        }
                        tokens.next();
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
            },
            "if" => {
                assert_eq!(*tokens.next().unwrap(), Token::Punctuation('('));

                let predicate = parse_comparison(tokens.next().unwrap(), tokens).unwrap();

                assert_eq!(*tokens.next().unwrap(), Token::Punctuation(')'));
                assert_eq!(*tokens.next().unwrap(), Token::Punctuation('{'));

                let mut main_body = vec![];

                while let Some(tk) = tokens.next() {
                    if *tk == Token::Punctuation('}') {
                        break;
                    }

                    main_body.push(_parse(tk, tokens, false).unwrap());
                }

                Some(ASTNode::If { predicate , main_body, else_body: None })
            },
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
            '*' => {
                let val = ASTValue::Deref(
                    tokens
                        .next()
                        .unwrap()
                        .extract_string_literal()
                        .expect("Expected String Literal"),
                );

                if let Some(set_value) = _try_set_value(&val, token, tokens)
                {
                    return Some(set_value);
                } else
                {
                    return Some(ASTNode::Value(val))
                }
            },
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
                        && **tokens.peek().unwrap() != Token::Punctuation(';')
                    {
                        let curr = tokens.next().unwrap();
                        let curr = match curr {
                            Token::StringLiteral(a) | Token::Keyword(a) | Token::StringValue(a) | Token::MathSymbol(a)  => a.clone(),
                            Token::Int(a) => a.to_string(),
                            Token::Float(a) => a.to_string(),
                            Token::Punctuation(a) => a.to_string(),
                            Token::CharValue(a) => a.to_string(),
                        };
                        buffer.push_str(&curr);
                        buffer.push(' ');
                    }

                    buffer = buffer.trim().to_string();
                    assert_eq!(buffer.pop().unwrap(), ']');
                    
                    buffer = buffer.trim().to_string();
                    assert_eq!(buffer.pop().unwrap(), ']');
                    
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
        Token::MathSymbol(x) if matches!(x.as_str(), "+" | "-") => {
            let lhs = _parse(tokens.next().unwrap(), tokens, true).unwrap();
            let rhs = _parse(tokens.next().unwrap(), tokens, true).unwrap();

            if x == "+"
            {
                Some(ASTNode::Add(Box::new(lhs), Box::new(rhs)))
            } else
            {
                Some(ASTNode::Sub(Box::new(lhs), Box::new(rhs)))
            }
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
