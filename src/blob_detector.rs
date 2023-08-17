use std::{collections::BTreeSet, time::Instant};

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
        let start = Instant::now();
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
                    }
                }
            }
        }
        let duration = start.elapsed();
        println!("BD: {duration:?}");

        detected
    }
}

#[cfg(test)]
mod tests {
    use crate::{blob_detector::BlobDetector, board::Board, point::Point};

    #[test]
    fn detects_blob() {
        const TILES: &str = "############\
                             ######ooooo#\
                             #o####ooooo#\
                             #oo###oo####\
                             #oo###oo##o#\
                             #oooooooooo#\
                             #o#oooooooo#\
                             #oooo#o##oo#\
                             #oooo#o##oo#\
                             #o#o#oo##oo#\
                             ############";
        let board = Board::new_from_str(12, 11, TILES);
        let detector = BlobDetector::new(&board);
        let blobs = detector.detect();

        let (_, pts) = blobs.into_iter().next().unwrap();

        let result: Vec<_> = TILES
            .chars()
            .enumerate()
            .map(|(i, c)| {
                let y = i / 12;
                let x = i - y * 12;
                if pts.contains(&Point::new(x, y)) {
                    'O'
                } else {
                    '.'
                }
            })
            .collect();
        let result_str: String = result.iter().collect();
        assert_eq!(
            result_str,
            "............\
             ......OOOOO.\
             .O....OOOOO.\
             .OO...OO....\
             .OO...OO..O.\
             .OOOOOOOOOO.\
             .O.OOOOOOOO.\
             .OOOO.O..OO.\
             .OOOO.O..OO.\
             .O.O.OO..OO.\
             ............"
        );

        result.chunks(12).for_each(|chunk| {
            chunk.into_iter().for_each(|c| print!("{c}"));
            println!();
        });
    }
}
