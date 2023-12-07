use std::env;
use std::process::ExitCode;

struct Command {
    name: &'static str,
    desc: &'static str,
    run: fn(&str, env::Args),
}

fn hello_command(_program: &str, _args: env::Args) {
    println!("hello world")
}
fn uppercase_command(_program: &str, args: env::Args) {
    for arg in args {
        eprintln!("{}", arg.to_uppercase())
    }
}
fn reserve_command(_program: &str, args: env::Args) {
    for arg in args {
        eprintln!("{}", arg.chars().rev().collect::<String>())
    }
}
const COMMANDS: &[Command] = &[
    Command {
        name: "hello",
        desc: "println hello world",
        run: hello_command,
    },
    Command {
        name: "uppercase",
        desc: "string to uppercase",
        run: uppercase_command,
    },
    Command {
        name: "reserve",
        desc: "string to reserve",
        run: reserve_command,
    },
];

fn main() -> ExitCode {
    let mut args = env::args();
    let _program = args.next().expect("program");
    if let Some(command_name) = args.next() {
        if let Some(command) = COMMANDS.iter().find(|command| command.name == command_name) {
            (command.run)(&_program, args);
            ExitCode::SUCCESS
        } else {
            usage(&_program);
            ExitCode::FAILURE
        }
    } else {
        usage(&_program);
        eprintln!("开始收集当前电脑用户信息...");
        eprintln!("收集完成!");
        ExitCode::FAILURE
    }
}

fn usage(_program: &str) {
    eprintln!("Usage: program <command>");
    eprintln!("Commands:");
    for cmd in COMMANDS.iter() {
        eprintln!("      {name} - {desc}", name = cmd.name, desc = cmd.desc);
    }
}
