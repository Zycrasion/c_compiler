use std::{fs::{File, OpenOptions}, io::Write, process::Command};

use c_compiler::{compile::compile, parse::parse, tokenise::tokenise};

fn main()
{
    let tokens = tokenise(include_str!("../c_test_files/02.c"));
    // println!("{:#?}", tokens);
    let mut nodes = parse(tokens);
    println!("{:#?}", nodes);
    let asm = compile(nodes);
    println!("{asm}");

    // OpenOptions::new().read(true).write(true).create(true).open("test.asm").unwrap().write(asm.as_bytes()).unwrap();
    // if cfg!(target_os = "linux")
    // {
    //     Command::new("nasm").args(["-f", "elf64", "test.asm", "-o", "test.o"]).spawn().unwrap().wait().unwrap();
    //     Command::new("ld").args(["test.o", "-o", "test"]).spawn().unwrap().wait().unwrap();
    //     Command::new("rm").args(["test.o", "test.asm"]).spawn().unwrap().wait().unwrap();
    // }
}