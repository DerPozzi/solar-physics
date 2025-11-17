use bevy::prelude::*;

pub struct SolarSystem;

const SCALE_DOWN: f64 = 10_000_000.0; // 1 Worldunit = 10_000km
const G_WORLD: f64 = 6.67430e-11; // Gravitational constant

pub struct PlanetData {
    name: &'static str,
    radius: f64, // in meters
    mass: f64,   // in kg
    color: Color,
}

#[derive(Component)]
#[require(Collider)]
struct Planet;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Radius(f64);

#[derive(Component)]
struct Mass(f64);

#[derive(Component, Default)]
struct Collider;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

fn add_planets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let planets = solar_system_data();

    for (i, planet) in planets.iter().enumerate() {
        let mesh = meshes.add(Circle::new((planet.radius / SCALE_DOWN) as f32));
        let material = materials.add(planet.color);
        commands.spawn((
            Mesh2d(mesh),
            MeshMaterial2d(material),
            Planet,
            Name(planet.name.to_string()),
            Radius(planet.radius / SCALE_DOWN),
            Mass(planet.mass),
            Transform::from_translation(Vec3::new(i as f32 * 32.0, 0.0, 0.0)),
            Velocity(Vec2::ONE),
        ));
    }
}

fn bounce_on_window_edges(
    windows: Query<&Window>,
    mut query: Query<(&mut Transform, &mut Velocity, &Radius), With<Planet>>,
) {
    let window = windows.single().unwrap();

    let half_width = window.width() / 2.0;
    let half_height = window.height() / 2.0;

    for (mut transform, mut vel, radius) in &mut query {
        let radius = radius.0 as f32;
        let x = transform.translation.x;
        let y = transform.translation.y;

        // Linker Rand
        if x - radius < -half_width {
            transform.translation.x = -half_width + radius;
            vel.0.x *= -1.0; // Horizontalrichtung umkehren
        }

        // Rechter Rand
        if x + radius > half_width {
            transform.translation.x = half_width - radius;
            vel.0.x *= -1.0;
        }

        // Unterer Rand
        if y - radius < -half_height {
            transform.translation.y = -half_height + radius;
            vel.0.y *= -1.0; // Vertikalrichtung umkehren (abprallen)
        }

        // Oberer Rand
        if y + radius > half_height {
            transform.translation.y = half_height - radius;
            vel.0.y *= -1.0;
        }
    }
}

pub fn solar_system_data() -> Vec<PlanetData> {
    vec![
        PlanetData {
            name: "Sun",
            radius: 696_340_000.0,
            mass: 1.9885e30,
            color: Color::srgb(1.0, 0.85, 0.3),
        },
        // PlanetData {
        //     name: "Mercury",
        //     radius: 2_439_700.0,
        //     mass: 3.3011e23,
        //     color: Color::srgb(0.55, 0.55, 0.55),
        // },
        // PlanetData {
        //     name: "Venus",
        //     radius: 6_051_800.0,
        //     mass: 4.8675e24,
        //     color: Color::srgb(0.9, 0.8, 0.55),
        // },
        // PlanetData {
        //     name: "Earth",
        //     radius: 6_371_000.0,
        //     mass: 5.97237e24,
        //     color: Color::srgb(0.2, 0.4, 0.8),
        // },
        // PlanetData {
        //     name: "Mars",
        //     radius: 3_389_500.0,
        //     mass: 6.4171e23,
        //     color: Color::srgb(0.8, 0.3, 0.1),
        // },
        PlanetData {
            name: "Jupiter",
            radius: 69_911_000.0,
            mass: 1.8982e27,
            color: Color::srgb(0.8, 0.7, 0.5),
        },
        // PlanetData {
        //     name: "Saturn",
        //     radius: 58_232_000.0,
        //     mass: 5.6834e26,
        //     color: Color::srgb(0.9, 0.8, 0.5),
        // },
        // PlanetData {
        //     name: "Uranus",
        //     radius: 25_362_000.0,
        //     mass: 8.6810e25,
        //     color: Color::srgb(0.6, 0.85, 0.9),
        // },
        // PlanetData {
        //     name: "Neptune",
        //     radius: 24_622_000.0,
        //     mass: 1.02413e26,
        //     color: Color::srgb(0.2, 0.3, 0.9),
        // },
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

fn apply_velocity(mut query: Query<(&Name, &mut Transform, &Velocity), With<Planet>>) {
    for (name, mut trans, vel) in &mut query {
        if name.0 == "Sun" {
            continue;
        }
        trans.translation.x += vel.x;
        trans.translation.y += vel.y;
        println!("{} has vel of {}x, {}y", name.0, vel.x, vel.y);
    }
}

fn gravity_between_bodies(mut query: Query<(&mut Velocity, &Transform, &Mass), With<Planet>>) {
    let g: f64 = G_WORLD / (SCALE_DOWN * 10000000000.0); // dein „Spiel-G“, du kannst später realistisch werden
    let eps: f32 = 1.0; // softening

    let mut combos = query.iter_combinations_mut();

    while let Some([(mut vel1, trans1, mass1), (mut vel2, trans2, mass2)]) = combos.fetch_next() {
        let p1 = trans1.translation.truncate(); // Vec2
        let p2 = trans2.translation.truncate();

        let r = p2 - p1;
        let dist_sq = r.length_squared() + eps * eps;
        let dist = dist_sq.sqrt();
        let dir = r / dist; // u

        // Beschleunigung 1 durch 2
        let a1 = dir * ((g * mass2.0) as f32 / dist_sq);

        // Beschleunigung 2 durch 1 (entgegengesetzt)
        let a2 = -dir * ((g * mass1.0) as f32 / dist_sq);

        // Velocity updaten (v = v + a * dt)
        vel1.0 += a1;
        vel2.0 += a2;
    }
}

impl Plugin for SolarSystem {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup, (add_planets, print_planets).chain()));
        app.add_systems(
            FixedUpdate,
            (
                apply_velocity,
                // bounce_on_window_edges,
                gravity_between_bodies,
            ),
        );
    }
}
