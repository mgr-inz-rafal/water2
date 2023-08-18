// TODO: Clean-up unwraps

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum Tile {
    Rock,
    Water,
    Air,
}

impl Tile {
    pub(crate) fn is_air(&self) -> bool {
        self == &Tile::Air
    }
}

// TODO: Better use single Vec in order to enable faster swapping of items
#[derive(Clone, Debug)]
pub(crate) struct Tiles {
    width: usize,
    height: usize,
    tiles: Vec<Vec<Tile>>,
}

impl Tiles {
    pub(crate) fn from_str(s: &str, width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            tiles: s
                .chars()
                .collect::<Vec<_>>()
                .chunks(width)
                .map(|chunk| {
                    chunk
                        .iter()
                        .map(|c| match c {
                            '#' => Tile::Rock,
                            '.' => Tile::Air,
                            'o' => Tile::Water,
                            _ => panic!("unknown tile: {c}"),
                        })
                        .collect()
                })
                .collect(),
        }
    }

    pub(crate) fn empty(width: usize, height: usize) -> Self {
        let row = vec![Tile::Air; width];
        Self {
            width,
            height,
            tiles: vec![row; height],
        }
    }

    pub(crate) fn at(&self, x: usize, y: usize) -> Option<&Tile> {
        self.within_limits(x, y).then(|| &self.tiles[y][x])
    }

    pub(crate) fn set_at(&mut self, x: usize, y: usize, tile: Tile) {
        if self.within_limits(x, y) {
            *self.tiles.get_mut(y).unwrap().get_mut(x).unwrap() = tile;
        }
    }

    fn within_limits(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }
}
