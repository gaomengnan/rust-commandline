#![allow(unused)]
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::ExitCode;
use std::sync::Arc;
use std::{result, thread};

use std::env;
use std::sync::mpsc::{channel, Receiver, Sender};

type Result<T> = result::Result<(), T>;

struct Command {
    name: &'static str,
    desc: &'static str,
    run: fn(&str, env::Args) -> Result<()>,
}

fn hello_command(_program: &str, _args: env::Args) -> Result<()> {
    println!("hello world");
    Ok(())
}
fn uppercase_command(_program: &str, args: env::Args) -> Result<()> {
    for arg in args {
        eprintln!("{}", arg.to_uppercase())
    }
    Ok(())
}
fn reserve_command(_program: &str, args: env::Args) -> Result<()> {
    for arg in args {
        eprintln!("{}", arg.chars().rev().collect::<String>())
    }
    Ok(())
}

fn underscore_to_camelcase_command(_program: &str, args: env::Args) -> Result<()> {
    for arg in args {
        eprintln!("{}", underscore_to_camelcase(&arg))
    }
    Ok(())
}

enum Message {
    // 客户端连接
    ClientConnected {
        author: Arc<TcpStream>,
    },
    // 断开连接
    ClientDisconected {
        author: Arc<TcpStream>,
    },
    // 消息
    New {
        author: Arc<TcpStream>,
        msg: Vec<u8>,
    },
}

#[derive(Debug)]
struct Client {
    conn: Arc<TcpStream>,
}
fn server(message: Receiver<Message>) {
    let mut clients = HashMap::new();
    loop {
        let msg = message.recv().expect("ERROR: could not hung up");

        match msg {
            Message::ClientConnected { author } => {
                let addr = author.peer_addr().expect("TODO: cache the addr");
                clients.insert(
                    addr,
                    Client {
                        conn: author.clone(),
                    },
                );
            }
            Message::ClientDisconected { author } => {
                let addr = author.peer_addr().expect("ERROR: could not got peer_addr");
                clients.remove(&addr);
            }
            Message::New { author, msg } => {
                for (addr, client) in &clients {
                    eprintln!("{addr}:{:?}", client);
                    let current_addr = author
                        .peer_addr()
                        .expect("ERROR: could not got sender peer_addr");
                    if current_addr != *addr {
                        let _ = client.conn.as_ref().write(&msg);
                    }
                }
            }
        };
    }
}

fn client(stream: Arc<TcpStream>, sender: Sender<Message>) -> Result<()> {
    sender
        .send(Message::ClientConnected {
            author: stream.clone(),
        })
        .map_err(|err| eprint!("ERROR: could not send message to server thread:{err}"))?;
    let mut buffer = vec![0, 64];
    loop {
        let n = stream.as_ref().read(&mut buffer).map_err(|err| {
            eprintln!("ERROR: could not read message from server thread:{err}");
            sender.send(Message::ClientDisconected {
                author: stream.clone(),
            });
        })?;
        sender
            .send(Message::New {
                msg: buffer[0..n].to_vec(),
                author: stream.clone(),
            })
            .map_err(|err| eprintln!("ERROR: could not send message to server: {err}"));
    }
}

fn start_tcp_server(_program: &str, _args: env::Args) -> Result<()> {
    let address = "127.0.0.1:6969";
    let listener = TcpListener::bind(address)
        .map_err(|err| eprintln!("ERROR: could not bind {address}: {err}"))?;
    println!(
    "[DEBUG] tcp server Listen on address:{address}",
    address = address
);
    let (sender, receiver) = channel();

    // 创建一个线程处理消息的接收
    thread::spawn(|| server(receiver));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let message_sender: Sender<Message> = sender.clone();
                let stream = Arc::new(stream);
                thread::spawn(|| client(stream, message_sender));
            }
            Err(err) => {
                eprintln!("Error: could not accept connection");
            }
        }
    }
    Ok(())
}

fn connect_tcp_server(_program: &str, _args: env::Args) -> Result<()> {
    let address = "127.0.0.1:6969";
    let listener = TcpListener::bind(address)
        .map_err(|err| eprintln!("ERROR: could not bind {address}: {err}"))?;
    println!(
    "[DEBUG] tcp server Listen on address:{address}",
    address = address
);
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                writeln!(stream, "hallow").map_err(|err| eprintln!("{err}"));
            }
            Err(err) => {
                eprintln!("Error: could not accept connection");
            }
        }
    }
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
    Command {
        name: "connect",
        desc: "connect a tcp server",
        run: connect_tcp_server,
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
        eprintln!("Tips: you should provider a command");
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
