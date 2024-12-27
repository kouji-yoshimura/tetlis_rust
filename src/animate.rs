use bevy::prelude::*;
use bevy::color::palettes::tailwind::*;
use crate::components::tetrimino::*;
use crate::components::cell::*;
use crate::resources::*;

pub fn render(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game: Res<Game>,
    mut query: Query<(Entity, &Position), With<Cell>>,
) {
    for (entity, position) in query.iter_mut() {
        let cell = game.matrix[position.x][position.y];
        match cell {
            CellState::InPlayBlock => commands.entity(entity).insert(MeshMaterial2d(materials.add(Color::from(NEUTRAL_600)))),
            CellState::Ghost       => commands.entity(entity).insert(MeshMaterial2d(materials.add(Color::from(NEUTRAL_300)))),
            CellState::FixedBlock  => commands.entity(entity).insert(MeshMaterial2d(materials.add(Color::from(NEUTRAL_900)))),
            _                      => commands.entity(entity).insert(MeshMaterial2d(materials.add(Color::from(NEUTRAL_400)))),
        };
    }
}

pub fn reflect_ghost(
    mut game: ResMut<Game>,
) {
    for x in 0..MATRIX_PLAYABLE_HEIGHT {
        for y in 0..MATRIX_WIDTH {
            if game.matrix[x][y] == CellState::Ghost {
                game.matrix[x][y] = CellState::Empty;
            }
        }
    }

    let tetrimino_pattern = tetrimino_pattern_from_tetrimino(game.playing_tetrimino, Some(game.facing));
    let mut ghost_reference_point = game.reference_point;
    'outer: loop {
        for (x, y) in tetrimino_pattern.iter() {
            let pattern_x = (ghost_reference_point.x as isize + *x as isize) as usize;
            let pattern_y = (ghost_reference_point.y as isize + *y as isize) as usize;
            if pattern_x == 0 || game.matrix[pattern_x - 1][pattern_y] == CellState::FixedBlock {
                break 'outer;
            }
        }
        ghost_reference_point.x = ghost_reference_point.x - 1;
    }

    for (x, y) in tetrimino_pattern.iter() {
        let pattern_x = (ghost_reference_point.x as isize + *x as isize) as usize;
        let pattern_y = (ghost_reference_point.y as isize + *y as isize) as usize;
        if game.matrix[pattern_x][pattern_y] == CellState::Empty {
            game.matrix[pattern_x][pattern_y] = CellState::Ghost;
        }
    }

    game.ghost_reference_point = Some(ghost_reference_point);
}

