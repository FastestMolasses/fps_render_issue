use crate::character_controller::fps::FpsControllerPlugin;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((FpsControllerPlugin,));
    }
}
