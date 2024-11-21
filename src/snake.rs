use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::types::{Direction, GameState, Position};

pub fn calculate_next_head_position(
    head: &Position,
    direction: &Direction,
) -> Result<Position, GameState> {
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
