#![allow(unused)]
use std::result;
use std::net::{TcpStream, TcpListener};
use std::process::ExitCode;
use std::io::Write;

use std::env;

type Result<T> = result::Result<(), T>;

struct Command {
    name: &'static str,
    desc: &'static str,
    run: fn(&str, env::Args)->Result<()>,
}

fn hello_command(_program: &str, _args: env::Args) ->Result<()> {
    println!("hello world");
    Ok(())
}
fn uppercase_command(_program: &str, args: env::Args) ->Result<()> {
    for arg in args {
        eprintln!("{}", arg.to_uppercase())
    }
    Ok(())
}
fn reserve_command(_program: &str, args: env::Args) ->Result<()> {
    for arg in args {
        eprintln!("{}", arg.chars().rev().collect::<String>())
    }
    Ok(())
}

fn underscore_to_camelcase_command(_program: &str, args: env::Args) ->Result<()> {
    for arg in args {
        eprintln!("{}", underscore_to_camelcase(&arg))
    }
    Ok(())
}

fn start_tcp_server(_program: &str, _args: env::Args) -> Result<()> {
    let address = "127.0.0.1:6969";
    let listener = TcpListener::bind(address).map_err(|err| {
        eprintln!("ERROR: could not bind {address}: {err}")
    })?;
    println!("[DEBUG] tcp server Listen on address:{address}", address = address);
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                writeln!(stream, "hallow").map_err(|err| {
                    eprintln!("{err}")
                });
            }
            Err(err) => {
                eprintln!("Error: could not accept connection");
            }
        }
    }
    println!("test");
    Ok(())
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
    Command {
        name: "underscore_to_camelcase",
        desc: "string from underscore to camelcase",
        run: underscore_to_camelcase_command,
    },
    Command {
        name: "tcpserver",
        desc: "run a tcp server",
        run: start_tcp_server,
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

fn underscore_to_camelcase(input: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;

    for c in input.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }

    result
}
