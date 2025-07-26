pub use cgmath::prelude::MetricSpace;
pub use cgmath::Point2;
pub use cgmath::Vector2;
use std::time::Duration;

pub type Point = Point2<f32>;
pub type Vector = Vector2<f32>;

#[derive(Debug, Copy, Clone)]
pub struct Lipid {
    pub head_position: Point2<f32>,
    pub tail_position: Point2<f32>,
    pub head_radius: f32,
    pub tail_length: f32,
    pub tail_width: f32,
}

#[derive(Debug, Clone)]
pub struct State {
    pub lipids: Vec<Lipid>,
    pub tick_time: Duration,
}

impl State {
    pub fn new() -> Self {
        Self {
            lipids: vec![],
            tick_time: Duration::ZERO,
        }
    }
}
