use std::thread::Thread;

use rand::{rngs::ThreadRng, Rng};

use crate::{
    blob_detector::BlobDetector,
    blobs::Blobs,
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

        for (_, points) in blobs {
            for pt in points.iter().rev() {
                // Try move down
                let dest_pt = Point::new(pt.x(), pt.y() + 1);
                if board.tiles().at(dest_pt.x(), dest_pt.y()).is_air() {
                    board.swap(pt.x(), pt.y(), pt.x(), pt.y() + 1);
                } else {
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
                                board.swap(pt.x(), pt.y(), pt.x() - 1, pt.y())
                            } else {
                                board.swap(pt.x(), pt.y(), pt.x() + 1, pt.y())
                            }
                        }
                        (true, false) => board.swap(pt.x(), pt.y(), pt.x() - 1, pt.y()),
                        (false, true) => board.swap(pt.x(), pt.y(), pt.x() + 1, pt.y()),
                        (false, false) => (),
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
