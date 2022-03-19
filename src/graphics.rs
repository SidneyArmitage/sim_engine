extern crate piston;
extern crate opengl_graphics;

use std::path::Path;
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use opengl_graphics::*;
use piston_window::PistonWindow;
use piston_window::{Graphics, Transformed, Rectangle, clear};
use piston_window::triangulation::{ty, tx};

pub fn run() {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("opengl_graphics: colored_image_test", [300, 300])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();
    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {

                let transform = c.transform.trans(0.0, 0.0);

                let tr = |p: [f64; 2]| [tx(transform, p[0], p[1]), ty(transform, p[0], p[1])];

                clear([1.0; 4], g);
                Rectangle::new([1.0, 0.0, 0.0, 1.0])
                    .draw([0.0, 0.0, 100.0, 100.0], &c.draw_state, c.transform, g);
                Rectangle::new([0.0, 1.0, 0.0, 0.3])
                    .draw([50.0, 50.0, 100.0, 100.0], &c.draw_state, c.transform, g);
            });
        }
    }
}