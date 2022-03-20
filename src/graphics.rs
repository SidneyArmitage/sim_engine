extern crate piston;
extern crate opengl_graphics;

use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use opengl_graphics::*;
use piston_window::PistonWindow;
use piston_window::{Rectangle, clear};
use crate::{sim_round, Control};


pub fn run<T, G>(control: &mut Control<T, G>) {
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
                clear([1.0; 4], g);
                sim_round(control, &c, g);
            });
        }
    }
}