use bevy::prelude::*;
use bevy::color::palettes::tailwind::*;
use crate::state::*;
use crate::resources::*;
use crate::components::cell::*;
use crate::components::tetrimino::*;

pub fn place_tetrimino_from_holding(
    mut game: ResMut<Game>,
    mut next_state: ResMut<NextState<GamePhase>>,
) {
    if game.holded_tetrimino.is_none() {
        next_state.set(GamePhase::Generation);
        return
    }

    let tetrimino = game.holded_tetrimino.unwrap();
    let reference_point = match tetrimino {
        Tetrimino::TypeI => Position { x: MATRIX_PLAYABLE_HEIGHT - 1, y: 4 },
        _ => Position { x: MATRIX_PLAYABLE_HEIGHT - 2, y: 4 },
    };

    for x in 0..MATRIX_PLAYABLE_HEIGHT {
        for y in 0..MATRIX_WIDTH {
            if game.matrix[x][y] == CellState::InPlayBlock {
                game.matrix[x][y] = CellState::Empty;
            }
        }
    }

    let tetrimino_pattern = tetrimino_pattern_from_tetrimino(tetrimino, Some(Facing::North));
    for (x, y) in tetrimino_pattern.iter() {
        let pattern_x = (reference_point.x as isize + *x as isize) as usize;
        let pattern_y = (reference_point.y as isize + *y as isize) as usize;
        game.matrix[pattern_x][pattern_y] = CellState::InPlayBlock;
    }

    game.holded_tetrimino = Some(game.playing_tetrimino);
    game.playing_tetrimino = tetrimino;
    game.reference_point = reference_point;
    game.facing = Facing::North;
}

pub fn refresh_holding(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game: Res<Game>,
    mut query: Query<(Entity, &Holding)>,
) {
    for (entity, holding) in query.iter_mut() {
        if game.holded_tetrimino.is_none() {
            commands.entity(entity).insert(MeshMaterial2d(materials.add(Color::from(NEUTRAL_900))));
            continue;
        }

        let tetrimino = game.holded_tetrimino.unwrap();
        let tetrimino_pattern = tetrimino_pattern_from_tetrimino(tetrimino, Some(Facing::North));
        for (cell_index, (x, y)) in tetrimino_pattern.iter().enumerate() {
            if holding.cell == cell_index as u8 {
                let (adjust_x, adjust_y) = match tetrimino {
                    Tetrimino::TypeI => (-5., 0.),
                    Tetrimino::TypeO => (-5., -5.),
                    _ => (0., -5.),
                };
                let cell_position = Vec3::new(holding.base_position.x + *y as f32 * 10. + adjust_x, holding.base_position.y + *x as f32 * 10. + adjust_y, holding.base_position.z);
                commands.entity(entity).insert(
                    Transform::from_translation(cell_position)
                        .with_scale(Vec3::new(9., 9., 1.))
                );
                commands.entity(entity).insert(MeshMaterial2d(materials.add(Color::from(NEUTRAL_400))));
            }
        }
    }
}
