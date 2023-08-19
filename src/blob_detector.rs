use std::{
    collections::{BTreeSet, VecDeque},
    time::Instant,
};

use crate::{
    blobs::{Blob, Blobs},
    board::Board,
    point::Point,
    tiles::Tile,
};

pub(crate) struct BlobDetector<'a> {
    board: &'a Board,
    done: BTreeSet<(usize, usize)>,
}

// TODO: Experiment with BTreeSets for potentially better lookup.
// When using Point instead of tuple (x,y), points should be sorted correctly (top row first)

#[derive(Debug)]
struct DetectedLineDef {
    start: usize,
    end: usize,
    touching: Vec<(usize, usize)>,
}

impl<'a> BlobDetector<'a> {
    pub(crate) fn new(board: &'a Board) -> Self {
        Self {
            board,
            done: Default::default(),
        }
    }

    fn _try_insert_blob(
        &self,
        x: usize,
        y: usize,
        current_blob: &mut Blob,
        recursion_counter: &mut i32,
        visited: &mut BTreeSet<(usize, usize)>,
    ) {
        if visited.contains(&(x, y)) {
            return;
        }
        *recursion_counter += 1;
        if self.board.tiles().at(x, y) == Some(&Tile::Water)
            && !current_blob.points().contains(&Point::new(x, y))
        {
            current_blob.points_mut().insert(Point::new(x, y));
            visited.insert((x, y));
            self._try_insert_blob(x + 1, y, current_blob, recursion_counter, visited);
            self._try_insert_blob(x - 1, y, current_blob, recursion_counter, visited);
            self._try_insert_blob(x, y + 1, current_blob, recursion_counter, visited);
            self._try_insert_blob(x, y - 1, current_blob, recursion_counter, visited);
        }
    }

    #[cfg(test)]
    fn already_detected(&self, x: usize, y: usize, detected: &Blobs) -> bool {
        detected
            .values()
            .map(|blob| blob.points())
            .flatten()
            .any(|pt| pt == &Point::new(x, y))
    }

    #[cfg(test)]
    pub(crate) fn detect_slow(&self) -> Blobs {
        let start = Instant::now();
        let mut detected: Blobs = Default::default();

        for y in 0..self.board.height() {
            for x in 0..self.board.width() {
                if !self.already_detected(x, y, &detected) {
                    let mut current_blob: Blob = Default::default();

                    let mut recursion_counter = 0;
                    let mut visited: BTreeSet<_> = Default::default();
                    self._try_insert_blob(
                        x,
                        y,
                        &mut current_blob,
                        &mut recursion_counter,
                        &mut visited,
                    );

                    if !current_blob.points().is_empty() {
                        detected.insert(
                            detected.keys().last().map_or(0, |last_key| last_key + 1),
                            current_blob,
                        );
                    }
                }
            }
        }
        let duration = start.elapsed();
        println!("BD (slow): {duration:?}");

        detected
    }

    fn find_line(&mut self, sx: usize, sy: usize) -> Option<DetectedLineDef> {
        let mut start = None;
        let mut touching = Vec::new();

        // TODO: This loop is probably not needed most of the time, since sx and sy point to the water
        for x in sx..self.board.width() {
            if self.board.tiles().at(x, sy) == Some(&Tile::Water) {
                start = Some(x);
                self.done.insert((x, sy));

                if self.board.tiles().at(x, sy - 1) == Some(&Tile::Water)
                    && !self.done.contains(&(x, sy - 1))
                {
                    touching.push((x, sy - 1));
                }

                if self.board.tiles().at(x, sy + 1) == Some(&Tile::Water)
                    && !self.done.contains(&(x, sy + 1))
                {
                    touching.push((x, sy + 1));
                }

                break;
            }
        }

        if let Some(start) = start {
            let mut last_x = None;
            // Find to the right
            for x in start + 1..self.board.width() {
                if self.board.tiles().at(x, sy) != Some(&Tile::Water) {
                    break;
                } else {
                    last_x = Some(x);
                    self.done.insert((x, sy));

                    if self.board.tiles().at(x, sy - 1) == Some(&Tile::Water)
                        && !self.done.contains(&(x, sy - 1))
                    {
                        touching.push((x, sy - 1));
                    }

                    if self.board.tiles().at(x, sy + 1) == Some(&Tile::Water)
                        && !self.done.contains(&(x, sy + 1))
                    {
                        touching.push((x, sy + 1));
                    }
                }
            }

            // Find to the left
            for x in (0..start).rev() {
                if self.board.tiles().at(x, sy) != Some(&Tile::Water) {
                    return Some(DetectedLineDef {
                        start: x + 1,
                        end: last_x.unwrap_or(sx),
                        touching,
                    });
                } else {
                    self.done.insert((x, sy));

                    if self.board.tiles().at(x, sy - 1) == Some(&Tile::Water)
                        && !self.done.contains(&(x, sy - 1))
                    {
                        touching.push((x, sy - 1));
                    }

                    if self.board.tiles().at(x, sy + 1) == Some(&Tile::Water)
                        && !self.done.contains(&(x, sy + 1))
                    {
                        touching.push((x, sy + 1));
                    }
                }
            }
        }

        None
    }

