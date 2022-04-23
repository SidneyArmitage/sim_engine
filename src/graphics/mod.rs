use sdl2::{self, EventPump};
use std::{
  sync::{mpsc::{self, Sender, TryRecvError, Receiver, SendError}},
  thread::{spawn},
};
pub struct App {
  tx: Sender<()>,
}

fn start (rx: &Receiver<()>, event_pump: &mut EventPump) {
  'main: loop {
    match rx.try_recv() {
        Ok(_) | Err(TryRecvError::Disconnected) => {
            println!("Terminating.");
            break 'main;
        }
        Err(TryRecvError::Empty) => {}
    }
    for event in event_pump.poll_iter() {
        match event {
            sdl2::event::Event::Quit {..} => {
              print!("quit message received");
              break 'main
            },
            _ => {},
        }
    }
  }
}

impl App {
  pub fn new() -> Self {
    let (tx, rx) = mpsc::channel::<()>();
    spawn(move || {
      let sdl = sdl2::init().unwrap();
      let video_subsystem = sdl.video().unwrap();
      let window = video_subsystem
        .window("test", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();
      let mut event_pump = sdl.event_pump().unwrap();
      let gl_context = window.gl_create_context().unwrap();
      let gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
      start(&rx, &mut event_pump)
    });
    Self {
      tx,
    }
  }

  pub fn close(&mut self) -> Result<(), SendError<()>> {
    self.tx.send(())
  }
}

#[cfg(test)]
mod tests {

use super::*;

  #[test]
  fn testSDL2OpensWindowClosesWindow () {
    let mut window = App::new();
    window.close().unwrap();
  }
}