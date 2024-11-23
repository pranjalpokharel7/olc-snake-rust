use std::sync::mpsc;

use rand::thread_rng;

use std::thread;
use std::time::Instant;
use std::{collections::VecDeque, time::Duration};

use constants::{FRAME_DURATION_HORIZONTAL, FRAME_DURATION_VERTICAL, SCREEN_STRING};
use input::process_input;
use screen::{clear_screen, find_empty_position, render_screen};
use snake::calculate_next_head_position;
use types::{Direction, GameState, Position};

mod constants;
mod input;
mod screen;
mod snake;
mod types;

fn main() {
    let mut terminal = ratatui::init();
    terminal.clear().unwrap();
    let (tx, rx) = mpsc::channel::<Result<Direction, GameState>>();
    let mut rng = thread_rng();

    let mut screen_buffer = SCREEN_STRING.as_bytes().to_vec();
    let mut snake: VecDeque<Position> = VecDeque::from(vec![
        Position { x: 12, y: 1 },
        Position { x: 11, y: 1 },
        Position { x: 10, y: 1 },
    ]);
    let mut food: Position = Position { x: 60, y: 15 };
    // let score: u32 = 0;
    let mut snake_direction: Direction = Direction::RIGHT;
    let mut game_state: GameState = GameState::ACTIVE;

    let mut t0 = Instant::now();
    let mut lag = Duration::from_millis(0);

    thread::spawn(move || loop {
        tx.send(process_input(snake_direction)).unwrap();
    });

    while game_state != GameState::OVER {
        // we need to constraint the time elapsed between two loop runs
        // so that the game will run at constant speed on both fast/slow devices
        let t1 = Instant::now();
        let delta = t1 - t0;
        lag += delta;
        t0 = t1;

        // handle logic
        let direction_recv = rx.try_recv();
        if direction_recv.is_ok() {
            match direction_recv.unwrap() {
                Ok(next_direction) => {
                    if !matches!(
                        (snake_direction, next_direction),
                        (Direction::UP, Direction::DOWN)
                            | (Direction::DOWN, Direction::UP)
                            | (Direction::LEFT, Direction::RIGHT)
                            | (Direction::RIGHT, Direction::LEFT)
                    ) {
                        snake_direction = next_direction;
                    }
                }
                Err(state) => game_state = state,
            }
        }

        let frame_cap = if snake_direction == Direction::UP || snake_direction == Direction::DOWN {
            FRAME_DURATION_VERTICAL
        } else {
            FRAME_DURATION_HORIZONTAL
        };
        while game_state != GameState::PAUSED && lag >= frame_cap {
            // collision derection against food
            if snake[0].x == food.x && snake[0].y == food.y {
                if let Some(pos) = snake.back() {
                    snake.push_back(Position { x: pos.x, y: pos.y });
                }
                food = find_empty_position(&screen_buffer, &mut rng);
            }

            // collision detection against snake itself
            let head = &snake[0];
            for i in 1..snake.len() {
                if snake[i].x == head.x && snake[i].y == head.y {
                    game_state = GameState::OVER;
                }
            }

            match calculate_next_head_position(head, &snake_direction) {
                Ok(next_head) => {
                    snake.push_front(next_head);
                    snake.pop_back();
                }
                Err(state) => game_state = state,
            }

            lag -= frame_cap;
        }

        // render screen
        clear_screen(&mut screen_buffer);
        render_screen(&mut terminal, &mut screen_buffer, &snake, &food);
    }

    ratatui::restore();
}
