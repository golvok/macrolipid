use piston::event_loop::EventLoop;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston::Button::Keyboard;
use piston::ButtonEvent;
use piston_window::PistonWindow;
use piston_window::Transformed;

use std::cmp::max;
use std::sync::mpsc;
use std::thread;

mod app;
mod engine;
mod initialization;
mod types;
use types::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Macrolipid", [400, 400]).exit_on_esc(true).build().unwrap();

    let (tx, rx) = mpsc::sync_channel::<State>(1);
    thread::spawn(move || {
        let mut e = engine::Engine::new(initialization::default());
        let mut tpf = 0;
        for _ in 1..1000000 {
            e.tick();
            tpf += 1;
            if tpf >= 1 {
                tx.send(e.current_state()).ok();
                tpf = 0;
            } else {
                if let Ok(_) = tx.try_send(e.current_state()) {
                    tpf = 0;
                }
            }
        }
    });

    // let font_data: &[u8] = include_bytes!("/usr/share/fonts/TTF/DejaVuSans.ttf");
    // let font: Font<'static> = Font::try_from_bytes(font_data).unwrap();

    let mut glyphs = window.load_font("/usr/share/fonts/TTF/DejaVuSans.ttf").unwrap();
    let mut app = app::App::new();

    let mut events = Events::new(EventSettings::new().max_fps(10));
    while let Some(e) = events.next(&mut window) {
        let event_settings = events.get_event_settings();
        if let Some(args) = e.render_args() {
            app.render(&args);
            window.draw_2d(&e, |c, gl, _dev| {
                graphics::text(
                    [1.0, 1.0, 1.0, 1.0],
                    32,
                    "Hello world!",
                    &mut glyphs,
                    c.transform.trans(10.0, 100.0),
                    gl,
                )
                .expect("tried drawing text");
            });
        }

        if let Ok(state) = rx.try_recv() {
            app.new_data(state)
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(args) = e.button_args() {
            if let Keyboard(key) = args.button {
                match key {
                    piston::Key::LeftBracket => events.set_max_fps(max(event_settings.max_fps - 2, 1)),
                    piston::Key::RightBracket => events.set_max_fps(event_settings.max_fps + 2),
                    _ => (),
                }
            }
        }
    }
}
