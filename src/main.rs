use minifb::{Key, Window, WindowOptions};
use std::time::Instant;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

const MAP: [[u8; 24]; 24] = [
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 1, 1, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1,
    ],
    [
        1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
];

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

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let frame_time = time.elapsed().as_secs_f64();
        time = Instant::now();

        let move_speed = frame_time * 5.0;
        let rot_speed = frame_time * 3.0;

        if window.is_key_down(Key::W) {
            let new_x = pos_x + dir_x * move_speed;
            let new_y = pos_y + dir_y * move_speed;

            if MAP[new_y as usize][new_x as usize] == 0 {
                pos_x = new_x;
                pos_y = new_y;
            }
        }
        if window.is_key_down(Key::S) {
            let new_x = pos_x - dir_x * move_speed;
            let new_y = pos_y - dir_y * move_speed;

            if MAP[new_y as usize][new_x as usize] == 0 {
                pos_x = new_x;
                pos_y = new_y;
            }
        }

        if window.is_key_down(Key::A) {
            let new_x = pos_x - plane_x * move_speed;
            let new_y = pos_y - plane_y * move_speed;

            if MAP[new_y as usize][new_x as usize] == 0 {
                pos_x = new_x;
                pos_y = new_y;
            }
        }

        if window.is_key_down(Key::D) {
            let new_x = pos_x + plane_x * move_speed;
            let new_y = pos_y + plane_y * move_speed;

            if MAP[new_y as usize][new_x as usize] == 0 {
                pos_x = new_x;
                pos_y = new_y;
            }
        }

        for x in 0..WIDTH {
            let camera_x = 2.0 * x as f64 / WIDTH as f64 - 1.0;
            let ray_dir_x = dir_x + plane_x * camera_x;
            let ray_dir_y = dir_y + plane_y * camera_x;

            let map_x = pos_x as i64;
            let map_y = pos_y as i64;

            let delta_dist_x = if ray_dir_x == 0.0 {
                1e30
            } else {
                (1.0 / ray_dir_x).abs()
            };

            let delta_dist_y = if ray_dir_y == 0.0 {
                1e30
            } else {
                (1.0 / ray_dir_y).abs()
            };

            let (step_x, side_dist_x) = if ray_dir_x < 0.0 {
                (-1, (pos_x - map_x as f64) * delta_dist_x)
            } else {
                (1, (map_x as f64 + 1.0 - pos_x) * delta_dist_x)
            };

            let (step_y, side_dist_y) = if ray_dir_y < 0.0 {
                (-1, (pos_y - map_y as f64) * delta_dist_y)
            } else {
                (1, (map_y as f64 + 1.0 - pos_y) * delta_dist_y)
            };

            let mut mx = map_x;
            let mut my = map_y;
            let mut sd_x = side_dist_x;
            let mut sd_y = side_dist_y;
            let mut hit = false;
            let mut side = 0;

            while !hit {
                if sd_x < sd_y {
                    sd_x += delta_dist_x;
                    mx += step_x;
                    side = 0;
                } else {
                    sd_y += delta_dist_y;
                    my += step_y;
                    side = 1;
                }
                if MAP[my as usize][mx as usize] > 0 {
                    hit = true;
                }
            }

            let perp_dist = if side == 0 {
                (mx as f64 - pos_x + (1 - step_x) as f64 / 2.0) / ray_dir_x
            } else {
                (my as f64 - pos_y + (1 - step_y) as f64 / 2.0) / ray_dir_y
            };

            let line_height = (HEIGHT as f64 / perp_dist) as i64;
            let draw_start = (-line_height / 2 + HEIGHT as i64 / 2).max(0);
            let draw_end = (line_height / 2 + HEIGHT as i64 / 2).min(HEIGHT as i64 - 1);

            let color: u32 = if side == 0 { 0xFF0000 } else { 0x00FF00 };

            for y in 0..HEIGHT {
                let idx = y * WIDTH + x;
                if y < draw_start as usize {
                    buffer[idx] = 0x222222;
                } else if y <= draw_end as usize {
                    buffer[idx] = color;
                } else {
                    buffer[idx] = 0x333333;
                }
            }
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
