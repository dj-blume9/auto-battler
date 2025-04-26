use bevy::prelude::*;
use card::plugin::CardPlugin;
mod card;
mod combat;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CardPlugin))
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
