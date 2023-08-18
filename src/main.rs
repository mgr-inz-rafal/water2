mod blob_detector;
mod blobs;
mod board;
mod console_painter;
mod engine;
mod ggez_painter;
mod point;
mod tiles;

use std::{env, f32::consts::PI};

use blob_detector::BlobDetector;
use board::Board;

use console_painter::ConsolePainter;
use engine::Engine;
use ggez::{
    event::{self, EventHandler},
    Context, GameResult,
};
use ggez_painter::GgezPainter;

const TITLE: &str = "Przelewaj";
const AUTHOR: &str = "mgr inż. Rafał";
const VERSION: &str = "0.1.1";

const WINDOW_WIDTH: usize = 1024;
const WINDOW_HEIGHT: usize = 768;

const PIXEL_SIZE: usize = 2;

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

struct Whatever {
    engine: Engine,
}

impl Whatever {
    fn new(engine: Engine) -> Self {
        Self { engine }
    }
}

impl EventHandler for Whatever {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let x = self.engine.clone();
        self.engine = x.tick();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        GgezPainter::paint(&self.engine, ctx).unwrap();
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
    let board = Board::new(PLAYFIELD_WIDTH, PLAYFIELD_HEIGHT);

    let mut blob_detector = BlobDetector::new(&board);
    let blobs = blob_detector.detect_quick();

    let engine = Engine::new(board, blobs);

    let whatever = Whatever::new(engine);

    event::run(ctx, event_loop, whatever);
}
