use bevy::prelude::*;
use crate::resources::*;
use crate::components::tetrimino::*;
use crate::components::cell::*;

pub fn fix_tetrimino(
    mut game: ResMut<Game>,
) {
    let tetrimino_pattern = tetrimino_pattern_from_tetrimino(game.playing_tetrimino, Some(game.facing));
    let reference_point = game.reference_point;

    for (x, y) in tetrimino_pattern.iter() {
        let pattern_x = (reference_point.x as isize + *x as isize) as usize;
        let pattern_y = (reference_point.y as isize + *y as isize) as usize;
        game.matrix[pattern_x][pattern_y] = CellState::FixedBlock;
    }

    game.is_holded = false;
}
