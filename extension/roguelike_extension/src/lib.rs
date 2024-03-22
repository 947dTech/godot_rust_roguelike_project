use godot::prelude::*;

struct RogueLikeExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RogueLikeExtension {}

mod static_map;
mod map_generator;
mod dynamic_map;
mod game_master;
