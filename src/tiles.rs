// TODO: Clean-up unwraps

#[derive(Debug, Clone, PartialEq, Eq)]
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

pub(crate) struct Tiles(Vec<Vec<Tile>>);

impl Tiles {
    pub(crate) fn from_str(s: &str, width: usize) -> Self {
        Self(
            s.chars()
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
        )
    }

    pub(crate) fn at(&self, x: usize, y: usize) -> &Tile {
        self.0.get(y).unwrap().get(x).unwrap()
    }

    pub(crate) fn set_at(&mut self, x: usize, y: usize, tile: Tile) {
        *self.0.get_mut(y).unwrap().get_mut(x).unwrap() = tile;
    }
}
