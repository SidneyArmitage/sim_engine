use crate::ModValue;

#[derive(Clone, Copy)]
pub enum Shape {
    SQUARE,
    CIRCLE,
}

#[derive(Clone, Copy)]
pub struct Drawable {
    pub x: f32,
    pub y: f32,
    pub shape: Shape,
}

pub fn shape_to_string(shape: &Shape) -> &'static str {
    match shape {
        Shape::SQUARE => "SQUARE",
        Shape::CIRCLE => "CIRCLE"
    }
}

pub fn draw(id: &isize, value: &ModValue) -> ModValue {
    let item = value.draw.unwrap();
    println!("x: {}, y: {}, shape: {}", item.x, item.y, shape_to_string(&item.shape));
    *value
}