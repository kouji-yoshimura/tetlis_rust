use bevy::prelude::*;
use crate::components::tetrimino::*;
use crate::components::cell::*;
use crate::resources::*;

pub fn place_tetrimino_from_queue(
    mut game: ResMut<Game>,
) {
    let reference_point = match game.next_queue[0] {
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

    let tetrimino_pattern = tetrimino_pattern_from_tetrimino(game.next_queue[0], Some(Facing::North));
    for (x, y) in tetrimino_pattern.iter() {
        let pattern_x = (reference_point.x as isize + *x as isize) as usize;
        let pattern_y = (reference_point.y as isize + *y as isize) as usize;
        game.matrix[pattern_x][pattern_y] = CellState::InPlayBlock;
    }

    game.playing_tetrimino = game.next_queue[0];
    game.reference_point = reference_point;
    game.facing = Facing::North;
    game.is_reached_bottom = false;
    game.fall_cooldown_value = (0.8 - ((game.current_level - 1) as f32 * 0.007)).powf((game.current_level - 1) as f32);
    game.fall_cooldown = Timer::from_seconds(game.fall_cooldown_value, TimerMode::Once);
}

pub fn shift_tetrimino_queue(
    mut game: ResMut<Game>,
) {
    for index in 0..QUEUE_SIZE-1 {
        game.next_queue[index] = game.next_queue[index + 1];
    }
    game.next_queue[QUEUE_SIZE - 1] = rand::random();
}


pub fn refresh_queue(
    mut commands: Commands,
    game: Res<Game>,
    mut query: Query<(Entity, &Queue)>,
) {
    for (entity, queue) in query.iter_mut() {
        for (queue_index, tetrimino) in game.next_queue.iter().enumerate() {
            let tetrimino_pattern = tetrimino_pattern_from_tetrimino(*tetrimino, Some(Facing::North));
            if queue.number == queue_index as u8 {
                for (cell_index, (x, y)) in tetrimino_pattern.iter().enumerate() {
                    if queue.cell == cell_index as u8 {
                        let (adjust_x, adjust_y) = match *tetrimino {
                            Tetrimino::TypeI => (-5., 0.),
                            Tetrimino::TypeO => (-5., -5.),
                            _ => (0., -5.),
                        };
                        let cell_position = Vec3::new(queue.base_position.x + *y as f32 * 10. + adjust_x, queue.base_position.y + *x as f32 * 10. + adjust_y, queue.base_position.z);
                        commands.entity(entity).insert(
                            Transform::from_translation(cell_position)
                                .with_scale(Vec3::new(9., 9., 1.))
                        );
                    }
                }
            }
        }
    }
}
