mod camera;
mod config;
mod debug;
mod earth;
mod orbit;
mod satellite;

use bevy::prelude::*;

use camera::CameraPlugin;
use config::ConfigPlugin;
use debug::DebugPlugin;
use earth::EarthPlugin;
use satellite::SatellitePlugin;

fn main() {
    App::new()
        // Bevy built-ins.
        .insert_resource(ClearColor(Color::NONE))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 1000.0,
            ..default()
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Earth!".into(),
                name: Some("Solar System".into()),
                ..default()
            }),
            ..default()
        }))
        // User defined plugins.
        .add_plugins(DebugPlugin)
        .add_plugins(ConfigPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(EarthPlugin)
        .add_plugins(SatellitePlugin)
        .run();
}
