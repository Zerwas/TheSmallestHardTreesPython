use std::hash::Hash;
use std::str::FromStr;

use itertools::Itertools;

use crate::digraph::classes::Buildable;

use super::Rooted;

/// An orientation of a tree which has a single vertex of
/// degree 3 and otherwise only vertices of degree 2 and 1.
///
/// Each String in the Vector represents a path that leaves the
/// vertex of degree 3 of the triad. `'0'` stands for
/// forward edge and `'1'` for backward edge.
///
#[derive(Clone, Hash, Default, PartialEq, Eq)]
pub struct Triad(String, String, String);

impl Triad {
    pub const fn iter(&self) -> Iter<'_> {
        Iter {
            triad: self,
            index: 0,
        }
    }
}

pub struct Iter<'a> {
    triad: &'a Triad,
    index: usize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<&'a str> {
        let result = match self.index {
            0 => &self.triad.0,
            1 => &self.triad.1,
            2 => &self.triad.2,
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}

impl FromIterator<String> for Triad {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        let arm1 = iter.next().expect(""); // TODO hack
        let arm2 = iter.next().expect("");
        let arm3 = iter.next().expect("");
        Triad(arm1, arm2, arm3)
    }
}

