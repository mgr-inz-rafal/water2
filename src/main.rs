mod blob_detector;
mod board;
mod point;
mod tiles;

use std::collections::{BTreeMap, BTreeSet};

use blob_detector::BlobDetector;
use board::Board;
use colored::Colorize;

use point::Point;
use tiles::Tile;

type Blob = BTreeSet<Point>;
type Blobs = BTreeMap<usize, Blob>;

const COLORS: &[(u8, u8, u8)] = &[(255, 0, 0), (0, 255, 0), (0, 0, 255)];

fn blob_index_from_point(x: usize, y: usize, blobs: &Blobs) -> Option<usize> {
    for (index, points) in blobs {
        if points.contains(&Point::new(x, y)) {
            return Some(*index);
        }
    }
    None
}

struct Engine {
    board: Board,
    blobs: Blobs,
}

impl Engine {
    fn new(board: Board, blobs: Blobs) -> Self {
        Self { board, blobs }
    }

    fn board(&self) -> &Board {
        &self.board
    }

    fn blobs(&self) -> &Blobs {
        &self.blobs
    }

    fn tick(self) -> Engine {
        let Engine { mut board, blobs } = self;

        for (_, points) in blobs {
            for pt in points.iter().rev() {
                let dest_pt = Point::new(pt.x(), pt.y() + 1);
                if board.tiles().at(dest_pt.x(), dest_pt.y()).is_air() {
                    board.swap(pt.x(), pt.y(), pt.x(), pt.y() + 1);
                }
            }
        }

        let blob_detector = BlobDetector::new(&board);
        let blobs = blob_detector.detect();

        Engine { board, blobs }
    }
}

impl HasBlobs for Engine {
    fn blobs(&self) -> &Blobs {
        self.blobs()
    }
}

impl HasBoard for Engine {
    fn board(&self) -> &Board {
        self.board()
    }
}

impl Paintable for Engine {}

trait HasBoard {
    fn board(&self) -> &Board;
}

trait HasBlobs {
    fn blobs(&self) -> &Blobs;
}

trait Paintable: HasBlobs + HasBoard {}

struct ConsolePainter {}

impl ConsolePainter {
    fn paint<T: Paintable>(playfield: &T) {
        for y in 0..playfield.board().height() {
            for x in 0..playfield.board().width() {
                if let Some(blob_index) = blob_index_from_point(x, y, playfield.blobs()) {
                    let (r, g, b) = COLORS[blob_index];
                    print!("{}", "o".truecolor(r, g, b))
                } else {
                    let c = playfield.board().tiles().at(x, y);
                    match c {
                        Tile::Rock => print!("{}", "#".purple()),
                        Tile::Water => print!("{}", ",".white()),
                        Tile::Air => print!("{}", ".".bright_black()),
                    }
                }
            }
            println!()
        }
    }
}

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
