#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::f32::consts::PI;

// External imports
use bevy::{
    prelude::*,
    window::{WindowResizeConstraints, WindowResolution},
};
use bevy_pixels::prelude::*;
use rand::Rng;


// Internal imports
mod components;
use components::{
    ParticleBundle, Position,
    Angle, Color, Trail,
    IntensityGrid, NonEmptyPositions
};

mod constants;
use constants::{
    SCREEN_WIDTH, SCREEN_HEIGHT, PARTICLE_COLOR,
    MOVEMENT_SPEED, PARTICLE_SIZE, NUM_PARTICLES
};

mod draw_systems;
use draw_systems::{draw_background, draw_objects};

mod movement;
use movement::{
    movement, bounce, 
    update_intensity_map, update_angle,
};



fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Physarum Simulation".to_string(),
                resolution: WindowResolution::new(
                    SCREEN_WIDTH as f32 * 2.0,
                    SCREEN_HEIGHT as f32 * 2.0,
                ),
                resize_constraints: WindowResizeConstraints {
                    min_width: SCREEN_WIDTH as f32 * 2.0,
                    min_height: SCREEN_HEIGHT as f32 * 2.0,
                    max_width: SCREEN_WIDTH as f32 * 2.0,
                    max_height: SCREEN_HEIGHT as f32 * 2.0,
                    ..default()
                },
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(PixelsPlugin {
            primary_window: Some(PixelsOptions {
                width: SCREEN_WIDTH,
                height: SCREEN_HEIGHT,
                ..default()
            }),
        })
        .insert_resource(
            IntensityGrid {
                grid: [[0; (SCREEN_WIDTH * 2 / PARTICLE_SIZE) as usize]; (SCREEN_HEIGHT * 2 / PARTICLE_SIZE) as usize]
            }
        )
        .insert_resource(
            NonEmptyPositions {
                positions: Vec::new()
            }
        )
        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        .add_systems((bounce, movement).chain().in_set(PixelsSet::Update))
        .add_systems(
            (draw_background, draw_objects)
                .chain()
                .in_set(PixelsSet::Draw)
        )
        .add_system(update_intensity_map)
        .add_system(update_angle)
        .run();
}


fn setup(
    mut commands: Commands
) {
    let angle_interval = 2.0 * PI / NUM_PARTICLES as f32;
    for i in 0..NUM_PARTICLES {
        commands.spawn(
            ParticleBundle {
                // position: Position { x: SCREEN_WIDTH as f32 / 2.0, y: SCREEN_HEIGHT as f32 / 2.0 },
                position: Position {
                    x: rand::thread_rng().gen_range(10..SCREEN_WIDTH - 10) as f32,
                    y: rand::thread_rng().gen_range(10..SCREEN_HEIGHT - 10) as f32
                },
                angle: Angle(i as f32 * angle_interval),
                color: Color(random_color(), random_color(), random_color())
            }
        );
    }
}

fn random_color() -> u8 {
    rand::thread_rng().gen_range(0..=255) as u8
}
