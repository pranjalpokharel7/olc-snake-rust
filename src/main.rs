#![allow(dead_code, unused_mut, unused_variables)]

use std::time::Instant;
use std::{collections::VecDeque, time::Duration};

use constants::{FRAME_DURATION, SCREEN_HEIGHT, SCREEN_STRING, SCREEN_WIDTH};
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    widgets::Paragraph,
    DefaultTerminal,
};

mod constants;

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Clone)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

#[derive(PartialEq)]
enum GameState {
    ACTIVE,
    PAUSED,
    OVER,
}

fn new_screen() -> String {
    SCREEN_STRING.into()
}

fn calculate_index(x: usize, y: usize) -> usize {
    y * SCREEN_WIDTH + x
}

fn render_screen(terminal: &mut DefaultTerminal, snake: &VecDeque<Position>) {
    let mut screen = new_screen();

    for sp in snake.iter().map(|pos| calculate_index(pos.x, pos.y)) {
        screen.replace_range(sp..sp + 1, "O");
    }

    let head_index = calculate_index(snake[0].x, snake[0].y);
    screen.replace_range(head_index..head_index + 1, "@");

    screen = format!("{} {:?}", screen, snake);

    terminal
        .draw(|frame| {
            frame.render_widget(Paragraph::new(screen), frame.area());
        })
        .unwrap();
}

fn process_input(snake_direction: &mut Direction, game_state: &mut GameState) -> () {
    if let event::Event::Key(key) = event::read().unwrap() {
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Up => *snake_direction = Direction::UP,
                KeyCode::Down => *snake_direction = Direction::DOWN,
                KeyCode::Left => *snake_direction = Direction::LEFT,
                KeyCode::Right => *snake_direction = Direction::RIGHT,
                KeyCode::Char('q') => *game_state = GameState::OVER,
                _ => (),
            }
        }
    }
}

fn calculate_head_next(head: &Position, direction: &Direction) -> Result<Position, GameState> {
    let next_position = match direction {
        Direction::UP => Position {
            x: head.x,
            y: head.y - 1,
        },
        Direction::DOWN => Position {
            x: head.x,
            y: head.y + 1,
        },
        Direction::LEFT => Position {
            x: head.x - 1,
            y: head.y,
        },
        Direction::RIGHT => Position {
            x: head.x + 1,
            y: head.y,
        },
    };

    return if next_position.x < 1
        || next_position.x >= SCREEN_WIDTH - 1
        || next_position.y < 1
        || next_position.y >= SCREEN_HEIGHT - 1
    {
        Err(GameState::PAUSED)
    } else {
        Ok(next_position)
    };
}

fn main() {
    let mut terminal = ratatui::init();
    terminal.clear().unwrap();

    let mut screen = new_screen();
    let mut snake: VecDeque<Position> = VecDeque::from(vec![
        Position { x: 12, y: 1 },
        Position { x: 11, y: 1 },
        Position { x: 10, y: 1 },
    ]);
    let food: Position = Position { x: 60, y: 15 };
    let score: u32 = 0;
    let mut snake_direction: Direction = Direction::RIGHT;
    let mut game_state: GameState = GameState::ACTIVE;

    let mut t0 = Instant::now();
    let mut lag = Duration::from_millis(0);
    while game_state != GameState::OVER {
        // we need to constraint the time elapsed between two loop runs
        // so that the game will run at constant speed on both fast/slow devices
        let t1 = Instant::now();
        let delta = t1 - t0;
        lag += delta;
        t0 = t1;

        render_screen(&mut terminal, &snake);

        // process_input(&mut snake_direction, &mut game_state);
        
        while lag >= FRAME_DURATION && game_state != GameState::PAUSED {
            // handle logic
            match calculate_head_next(&snake[0], &snake_direction) {
                Ok(next_head) => {
                    snake.push_front(next_head);
                    snake.pop_back();
                }
                Err(state) => game_state = state,
            }

            lag -= FRAME_DURATION;
        }
    }

    ratatui::restore();
}
