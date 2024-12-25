use bevy::prelude::*;
use crate::resources::*;
use crate::state::*;
use crate::components::tetrimino::*;
use crate::components::cell::*;
use crate::components::keys::*;

pub fn fall_tetrimino(
    mut game: ResMut<Game>,
    time: Res<Time>,
) {
    if !game.fall_cooldown.tick(time.delta()).finished() {
        return
    }

    let mut reference_point = game.reference_point;
    let tetrimino_pattern = tetrimino_pattern_from_tetrimino(game.playing_tetrimino, Some(game.facing));
    for (x, y) in tetrimino_pattern.iter() {
        let pattern_x = (reference_point.x as isize + *x as isize) as usize;
        let pattern_y = (reference_point.y as isize + *y as isize) as usize;
        if pattern_x == 0 || game.matrix[pattern_x - 1][pattern_y] == CellState::FixedBlock {
            game.is_reached_bottom = true;
            game.fall_cooldown.reset();
            return
        }
    }

    for (x, y) in tetrimino_pattern.iter() {
        let pattern_x = (reference_point.x as isize + *x as isize) as usize;
        let pattern_y = (reference_point.y as isize + *y as isize) as usize;
        game.matrix[pattern_x][pattern_y] = CellState::Empty;
    }

    reference_point.x = reference_point.x - 1;
    for (x, y) in tetrimino_pattern.iter() {
        let pattern_x = (reference_point.x as isize + *x as isize) as usize;
        let pattern_y = (reference_point.y as isize + *y as isize) as usize;
        game.matrix[pattern_x][pattern_y] = CellState::InPlayBlock;
    }

    if game.is_soft_dropping {
        game.score = game.score + 1;
    }
    game.reference_point = reference_point;
    game.fall_cooldown.reset();
}

pub fn move_tetrimino(
    keys: Res<ButtonInput<KeyCode>>,
    mut game: ResMut<Game>,
) {
    if keys.any_just_pressed(HARD_DROP) {
        let tetrimino_pattern = tetrimino_pattern_from_tetrimino(game.playing_tetrimino, Some(game.facing));

        for x in 0..MATRIX_PLAYABLE_HEIGHT {
            for y in 0..MATRIX_WIDTH {
                if game.matrix[x][y] == CellState::InPlayBlock {
                    game.matrix[x][y] = CellState::Empty;
                }
            }
        }

        let next_reference_point = game.ghost_reference_point.unwrap();
        for (x, y) in tetrimino_pattern.iter() {
            let pattern_x = (next_reference_point.x as isize + *x as isize) as usize;
            let pattern_y = (next_reference_point.y as isize + *y as isize) as usize;
            game.matrix[pattern_x][pattern_y] = CellState::InPlayBlock;
        }

        let previous_reference_point = game.reference_point;
        let new_score = game.score + (previous_reference_point.x - next_reference_point.x) as u32 * 2;

        game.reference_point = next_reference_point;
        game.is_reached_bottom = true;
        game.score = new_score;
    }

    if keys.any_just_pressed(MOVE_LEFT) || keys.any_just_pressed(MOVE_RIGHT) {
        let tetrimino_pattern = tetrimino_pattern_from_tetrimino(game.playing_tetrimino, Some(game.facing));
        let mut is_movable_left = true;
        let mut is_movable_right = true;
        for (x, y) in tetrimino_pattern.iter() {
            let pattern_y = (game.reference_point.y as isize + *y as isize) as usize;
            let pattern_x = (game.reference_point.x as isize + *x as isize) as usize;
            if pattern_y == 0 || game.matrix[pattern_x][pattern_y - 1] == CellState::FixedBlock {
                is_movable_left = false;
                break;
            }
            if pattern_y == MATRIX_WIDTH - 1 || game.matrix[pattern_x][pattern_y + 1] == CellState::FixedBlock {
                is_movable_right = false;
                break;
            }
        }

        if keys.any_just_pressed(MOVE_LEFT) && is_movable_left {
            game.reference_point.y = game.reference_point.y - 1;
        }
        if keys.any_just_pressed(MOVE_RIGHT) && is_movable_right {
            game.reference_point.y = game.reference_point.y + 1;
        }

        for x in 0..MATRIX_PLAYABLE_HEIGHT {
            for y in 0..MATRIX_WIDTH {
                if game.matrix[x][y] == CellState::InPlayBlock {
                    game.matrix[x][y] = CellState::Empty;
                }
            }
        }

        for (x, y) in tetrimino_pattern.iter() {
            let pattern_x = (game.reference_point.x as isize + *x as isize) as usize;
            let pattern_y = (game.reference_point.y as isize + *y as isize) as usize;
            game.matrix[pattern_x][pattern_y] = CellState::InPlayBlock;
        }
    }

    if keys.any_pressed(SOFT_DROP) && !game.is_soft_dropping {
        game.is_soft_dropping = true;
        game.fall_cooldown = Timer::from_seconds(game.fall_cooldown_value / 20., TimerMode::Once);
    } else if game.is_soft_dropping {
        game.is_soft_dropping = false;
        game.fall_cooldown = Timer::from_seconds(game.fall_cooldown_value, TimerMode::Once);
    }
}

