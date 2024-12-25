use bevy::prelude::*;
use bevy::color::palettes::tailwind::*;
use bevy::input::common_conditions::*;

mod components;
use crate::components::tetrimino::*;
use crate::components::cell::*;
use crate::components::text::*;
use crate::components::keys::*;

mod resources;
use crate::resources::*;

mod generation;
use crate::generation::*;

mod holding;
use crate::holding::*;

mod falling;
use crate::falling::*;

mod lock;
use crate::lock::*;

mod eliminate;
use crate::eliminate::*;

mod check;
use crate::check::*;

mod game_over;
use crate::game_over::*;

mod animate;
use crate::animate::*;

mod state;
use crate::state::*;

mod indicator;
use crate::indicator::*;

fn main() {
    App::new()
        .init_resource::<Game>()
        .add_plugins(DefaultPlugins)
        .insert_state(GamePhase::Generation)
        .add_systems(Startup, setup)
        .add_systems(Update, (
            (
                place_tetrimino_from_queue,
                shift_tetrimino_queue,
                reflect_ghost,
                refresh_queue,
                refresh_holding,
                render,
            ).chain()
                .run_if(in_state(GamePhase::Generation)),
            (
                place_tetrimino_from_holding,
                reflect_ghost,
                refresh_holding,
                render,
            ).chain()
                .run_if(in_state(GamePhase::Holding)),
            (
                move_tetrimino,
                rotate_tetrimino,
                hold_tetrimino,
                fall_tetrimino,
                reflect_ghost,
                render,
            ).chain()
                .run_if(in_state(GamePhase::Falling)),
            (
                fix_tetrimino,
                render,
            ).chain()
                .run_if(in_state(GamePhase::Lock)),
            (
                delete_lines,
                render,
            ).chain()
                .run_if(in_state(GamePhase::Eliminate)),
            (
                check_game_over,
                check_level_up,
            ).chain()
                .run_if(in_state(GamePhase::Check)),
            // (
            //     print_game_over,
            // ).chain()
            //     .run_if(in_state(GamePhase::GameOver)),
            (
                test_reflesh,
                refresh_queue,
                refresh_holding,
            ).chain()
                .run_if(input_just_pressed(KeyCode::KeyQ)),
            transition_phase,
            refresh_texts,
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game: ResMut<Game>,
) {
    commands.spawn(Camera2d);

    reset_game(&mut game);
    for row in 0..MATRIX_PLAYABLE_HEIGHT {
        for column in 0..MATRIX_WIDTH {
            let position = Vec3::new(20.0 * (column as f32 - MATRIX_WIDTH as f32 / 2.0), 20.0 * (row as f32 - MATRIX_PLAYABLE_HEIGHT as f32 / 2.0), 0.0);
            commands.spawn((
                Mesh2d(meshes.add(Rectangle::default())),
                MeshMaterial2d(materials.add(Color::from(NEUTRAL_400))),
                Transform::from_translation(position)
                    .with_scale(Vec3::new(19., 19., 0.)),
                Position {
                    x: row,
                    y: column,
                },
                Cell,
            ));
        }
    }

    let left = 340.0;
    let top = 250.0;
    let indicators: [(&str, &str, IndicatorType); 3] = [
        ("Score", "0", IndicatorType::Score),
        ("Lines", "0", IndicatorType::NumberOfLinesCleard),
        ("Level", "0", IndicatorType::CurrentLevel),
    ];
    for (i, (label, default_value, indicator_type)) in indicators.iter().enumerate() {
        commands.spawn((
            Text::new(*label),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextLayout::new_with_justify(JustifyText::Left),
            Node {
                display: Display::Flex,
                position_type: PositionType::Relative,
                top: Val::Px(top + i as f32 * 40.),
                left: Val::Px(left),
                ..default()
            },
            Indicator,
        ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new(*default_value),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextLayout::new_with_justify(JustifyText::Left),
                    Node {
                        display: Display::Flex,
                        position_type: PositionType::Relative,
                        left: Val::Px(70.0),
                        ..default()
                    },
                    *indicator_type,
                ));
            });
    }

    for queue_number in 0..QUEUE_SIZE {
        let position = Vec3::new(140., (QUEUE_SIZE - queue_number - 1) as f32 * 60. - 160., 0.);
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::default())),
            MeshMaterial2d(materials.add(Color::from(NEUTRAL_900))),
            Transform::from_translation(position)
                .with_scale(Vec3::new(50., 50., 0.)),
        ));
        for num in 0..4 {
            let cell_position = Vec3::new(position.x + (num as f32 - 2.) * 10. + 5., position.y, 1.);
            commands.spawn((
                Mesh2d(meshes.add(Rectangle::default())),
                MeshMaterial2d(materials.add(Color::from(NEUTRAL_400))),
                Transform::from_translation(cell_position)
                    .with_scale(Vec3::new(9., 9., 1.)),
                Queue {
                    number: queue_number as u8,
                    cell: num as u8,
                    base_position: position,
                },
            ));
        }
    }

    let holding_position = Vec3::new(-160., 165., 0.);
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::from(NEUTRAL_900))),
        Transform::from_translation(holding_position)
            .with_scale(Vec3::new(50., 50., 0.)),
    ));
    for num in 0..4 {
        let cell_position = Vec3::new(holding_position.x + (num as f32 - 2.) * 10. + 5., holding_position.y, 0.);
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::default())),
            MeshMaterial2d(materials.add(Color::from(NEUTRAL_900))),
            Transform::from_translation(cell_position)
                .with_scale(Vec3::new(9., 9., 1.)),
            Holding {
                cell: num as u8,
                base_position: holding_position,
            },
        ));
    }
}

fn reset_game(game: &mut ResMut<Game>) {
    for line in game.matrix.iter_mut() {
        for cell in line.iter_mut() {
            *cell = CellState::Empty;
        }
    }
    for index in 0..QUEUE_SIZE {
        game.next_queue[index] = rand::random();
    }
    game.fall_cooldown = Timer::from_seconds(game.fall_cooldown_value, TimerMode::Once);
    game.score = 0;
    game.number_of_lines = 0;
    game.current_level = 1;
    game.holded_tetrimino = None;
}

fn test_reflesh(
    mut game: ResMut<Game>,
    mut next_state: ResMut<NextState<GamePhase>>,
) {
    reset_game(&mut game);
    next_state.set(GamePhase::Generation);
}
