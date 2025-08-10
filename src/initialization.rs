use crate::types::*;
use cgmath::Basis2;
use cgmath::Rad;
use cgmath::Rotation;
use cgmath::Rotation2;

pub fn default() -> State {
    let mut result = State::new();
    for irow in 0..=1 {
        for icol in 0..20 {
            let centre = Point::new(100.0 + icol as f32 * 3.5, 100.0 + irow as f32 * 20.0 + (icol % 3) as f32 * 5.25);
            let tail_length = 10.0;
            let tail_vec_x = Vector::new(tail_length * 0.5, 0.0);
            let angle = if irow % 2 == 0 { 1.0 } else { 0.0 };
            let rot: Basis2<f32> = Rotation2::from_angle(Rad(angle * std::f32::consts::PI));
            let offset_vec = rot.rotate_vector(tail_vec_x);
            result.lipids.push(Lipid {
                head_position: centre - offset_vec,
                tail_position: centre + offset_vec,
                linear_velocity: Vector2::new(0.0, 0.0),
                angular_velocity: 0.0,
                head_radius: 3.0,
                tail_length: tail_length,
                tail_width: 1.,
            })
        }
    }

    result
}
