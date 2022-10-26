use crate::{DrawColor, DrawScreen};
use std::collections::VecDeque;

#[derive(Clone)]
struct SnakeElement {
  x: u16,
  y: u16,
  character: char,
  color: DrawColor,
}

struct Snake {
  // Front of VecDeque is always the head of the snake
  // Back of VecDeque is always the tail of the snake
  elements: VecDeque<SnakeElement>
}

impl Snake {
  pub fn get_length(&self) -> usize {
    self.elements.len()
  }
  pub fn get_head_element(&self) -> Option<&SnakeElement> {
    self.elements.front()
  }
}

pub struct GameBoard {
  snake: Snake,
  width: u16,
  height: u16,
}

impl GameBoard {
  pub fn new(position: (u16, u16), width: u16, height: u16, screen: &mut DrawScreen) -> Self {
    let game_board = GameBoard{snake: Snake{elements: VecDeque::new()}, width, height};

    // Draw initial game board
    for x in 0..=width {
      for y in 0..=height {
        screen.update(position.0 + x, position.1 + y, '·', DrawColor::White);
      }
    }

    game_board
  }
}