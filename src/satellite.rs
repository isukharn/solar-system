use std::f32::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4};

use bevy::prelude::*;

use crate::orbit::{Orbit, OrbitPlugin, OrbitalElements};

const EARTH_DIAMETER: f32 = 12_742.; // km
const ALTITUDE: f32 = 400.0; // km
const MAGNIFICATION: f32 = 10000.0; // 1000x

#[derive(Component, Debug)]
struct Satellite;

pub struct SatellitePlugin;

impl Plugin for SatellitePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(OrbitPlugin).add_systems(Startup, spawn);
    }
}

fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let inclinations = vec![0.0, FRAC_PI_4, -FRAC_PI_3, 3.0 * FRAC_PI_2];
    let longitudes = vec![0.0, -FRAC_PI_4, 0.0, 0.0];
    for (i, l) in inclinations.into_iter().zip(longitudes) {
        commands.spawn((
            Mesh3d(
                meshes.add(
                    Sphere {
                        radius: 0.009 * MAGNIFICATION,
                    }
                    .mesh()
                    .uv(32, 18),
                ),
            ),
            MeshMaterial3d(materials.add(Color::WHITE)),
            Transform::default().with_translation(Vec3::new(
                EARTH_DIAMETER / 2.0 + ALTITUDE,
                0.0,
                0.0,
            )),
            Orbit {
                speed: 7.67, // km/s with time speed in mind
                ..default()
            },
            OrbitalElements {
                inclination: i,
                longitude_ascending: l,
                ..default()
            },
            Satellite {},
        ));
    }
}
