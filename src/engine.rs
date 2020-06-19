use crate::types::*;
use cgmath::prelude::MetricSpace;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

pub struct Engine {
    prev: State,
    curr: State,
    rng: SmallRng,
}

impl Engine {
    pub fn new(initial_state: State) -> Self {
        Self {
            prev: initial_state.clone(),
            curr: initial_state,
            rng: SmallRng::from_seed([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]),
        }
    }

    pub fn tick(&mut self) {
        self.prev = self.curr.clone();

        for (ilipid, l) in self.curr.lipids.iter_mut().enumerate() {
            let mut force = cgmath::Vector2::<f32> {
                x: self.rng.gen(),
                y: self.rng.gen(),
            };
            force -= (0.5, 0.5).into();
            force *= 20.;
            for (jlipid, jl) in self.prev.lipids.iter().enumerate() {
                if jlipid == ilipid {
                    continue;
                }
                let distance2 = jl.head_position.distance2(l.head_position);
                if 3. * 3. < distance2 && distance2 < 10.0 * 10.0 {
                    force += (jl.head_position - l.head_position) / distance2;
                }
            }

            *l = Lipid {
                head_position: l.head_position + force.cast::<f32>().unwrap() * 0.1,
                tail_position: l.tail_position + force.cast::<f32>().unwrap() * 0.1,
                head_radius: l.head_radius,
            }
        }
    }

    pub fn current_state(&self) -> State {
        self.curr.clone()
    }
}
