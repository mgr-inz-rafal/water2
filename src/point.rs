use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub(crate) struct Point((usize, usize));

impl Point {
    pub(crate) fn new(x: usize, y: usize) -> Self {
        Self((x, y))
    }

    pub(crate) fn x(&self) -> usize {
        self.inner().0
    }

    pub(crate) fn y(&self) -> usize {
        self.inner().1
    }

    fn inner(&self) -> &(usize, usize) {
        &self.0
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.x().cmp(&other.x()), self.y().cmp(&other.y())) {
            (Ordering::Equal, Ordering::Equal) => Ordering::Equal,
            (Ordering::Less, Ordering::Equal) | (_, Ordering::Less) => Ordering::Less,
            (Ordering::Greater, Ordering::Equal) | (_, Ordering::Greater) => Ordering::Greater,
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Point {}
