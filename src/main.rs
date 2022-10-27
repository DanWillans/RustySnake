mod game;
mod rendering;
use crossterm::Result;
use device_query::{DeviceQuery, DeviceState, Keycode};
use game::{draw_title_screen, Direction, GameBoard};
use rendering::{DrawColor, DrawScreen};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

fn main() -> Result<()> {
    // Game constants
    let screen_width = 150;
    let screen_height = 40;
    let game_board_width = 80;
    let game_board_height = screen_height - 2;
    let game_board_start_position = (screen_width / 2 - game_board_width / 2, 1);

    // Setup the DrawScreen which will be used by other components
    let mut draw_screen = DrawScreen::new(screen_width, screen_height);

    // Draw title screen
    draw_title_screen((35, 17), &mut draw_screen);
    draw_screen.draw();

    // Setup input from keyboard handling
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let device_state = DeviceState::new();
        let mut last_key = &Keycode::Escape;
        loop {
            let keys: Vec<Keycode> = device_state.get_keys();
            if let Some(key) = keys.last() {
                if key == &Keycode::W && last_key != &Keycode::W {
                    tx.send(Keycode::W).unwrap();
                    last_key = &Keycode::W;
                } else if key == &Keycode::D && last_key != &Keycode::D {
                    tx.send(Keycode::D).unwrap();
                    last_key = &Keycode::D;
                } else if key == &Keycode::S && last_key != &Keycode::S {
                    tx.send(Keycode::S).unwrap();
                    last_key = &Keycode::S;
                } else if key == &Keycode::A && last_key != &Keycode::A {
                    tx.send(Keycode::A).unwrap();
                    last_key = &Keycode::A;
                } else if key == &Keycode::Enter && last_key != &Keycode::Enter {
                    tx.send(Keycode::Enter).unwrap();
                    last_key = &Keycode::Enter;
                } else if key == &Keycode::Space && last_key != &Keycode::Space {
                    tx.send(Keycode::Space).unwrap();
                    last_key = &Keycode::Space;
                }
            }
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Wait for space bar to be pressed
    loop {
        if let Ok(key) = rx.try_recv() {
            if key == Keycode::Space {
                break;
            }
        }
        thread::sleep(Duration::from_millis(1));
    }

    draw_screen.clear();

    // Setup the GameBoard
    let mut game_board = GameBoard::new(
        game_board_start_position,
        game_board_width,
        game_board_height,
        &mut draw_screen,
    );

    // Setup required parameters for FPS
    let desired_fps: f64 = 10.0;
    let time_per_frame: f64 = 1.0 / desired_fps;
    let mut last_time = Instant::now();

    // Setup signal handler to catch ctrl+c
    let signal_capture = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&signal_capture))?;

    // Loop until a signal has been captured
    while !signal_capture.load(Ordering::Relaxed) {
        // Check if input has been sent from the input thread. If it has then update the snake direction
        if let Ok(key) = rx.try_recv() {
            if key == Keycode::W {
                game_board.update_snake_direction(Direction::North);
            } else if key == Keycode::D {
                game_board.update_snake_direction(Direction::East);
            } else if key == Keycode::S {
                game_board.update_snake_direction(Direction::South);
            } else if key == Keycode::A {
                game_board.update_snake_direction(Direction::West);
            } else if key == Keycode::Enter {
                game_board = game_board.reset(&mut draw_screen);
            }
        }

        // Update the GameBoard. This moves and grows the snake, detects collisions and respawns food.
        game_board.update(&mut draw_screen);

        // Flush the queued changes to draw the screen
        draw_screen.draw();

        // Wait a specified amount of time to reach the desired FPS
        let wait_time = time_per_frame - last_time.elapsed().as_secs_f64();
        if wait_time > 0.0 {
            thread::sleep(Duration::from_secs_f64(wait_time));
        }
        // Calculate the actual FPS based on elapsed time after the wait and draw queue the value to be drawn to the screen.
        let elapsed_time = last_time.elapsed();
        let fps = 1.0 / elapsed_time.as_secs_f32();
        let fps_text = format!("fps: {:.2}", fps);
        draw_screen.update_with_string(
            screen_width - (fps_text.len() + 1) as u16,
            1,
            fps_text,
            DrawColor::White,
        );

        // Update time
        last_time = Instant::now();
    }
    Ok(())
}
