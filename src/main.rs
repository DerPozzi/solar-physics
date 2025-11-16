use bevy::prelude::*;

mod solar_system;

use solar_system::SolarSystem;

fn main() {
    App::new().add_plugins(DefaultPlugins).add_plugins(SolarSystem).run();
}
