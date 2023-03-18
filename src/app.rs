use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{RenderArgs, UpdateArgs};
// use piston_window::Glyphs;

use crate::types::*;

pub struct App {
    gl: GlGraphics,
    state: State,
    // glyphs: Glyphs,
}

impl App {
    pub fn new(// glyphs: Glyphs
    ) -> Self {
        Self {
            gl: GlGraphics::new(OpenGL::V3_2),
            state: State::new(),
            // glyphs: glyphs,
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
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
                        head_radius: _,
                        tail_length: _,
                    } => {
                        line(
                            GREEN,
                            1.0,
                            [
                                head_position.x.into(),
                                head_position.y.into(),
                                tail_position.x.into(),
                                tail_position.y.into(),
                            ],
                            c.transform,
                            gl,
                        );
                    }
                }
            }

            // render heads after, since they are small
            for m in &state.lipids {
                match m {
                    Lipid {
                        head_position,
                        tail_position: _,
                        head_radius,
                        tail_length: _,
                    } => {
                        let transform = c.transform.trans(head_position.x.into(), head_position.y.into());
                        let square = rectangle::square(0.0, 0.0, *head_radius as f64);
                        rectangle(RED, square, transform, gl);
                    }
                }
            }

            // text(WHITE, 32, "Hello world!", &mut self.glyphs, c.transform.trans(10.0, 100.0), gl);
            // // Update glyphs before rendering. -- why?
            // glyphs.factory.encoder.flush(device);
        });
    }

    pub fn new_data(&mut self, state: State) {
        self.state = state;
    }

    pub fn update(&mut self, _args: &UpdateArgs) {}
}
