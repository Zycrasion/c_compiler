use std::fmt::format;

use low_level_ir::*;

use crate::parse::{ASTNode, ASTValue};

fn compile_value(value: ASTNode, compiler: &mut Compiler) -> Value {
    if let ASTNode::Add(lhs, rhs) = value {
        return Value::Add(
            Box::new(compile_value(*lhs, compiler)),
            Box::new(compile_value(*rhs, compiler)),
        );
    }

    if let ASTNode::Sub(lhs, rhs) = value {
        return Value::Sub(
            Box::new(compile_value(*lhs, compiler)),
            Box::new(compile_value(*rhs, compiler)),
        );
    }

    if let ASTNode::Value(value) = value {
        compile_astvalue(value, compiler)
    } else {
        eprintln!("Expected a value; Recieved {value:#?} instead");
        panic!()
    }
}

pub fn compile_astvalue(value: ASTValue, compiler: &mut Compiler) -> Value {
    match value {
        ASTValue::IntValue(value) => Value::Int(value.to_string()),
        ASTValue::StringLiteral(value) => Value::Variable(value),
        ASTValue::FunctionCall(name, values) => Value::FunctionCall(
            name,
            values
                .iter()
                .cloned()
                .map(|v| compile_value(v, compiler))
                .collect(),
        ),
        ASTValue::Deref(name) => Value::Dereference(name),
        ASTValue::Ref(name) => Value::Reference(name),
        ASTValue::CharValue(value) => Value::Char(value),
        ASTValue::StringValue(value) => {
            let define_name = format!("_SD{}", compiler.string_defines.len());
            compiler.string_defines.push((define_name.clone(), value));
            Value::StringLiteral(define_name)
        }
    }
}

fn compile_node(node: ASTNode, compiler: &mut Compiler) -> Vec<Operand> {
    let mut statements = vec![];

    match node {
        ASTNode::If {
            predicate,
            main_body,
            else_body,
        } => {
            let lhs = compile_value(*predicate.lhs, compiler);
            let rhs = compile_value(*predicate.rhs, compiler);
            statements.push(Operand::If {
                predicate: ComparePredicate {
                    operation: predicate.operation,
                    lhs,
                    rhs,
                },
                main_body: compile_list(main_body, compiler),
            });
        }
        ASTNode::SetVariable(lhs, value) => {
            statements.push(Operand::SetValue(
                compile_astvalue(lhs, compiler),
                compile_value(*value, compiler),
            ));
        }
        ASTNode::FunctionCall(name, values) => {
            statements.push(Operand::FunctionCall(
                name,
                values
                    .iter()
                    .cloned()
                    .map(|v| compile_value(v, compiler))
                    .collect(),
            ));
        }
        ASTNode::InlineAssembly(assembly) => {
            statements.push(Operand::InlineAssembly(format!(
                "{assembly} ; User Defined Inline Assembly"
            )));
        }
        ASTNode::FunctionDeclaration(ty, name, inner, params) => {
            statements.push(Operand::FunctionDecl(
                ty.into_ir(),
                name,
                compile_list(inner, compiler),
                params
                    .iter()
                    .cloned()
                    .map(|v| (v.0, v.1.into_ir()))
                    .collect(),
            ));
        }
        ASTNode::VariableDeclaration(ty, name, value) => {
            statements.push(Operand::DeclareVariable(
                ty.into_ir(),
                name,
                compile_value(*value, compiler),
            ));
        }
        ASTNode::Add(lhs, rhs) => {
            statements.push(Operand::Add(
                OperandType::Int(Size::DoubleWord),
                compile_value(*lhs, compiler),
                compile_value(*rhs, compiler),
            ));
        }
        ASTNode::Sub(lhs, rhs) => {
            statements.push(Operand::Subtract(
                OperandType::Int(Size::DoubleWord),
                compile_value(*lhs, compiler),
                compile_value(*rhs, compiler),
            ));
        }
        ASTNode::Return(value) => {
            if value.is_none() {
                statements.push(Operand::Return(Value::Null));
            } else {
                let value = compile_value(*value.unwrap(), compiler);
                statements.push(Operand::Return(value));
            }
        }
        ASTNode::Value(value) => {}
    }

    statements
}

fn compile_list(ast: Vec<ASTNode>, compiler: &mut Compiler) -> Vec<Operand> {
    let mut statements = vec![];

    for node in ast {
        statements.append(&mut compile_node(node, compiler));
    }

    statements
}

pub fn add_header(s: String) -> String {
    format!("[bits 64]\nsection .text\nglobal _start\n{s}")
}

pub fn compile(ast: Vec<ASTNode>) -> String {
    let mut ir_compiler = Compiler::new();

    for node in ast {
        let mut operands = compile_node(node, &mut ir_compiler);
        ir_compiler.operands.append(&mut operands);
    }

    ir_compiler.compile()
}
