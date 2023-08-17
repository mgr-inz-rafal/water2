use std::path::Path;

use crate::tiles::{Tile, Tiles};

#[derive(Clone, Debug)]
pub(crate) struct Board {
    width: usize,
    height: usize,
    tiles: Tiles,
}

impl Board {
    pub(crate) fn swap(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let source = self.tiles.at(x1, y1).clone();
        let target = self.tiles.at(x2, y2).clone();
        self.tiles.set_at(x1, y1, target);
        self.tiles.set_at(x2, y2, source.clone());
    }

    pub(crate) fn tiles(&self) -> &Tiles {
        &self.tiles
    }

    pub(crate) fn width(&self) -> usize {
        self.width
    }

    pub(crate) fn height(&self) -> usize {
        self.height
    }

    pub(crate) fn from_image(path: impl AsRef<Path>) -> Self {
        let image = image::open(path).unwrap().to_rgb8();
        let pixels = image.enumerate_pixels();

        let mut board = Self {
            width: 320,
            height: 320,
            tiles: Tiles::empty(320, 320),
        };

        for (x, y, rgb) in pixels {
            match rgb.0 {
                // Black
                [0, 0, 0] => board.tiles.set_at(x as usize, y as usize, Tile::Rock),
                // White
                [255, 255, 255] => (),
                // Blue
                [0, 0, 255] => board.tiles.set_at(x as usize, y as usize, Tile::Water),
                _ => (), //panic!("unsupported color"),
            }
        }
        board
    }

    pub(crate) fn new_test_1() -> Self {
        const WIDTH: usize = 10;
        const TEST_1: &str = "##########\
                              #oooo..#.#\
                              #oooo#o#.#\
                              #ooo.#ooo#\
                              ##########\
                              #oooooooo#\
                              #oooooooo#\
                              #oooooooo#\
                              #oo#######\
                              #oo#.###.#\
                              #oo#.#...#\
                              #oo#.#.#.#\
                              #oo#...#.#\
                              #..#####.#\
                              #........#\
                              ##########";

        Self {
            width: WIDTH,
            height: 16,
            tiles: Tiles::from_str(TEST_1, WIDTH),
        }
    }
}
