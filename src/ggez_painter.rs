use thiserror::Error;

use ggez::{
    conf::{NumSamples, WindowMode, WindowSetup},
    event::EventLoop,
    graphics::{self, Color, DrawMode, DrawParam, Mesh, MeshBuilder, Rect},
    Context, ContextBuilder,
};

use crate::{console_painter::Paintable, game::Renderer, tiles::Tile};

#[derive(Error, Debug)]
pub enum Error {
    #[error("unable to draw rectangle at ({0}, {1})")]
    UnableToDrawRectangle(usize, usize),
    #[error("finishing canvas failed")]
    UnableToFinishCanvasOperation,
}

pub(crate) struct GgezPainter {}

impl GgezPainter {
    pub(crate) fn paint<T: Paintable>(
        playfield: &T,
        renderer: &Renderer,
        ctx: &mut Context,
    ) -> Result<(), Error> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        let pixel_size = renderer.pixel_size as f32;

        let mut mesh_builder = MeshBuilder::default();
        for y in 0..playfield.board().height() {
            for x in 0..playfield.board().width() {
                mesh_builder
                    .rectangle(
                        DrawMode::fill(),
                        Rect::new(
                            x as f32 * pixel_size,
                            y as f32 * pixel_size,
                            pixel_size,
                            pixel_size,
                        ),
                        match playfield.board().tiles().at(x, y) {
                            Some(Tile::Rock) => Color::BLACK,
                            Some(Tile::Water) => Color::BLUE,
                            Some(Tile::Air) => Color::WHITE,
                            None => Color::MAGENTA,
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
    }

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
