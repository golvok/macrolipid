use crate::types::*;
pub struct Engine {
    curr: State,
}

impl Engine {
    pub fn new(initial_state: State) -> Self {
        Self {
            curr: initial_state,
        }
    }

    pub fn tick(&mut self) {
        for l in self.curr.lipids.iter_mut() {
            *l = Lipid {
                head_position: (l.head_position.0 + 1., l.head_position.1 + 1.),
                tail_position: (l.tail_position.0 + 1., l.tail_position.1 + 1.),
                head_radius: l.head_radius,
            }
        }
    }

    pub fn current_state(&self) -> State {
        self.curr.clone()
    }
}
