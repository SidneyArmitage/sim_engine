use opengl_graphics::GlGraphics;
use piston_window::{Rectangle, CircleArc, Context};

use crate::example::shapes::ModValue;

#[derive(Clone, Copy)]
pub enum Shape {
    SQUARE,
    CIRCLE,
}

#[derive(Clone, Copy)]
pub struct Drawable {
    pub x: f64,
    pub y: f64,
    pub shape: Shape,
}

pub fn shape_to_string(shape: &Shape) -> &'static str {
    match shape {
        Shape::SQUARE => "SQUARE",
        Shape::CIRCLE => "CIRCLE"
    }
}

pub fn draw(c: &Context, g: &mut GlGraphics, value: &ModValue) {
    let item = value.draw.unwrap();
    match item.shape {
        Shape::SQUARE => Rectangle::new([1.0, 0.0, 0.0, 1.0]).draw([item.x, item.y, 100.0, 100.0], &c.draw_state, c.transform, g),
        Shape::CIRCLE => CircleArc::new([0.0, 1.0, 0.0, 1.0], 1., 0., std::f64::consts::PI * 2.0).draw([item.x, item.y, 100.0, 100.0], &c.draw_state, c.transform, g)
    }
    ()
}