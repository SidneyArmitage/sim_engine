

use crate::drawable::{Drawable, Shape};
use std::collections::HashMap;

pub mod drawable;

struct Control {
    drawable: HashMap<isize, Drawable>
}

fn sim_round(control: Control) {
    for (_, drawable) in control.drawable.into_iter() {
        drawable::draw(&drawable);
    }
}

fn main() {
    println!("Hello, world!");
    let mut map = HashMap::new();
    map.insert(0, Drawable {
        x: 1.,
        y: 1.,
        shape: Shape::SQUARE,
    });
    map.insert(1, Drawable {
        x: 1.,
        y: 1.,
        shape: Shape::CIRCLE,
    });
    let control = Control {
        drawable: map,
    };
    sim_round(control);
}
