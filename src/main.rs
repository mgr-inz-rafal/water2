mod blob_detector;
mod blobs;
mod board;
mod console_painter;
mod engine;
mod game;
mod ggez_painter;
mod point;
mod tiles;

use blob_detector::BlobDetector;
use board::Board;

use engine::Engine;
use game::{Game, GameConfig};
use ggez::event::{self};
use ggez_painter::GgezPainter;

const TITLE: &str = "Przelewaj Sobie Wodziczkę";
const AUTHOR: &str = "mgr inż. Rafał";
const VERSION: &str = "0.1.1";

const WINDOW_WIDTH: usize = 1024;
const WINDOW_HEIGHT: usize = 768;

const PIXEL_SIZE: usize = 4;

const PLAYFIELD_WIDTH: usize = WINDOW_WIDTH / PIXEL_SIZE;
const PLAYFIELD_HEIGHT: usize = WINDOW_HEIGHT / PIXEL_SIZE;

fn main() {
    let (ctx, event_loop) = GgezPainter::init(WINDOW_WIDTH, WINDOW_HEIGHT, VERSION, TITLE, AUTHOR);

    let board = Board::new(PLAYFIELD_WIDTH, PLAYFIELD_HEIGHT, PIXEL_SIZE);

    let mut blob_detector = BlobDetector::new(&board);
    let blobs = blob_detector.detect_quick();

    let engine = Engine::new(board, blobs);

    let whatever = Game::new(
        engine,
        GameConfig {
            //console_preview: true,
            ..Default::default()
        },
    );

    event::run(ctx, event_loop, whatever);
}
