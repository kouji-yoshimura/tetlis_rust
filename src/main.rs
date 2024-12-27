use bevy::prelude::*;
use bevy::input::common_conditions::*;
use tetlis_rust::animate::*;
use tetlis_rust::check::*;
use tetlis_rust::eliminate::*;
use tetlis_rust::falling::*;
use tetlis_rust::generation::*;
use tetlis_rust::holding::*;
use tetlis_rust::indicator::*;
use tetlis_rust::lock::*;
use tetlis_rust::resources::*;
use tetlis_rust::setup::*;
use tetlis_rust::state::*;

fn main() {
    App::new()
        .init_resource::<Game>()
        .add_plugins(DefaultPlugins)
        .insert_state(GamePhase::Generation)
        .add_systems(Startup, (
            setup_resources,
            setup_entities,
        ))
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
            (
                handle_restart,
                refresh_queue,
                refresh_holding,
            ).chain()
                .run_if(input_just_pressed(KeyCode::KeyQ)),
            transition_phase,
            refresh_texts,
        ))
        .run();
}

fn handle_restart(
    mut game: ResMut<Game>,
    mut next_state: ResMut<NextState<GamePhase>>,
) {
    reset_resources(&mut game);
    next_state.set(GamePhase::Generation);
}
