use std::collections::{BTreeMap, BTreeSet};

use colored::Colorize;

type Blob = BTreeSet<(usize, usize)>;
type Blobs = BTreeMap<usize, Blob>;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Rock,
    Water,
    Air,
}

const COLORS: &[(u8, u8, u8)] = &[(255, 0, 0), (0, 255, 0), (0, 0, 255)];

struct Tiles(Vec<Vec<Tile>>);

impl Tiles {
    fn from_str(s: &str, width: usize) -> Self {
        Self(
            s.chars()
                .collect::<Vec<_>>()
                .chunks(width)
                .map(|chunk| {
                    chunk
                        .iter()
                        .map(|c| match c {
                            '#' => Tile::Rock,
                            '.' => Tile::Air,
                            'o' => Tile::Water,
                            _ => panic!("unknown tile: {c}"),
                        })
                        .collect()
                })
                .collect(),
        )
    }
}

struct Board {
    width: usize,
    height: usize,
    tiles: Tiles,
}

impl Board {
    fn at(&self, x: usize, y: usize) -> &Tile {
        self.tiles.0.get(y).unwrap().get(x).unwrap()
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
        if points.contains(&(x, y)) {
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
        if self.board.at(x, y) == &Tile::Water && !current_blob_points.contains(&(x, y)) {
            current_blob_points.insert((x, y));
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
            .any(|(xx, yy)| xx == &x && yy == &y)
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

    fn tick(&mut self) {}
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
                    let c = playfield.board().at(x, y);
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
        engine.tick();

        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
    }
}
