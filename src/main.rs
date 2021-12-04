mod rt_util;
mod tree_node;

use std::fmt::format;
use std::io::{self, BufRead, Write};
use lazy_static::lazy_static;


use lrlex::lrlex_mod;
use lrpar::lrpar_mod;
use crate::DrawableKind::Exit;
use crate::rt_util::*;

// Using `lrlex_mod!` brings the lexer for `calc.l` into scope. By default the
// module name will be `calc_l` (i.e. the file name, minus any extensions,
// with a suffix of `_l`).
lrlex_mod!("lexer.l");
// Using `lrpar_mod!` brings the parser for `calc.y` into scope. By default the
// module name will be `calc_y` (i.e. the file name, minus any extensions,
// with a suffix of `_y`).
lrpar_mod!("parser.y");

const VERSION: &str = "0.1.3.1";

lazy_static! {
    static ref EXIT: Vec<String> = {
        let mut v = Vec::new();
        v.push("exit".to_string());
        v.push("q".to_string());
        v.push("quit".to_string());
        v
    };
}

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

    let args = std::env::args();
    if args.len() < 2 {
        // interactive shell mode
        shell();
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


        println!("file is :{}", file);
    }
}

fn shell() {
    // basic information
    println!("Penguin compiler: version {}", VERSION);
    let mut rt = RunTime::new();

    // Get the `LexerDef` for the `drawing` language.
    // let lexerdef = parser_lexer_l::lexerdef();
    let lexerdef = lexer_l::lexerdef();
    let stdin = io::stdin();
    let mut gl_input = String::new();
    let mut gl_backup = String::new();
    let mut is_continue = false;
    let mut shutdown = false;
    'label: loop {
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
                    gl_input += " ";
                    continue;
                }

                #[cfg(feature = "debug")] {
                    println!("global input: {}", gl_input);
                }
                gl_backup = gl_input.clone();

                for mut v in gl_input.split(";") {
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

                    let lexer = lexerdef.lexer(v.as_str());
                    // Pass the lexer to the parser and lex and parse the input.
                    let (res, errs) = parser_y::parse(&lexer);
                    for e in errs {
                        println!("{}", e.pp(&lexer, &parser_y::token_epp));
                    }
                    match res {
                        Some(r) => {
                            if cfg!(feature="debug") {
                                println!("Result: {:#?}", r);
                            }
                            if let Ok(r) = r {
                                match r {
                                    DrawableKind::Rot(r) => rt.set_rot(r),
                                    DrawableKind::Scale(x, y) => rt.set_scale((x, y)),
                                    DrawableKind::Origin(x, y) => rt.set_origin((x, y)),
                                    DrawableKind::DrawableFor(x) => rt.for_draw(x),
                                    DrawableKind::Show => rt.show(),
                                    DrawableKind::Exit => {
                                        break 'label;
                                    }
                                }
                            } else {
                                println!("Illegal command");
                            }
                        }
                        _ => eprintln!("Unable to evaluate expression.")
                    }
                }


                // prepare for new input
                gl_input.clear();
                is_continue = false;
            }
            _ => break
        }
    }
}
