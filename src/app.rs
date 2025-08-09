use image::RgbaImage;
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, Texture, TextureSettings};
use piston::input::{RenderArgs, UpdateArgs};
use std::time::Duration;

use crate::types::*;

pub struct App<'a> {
    gl: GlGraphics,
    state: State,
    glyph_cache: GlyphCache<'a>,
    debug_texture0: Texture,
}

impl App<'_> {
    pub fn new() -> Self {
        Self {
            gl: GlGraphics::new(OpenGL::V4_2),
            state: State::new(),
            glyph_cache: GlyphCache::new("/usr/share/fonts/TTF/DejaVuSans.ttf", (), TextureSettings::new()).unwrap(),
            debug_texture0: opengl_graphics::CreateTexture::create(
                &mut (),
                opengl_graphics::Format::Rgba8,
                &[0u8; 400 * 400 * 4],
                [400, 400],
                &TextureSettings::new(),
            )
            .unwrap(),
        }
    }

    pub fn render(&mut self, args: &RenderArgs, max_fps: u64) {
        use graphics::color::*;
        use graphics::*;

        let state = &self.state;
        let glyph_cache = &mut self.glyph_cache;
        let debug_texture0 = &mut self.debug_texture0;

        let mut img = RgbaImage::new(400, 400);
        img.put_pixel(0, 0, ::image::Rgba::<u8>([0, 255, 0, 255]));
        img.put_pixel(100, 100, ::image::Rgba::<u8>([255, 0, 0, 255]));
        debug_texture0.update(&img);

        self.gl.draw(args.viewport(), |c, gl| {
            let scale = 2.5;
            let objects_transform = c.transform.scale(scale, scale);

            clear(BLACK, gl);
            ::graphics::image(debug_texture0, objects_transform, gl);

            for (i_lipid, lipid) in state.lipids.iter().enumerate() {
                match lipid {
                    Lipid {
                        head_position,
                        tail_position,
                        linear_velocity: _,
                        angular_velocity: _,
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
                        linear_velocity: _,
                        angular_velocity: _,
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

            let min_frame_time = Duration::new(0, (1_000_000_000.0 / max_fps as f64) as u32);
            text::Text::new_color(WHITE, 16)
                .draw(
                    &format!("Min. Frame Time: {min_frame_time:?}"),
                    glyph_cache,
                    &DrawState::default(),
                    c.transform.trans(0.0, 16.0),
                    gl,
                )
                .unwrap();

            let tick_time = state.tick_time;
            text::Text::new_color(WHITE, 16)
                .draw(
                    &format!("Tick Time: {tick_time:?}"),
                    glyph_cache,
                    &DrawState::default(),
                    c.transform.trans(0.0, 32.0),
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
