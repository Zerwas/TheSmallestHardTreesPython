//! A mapping from G to H such that (h(u), h(v)) ∈ E(H) whenever (u, v) ∈ E(G).
//!
//! If such a homomorphism exists between G and H we say that G homomorphically
//! maps to H, and write G → H. Two directed graphs G and H are homomorphically
//! equivalent if G → H and H → G.  A homomorphism from G to H is sometimes also
//! called an H-colouring of G. This termi- nology originates from the
//! observation that H-colourings generalise classical colourings in the sense
//! that a graph is n-colourable if and only if it has a Kn -colouring. Graph
//! n-colorability is not the only natural graph property that can be described
//! in terms of homomorphisms:

mod problem;

pub use problem::ColouringProblem;
