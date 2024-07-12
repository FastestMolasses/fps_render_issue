use crate::GameState;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Playing)
                .load_collection::<TestMapAsset>(),
        );
    }
}

// The following asset collections will be loaded during the State `GameState::Loading`
// When done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct TestMapAsset {
    #[asset(path = "room01.glb")]
    pub test_map: Handle<Gltf>,
}
