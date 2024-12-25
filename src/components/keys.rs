use bevy::prelude::*;

pub const MOVE_RIGHT: [KeyCode; 1]               = [KeyCode::ArrowRight];
pub const MOVE_LEFT: [KeyCode; 1]                = [KeyCode::ArrowLeft];
pub const ROTATE_CLOCKWISE: [KeyCode; 2]         = [KeyCode::ArrowUp, KeyCode::KeyX];
pub const ROTATE_COUNTER_CLOCKWISE: [KeyCode; 1] = [KeyCode::KeyZ];
pub const HOLD: [KeyCode; 3]                     = [KeyCode::ShiftLeft, KeyCode::ShiftRight, KeyCode::KeyC];
pub const HARD_DROP: [KeyCode; 1]                = [KeyCode::Space];
pub const SOFT_DROP: [KeyCode; 1]                = [KeyCode::ArrowDown];
