use crate::types::*;
use cgmath::Basis2;
use cgmath::Rad;
use cgmath::Rotation;
use cgmath::Rotation2;

pub fn default() -> State {
    let mut result = State { lipids: vec![] };
    for irow in 0..=1 {
        for icol in 0..200 {
            let centre = Point::new(100.0 + icol as f32 * 1.0, 100.0 + irow as f32 * 10.0 + (icol % 5) as f32 * 2.0);
            let tail_length = 10.0;
            let tail_vec_x = Vector::new(tail_length * 0.5, 0.0);
            let angle = if irow % 2 == 0 { 0.5 } else { 1.5 };
            let rot: Basis2<f32> = Rotation2::from_angle(Rad(angle * std::f32::consts::PI));
            let offset_vec = rot.rotate_vector(tail_vec_x);
            result.lipids.push(Lipid {
                head_position: centre - offset_vec,
                tail_position: centre + offset_vec,
                head_radius: 3.,
                tail_length: tail_length,
            })
        }
    }

    result
}
