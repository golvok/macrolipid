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
            let mut force = Vector {
                x: self.rng.gen_range(-10., 10.),
                y: self.rng.gen_range(-10., 10.),
            };
            for (jlipid, jl) in self.prev.lipids.iter().enumerate() {
                if jlipid == ilipid {
                    continue;
                }
                let distance2 = jl.head_position.distance2(l.head_position);
                if 0.1 < distance2 && distance2 < 3.0 * 3.0 {
                    force -= (jl.head_position - l.head_position) / distance2;
                }
            }

            *l = Lipid {
                head_position: apply_force(self.bounds, l.head_position, force * 0.1),
                tail_position: apply_force(self.bounds, l.tail_position, force * 0.1),
                head_radius: l.head_radius,
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
