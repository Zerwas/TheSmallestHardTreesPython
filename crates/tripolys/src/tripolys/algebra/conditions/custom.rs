use std::error::Error;
use std::fmt;
use std::str::FromStr;

use super::{Arity, Linear, Partition, Set, Tuple};

pub struct Condition {
    operations: Vec<Arity>,
}

impl Linear for Condition {
    fn arities(&self) -> Vec<Arity> {
        self.operations.clone()
    }

    fn partition<V>(&self, vertices: Set<u32>) -> Partition<(usize, Tuple<u32>)> {
        todo!()
    }
}

impl FromStr for Condition {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s)
    }
}

// E.g. fxyy=fyyx,fxxy=gyxx
fn parse(s: &str) -> Result<Condition, ParseError> {
    let identities = s.split(',');
    let operations = Vec::new();

    Ok(Condition { operations })
}

#[derive(Clone, Debug)]
pub enum ParseError {
    NonMatchingArities(String),
    InvalidCharacter(String),
}

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidCharacter(s) | Self::NonMatchingArities(s) => s.fmt(f),
        }
    }
}
