mod rt_util;
mod tree_node;

use std::io::{self, BufRead, Write};

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

const VERSION: &str = "0.1.3";

fn main() {
    // basic information
    println!("Drawing compiler: version {}", VERSION);
    let mut rt = RunTime::new();

    // Get the `LexerDef` for the `drawing` language.
    // let lexerdef = parser_lexer_l::lexerdef();
    let lexerdef = lexer_l::lexerdef();
    let stdin = io::stdin();
    loop {
        print!(">>> ");
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


                let l = l.to_lowercase();
                let lexer = lexerdef.lexer(l.as_str());
                // Pass the lexer to the parser and lex and parse the input.
                let (res, errs) = parser_y::parse(&lexer);
                for e in errs {
                    println!("{}", e.pp(&lexer, &parser_y::token_epp));
                }
                match res {
                    Some(r) => {
                        // todo: match kind of token here and execute the corresponding logic
                        println!("Result: {:#?}", r);
                        if let Ok(r) =  r {
                            match r {
                                DrawableKind::Rot(r) => rt.set_rot(r),
                                DrawableKind::Scale(x, y) => rt.set_scale((x, y)),
                                DrawableKind::Origin(x, y) => rt.set_origin((x, y)),
                                DrawableKind::DrawableFor(x) => rt.for_draw(x),
                                DrawableKind::Exit => break,
                            }
                        } else {
                            println!("Illegal command");
                            continue;
                        }
                    }
                    _ => eprintln!("Unable to evaluate expression.")
                }
            }
            _ => break
        }
    }
}
