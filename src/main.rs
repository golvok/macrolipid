use glutin_window::GlutinWindow;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use std::sync::mpsc;
use std::thread;
use std::time;

mod app;
mod types;
use types::*;

fn main() {
    let mut window: GlutinWindow = WindowSettings::new("Macrolipid", [400, 400])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let (tx, rx) = mpsc::sync_channel::<State>(1);
    thread::spawn(move || {
        for i_int in 1..10 {
            let i: f32 = i_int as f32;
            tx.try_send(State {
                lipids: vec![Lipid {
                    head_position: (20. * i, 20. * i),
                    tail_position: (20. * i + 5., 20. * i + 5.),
                    head_radius: 2.,
                }],
            })
            .ok();
            thread::sleep(time::Duration::from_millis(200))
        }
    });

    let mut app = app::App::new();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Ok(state) = rx.try_recv() {
            app.new_data(state)
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
