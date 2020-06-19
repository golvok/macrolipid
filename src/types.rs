use cgmath::*;

#[derive(Debug, Copy, Clone)]
pub struct Lipid {
    pub head_position: Point2<f32>,
    pub tail_position: Point2<f32>,
    pub head_radius: f32,
}

#[derive(Debug, Clone)]
pub struct State {
    pub lipids: Vec<Lipid>,
}

impl State {
    pub fn new() -> Self {
        Self { lipids: vec![] }
    }
}
