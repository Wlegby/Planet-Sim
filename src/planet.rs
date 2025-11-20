use crate::{
    rel_pos_to_w_pos,
    settings::{self, Setting},
};
use macroquad::prelude::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Planet {
    pub pos: Vec2,      // the position in 10⁹m
    pub mass: f64,      // in 10²⁴kg
    pub radius: f64,    // in 10⁶m
    pub velocity: Vec2, // km/s
    pub color: Color,   // just the displayed planet color
}

impl Planet {
    pub fn new(pos: Vec2, mass: f64, radius: f64, velocity: Vec2, color: Color) -> Self {
        Self {
            pos,
            mass,
            radius,
            velocity,
            color,
        }
    }

    pub fn place_in_orbit(
        distance: f32,
        mass: f64,
        radius: f64,
        orbital_speed: f32,
        color: Color,
        phase: f32,
    ) -> Self {
        let x = distance * phase.cos();
        let y = distance * phase.sin();
        let vx = -orbital_speed * phase.sin();
        let vy = orbital_speed * phase.cos();

        Planet::new(Vec2::new(x, y), mass, radius, Vec2::new(vx, vy), color)
    }

    pub fn move_self(&mut self, dt: f32) {
        self.pos += self.velocity * dt / 1e6; // adjust for the velocity to be in km/s
    }

    pub fn draw(&self, settings: &Setting) {
        let pos = rel_pos_to_w_pos(self.pos * settings.distance_scale as f32);
        let circle_rad = self.radius as f32 * settings.size_scale as f32;

        draw_circle(
            pos.x - settings.cam_pos.x,
            pos.y - settings.cam_pos.y,
            circle_rad.max(1.),
            self.color,
        );
    }
}