pub fn rotate_tetrimino(
    keys: Res<ButtonInput<KeyCode>>,
    mut game: ResMut<Game>,
) {
    if keys.any_just_pressed(ROTATE_CLOCKWISE) || keys.any_just_pressed(ROTATE_COUNTER_CLOCKWISE) {
        let playing_tetrimino = game.playing_tetrimino;
        let mut reference_point = game.reference_point;
        let facing = game.facing;
        match playing_tetrimino {
            Tetrimino::TypeT | Tetrimino::TypeL | Tetrimino::TypeJ | Tetrimino::TypeS | Tetrimino::TypeZ => {
                let next_facing = if keys.any_just_pressed(ROTATE_COUNTER_CLOCKWISE) {
                    facing.turn_counter_clockwise()
                } else {
                    facing.turn_clockwise()
                };
                let tetrimino_pattern = tetrimino_pattern_from_tetrimino(playing_tetrimino, Some(next_facing));
                for (x, y) in tetrimino_pattern.iter() {
                    let mut pattern_x = reference_point.x as isize + *x as isize;
                    let mut pattern_y = reference_point.y as isize + *y as isize;
                    if pattern_x < 0 {
                        reference_point.x = reference_point.x + 1;
                        pattern_x = pattern_x + 1;
                    }
                    if pattern_y < 0 {
                        reference_point.y = reference_point.y + 1;
                        pattern_y = pattern_y + 1;
                    }
                    if pattern_y >= MATRIX_WIDTH as isize {
                        reference_point.y = reference_point.y - 1;
                        pattern_y = pattern_y - 1;
                    }
                    if pattern_x >= 0 && pattern_y >= 0 && pattern_y < MATRIX_WIDTH as isize{
                        if game.matrix[pattern_x as usize][pattern_y as usize] == CellState::FixedBlock {
                            reference_point.x = (reference_point.x as isize - *x as isize) as usize;
                            reference_point.y = (reference_point.y as isize - *y as isize) as usize;
                        }
                    }
                }
                let mut is_rotatable = true;
                for (x, y) in tetrimino_pattern.iter() {
                    let pattern_x = reference_point.x as isize + *x as isize;
                    let pattern_y = reference_point.y as isize + *y as isize;
                    if game.matrix[pattern_x as usize][pattern_y as usize] == CellState::FixedBlock {
                        is_rotatable = false;
                        break;
                    }
                }
                if is_rotatable {
                    game.facing = next_facing;
                    game.reference_point = reference_point;
                }
            },
            Tetrimino::TypeI => {
                let next_facing = facing.next();
                let tetrimino_pattern = tetrimino_pattern_from_tetrimino(playing_tetrimino, Some(next_facing));
                let mut ref_x: isize = game.reference_point.x as isize;
                let mut ref_y: isize = game.reference_point.y as isize;
                match next_facing {
                    Facing::North => {
                        ref_x = ref_x + 1;
                    },
                    Facing::East  => {
                        if ref_y < MATRIX_WIDTH as isize - 1 {
                            ref_y = ref_y + 1;
                        }
                    },
                    Facing::South => {
                        if ref_x > 0 {
                            ref_x = ref_x - 1;
                        }
                    },
                    Facing::West  => {
                        if ref_y > 0 {
                            ref_y = ref_y - 1;
                        }
                    },
                };

                for (x, y) in tetrimino_pattern.iter() {
                    let mut pattern_x = ref_x + *x as isize;
                    let mut pattern_y = ref_y + *y as isize;
                    if pattern_x < 0 {
                        ref_x = ref_x + 1;
                        pattern_x = pattern_x + 1;
                    }
                    if pattern_y < 0 {
                        ref_y = ref_y + 1;
                        pattern_y = pattern_y + 1;
                    }
                    if pattern_y >= MATRIX_WIDTH as isize {
                        ref_y = ref_y - 1;
                        pattern_y = pattern_y - 1;
                    }
                    if pattern_x >= 0 && pattern_y >= 0 || pattern_x < MATRIX_WIDTH as isize {
                        if game.matrix[pattern_x as usize][pattern_y as usize] == CellState::FixedBlock {
                            ref_x = ref_x - *x as isize;
                            ref_y = ref_y - *y as isize;
                        }
                    }
                }

                let mut is_rotatable = true;
                for (x, y) in tetrimino_pattern.iter() {
                    let pattern_x = ref_x + *x as isize;
                    let pattern_y = ref_y + *y as isize;
                    if game.matrix[pattern_x as usize][pattern_y as usize] == CellState::FixedBlock {
                        is_rotatable = false;
                        break;
                    }
                }
                if is_rotatable {
                    game.facing = next_facing;
                    game.reference_point = Position { x: ref_x as usize, y: ref_y as usize };
                }
            },
            _ => {},
        }

        for x in 0..MATRIX_PLAYABLE_HEIGHT {
            for y in 0..MATRIX_WIDTH {
                if game.matrix[x][y] == CellState::InPlayBlock {
                    game.matrix[x][y] = CellState::Empty;
                }
            }
        }

        let tetrimino_pattern = tetrimino_pattern_from_tetrimino(game.playing_tetrimino, Some(game.facing));
        for (x, y) in tetrimino_pattern.iter() {
            let pattern_x = (game.reference_point.x as isize + *x as isize) as usize;
            let pattern_y = (game.reference_point.y as isize + *y as isize) as usize;
            game.matrix[pattern_x][pattern_y] = CellState::InPlayBlock;
        }
    }
}

pub fn hold_tetrimino(
    keys: Res<ButtonInput<KeyCode>>,
    mut game: ResMut<Game>,
    mut next_state: ResMut<NextState<GamePhase>>,
) {
    if keys.any_just_pressed(HOLD) && !game.is_holded {
        game.is_holded = true;
        if game.holded_tetrimino.is_some() {
            next_state.set(GamePhase::Holding);
        } else {
            game.holded_tetrimino = Some(game.playing_tetrimino);
            next_state.set(GamePhase::Generation);
        }
    }
}
