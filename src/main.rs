use std::sync::mpsc::{self, Sender};

use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};

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

fn find_empty_space(screen_buffer: &String, rng: &mut ThreadRng) -> Position {
    let mut x: usize = 0;
    let mut y: usize = 0;

    let mut invalid = true;
    while invalid {
        x = rng.gen_range(1..SCREEN_WIDTH);
        y = rng.gen_range(1..SCREEN_HEIGHT);

        let index = calculate_index(x, y);
        invalid = screen_buffer.chars().nth(index).expect(
            format!(
                "invalid index when trying to calculate next position for food: {}",
                index
            )
            .as_str(),
        ) != ' ';
    }

    Position { x, y }
}

fn render_screen(
    terminal: &mut DefaultTerminal,
    screen_buffer: &mut String,
    snake: &VecDeque<Position>,
    food: &Position,
) {
    for sp in snake.iter().map(|pos| calculate_index(pos.x, pos.y)) {
        screen_buffer.replace_range(sp..sp + 1, "O");
    }

    let head_index = calculate_index(snake[0].x, snake[0].y);
    screen_buffer.replace_range(head_index..head_index + 1, "@");

    let food_index = calculate_index(food.x, food.y);
    screen_buffer.replace_range(food_index..food_index + 1, "%");

    (*screen_buffer).push_str(format!("{:?}", snake).as_str());

    terminal
        .draw(|frame| {
            frame.render_widget(
                Paragraph::new(String::from(screen_buffer.as_str())),
                frame.area(),
            );
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
                KeyCode::Char('p') => Err(GameState::PAUSED),
                KeyCode::Char('s') => Err(GameState::ACTIVE),
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
        Err(GameState::PAUSED)
    } else {
        Ok(next_position)
    };
}

fn main() {
    let mut terminal = ratatui::init();
    terminal.clear().unwrap();
    let (tx, rx) = mpsc::channel::<Result<Direction, GameState>>();
    let mut rng = thread_rng();

    let mut screen_buffer = new_screen();
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

    thread::spawn(move || process_input_thread(snake_direction, tx));

    while game_state != GameState::OVER {
        // we need to constraint the time elapsed between two loop runs
        // so that the game will run at constant speed on both fast/slow devices
        let t1 = Instant::now();
        let delta = t1 - t0;
        lag += delta;
        t0 = t1;

        // handle logic
        let next_direction = rx.try_recv();
        if next_direction.is_ok() {
            match next_direction.unwrap() {
                Ok(direction) => snake_direction = direction,
                Err(state) => game_state = state,
            }
        }

        while game_state != GameState::PAUSED && lag >= FRAME_DURATION {
            if snake[0].x == food.x && snake[0].y == food.y {
                if let Some(pos) = snake.back() {
                    snake.push_back(Position { x: pos.x, y: pos.y });
                }
                food = find_empty_space(&screen_buffer, &mut rng);
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

        // render screen
        screen_buffer = new_screen();
        render_screen(&mut terminal, &mut screen_buffer, &snake, &food);
    }

    ratatui::restore();
}
