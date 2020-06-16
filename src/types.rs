pub struct Lipid {
    pub head_position: (f32, f32),
    pub tail_position: (f32, f32),
    pub head_radius: f32,
}

pub struct State {
    pub lipids: Vec<Lipid>,
}

impl State {
    pub fn new() -> Self {
        Self { lipids: vec![] }
    }
}
