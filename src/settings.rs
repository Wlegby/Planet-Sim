use macroquad::math::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct Setting {
    pub size_scale: f64,
    pub distance_scale: f64,
    pub sun_size: f64,
    pub cam_pos: Vec2,
    pub focused_planet: usize,
}

impl Default for Setting {
    fn default() -> Self {
        Self::unrealistic()
    }
}

impl Setting {
    pub fn new(
        size_scale: f64,
        distance_scale: f64,
        sun_size: f64,
        cam_pos: Vec2,
        focused_planet: usize,
    ) -> Self {
        Self {
            size_scale,
            distance_scale,
            sun_size,
            cam_pos,
            focused_planet,
        }
    }

    // full system view (not correct ratio between size & distance)
    pub fn unrealistic() -> Self {
        Self::new(0.6, 0.275, 10., Vec2::default(), 0)
    }

    pub fn realistic() -> Self {
        Self::new(0.275 / 1e3, 0.275, 695.508, Vec2::default(), 0)
    }
}
