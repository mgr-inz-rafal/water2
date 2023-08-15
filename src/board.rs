use crate::tiles::Tiles;

pub(crate) struct Board {
    width: usize,
    height: usize,
    tiles: Tiles,
}

impl Board {
    pub(crate) fn swap(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let source = self.tiles.at(x1, y1).clone();
        let target = self.tiles.at(x2, y2).clone();
        self.tiles.set_at(x1, y1, target);
        self.tiles.set_at(x2, y2, source);
    }

    pub(crate) fn tiles(&self) -> &Tiles {
        &self.tiles
    }

    pub(crate) fn width(&self) -> usize {
        self.width
    }

    pub(crate) fn height(&self) -> usize {
        self.height
    }

    pub(crate) fn new_test_1() -> Self {
        const WIDTH: usize = 10;
        const TEST_1: &str = "##########\
                              #.oo.o...#\
                              #.oooo...#\
                              #..o.....#\
                              #........#\
                              #....o...#\
                              #...ooo..#\
                              #....o...#\
                              #........#\
                              #........#\
                              #........#\
                              #........#\
                              #........#\
                              #........#\
                              #........#\
                              ##########";

        Self {
            width: WIDTH,
            height: 16,
            tiles: Tiles::from_str(TEST_1, WIDTH),
        }
    }
}
