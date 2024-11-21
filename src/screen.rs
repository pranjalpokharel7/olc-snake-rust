use rand::rngs::ThreadRng;
use rand::Rng;

use std::collections::VecDeque;

use ratatui::{widgets::Paragraph, DefaultTerminal};

use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::types::Position;

#[inline(always)]
pub fn calculate_index(x: usize, y: usize) -> usize {
    y * SCREEN_WIDTH + x
}

pub fn clear_screen(screen_buffer: &mut Vec<u8>) {
    // draw empty spaces
    for x in 1..SCREEN_WIDTH - 2 {
        for y in 1..SCREEN_HEIGHT - 1 {
            screen_buffer[calculate_index(x, y)] = b' ';
        }
    }
}

pub fn find_empty_position(screen_buffer: &Vec<u8>, rng: &mut ThreadRng) -> Position {
    let mut x: usize = 0;
    let mut y: usize = 0;

    let mut invalid = true;
    while invalid {
        x = rng.gen_range(1..SCREEN_WIDTH);
        y = rng.gen_range(1..SCREEN_HEIGHT);

        let index = calculate_index(x, y);
        invalid = screen_buffer[index] != b' ';
    }

    Position { x, y }
}

pub fn render_screen(
    terminal: &mut DefaultTerminal,
    screen_buffer: &mut [u8],
    snake: &VecDeque<Position>,
    food: &Position,
) {
    // draw snake
    for sp in snake.iter().map(|pos| calculate_index(pos.x, pos.y)) {
        screen_buffer[sp] = b'O';
    }
    let head_index = calculate_index(snake[0].x, snake[0].y);
    screen_buffer[head_index] = b'@';

    // draw food
    let food_index = calculate_index(food.x, food.y);
    screen_buffer[food_index] = b'%';

    // render buffer to terminal
    terminal
        .draw(|frame| {
            frame.render_widget(
                Paragraph::new(String::from_utf8_lossy(screen_buffer)),
                frame.area(),
            );
        })
        .unwrap();
}
