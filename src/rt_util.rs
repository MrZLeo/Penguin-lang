use std::f64::consts::E;
use std::f64::consts::PI;
use cfgrammar::RIdx;
use lrlex::{DefaultLexeme, lrlex_mod};
use lrpar::{Lexeme, lrpar_mod, Node};
use plotters::prelude::*;

lrpar_mod!("func.y");
lrlex_mod!("func.l");

#[derive(Debug)]
pub enum DrawableKind<'a> {
    DrawableFor(ForStruct<'a>),
    Rot(f64),
    Origin(f64, f64),
    Scale(f64, f64),
    Exit,
}

#[derive(Debug)]
pub struct ForStruct<'a> {
    pub ch: &'a str,
    pub from: f64,
    pub to: f64,
    pub step: f64,
    pub x: &'a str,
    pub y: &'a str,
}

pub struct RunTime {
    origin: (f64, f64),
    rot: f64,
    scale: (f64, f64),
}

pub enum Funcs {
    Cos(f64),
    Tan(f64),
    Sin(f64),
    Ln(f64),
}

impl RunTime {
    pub fn new() -> Self {
        RunTime {
            origin: (0.0, 0.0),
            rot: 0.0,
            scale: (1.0, 1.0),
        }
    }

    pub fn set_origin(&mut self, origin: (f64, f64)) {
        self.origin = origin;
    }

    pub fn set_rot(&mut self, rot: f64) {
        self.rot = rot;
    }

    pub fn set_scale(&mut self, scale: (f64, f64)) {
        self.scale = scale;
    }

    // todo: runtime capability: draw a picture :)
    pub fn for_draw(&mut self, stat: ForStruct) {
        let root = BitMapBackend::new("graph/0.png", (1024, 1024))
            .into_drawing_area();
        root.fill(&WHITE).unwrap();
        let mut chart = ChartBuilder::on(&root)
            .margin(60)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(stat.from as f32..stat.to as f32, -4f32..4f32).unwrap();

        chart.configure_mesh().draw().unwrap();

        let lexerdef = func_l::lexerdef();

        chart.draw_series(PointSeries::of_element(
            (stat.from as f32..stat.to as f32)
                .step(stat.step as f32)
                .values()
                .map(|v| {
                    let lexer = lexerdef.lexer(stat.x);
                    let (x, errs) = func_y::parse(&lexer);
                    for e in errs {
                        println!("{}", e.pp(&lexer, &func_y::token_epp));
                    }

                    let lexer = lexerdef.lexer(stat.y);
                    let (y, errs) = func_y::parse(&lexer);
                    for e in errs {
                        println!("{}", e.pp(&lexer, &func_y::token_epp));
                    }

                    if let Some(x) = x {
                        if let Some(y) = y {
                            self.process_data(Eval::new(stat.x).eval(&x, v), Eval::new(stat.y).eval(&y, v))
                        } else {
                            unreachable!()
                        }
                    } else {
                        unreachable!()
                    }
                }),
            2,
            ShapeStyle::from(&RED).filled(),
            &|coord, size, style| {
                EmptyElement::at(coord)
                    + Circle::new((0, 0), size, style)
            },
        )).unwrap();
        println!("Draw success in ./graph/0.png");
    }

    fn process_data(&self, x: f32, y: f32) -> (f32, f32) {
        // scale
        let mut x = x as f64 * self.scale.0;
        let mut y = y as f64 * self.scale.1;

        // rotation
        //     temp=local_x*cos(Rot_angle)+local_y*sin(Rot_angle);
        //     local_y=local_y*cos(Rot_angle)-local_x*sin(Rot_angle);
        //     local_x = temp;
        x = x * self.rot.cos() + y * self.rot.sin();
        y = y * self.rot.cos() - x * self.rot.sin();

        // translation
        x += self.origin.0;
        y += self.origin.1;

        (x as f32, y as f32)
    }
}

struct Eval<'a> {
    s: &'a str,
}

impl<'a> Eval<'a> {
    fn new(s: &'a str) -> Self {
        Eval { s }
    }

    fn eval(&self, n: &Node<DefaultLexeme<u32>, u32>, x: f32) -> f32 {
        match *n {
            Node::Nonterm {
                ridx: RIdx(ridx),
                ref nodes,
            } if ridx == func_y::R_E => {
                if nodes.len() == 1 {
                    self.eval(&nodes[0], x)
                } else {
                    if let Node::Term { lexeme } = nodes[1] {
                        if &self.s[lexeme.span().start()..lexeme.span().end()] == "+" {
                            debug_assert_eq!(nodes.len(), 3);
                            self.eval(&nodes[0], x) + self.eval(&nodes[2], x)
                        } else {
                            debug_assert_eq!(nodes.len(), 3);
                            self.eval(&nodes[0], x) - self.eval(&nodes[2], x)
                        }
                    } else {
                        unreachable!()
                    }
                }
            }
            Node::Nonterm {
                ridx: RIdx(ridx),
                ref nodes,
            } if ridx == func_y::R_T => {
                if nodes.len() == 1 {
                    self.eval(&nodes[0], x)
                } else {
                    if let Node::Term { lexeme } = nodes[1] {
                        if &self.s[lexeme.span().start()..lexeme.span().end()] == "*" {
                            debug_assert_eq!(nodes.len(), 3);
                            self.eval(&nodes[0], x) * self.eval(&nodes[2], x)
                        } else {
                            debug_assert_eq!(nodes.len(), 3);
                            self.eval(&nodes[0], x) / self.eval(&nodes[2], x)
                        }
                    } else {
                        unreachable!()
                    }
                }
            }
            Node::Nonterm {
                ridx: RIdx(ridx),
                ref nodes,
            } if ridx == func_y::R_F => {
                if nodes.len() == 1 {
                    if let Node::Term { lexeme } = nodes[0] {
                        // self.s[lexeme.span().start()..lexeme.span().end()]
                        //     .parse()
                        //     .unwrap()
                        x
                    } else {
                        unreachable!();
                    }
                } else if let Node::Term { lexeme } = nodes[0] {
                    let str = &self.s[lexeme.span().start()..lexeme.span().end()];
                    // println!("str: {}", str);
                    match str {
                        "sin" => x.sin(),
                        "cos" => x.cos(),
                        "tan" => x.tan(),
                        "ln" => x.ln(),
                        "exp" => x.exp(),
                        "sqrt" => x.sqrt(),
                        _ => unreachable!()
                    }
                } else {
                    debug_assert_eq!(nodes.len(), 3);
                    self.eval(&nodes[1], x)
                }
            }
            _ => unreachable!(),
        }
    }
}