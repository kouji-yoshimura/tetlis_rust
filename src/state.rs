use bevy::prelude::*;
use crate::resources::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GamePhase {
    #[default]
    Generation,
    Holding,
    Falling,
    Lock,
    Animate,
    Eliminate,
    Check,
    GameOver,
}

pub fn transition_phase(
    state: Res<State<GamePhase>>,
    mut next_state: ResMut<NextState<GamePhase>>,
    mut game: ResMut<Game>,
) {
    match state.get() {
        GamePhase::Generation => next_state.set(GamePhase::Falling),
        GamePhase::Holding => next_state.set(GamePhase::Falling),
        GamePhase::Falling => {
            if game.is_reached_bottom {
                game.is_reached_bottom = false;
                next_state.set(GamePhase::Lock);
            }
        },
        GamePhase::Lock => next_state.set(GamePhase::Eliminate),
        GamePhase::Eliminate => next_state.set(GamePhase::Check),
        GamePhase::Check => {
            if game.is_game_over {
                next_state.set(GamePhase::GameOver);
            } else {
                next_state.set(GamePhase::Generation);
            }
        },
        _ => {},
    };
}
