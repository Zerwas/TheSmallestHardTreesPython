//! A set of abstractions to define a CSP.

/// An interface to define a Constraint Satisfaction Problem.
///
/// The Constraint Satisfaction Problem (CSP) provides a common framework for
/// expressing a wide range of both theoretical and real-life combinatorial
/// problems. Roughly, these are problems where one is given a collection of
/// `Constraints` on overlapping sets of variables and the goal is to assign
/// values to the variables so as to satisfy the constraints.
pub trait Problem: Domains + Constraints {}

/// Defines the domains of a CSP.
pub trait Domains {
    /// The size of the problem, that is, its number of variables.
    ///
    /// Each variable is assumed to be identified with an integer
    /// ranging from 0 until `problem.size()`.
    fn size(&self) -> usize;

    /// Returns a finite set of values that the variable `x` can take.
    fn domain(&self, x: Variable) -> Vec<Value>;
}

/// A set of binary constraints.
pub trait Constraints {
    /// Returns every pair of variables
    fn arcs(&self) -> Vec<(Variable, Variable)>;

    /// Returns true, if the two assignments are consistent with each other, false otherwise.
    fn check(&self, ai: (Variable, Value), aj: (Variable, Value)) -> bool;
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Value(pub usize);

impl std::ops::Deref for Value {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Variable(pub usize);

impl std::ops::Deref for Variable {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub type Solution = Vec<Value>;
