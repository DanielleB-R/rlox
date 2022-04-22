use std::{env, fs, io, process};

use rlox::vm::{InterpretResult, VM};

fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).unwrap()
}

fn repl(vm: &mut VM) {
    let mut line = String::new();
    loop {
        line.clear();
        print!("> ");
        if let Err(_) = io::stdin().read_line(&mut line) {
            println!();
            break;
        }

        vm.interpret(&line);
    }
}

fn run_file(vm: &mut VM, filename: &str) {
    let source = read_file(filename);
    match vm.interpret(&source) {
        InterpretResult::Ok => {}
        InterpretResult::CompileError => process::exit(65),
        InterpretResult::RuntimeError => process::exit(70),
    }
}

fn main() {
    let mut vm = VM::new();

    let args: Vec<_> = env::args().collect();
    if args.len() == 1 {
        repl(&mut vm);
    } else if args.len() == 2 {
        run_file(&mut vm, &args[1]);
    } else {
        eprintln!("Usage: rlox [path]");
        process::exit(64);
    }

    drop(vm);
    process::exit(0);
}
