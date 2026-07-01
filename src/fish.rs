use macroquad::prelude::*;

// use crate::window_conf;
use crate::{
    boid_rules::{
        calculate_Cohesion, calculate_alignment, calculate_separation, neighbor_detection,
    },
    fish,
};

#[derive(Debug)]
pub struct Fish {
    pub position: [f32; 2],
    pub velocity: [f32; 2],
}

impl Fish {
    pub fn draw(&self) {
        let original_x = self.position[0];
        let original_y = self.position[1];

        draw_poly(original_x, original_y, 3, 5.1, 0.0, BLUE);
    }

    // pub fn update(&mut self, flock: &Vec<Fish>) {
    //     let detect_neighbors = neighbor_detection(&flock, 20.10);
    //     let weight: f32 = 1.2;
    //     let sep_weight = 1.5;
    //     let ali_weight = 1.0;
    //     let coh_weight = 1.0;

    //     let separation_force = calculate_separation(flock);
    //     let alignment_force = calculate_alignment(all_fishes);
    //     let cohesion_force = calculate_cohesion(all_fishes);
    // }

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
                    velocity: [300.0, 0.0],
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

pub fn update_flock(flock: &mut Vec<Fish>, neighbor_radius: f32, separation_radius: f32) {
    let neighbors_map = neighbor_detection(flock, neighbor_radius);

    let mut new_velocities = Vec::with_capacity(flock.len());
    for i in 0..flock.len() {
        let neighbors = &neighbors_map[i];

        let sep = calculate_separation(i, flock, neighbors, separation_radius);
        let align = calculate_alignment(i, flock, neighbors);
        let coh = calculate_Cohesion(i, flock, neighbors);

        let sep_w = 1.5;
        let align_w = 1.0;
        let coh_w = 1.0;

        let acc_x = sep[0] * sep_w + align[0] * align_w + coh[0] * coh_w;
        let acc_y = sep[1] * sep_w + align[1] * align_w + coh[1] * coh_w;

        let mut new_vx = flock[i].velocity[0] + acc_x;
        let mut new_vy = flock[i].velocity[1] + acc_y;

        let max_speed = 3.0;
        let speed_sq = new_vx * new_vx + new_vy * new_vy;
        if speed_sq > max_speed * max_speed {
            let speed = speed_sq.sqrt();
            new_vx = (new_vx / speed) * max_speed;
            new_vy = (new_vy / speed) * max_speed;
        }

        new_velocities.push([new_vx, new_vy]);
    }

    for i in 0..flock.len() {
        flock[i].velocity = new_velocities[i];
        flock[i].position[0] += flock[i].velocity[0];
        flock[i].position[1] += flock[i].velocity[1];

        keep_in_bounds(&mut flock[i]);
    }
}
pub fn keep_in_bounds(fish: &mut Fish) {
    let width = screen_width();
    let height = screen_height();

    if fish.position[0] > width {
        fish.position[0] = 0.0;
    } else if fish.position[0] < 0.0 {
        fish.position[0] = width;
    }

    if fish.position[1] > height {
        fish.position[1] = 0.0;
    } else if fish.position[1] < 0.0 {
        fish.position[1] = height;
    }
}
