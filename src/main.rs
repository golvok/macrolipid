use glutin_window::GlutinWindow;
use opengl_graphics::OpenGL;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston::Button::Keyboard;
use piston::ButtonEvent;
use piston::EventLoop;

use std::cmp::max;
use std::sync::mpsc;
use std::thread;

mod app;
mod engine;
mod initialization;
mod types;
use types::*;

fn main() {
    let mut window: GlutinWindow = WindowSettings::new("Macrolipid", [400, 400])
        .graphics_api(OpenGL::V4_2)
        .build()
        .unwrap();

    let (tx, rx) = mpsc::sync_channel::<State>(1);
    thread::spawn(move || {
        let mut e = engine::Engine::new(initialization::default());
        loop {
            e.tick();
            tx.try_send(e.current_state()).ok();
            // tx.send(e.current_state()).ok();
        }
    });

    let mut app = app::App::new();

    let mut old_fps = 60;
    let mut events = Events::new(EventSettings::new().max_fps(60));
    'main_loop: while let Some(e) = events.next(&mut window) {
        let event_settings = events.get_event_settings();
        if let Some(args) = e.render_args() {
            app.render(&args, event_settings.max_fps);
        }

        if let Ok(state) = rx.try_recv() {
            app.new_data(state)
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(args) = e.button_args() {
            if let Keyboard(key) = args.button {
                use piston::ButtonState::*;
                use piston::Key;
                match (key, args.state) {
                    (Key::Comma, Press) => events.set_max_fps(max(event_settings.max_fps, 4) - 2),
                    (Key::Period, Press) => events.set_max_fps(event_settings.max_fps + 2),
                    (Key::S, Press) => {
                        old_fps = event_settings.max_fps;
                        events.set_max_fps(2);
                    }
                    (Key::S, Release) => events.set_max_fps(old_fps),
                    (Key::Q, Press) => break 'main_loop,
                    (Key::Escape, Press) => break 'main_loop,
                    _ => (),
                }
            }
        }
    }
}
