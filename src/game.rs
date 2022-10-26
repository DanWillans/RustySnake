use crate::{DrawColor, DrawScreen};
use rand::Rng;
use std::collections::VecDeque;

struct SnakeElement {
    x: i16,
    y: i16,
    character: char,
    color: DrawColor,
}

enum Direction {
    North,
    East,
    West,
    South,
}

struct Snake {
    // Front of VecDeque is always the head of the snake
    // Back of VecDeque is always the tail of the snake
    elements: VecDeque<SnakeElement>,
    direction: Direction,
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
    position: (u16, u16),
    game_active: bool,
}

// Because we use larger unicode characters for the snake and they take up two spaces we'll need to alter the game board in comparison to the DrawScreen.
// The GameBoard is half the resolution in the x direction than the game board. When drawing to the board use normal x, y co-ordinates relative to the top left of the GameBoard(0,0).
// The drawing functions will correct the positions relative to the DrawScreen.
impl GameBoard {
    pub fn new(position: (u16, u16), width: u16, height: u16, screen: &mut DrawScreen) -> Self {
        // Initialise game board
        let mut game_board = GameBoard {
            snake: Snake {
                elements: VecDeque::new(),
                direction: Direction::East,
            },
            width,
            height,
            position,
            game_active: true,
        };

        // Initialise snake elements
        game_board.snake.elements.push_back(SnakeElement {
            x: 2,
            y: 0,
            character: '🙂',
            color: DrawColor::Green,
        });
        game_board.snake.elements.push_back(SnakeElement {
            x: 1,
            y: 0,
            character: '🐍',
            color: DrawColor::Green,
        });
        game_board.snake.elements.push_back(SnakeElement {
            x: 0,
            y: 0,
            character: '🐍',
            color: DrawColor::Green,
        });

        // Update border of the draw screen
        // Draw corners of the draw screen
        screen.update(position.0, position.1, '╔', DrawColor::GameBorder);
        screen.update(
            position.0 + width - 1,
            position.1,
            '╗',
            DrawColor::GameBorder,
        );
        screen.update(
            position.0 + width - 1,
            position.1 + height,
            '╝',
            DrawColor::GameBorder,
        );
        screen.update(position.0, position.1 + height, '╚', DrawColor::GameBorder);
        // Draw left and right borders.
        // Divide by 2 because the character we're using is double length
        for i in 1..height {
            screen.update(position.0, position.1 + i, '║', DrawColor::GameBorder);
            screen.update(
                position.0 + width - 1,
                position.1 + i,
                '║',
                DrawColor::GameBorder,
            );
        }
        // Draw top and bottom border
        for i in 1..width - 1 {
            screen.update(position.0 + i, position.1, '═', DrawColor::GameBorder);
            screen.update(
                position.0 + i,
                position.1 + height,
                '═',
                DrawColor::GameBorder,
            );
        }

        // Draw the first apple and first snake
        game_board.draw_apple(screen);
        game_board.draw_snake(screen);

        game_board
    }

    fn draw_element(&self, element: &SnakeElement, screen: &mut DrawScreen) {
        screen.update(
            self.position.0 + (element.x as u16 * 2 + 1),
            self.position.1 + (element.y as u16 + 1),
            element.character,
            element.color,
        );
    }

    // Draw a random apple on the game board
    fn draw_apple(&self, screen: &mut DrawScreen) {
        let mut rng = rand::thread_rng();
        let rand_x = rng.gen_range(0..(self.width / 2)) as i16;
        let rand_y = rng.gen_range(0..self.height - 2) as i16;
        let element = SnakeElement {
            x: rand_x,
            y: rand_y,
            character: '🍎',
            color: DrawColor::Red,
        };
        self.draw_element(&element, screen);
    }

    // Draw the snake on the game board
    fn draw_snake(&self, screen: &mut DrawScreen) {
        for element in &self.snake.elements {
            self.draw_element(element, screen);
        }
    }

    // Remove tail SnakeElement from GameBoard and Snake
    fn remove_tail_element(&mut self, screen: &mut DrawScreen) {
        if let Some(element) = self.snake.elements.back_mut() {
            element.character = ' ';
        }
        if let Some(element) = self.snake.elements.back() {
            self.draw_element(element, screen);
        }
        self.snake.elements.pop_back();
    }

    fn update_snake_position(&mut self, screen: &mut DrawScreen) {
        self.remove_tail_element(screen);

        // Update character of the head as we'll be adding a new head onto the front based
        // on direction
        if let Some(element) = self.snake.elements.front_mut() {
            element.character = '🐍';
        }

        // Helper closure for adding to the head
        let mut add_head = |x: i16, y: i16| {
            if let Some(head_pos) = self.snake.elements.front() {
                self.snake.elements.push_front(SnakeElement {
                    x: (head_pos.x as i16 + x),
                    y: (head_pos.y as i16 + y),
                    character: '🙂',
                    color: DrawColor::Green,
                });
            }
        };

        // Add new Head depending on direction
        match self.snake.direction {
            Direction::North => add_head(0, -1),
            Direction::East => add_head(1, 0),
            Direction::South => add_head(0, 1),
            Direction::West => add_head(-1, 0),
        }
    }

    // Returns true if the snake head collides with the border
    fn check_border_collision(&self) -> bool {
        if let Some(head) = self.snake.get_head_element() {
            if head.x < 0
                || head.x > (self.width as i16 - 2) / 2
                || head.y > self.height as i16 - 2
                || head.y < 0
            {
                return true;
            }
        }
        false
    }

    fn game_over(&mut self, screen: &mut DrawScreen) {
        self.game_active = false;
        let game_over_text = "Oh my goodness you did such a big lose! Try gen.";
        let text_pos_x = self.position.0 + self.width / 2 - (game_over_text.len() / 2) as u16;
        let text_pos_y = self.position.1 + self.height / 2;
        screen.update_with_string(text_pos_x, text_pos_y, game_over_text.to_string(), DrawColor::Red);
    }

    pub fn update(&mut self, screen: &mut DrawScreen) {
        if self.game_active {
            self.update_snake_position(screen);

            // Calculate if we've collided with the border
            if self.check_border_collision() {
                self.game_over(screen);
                return;
            }

            // Calculate if we've collided with ourselves

            // Calculate if we've eaten an apple

            // Redraw the snake
            self.draw_snake(screen);
        }
    }
}
