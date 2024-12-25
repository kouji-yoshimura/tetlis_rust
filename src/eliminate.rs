use bevy::prelude::*;
use crate::resources::*;
use crate::components::cell::*;

pub fn delete_lines(
    mut game: ResMut<Game>,
) {
    let mut deleted_lines = Vec::new();
    for x in 0..MATRIX_HEIGHT {
        let mut is_filled_line = true;
        for y in 0..MATRIX_WIDTH {
            if game.matrix[x][y] != CellState::FixedBlock {
                is_filled_line = false;
                break;
            }
        }
        if is_filled_line {
            deleted_lines.push(x);
        }
    }
    for (i, line) in deleted_lines.iter().enumerate() {
        for x in 0..MATRIX_PLAYABLE_HEIGHT {
            if x < line - i {
                continue;
            }
            for y in 0..MATRIX_WIDTH {
                game.matrix[x][y] = game.matrix[x + 1][y];
            }
        }
    }

    for x in 0..MATRIX_PLAYABLE_HEIGHT {
        for y in 0..MATRIX_WIDTH {
            if game.matrix[x][y] == CellState::InPlayBlock {
                game.matrix[x][y] = CellState::Empty;
            }
        }
    }

    let base_point = match deleted_lines.len() {
        0 => 0u32,
        1 => 100,
        2 => 300,
        3 => 500,
        _ => 800,
    };
    let point = base_point * game.current_level as u32;
    game.score = game.score + point;
    game.number_of_lines = game.number_of_lines + deleted_lines.len() as u32;
}
