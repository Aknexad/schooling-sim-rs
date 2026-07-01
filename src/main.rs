use macroquad::prelude::*;

mod boid_rules;
mod fish;
mod window_conf;

use window_conf::default;
#[macroquad::main(default())]
async fn main() {
    let mut flock = fish::Fish::flock(100);

    loop {
        clear_background(BLACK);

        fish::update_flock(&mut flock, 50.0, 25.0);

        for fish in flock.iter_mut() {
            fish.draw();
        }

        next_frame().await
    }
}

// [321.292, 260.0], velocity: 300.0 }, Fish { position: [317.6466, 221.76599], velocity: 300.0 }, Fish { position: [179.93251, 239.96516], velocity: 300.0 },
