mod rt_util;
mod tree_node;

use std::fs;
use std::io::{self, BufRead, Write};
use lazy_static::lazy_static;
use regex::Regex;
use lrlex::lrlex_mod;
use lrpar::lrpar_mod;
use crate::rt_util::*;

// Using `lrlex_mod!` brings the lexer for `calc.l` into scope. By default the
// module name will be `calc_l` (i.e. the file name, minus any extensions,
// with a suffix of `_l`).
lrlex_mod!("lexer.l");
// Using `lrpar_mod!` brings the parser for `calc.y` into scope. By default the
// module name will be `calc_y` (i.e. the file name, minus any extensions,
// with a suffix of `_y`).
lrpar_mod!("parser.y");

const VERSION: &str = "0.2.3";

lazy_static!(
    static ref EXIT: Vec<String> = {
        let mut v = Vec::new();
        v.push("exit".to_string());
        v.push("q".to_string());
        v.push("quit".to_string());
        v
    };
    static ref SUFFIX: Regex = Regex::new("^.*\\.pg$").unwrap();
);

fn info() {
    println!(r"                                  _     ");
    println!(r"    ____  ___  ____  ____ ___  __(_)___ ");
    println!(r"   / __ \/ _ \/ __ \/ __ `/ / / / / __ \");
    println!(r"  / /_/ /  __/ / / / /_/ / /_/ / / / / /");
    println!(r" / .___/\___/_/ /_/\__, /\__,_/_/_/ /_/ ");
    println!(r"/_/               /____/                ");
}


fn main() {
    #[cfg(feature = "debug")] {
        println!("debug mode is open.");
    }

    info();

    let runtime = RunTime::new();

    let args = std::env::args();
    if args.len() < 2 {
        // interactive shell mode
        shell(runtime);
    } else {
        // read file and do the interpretation
        // second argument is the file name
        let mut file = String::new();
        for (index, arg) in args.enumerate() {
            if index == 1 {
                file = arg;
                break;
            }
        }
        println!("# file is: {}", file);
        if SUFFIX.is_match(&file) {
            crate::file(runtime, fs::read_to_string(file).unwrap().to_lowercase());
        } else {
            eprintln!("Error: file format is not support");
        }
    }
}

fn file(mut rt: RunTime, file: String) {
    println!("# whole file: \n{}", file);
    println!("# program launch");
    let file: String = file
        .split("\n")
        .filter(
            |x| {
                x.len() > 0
                    && !x.starts_with("//")
                    && !x.starts_with("--")
            }
        )
        .map(|x| {
            &x[0..match x.find("//") {
                None => { x.len() }
                Some(index) => { index }
            }]
        })
        .map(|x| {
            &x[0..match x.find("--") {
                None => { x.len() }
                Some(index) => { index }
            }]
        })
        .collect();
    println!("# file delete comment: \n{}", file);

    file.split(";")
        .into_iter()
        .filter(|stat| {
            stat.len() > 0
        })
        .for_each(
            |stat| {
                #[cfg(feature = "debug")] {
                    println!("# file statement: {}", stat);
                }
                let mut stat = stat.to_string();
                if !EXIT.contains(&stat) {
                    stat = format!("{};", stat);
                }
                rt.run(stat.as_str());
            }
        )
}

fn shell(mut rt: RunTime) {
    // basic information
    println!("Penguin compiler: version {}", VERSION);

    // Get the `LexerDef` for the `drawing` language.
    // let lexerdef = parser_lexer_l::lexerdef();
    let stdin = io::stdin();
    let mut gl_input = String::new();
    let mut is_continue = false;
    loop {
        if is_continue {
            print!("...");
        } else {
            print!(">>> ");
        }
        io::stdout().flush().ok();
        match stdin.lock().lines().next() {
            Some(Ok(ref l)) => {
                if l.trim().is_empty() {
                    continue;
                }
                if l.starts_with("//") || l.starts_with("--") {
                    continue;
                }
                // Now we create a lexer with the `lexer` method with which
                // we can lex an input.
                let l = l[0..match l.find("//") {
                    None => { l.len() }
                    Some(idx) => { idx }
                }].to_string();
                let l = l[0..match l.find("--") {
                    None => { l.len() }
                    Some(idx) => { idx }
                }].to_string();

                // if is not the end of line, continue
                gl_input += &l.trim_end().to_lowercase();
                if !gl_input.ends_with(";") && !EXIT.contains(&gl_input) {
                    is_continue = true;
                    continue;
                }

                #[cfg(feature = "debug")] {
                    println!("global input: {}", gl_input);
                }

                for v in gl_input.split(";") {
                    if v.len() <= 0 {
                        break;
                    }

                    #[cfg(feature = "debug")] {
                        println!("v: {}", v);
                        println!("v's len: {}", v.len());
                    }

                    let mut v = String::from(v);
                    if !EXIT.contains(&v.to_string()) {
                        v = format!("{};", v);
                    }

                    rt.run(v.as_str());
                }

                // prepare for new input
                gl_input.clear();
                is_continue = false;
            }
            _ => break
        }
    }
}
