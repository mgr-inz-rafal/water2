#[derive(Debug, Clone)]
enum Tile {
    Rock,
    Water,
    Air,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Rock => write!(f, "#"),
            Tile::Water => write!(f, "o"),
            Tile::Air => write!(f, "."),
        }
    }
}

struct Tiles(Vec<Vec<Tile>>);

impl Tiles {
    fn from_str(s: &str, width: usize) -> Self {
        Self(
            s.chars()
                .into_iter()
                .collect::<Vec<_>>()
                .chunks(width)
                .into_iter()
                .map(|chunk| {
                    chunk
                        .into_iter()
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
}

struct Board {
    width: usize,
    height: usize,
    tiles: Tiles,
}

impl Board {
    fn at(&self, x: usize, y: usize) -> &Tile {
        self.tiles.0.get(y).unwrap().get(x).unwrap()
    }

    fn new_test_1() -> Self {
        const WIDTH: usize = 10;
        const TEST_1: &str = "##########\
                              #.oo.o...#\
                              #.oooo...#\
                              #..o.....#\
                              #........#\
                              #........#\
                              #........#\
                              #........#\
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

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.at(x, y));
            }
            writeln!(f);
        }
        Ok(())
    }
}

fn main() {
    let board = Board::new_test_1();
    println!("{board}");
}
