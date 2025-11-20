use macroquad::{math::Vec2, time::get_frame_time};

use crate::{
    planet::{self, Planet},
    settings::Setting,
    G, TIME_SCALE,
};

pub fn planet_physics(planets: &Vec<Planet>, dt: f32) -> Vec<Planet> {
    let mut adjusted_gravity = gravity(planets, dt);
    move_planets(&mut adjusted_gravity, dt);

    adjusted_gravity
}

pub fn move_planets(planets: &mut Vec<Planet>, dt: f32) {
    // skip the first one, to make the sun stationary
    for p in planets.iter_mut().skip(1) {
        p.move_self(dt);
    }
}

pub fn gravity(planets: &Vec<Planet>, dt: f32) -> Vec<Planet> {
    let mut updated_planets = planets.clone();

    for (i, p) in planets.iter().enumerate() {
        let mut acceleration = Vec2::default();

        for other_p in planets {
            if p == other_p {
                continue;
            }

            let d = p.pos.distance(other_p.pos) as f64;
            let force = G * (p.mass * other_p.mass) / d.powf(2.);
            let force_vec = (other_p.pos - p.pos).normalize() * force as f32;

            acceleration += force_vec / p.mass as f32;
        }

        updated_planets[i].velocity += acceleration * dt;
    }

    updated_planets
}
