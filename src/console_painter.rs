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

pub(crate) struct ConsolePainter {}

impl ConsolePainter {
    pub(crate) fn paint<T: Paintable>(playfield: &T) {
        for y in 0..playfield.board().height() {
            for x in 0..playfield.board().width() {
                if let Some(blob_index) = Self::blob_index_from_point(x, y, playfield.blobs()) {
                    let (r, g, b) = COLORS[blob_index];
                    print!("{}", "o".truecolor(r, g, b))
                } else {
                    let c = playfield.board().tiles().at(x, y);
                    match c {
                        Tile::Rock => print!("{}", "#".black().on_white()),
                        Tile::Water => print!("{}", ",".white()),
                        Tile::Air => print!("{}", ".".bright_black()),
                    }
                }
            }
            println!()
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
}
