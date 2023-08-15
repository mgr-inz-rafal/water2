mod point;
mod tiles;

use std::collections::{BTreeMap, BTreeSet};

use colored::Colorize;

use point::Point;
use tiles::{Tile, Tiles};

type Blob = BTreeSet<Point>;
type Blobs = BTreeMap<usize, Blob>;

const COLORS: &[(u8, u8, u8)] = &[(255, 0, 0), (0, 255, 0), (0, 0, 255)];

struct Board {
    width: usize,
    height: usize,
    tiles: Tiles,
}

impl Board {
    fn swap(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let source = self.tiles.at(x1, y1).clone();
        let target = self.tiles.at(x2, y2).clone();
        self.tiles.set_at(x1, y1, target);
        self.tiles.set_at(x2, y2, source);
    }

    fn new_test_1() -> Self {
        const WIDTH: usize = 10;
        const TEST_1: &str = "##########\
                              #.oo.o...#\
                              #.oooo...#\
                              #..o.....#\
                              #........#\
                              #....o...#\
                              #...ooo..#\
                              #....o...#\
                              #........#\
                              #........#\
                              #........#\
                              #........#\
                              #........#\
                              #........#\
                              #........#\
                              ##########";

        Self {
            width: WIDTH,
            height: 16,
            tiles: Tiles::from_str(TEST_1, WIDTH),
        }
    }
}

fn blob_index_from_point(x: usize, y: usize, blobs: &Blobs) -> Option<usize> {
    for (index, points) in blobs {
        if points.contains(&Point::new(x, y)) {
            return Some(*index);
        }
    }
    None
}

struct BlobDetector<'a> {
    board: &'a Board,
}

impl<'a> BlobDetector<'a> {
    fn new(board: &'a Board) -> Self {
        Self { board }
    }

    fn try_insert_blob(&self, x: usize, y: usize, current_blob_points: &mut Blob) {
        if self.board.tiles.at(x, y) == &Tile::Water
            && !current_blob_points.contains(&Point::new(x, y))
        {
            current_blob_points.insert(Point::new(x, y));
            self.try_insert_blob(x + 1, y, current_blob_points);
            self.try_insert_blob(x - 1, y, current_blob_points);
            self.try_insert_blob(x, y + 1, current_blob_points);
            self.try_insert_blob(x, y - 1, current_blob_points);
        }
    }

    fn already_detected(&self, x: usize, y: usize, detected: &Blobs) -> bool {
        detected
            .values()
            .flatten()
            .any(|pt| pt == &Point::new(x, y))
    }

    fn detect(&self) -> Blobs {
        let mut detected: Blobs = Default::default();

        for y in 0..self.board.height {
            for x in 0..self.board.width {
                if !self.already_detected(x, y, &detected) {
                    let mut current_blob_points: Blob = Default::default();
                    self.try_insert_blob(x, y, &mut current_blob_points);

                    if !current_blob_points.is_empty() {
                        detected.insert(
                            detected.keys().last().map_or(0, |last_key| last_key + 1),
                            current_blob_points,
                        );
                    }
                }
            }
        }

        detected
    }
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
                if board.tiles.at(dest_pt.x(), dest_pt.y()).is_air() {
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
        for y in 0..playfield.board().height {
            for x in 0..playfield.board().width {
                if let Some(blob_index) = blob_index_from_point(x, y, playfield.blobs()) {
                    let (r, g, b) = COLORS[blob_index];
                    print!("{}", "o".truecolor(r, g, b))
                } else {
                    let c = playfield.board().tiles.at(x, y);
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
