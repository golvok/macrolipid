use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::input::{RenderArgs, UpdateArgs};

use crate::types::*;

pub struct App<'a> {
    gl: GlGraphics,
    state: State,
    glyph_cache: GlyphCache<'a>,
}

impl App<'_> {
    pub fn new() -> Self {
        Self {
            gl: GlGraphics::new(OpenGL::V3_2),
            state: State::new(),
            glyph_cache: GlyphCache::new("/usr/share/fonts/TTF/DejaVuSans.ttf", (), TextureSettings::new()).unwrap(),
        }
    }

    pub fn render(&mut self, args: &RenderArgs, max_fps: u64) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        let (_xmax, _ymax) = (args.window_size[0], args.window_size[1]);
        let state = &self.state;
        let glyph_cache = &mut self.glyph_cache;

        self.gl.draw(args.viewport(), |c, gl| {
            let scale = 2.5;
            let objects_transform = c.transform.scale(scale, scale);

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
                            1.0 / scale,
                            [
                                head_position.x.into(),
                                head_position.y.into(),
                                tail_position.x.into(),
                                tail_position.y.into(),
                            ],
                            objects_transform,
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
                        let square = rectangle::square(head_position.x.into(), head_position.y.into(), *head_radius as f64);
                        rectangle(RED, square, objects_transform, gl);
                    }
                }
            }

            text::Text::new_color(WHITE, 16)
                .draw(
                    &format!("Max FPS: {max_fps}"),
                    glyph_cache,
                    &DrawState::default(),
                    c.transform.trans(0.0, 16.0),
                    gl,
                )
                .unwrap();
        });
    }

    pub fn new_data(&mut self, state: State) {
        self.state = state;
    }

    pub fn update(&mut self, _args: &UpdateArgs) {}
}
