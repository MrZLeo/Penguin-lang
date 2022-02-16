mod rt_util;
mod tree_node;

use crate::rt_util::*;
use lazy_static::lazy_static;
use lrlex::lrlex_mod;
use lrpar::lrpar_mod;
use regex::Regex;
use std::fs;
use std::io::{self, BufRead, ErrorKind, Write};

// Using `lrlex_mod!` brings the lexer for `calc.l` into scope. By default the
// module name will be `calc_l` (i.e. the file name, minus any extensions,
// with a suffix of `_l`).
lrlex_mod!("lexer.l");
// Using `lrpar_mod!` brings the parser for `calc.y` into scope. By default the
// module name will be `calc_y` (i.e. the file name, minus any extensions,
// with a suffix of `_y`).
lrpar_mod!("parser.y");

const VERSION: &str = "0.2.6";

lazy_static! {
    static ref EXIT: Vec<String> = {
        let v = vec!["exit".to_string(), "q".to_string(), "quit".to_string()];
        v
    };
    static ref SUFFIX: Regex = Regex::new("^.*\\.pg$").unwrap();
}

fn info() {
    println!(r"                                  _     ");
    println!(r"    ____  ___  ____  ____ ___  __(_)___ ");
    println!(r"   / __ \/ _ \/ __ \/ __ `/ / / / / __ \");
    println!(r"  / /_/ /  __/ / / / /_/ / /_/ / / / / /");
    println!(r" / .___/\___/_/ /_/\__, /\__,_/_/_/ /_/ ");
    println!(r"/_/               /____/                ");
    println!("Penguin compiler: version {}", VERSION);
}

fn main() {
    info();
    #[cfg(feature = "debug")]
    {
        println!("### debug mode is open. ###");
    }

    // create directory for graph that user will draw
    match fs::create_dir("graph") {
        Err(e) => match e.kind() {
            ErrorKind::AlreadyExists => {}
            _ => eprintln!("!{:?}", e),
        },
        Ok(_) => {
            println!("# Created directory `graph` for output.")
        }
    }

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
        #[cfg(feature = "debug")]
        {
            println!("# file is: {}", file);
        }
        if SUFFIX.is_match(&file) {
            crate::file(runtime, fs::read_to_string(file).unwrap().to_lowercase());
        } else {
            eprintln!("Error: file format is not support");
        }
    }
}

fn file(mut rt: RunTime, file: String) {
    #[cfg(feature = "debug")]
    {
        println!("# whole file:");
        println!("------------------------------");
        println!("{}", file);
        println!("------------------------------");
        println!("# program launch");
    }
    let file: String = file
        .split('\n')
        .filter(|x| !x.is_empty() && !x.starts_with("//") && !x.starts_with("--"))
        .map(|x| {
            &x[0..match x.find("//") {
                None => x.len(),
                Some(index) => index,
            }]
        })
        .map(|x| {
            &x[0..match x.find("--") {
                None => x.len(),
                Some(index) => index,
            }]
        })
        .collect();
    #[cfg(feature = "debug")]
    {
        println!("# file delete comment:");
        println!("------------------------------");
        println!("{}", file);
        println!("------------------------------");
    }

    // todo 文件中混杂exit怎么办？
    file.split(';')
        .into_iter()
        .map(|stat| stat.trim_start().trim_end())
        .filter(|stat| !stat.is_empty())
        .for_each(|stat| {
            #[cfg(feature = "debug")]
            {
                println!("# file statement: {}", stat);
                println!("# len of line: {}", stat.len());
            }
            let mut stat = stat.to_string();
            if !EXIT.contains(&stat) {
                stat = format!("{};", stat);
            }
            rt.run(stat.as_str());
        })
}

fn shell(mut rt: RunTime) {
    let stdin = io::stdin();
    let mut gl_input = String::new();
    let mut is_continue = false;
    loop {
        if is_continue {
            print!("... ");
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
                    None => l.len(),
                    Some(idx) => idx,
                }]
                    .to_string();
                let l = l[0..match l.find("--") {
                    None => l.len(),
                    Some(idx) => idx,
                }]
                    .to_string();

                // if is not the end of line, continue
                gl_input += &l.trim_end().to_lowercase();
                if !gl_input.ends_with(';') && !EXIT.contains(&gl_input) {
                    is_continue = true;
                    continue;
                }

                #[cfg(feature = "debug")]
                {
                    println!("global input: {}", gl_input);
                }

                for v in gl_input.split(';') {
                    if v.is_empty() {
                        break;
                    }

                    #[cfg(feature = "debug")]
                    {
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
            _ => break,
        }
    }
}
