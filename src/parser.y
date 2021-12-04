%start Stat
%%
Stat -> Result<DrawableKind, ()>:
      'ORIGIN' 'IS' Origin 'SEMICOLON' { $3 }
    | 'ROT' 'IS' Rot 'SEMICOLON' { $3 }
    | 'SCALE' 'IS' Scale 'SEMICOLON' { $3 }
    | 'FOR' DrawFor 'SEMICOLON' { $2 }
    | 'EXIT' { Ok(DrawableKind::Exit) }
    ;

Origin -> Result<DrawableKind, ()>:
    'LB' Expr 'COMMA' Expr 'RB' {
      Ok(DrawableKind::Origin($2?, $4?))
    }
    ;

Rot -> Result<DrawableKind, ()>:
    Expr {
      Ok(DrawableKind::Rot($1?))
    }
    ;

Scale -> Result<DrawableKind, ()>:
    'LB' Expr 'COMMA' Expr 'RB' {
      Ok(DrawableKind::Scale($2?, $4?))
    }
    ;

DrawFor -> Result<DrawableKind, ()>:
    'T' 'FROM' Expr 'TO' Expr 'STEP' Expr 'DRAW' 'LB' TreeE 'COMMA' TreeE 'RB'
        {
            Ok(DrawableKind::DrawableFor(
                ForStruct {
                    from: $3?,
                    to: $5?,
                    step: $7?,
                    x: $10?.unwrap(),
                    y: $12?.unwrap(),
                }
            ))
        }
    ;

TreeE -> Result<Option<Box<TreeNode>>, ()>:
      TreeE 'PLUS' TreeT
        {
            let v = $2.map_err(|_| ())?;
            Ok(Some(Box::new(TreeNode {
                val: $lexer.span_str(v.span()).to_string(),
                left: $1?,
                right: $3?,
            })))
        }
    | TreeE 'MINUS' TreeT
        {
            let v = $2.map_err(|_| ())?;
            Ok(Some(Box::new(TreeNode {
                val: $lexer.span_str(v.span()).to_string(),
                left: $1?,
                right: $3?,
            })))
        }
    | TreeT { $1 }
    ;

TreeT -> Result<Option<Box<TreeNode>>, ()>:
      TreeT 'MUL' TreeF
        {
            let v = $2.map_err(|_| ())?;
            Ok(Some(Box::new(TreeNode {
                val: $lexer.span_str(v.span()).to_string(),
                left: $1?,
                right: $3?,
            })))
        }
    | TreeT 'DIV' TreeF
        {
            let v = $2.map_err(|_| ())?;
            Ok(Some(Box::new(TreeNode {
                val: $lexer.span_str(v.span()).to_string(),
                left: $1?,
                right: $3?,
            })))
        }
    | TreeF { $1 }
    ;

TreeF -> Result<Option<Box<TreeNode>>, ()>:
      'PLUS' TreeF { Ok($2?) }
    | 'MINUS' TreeF
        {
            let v = $1.map_err(|_| ())?;
            Ok(Some(Box::new(TreeNode {
                val: $lexer.span_str(v.span()).to_string(),
                left: None,
                right: $2?,
            })))
        }
    | TreeC { $1 }
    ;

TreeC -> Result<Option<Box<TreeNode>>, ()>:
      TreeA 'POWER' TreeC
        {
            let v = $2.map_err(|_| ())?;
            Ok(Some(Box::new(TreeNode {
                val: $lexer.span_str(v.span()).to_string(),
                left: $1?,
                right: $3?,
            })))
        }
    | TreeA { $1 }
    ;

TreeA -> Result<Option<Box<TreeNode>>, ()>:
      'LB' TreeE 'RB' { $2 }
    | 'FUNC' 'LB' TreeE 'RB'
        {
            let v = $1.map_err(|_| ())?;
            Ok(Some(Box::new(TreeNode {
                val: $lexer.span_str(v.span()).to_string(),
                left: $3?,
                right: None,
            })))
        }
    | 'T'
        {
            let v = $1.map_err(|_| ())?;
            Ok(Some(Box::new(TreeNode {
                val: $lexer.span_str(v.span()).to_string(),
                left: None,
                right: None,
            })))
        }
    | 'CONST'
        {
            let v = $1.map_err(|_| ())?;
            Ok(Some(Box::new(TreeNode {
                val: $lexer.span_str(v.span()).to_string(),
                left: None,
                right: None,
            })))
        }
    | 'FLOAT'
        {
           let v = $1.map_err(|_| ())?;
           Ok(Some(Box::new(TreeNode {
               val: $lexer.span_str(v.span()).to_string(),
               left: None,
               right: None,
           })))
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
      'PLUS' Factor { Ok($2?) }
    | 'MINUS' Factor { Ok(-$2?) }
    | Component { $1 }
    ;

Component -> Result<f64, ()>:
      Atom 'POWER' Component
        {
          Ok($1?.powf($3?))
        }
    | Atom { $1 }
    ;

Atom -> Result<f64, ()>:
      'LB' Expr 'RB' { $2 }
    | 'FLOAT'
      {
          let v = $1.map_err(|_| ())?;
          parse_float($lexer.span_str(v.span()))
      }
    | 'CONST' {
        let v = $1.map_err(|_| ())?;
        match $lexer.span_str(v.span()) {
            "e" => Ok(std::f64::consts::E),
            "pi" => Ok(std::f64::consts::PI),
            _ => Err(()),
        }
    }
    ;

%%
// Any functions here are in scope for all the grammar actions above.
use crate::rt_util::*;
use crate::tree_node::TreeNode;

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
