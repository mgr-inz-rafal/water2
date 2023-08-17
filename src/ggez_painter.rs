use std::{env, path::Path};

use ggez::{
    conf::WindowMode,
    event::EventLoop,
    graphics::{self, Color, DrawParam, Image, ImageFormat},
    Context, ContextBuilder,
};

use crate::console_painter::Paintable;

pub(crate) struct GgezPainter {}

impl GgezPainter {
    pub(crate) fn paint<T: Paintable>(playfield: &T, ctx: &mut Context) {
        let total_pixels = playfield.board().width() * playfield.board().height();
        let mut pixel_buffer: Vec<u8> = Vec::with_capacity(total_pixels);
        pixel_buffer.resize(total_pixels * 4, 0);

        let mut canvas = graphics::Canvas::from_frame(ctx, Color::RED);
        let mut index = 0;
        for y in 0..playfield.board().height() {
            for x in 0..playfield.board().width() {
                let tile = playfield.board().tiles().at(x, y);
                match tile {
                    crate::tiles::Tile::Rock => {
                        pixel_buffer[index] = 0;
                        pixel_buffer[index + 1] = 0;
                        pixel_buffer[index + 2] = 0;
                        pixel_buffer[index + 3] = 255;
                    }
                    crate::tiles::Tile::Water => {
                        pixel_buffer[index] = 0;
                        pixel_buffer[index + 1] = 0;
                        pixel_buffer[index + 2] = 255;
                        pixel_buffer[index + 3] = 255;
                    }
                    crate::tiles::Tile::Air => {
                        pixel_buffer[index] = 255;
                        pixel_buffer[index + 1] = 255;
                        pixel_buffer[index + 2] = 255;
                        pixel_buffer[index + 3] = 255;
                    }
                }
                index = index + 4;
            }
        }
        let image = Image::from_pixels(
            ctx,
            &pixel_buffer,
            ImageFormat::Rgba8UnormSrgb,
            playfield.board().width() as u32,
            playfield.board().height() as u32,
        );
        canvas.draw(&image, DrawParam::default());
        canvas.finish(ctx).unwrap();
    }

    // fn blob_index_from_point(x: usize, y: usize, blobs: &Blobs) -> Option<usize> {
    //     for (index, points) in blobs {
    //         if points.contains(&Point::new(x, y)) {
    //             return Some(*index);
    //         }
    //     }
    //     None
    // }

    pub(crate) fn init() -> (Context, EventLoop<()>) {
        let window_mode = WindowMode::default();
        let window_mode = window_mode.dimensions(800.0, 800.0);
        ContextBuilder::new("my_game", "Cool Game Author")
            .window_mode(window_mode)
            .build()
            .expect("aieee, could not create ggez context!")
    }
}
