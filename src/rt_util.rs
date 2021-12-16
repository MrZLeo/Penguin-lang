use crate::tree_node;
use crate::tree_node::TreeNode;
use lazy_static::lazy_static;
use lrlex::{lrlex_mod, DefaultLexeme};
use lrpar::lrpar_mod;
use plotters::prelude::*;
use std::process::exit;
lrlex_mod!("lexer.l");
lrpar_mod!("parser.y");

lazy_static! {
    static ref LEXER_DEF: lrlex::LRNonStreamingLexerDef<DefaultLexeme, u32> = lexer_l::lexerdef();
    static ref PIC_NUM: u32 = 0;
}

#[derive(Debug)]
pub enum DrawableKind {
    DrawableFor(ForStruct),
    Rot(f64),
    Origin(f64, f64),
    Scale(f64, f64),
    Show,
    Exit,
    XRange(f64, f64),
    YRange(f64, f64),
    DotSize(f64),
    Color(String),
}

#[derive(Debug)]
pub struct ForStruct {
    pub from: f64,
    pub to: f64,
    pub step: f64,
    pub x: Box<TreeNode>,
    pub y: Box<TreeNode>,
}

pub struct RunTime {
    origin: (f64, f64),
    rot: f64,
    scale: (f64, f64),
    graph: Vec<(ForStruct, RunTime)>,
    x_range: (f64, f64),
    y_range: (f64, f64),
    size: f64,
    color: String,
    pic_num: u32,
}

impl RunTime {
    pub fn new() -> Self {
        RunTime {
            origin: (0.0, 0.0),
            rot: 0.0,
            scale: (1.0, 1.0),
            graph: Vec::new(),
            x_range: (0.0, 10.0),
            y_range: (-4.0, 4.0),
            size: 2.0,
            color: "blue".to_string(),
            pic_num: 0,
        }
    }

    // use for snapshot
    fn from(origin: (f64, f64), rot: f64, scale: (f64, f64), size: f64, color: String) -> Self {
        RunTime {
            origin,
            rot,
            scale,
            graph: Vec::new(),
            x_range: (0.0, 0.0),
            y_range: (0.0, 0.0),
            size,
            color,
            pic_num: 0,
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

    pub fn set_x_range(&mut self, x_range: (f64, f64)) {
        self.x_range = x_range;
    }

    pub fn set_y_range(&mut self, y_range: (f64, f64)) {
        self.y_range = y_range;
    }

    pub fn set_size(&mut self, size: f64) {
        self.size = size;
    }

    pub fn set_color(&mut self, color: String) {
        self.color = color;
    }

    pub fn for_draw(&mut self, stat: ForStruct) {
        self.graph.push((
            stat,
            RunTime::from(
                self.origin,
                self.rot,
                self.scale,
                self.size,
                self.color.clone(),
            ),
        ));
    }

    pub fn show(&mut self) {
        let file_name = format!("graph/{}.png", self.pic_num);
        let root = BitMapBackend::new(file_name.as_str(), (1024, 1024)).into_drawing_area();

        root.fill(&WHITE).unwrap();
        let mut chart = ChartBuilder::on(&root)
            .margin(60)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(
                self.x_range.0 as f32..self.x_range.1 as f32,
                self.y_range.0 as f32..self.y_range.1 as f32,
            )
            .unwrap();

        chart.configure_mesh().draw().unwrap();

        self.graph.iter().for_each(|(stat, rt)| {
            #[cfg(feature = "debug")]
            {
                println!("### show {} ###", &file_name);
                println!("--------------------");
                println!("origin: ({}, {})", rt.origin.0, rt.origin.1);
                println!("scale: ({},{})", rt.scale.0, rt.scale.1);
                println!("rot: {}", rt.rot);
                println!("size: {}", rt.size);
                println!("color: {}", rt.color);
                println!("x: [{}, {}]", self.x_range.0, self.x_range.1);
                println!("y: [{}, {}]", self.y_range.0, self.y_range.1);
                println!("--------------------");
            }
            let from = f64::max(stat.from, self.x_range.0);
            let to = f64::min(stat.to, self.x_range.1);
            chart
                .draw_series(PointSeries::of_element(
                    (from as f32..(to + stat.step) as f32)
                        .step(stat.step as f32)
                        .values()
                        .map(|v| {
                            rt.process_data(
                                tree_node::eval(&stat.x, v as f64),
                                tree_node::eval(&stat.y, v as f64),
                            )
                        })
                        .filter(|(x, y)| {
                            y.to_owned() as f64 <= self.y_range.1
                                && y.to_owned() as f64 >= self.y_range.0
                                && x.to_owned() as f64 >= self.x_range.0
                                && x.to_owned() as f64 <= self.x_range.1
                        }),
                    rt.size,
                    match rt.color.as_str() {
                        "blue" => ShapeStyle::from(&BLUE).filled(),
                        "red" => ShapeStyle::from(&RED).filled(),
                        "green" => ShapeStyle::from(&GREEN).filled(),
                        "black" => ShapeStyle::from(&BLACK).filled(),
                        "yellow" => ShapeStyle::from(&YELLOW).filled(),
                        "cyan" => ShapeStyle::from(&CYAN).filled(),
                        "magenta" => ShapeStyle::from(&MAGENTA).filled(),
                        _ => ShapeStyle::from(&TRANSPARENT).filled(),
                    },
                    &|coord, size, style| {
                        EmptyElement::at(coord) + Circle::new((0, 0), size, style)
                    },
                ))
                .unwrap();
        });
        println!("Draw success in {}", file_name);
        self.graph.clear();
        self.pic_num += 1;
    }

    fn process_data(&self, x: f64, y: f64) -> (f32, f32) {
        // scale
        let mut x = x * self.scale.0;
        let mut y = y * self.scale.1;

        // rotation
        //     temp=local_x*cos(Rot_angle)+local_y*sin(Rot_angle);
        //     local_y=local_y*cos(Rot_angle)-local_x*sin(Rot_angle);
        //     local_x = temp;
        let temp = x * self.rot.cos() + y * self.rot.sin();
        y = y * self.rot.cos() - x * self.rot.sin();
        x = temp;

        // translation
        x += self.origin.0;
        y += self.origin.1;

        (x as f32, y as f32)
    }

    pub fn run(&mut self, l: &str) {
        let lexer = LEXER_DEF.lexer(l);
        // Pass the lexer to the parser and lex and parse the input.
        let (res, errs) = parser_y::parse(&lexer);
        for e in errs {
            println!("{}", e.pp(&lexer, &parser_y::token_epp));
        }
        match res {
            Some(r) => {
                if cfg!(feature = "debug") {
                    println!("Result: {:#?}", r);
                }
                if let Ok(r) = r {
                    match r {
                        DrawableKind::Rot(r) => self.set_rot(r),
                        DrawableKind::Scale(x, y) => self.set_scale((x, y)),
                        DrawableKind::Origin(x, y) => self.set_origin((x, y)),
                        DrawableKind::DrawableFor(x) => self.for_draw(x),
                        DrawableKind::Show => self.show(),
                        DrawableKind::XRange(l, r) => self.set_x_range((l, r)),
                        DrawableKind::YRange(l, r) => self.set_y_range((l, r)),
                        DrawableKind::DotSize(size) => self.set_size(size),
                        DrawableKind::Color(color) => self.set_color(color),
                        DrawableKind::Exit => exit(0),
                    }
                } else {
                    println!("Illegal command");
                }
            }
            _ => eprintln!("Unable to evaluate expression."),
        }
    }
}
