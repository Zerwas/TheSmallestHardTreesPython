//! # Tripolys
//!
//! `tripolys` is a program for generating triads and checking for polymorphisms.
//!
//! For a given digraph H the complexity of the constraint satisfaction problem
//! for H, also called CSP(H), only depends on the set of polymorphisms of H.
//! The program aims to study the structure of oriented trees with CSPs of
//! varying complexity.
//! To do this we focus on the case where H is a triad, e.g., an orientation of
//! a tree which has a single vertex of degree 3 and otherwise only vertices of
//! degree 2 and 1.

#![warn(clippy::correctness, clippy::pedantic, clippy::nursery)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::redundant_closure_for_method_calls)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::use_self)]
#![allow(clippy::missing_errors_doc)]

#[macro_use]
extern crate itertools;

pub mod algebra;
pub mod colouring;
pub mod digraph;
pub mod tree;
