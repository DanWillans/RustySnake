use std::io::{Write, Stdout, stdout};
use crossterm::{execute, queue, terminal, cursor, style::{self, Stylize, StyledContent}};

#[derive(Clone, Copy)]
pub enum DrawColor {
  White,
  Green,
  Red,
  Border,
  GameBorder,
}

pub struct DrawPixel{
  pub character: char,
  pub color: DrawColor,
}

impl DrawPixel{
  fn get_stylize(&self) -> StyledContent<char> {
    match &self.color{
      DrawColor::Red => self.character.red(),
      DrawColor::White => self.character.white(),
      DrawColor::Green => self.character.green(),
      DrawColor::Border => self.character.green(),
      DrawColor::GameBorder => self.character.dark_yellow(),
    }
  }
}

pub struct DrawScreen{
  io: Stdout,
}

impl DrawScreen {
  pub fn new(width: u16, height: u16) -> Self {
    let io = stdout();
    let mut screen = Self{io};
    // Let's make sure we clear the draw screen first
    if let Err(res) = execute!(stdout(), terminal::Clear(terminal::ClearType::All)) {
      println!("Error clearing terminal {}!", res.to_string());
    }
    // Update border of the draw screen
    // Draw corners of the draw screen
    screen.update(0, 0, '╔', DrawColor::Border);
    screen.update(width - 1, 0, '╗', DrawColor::Border);
    screen.update(width - 1, height, '╝', DrawColor::Border);
    screen.update(0, height, '╚', DrawColor::Border);
    // Draw left and right borders.
    // Divide by 2 because the character we're using is double length
    for i in 1..height{
      screen.update(0, i, '║', DrawColor::Border);
      screen.update(width - 1, i, '║', DrawColor::Border);
    }
    // Draw top and bottom border
    for i in 1..width - 1 {
      screen.update(i, 0, '═', DrawColor::Border);
      screen.update(i, height, '═', DrawColor::Border);
    }

    screen
  }

  pub fn draw(&mut self){
    if let Err(_res) = self.io.flush(){
      println!("Error flushing to stdout io");
    }
  }

  pub fn update(&mut self, x: u16, y: u16, character: char, color: DrawColor){
    if let Err(_res) = queue!(self.io, cursor::MoveTo(x,y), style::PrintStyledContent(DrawPixel{character, color}.get_stylize())) { 
      println!("Error queueing to stdout io");
    }
  }

  pub fn update_with_string(&mut self, x: u16, y: u16, string: String, color: DrawColor){
    let chars = string.as_bytes();
    for i in 0..string.len() {
      let c = chars[i] as char;
      self.update(x + (i as u16), y, c, color);
    }
  }

}