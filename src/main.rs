use glutin_window::GlutinWindow;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use std::sync::mpsc;
use std::thread;

mod app;
mod engine;
mod initialization;
mod types;
use types::*;

fn main() {
    let mut window: GlutinWindow = WindowSettings::new("Macrolipid", [400, 400]).exit_on_esc(true).build().unwrap();

    let (tx, rx) = mpsc::sync_channel::<State>(1);
    thread::spawn(move || {
        let mut e = engine::Engine::new(initialization::default());
        let mut tpf = 0;
        for _ in 1..1000000 {
            e.tick();
            tpf += 1;
            if tpf >= 10 {
                tx.send(e.current_state()).ok();
                tpf = 0;
            } else {
                if let Ok(_) = tx.try_send(e.current_state()) {
                    tpf = 0;
                }
            }
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
