#[derive(Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

#[derive(PartialEq)]
pub enum GameState {
    ACTIVE,
    PAUSED,
    OVER,
}
