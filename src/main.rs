use std::{env, fs::{File, OpenOptions}, io::{Read, Write}, process::Command};

use c_compiler::{compile::{add_header, compile}, parse::{parse, ASTNode}, tokenise::tokenise};

const C_WRAPPER : &str = include_str!("_wrapper_file.c");

fn parse_file<S : AsRef<str>>(contents : S) -> Vec<ASTNode>
{
    let tokens = tokenise(contents.as_ref());
    let nodes = parse(tokens);
    nodes
}

fn main()
{
    let arg = env::args().collect::<Vec<String>>();
    if arg.contains(&"--assemble".to_string())
    {
        println!("Only Assembling");
        assemble();
    } else {
        let mut wrapper = parse_file(C_WRAPPER);
        let mut buffer = String::new();
        File::open(env::args().nth(1).unwrap()).unwrap().read_to_string(&mut buffer).unwrap();
        let mut main_file = parse_file(buffer);
        wrapper.append(&mut main_file);
        let asm = compile(wrapper);
    
        OpenOptions::new().read(true).write(true).truncate(true).create(true).open("test.asm").unwrap().write(asm.as_bytes()).unwrap();
        assemble();
    }
    println!("Finished Compilation!");
}

pub fn assemble()
{
    if cfg!(target_os = "linux")
    {
        Command::new("nasm").args(["-f", "elf64", "test.asm", "-o", "test.o"]).spawn().unwrap().wait().unwrap();
        Command::new("ld").args(["test.o", "-lc", "-I", "/lib64/ld-linux-x86-64.so.2", "-o", "test"]).spawn().unwrap().wait().unwrap();
        Command::new("rm").args(["test.o", /*"test.asm"*/]).spawn().unwrap().wait().unwrap();
    }
}