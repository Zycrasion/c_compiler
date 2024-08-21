use std::collections::HashMap;

use crate::parse::ASTNode;

fn __variable_pass(node : &mut ASTNode, stack_ptr : &mut u32, variable_offsets : &mut HashMap<String, u32>)
{
    match node
    {
        ASTNode::FunctionDeclaration(_, _, inner_tree) => {
            _variable_pass(inner_tree)
        },
        ASTNode::VariableDeclaration(_ty, name, ref mut inner, ref mut offset) => {
            __variable_pass(inner, stack_ptr, variable_offsets);
 
            *stack_ptr += _ty.bytes();

            let _ = variable_offsets.insert(name.clone(), *stack_ptr);
            *offset = *stack_ptr;
        },
        ASTNode::StringLiteral(name) => {
            if variable_offsets.contains_key(name)
            {
                *node = ASTNode::VariableReference(*variable_offsets.get(name.as_str()).unwrap());
            }
        },
        ASTNode::Return(ref mut inner) => __variable_pass(inner, stack_ptr, variable_offsets),
        _ => {}
    }
}

fn _variable_pass(ast : &mut Vec<ASTNode>)
{
    let mut variable_offsets : HashMap<String, u32>= HashMap::new();
    let mut stack_ptr = 0u32;

    for node in ast
    {
        __variable_pass(node, &mut stack_ptr, &mut variable_offsets)
    }
}

pub fn variable_pass(ast : &mut Vec<ASTNode>)
{
    _variable_pass(ast)
}