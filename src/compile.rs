use low_level_ir::{IRModule, IRStatement, Operand, OperandType, Size, Value, ValueCodegen};

use crate::parse::{ASTNode, ASTValue};

fn compile_value(value : ASTNode) -> Value
{
    if let ASTNode::Value(val) = value
    {
        match val
        {
            ASTValue::IntValue(value) => Value::Int(value.to_string()),
            ASTValue::StringLiteral(value) => Value::VariableReference(value) 
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
        ASTNode::FunctionDeclaration(ty, name, inner) => {
            statements.push(IRStatement
            {
                op_type: ty.into_ir(),
                operand: Operand::Label,
                lhs: Value::StringLiteral(name),
                rhs: None,
            });

            statements.append(&mut compile_list(inner));
        },
        ASTNode::VariableDeclaration(ty, name, value) =>
        {
            statements.push(IRStatement
            {
                op_type : ty.into_ir(),
                operand : Operand::Move,
                lhs : Value::Variable(ty.size(), name),
                rhs : Some(compile_value(*value))
            })
        },
        ASTNode::Return(value) => 
        {
            statements.push(IRStatement
            {
                // TODO: ADD TYPES FOR RETURN
                op_type: OperandType::Int(Size::DoubleWord),
                operand: Operand::Return,
                lhs: compile_value(*value),
                rhs: None,
            });
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

pub fn compile(ast : Vec<ASTNode>) -> String
{
    let mut ir_module = IRModule::new();

    for node in ast
    {
        ir_module.statements.append(&mut compile_node(node));
    }

    ir_module.compile()
}