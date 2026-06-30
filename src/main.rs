use macroquad::prelude::*;

mod fish;
mod window_conf;

use window_conf::default;
#[macroquad::main(default())]
async fn main() {
    let mut flock = fish::Fish::flock(100);

    loop {
        clear_background(BLACK);

        for fish in flock.iter_mut() {
            fish.draw();
            fish.update();
        }

        next_frame().await
    }
}
