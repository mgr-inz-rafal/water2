mod blob_detector;
mod blobs;
mod board;
mod console_painter;
mod engine;
mod point;
mod tiles;

use blob_detector::BlobDetector;
use board::Board;

use console_painter::ConsolePainter;
use engine::Engine;

fn main() {
    let board = Board::new_test_1();

    let blob_detector = BlobDetector::new(&board);
    let blobs = blob_detector.detect();

    let mut engine = Engine::new(board, blobs);

    loop {
        ConsolePainter::paint(&engine);
        engine = engine.tick();

        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
    }
}
