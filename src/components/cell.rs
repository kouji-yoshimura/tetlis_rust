use bevy::prelude::*;
use std::fmt;

#[derive(Component)]
pub struct Cell;

#[derive(Component, Default, Copy, Clone, PartialEq, Eq)]
pub enum CellState {
    #[default]
    Empty,
    InPlayBlock,
    Ghost,
    FixedBlock,
}

impl fmt::Display for CellState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CellState::Empty => write!(f, "Empty"),
            CellState::InPlayBlock => write!(f, "InPlayBlock"),
            CellState::Ghost => write!(f, "Ghost"),
            CellState::FixedBlock => write!(f, "FixedBlock"),
        }
    }
}

#[derive(Component, Default, Copy, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}
