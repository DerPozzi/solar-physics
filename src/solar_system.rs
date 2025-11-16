use bevy::prelude::*;

pub struct SolarSystem;

pub struct PlanetData {
    pub name: &'static str,
    pub radius: f64, // in meters
    pub mass: f64,   // in kg
}

#[derive(Component)]
struct Planet;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Radius(f64);

#[derive(Component)]
struct Mass(f64);

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

fn add_planets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let planets = solar_system_data();

    for planet in planets {
        let mesh = meshes.add(Circle::default());
        let material = materials.add(Color::srgb(1.0, 1.0, 1.0));
        commands.spawn((
            Mesh2d(mesh),
            MeshMaterial2d(material),
            Planet,
            Name(planet.name.to_string()),
            Radius(planet.radius),
            Mass(planet.mass),
            Transform::from_translation(Vec3::ZERO),
            Velocity(Vec2::ZERO),
        ));
    }
}

pub fn solar_system_data() -> Vec<PlanetData> {
    vec![
        PlanetData {
            name: "Sonne",
            radius: 696_340_000.0,
            mass: 1.9885e30,
        },
        PlanetData {
            name: "Merkur",
            radius: 2_439_700.0,
            mass: 3.3011e23,
        },
        PlanetData {
            name: "Venus",
            radius: 6_051_800.0,
            mass: 4.8675e24,
        },
        PlanetData {
            name: "Erde",
            radius: 6_371_000.0,
            mass: 5.97237e24,
        },
        PlanetData {
            name: "Mars",
            radius: 3_389_500.0,
            mass: 6.4171e23,
        },
        PlanetData {
            name: "Jupiter",
            radius: 69_911_000.0,
            mass: 1.8982e27,
        },
        PlanetData {
            name: "Saturn",
            radius: 58_232_000.0,
            mass: 5.6834e26,
        },
        PlanetData {
            name: "Uranus",
            radius: 25_362_000.0,
            mass: 8.6810e25,
        },
        PlanetData {
            name: "Neptun",
            radius: 24_622_000.0,
            mass: 1.02413e26,
        },
    ]
}

fn print_planets(query: Query<(&Name, &Radius, &Mass, &Velocity), With<Planet>>) {
    for (name, radius, mass, vel) in &query {
        println!(
            "Planet: {} - {}kg - {}m - {}x/{}y",
            name.0, mass.0, radius.0, vel.x, vel.y
        );
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

impl Plugin for SolarSystem {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup, (add_planets, print_planets).chain()));
    }
}
