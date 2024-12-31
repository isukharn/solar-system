use bevy::{color::palettes::css::*, prelude::*};
use std::f32::consts::{FRAC_PI_2, TAU};

use crate::config::Config;

const ORBIT_RESOLUTION: usize = 1000;

#[derive(Component, Default, Debug)]
pub struct OrbitalElements {
    pub inclination: f32, // angle between the reference plane and orbital plane (degrees)
    pub longitude_ascending: f32, // Ω (degrees)
    pub _arg_periapsis: f32, // ω (degrees)
    pub _eccentricity: f32, //
    pub _semi_major_axis: f32, // (m)
    pub _mean_anomaly: f64, // (degrees)
}

#[derive(Component, Debug)]
#[require(OrbitalElements)]
pub struct Orbit {
    pub speed: f32, // km/s
    pub angle: f32, // radiants
}

impl Default for Orbit {
    fn default() -> Self {
        Self {
            speed: 0.0,
            angle: FRAC_PI_2,
        }
    }
}

pub struct OrbitPlugin;

impl Plugin for OrbitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, orbit);
    }
}

fn orbit(
    mut gizmos: Gizmos,
    time: Res<Time>,
    config: Res<Config>,
    mut query: Query<(&mut Transform, &mut Orbit, &OrbitalElements)>,
) {
    // Move and draw orbit trace
    for (mut trans, mut orbit, orbit_elem) in query.iter_mut() {
        let height = trans.translation.distance(Vec3::ZERO);
        let angle_change = config.time_scale * orbit.speed * time.delta_secs() / height;
        orbit.angle += angle_change;
        if orbit.angle > TAU {
            orbit.angle -= TAU;
        }

        let domain = Interval::EVERYWHERE;
        // trace a circle on the xz plane
        let curve = FunctionCurve::new(domain, |t| {
            // circle
            let mut v = Vec3 {
                x: ops::sin(t),
                y: 0.0,
                z: ops::cos(t),
            };
            // rotate around X
            v = Quat::from_rotation_x(orbit_elem.inclination) * v;
            // rotate around Y
            v = Quat::from_rotation_y(orbit_elem.longitude_ascending) * v;
            // Adjust R
            v * height
        });
        let times_and_colors = (0..=ORBIT_RESOLUTION)
            .map(|n| n as f32 / ORBIT_RESOLUTION as f32)
            .map(|n| n * TAU + orbit.angle) // full circle
            .map(|t| (t, DARK_SLATE_GRAY.with_alpha((t - orbit.angle) / TAU)));

        // Move the entity
        if let Some(s) = curve.sample(orbit.angle) {
            trans.translation = s;
        }
        // draw orbit trace
        gizmos.curve_gradient_3d(curve, times_and_colors);
    }
}
