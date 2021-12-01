use cfgrammar::yacc::{YaccKind, YaccOriginalActionKind};
use lrlex::CTLexerBuilder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    CTLexerBuilder::new()
        .lrpar_config(|ctp| {
            ctp.yacckind(YaccKind::Grmtools)
                .grammar_in_src_dir("parser.y")
                .unwrap()
        })
        .lexer_in_src_dir("lexer.l")?
        .build()?;


    CTLexerBuilder::new()
        .lrpar_config(|ctp| {
            ctp.yacckind(YaccKind::Original(YaccOriginalActionKind::GenericParseTree))
                .grammar_in_src_dir("func.y")
                .unwrap()
        })
        .lexer_in_src_dir("func.l")?
        .build()?;

    Ok(())
}

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     CTLexerBuilder::new()
//         .lrpar_config(|ctp| {
//             ctp.yacckind(YaccKind::Grmtools)
//                 .grammar_in_src_dir("parser.y")
//                 .unwrap()
//         })
//         .lexer_in_src_dir("lexer.l")?
//         .mod_name("parser_lexer")
//         .build()?;
//
//
//     CTLexerBuilder::new()
//         .lrpar_config(|ctp| {
//             ctp.yacckind(YaccKind::Original(YaccOriginalActionKind::GenericParseTree))
//                 .grammar_in_src_dir("func.y")
//                 .unwrap()
//         })
//         .lexer_in_src_dir("lexer.l")?
//         .mod_name("func_lexer")
//         .build()?;
//
//     Ok(())
// }

