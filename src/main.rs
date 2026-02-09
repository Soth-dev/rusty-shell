//#[allow(unused_imports)]
use std::{
    env::var,
    fs::metadata,
    io::{self, Write},
    os::unix::fs::PermissionsExt,
    path::Path,
};

fn main() {
    let paths = match var("PATH") {
        Ok(t) => t.split(':').map(|s| s.to_string()).collect(),
        Err(_) => vec![],
    };
    //print!("{:#?}", paths);
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
        let (cmd, arg) = match input.trim().split_once(' ') {
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
                        if Path::new(path).join(arg).is_file() {
                            match metadata(format!("{}/{}", path, arg)) {
                                Ok(m) => {
                                    if m.permissions().mode() & 0o111 != 0 {
                                        println!("{} is {}/{}", arg, path, arg);
                                        done = true;
                                        break;
                                    }
                                }
                                Err(_) => continue,
                            }
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
