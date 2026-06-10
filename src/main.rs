use minifb::{Key, Window, WindowOptions};
use std::time::Instant;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

fn main() {
    let mut window =
        Window::new("wolfenstein rust", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut buffer = vec![0u32; WIDTH * HEIGHT];

    let mut pos_x: f64 = 22.0;
    let mut pos_y: f64 = 12.0;
    let mut dir_x: f64 = -1.0;
    let mut dir_y: f64 = 0.0;

    let mut plane_x: f64 = 0.0;
    let mut plane_y: f64 = 0.5;

    let mut time = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {}
}
