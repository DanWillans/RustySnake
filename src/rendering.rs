use std::io::{Write, Stdout, stdout};
use std::ops::Index;
use crossterm::{execute, queue, terminal, cursor, style::{self, Stylize, StyledContent}};

#[derive(Clone, Copy)]
pub enum DrawColor {
  White,
  Green,
  Red,
  Blue,
}

pub struct DrawPixel{
  pub character: char,
  pub color: DrawColor,
}

impl DrawPixel{
  fn get_stylize(&self) -> StyledContent<char> {
    match &self.color{
      DrawColor::White => self.character.white(),
      DrawColor::Green => self.character.green(),
      _ => self.character.yellow(),
    }
  }
}

pub struct DrawScreen{
  width: u16,
  height: u16,
  // Don't think I need this. We queue commands into stdout anyway.
  // If we want to abstrac to a different draw interface we'd probably want to implement this step.
  // draw_buffer: Vec<DrawPixel>,
  io: Stdout,
}

impl DrawScreen {
  pub fn new(width: u16, height: u16) -> Self {
    let io = stdout();
    // Let's make sure we clear the draw screen first
    execute!(stdout(), terminal::Clear(terminal::ClearType::All));
    Self{width, height, io}
  }

  pub fn draw(&mut self){
    self.io.flush();
  }

  pub fn update(&mut self, x: u16, y: u16, character: char, color: DrawColor){
    queue!(self.io, cursor::MoveTo(x,y), style::PrintStyledContent(DrawPixel{character, color}.get_stylize()));
  }

  pub fn update_with_string(&mut self, x: u16, y: u16, string: String, color: DrawColor){
    let chars = string.as_bytes();
    for i in 0..string.len() {
      let c = chars[i] as char;
      self.update(x + (i as u16), y, c, color);
    }
  }

}