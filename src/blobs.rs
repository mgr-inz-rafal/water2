use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use itertools::Itertools;
use rand::{rngs::ThreadRng, Rng};

use crate::point::{self, Point};

pub(crate) type Blobs = BTreeMap<usize, Blob>;

#[derive(Clone, Default, Debug, PartialEq)]
pub(crate) struct Blob {
    points: BTreeSet<Point>,
}

impl Blob {
    pub(crate) fn new(points: BTreeSet<Point>) -> Self {
        Self { points }
    }

    pub(crate) fn points(&self) -> &BTreeSet<Point> {
        &self.points
    }

    pub(crate) fn points_mut(&mut self) -> &mut BTreeSet<Point> {
        &mut self.points
    }
}

impl<'a> IntoIterator for &'a Blob {
    type Item = &'a Point;
    type IntoIter = PointIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        PointIterator::new(&self.points)
    }
}

#[derive(Debug)]
pub(crate) struct PointIterator<'a> {
    points: HashMap<usize, BTreeSet<&'a Point>>,
    keys: BTreeSet<usize>,
    rng: ThreadRng,
}

impl<'a> PointIterator<'a> {
    pub(crate) fn new(points: &'a BTreeSet<Point>) -> Self {
        let grouped_points = points
            .into_iter()
            .into_grouping_map_by(|pt| pt.y())
            .collect::<BTreeSet<_>>(); // TODO: HashSet?

        Self {
            keys: grouped_points.keys().cloned().collect(),
            points: grouped_points,
            rng: rand::thread_rng(),
        }
    }
}

impl<'a> Iterator for PointIterator<'a> {
    type Item = &'a Point;
    fn next(&mut self) -> Option<Self::Item> {
        if self.keys.is_empty() {
            return None;
        }

        if let Some(lowest_row) = self.keys.last().copied() {
            let points_in_row = self.points.get(&lowest_row).unwrap().len();
            if points_in_row == 0 {
                self.keys.remove(&lowest_row);
            }
        }

        if let Some(lowest_row) = self.keys.last() {
            // TODO: Check if this gonna be faster with Vec, which is IndexMut and can work with choose()
            // Currently we think we save time by having `&'a Point` instead of owned one, but we probably loose more
            // time with these iter().skip() and clone()
            let points_in_row = self.points.get_mut(lowest_row).unwrap();
            let selected_point = points_in_row
                .iter()
                .skip(self.rng.gen_range(0..points_in_row.len()))
                .next()
                .unwrap()
                .clone();
            points_in_row.remove(selected_point);
            return Some(selected_point);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::point::Point;

    use super::Blob;

    impl Blob {
        fn from_iter(points: impl IntoIterator<Item = Point>) -> Self {
            Self {
                points: points.into_iter().collect(),
            }
        }
    }

    #[test]
    fn should_correctly_iter_points() {
        // This algorithm uses randomness, hence, it may happen that it'll iterate points in the sorted order, which
        // is not preferred, but still acceptable. Therefore, if we detect such condition we try a few more times to
        // verify, that eventually the points are yielded in random order.
        const ALLOWED_FALSE_POSITIVES: usize = 10;

        // For the sake of simulation our preferred way is to iterate points row by row from the lowest row.
        // However, the points in any particular row should be randomized, so water doesn't exhibit a bias
        // to go towards any particular direction.
        let not_expected = vec![
            Point::new(4, 4),
            Point::new(3, 4),
            Point::new(3, 3),
            Point::new(3, 2),
            Point::new(2, 2),
            Point::new(1, 2),
        ];

        for _ in 0..ALLOWED_FALSE_POSITIVES {
            let blob = Blob::from_iter([
                Point::new(3, 4),
                Point::new(4, 4),
                Point::new(1, 2),
                Point::new(3, 2),
                Point::new(2, 2),
                Point::new(3, 3),
            ]);

            let actual = blob.into_iter().cloned().collect::<Vec<_>>();

            dbg!(&actual);
            dbg!(&not_expected);

            if actual != not_expected {
                return;
            }
        }

        assert!(false);
    }
}
