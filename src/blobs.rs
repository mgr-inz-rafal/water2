use std::collections::{BTreeMap, BTreeSet};

use crate::point::Point;

pub(crate) type Blob = BTreeSet<Point>;
pub(crate) type Blobs = BTreeMap<usize, Blob>;