    fn find_first_water_point(&mut self) -> Option<(usize, usize)> {
        for y in 0..self.board.height() {
            for x in 0..self.board.width() {
                if !self.done.contains(&(x, y)) && self.board.tiles().at(x, y) == Some(&Tile::Water)
                {
                    self.done.insert((x, y));
                    return Some((x, y));
                }
            }
        }
        None
    }

    // TODO: no mut, hold the `done` as function local variable
    pub(crate) fn detect_quick(&mut self) -> Blobs {
        let start = Instant::now();
        let mut index = 0;
        let mut blobs: Blobs = Default::default();
        loop {
            let first_point = self.find_first_water_point();
            let mut to_be_analyzed: VecDeque<_> = Default::default();
            let mut blob: Blob = Default::default();
            if let Some((x, y)) = first_point {
                let detected_line = self.find_line(x, y);
                if let Some(detected_line) = detected_line {
                    for b in detected_line.start..=detected_line.end {
                        blob.points_mut().insert(Point::new(b, y));
                    }
                    to_be_analyzed.extend(detected_line.touching);
                }
            } else {
                break;
            }

            loop {
                let tba = to_be_analyzed.pop_front();
                if let Some((x, y)) = tba {
                    if self.done.contains(&(x, y)) {
                        continue;
                    }
                    let detected_line = self.find_line(x, y);
                    if let Some(detected_line) = detected_line {
                        for b in detected_line.start..=detected_line.end {
                            blob.points_mut().insert(Point::new(b, y));
                        }
                        to_be_analyzed.extend(detected_line.touching);
                    }
                } else {
                    break;
                }
            }

            blobs.insert(index, blob);
            index += 1;
        }

        let _duration = start.elapsed();

        blobs
    }
}

#[cfg(test)]
mod tests {
    use crate::{blob_detector::BlobDetector, board::Board, point::Point};

    #[test]
    fn detects_blob() {
        const TILES: &str = "############\
                             ####oooooo##\
                             #o####o#oo##\
                             #oo##oo#o###\
                             #oo##oo###o#\
                             #ooooooo#oo#\
                             #o#ooooo#oo#\
                             #oooo#o##oo#\
                             #oooo#o##oo#\
                             #o#o#oo##oo#\
                             ############";
        let board = Board::new_from_str(12, 11, TILES);
        let mut detector = BlobDetector::new(&board);
        let blobs = detector.detect_quick();

        dbg!(&blobs.len());

        let blobs_slow = detector.detect_slow();
        dbg!(&blobs_slow.len());

        assert_eq!(blobs, blobs_slow);

        let (_, blob) = blobs.into_iter().next().unwrap();

        let result: Vec<_> = TILES
            .chars()
            .enumerate()
            .map(|(i, _)| {
                let y = i / 12;
                let x = i - y * 12;
                if blob.points().contains(&Point::new(x, y)) {
                    'o'
                } else {
                    '#'
                }
            })
            .collect();
        result.chunks(12).for_each(|chunk| {
            chunk.into_iter().for_each(|c| print!("{c}"));
            println!();
        });

        // let result_str: String = result.iter().collect();
        // assert_eq!(result_str, TILES);
    }
}
