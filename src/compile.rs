use low_level_ir::{IRModule, Operand, OperandType, Size, Value, ValueCodegen};

use crate::parse::{ASTNode, ASTValue};

fn compile_value(value : ASTNode) -> Value
{
    if let ASTNode::Add(lhs, rhs) = value
    {
        return Value::Add(Box::new(compile_value(*lhs)), Box::new(compile_value(*rhs)))
    }

    if let ASTNode::Sub(lhs, rhs) = value
    {
        return Value::Sub(Box::new(compile_value(*lhs)), Box::new(compile_value(*rhs)))
    }
    
    if let ASTNode::Value(val) = value
    {
        match val
        {
            ASTValue::IntValue(value) => Value::Int(value.to_string()),
            ASTValue::StringLiteral(value) => Value::Variable(value),
            ASTValue::FunctionCall(name, values) => Value::FunctionCall(name, values.iter().cloned().map(|v| compile_value(v)).collect()),
            ASTValue::Deref(name) => Value::Dereference(name),
            ASTValue::Ref(name) => Value::Reference(name),
        }
    } else
    {
        eprintln!("Expected a value; Recieved {value:#?} instead");
        panic!()
    }
}

fn compile_node(node : ASTNode) -> Vec<Operand>
{
    let mut statements = vec![];
    
    match node
    {
        ASTNode::FunctionCall(name, values) => {
            statements.push(Operand::FunctionCall(name, values.iter().cloned().map(|v| compile_value(v)).collect()));
        },
        ASTNode::InlineAssembly(assembly) => {
            statements.push(Operand::InlineAssembly(format!("{assembly} ; User Defined Inline Assembly")));
        }
        ASTNode::FunctionDeclaration(ty, name, inner, params) => {
            statements.push(Operand::FunctionDecl(ty.into_ir(), name, compile_list(inner), params.iter().cloned().map(|v| (v.0, v.1.into_ir())).collect()));
        },
        ASTNode::VariableDeclaration(ty, name, value) =>
        {
            statements.push(Operand::DeclareVariable(ty.into_ir(), name, compile_value(*value)));
        },
        ASTNode::Add(lhs, rhs) =>
        {
            statements.push(Operand::Add(OperandType::Int(Size::DoubleWord), compile_value(*lhs), compile_value(*rhs)));
        }
        ASTNode::Sub(lhs, rhs) =>
        {
            statements.push(Operand::Subtract(OperandType::Int(Size::DoubleWord), compile_value(*lhs), compile_value(*rhs)));
        }
        ASTNode::Return(value) => 
        {
            if value.is_none()
            {
                statements.push(Operand::Return(Value::Null));
            } else
            {
                let value = compile_value(*value.unwrap());
                statements.push(Operand::Return(value));    
            }
        },
        ASTNode::Value(value) => {},
    }

    statements
}

fn compile_list(ast : Vec<ASTNode>) -> Vec<Operand>
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
        ir_module.operands.append(&mut compile_node(node));
    }

    // Make sure variables are automatically dropped after their final use
    ir_module.optimise();

    ir_module.compile()
}