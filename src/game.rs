use ggez::{
    event::{self, EventHandler},
    input::keyboard::KeyCode,
    Context, GameResult,
};

use crate::{
    console_painter::{ConsolePainter, HasBoard},
    engine::Engine,
    ggez_painter::GgezPainter,
    tiles::{Tile, TileUpdateOperation, TileUpdateRule},
};

#[derive(Default)]
pub(crate) struct GameConfig {
    pub(crate) console_preview: bool,

    // TODO: Support performance meters after there is an option to load board from file,
    // so we get repetitive results.
    pub(crate) _perf_tick: bool,
    pub(crate) _perf_blob_detect: bool,
}

pub struct Renderer {
    pub pixel_size: usize,
    pub left_button_down: bool,
    pub right_button_down: bool,
    pub middle_button_down: bool,
    tile_to_draw: Tile,
}

impl Default for Renderer {
    fn default() -> Self {
        Self {
            pixel_size: 4,
            left_button_down: false,
            right_button_down: false,
            middle_button_down: false,
            tile_to_draw: Tile::Rock,
        }
    }
}

pub(crate) struct Game {
    engine: Engine,
    cfg: GameConfig,
    renderer: Renderer,
}

impl Game {
    pub(crate) fn new(engine: Engine, cfg: GameConfig) -> Self {
        Self {
            engine,
            cfg,
            renderer: Default::default(),
        }
    }

    pub(crate) fn windows_size(&self) -> (usize, usize) {
        (
            self.renderer.pixel_size * self.engine.board().width(),
            self.renderer.pixel_size * self.engine.board().height(),
        )
    }

    fn update_tile(&mut self, x: usize, y: usize, op: &TileUpdateOperation) {
        // TODO: No magic numbers
        let engine = &mut self.engine;
        let board = engine.board_mut();
        let height = board.height();
        let width = board.width();
        let pixel_size = self.renderer.pixel_size;
        let tiles = board.tiles_mut();
        for xx in x - 10..x + 10 {
            for yy in y - 10..y + 10 {
                let current = tiles.at(xx / pixel_size, yy / pixel_size);
                if Self::within_bounds(xx, yy, height, width, pixel_size)
                    && TileUpdateRule::is_allowed(current, op)
                {
                    tiles.set_at(xx / pixel_size, yy / pixel_size, op.target())
                }
            }
        }
    }

    fn within_bounds(x: usize, y: usize, height: usize, width: usize, pixel_size: usize) -> bool {
        // TODO: No magic numbers
        x / pixel_size != 0
            && x / pixel_size != width - 1
            && y / pixel_size != 0
            && y / pixel_size != height - 1
    }

    fn purge_tile(&mut self, x: usize, y: usize) {
        self.update_tile(x, y, &TileUpdateOperation::Purge);
    }

    fn erase_tile(&mut self, x: usize, y: usize) {
        self.update_tile(x, y, &TileUpdateOperation::Erase);
    }

    fn draw_tile(&mut self, x: usize, y: usize) {
        self.update_tile(
            x,
            y,
            &TileUpdateOperation::Paint(self.renderer.tile_to_draw),
        );
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.engine.tick() {
            ctx.request_quit();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        GgezPainter::paint(&self.engine, &self.renderer, ctx).unwrap();
        if self.cfg.console_preview {
            ConsolePainter::paint(&self.engine);
            println!("Press Enter for next frame");
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
        }
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: event::MouseButton,
        x: f32,
        y: f32,
    ) -> Result<(), ggez::GameError> {
        match button {
            event::MouseButton::Left => {
                self.renderer.left_button_down = true;
                self.draw_tile(x as usize, y as usize);
            }
            event::MouseButton::Right => {
                self.renderer.right_button_down = true;
                self.erase_tile(x as usize, y as usize);
            }
            event::MouseButton::Middle => {
                self.renderer.middle_button_down = true;
                self.purge_tile(x as usize, y as usize);
            }
            event::MouseButton::Other(_) => (),
        }
        Ok(())
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        button: event::MouseButton,
        _x: f32,
        _y: f32,
    ) -> Result<(), ggez::GameError> {
        match button {
            event::MouseButton::Left => self.renderer.left_button_down = false,
            event::MouseButton::Right => self.renderer.right_button_down = false,
            event::MouseButton::Middle => self.renderer.middle_button_down = false,
            event::MouseButton::Other(_) => (),
        }
        Ok(())
    }

    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        x: f32,
        y: f32,
        _dx: f32,
        _dy: f32,
    ) -> Result<(), ggez::GameError> {
        match (
            self.renderer.left_button_down,
            self.renderer.middle_button_down,
            self.renderer.right_button_down,
        ) {
            (true, false, false) => self.draw_tile(x as usize, y as usize),
            (false, true, false) => self.purge_tile(x as usize, y as usize),
            (false, false, true) => self.erase_tile(x as usize, y as usize),
            _ => (),
        }

        Ok(())
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
    ) -> Result<(), ggez::GameError> {
        match input.keycode {
            Some(KeyCode::Key1) => self.renderer.tile_to_draw = Tile::Rock,
            Some(KeyCode::Key2) => self.renderer.tile_to_draw = Tile::Water,
            _ => (),
        }
        Ok(())
    }
}
