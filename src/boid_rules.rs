use crate::fish::Fish;

pub fn neighbor_detection(flock: &[Fish], neighbor_radius: f32) -> Vec<Vec<usize>> {
    let total_in_flock = flock.len();
    let radius_sq = neighbor_radius * neighbor_radius;

    let mut neighbors_map = vec![vec![]; total_in_flock];

    for i in 0..total_in_flock {
        for j in 0..total_in_flock {
            if i == j {
                continue;
            }

            let dx = flock[i].position[0] - flock[j].position[0];
            let dy = flock[i].position[1] - flock[j].position[1];
            let dist_sq = (dx * dx) + (dy * dy);

            if dist_sq < radius_sq {
                neighbors_map[i].push(j);
            }
        }
    }

    neighbors_map
}

pub fn calculate_separation(
    fish_idx: usize,
    flock: &[Fish],
    neighbors: &[usize],
    separation_radius: f32,
) -> [f32; 2] {
    let mut steer: [f32; 2] = [0.0, 0.0];

    let mut count = 0;
    let sep_r_sq = separation_radius * separation_radius;

    for &neighbor_idx in neighbors {
        let neighbor = &flock[neighbor_idx];

        let dx = flock[fish_idx].position[0] - neighbor.position[0];
        let dy = flock[fish_idx].position[1] - neighbor.position[1];
        let dist_sq = dx * dx + dy * dy;

        if dist_sq < sep_r_sq && dist_sq > 0.0 {
            let dist = dist_sq.sqrt();
            steer[0] += dx / dist;
            steer[1] += dy / dist;
            count += 1;
        }
    }

    if count > 0 {
        steer[0] /= count as f32;
        steer[1] /= count as f32;
    }

    steer
}

pub fn calculate_alignment(fish_idx: usize, flock: &[Fish], neighbors: &[usize]) -> [f32; 2] {
    let mut avg_velocity: [f32; 2] = [0.0, 0.0];
    let count = neighbors.len();

    if count == 0 {
        return [0.0, 0.0];
    }

    for &index in neighbors {
        avg_velocity[0] += flock[index].velocity[0];
        avg_velocity[1] += flock[index].velocity[1];
    }

    avg_velocity[0] /= count as f32;
    avg_velocity[1] /= count as f32;

    let steering = [
        avg_velocity[0] - flock[fish_idx].velocity[0],
        avg_velocity[1] - flock[fish_idx].velocity[1],
    ];

    steering
}

pub fn calculate_Cohesion(fish_idx: usize, flock: &[Fish], neighbors: &[usize]) -> [f32; 2] {
    let mut avg_position: [f32; 2] = [0.0, 0.0];

    let count = neighbors.len();

    if count == 0 {
        return [0.0, 0.0];
    }

    for &index in neighbors {
        avg_position[0] += flock[index].position[0];
        avg_position[1] += flock[index].position[1];
    }

    avg_position[0] /= count as f32;
    avg_position[1] /= count as f32;

    let desired_direction = [
        avg_position[0] - flock[fish_idx].position[0],
        avg_position[1] - flock[fish_idx].position[1],
    ];

    let steering = [
        desired_direction[0] - flock[fish_idx].velocity[0],
        desired_direction[1] - flock[fish_idx].velocity[1],
    ];

    steering
}
