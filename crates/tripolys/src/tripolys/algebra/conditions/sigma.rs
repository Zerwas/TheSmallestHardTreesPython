// use crate::digraph::traits::Vertices;

// use super::{HeightOne, Linear, Partition, Precolor};

// /// σ(x,y) = σ(y,x)
// #[derive(Clone, Copy, Debug)]
// pub struct Sigma(pub usize);

// impl Precolor for Sigma {}

// impl Linear for Sigma {
//     fn arities(&self) -> Vec<super::Arity> {
//         todo!()
//     }

//     fn partition<G>(&self, vertices: &G) -> Partition<(usize, Vec<G::Vertex>)>
//     where
//         for<'a> G: Vertices<'a>,
//     {
//         todo!()
//     }
// }

// /// σ(x,y) = σ(y,x)
// #[derive(Clone, Copy, Debug)]
// pub struct Sigma2;

// impl<V> Linear<V> for Sigma2 {
//     fn to_contract(&self, vertices: &[V]) -> Vec<Vec<Vec<V>>> {
//         let mut vec = Vec::new();

//         for i in 0..vertices.len() {
//             for j in i + 1..vertices.len() {
//                 vec.push(vec![
//                     vec![vertices[i], vertices[j]],
//                     vec![vertices[j], vertices[i]],
//                 ]);
//             }
//         }
//         vec
//     }
// }

// impl HeightOne for Sigma2 {
//     // f(x,y) = f(y,x)
//     fn eq_under<T: PartialEq>(v: &[T], w: &[T]) -> bool {
//         debug_assert!(v.len() == 2 && w.len() == 2, "length must be equal to 2!");
//         v[0] == w[1] && v[1] == w[0]
//     }
// }

// /// σ(x,y,z) = σ(z,x,y)
// #[derive(Clone, Copy, Debug)]
// pub struct Sigma3;

// impl<V> Linear<V> for Sigma3 {
//     fn to_contract(&self, vertices: &[V]) -> Vec<Vec<Vec<V>>> {
//         let mut vec = Vec::new();
//         // TODO handle 2-tuples
//         for (&x, &y) in vertices.iter().tuple_combinations::<(_, _)>() {
//             // vec.push(vec![vec![x, y, y], vec![y, x, y], vec![y, y, x]]);
//         }
//         for (&x, &y, &z) in vertices.iter().tuple_combinations::<(_, _, _)>() {
//             vec.push(vec![vec![x, y, z], vec![z, x, y]]);
//         }
//         vec
//     }
// }

// impl HeightOne for Sigma {
//     fn eq_under<T: PartialEq>(v: &[T], w: &[T]) -> bool {
//         todo!()
//         // debug_assert!(v.len() == 3 && w.len() == 3, "length must be equal to 2!");
//         // v[0] == w[1] && v[1] == w[2] && v[2] == w[0] || w[0] == v[1] && w[1] == v[2] && w[2] == v[0]
//     }
// }
