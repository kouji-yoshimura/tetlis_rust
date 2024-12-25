use bevy::prelude::*;
use crate::components::tetrimino::*;
use crate::components::cell::*;

pub const MATRIX_WIDTH: usize = 10;
pub const MATRIX_HEIGHT: usize = 40;
pub const MATRIX_PLAYABLE_HEIGHT: usize = 20;
pub const TETRIMINO_BLOCKS: usize = 4;
pub const QUEUE_SIZE: usize = 6;

#[derive(Resource)]
pub struct Game {
    pub matrix: [[CellState; MATRIX_WIDTH]; MATRIX_HEIGHT],
    pub next_queue: [Tetrimino; QUEUE_SIZE],
    pub holded_tetrimino: Option<Tetrimino>,
    pub facing: Facing,
    pub playing_tetrimino: Tetrimino,
    pub reference_point: Position,
    pub ghost_reference_point: Option<Position>,
    pub fall_cooldown: Timer,
    pub fall_cooldown_value: f32,
    pub score: u32,
    pub play_time: Timer,
    pub number_of_lines: u32,
    pub current_level: u8,
    pub is_reached_bottom: bool,
    pub is_game_over: bool,
    pub is_soft_dropping: bool,
    pub is_holded: bool,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            matrix: [[Default::default(); MATRIX_WIDTH]; MATRIX_HEIGHT],
            next_queue: [Default::default(); QUEUE_SIZE],
            holded_tetrimino: None,
            facing: Default::default(),
            playing_tetrimino: Default::default(),
            reference_point: Default::default(),
            ghost_reference_point: None,
            fall_cooldown: Default::default(),
            fall_cooldown_value: 1.0,
            score: 0,
            play_time: Default::default(),
            number_of_lines: 0,
            current_level: 1,
            is_reached_bottom: Default::default(),
            is_game_over: Default::default(),
            is_soft_dropping: Default::default(),
            is_holded: Default::default(),
        }
    }
}
