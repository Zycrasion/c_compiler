use std::{fs::{File, OpenOptions}, io::Write, process::Command};

use c_compiler::{compile::{add_header, compile}, parse::parse, tokenise::tokenise};

const C_WRAPPER : &str = include_str!("_wrapper_file.c");

fn compile_file<S : AsRef<str>>(contents : S) -> String
{
    let tokens = tokenise(contents.as_ref());
    // println!("{:#?}", tokens);
    let nodes = parse(tokens);
    println!("{nodes:#?}");
    compile(nodes)
}

fn main()
{
    let wrapper = compile_file(C_WRAPPER);
    let asm = compile_file(include_str!("../c_test_files/02.c"));
    let asm = format!("{}\n{asm}", add_header(wrapper));

    OpenOptions::new().read(true).write(true).truncate(true).create(true).open("test.asm").unwrap().write(asm.as_bytes()).unwrap();
    if cfg!(target_os = "linux")
    {
        Command::new("nasm").args(["-f", "elf64", "test.asm", "-o", "test.o"]).spawn().unwrap().wait().unwrap();
        Command::new("ld").args(["test.o", "-o", "test"]).spawn().unwrap().wait().unwrap();
        Command::new("rm").args(["test.o", /*"test.asm"*/]).spawn().unwrap().wait().unwrap();
    }
}