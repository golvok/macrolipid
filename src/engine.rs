use crate::types::*;
use cgmath::Basis2;
use cgmath::Rad;
use cgmath::Rotation;
use cgmath::Rotation2;
use std::time::Instant;

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
            rng: SmallRng::from_seed([
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
            ]),
            bounds: ((0., 0.).into(), (800., 800.).into()),
        }
    }

    pub fn tick(&mut self) {
        self.prev = self.curr.clone();
        let start_time = Instant::now();
        let time_step = 0.0005;

        // make head longer?
        // head-tail repulsion
        // simulate water. bilayers do not form without water IRL
        //   for each grid cell, start with 0, and accumulate/remove water for each nearby head/tail
        //     do sgn(x)*sqrt(abs(x)) or something to mimic effect of pessure?
        //   for each segment, determine x&y gradient (use a kernel?) and apply as a force

        for (ilipid, l) in self.curr.lipids.iter_mut().enumerate() {
            let mut ext_force = Vector { x: 0., y: 0. };
            let centre_of_mass = l.head_position + (l.tail_position - l.head_position) * 0.33;
            let mut torque: f32 = 0.0; // (CCW is positive)

            let min_error2: f32 = (0.01 as f32).powf(2.0);
            let max_dist2: f32 = (15.0 as f32).powf(2.0);
            for (jlipid, jl) in self.prev.lipids.iter().enumerate() {
                if jlipid == ilipid {
                    continue;
                }

                // attraction & repulsion on our head from the other head
                let head_dist2 = jl.head_position.distance2(l.head_position);
                let head_error2 = head_dist2 - (l.head_radius + jl.head_radius).powf(2.0);
                if min_error2 < head_error2.abs() && head_dist2 < max_dist2 {
                    let coeff = if head_error2 < 0.0 { -50.0 } else { 1.5 };
                    let force_here = coeff * (jl.head_position - l.head_position);
                    let offset = centre_of_mass - l.head_position;
                    ext_force += force_here;
                    torque += offset.x * force_here.y - offset.y * force_here.x;
                }

                // multi-point attraction & repulsion from/to tails
                let tail_points = [0.33, 0.67, 1.0];

                // force on this head from the other tail points
                for tail_distance2 in tail_points.iter() {
                    let tpos_j = jl.head_position + (jl.tail_position - jl.head_position) * *tail_distance2;
                    let tail_tail_dist2 = tpos_j.distance2(l.head_position);
                    let tail_tail_error2 = tail_tail_dist2 - (l.head_radius + jl.tail_width / 2.0).powf(2.0);
                    if min_error2 < tail_tail_error2.abs() && tail_tail_dist2 < max_dist2 {
                        let coeff = if tail_tail_error2 < 0.0 { -2.0 } else { -2.0 };
                        let force_here = coeff / tail_points.len() as f32 * (tpos_j - l.head_position);
                        let offset = centre_of_mass - l.head_position;
                        ext_force += force_here;
                        torque += offset.x * force_here.y - offset.y * force_here.x;
                    }
                }

                for tail_distance1 in tail_points.iter() {
                    let tpos_i = l.head_position + (l.tail_position - l.head_position) * *tail_distance1;

                    // force on this tail point from the other tail points
                    for tail_distance2 in tail_points.iter() {
                        let tpos_j = jl.head_position + (jl.tail_position - jl.head_position) * *tail_distance2;
                        let tail_tail_dist2 = tpos_j.distance2(tpos_i);
                        let tail_tail_error2 = tail_tail_dist2 - (l.tail_width / 2.0 + jl.tail_width / 2.0).powf(2.0);
                        if min_error2 < tail_tail_error2.abs() && tail_tail_dist2 < max_dist2 {
                            let coeff = if tail_tail_error2 < 0.0 { -50.0 } else { 1.5 };
                            let force_here = coeff / tail_points.len() as f32 * (tpos_j - tpos_i);
                            let offset = centre_of_mass - tpos_i;
                            ext_force += force_here;
                            torque += offset.x * force_here.y - offset.y * force_here.x;
                        }
                    }

                    // repulsive force on this tail point from the other head
                    let head_tail_dist2 = jl.head_position.distance2(tpos_i);
                    let head_tail_error2 = head_tail_dist2 - (l.tail_width / 2.0 + jl.head_radius).powf(2.0);
                    if min_error2 < head_tail_error2.abs() && head_tail_dist2 < max_dist2 {
                        let coeff = if head_tail_error2 < 0.0 { -2.0 } else { -2.0 };
                        let force_here = coeff * (jl.head_position - tpos_i);
                        let offset = centre_of_mass - tpos_i;
                        ext_force += force_here;
                        torque += offset.x + force_here.y - offset.y * force_here.x;
                    }
                }
            }

            // random (~brownian) perturbations
            ext_force += Vector {
                x: self.rng.gen_range(-1.0..1.0),
                y: self.rng.gen_range(-1.0..1.0),
            } * 50.0;
            torque += self.rng.gen_range(-1.0..1.0) * 0.5;

            // pull/push the head and tail of the same lipid together/apart if they are too far from the natural distance
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

            let head_force = head_normal * torque * 0.2 / 6.0 + ext_force + head_tail_attraction;
            let tail_force = tail_normal * torque * 0.1 / 6.0 + ext_force - head_tail_attraction;

            *l = Lipid {
                head_position: apply_force(self.bounds, l.head_position, head_force * time_step),
                tail_position: apply_force(self.bounds, l.tail_position, tail_force * time_step),
                head_radius: l.head_radius,
                tail_length: l.tail_length,
                tail_width: l.tail_width,
            }
        }

        self.curr.tick_time = Instant::now() - start_time;
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
