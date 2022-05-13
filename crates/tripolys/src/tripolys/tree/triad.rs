use std::hash::Hash;
use std::str::FromStr;

use itertools::Itertools;

use crate::digraph::{AdjMap, ToGraph};
use crate::tree::{Rooted, Tree};

use super::Balanced;

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

impl Tree for Triad {}

impl ToGraph for Triad {
    type V = u32;

    fn to_graph(&self) -> AdjMap<Self::V> {
        let mut graph = AdjMap::<u32>::new();
        let mut id = 1;
        graph.add_vertex(0);

        for arm in self.iter() {
            for (i, c) in arm.chars().enumerate() {
                graph.add_vertex(id);

                match c {
                    '1' => {
                        if i == 0 {
                            graph.add_edge(&id, &0);
                        } else {
                            graph.add_edge(&id, &(id - 1));
                        }
                    }
                    '0' => {
                        if i == 0 {
                            graph.add_edge(&0, &id);
                        } else {
                            graph.add_edge(&(id - 1), &id);
                        }
                    }
                    _ => {} // Impossible
                }
                id += 1;
            }
        }
        graph
    }
}

impl From<&Triad> for AdjMap<u32> {
    fn from(triad: &Triad) -> Self {
        triad.to_graph()
    }
}

impl From<Triad> for AdjMap<u32> {
    fn from(triad: Triad) -> Self {
        (&triad).to_graph()
    }
}

impl Rooted for Triad {}

impl Balanced for Triad {
    fn level(&self, id: &u32) -> Option<u32> {
        let mut count = *id as usize;

        for arm in self.iter() {
            if count <= arm.len() {
                let mut rank = 0;
                let mut chars = arm.chars();

                while count > 0 {
                    let c = chars.next().unwrap();

                    if c == '0' {
                        rank += 1;
                    } else {
                        rank -= 1;
                    }
                    count -= 1;
                }
                return Some(rank);
            }
            count -= arm.len();
        }
        None
    }
}

impl std::fmt::Display for Triad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        for (i, arm) in self.iter().enumerate() {
            if i > 0 {
                s.push(',');
            }
            s.push_str(arm);
        }
        write!(f, "{}", s)
    }
}

impl FromStr for Triad {
    type Err = ParseTriadError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let arms = s
            .split(&[',', '_'][..])
            .map(|x| x.to_string())
            .collect_vec();

        if arms.len() != 3 {
            return Err(ParseTriadError::ArmQuantity);
        }

        for arm in &arms {
            for c in arm.chars() {
                if c != '0' && c != '1' {
                    return Err(ParseTriadError::InvalidCharacter(c));
                }
            }
        }
        Ok(arms.into_iter().collect())
    }
}

#[derive(Debug)]
pub enum ParseTriadError {
    ArmQuantity,
    InvalidCharacter(char),
}

impl std::fmt::Display for ParseTriadError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseTriadError::ArmQuantity => write!(f, "A triad must have exactly 3 arms!"),
            ParseTriadError::InvalidCharacter(c) => write!(f, "Could not parse: {}", c),
        }
    }
}

impl std::error::Error for ParseTriadError {
    fn description(&self) -> &str {
        match self {
            ParseTriadError::ArmQuantity => "A triad can have at most 3 arms!",
            ParseTriadError::InvalidCharacter(_) => "Only 0 and 1 are allowed in triad arms!",
        }
    }
}
