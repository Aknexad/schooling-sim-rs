use macroquad::prelude::*;

use crate::window_conf;

#[derive(Debug)]
pub struct Fish {
    pub position: [f32; 2],
    pub velocity: f32,
}

impl Fish {
    pub fn draw(&self) {
        let original_x = self.position[0];
        let original_y = self.position[1];

        draw_poly(original_x, original_y, 3, 5.1, 0.0, BLUE);
    }

    pub fn update(&mut self) {
        let default_windows_size = window_conf::default();
        let dt = get_frame_time();

        // X
        if is_key_down(KeyCode::Right) {
            if self.position[0] > default_windows_size.window_width as f32 {
                self.position[0] = 0.0;
            }

            self.position[0] += self.velocity * dt;
        }
        if is_key_down(KeyCode::Left) {
            if self.position[0] <= 0.0 {
                self.position[0] = default_windows_size.window_width as f32;
            }

            self.position[0] -= self.velocity * dt;
        }

        // Y
        if is_key_down(KeyCode::Up) {
            if self.position[1] < 0.0 {
                self.position[1] = default_windows_size.window_height as f32
            }

            self.position[1] -= self.velocity * dt;
        }
        if is_key_down(KeyCode::Down) {
            if self.position[1] > default_windows_size.window_height as f32 {
                self.position[1] = 0.0
            }

            self.position[1] += self.velocity * dt;
        }
    }

    pub fn flock(fish_count: usize) -> Vec<Fish> {
        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        let mut flock: Vec<Fish> = Vec::new();

        while flock.len() < fish_count {
            let angle = rand::gen_range(0.0, 2.0 * std::f32::consts::PI);
            let radius: f32 = fish_count as f32 * rand::gen_range::<f32>(0.0, 1.0).sqrt();

            let x = center_x + radius * angle.cos();
            let y = center_y + radius * angle.sin();
            let collisions = eliminate_collisions(&mut flock, x, y);

            if !collisions {
                flock.push(Fish {
                    position: [x, y],
                    velocity: 300.0,
                });
            }
        }

        flock
    }
}

fn eliminate_collisions(flock: &mut Vec<Fish>, new_fish_x: f32, new_fish_y: f32) -> bool {
    let mut collisions: bool = false;
    let min_distance = 7.7;
    for fish in flock {
        let dx = fish.position[0] - new_fish_x;
        let dy = fish.position[1] - new_fish_y;
        let dist = (dx * dx) + (dy * dy);

        if dist < min_distance {
            collisions = true;
            break;
        }
    }

    collisions
}

// let mouse_position = mouse_position();

// self.position[0] = mouse_position.0 + 5.5;
// self.position[1] = mouse_position.1 + 5.5;

// if mouse_position.0 != self.position[0] && mouse_position.0 != 0.0 {
//     self.position[0] += self.velocity * dt;
// }
