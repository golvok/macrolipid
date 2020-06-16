extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use std::sync::mpsc;
use std::thread;
use std::time;

pub struct Lipid {
    head_position: (f32, f32),
    tail_position: (f32, f32),
    head_radius: f32,
}

pub enum MoleculeEnum {
    Lipid(Lipid),
}

pub struct App {
    gl: GlGraphics,
    rx: mpsc::Receiver<MoleculeEnum>,
    molecules: Vec<MoleculeEnum>,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
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

    fn update(&mut self, _args: &UpdateArgs) {
        // get data from queue, if any & update member
        match self.rx.try_recv() {
            Ok(molecule) => self.molecules.push(molecule),
            Result::Err(_err) => {}
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new("Macrolipid", [400, 400])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let (tx, rx) = mpsc::channel::<MoleculeEnum>();
    thread::spawn(move || {
        for i_int in 1..10 {
            let i: f32 = i_int as f32;
            tx.send(MoleculeEnum::Lipid(Lipid {
                head_position: (20. * i, 20. * i),
                tail_position: (20. * i + 5., 20. * i + 5.),
                head_radius: 2.,
            }))
            .unwrap();
            thread::sleep(time::Duration::from_millis(200))
        }
    });

    let mut app = App {
        gl: GlGraphics::new(opengl),
        rx: rx,
        molecules: vec![],
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
