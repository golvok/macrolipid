use crate::types::*;
use cgmath::Basis2;
use cgmath::Rad;
use cgmath::Rotation;
use cgmath::Rotation2;

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

pub struct Engine {
    prev: State,
    curr: State,
    rng: SmallRng,
    bounds: (Point, Point),
}

impl Engine {
    pub fn new(initial_state: State) -> Self {
        Self {
            prev: initial_state.clone(),
            curr: initial_state,
            rng: SmallRng::from_seed([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]),
            bounds: ((0., 0.).into(), (800., 800.).into()),
        }
    }

    pub fn tick(&mut self) {
        self.prev = self.curr.clone();

        for (ilipid, l) in self.curr.lipids.iter_mut().enumerate() {
            let mut ext_force = Vector { x: 0., y: 0. };
            let centre_of_mass = l.head_position + (l.tail_position - l.head_position) * 0.33;
            let mut torque: f32 = 0.0; // (CCW is positive)

            let min_sf_dist: f32 = 0.77;
            let max_sf_dist: f32 = 5.0;
            for (jlipid, jl) in self.prev.lipids.iter().enumerate() {
                if jlipid == ilipid {
                    continue;
                }

                let head_distance2 = jl.head_position.distance2(l.head_position);
                let head_error2 = head_distance2 - (l.head_radius + jl.head_radius).powf(2.0);
                if head_error2 < max_sf_dist.powf(2.0) && head_error2.abs() > min_sf_dist.powf(2.0) {
                    let coeff = if head_error2 > 0.0 { 1.0 } else { 1.5 };
                    let force_here = coeff * (jl.head_position - l.head_position) / head_error2;
                    let offset = centre_of_mass - l.head_position;
                    ext_force += force_here;
                    torque += offset.x * force_here.y - offset.y * force_here.x;
                }

                let tail_points = [0.33, 0.67, 1.0];
                for tail_distance1 in tail_points.iter() {
                    let tpos_i = l.head_position + (l.tail_position - l.head_position) * *tail_distance1;
                    for tail_distance2 in tail_points.iter() {
                        let tpos_j = jl.head_position + (jl.tail_position - jl.head_position) * *tail_distance2;
                        let tail_distance2 = tpos_j.distance2(tpos_i);
                        let tail_error2 = tail_distance2 - 2.0;
                        if tail_distance2 < max_sf_dist.powf(2.0) && tail_error2.abs() > min_sf_dist.powf(2.0) {
                            let force_here = 1. / tail_points.len() as f32 * (tpos_j - tpos_i) / tail_error2;
                            let offset = centre_of_mass - tpos_i;
                            ext_force += force_here;
                            torque += offset.x * force_here.y - offset.y * force_here.x;
                        }
                    }
                }
            }

            // random (~brownian) perturbations
            ext_force += Vector {
                x: self.rng.gen_range(-1., 1.),
                y: self.rng.gen_range(-1., 1.),
            } * 5.0;
            torque += self.rng.gen_range(-1., 1.) * 0.5;

            let head_tail_distance2 = l.head_position.distance2(l.tail_position);
            let head_tail_error2 = head_tail_distance2 - l.tail_length.powf(2.0);
            let head_tail_attraction = if head_tail_error2.abs() > 0.1 {
                let tail_to_head_unit = (l.tail_position - l.head_position) * head_tail_error2;
                tail_to_head_unit / head_tail_distance2
            } else {
                (0.0_f32, 0.0_f32).into()
            };

            let make_ccw_normal: Basis2<f32> = Rotation2::from_angle(Rad(0.5 * std::f32::consts::PI));
            let head_normal = make_ccw_normal.rotate_vector(centre_of_mass - l.head_position);
            let tail_normal = make_ccw_normal.rotate_vector(centre_of_mass - l.tail_position);

            let mass = 0.1;
            let head_force = (head_normal * torque * 0.1 + ext_force + head_tail_attraction) * mass;
            let tail_force = (tail_normal * torque * 0.1 + ext_force - head_tail_attraction) * mass;

            *l = Lipid {
                head_position: apply_force(self.bounds, l.head_position, head_force * 0.1),
                tail_position: apply_force(self.bounds, l.tail_position, tail_force * 0.1),
                head_radius: l.head_radius,
                tail_length: l.tail_length,
            }
        }
    }

    pub fn current_state(&self) -> State {
        self.curr.clone()
    }
}

pub fn apply_force(bounds: (Point, Point), p: Point, f: Vector) -> Point {
    let proposed = p + f;
    if proposed.x > bounds.1.x || proposed.y > bounds.1.y || proposed.x < bounds.0.x || proposed.y < bounds.0.y {
        p
    } else {
        proposed
    }
}
