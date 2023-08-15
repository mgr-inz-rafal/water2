use std::{
    collections::{BTreeMap, BTreeSet},
    thread::Thread,
};

use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};

use crate::{
    blob_detector::BlobDetector,
    blobs::{Blob, Blobs},
    board::Board,
    console_painter::{HasBlobs, HasBoard, Paintable},
    point::Point,
};

pub(crate) struct Engine {
    board: Board,
    blobs: Blobs,
    rng: ThreadRng,
}

impl Engine {
    pub(crate) fn new(board: Board, blobs: Blobs) -> Self {
        Self {
            board,
            blobs,
            rng: rand::thread_rng(),
        }
    }

    fn board(&self) -> &Board {
        &self.board
    }

    fn blobs(&self) -> &Blobs {
        &self.blobs
    }

    pub(crate) fn tick(self) -> Engine {
        let Engine {
            mut board,
            blobs,
            mut rng,
        } = self;

        // TODO: Quite ugly and hacky, please revrite.
        let mut new_blobs: BTreeMap<usize, (Blob, bool)> = Default::default();

        for (index, mut points) in blobs {
            let mut new_points: BTreeSet<_> = Default::default();

            let mut did_move_down = false;
            for pt in points.iter().rev() {
                // Try move down
                let dest_pt = Point::new(pt.x(), pt.y() + 1);
                if board.tiles().at(dest_pt.x(), dest_pt.y()).is_air() {
                    board.swap(pt.x(), pt.y(), pt.x(), pt.y() + 1);
                    new_points.insert(dest_pt);
                    did_move_down = true;
                } else {
                    new_points.insert(pt.clone());

                    // Didn't move down, try sideways
                    let dest_pt_left = Point::new(pt.x() - 1, pt.y());
                    let dest_pt_right = Point::new(pt.x() + 1, pt.y());
                    match (
                        board
                            .tiles()
                            .at(dest_pt_left.x(), dest_pt_left.y())
                            .is_air(),
                        board
                            .tiles()
                            .at(dest_pt_right.x(), dest_pt_right.y())
                            .is_air(),
                    ) {
                        (true, true) => {
                            if rng.gen::<bool>() {
                                board.swap(pt.x(), pt.y(), pt.x() - 1, pt.y());
                                new_points.insert(Point::new(pt.x() - 1, pt.y()));
                            } else {
                                board.swap(pt.x(), pt.y(), pt.x() + 1, pt.y());
                                new_points.insert(Point::new(pt.x() + 1, pt.y()));
                            }
                        }
                        (true, false) => {
                            board.swap(pt.x(), pt.y(), pt.x() - 1, pt.y());
                            new_points.insert(Point::new(pt.x() - 1, pt.y()));
                        }
                        (false, true) => {
                            board.swap(pt.x(), pt.y(), pt.x() + 1, pt.y());
                            new_points.insert(Point::new(pt.x() + 1, pt.y()));
                        }
                        (false, false) => {
                            new_points.insert(pt.clone());
                        }
                    }
                }
            }

            new_blobs.insert(index, (new_points, did_move_down));

            for (_, (points, did_move_down)) in new_blobs.iter() {
                if !did_move_down {
                    // No single droplet from this blob moved down, try move up.
                    let top_row = points.first().unwrap().y();
                    let top_points: Vec<_> = points.iter().filter(|pt| pt.y() == top_row).collect();

                    let destination_candidates: Vec<_> = points
                        .iter()
                        .rev()
                        .filter(|pt| {
                            let pt_up = board.tiles().at(pt.x(), pt.y() - 1);
                            pt_up.is_air() && pt.y() != top_row
                        })
                        .map(|pt| Point::new(pt.x(), pt.y() - 1))
                        .collect();

                    if !destination_candidates.is_empty() {
                        let lowest_row = destination_candidates.first().unwrap().y();
                        let lowest_candidates: Vec<_> = destination_candidates
                            .iter()
                            .filter(|pt| pt.y() == lowest_row)
                            .collect();

                        if !top_points.is_empty() && !lowest_candidates.is_empty() {
                            let top_point = top_points.choose(&mut rng).unwrap();
                            let destination_point = lowest_candidates.choose(&mut rng).unwrap();

                            board.swap(
                                top_point.x(),
                                top_point.y(),
                                destination_point.x(),
                                destination_point.y(),
                            );
                        }
                    }
                }
            }
        }

        // TODO: It's super inefficient to re-detect blobs each tick.
        // Split and merge blobs as they move.
        let blob_detector = BlobDetector::new(&board);
        let blobs = blob_detector.detect();

        Engine { board, blobs, rng }
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
