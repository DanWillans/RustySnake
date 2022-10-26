pub mod rendering;
use rendering::{DrawScreen, DrawColor, DrawPixel};
use std::io::{stdout, Write, Error};
use std::ops::Index;
use std::thread;
use std::time::{Instant, Duration};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use crossterm::{
    execute, queue,
    style::{self, Stylize}, cursor, terminal, Result
};

fn main() -> Result<()> {
  let mut draw_screen = DrawScreen::new(150, 40);
  let desired_fps : f32 = 10.0;
  let time_per_frame : u128 = 1000 / desired_fps;
  let signal_capture = Arc::new(AtomicBool::new(false));
  signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&signal_capture))?;
  let mut last_time = Instant::now();
  while !signal_capture.load(Ordering::Relaxed){
    for y in 0..40 {
      for x in 0..150 {
        if (y == 0 || y == 40 - 1) || (x == 0 || x == 150 - 1) {
          // in this loop we are more efficient by not flushing the buffer.
          draw_screen.update(x, y, '=', DrawColor::Green);
        }
      }
    }
    draw_screen.draw();
    let wait_time = time_per_frame - last_time.elapsed().as_millis();
    if wait_time > 0 {
        thread::sleep(Duration::as_millis(&Duration::from_millis(wait_time)));
    }
    let elapsed_time = last_time.elapsed();
    let fps = 1.0 / elapsed_time.as_secs_f32();
    draw_screen.update_with_string(20, 10, format!("fps: {:.2}",fps), DrawColor::White);
    last_time = Instant::now();
  }
  Ok(())
}
