#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
        let (cmd, arg) = match input.trim().split_once(" ") {
            Some((fst, snd)) => (fst, snd),
            None => (input.trim(), ""),
        };
        match cmd {
            "exit" => break,
            "echo" => println!("{}", arg),
            "type" => {
                if ["echo", "exit", "type"].contains(&arg) {
                    println!("{} is a shell builtin", arg);
                } else {
                    println!("{}: not found", arg);
                }
            }
            any => println!("{}: command not found", any),
        }
    }
}
