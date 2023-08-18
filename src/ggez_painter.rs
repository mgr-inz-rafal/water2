use std::{env, path::Path};

use thiserror::Error;

use ggez::{
    conf::{NumSamples, WindowMode, WindowSetup},
    event::EventLoop,
    graphics::{self, Color, DrawMode, DrawParam, Image, ImageFormat, Mesh, MeshBuilder, Rect},
    Context, ContextBuilder,
};

use crate::console_painter::Paintable;

#[derive(Error, Debug)]
pub enum Error {
    #[error("unable to draw rectangle at ({0}, {1})")]
    UnableToDrawRectangle(usize, usize),
    #[error("finishing canvas failed")]
    UnableToFinishCanvasOperation,
}

pub(crate) struct GgezPainter {}

impl GgezPainter {
    pub(crate) fn paint<T: Paintable>(playfield: &T, ctx: &mut Context) -> Result<(), Error> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        let mut mesh_builder = MeshBuilder::default();
        for y in 0..playfield.board().height() {
            for x in 0..playfield.board().width() {
                mesh_builder
                    .rectangle(
                        DrawMode::fill(),
                        // TODO: No magic numbers
                        Rect::new(x as f32 * 2.0, y as f32 * 2.0, 2.0, 2.0),
                        match playfield.board().tiles().at(x, y) {
                            crate::tiles::Tile::Rock => Color::BLACK,
                            crate::tiles::Tile::Water => Color::CYAN,
                            crate::tiles::Tile::Air => Color::WHITE,
                        },
                    )
                    .map_err(|_| Error::UnableToDrawRectangle(x, y))?;
            }
        }
        let mesh = Mesh::from_data(ctx, mesh_builder.build());
        canvas.draw(&mesh, DrawParam::default());
        canvas
            .finish(ctx)
            .map_err(|_| Error::UnableToFinishCanvasOperation)?;

        Ok(())

        /*
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
        */
    }

    // fn blob_index_from_point(x: usize, y: usize, blobs: &Blobs) -> Option<usize> {
    //     for (index, points) in blobs {
    //         if points.contains(&Point::new(x, y)) {
    //             return Some(*index);
    //         }
    //     }
    //     None
    // }

    pub(crate) fn init(
        width: usize,
        height: usize,
        version: &str,
        title: &str,
        author: &str,
    ) -> (Context, EventLoop<()>) {
        let window_mode = WindowMode::default().dimensions(width as f32, height as f32);
        let window_setup = WindowSetup::default()
            .title(&format!("{title} by {author} - v{version}"))
            .samples(NumSamples::One);
        ContextBuilder::new(title, author)
            .window_mode(window_mode)
            .window_setup(window_setup)
            .build()
            .expect("aieee, could not create ggez context!")
    }
}
