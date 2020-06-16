use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};
use std::sync::mpsc;

use crate::types::*;

pub struct App {
    pub gl: GlGraphics,
    pub rx: mpsc::Receiver<MoleculeEnum>,
    pub molecules: Vec<MoleculeEnum>,
}

impl App {
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        // use current data
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        let (_xmax, _ymax) = (args.window_size[0], args.window_size[1]);
        let molecules = &self.molecules;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            for m in molecules {
                match m {
                    MoleculeEnum::Lipid(Lipid {
                        head_position,
                        tail_position,
                        head_radius,
                    }) => {
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

    pub fn update(&mut self, _args: &UpdateArgs) {
        // get data from queue, if any & update member
        match self.rx.try_recv() {
            Ok(molecule) => self.molecules.push(molecule),
            Result::Err(_err) => {}
        }
    }
}
