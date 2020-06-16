use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{RenderArgs, UpdateArgs};

use crate::types::*;

pub struct App {
    gl: GlGraphics,
    state: State,
}

impl App {
    pub fn new() -> Self {
        Self {
            gl: GlGraphics::new(OpenGL::V3_2),
            state: State::new(),
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        let (_xmax, _ymax) = (args.window_size[0], args.window_size[1]);
        let state = &self.state;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            for m in &state.lipids {
                match m {
                    Lipid {
                        head_position,
                        tail_position,
                        head_radius,
                    } => {
                        line(
                            GREEN,
                            1.0,
                            [
                                head_position.0.into(),
                                head_position.1.into(),
                                tail_position.0.into(),
                                tail_position.1.into(),
                            ],
                            c.transform,
                            gl,
                        );
                        let transform = c
                            .transform
                            .trans(head_position.0.into(), head_position.1.into());
                        let square = rectangle::square(0.0, 0.0, *head_radius as f64);
                        rectangle(RED, square, transform, gl);
                    }
                }
            }
        });
    }

    pub fn new_data(&mut self, state: State) {
        self.state = state;
    }

    pub fn update(&mut self, _args: &UpdateArgs) {}
}
