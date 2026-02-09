#[allow(unused_imports)]
use std::{
    env::var,
    io::{self, Write},
    path::Path,
};

fn main() {
    let paths = match var("PATH") {
        Ok(t) => t.split(":").map(|s| s.to_string()).rev().collect(),
        Err(_) => vec![],
    };
    //print!("{:#?}", paths);
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
            "type" => match arg {
                "echo" | "exit" | "type" => println!("{} is a shell builtin", arg),
                _ => {
                    let mut done = false;
                    for path in &paths {
                        match Path::new(path).join(arg).exists() {
                            true => {
                                println!("{} is {}/{}", arg, path, arg);
                                done = true;
                                break;
                            }
                            false => continue,
                        }
                    }
                    if !done {
                        println!("{}: not found", arg);
                    }
                }
            },
            any => println!("{}: command not found", any),
        }
    }
}
