#![allow(dead_code, unused_mut, unused_variables)]

use std::collections::VecDeque;
use std::time::Instant;

use constants::{SCREEN_HEIGHT, SCREEN_STRING, SCREEN_WIDTH};
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    widgets::Paragraph,
    DefaultTerminal,
};

mod constants;

struct Position {
    x: usize,
    y: usize,
}

enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

#[derive(PartialEq)]
enum GameState {
    ACTIVE,
    OVER,
}

fn new_screen() -> String {
    SCREEN_STRING.into()
}

fn render_screen(terminal: &mut DefaultTerminal, snake: &VecDeque<Position>) {
    let mut screen = new_screen();

    for sp in snake.iter().map(|pos| pos.y * SCREEN_WIDTH + pos.x) {
        screen.replace_range(sp..sp + 1, "O");
    }

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
        Err(GameState::OVER)
    } else {
        Ok(next_position)
    };
}

fn main() {
    let mut terminal = ratatui::init();
    terminal.clear().unwrap();

    let mut screen = new_screen();
    let mut snake: VecDeque<Position> = VecDeque::from(vec![
        Position { x: 60, y: 10 },
        Position { x: 61, y: 10 },
        Position { x: 62, y: 10 },
    ]);
    let food: Position = Position { x: 60, y: 15 };
    let score: u32 = 0;
    let mut snake_direction: Direction = Direction::RIGHT;
    let mut game_state: GameState = GameState::ACTIVE;

    let t0 = Instant::now();

    while game_state == GameState::ACTIVE {
        // we need to constraint the time elapsed between two loop runs
        // so that the game will run at constant speed on both fast/slow devices
        // let t1 = Instant::now();
        // let delta = t1 - t0;
        // let duration = t1.elapsed();

        render_screen(&mut terminal, &snake);

        process_input(&mut snake_direction, &mut game_state);

        // // handle logic
        match calculate_head_next(&snake[0], &snake_direction) {
            Ok(next_head) => {
                snake.push_front(next_head);
            }
            Err(state) => game_state = state,
        }

        // // pop snake's tail
        snake.pop_back();

    }

    ratatui::restore();
}
