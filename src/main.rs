#![feature(slice_patterns)]

use std::env;
use std::process::{Command, Output};

fn start(command: &'static str, args: Vec<String>, expect: &'static str) -> Output {
    return Command::new(command).args(args).output().expect(expect);
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    match args.as_slice() {
        [command, rest..] => {
            let output = match command.as_ref() {
                "list" => start("nix-env", vec!["-q".to_string()], "install failed"),
                "search" => {
                    let search_string = format!("{}", rest.join(" "));
                    start(
                        "nix-env",
                        vec!["-qa".to_string(), search_string],
                        "failed to execute process",
                    )
                }
                "install" => {
                    let mut args = vec!["-i".to_string()];
                    args.append(&mut rest.to_vec());
                    start("nix-env", args, "install failed")
                }
                "uninstall" => {
                    let mut args = vec!["--uninstall".to_string()];
                    args.append(&mut rest.to_vec());
                    start("nix-env", args, "install failed")
                }
                _ => Command::new(command)
                    .args(rest)
                    .output()
                    .expect("command not found"),
            };
            println!("{}", String::from_utf8_lossy(&output.stdout));
            println!("{}", String::from_utf8_lossy(&output.stderr));
        }

        _ => println!("{}", "lol"),
    }
}
