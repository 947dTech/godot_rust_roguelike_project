use godot::prelude::*;

pub struct DynamicMapManager {
    pub player_position: (i32, i32),
    pub item_positions: Vec<(i32, i32)>,
    pub enemy_positions: Vec<(i32, i32)>,
}

impl DynamicMapManager {
    pub fn new() -> Self {
        Self {
            player_position: (0, 0),
            item_positions: vec![],
            enemy_positions: vec![],
        }
    }
}
