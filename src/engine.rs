use std::{
    collections::{BTreeMap, BTreeSet},
    time::{Duration, Instant},
};

use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};

use crate::{
    blob_detector::BlobDetector,
    blobs::{Blob, Blobs},
    board::Board,
    console_painter::{HasBlobs, HasBoard, Paintable},
    point::Point,
};

#[derive(Clone)]
pub(crate) struct Engine {
    board: Board,
    blobs: Blobs,
    rng: ThreadRng,
    perf_check: Option<usize>,
    perf_data: Vec<Duration>,
}

impl Engine {
    pub(crate) fn new(board: Board, blobs: Blobs, perf_check: Option<usize>) -> Self {
        Self {
            board,
            blobs,
            rng: rand::thread_rng(),
            perf_check,
            perf_data: perf_check.map_or(Default::default(), Vec::with_capacity),
        }
    }

    fn board(&self) -> &Board {
        &self.board
    }

    pub(crate) fn board_mut(&mut self) -> &mut Board {
        &mut self.board
    }

    fn blobs(&self) -> &Blobs {
        &self.blobs
    }

    pub(crate) fn tick(&mut self) -> bool {
        let start = Instant::now();

        // TODO: Quite ugly and hacky, please rewrite.
        let mut new_blobs: BTreeMap<usize, Blob> = Default::default();

        for (index, blob) in &self.blobs {
            let mut new_points: BTreeSet<_> = Default::default();

            for pt in blob.into_iter() {
                // Try move down
                let dest_pt = Point::new(pt.x(), pt.y() + 1);
                let maybe_tile = self.board.tiles().at(dest_pt.x(), dest_pt.y());
                if let Some(tile) = maybe_tile {
                    if tile.is_air() {
                        self.board.swap(pt.x(), pt.y(), pt.x(), pt.y() + 1);
                        new_points.insert(dest_pt);
                    } else {
                        // Didn't move down, try sideways
                        let dest_pt_left = Point::new(pt.x() - 1, pt.y());
                        let dest_pt_right = Point::new(pt.x() + 1, pt.y());

                        let maybe_tile_left =
                            self.board.tiles().at(dest_pt_left.x(), dest_pt_left.y());
                        let maybe_tile_right =
                            self.board.tiles().at(dest_pt_right.x(), dest_pt_right.y());

                        if let (Some(tile_left), Some(tile_right)) =
                            (maybe_tile_left, maybe_tile_right)
                        {
                            match (tile_left.is_air(), tile_right.is_air()) {
                                (true, true) => {
                                    if self.rng.gen::<bool>() {
                                        self.board.swap(pt.x(), pt.y(), pt.x() - 1, pt.y());
                                        new_points.insert(Point::new(pt.x() - 1, pt.y()));
                                    } else {
                                        self.board.swap(pt.x(), pt.y(), pt.x() + 1, pt.y());
                                        new_points.insert(Point::new(pt.x() + 1, pt.y()));
                                    }
                                }
                                (true, false) => {
                                    self.board.swap(pt.x(), pt.y(), pt.x() - 1, pt.y());
                                    new_points.insert(Point::new(pt.x() - 1, pt.y()));
                                }
                                (false, true) => {
                                    self.board.swap(pt.x(), pt.y(), pt.x() + 1, pt.y());
                                    new_points.insert(Point::new(pt.x() + 1, pt.y()));
                                }
                                (false, false) => {
                                    new_points.insert(pt.clone());
                                }
                            }
                        }
                    }
                }
            }

            new_blobs.insert(*index, Blob::new(new_points));

            for (_, blob) in new_blobs.iter() {
                // No single droplet from this blob moved down, try move up.
                let Some(top_row) = blob.points().first() else {
                    continue;
                };
                let top_points: Vec<_> = blob
                    .points()
                    .iter()
                    .filter(|pt| pt.y() == top_row.y())
                    .collect();
                if top_points.is_empty() {
                    continue;
                }

                let destination_candidates: Vec<_> = blob
                    .points()
                    .iter()
                    .rev()
                    .filter(|pt| {
                        if let Some(pt_up) = self.board.tiles().at(pt.x(), pt.y() - 1) {
                            pt_up.is_air() && pt.y() != top_row.y()
                        } else {
                            false
                        }
                    })
                    .map(|pt| Point::new(pt.x(), pt.y() - 1))
                    .collect();

                if !destination_candidates.is_empty() {
                    let lowest_row = destination_candidates.first().unwrap().y();
                    let lowest_candidates: Vec<_> = destination_candidates
                        .iter()
                        .filter(|pt| pt.y() == lowest_row)
                        .collect();

                    if !lowest_candidates.is_empty() {
                        let top_point = top_points.choose(&mut self.rng).unwrap();
                        let destination_point = lowest_candidates.choose(&mut self.rng).unwrap();

                        self.board.swap(
                            top_point.x(),
                            top_point.y(),
                            destination_point.x(),
                            destination_point.y(),
                        );
                    }
                }
            }
        }

        // TODO: It's super inefficient to re-detect blobs each tick.
        // Split and merge blobs as they move.
        let mut blob_detector = BlobDetector::new(&self.board);
        self.blobs = blob_detector.detect_quick();

        let duration = start.elapsed();
        if let Some(samples) = self.perf_check.as_mut() {
            *samples -= 1;
            self.perf_data.push(duration);
            if *samples == 0 {
                println!("{:?}", self.perf_data);
                println!(
                    "avg={}",
                    self.perf_data
                        .iter()
                        .map(|duration| duration.as_millis())
                        .sum::<u128>()
                        / self.perf_data.len() as u128
                );
                return true;
            }
        }
        false
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
