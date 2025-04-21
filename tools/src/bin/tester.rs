#![allow(non_snake_case)]

use std::{io::Read, process::Stdio};
use tools::*;

fn main() {
    if std::env::args().len() < 2 {
        eprintln!("Usage: {} <command> [<args>...]", std::env::args().nth(0).unwrap());
        return;
    }
    let (command, args) = (std::env::args().nth(1).unwrap(), std::env::args().skip(2).collect::<Vec<_>>());
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let input = parse_input(&input);

    let mut p = std::process::Command::new(command.clone())
        .args(args.clone())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| {
            eprintln!("failed to execute the command");
            eprintln!("{}", e);
            std::process::exit(1)
        });
    match exec(&mut p, true, &input, false) {
        Ok(score) => {
            eprintln!("Score = {}", score);
        }
        Err(err) => {
            if let Ok(Some(status)) = p.try_wait() {
                if !status.success() {
                    std::process::exit(1);
                }
            }
            let _ = p.kill();
            eprintln!("{}", err);
            eprintln!("Score = 0");
        }
    }
    let mut p = std::process::Command::new(command)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| {
            eprintln!("failed to execute the command");
            eprintln!("{}", e);
            std::process::exit(1)
        });
    match exec(&mut p, true, &input, true) {
        Ok(score) => {
            eprintln!("Score for ground truth = {}", score);
        }
        Err(err) => {
            if let Ok(Some(status)) = p.try_wait() {
                if !status.success() {
                    std::process::exit(1);
                }
            }
            let _ = p.kill();
            eprintln!("{}", err);
            eprintln!("Score for ground truth = 0");
        }
    }
}
