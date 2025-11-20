#![allow(unused)]

// I have given the planets a minimum display size, so if the planet size would be too small, it
// get displayed by a 1px dot

mod physics;
mod planet;
mod settings;

use physics::*;
use planet::*;
use settings::*;

use macroquad::{
    miniquad::{native::linux_x11::libx11::Time, window},
    prelude::*,
};

// the bigger the number the more wrong the simulation gets (because of software limitations)
const TIME_SCALE: f32 = 1. * 60. * 60. * 24. * 7.; // number of simulated secons for each real s

// adjusted G so that i can leave the distances in 10⁹m and masses in 10²⁴kg
const G: f64 = 6.6743e-8 as f64;

#[macroquad::main("Gravity Sim")]
async fn main() {
    let mut settings = Setting::unrealistic();

    let mut planets = solar_system(&settings);

    let mut sim_time: f32 = 0.0; // simulated seconds

    loop {
        settings.cam_pos = planets[settings.focused_planet].pos * settings.distance_scale as f32;

        for p in &planets {
            p.draw(&settings);
        }

        let dt = get_frame_time() * TIME_SCALE;
        sim_time += dt;

        let days = (sim_time / (60.0 * 60.0 * 24.0)) as u32;
        let years = days / 365;

        draw_text(
            &format!("Sim time: {} years ({} days)", years, days),
            10.,
            40.,
            30.,
            WHITE,
        );

        planets = planet_physics(&planets, dt);

        handle_input(&mut settings, planets.len());
        planets[0].radius = settings.sun_size;

        next_frame().await;
    }
}

fn handle_input(settings: &mut Setting, num: usize) {
    // Zoom in/out
    if is_key_down(KeyCode::Up) {
        settings.distance_scale *= 1.01;
    }
    if is_key_down(KeyCode::Down) {
        settings.distance_scale /= 1.01;
    }
    // Size planets
    if is_key_down(KeyCode::Right) {
        settings.size_scale *= 1.01;
    }
    if is_key_down(KeyCode::Left) {
        settings.size_scale /= 1.01;
    }
    // size sun
    if is_key_down(KeyCode::W) {
        settings.sun_size *= 1.01;
    }
    if is_key_down(KeyCode::S) {
        settings.sun_size /= 1.01;
    }

    // Zoom in/out and scale planets
    if is_key_down(KeyCode::Home) {
        settings.distance_scale *= 1.01;
        settings.size_scale *= 1.01;
    }
    if is_key_down(KeyCode::End) {
        settings.distance_scale /= 1.01;
        settings.size_scale /= 1.01;
    }
    if is_key_pressed(KeyCode::N) {
        settings.focused_planet += 1;
        settings.focused_planet %= num;
    }
    if is_key_pressed(KeyCode::P) {
        if settings.focused_planet > 0 {
            settings.focused_planet -= 1;
        } else {
            settings.focused_planet = num - 1;
        }
    }
}

// A function that adjusts positions so that (0, 0) is the center of the window
pub fn rel_pos_to_w_pos(p: Vec2) -> Vec2 {
    let (ww, wh) = window::screen_size();

    Vec2::new(p.x + ww / 2., p.y + wh / 2.)
}

pub fn solar_system(settings: &Setting) -> Vec<Planet> {
    let mut planets = Vec::new();

    let sun = Planet::new(
        Vec2::new(0., 0.),
        1988000.,
        settings.sun_size,
        Vec2::new(0., 0.),
        YELLOW,
    );
    planets.push(sun);

    let merkur = Planet::place_in_orbit(
        57.9092, 0.33, 2.4397, 47.87, BROWN, 0.0, // phase in radians
    );
    planets.push(merkur);

    let venus = Planet::place_in_orbit(
        108.207,
        4.8673,
        6.0518,
        35.02,
        BEIGE,
        std::f32::consts::FRAC_PI_4, // 45°
    );
    planets.push(venus);

    let earth = Planet::place_in_orbit(
        149.6001,
        5.97,
        6.37814,
        29.78,
        BLUE,
        std::f32::consts::FRAC_PI_2, // 90°
    );
    planets.push(earth.clone());

    let mars = Planet::place_in_orbit(
        227.936,
        0.64169,
        3.3962,
        24.077,
        RED,
        std::f32::consts::PI, // 180°
    );
    planets.push(mars);

    let jupiter = Planet::place_in_orbit(
        778.5,
        1898.5,
        71.492,
        13.07,
        BROWN,
        std::f32::consts::PI + std::f32::consts::FRAC_PI_4, // 225°
    );
    planets.push(jupiter);

    let saturn = Planet::place_in_orbit(
        1433.,
        568.46,
        60.268,
        9.69,
        BEIGE,
        std::f32::consts::PI + std::f32::consts::FRAC_PI_2, // 270°
    );
    planets.push(saturn);

    let uranus = Planet::place_in_orbit(
        2872.,
        86.818,
        25.559,
        6.81,
        BLUE,
        1.25 * std::f32::consts::PI, // ~225°
    );
    planets.push(uranus);

    let neptun = Planet::place_in_orbit(
        4495.,
        102.45,
        24.764,
        5.43,
        BLUE,
        0. * std::f32::consts::PI, // ~315°
    );
    planets.push(neptun);

    planets
}
