use bevy::prelude::*;
use bevy_pixels::prelude::*;
use crate::components::{
    Color, Angle, Position, Trail,
    IntensityGrid, NonEmptyPositions
};
use crate::constants::{
    MOVEMENT_SPEED, SCREEN_HEIGHT,
    SCREEN_WIDTH, PARTICLE_SIZE, TRAIL_FADE_SPEED, 
    SENSE_FORWARD_DISTANCE, SENSE_BOX_SIZE
};
use rand::Rng;
use std::f32::consts::PI;
use std::cmp::{min, max};

pub fn movement(
    options_query: Query<&PixelsOptions>,
    mut query: Query<(&mut Position, &Angle, &Color)>,
    mut particle_intensity_grid: ResMut<IntensityGrid>,
) {
    let Ok(options) = options_query.get_single() else { return };

    for (mut pos, angle, color) in &mut query {
        particle_intensity_grid.grid[pos.y.round() as usize][pos.x.round() as usize] = 250;
        let direction = (angle.0.cos(), angle.0.sin());
        pos.x = (pos.x as f32 + (direction.0 * MOVEMENT_SPEED)).clamp(0.0, (options.width - PARTICLE_SIZE) as f32);
        pos.y = (pos.y as f32 + (direction.1 * MOVEMENT_SPEED)).clamp(0.0, (options.height - PARTICLE_SIZE) as f32);
        particle_intensity_grid.grid[pos.y.round() as usize][pos.x.round() as usize] = 255;
    }
}

pub fn bounce(
    mut query: Query<(&Position, &mut Angle)>,
) {
    for (pos, mut angle) in &mut query {
        if pos.x >= (SCREEN_WIDTH - PARTICLE_SIZE) as f32 || pos.x <= PARTICLE_SIZE as f32 {
            angle.0 = rand::thread_rng().gen_range(0..=360) as f32 * PI / 180.0;
        }
        if pos.y >= (SCREEN_HEIGHT - PARTICLE_SIZE) as f32 || pos.y <= PARTICLE_SIZE as f32 {
            angle.0 = rand::thread_rng().gen_range(0..=360) as f32 * PI / 180.0;
        }
    }
}


pub fn update_intensity_map(
    mut particle_intensity_grid: ResMut<IntensityGrid>,
    mut non_empty: ResMut<NonEmptyPositions>,
    time: Res<Time>
) {
    non_empty.positions = Vec::new();
    for ypos in 1..(particle_intensity_grid.grid.len() - 1) {
        for xpos in 1..(particle_intensity_grid.grid[ypos].len() - 1) {
            if particle_intensity_grid.grid[ypos][xpos] < TRAIL_FADE_SPEED {
                particle_intensity_grid.grid[ypos][xpos] = 0;
            } else {
                particle_intensity_grid.grid[ypos][xpos] -= TRAIL_FADE_SPEED;
                non_empty.positions.push((xpos, ypos));
            }
        }
    }
}

fn compute_average_intensity(
    arr_slice: &[[u8; (SCREEN_WIDTH * 2 / PARTICLE_SIZE) as usize]; (SCREEN_HEIGHT * 2 / PARTICLE_SIZE) as usize],
    xpos: usize,
    ypos: usize
) -> u8 {
    let sum = arr_slice[ypos - 1][xpos - 1] + arr_slice[ypos - 1][xpos] + arr_slice[ypos - 1][xpos + 1]
        + arr_slice[ypos][xpos - 1] + arr_slice[ypos][xpos] + arr_slice[ypos][xpos + 1]
        + arr_slice[ypos + 1][xpos - 1] + arr_slice[ypos + 1][xpos] + arr_slice[ypos + 1][xpos + 1];
    ((sum as f32) / 9.0).round() as u8
}

fn compute_sum_intensity(
    arr_slice: &[[u8; (SCREEN_WIDTH * 2 / PARTICLE_SIZE) as usize]; (SCREEN_HEIGHT * 2 / PARTICLE_SIZE) as usize],
    xpos: usize,
    ypos: usize,
    box_size: usize
) -> u32 {
    let mut sum: u32 = 0;
    for yidx in (max(0, ypos - box_size))..(min(ypos + box_size, (SCREEN_HEIGHT * 2 / PARTICLE_SIZE) as usize)) {
        for xidx in (max(0, xpos - box_size))..(min(xpos + box_size, (SCREEN_WIDTH * 2 / PARTICLE_SIZE) as usize)) {
            sum += arr_slice[yidx][xidx] as u32;
        }
    }
    sum
}

pub fn update_angle(
    mut query: Query<(&mut Angle, &Position)>,
    particle_intensity_grid: Res<IntensityGrid>
) {
    for (mut angle, position) in &mut query {
        let (original_angle, left_angle, right_angle) = (angle.0, angle.0 - (PI / 3.0), angle.0 + PI / 3.0);

        let original_direction = (original_angle.cos() * SENSE_FORWARD_DISTANCE, original_angle.sin() * SENSE_FORWARD_DISTANCE);
        let left_direction = (left_angle.cos() * SENSE_FORWARD_DISTANCE, left_angle.sin() * SENSE_FORWARD_DISTANCE);
        let right_direction = (right_angle.cos() * SENSE_FORWARD_DISTANCE, right_angle.sin() * SENSE_FORWARD_DISTANCE);

        let original_coords = ((position.x + original_direction.0).round() as usize, (position.y + original_direction.1).round() as usize);
        let left_coords = ((position.x + left_direction.0).round() as usize, (position.y + left_direction.1).round() as usize);
        let right_coords = ((position.x + right_direction.0).round() as usize, (position.y + right_direction.1).round() as usize);

        let original_intensity = compute_sum_intensity(&(particle_intensity_grid.grid), original_coords.0, original_coords.1, SENSE_BOX_SIZE);
        let left_intensity = compute_sum_intensity(&(particle_intensity_grid.grid), left_coords.0, left_coords.1, SENSE_BOX_SIZE);
        let right_intensity = compute_sum_intensity(&(particle_intensity_grid.grid), right_coords.0, right_coords.1, SENSE_BOX_SIZE);

        if original_intensity > left_intensity && original_intensity > right_intensity {
            angle.0 += 0.0;
        } else if original_intensity < left_intensity && original_intensity < right_intensity {
            angle.0 += rand::thread_rng().gen_range(0..=60) as f32 / 10.0;
        } else if right_intensity > left_intensity {
            angle.0 += rand::thread_rng().gen_range(100..=200) as f32 / 100.0;
        } else if left_intensity > right_intensity {
            angle.0 -= rand::thread_rng().gen_range(100..=200) as f32 / 100.0;
        }
    }
}
