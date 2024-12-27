use bevy::prelude::*;
use bevy::color::palettes::tailwind::*;
use crate::components::tetrimino::*;
use crate::components::cell::*;
use crate::components::text::*;
use crate::resources::*;

pub fn setup_resources(
    mut game: ResMut<Game>,
) {
    reset_resources(&mut game);
}

pub fn setup_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    spawn_matrix(&mut commands, &mut meshes, &mut materials);
    spawn_indicator_texts(&mut commands);
    spawn_next_queue(&mut commands, &mut meshes, &mut materials);
    spawn_holding_box(&mut commands, &mut meshes, &mut materials);
}

pub fn reset_resources(game: &mut ResMut<Game>) {
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

fn spawn_matrix(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
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
}

fn spawn_indicator_texts(
    commands: &mut Commands,
) {
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
}

fn spawn_next_queue(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    for queue_number in 0..QUEUE_SIZE {
        let position = Vec3::new(140., (QUEUE_SIZE - queue_number - 1) as f32 * 60. - 160., 0.);
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::default())),
            MeshMaterial2d(materials.add(Color::from(NEUTRAL_900))),
            Transform::from_translation(position)
                .with_scale(Vec3::new(50., 50., 0.)),
        ));
        for num in 0..NUMBER_OF_TETRIMINO_BLOCKS {
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
}

fn spawn_holding_box(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let position = Vec3::new(-160., 165., 0.);
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::from(NEUTRAL_900))),
        Transform::from_translation(position)
            .with_scale(Vec3::new(50., 50., 0.)),
    ));
    for num in 0..NUMBER_OF_TETRIMINO_BLOCKS {
        let cell_position = Vec3::new(position.x + (num as f32 - 2.) * 10. + 5., position.y, 0.);
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::default())),
            MeshMaterial2d(materials.add(Color::from(NEUTRAL_900))),
            Transform::from_translation(cell_position)
                .with_scale(Vec3::new(9., 9., 1.)),
            Holding {
                cell: num as u8,
                base_position: position,
            },
        ));
    }
}
