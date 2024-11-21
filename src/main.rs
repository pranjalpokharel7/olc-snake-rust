use std::sync::mpsc::{self, Sender};

use std::thread;
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

#[derive(PartialEq, Clone, Copy)]
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

fn process_input(snake_direction: Direction) -> Result<Direction, GameState> {
    if let event::Event::Key(key) = event::read().unwrap() {
        if key.kind == KeyEventKind::Press {
            return match key.code {
                KeyCode::Up => Ok(Direction::UP),
                KeyCode::Down => Ok(Direction::DOWN),
                KeyCode::Left => Ok(Direction::LEFT),
                KeyCode::Right => Ok(Direction::RIGHT),
                KeyCode::Char('q') => Err(GameState::OVER),
                _ => Ok(snake_direction),
            };
        }
    }
    return Ok(snake_direction);
}

fn process_input_thread(snake_direction: Direction, tx: Sender<Result<Direction, GameState>>) {
    loop {
        tx.send(process_input(snake_direction)).unwrap();
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

    let (tx, rx) = mpsc::channel::<Result<Direction, GameState>>();

    let mut snake: VecDeque<Position> = VecDeque::from(vec![
        Position { x: 12, y: 1 },
        Position { x: 11, y: 1 },
        Position { x: 10, y: 1 },
    ]);
    // let food: Position = Position { x: 60, y: 15 };
    // let score: u32 = 0;
    let mut snake_direction: Direction = Direction::RIGHT;
    let mut game_state: GameState = GameState::ACTIVE;

    let mut t0 = Instant::now();
    let mut lag = Duration::from_millis(0);

    thread::spawn(move || process_input_thread(snake_direction, tx));

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
            let next_direction = rx.try_recv();
            if next_direction.is_ok() {
                match next_direction.unwrap() {
                    Ok(direction) => snake_direction = direction,
                    Err(state) => game_state = state,
                }
            }

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
