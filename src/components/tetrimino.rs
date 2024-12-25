use bevy::prelude::*;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

pub const TETRIMINO_PATTERNS: [[(isize, isize); 4]; 7] = [
    [(0, 0), (0, 1), (1, 0), (1, 1)],   // TypeO
    [(0, 0), (0, -1), (0, 1), (0, 2)],  // TypeI
    [(0, 0), (0, -1), (0, 1), (1, 0)],  // TypeT
    [(0, 0), (0, -1), (0, 1), (1, 1)],  // TypeL
    [(0, 0), (0, -1), (0, 1), (1, -1)], // TypeJ
    [(0, 0), (0, -1), (1, 0), (1, 1)],  // TypeS
    [(0, 0), (0, 1), (1, 0), (1, -1)],  // TypeZ
];

#[derive(Default, Copy, Clone)]
pub enum Facing {
    #[default]
    North,
    East,
    South,
    West,
}

impl Facing {
    pub fn next(&self) -> Facing {
        match self {
            Facing::North => Facing::East,
            Facing::East  => Facing::South,
            Facing::South => Facing::West,
            Facing::West  => Facing::North,
        }
    }

    pub fn turn_clockwise(&self) -> Facing {
        match self {
            Facing::North => Facing::East,
            Facing::East  => Facing::South,
            Facing::South => Facing::West,
            Facing::West  => Facing::North,
        }
    }

    pub fn turn_counter_clockwise(&self) -> Facing {
        match self {
            Facing::North => Facing::West,
            Facing::West  => Facing::South,
            Facing::South => Facing::East,
            Facing::East  => Facing::North,
        }
    }
}

#[derive(Component, Default, Copy, Clone, Debug)]
pub enum Tetrimino {
    #[default]
    TypeO,
    TypeI,
    TypeT,
    TypeL,
    TypeJ,
    TypeS,
    TypeZ,
}

impl Distribution<Tetrimino> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Tetrimino {
        match rng.gen_range(0..=6) {
            0 => Tetrimino::TypeO,
            1 => Tetrimino::TypeI,
            2 => Tetrimino::TypeT,
            3 => Tetrimino::TypeL,
            4 => Tetrimino::TypeJ,
            5 => Tetrimino::TypeS,
            _ => Tetrimino::TypeZ,
        }
    }
}

#[derive(Component)]
pub struct Holding {
    pub cell: u8,
    pub base_position: Vec3,
}

#[derive(Component)]
pub struct Queue {
    pub number: u8,
    pub cell: u8,
    pub base_position: Vec3,
}

pub fn tetrimino_pattern_from_tetrimino(
    tetrimino: Tetrimino,
    facing: Option<Facing>,
) -> [(isize, isize); 4] {
    let pattern = match tetrimino {
        Tetrimino::TypeO => TETRIMINO_PATTERNS[0],
        Tetrimino::TypeI => TETRIMINO_PATTERNS[1],
        Tetrimino::TypeT => TETRIMINO_PATTERNS[2],
        Tetrimino::TypeL => TETRIMINO_PATTERNS[3],
        Tetrimino::TypeJ => TETRIMINO_PATTERNS[4],
        Tetrimino::TypeS => TETRIMINO_PATTERNS[5],
        Tetrimino::TypeZ => TETRIMINO_PATTERNS[6],
    };

    let facing = facing.unwrap_or(Facing::North);
    let mut faced_pattern: [(isize, isize); 4] = [(0, 0); 4];
    for (i, (x, y)) in pattern.iter().enumerate() {
        faced_pattern[i] = match facing {
            Facing::North => (*x, *y),
            Facing::East => (-*y, *x),
            Facing::South => (-*x, -*y),
            Facing::West => (*y, -*x),
        };
    }

    faced_pattern
}
