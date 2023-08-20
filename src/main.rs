mod blob_detector;
mod blobs;
mod board;
mod console_painter;
mod engine;
mod ggez_painter;
mod point;
mod tiles;

use blob_detector::BlobDetector;
use board::Board;

use engine::Engine;
use ggez::input::keyboard::KeyCode;
use ggez::{
    event::{self, EventHandler},
    Context, GameResult,
};
use ggez_painter::GgezPainter;
use tiles::{Tile, TileUpdateOperation, TileUpdateRule};

const TITLE: &str = "Przelewaj Sobie Wodziczkę";
const AUTHOR: &str = "mgr inż. Rafał";
const VERSION: &str = "0.1.1";

const WINDOW_WIDTH: usize = 1024;
const WINDOW_HEIGHT: usize = 768;

const PIXEL_SIZE: usize = 4;

const PLAYFIELD_WIDTH: usize = WINDOW_WIDTH / PIXEL_SIZE;
const PLAYFIELD_HEIGHT: usize = WINDOW_HEIGHT / PIXEL_SIZE;

// fn main() {
//     let board = Board::new_test_1();

//     let blob_detector = BlobDetector::new(&board);
//     let blobs = blob_detector.detect();

//     let mut engine = Engine::new(board, blobs);

//     loop {
//         ConsolePainter::paint(&engine);

//         engine = engine.tick();

//         let mut line = String::new();
//         std::io::stdin().read_line(&mut line).unwrap();
//     }
// }

struct Game {
    engine: Engine,

    // Drawing
    // TODO: Extract to dedicated struct
    left_button_down: bool,
    right_button_down: bool,
    middle_button_down: bool,
    tile_to_draw: Tile,
}

impl Game {
    fn new(engine: Engine) -> Self {
        Self {
            engine,
            left_button_down: false,
            right_button_down: false,
            middle_button_down: false,
            tile_to_draw: Tile::Rock,
        }
    }

    fn update_tile(&mut self, x: usize, y: usize, op: &TileUpdateOperation) {
        // TODO: No magic numbers
        let engine = &mut self.engine;
        let board = engine.board_mut();
        let height = board.height();
        let width = board.width();
        let pixel_size = board.pixel_size();
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
        self.update_tile(x, y, &TileUpdateOperation::Paint(self.tile_to_draw));
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.engine.tick();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        GgezPainter::paint(&self.engine, ctx).unwrap();
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
                self.left_button_down = true;
                self.draw_tile(x as usize, y as usize);
            }
            event::MouseButton::Right => {
                self.right_button_down = true;
                self.erase_tile(x as usize, y as usize);
            }
            event::MouseButton::Middle => {
                self.middle_button_down = true;
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
            event::MouseButton::Left => self.left_button_down = false,
            event::MouseButton::Right => self.right_button_down = false,
            event::MouseButton::Middle => self.middle_button_down = false,
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
            self.left_button_down,
            self.middle_button_down,
            self.right_button_down,
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
            Some(KeyCode::Key1) => self.tile_to_draw = Tile::Rock,
            Some(KeyCode::Key2) => self.tile_to_draw = Tile::Water,
            _ => (),
        }
        Ok(())
    }
}

fn main() {
    let (ctx, event_loop) = GgezPainter::init(WINDOW_WIDTH, WINDOW_HEIGHT, VERSION, TITLE, AUTHOR);

    // let manifest_dir = match env::var("CARGO_MANIFEST_DIR") {
    //     Ok(manifest_dir) => manifest_dir,
    //     Err(_) => ".".to_string(),
    // };

    //let mut board = Board::from_image(format!("{}/{}", manifest_dir, "resources/woter02.png"));
    let board = Board::new(PLAYFIELD_WIDTH, PLAYFIELD_HEIGHT, PIXEL_SIZE);

    let mut blob_detector = BlobDetector::new(&board);
    let blobs = blob_detector.detect_quick();

    let engine = Engine::new(board, blobs);

    let whatever = Game::new(engine);

    event::run(ctx, event_loop, whatever);
}
