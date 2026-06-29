use macroquad::prelude::*;

mod fish;
mod window_conf;

use window_conf::default;
#[macroquad::main(default())]
async fn main() {
    let mut f = fish::Fish {
        position: [150.0, 200.1],
        velocity: 300.5,
    };

    loop {
        clear_background(BLACK);

        f.update();
        f.draw();

        next_frame().await
    }
}
