use std::path::Path;

use crate::tiles::{Tile, Tiles};

#[derive(Clone, Debug)]
pub(crate) struct Board {
    width: usize,
    height: usize,
    // TODO: This doesn't belong here - it's a renderer thing, not board
    pixel_size: usize,
    tiles: Tiles,
}

impl Board {
    pub(crate) fn new(width: usize, height: usize, pixel_size: usize) -> Self {
        let mut tiles = Tiles::empty(width, height);
        for x in 0..width {
            tiles.set_at(x, 0, Tile::Rock);
            tiles.set_at(x, height - 1, Tile::Rock);
        }
        for y in 0..height {
            tiles.set_at(0, y, Tile::Rock);
            tiles.set_at(width - 1, y, Tile::Rock);
        }

        Self {
            width,
            height,
            tiles,
            pixel_size,
        }
    }

    pub(crate) fn swap(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let maybe_source = self.tiles.at(x1, y1).cloned();
        let maybe_target = self.tiles.at(x2, y2).cloned();
        if let (Some(source), Some(target)) = (maybe_source, maybe_target) {
            self.tiles.set_at(x1, y1, target);
            self.tiles.set_at(x2, y2, source);
        }
    }

    pub(crate) fn tiles(&self) -> &Tiles {
        &self.tiles
    }

    pub(crate) fn tiles_mut(&mut self) -> &mut Tiles {
        &mut self.tiles
    }

    pub(crate) fn width(&self) -> usize {
        self.width
    }

    pub(crate) fn height(&self) -> usize {
        self.height
    }

    // This will eventually be used when we add an option to load the board from bitmap.
    pub(crate) fn from_image(path: impl AsRef<Path>) -> Self {
        let image = image::open(path).unwrap().to_rgb8();
        let pixels = image.enumerate_pixels();

        let mut board = Self {
            width: image.width() as usize,
            height: image.height() as usize,
            tiles: Tiles::empty(image.width() as usize, image.height() as usize),
            pixel_size: 1,
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

    #[cfg(test)]
    pub(crate) fn new_from_str(width: usize, height: usize, tiles: &str) -> Self {
        Self {
            width,
            height,
            tiles: Tiles::from_str(tiles, width, height),
            pixel_size: 1,
        }
    }

    #[cfg(test)]
    pub(crate) fn _new_test_1() -> Self {
        const WIDTH: usize = 10;
        const HEIGHT: usize = 16;
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
            height: HEIGHT,
            tiles: Tiles::from_str(TEST_1, WIDTH, HEIGHT),
            pixel_size: 1,
        }
    }

    pub(crate) fn pixel_size(&self) -> usize {
        self.pixel_size
    }
}
