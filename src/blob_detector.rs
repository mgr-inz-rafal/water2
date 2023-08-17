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
    done: Vec<(usize, usize)>,
}

// TODO: Experiment with BTreeSets for potentially better lookup.
// When using Point instead of tuple (x,y), points should be sorted correctly (top row first)

#[derive(Debug)]
struct DetectedLineDef {
    start: usize,
    end: usize,
    touching: Vec<(usize, usize)>,
}

impl DetectedLineDef {
    fn new(start: usize, end: usize, touching: Vec<(usize, usize)>) -> Self {
        Self {
            start,
            end,
            touching,
        }
    }
}

impl<'a> BlobDetector<'a> {
    pub(crate) fn new(board: &'a Board) -> Self {
        Self {
            board,
            done: Default::default(),
        }
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

    fn find_line(&mut self, sx: usize, sy: usize) -> Option<DetectedLineDef> {
        let mut start = None;
        let mut touching = Vec::new();

        // TODO: This loop is probably not needed most of the time, since sx and sy point to the water
        for x in sx..self.board.width() {
            if self.board.tiles().at(x, sy) == &Tile::Water {
                start = Some(x);
                self.done.push((x, sy));

                if self.board.tiles().at(x, sy - 1) == &Tile::Water {
                    if !self.done.contains(&(x, sy - 1)) {
                        touching.push((x, sy - 1));
                    }
                }

                if self.board.tiles().at(x, sy + 1) == &Tile::Water {
                    if !self.done.contains(&(x, sy + 1)) {
                        touching.push((x, sy + 1));
                    }
                }

                break;
            }
        }

        if let Some(start) = start {
            let mut last_x = None;
            // Find to the right
            for x in start + 1..self.board.width() {
                if self.board.tiles().at(x, sy) != &Tile::Water {
                    break;
                } else {
                    last_x = Some(x);
                    println!("to right x={x}");
                    self.done.push((x, sy));

                    if self.board.tiles().at(x, sy - 1) == &Tile::Water {
                        if !self.done.contains(&(x, sy - 1)) {
                            touching.push((x, sy - 1));
                        }
                    }

                    if self.board.tiles().at(x, sy + 1) == &Tile::Water {
                        if !self.done.contains(&(x, sy + 1)) {
                            touching.push((x, sy + 1));
                        }
                    }
                }
            }

            // Find to the left
            for x in (0..start).rev() {
                println!("to left x={x}");
                if self.board.tiles().at(x, sy) != &Tile::Water {
                    return Some(DetectedLineDef {
                        start: x + 1,
                        end: last_x.unwrap_or(sx),
                        touching,
                    });
                } else {
                    self.done.push((x, sy));

                    if self.board.tiles().at(x, sy - 1) == &Tile::Water {
                        if !self.done.contains(&(x, sy - 1)) {
                            touching.push((x, sy - 1));
                        }
                    }

                    if self.board.tiles().at(x, sy + 1) == &Tile::Water {
                        if !self.done.contains(&(x, sy + 1)) {
                            touching.push((x, sy + 1));
                        }
                    }
                }
            }
        }

        None
    }

    fn find_first_water_point(&self) -> Option<(usize, usize)> {
        for y in 0..self.board.height() {
            for x in 0..self.board.width() {
                if self.board.tiles().at(x, y) == &Tile::Water {
                    return Some((x, y));
                }
            }
        }
        None
    }

    // TODO: no mut, hold the `done` as function local variable
    pub(crate) fn detect_quick(&mut self) -> Blobs {
        let first_point = self.find_first_water_point();
        let mut to_be_analyzed: VecDeque<_> = Default::default();
        let mut blob: Blob = Default::default();
        if let Some((x, y)) = first_point {
            let detected_line = self.find_line(x, y);
            dbg!(&detected_line);
            if let Some(detected_line) = detected_line {
                for b in detected_line.start..=detected_line.end {
                    blob.insert(Point::new(b, y));
                }
                to_be_analyzed.extend(detected_line.touching);
            }
        }

        loop {
            let tba = to_be_analyzed.pop_front();
            if let Some((x, y)) = tba {
                if self.done.contains(&(x, y)) {
                    continue;
                }
                println!("now checking {x},{y}");
                let detected_line = self.find_line(x, y);
                if let Some(detected_line) = detected_line {
                    dbg!(&detected_line);
                    for b in detected_line.start..=detected_line.end {
                        blob.insert(Point::new(b, y));
                    }
                    to_be_analyzed.extend(detected_line.touching);
                }
            } else {
                break;
            }
        }

        let mut blobs: Blobs = Default::default();
        blobs.insert(0, blob);

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
                             #oooooooooo#\
                             #o#oooooooo#\
                             #oooo#o##oo#\
                             #oooo#o##oo#\
                             #o#o#oo##oo#\
                             ############";
        let board = Board::new_from_str(12, 11, TILES);
        let mut detector = BlobDetector::new(&board);
        let blobs = detector.detect_quick();

        let (_, pts) = blobs.into_iter().next().unwrap();

        let result: Vec<_> = TILES
            .chars()
            .enumerate()
            .map(|(i, c)| {
                let y = i / 12;
                let x = i - y * 12;
                if pts.contains(&Point::new(x, y)) {
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

        let result_str: String = result.iter().collect();
        assert_eq!(result_str, TILES);
    }
}
