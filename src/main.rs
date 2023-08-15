use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
};

use colored::Colorize;

#[derive(Clone, Debug)]
struct Point((usize, usize));

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self((x, y))
    }

    fn x(&self) -> usize {
        self.inner().0
    }

    fn y(&self) -> usize {
        self.inner().1
    }

    fn inner(&self) -> &(usize, usize) {
        &self.0
    }
}

type Blob = BTreeSet<Point>;
type Blobs = BTreeMap<usize, Blob>;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Rock,
    Water,
    Air,
}

impl Tile {
    fn is_air(&self) -> bool {
        self == &Tile::Air
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.x().cmp(&other.x()), self.y().cmp(&other.y())) {
            (Ordering::Equal, Ordering::Equal) => Ordering::Equal,
            (Ordering::Less, Ordering::Equal) | (_, Ordering::Less) => Ordering::Less,
            (Ordering::Greater, Ordering::Equal) | (_, Ordering::Greater) => Ordering::Greater,
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Point {}

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

    fn set_at(&mut self, x: usize, y: usize, tile: Tile) {
        *self.tiles.0.get_mut(y).unwrap().get_mut(x).unwrap() = tile;
    }

    fn swap(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let source = self.at(x1, y1).clone();
        let target = self.at(x2, y2).clone();
        self.set_at(x1, y1, target);
        self.set_at(x2, y2, source);
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
        if points.contains(&Point((x, y))) {
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
        if self.board.at(x, y) == &Tile::Water && !current_blob_points.contains(&Point((x, y))) {
            current_blob_points.insert(Point((x, y)));
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
            .any(|Point((xx, yy))| xx == &x && yy == &y) // TODO: Use eq here
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
                if board.at(dest_pt.x(), dest_pt.y()).is_air() {
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
        engine = engine.tick();

        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
    }
}
