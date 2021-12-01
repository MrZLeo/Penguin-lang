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

// We use Turtle crate to accomplish the drawing function
// but it has a origin in the central of screen
// in order to make it in the left and top of screen
// we need a factor to convert it
const HORIZONTAL: f64 = -320.0;
const PERPENDICULAR: f64 = 250.0;

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
        let root = BitMapBackend::new("graph/0.png", (960, 840))
            .into_drawing_area();
        root.fill(&WHITE).unwrap();
        let mut chart = ChartBuilder::on(&root)
            .margin(10)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(0f32..300f32, 0f32..300f32).unwrap();

        chart.configure_mesh().draw().unwrap();

        // let root = BitMapBackend::new("plotters-doc-data/5.png", (640, 480)).into_drawing_area();
        // root.fill(&WHITE);
        // let root = root.margin(10, 10, 10, 10);
        // // After this point, we should be able to draw construct a chart context
        // let mut chart = ChartBuilder::on(&root)
        //     // Set the caption of the chart
        //     .caption("This is our first plot", ("sans-serif", 40).into_font())
        //     // Set the size of the label region
        //     .x_label_area_size(20)
        //     .y_label_area_size(40)
        //     // Finally attach a coordinate on the drawing area and make a chart context
        //     .build_cartesian_2d(0f32..10f32, 0f32..10f32)?;
        //
        // // Then we can draw a mesh
        // chart
        //     .configure_mesh()
        //     // We can customize the maximum number of labels allowed for each axis
        //     .x_labels(5)
        //     .y_labels(5)
        //     // We can also change the format of the label text
        //     .y_label_formatter(&|x| format!("{:.3}", x))
        //     .draw()?;

        // chart
        //     .draw_series(PointSeries::of_element(
        //         (stat.from as f32..stat.to as f32)
        //             .step(stat.step as f32)
        //             .values()
        //             .map(
        //                 |x| {
        //                     // let lexer = lexerdef.lexer(stat.x);
        //                     // let (x, errs) = func_y::parse(&lexer);
        //                     // for e in errs {
        //                     //     println!("{}", e.pp(&lexer, &func_y::token_epp));
        //                     // }
        //                     //
        //                     // let lexer = lexerdef.lexer(stat.y);
        //                     // let (y, errs) = func_y::parse(&lexer);
        //                     // for e in errs {
        //                     //     println!("{}", e.pp(&lexer, &func_y::token_epp));
        //                     // }
        //                     //
        //                     // (x.unwrap(), y.unwrap())
        //                     (x, x)
        //                 }
        //             ),
        //         5,
        //         &RED,
        //         &|coord, size, style| {
        //             EmptyElement::at(coord)
        //                 + Circle::new((0, 0), size, style)
        //                 + Text::new(format!("{:?}", coord), (0, 15), ("sans-serif", 15))
        //         },
        //     )).unwrap();
        let lexerdef = func_l::lexerdef();

        chart.draw_series(LineSeries::new(
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
                            (Eval::new(stat.x).eval(&x, v), Eval::new(stat.y).eval(&y, v))
                        } else {
                            unreachable!()
                        }
                    } else {
                        unreachable!()
                    }
                }),
            &BLUE,
        )).unwrap();
    }

    // todo: <11.30> 1. comment; 2. for statement
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
                    if let Node::Term = nodes[0] {
                        // self.s[lexeme.span().start()..lexeme.span().end()]
                        //     .parse()
                        //     .unwrap()
                        x
                    } else {
                        unreachable!();
                    }
                } else if let Node::Term { lexeme } = nodes[0] {
                    match &self.s[lexeme.span().start()..lexeme.span().end()] {
                        "sin" => x.sin(),
                        "cos" => x.cos(),
                        "tan" => x.tan(),
                        "ln" => x.ln(),
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