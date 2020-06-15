extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use std::sync::mpsc
use std::thread;

pub struct App {
	gl: GlGraphics,
}

impl App {
	fn render(&mut self, args: &RenderArgs) {
		use graphics::*;

		// use current data
		const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
		const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

		let square = rectangle::square(0.0, 0.0, 50.0);
		let (xmax, ymax) = (args.window_size[0], args.window_size[1]);

		self.gl.draw(args.viewport(), |c, gl| {
			clear(BLACK, gl);

			let transform = c
				.transform
				.trans(xmax/2.0, ymax/2.0)
				.rot_rad(0.0)
				.trans(-25.0, -25.0);

			rectangle(RED, square, transform, gl);
		});
	}

	fn update(&mut self, _args: &UpdateArgs) {
		// get data from queue, if any & update member
	}
}

fn main() {
	let opengl = OpenGL::V3_2;

	let mut window: GlutinWindow = WindowSettings::new("Macrolipid", [200, 200])
		.graphics_api(opengl)
		.exit_on_esc(true)
		.build()
		.unwrap();

	let mut app = App {
		gl: GlGraphics::new(opengl),
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
