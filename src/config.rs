use bevy::prelude::*;

#[derive(Debug, Clone, Default)]
enum TimeSpeed {
    SECOND,
    MINUTE,
    MINUTES10,
    MINUTES30,
    #[default]
    HOUR,
    DAY,
}

impl TimeSpeed {
    fn to_seconds(&self) -> f32 {
        match self {
            TimeSpeed::SECOND => 1.0,
            TimeSpeed::MINUTE => 60.0,
            TimeSpeed::MINUTES10 => 10.0 * 60.0,
            TimeSpeed::MINUTES30 => 30.0 * 60.0,
            TimeSpeed::HOUR => 60.0 * 60.0,
            TimeSpeed::DAY => 24.0 * 60.0 * 60.0,
        }
    }
    fn to_description(&self) -> &str {
        match self {
            TimeSpeed::SECOND => "1 second",
            TimeSpeed::MINUTE => "1 minute",
            TimeSpeed::MINUTES10 => "10 minutes",
            TimeSpeed::MINUTES30 => "30 minutes",
            TimeSpeed::HOUR => "1 hour",
            TimeSpeed::DAY => "1 day",
        }
    }
}

#[derive(Resource, Debug)]
pub struct Config {
    time_speed: TimeSpeed,
    pub time_scale: f32,
}

#[derive(Component, Debug)]
struct ConfigInfo;

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn)
            .add_systems(Update, (input, update).chain());
    }
}

fn spawn(mut commands: Commands) {
    commands.spawn((Text::new("1 second = 1 second"), ConfigInfo {}));

    commands.insert_resource(Config {
        time_speed: TimeSpeed::default(),
        time_scale: TimeSpeed::default().to_seconds(),
    });
}

fn input(keyboard: Res<ButtonInput<KeyCode>>, mut config: ResMut<Config>) {
    if keyboard.just_pressed(KeyCode::Equal) {
        config.time_speed = match config.time_speed {
            TimeSpeed::SECOND => TimeSpeed::MINUTE,
            TimeSpeed::MINUTE => TimeSpeed::MINUTES10,
            TimeSpeed::MINUTES10 => TimeSpeed::MINUTES30,
            TimeSpeed::MINUTES30 => TimeSpeed::HOUR,
            TimeSpeed::HOUR => TimeSpeed::DAY,
            _ => config.time_speed.clone(),
        }
    }
    if keyboard.just_pressed(KeyCode::Minus) {
        config.time_speed = match config.time_speed {
            TimeSpeed::DAY => TimeSpeed::HOUR,
            TimeSpeed::HOUR => TimeSpeed::MINUTES30,
            TimeSpeed::MINUTES30 => TimeSpeed::MINUTES10,
            TimeSpeed::MINUTES10 => TimeSpeed::MINUTE,
            TimeSpeed::MINUTE => TimeSpeed::SECOND,
            _ => config.time_speed.clone(),
        }
    }

    config.time_scale = config.time_speed.to_seconds();
}

fn update(config: Res<Config>, mut query: Query<&mut Text, With<ConfigInfo>>) {
    let mut txt = query.single_mut();

    let s = config.time_speed.to_description();
    **txt = format!("1 second = {s}");
}
