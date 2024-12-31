use bevy::prelude::*;
use std::f32::consts::TAU;

use crate::config::Config;

const DIAMETER: f32 = 12_742.; // km
const RADIUS: f32 = DIAMETER / 2.;
const SCALE: Vec3 = Vec3::splat(RADIUS / 500.);
const ROTATION_SPEED: f32 = TAU / (24. * 60.0 * 60.0); // rad per earth second

#[derive(Component, Debug)]
#[require(SceneRoot)]
struct Earth;

pub struct EarthPlugin;

impl Plugin for EarthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn)
            .add_systems(Update, (draw_axis, rotate));
    }
}

fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("Earth.glb"))),
        Transform::from_scale(SCALE),
        Earth {},
    ));
}

fn draw_axis(mut gizmos: Gizmos) {
    gizmos.line(
        Vec3::new(0., -110. - RADIUS, 0.),
        Vec3::new(0., 110. + RADIUS, 0.),
        Color::WHITE,
    );
    // gizmos.line(
    //     Vec3::new(-110. - RADIUS, 0., 0.),
    //     Vec3::new(110. + RADIUS, 0., 0.),
    //     Color::WHITE,
    // );
}

fn rotate(time: Res<Time>, config: Res<Config>, mut query: Query<&mut Transform, With<Earth>>) {
    for mut transform in query.iter_mut() {
        transform.rotate_y(config.time_scale * ROTATION_SPEED * time.delta_secs());
    }
}
