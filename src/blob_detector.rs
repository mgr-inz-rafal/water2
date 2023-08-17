use std::collections::BTreeSet;

use crate::{
    blobs::{Blob, Blobs},
    board::Board,
    point::Point,
    tiles::Tile,
};

pub(crate) struct BlobDetector<'a> {
    board: &'a Board,
}

impl<'a> BlobDetector<'a> {
    pub(crate) fn new(board: &'a Board) -> Self {
        Self { board }
    }

    fn try_insert_blob(
        &self,
        x: usize,
        y: usize,
        current_blob_points: &mut Blob,
        recursion_counter: &mut i32,
        visited: &mut BTreeSet<(usize, usize)>,
    ) {
        if visited.contains(&(x, y)) {
            return;
        }
        *recursion_counter += 1;
        if self.board.tiles().at(x, y) == &Tile::Water
            && !current_blob_points.contains(&Point::new(x, y))
        {
            current_blob_points.insert(Point::new(x, y));
            visited.insert((x, y));
            self.try_insert_blob(x + 1, y, current_blob_points, recursion_counter, visited);
            self.try_insert_blob(x - 1, y, current_blob_points, recursion_counter, visited);
            self.try_insert_blob(x, y + 1, current_blob_points, recursion_counter, visited);
            self.try_insert_blob(x, y - 1, current_blob_points, recursion_counter, visited);
        }
    }

    fn already_detected(&self, x: usize, y: usize, detected: &Blobs) -> bool {
        detected
            .values()
            .flatten()
            .any(|pt| pt == &Point::new(x, y))
    }

    pub(crate) fn detect(&self) -> Blobs {
        let mut detected: Blobs = Default::default();

        for y in 0..self.board.height() {
            for x in 0..self.board.width() {
                if !self.already_detected(x, y, &detected) {
                    let mut current_blob_points: Blob = Default::default();

                    let mut recursion_counter = 0;
                    let mut visited: BTreeSet<_> = Default::default();
                    self.try_insert_blob(
                        x,
                        y,
                        &mut current_blob_points,
                        &mut recursion_counter,
                        &mut visited,
                    );

                    if !current_blob_points.is_empty() {
                        detected.insert(
                            detected.keys().last().map_or(0, |last_key| last_key + 1),
                            current_blob_points,
                        );
                        println!("Blob detected with depth: {recursion_counter}");
                    }
                }
            }
        }

        detected
    }
}
