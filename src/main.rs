mod rendering;
mod game;
use crossterm::Result;
use rendering::{DrawColor, DrawScreen};
use game::GameBoard;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

fn main() -> Result<()> {
    let screen_width = 150;
    let screen_height = 40;
    let game_board_width = 80;
    let game_board_height = screen_height - 2;
    let game_board_start_position = (screen_width / 2 - game_board_width / 2, 1);
    let mut draw_screen = DrawScreen::new(screen_width, screen_height);
    let mut game_board = GameBoard::new(game_board_start_position, 80, game_board_height, &mut draw_screen);
    let desired_fps: f64 = 10.0;
    let time_per_frame: f64 = 1.0 / desired_fps;
    let signal_capture = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&signal_capture))?;
    let mut last_time = Instant::now();
    while !signal_capture.load(Ordering::Relaxed) {
        game_board.update(&mut draw_screen);
        draw_screen.draw();
        let wait_time = time_per_frame - last_time.elapsed().as_secs_f64();
        if wait_time > 0.0 {
            thread::sleep(Duration::from_secs_f64(wait_time));
        }
        let elapsed_time = last_time.elapsed();
        let fps = 1.0 / elapsed_time.as_secs_f32();
        let fps_text = format!("fps {:.2}", fps);
        draw_screen.update_with_string(screen_width - (fps_text.len() + 1) as u16, 1, fps_text, DrawColor::White);
        last_time = Instant::now();
    }
    Ok(())
}
