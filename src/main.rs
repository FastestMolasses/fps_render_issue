use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use fps_render_issue::WorldPlugin;

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 10000.0,
        })
        .insert_resource(ClearColor(Color::linear_rgb(0.83, 0.96, 0.96)))
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "fps_test_01".to_string(),
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
            WorldPlugin,
        ))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .run();
}
