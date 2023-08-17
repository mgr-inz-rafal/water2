mod blob_detector;
mod blobs;
mod board;
mod console_painter;
mod engine;
mod ggez_painter;
mod point;
mod tiles;

use std::env;

use blob_detector::BlobDetector;
use board::Board;

use console_painter::ConsolePainter;
use engine::Engine;
use ggez::{
    event::{self, EventHandler},
    Context, GameResult,
};
use ggez_painter::GgezPainter;

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
        GgezPainter::paint(&self.engine, ctx);
        Ok(())
    }
}

fn main() {
    let (mut ctx, event_loop) = GgezPainter::init();

    let manifest_dir = match env::var("CARGO_MANIFEST_DIR") {
        Ok(manifest_dir) => manifest_dir,
        Err(_) => ".".to_string(),
    };

    let mut board = Board::from_image(format!("{}/{}", manifest_dir, "resources/woter02.png"));

    let mut blob_detector = BlobDetector::new(&board);
    let blobs = blob_detector.detect_quick();

    let mut engine = Engine::new(board, blobs);

    let whatever = Whatever::new(engine);

    event::run(ctx, event_loop, whatever);
}
