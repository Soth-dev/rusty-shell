//#[allow(unused_imports)]
use std::{
    env::var,
    fs::metadata,
    io::{self, Write},
    os::unix::fs::PermissionsExt,
    path::Path,
    process::{Command, Stdio},
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
        let (cmd, args) = match input.trim().split_once(' ') {
            Some((fst, snd)) => (fst, snd),
            _ => (input.trim(), ""),
        };
        match cmd {
            "" => continue,
            "exit" => break,
            "echo" => println!("{}", args),
            "type" => match args {
                "echo" | "exit" | "type" => println!("{} is a shell builtin", args),
                _ => match check_exec(&paths, args) {
                    Some(path) => println!("{} is {}", args, path),
                    _ => println!("{}: not found", args),
                },
            },
            _ => match check_exec(&paths, cmd) {
                Some(_path) => {
                    let _run = Command::new(cmd)
                        .args(args.split_whitespace())
                        //.env("PATH", path)
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .spawn()
                        .expect("err")
                        .wait()
                        .expect("err");
                }
                _ => println!("{}: command not found", cmd),
            },
        }
    }
}

fn check_exec(paths: &Vec<String>, file: &str) -> Option<String> {
    for path in paths {
        if Path::new(path).join(file).is_file() {
            match metadata(format!("{}/{}", path, file)) {
                Ok(m) => {
                    if m.permissions().mode() & 0o111 != 0 {
                        return Some(format!("{}/{}", path, file));
                    }
                }
                Err(_) => continue,
            }
        }
    }
    None
}
