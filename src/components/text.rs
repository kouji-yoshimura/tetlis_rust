use bevy::prelude::*;

#[derive(Component)]
pub struct Indicator;

#[derive(Component, Copy, Clone, PartialEq, Eq)]
pub enum IndicatorType {
    Score,
    NumberOfLinesCleard,
    CurrentLevel,
}


