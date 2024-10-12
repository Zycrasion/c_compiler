use low_level_ir::{IRModule, IRStatement, Operand, OperandType, Size, Value, ValueCodegen};

use crate::parse::{ASTNode, ASTValue};

fn compile_value(value : ASTNode) -> Value
{
    if let ASTNode::Value(val) = value
    {
        match val
        {
            ASTValue::IntValue(value) => Value::Int(value.to_string()),
            ASTValue::StringLiteral(value) => Value::VariableReference(value),
            ASTValue::FunctionCall(name) => Value::FunctionCall(name),
        }
    } else
    {
        eprintln!("Expected a value; Recieved {value:#?} instead");
        panic!()
    }
}

fn compile_node(node : ASTNode) -> Vec<IRStatement>
{
    let mut statements = vec![];
    
    match node
    {
        ASTNode::FunctionCall(name) => {
            statements.push(Operand::FunctionCall(name).ir(OperandType::Undefined));
        },
        ASTNode::InlineAssembly(assembly) => {
            statements.push(Operand::InlineAssembly(format!("{assembly} ; User Defined Inline Assembly")).ir(OperandType::Undefined));
        }
        ASTNode::FunctionDeclaration(ty, name, inner) => {
            statements.push(Operand::FunctionDecl(name).ir(ty.into_ir()));

            statements.append(&mut compile_list(inner));
        },
        ASTNode::VariableDeclaration(ty, name, value) =>
        {
            statements.push(Operand::Move(Value::Variable(ty.size(), name), compile_value(*value)).ir(ty.into_ir()))
        },
        ASTNode::Return(value) => 
        {
            if value.is_none()
            {
                statements.push(Operand::Return(Value::Null).ir(OperandType::Undefined));
            } else
            {
                let value = compile_value(*value.unwrap());
                statements.push(Operand::Return(value).ir(OperandType::Int(Size::DoubleWord) /* value.get_type() */));    
            }
        },
        ASTNode::Value(value) => {},
    }

    statements
}

fn compile_list(ast : Vec<ASTNode>) -> Vec<IRStatement>
{
    let mut statements = vec![];

    for node in ast
    {
        statements.append(&mut compile_node(node));
    }

    statements
}

pub fn add_header(s : String) -> String
{
    format!("[bits 64]\nsection .text\nglobal _start\n{s}")
}

pub fn compile(ast : Vec<ASTNode>) -> String
{
    let mut ir_module = IRModule::new();

    for node in ast
    {
        ir_module.statements.append(&mut compile_node(node));
    }

    // Make sure variables are automatically dropped after their final use
    ir_module.optimise();

    ir_module.compile()
}