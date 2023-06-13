use bevy::prelude::*;
use bevy_pixels::prelude::*;
use crate::components::*;
use crate::constants::*;
use rand::Rng;

pub fn draw_background(mut wrapper_query: Query<&mut PixelsWrapper>) {
    let Ok(mut wrapper) = wrapper_query.get_single_mut() else { return };
    let frame = wrapper.pixels.frame_mut();
    frame.copy_from_slice(&[0x00, 0x00, 0x00, 0xff].repeat(frame.len() / 4));
}

pub fn draw_objects(
    mut wrapper_query: Query<(&mut PixelsWrapper, &PixelsOptions)>,
    query: Query<(&Position, &Color)>,
    intensity_grid: Res<IntensityGrid>,
    non_empty: Res<NonEmptyPositions>
) {
    let Ok((mut wrapper, options)) = wrapper_query.get_single_mut() else { return };
    let frame = wrapper.pixels.frame_mut();
    let frame_width_bytes = (options.width * 4) as usize;

    for (xpos, ypos) in non_empty.positions.iter() {
        let position = Position { x: *xpos as f32, y: *ypos as f32 };
        let x_offset = (position.x.round() as u32 * 4) as usize;
        let width_bytes = (PARTICLE_SIZE * 4) as usize;
        let intensity = intensity_grid.grid[position.y.round() as usize][position.x.round() as usize];
        let object_row = &[PARTICLE_COLOR.0, PARTICLE_COLOR.1, PARTICLE_COLOR.2, intensity].repeat(PARTICLE_SIZE as usize);
        let rounded_y = position.y.round() as u32;

        for y in rounded_y..(rounded_y + PARTICLE_SIZE) {
            let y_offset = y as usize * frame_width_bytes;
            let i = y_offset + x_offset;
            let j = i + width_bytes;

            frame[i..j].copy_from_slice(object_row);
        }
    }
}

