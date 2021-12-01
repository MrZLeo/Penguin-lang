%start Stat
%%
Stat -> Result<DrawableKind<'input>, ()>:
      'ORIGIN' 'IS' Origin 'SEMICOLON' { $3 }
    | 'ROT' 'IS' Rot 'SEMICOLON' { $3 }
    | 'SCALE' 'IS' Scale 'SEMICOLON' { $3 }
    | 'FOR' DrawFor 'SEMICOLON' { $2 }
    | 'EXIT' { Ok(DrawableKind::Exit) }
    ;

Origin -> Result<DrawableKind<'input>, ()>:
    'LB' Expr 'COMMA' Expr 'RB' {
      Ok(DrawableKind::Origin($2?, $4?))
    }
    ;

Rot -> Result<DrawableKind<'input>, ()>:
    Expr {
      Ok(DrawableKind::Rot($1?))
    }
    ;

Scale -> Result<DrawableKind<'input>, ()>:
    'LB' Expr 'COMMA' Expr 'RB' {
      Ok(DrawableKind::Scale($2?, $4?))
    }
    ;

DrawFor -> Result<DrawableKind<'input>, ()>:
    Alphabet 'FROM' Expr 'TO' Expr 'STEP' Expr 'DRAW' 'LB' Alphabet 'COMMA' Alphabet 'RB'
        {
            Ok(DrawableKind::DrawableFor(
                ForStruct {
                    ch: $1?,
                    from: $3?,
                    to: $5?,
                    step: $7?,
                    x: $10?,
                    y: $12?,
                }
            ))
        }
    ;

Alphabet -> Result<&'input str, ()>:
      'ALPHABET'
        {
            let v = $1.map_err(|_| ())?;
            Ok($lexer.span_str(v.span()))
        }
    | 'FLOAT'
        {
           let v = $1.map_err(|_| ())?;
           Ok($lexer.span_str(v.span()))
        }
    ;

Expr -> Result<f64, ()>:
      Expr 'PLUS' Term { Ok($1? + $3?) }
    | Expr 'MINUS' Term { Ok($1? - $3?) }
    | Term { $1 }
    ;

Term -> Result<f64, ()>:
      Term 'MUL' Factor { Ok($1? * $3?) }
    | Term 'DIV' Factor { Ok($1? / $3?) }
    | Factor { $1 }
    ;

Factor -> Result<f64, ()>:
      'LB' Expr 'RB' { $2 }
    | 'FLOAT'
      {
          let v = $1.map_err(|_| ())?;
          parse_float($lexer.span_str(v.span()))
      }
    ;
%%
// Any functions here are in scope for all the grammar actions above.
use crate::rt_util::*;

fn parse_float(s: &str) -> Result<f64, ()> {
    match s.parse::<f64>() {
        Ok(val) => {
            Ok(val)
        },
        Err(_) => {
            eprintln!("{} cannot be represented as a f64", s);
            Err(())
        }
    }
}
