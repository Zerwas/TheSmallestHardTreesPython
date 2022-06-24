//! # Tripolys
//!
//! `tripolys` is a program for checking homomorphisms and testing polymorphism
//! conditions of directed graphs. It also implements an algorithm to generate
//! orientations of trees, and core orientations of trees.

#![warn(clippy::correctness, clippy::pedantic, clippy::nursery)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::redundant_closure_for_method_calls)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::use_self)]
#![allow(clippy::missing_errors_doc)]

pub mod algebra;
pub mod digraph;
pub mod hcoloring;
pub mod tree;
