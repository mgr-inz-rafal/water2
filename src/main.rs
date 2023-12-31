mod blob_detector;
mod blobs;
mod board;
mod console_painter;
mod engine;
mod game;
mod ggez_painter;
mod point;
mod tiles;

use clap::Parser;

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

#[derive(Parser, Debug)]
struct Args {
    /// Picture to laod.
    #[arg(short, long)]
    picture: Option<String>,
    /// Enables performance check. Engine will run first X frames and provide timing data on stdout.
    #[arg(short = 'c', long)]
    perf_check: Option<usize>,
}

fn main() {
    let args = Args::parse();

    let board = match args.picture {
        Some(path) => Board::from_image(path),
        None => Board::new(PLAYFIELD_WIDTH, PLAYFIELD_HEIGHT),
    };

    //let board = Board::_new_test_1();

    let mut blob_detector = BlobDetector::new(&board);
    let blobs = blob_detector.detect_quick();

    let engine = Engine::new(board, blobs, args.perf_check);

    let game = Game::new(
        engine,
        GameConfig {
            //console_preview: true,
            ..Default::default()
        },
    );

    let (window_width, window_height) = game.windows_size();
    let (ctx, event_loop) = GgezPainter::init(window_width, window_height, VERSION, TITLE, AUTHOR);

    event::run(ctx, event_loop, game);
}
