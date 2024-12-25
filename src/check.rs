use bevy::prelude::*;
use crate::resources::*;
use crate::components::tetrimino::*;
use crate::components::cell::*;

pub fn check_game_over(
    mut game: ResMut<Game>,
) {
    let next_tetrimino = game.next_queue[0];
    let reference_point = match next_tetrimino {
        Tetrimino::TypeI => Position { x: MATRIX_PLAYABLE_HEIGHT - 1, y: 4 },
        _ => Position { x: MATRIX_PLAYABLE_HEIGHT - 2, y: 4 },
    };

    game.is_game_over = false;
    let tetrimino_pattern = tetrimino_pattern_from_tetrimino(game.playing_tetrimino, Some(game.facing));
    for (x, y) in tetrimino_pattern.iter() {
        let pattern_x = (reference_point.x as isize + *x as isize) as usize;
        let pattern_y = (reference_point.y as isize + *y as isize) as usize;
        if game.matrix[pattern_x][pattern_y] == CellState::FixedBlock {
            game.is_game_over = true;
            return
        }
    }
}

pub fn check_level_up(
    mut game: ResMut<Game>,
) {
    let mut accumlator: u32 = 0;
    for level in 1..=15 {
        accumlator = accumlator + level * 5;
        if game.current_level > level as u8 {
            continue;
        }
        if game.number_of_lines >= accumlator {
            game.current_level = (level + 1) as u8;
        }
        break;
    }
}
