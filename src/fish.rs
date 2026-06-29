use macroquad::prelude::*;

use crate::window_conf;
pub struct Fish {
    pub position: [f32; 2],
    pub velocity: f32,
}

impl Fish {
    pub fn draw(&self) {
        draw_poly(self.position[0], self.position[1], 3, 20.1, 0.0, BLUE);
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
}

// let mouse_position = mouse_position();

// self.position[0] = mouse_position.0 + 5.5;
// self.position[1] = mouse_position.1 + 5.5;

// if mouse_position.0 != self.position[0] && mouse_position.0 != 0.0 {
//     self.position[0] += self.velocity * dt;
// }
