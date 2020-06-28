use crate::types::*;

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
            let mut tail_force = Vector { x: 0., y: 0. };
            let mut head_force = Vector { x: 0., y: 0. };
            for (jlipid, jl) in self.prev.lipids.iter().enumerate() {
                if jlipid == ilipid {
                    continue;
                }

                let head_distance2 = jl.head_position.distance2(l.head_position);
                let head_error2 = head_distance2 - (l.head_radius + jl.head_radius).powf(2.0);
                if head_error2 < 3_f32.powf(2.0) && head_error2.abs() > 0.5 {
                    head_force += if head_error2 > 0.0 { 1.0 } else { 1.5 } / head_error2
                        * (jl.head_position - l.head_position)
                        / head_distance2.sqrt();
                }

                let tail_points = [0.4, 0.7, 1.0];
                for tail_distance1 in tail_points.iter() {
                    let tpos_j =
                        jl.head_position + (jl.tail_position - jl.head_position) * *tail_distance1;
                    for tail_distance2 in tail_points.iter() {
                        let tpos_i =
                            l.head_position + (l.tail_position - l.head_position) * *tail_distance2;
                        let tail_distance2 = tpos_j.distance2(tpos_i);
                        let tail_error2 = tail_distance2 - 1.0;
                        if tail_distance2 < 3_f32.powf(2.0) && tail_error2.abs() > 0.5 {
                            tail_force += 0.33 * (tpos_j - tpos_i) / tail_error2;
                        }
                    }
                }
            }

            let head_tail_distance2 = l.head_position.distance2(l.tail_position);
            let head_tail_error2 = head_tail_distance2 - l.tail_length.powf(2.0);
            if head_tail_error2.abs() > 0.1 {
                let tail_to_head_unit = (l.tail_position - l.head_position) * head_tail_error2;
                let abc = tail_to_head_unit / head_tail_distance2;
                head_force += 0.1 * abc;
                tail_force += 0.1 * -abc;
            }

            let brownian_force = Vector {
                x: self.rng.gen_range(-1., 1.),
                y: self.rng.gen_range(-1., 1.),
            };
            head_force += brownian_force * 5.0;
            tail_force += brownian_force * 5.0;

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
    if proposed.x > bounds.1.x
        || proposed.y > bounds.1.y
        || proposed.x < bounds.0.x
        || proposed.y < bounds.0.y
    {
        p
    } else {
        proposed
    }
}
