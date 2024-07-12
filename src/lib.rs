#![allow(clippy::type_complexity)]

mod character_controller;
mod loading;
mod game;
mod player;
mod setup;

use crate::loading::LoadingPlugin;
use crate::game::GamePlugin;

use bevy::app::App;
use bevy::prelude::*;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    /// During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    /// Here the menu is drawn and waiting for player interaction
    Menu,
    /// During this State the actual game logic is executed
    Playing,
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_plugins((
            LoadingPlugin,
            GamePlugin,
        ));
    }
}
