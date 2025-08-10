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
            bounds: ((3., 3.).into(), (397., 397.).into()),
        }
    }

    pub fn tick(&mut self) {
        self.prev = self.curr.clone();
        let start_time = Instant::now();
        let time_step = 0.0001 as f32;
        let center_frac = 0.33 as f32; // fraction for the distance from head to the center of mass
        let first_moment = 30.0 as f32;
        let friction_loss_frac = 0.995;
        let min_error2: f32 = (0.5 as f32).powf(2.0);
        let max_dist2: f32 = (11.0 as f32).powf(2.0); // try to make the forces only short-ranged, like surface tension is
        let tail_points = [0.33, 0.67, 1.0]; // multi-point attraction & repulsion from/to tails
        let num_tail_points_f = tail_points.len() as f32;

        let mut water = ::ndarray::Array2::<f64>::zeros((400, 400));
        let water_kernel = ::ndarray::arr2(&[
            [0.0, 0.5, 1.0, 0.5, 0.0],
            [0.5, 1.0, 1.0, 1.0, 0.0],
            [0.0, 1.0, 1.0, 1.0, 0.5],
            [0.0, 1.0, 1.0, 1.0, 0.0],
            [0.0, 0.5, 1.0, 0.5, 0.0],
        ]);
        for l in self.curr.lipids.iter() {
            let head_index_x = l.head_position.x as usize;
            let head_index_y = l.head_position.y as usize;
            let mut local_water = water.slice_mut(::ndarray::s![
                head_index_y - 2..head_index_y + 3,
                head_index_x - 2..head_index_x + 3
            ]);
            local_water += &water_kernel;

            for tail_distance1 in tail_points.iter() {
                let tail_ipos = l.head_position + (l.tail_position - l.head_position) * *tail_distance1;
                let tail_index_ix = tail_ipos.x as usize;
                let tail_index_iy = tail_ipos.y as usize;

                let mut local_water = water.slice_mut(::ndarray::s![
                    tail_index_iy - 2..tail_index_iy + 3,
                    tail_index_ix - 2..tail_index_ix + 3
                ]);
                local_water -= &water_kernel;
            }
        }

        for w in water.iter_mut() {
            *w = w.min(1.0).max(-1.0);
        }

        self.curr
            .debug_array0
            .index_axis_mut(::ndarray::Axis(2), 2) // blue channel
            .assign(&water.mapv(|e| (e.max(0.0) * 255.0) as u8));
        self.curr
            .debug_array0
            .index_axis_mut(::ndarray::Axis(2), 0) // red channel
            .assign(&water.mapv(|e| ((e * -1.0).max(0.0) * 255.0) as u8));
        self.curr.debug_array0.index_axis_mut(::ndarray::Axis(2), 3).fill(255); // alpha

        // make head longer?
        // simulate water. bilayers do not form without water IRL
        //   for each grid cell, start with 0, and accumulate/remove water for each nearby head/tail
        //     do sgn(x)*sqrt(abs(x)) or something to mimic effect of pessure?
        //   for each segment, determine x&y gradient (use a kernel?) and apply as a force

        let x_kernel = ::ndarray::arr2(&[
            [0.0, -0.1, -0.3, -0.1, 0.0],
            [-0.1, -0.5, -0.7, -0.5, -0.1],
            [0.0, 0.0, 0.0, 0.0, 0.0],
            [0.1, 0.3, 0.7, 0.3, 0.1],
            [0.0, 0.1, 0.3, 0.1, 0.0],
        ]);
        let y_kernel = x_kernel.t().clone();

        for (ilipid, l) in self.curr.lipids.iter_mut().enumerate() {
            let mut ext_force = Vector { x: 0., y: 0. };
            let centre_of_mass = l.head_position + (l.tail_position - l.head_position) * center_frac;
            let mut ext_torque: f32 = 0.0; // (CCW is positive)

            {
                // water: head
                let head_index_x = l.head_position.x as usize;
                let head_index_y = l.head_position.y as usize;
                let local_water = water.slice(::ndarray::s![
                    head_index_y - 2..head_index_y + 3,
                    head_index_x - 2..head_index_x + 3
                ]);
                let force_here = 1000.0
                    * Vector {
                        x: x_kernel.iter().zip(local_water.iter()).map(|(x, y)| x * y).sum::<f64>() as f32,
                        y: y_kernel.iter().zip(local_water.iter()).map(|(x, y)| x * y).sum::<f64>() as f32,
                    };
                let offset = centre_of_mass - l.head_position;
                ext_force += force_here;
                ext_torque += offset.x * force_here.y - offset.y * force_here.x;
            }

            // water: tail
            for tail_distance1 in tail_points.iter() {
                let tail_ipos = l.head_position + (l.tail_position - l.head_position) * *tail_distance1;
                let tail_index_ix = tail_ipos.x as usize;
                let tail_index_iy = tail_ipos.y as usize;

                let local_water = water.slice(::ndarray::s![
                    tail_index_iy - 2..tail_index_iy + 3,
                    tail_index_ix - 2..tail_index_ix + 3
                ]);
                let force_here = 1000.0 / num_tail_points_f
                    * Vector {
                        x: -x_kernel.iter().zip(local_water.iter()).map(|(x, y)| x * y).sum::<f64>() as f32,
                        y: -y_kernel.iter().zip(local_water.iter()).map(|(x, y)| x * y).sum::<f64>() as f32,
                    };

                let offset = centre_of_mass - tail_ipos;
                ext_force += force_here;
                ext_torque += offset.x * force_here.y - offset.y * force_here.x;
            }

            for (jlipid, jl) in self.prev.lipids.iter().enumerate() {
                if jlipid == ilipid {
                    continue;
                }

                // attraction & repulsion on our head from the other head
                let head_dist2 = jl.head_position.distance2(l.head_position);
                let head_error2 = head_dist2 - (l.head_radius + jl.head_radius).powf(2.0);
                if min_error2 < head_error2.abs() && head_dist2 < max_dist2 {
                    let coeff = if head_error2 < 0.0 { -1.5 } else { 0.0 };
                    let force_here = coeff * (jl.head_position - l.head_position);
                    let offset = centre_of_mass - l.head_position;
                    ext_force += force_here;
                    ext_torque += offset.x * force_here.y - offset.y * force_here.x;
                }

                // repulsion on this head from the other tail points
                for tail_distance2 in tail_points.iter() {
                    let tpos_j = jl.head_position + (jl.tail_position - jl.head_position) * *tail_distance2;
                    let tail_tail_dist2 = tpos_j.distance2(l.head_position);
                    let tail_tail_error2 = tail_tail_dist2 - (l.head_radius + jl.tail_width / 2.0).powf(2.0);
                    if min_error2 < tail_tail_error2.abs() && tail_tail_dist2 < max_dist2 {
                        let coeff = if tail_tail_error2 < 0.0 { -1.5 } else { 0.0 };
                        let force_here = coeff / num_tail_points_f * (tpos_j - l.head_position);
                        let offset = centre_of_mass - l.head_position;
                        ext_force += force_here;
                        ext_torque += offset.x * force_here.y - offset.y * force_here.x;
                    }
                }

                for tail_distance1 in tail_points.iter() {
                    let tpos_i = l.head_position + (l.tail_position - l.head_position) * *tail_distance1;

                    // atraction and repulsion on this tail point from the other tail points
                    for tail_distance2 in tail_points.iter() {
                        let tpos_j = jl.head_position + (jl.tail_position - jl.head_position) * *tail_distance2;
                        let tail_tail_dist2 = tpos_j.distance2(tpos_i);
                        let tail_tail_error2 = tail_tail_dist2 - (l.tail_width / 2.0 + jl.tail_width / 2.0).powf(2.0);
                        if min_error2 < tail_tail_error2.abs() && tail_tail_dist2 < max_dist2 {
                            let coeff = if tail_tail_error2 < 0.0 { -1.5 } else { 0.0 };
                            let force_here = coeff / num_tail_points_f.powf(2.0) * (tpos_j - tpos_i);
                            let offset = centre_of_mass - tpos_i;
                            ext_force += force_here;
                            ext_torque += offset.x * force_here.y - offset.y * force_here.x;
                        }
                    }

                    // repulsion on this tail point from the other head
                    let head_tail_dist2 = jl.head_position.distance2(tpos_i);
                    let head_tail_error2 = head_tail_dist2 - (l.tail_width / 2.0 + jl.head_radius).powf(2.0);
                    if min_error2 < head_tail_error2.abs() && head_tail_dist2 < max_dist2 {
                        let coeff = if head_tail_error2 < 0.0 { -1.5 } else { 0.0 };
                        let force_here = coeff / num_tail_points_f * (jl.head_position - tpos_i);
                        let offset = centre_of_mass - tpos_i;
                        ext_force += force_here;
                        ext_torque += offset.x + force_here.y - offset.y * force_here.x;
                    }
                }
            }

            // random (~brownian) perturbations
            ext_force += Vector {
                x: self.rng.gen_range(-1.0..1.0),
                y: self.rng.gen_range(-1.0..1.0),
            } * 20000.0;
            ext_torque += self.rng.gen_range(-1.0..1.0) * 4000.0;

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

            // F = m*a = m*Dv/Dt => Dv = (F/m)*Dt => v(t+Dt) = v(t) + (F/m)*Dt
            let new_lin_vel = l.linear_velocity + ext_force * time_step;
            let new_ang_vel = l.angular_velocity + ext_torque * time_step;
            let head_vel = head_normal * new_ang_vel * l.tail_length * center_frac / first_moment + new_lin_vel + head_tail_attraction;
            let tail_vel =
                tail_normal * new_ang_vel * l.tail_length * (1.0 - center_frac) / first_moment + new_lin_vel - head_tail_attraction;

            *l = Lipid {
                // Ds = v*Dt => s(t + Dt) = s(t) + v*Dt
                head_position: apply_velocity(self.bounds, l.head_position, head_vel * time_step),
                tail_position: apply_velocity(self.bounds, l.tail_position, tail_vel * time_step),
                linear_velocity: l.linear_velocity * friction_loss_frac + ext_force * time_step,
                angular_velocity: l.angular_velocity * friction_loss_frac + ext_torque * time_step,
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

fn apply_velocity(bounds: (Point, Point), p: Point, v: Vector) -> Point {
    let proposed = p + v;
    if proposed.x > bounds.1.x || proposed.y > bounds.1.y || proposed.x < bounds.0.x || proposed.y < bounds.0.y {
        p
    } else {
        proposed
    }
}
