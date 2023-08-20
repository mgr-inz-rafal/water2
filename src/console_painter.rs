// Console painter is temporarily not used at all.
#![allow(dead_code)]

use colored::Colorize;

use crate::{blobs::Blobs, board::Board, point::Point, tiles::Tile};

pub(crate) trait HasBoard {
    fn board(&self) -> &Board;
}

pub(crate) trait HasBlobs {
    fn blobs(&self) -> &Blobs;
}

pub(crate) trait Paintable: HasBlobs + HasBoard {}

const COLORS: &[(u8, u8, u8)] = &[
    (255, 0, 0),
    (0, 255, 0),
    (0, 0, 255),
    (255, 255, 0),
    (255, 0, 255),
    (0, 255, 255),
    (128, 0, 0),
    (0, 128, 0),
    (0, 0, 128),
    (128, 128, 0),
    (128, 0, 128),
    (0, 128, 128),
];

// Don't allow huge previews on the console, as it is only used for debugging using a smallish boards.
const DRAW_LIMIT: usize = 20;

pub(crate) struct ConsolePainter {}

impl ConsolePainter {
    pub(crate) fn paint<T: Paintable>(playfield: &T) {
        for y in 0..std::cmp::min(playfield.board().height(), DRAW_LIMIT) {
            for x in 0..std::cmp::min(playfield.board().width(), DRAW_LIMIT) {
                if let Some(blob_index) = Self::blob_index_from_point(x, y, playfield.blobs()) {
                    let (r, g, b) = COLORS[blob_index];
                    print!("{}", "o".truecolor(r, g, b))
                } else {
                    let c = playfield.board().tiles().at(x, y);
                    match c {
                        Some(Tile::Rock) => print!("{}", "#".black().on_white()),
                        Some(Tile::Water) => print!("{}", ",".white()),
                        Some(Tile::Air) => print!("{}", ".".bright_black()),
                        None => print!("{}", "?".magenta()),
                    }
                }
            }
            println!()
        }
    }

    fn blob_index_from_point(x: usize, y: usize, blobs: &Blobs) -> Option<usize> {
        for (index, blob) in blobs {
            if blob.points().contains(&Point::new(x, y)) {
                return Some(*index);
            }
        }
        None
    }
}
