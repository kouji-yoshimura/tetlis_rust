use bevy::prelude::*;
use crate::components::text::*;
use crate::resources::*;

pub fn refresh_texts(
    mut commands: Commands,
    query: Query<&Children, With<Indicator>>,
    q_child: Query<(Entity, &IndicatorType)>,
    game: Res<Game>,
) {
    for children in query.iter() {
        for &child in children.iter() {
            let child_res = q_child.get(child);
            if child_res.is_err() {
                continue;
            }

            let (entity, indicator_type) = child_res.unwrap();
            match indicator_type {
                IndicatorType::Score => {
                    commands.entity(entity).insert(Text::new(&game.score.to_string()));
                },
                IndicatorType::NumberOfLinesCleard => {
                    commands.entity(entity).insert(Text::new(&game.number_of_lines.to_string()));
                },
                IndicatorType::CurrentLevel => {
                    commands.entity(entity).insert(Text::new(&game.current_level.to_string()));
                },
            };
        }
    }
}
