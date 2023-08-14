use std::collections::{BTreeMap, BTreeSet};

use colored::Colorize;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Rock,
    Water,
    Air,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Rock => write!(f, "{}", "#".purple()),
            Tile::Water => write!(f, "{}", "o".bold().blue()),
            Tile::Air => write!(f, "{}", ".".bright_black()),
        }
    }
}

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

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.at(x, y))?;
            }
            writeln!(f)?
        }
        Ok(())
    }
}

struct BlobDetector<'a> {
    board: &'a Board,
    detected: BTreeMap<usize, BTreeSet<(usize, usize)>>,
}

impl<'a> BlobDetector<'a> {
    fn new(board: &'a Board) -> Self {
        Self {
            board,
            detected: Default::default(),
        }
    }

    fn try_insert_blob(&mut self, x: usize, y: usize, det: &mut BTreeSet<(usize, usize)>) {
        if self.board.at(x, y) == &Tile::Water && !det.contains(&(x, y)) {
            det.insert((x, y));
            self.try_insert_blob(x + 1, y, det);
            self.try_insert_blob(x - 1, y, det);
            self.try_insert_blob(x, y + 1, det);
            self.try_insert_blob(x, y - 1, det);
        }
    }

    fn already_detected(&self, x: usize, y: usize) -> bool {
        self.detected
            .values()
            .flatten()
            .any(|(xx, yy)| xx == &x && yy == &y)
    }

    fn detect(&mut self) -> &BTreeMap<usize, BTreeSet<(usize, usize)>> {
        let mut index = 0;

        for y in 0..self.board.height {
            for x in 0..self.board.width {
                if !self.already_detected(x, y) {
                    let mut detected: BTreeSet<(usize, usize)> = Default::default();
                    self.try_insert_blob(x, y, &mut detected);

                    if !detected.is_empty() {
                        self.detected.insert(index, detected);
                        index += 1;
                    }
                }
            }
        }

        &self.detected
    }
}

fn main() {
    let board = Board::new_test_1();

    let mut blob_detector = BlobDetector::new(&board);
    let blobs = blob_detector.detect();

    dbg!(&blobs);

    println!("{board}");
}
