use std::f64::consts::E;
use std::f64::consts::PI;
use cfgrammar::RIdx;
use plotters::prelude::*;
use crate::tree_node::TreeNode;
use crate::tree_node;


#[derive(Debug)]
pub enum DrawableKind {
    DrawableFor(ForStruct),
    Rot(f64),
    Origin(f64, f64),
    Scale(f64, f64),
    Exit,
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
            .build_cartesian_2d(0f32..10f32, -4f32..4f32).unwrap();

        chart.configure_mesh().draw().unwrap();

        chart.draw_series(PointSeries::of_element(
            (stat.from as f32..stat.to as f32)
                .step(stat.step as f32)
                .values()
                .map(|v| {
                    self.process_data(tree_node::eval(&stat.x, v as f64),
                                      tree_node::eval(&stat.y, v as f64))
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

    fn process_data(&self, x: f64, y: f64) -> (f32, f32) {
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
