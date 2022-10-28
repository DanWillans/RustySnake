// Copyright (c) 2022 DanWillans
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
  width: u16,
  height: u16,
  io: Stdout,
}

impl DrawScreen {
  pub fn new(width: u16, height: u16) -> Self {
    let io = stdout();
    let mut screen = Self{width, height, io};
    // Let's make sure we clear the draw screen first
    if let Err(res) = execute!(stdout(), terminal::Clear(terminal::ClearType::All)) {
      println!("Error clearing terminal {}!", res.to_string());
    }

    screen.draw_border();

    screen
  }

  pub fn draw_border(&mut self){
    // Update border of the draw screen
    // Draw corners of the draw screen
    self.update(0, 0, '╔', DrawColor::Border);
    self.update(self.width - 1, 0, '╗', DrawColor::Border);
    self.update(self.width - 1, self.height, '╝', DrawColor::Border);
    self.update(0, self.height, '╚', DrawColor::Border);
    // Draw left and right borders.
    // Divide by 2 because the character we're using is double length
    for i in 1..self.height{
      self.update(0, i, '║', DrawColor::Border);
      self.update(self.width - 1, i, '║', DrawColor::Border);
    }
    // Draw top and bottom border
    for i in 1..self.width - 1 {
      self.update(i, 0, '═', DrawColor::Border);
      self.update(i, self.height, '═', DrawColor::Border);
    }
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

  pub fn clear(&mut self){
    for y in 1..self.height {
      for x in 1..self.width - 1 {
                self.update(
                    x,
                    y,
                    ' ',
                    DrawColor::White,
                );
            }
        }
  }

}