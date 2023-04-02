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
        use graphics::color::*;
        use graphics::*;

        let state = &self.state;
        let glyph_cache = &mut self.glyph_cache;

        self.gl.draw(args.viewport(), |c, gl| {
            let scale = 2.5;
            let objects_transform = c.transform.scale(scale, scale);

            clear(BLACK, gl);

            for (i_lipid, lipid) in state.lipids.iter().enumerate() {
                match lipid {
                    Lipid {
                        head_position,
                        tail_position,
                        head_radius: _,
                        tail_length: _,
                        tail_width,
                    } => {
                        line(
                            GREEN.shade(i_lipid as f32 / 1.5 / state.lipids.len() as f32),
                            *tail_width as f64,
                            [
                                head_position.x as f64,
                                head_position.y as f64,
                                tail_position.x as f64,
                                tail_position.y as f64,
                            ],
                            objects_transform,
                            gl,
                        );
                    }
                }
            }

            // render heads after, since they are small
            for (i_lipid, lipid) in state.lipids.iter().enumerate() {
                match lipid {
                    Lipid {
                        head_position,
                        tail_position: _,
                        head_radius,
                        tail_length: _,
                        tail_width: _,
                    } => {
                        let square = rectangle::centered([
                            head_position.x as f64,
                            head_position.y as f64,
                            *head_radius as f64,
                            *head_radius as f64,
                        ]);
                        rectangle(
                            RED.shade(i_lipid as f32 / 1.5 / state.lipids.len() as f32),
                            square,
                            objects_transform,
                            gl,
                        );
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
