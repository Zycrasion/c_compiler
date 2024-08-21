use crate::parse::ASTNode;

fn _compile(node: ASTNode) -> String {
    let mut result = String::new();

    match node {
        ASTNode::VariableReference(off) => {
            result.push_str(&format!("[rbp-{off}]"));
        }
        ASTNode::VariableDeclaration(_ty, _name, value, offset) =>
        {
            result.push_str(&format!("sub rsp, {}\n", _ty.bytes()));
            if let ASTNode::VariableReference(off) = *value
            {
                result.push_str(&format!("mov eax, {} [rbp-{}]\n", _ty.size_name(), off));
                result.push_str(&format!("mov {} [rbp-{}], eax\n", _ty.size_name(), offset));

            } else
            {
                result.push_str(&format!("mov {} [rbp-{}], {}\n", _ty.size_name(), offset, _compile(*value)));
            }
        }
        ASTNode::FunctionDeclaration(_ty, name, inner) => {
            result.push_str(&format!("{name}:\n")); // Setup a label
            result.push_str("push rbp\n"); // Save rbp, we are going to override it
            result.push_str("mov rbp, rsp\n"); // Set rbp to the current stack address to keep a base to reference variables off of

            // Compile the body
            result.push_str(&compile_list(inner));
        }
        ASTNode::Return(value) => {
            result.push_str(&format!("mov rax, {}\n", _compile(*value)));
            result.push_str("mov rsp, rbp\n"); // Restore stack pointer
            result.push_str("pop rbp\n"); // Load the original rbp off of the stack
            result.push_str("ret\n"); // Return to previous function
        }
        ASTNode::IntValue(value) => result.push_str(&format!("{value}")),
        ASTNode::FloatValue(_) => todo!(),
        ASTNode::StringLiteral(_) => panic!(),
    }
    result
}

pub fn compile_list(ast: Vec<ASTNode>) -> String {
    let mut result = String::new();

    for node in ast {
        result.push_str(&_compile(node));
    }

    result
}

/// TODO: Remove the need for an assembler, directly compile to ML
pub fn compile(ast: Vec<ASTNode>) -> String {
    let mut result = String::new();

    result.push_str("[bits 64]\nsection .text\nglobal _start\n_start:\ncall main\nmov rdi, rax\nmov rax, 60\nsyscall\n");

    for node in ast {
        result.push_str(&_compile(node));
    }

    result
}
