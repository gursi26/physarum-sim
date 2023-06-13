use bevy::prelude::*;
use crate::constants::*;

#[derive(Bundle)]
pub struct ParticleBundle {
    pub angle: Angle,
    pub position: Position,
    pub color: Color
}

#[derive(Component)]
pub struct Angle(pub f32);

#[derive(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32
}

#[derive(Component)]
pub struct Color(pub u8, pub u8, pub u8);

#[derive(Component)]
pub struct Trail;

#[derive(Resource)]
pub struct IntensityGrid {
    pub grid: [[u8; (SCREEN_WIDTH * 2 / PARTICLE_SIZE) as usize]; (SCREEN_HEIGHT * 2 / PARTICLE_SIZE) as usize]
}

#[derive(Resource)]
pub struct NonEmptyPositions {
    pub positions: Vec<(usize, usize)>
}
