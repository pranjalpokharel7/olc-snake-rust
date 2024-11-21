use ratatui::crossterm::event::{self, KeyCode, KeyEventKind};

use crate::types::{Direction, GameState};

pub fn process_input(snake_direction: Direction) -> Result<Direction, GameState> {
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
