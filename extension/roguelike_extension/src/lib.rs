//! # Roguelike Extension

use godot::prelude::*;

struct RogueLikeExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RogueLikeExtension {}

pub mod static_map;
pub mod map_generator;
pub mod dynamic_map;
pub mod game_master;
pub mod player;
pub mod mob;
pub mod item;
