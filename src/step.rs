use crate::Drawable;
use crate::ModValue;

pub fn down(id: &isize, value: &ModValue) -> ModValue {
    let draw = value.draw.unwrap();
    ModValue {
        down: Some(()),
        draw: Some ( Drawable {
            y: draw.y - 1.,
            x: draw.x,
            shape: draw.shape,
        }),
    }
}