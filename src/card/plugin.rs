use bevy::prelude::*;
use crate::combat::systems::resolve_combat;

use super::systems::*;

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_hand);        
        app.add_systems(Startup, spawn_enemy_hand);
        app.add_systems(Startup, resolve_combat.after(spawn_hand));
    }
}

