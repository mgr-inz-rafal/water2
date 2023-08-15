use crate::{board::Board, Blobs, point::Point, blob_detector::BlobDetector, HasBlobs, HasBoard, Paintable};

pub(crate) struct Engine {
    board: Board,
    blobs: Blobs,
}

impl Engine {
    pub(crate) fn new(board: Board, blobs: Blobs) -> Self {
        Self { board, blobs }
    }

    fn board(&self) -> &Board {
        &self.board
    }

    fn blobs(&self) -> &Blobs {
        &self.blobs
    }

    pub(crate) fn tick(self) -> Engine {
        let Engine { mut board, blobs } = self;

        for (_, points) in blobs {
            for pt in points.iter().rev() {
                let dest_pt = Point::new(pt.x(), pt.y() + 1);
                if board.tiles().at(dest_pt.x(), dest_pt.y()).is_air() {
                    board.swap(pt.x(), pt.y(), pt.x(), pt.y() + 1);
                }
            }
        }

        // TODO: It's super inefficient to re-detect blobs each tick.
        // Split and merge blobs as they move.
        let blob_detector = BlobDetector::new(&board);
        let blobs = blob_detector.detect();

        Engine { board, blobs }
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
