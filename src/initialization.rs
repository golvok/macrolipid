use crate::types::*;

pub fn default() -> State {
    let mut result = State { lipids: vec![] };
    for i in 0..1000 {
        let x = (i % 100) as f32;
        let y = (i / 100) as f32;
        result.lipids.push(Lipid {
            head_position: (100.0 + 6.0 * x, 100.0 + 6.0 * y).into(),
            tail_position: (105.0 + 6.0 * x, 105.0 + 6.0 * y).into(),
            head_radius: 2.,
        })
    }

    result
}
