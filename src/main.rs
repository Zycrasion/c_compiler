use std::{fs::{File, OpenOptions}, io::Write, process::Command};

use c_compiler::{compile::{add_header, compile}, parse::{parse, ASTNode}, tokenise::tokenise};

const C_WRAPPER : &str = include_str!("_wrapper_file.c");

fn parse_file<S : AsRef<str>>(contents : S) -> Vec<ASTNode>
{
    let tokens = tokenise(contents.as_ref());
    // println!("{:#?}", tokens);
    parse(tokens)
}

fn main()
{
    let mut wrapper = parse_file(C_WRAPPER);
    let mut main_file = parse_file(include_str!("../c_test_files/03.c"));
    wrapper.append(&mut main_file);
    let asm = compile(wrapper);

    OpenOptions::new().read(true).write(true).truncate(true).create(true).open("test.asm").unwrap().write(asm.as_bytes()).unwrap();
    if cfg!(target_os = "linux")
    {
        Command::new("nasm").args(["-f", "elf64", "test.asm", "-o", "test.o"]).spawn().unwrap().wait().unwrap();
        Command::new("ld").args(["test.o", "-o", "test"]).spawn().unwrap().wait().unwrap();
        Command::new("rm").args(["test.o", /*"test.asm"*/]).spawn().unwrap().wait().unwrap();
    }
}