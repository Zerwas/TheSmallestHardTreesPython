use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::tree::{is_core, is_rooted_core, Node};
use itertools::Itertools;
use rayon::prelude::*;

pub struct Config {
    /// Maximal degree of each node
    pub max_arity: usize,
    /// Constrain to cores
    pub core: bool,
    /// Only enumerate triads
    pub triads: bool,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            max_arity: 2,
            core: true,
            triads: false,
        }
    }
}

pub struct Generator {
    rooted_trees: Vec<Vec<Arc<Node>>>,
    config: Config,
}

impl Generator {
    pub fn new() -> Generator {
        Generator::with_config(Config::default())
    }

    pub fn with_config(config: Config) -> Generator {
        Generator {
            rooted_trees: vec![vec![Arc::new(Node::leaf())]],
            config,
        }
    }

    /// Returns all unique sets of `n` rooted trees whose number of nodes sum
    /// up to `total`. The trees are sorted by their number of nodes in
    /// ascending order.
    fn rooted_trees(&self, total: usize, n: usize) -> impl Iterator<Item = Vec<Arc<Node>>> + '_ {
        addends(total, n)
            .into_iter()
            .flat_map(|vec| {
                vec.into_iter()
                    .map(|idx| self.rooted_trees[idx - 1].iter().cloned())
                    .multi_cartesian_product()
            })
            .filter(|vec| vec.windows(2).all(|w| w[0] <= w[1])) // excludes permutations
    }

    fn forward(&mut self, order: usize) {
        for step in self.rooted_trees.len() + 1..order {
            let mut trees = Vec::<Vec<Arc<Node>>>::new();
            let mut rooted_core_time = Duration::from_secs(0);
            let mut num = 0;

            for arity in 1..self.config.max_arity {
                let treenagers = self
                    .rooted_trees(step - 1, arity)
                    .par_bridge()
                    .flat_map(|children| connect_by_vertex(&children))
                    .collect::<Vec<_>>();

                num += treenagers.len();
                let start = Instant::now();
                let filtered = treenagers
                    .into_par_iter()
                    .filter_map(|child| {
                        if !self.config.core || is_rooted_core(&child) {
                            Some(Arc::new(child))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                rooted_core_time += start.elapsed();
                trees.push(filtered);
            }
            println!("    - rcc_time: {:?}, #rcc: {}", rooted_core_time, num);

            let trees = trees.into_iter().flatten().collect_vec();
            if self.config.core {
                println!(
                    "    - Generated {} rooted core trees with {} nodes",
                    trees.len(),
                    step
                );
            }
            self.rooted_trees.push(trees);
        }
    }

    fn unique_trees(&self, order: usize) -> Vec<Node> {
        // A tree with centre is a rooted tree where at least two children of the root
        // have height d−1
        let centered = (2..=self.config.max_arity)
            .flat_map(move |arity| {
                self.rooted_trees(order - 1, arity)
                    .flat_map(|children| connect_by_vertex(&children))
            })
            .filter(|tree| tree.is_centered());

        // A bicentered tree is formed by taking two rooted trees of equal
        // height and adding an edge between their roots
        let bicentered = self
            .rooted_trees(order, 2)
            .filter(|c| c[0].height == c[1].height)
            .flat_map(|v| connect_by_edge(&v[0], &v[1]));

        centered.chain(bicentered).collect()
    }

    fn unique_triads(&self, order: usize) -> Vec<Node> {
        self.rooted_trees(order - 1, 3)
            .filter(|arms| {
                arms.iter()
                    .all(|root| root.max_arity < 3 && root.arity() < 2)
            })
            .flat_map(|arms| connect_by_vertex(&arms))
            .collect()
    }

    pub fn resume(&mut self, order: usize) -> Result<Vec<Node>, TreenumError> {
        assert!(order > 0, "Number of nodes must be greater than 0");

        if self.config.triads && order < 4 {
            return Err(TreenumError::TriadNumNodes(order));
        }
        if order < 1 {
            return Err(TreenumError::TreeNumNodes(order));
        }

        if order == 1 {
            return Ok(vec![Node::leaf()]);
        }

        self.forward(order);

        let items = if self.config.triads {
            self.unique_triads(order)
        } else {
            self.unique_trees(order)
        };

        let num_items = items.len();
        let mut cc_time = Duration::from_secs(0);
        let mut filter = |t: &Node| {
            let start = Instant::now();
            let p = is_core(t);
            cc_time += start.elapsed();
            p
        };
        let filtered = items
            .into_iter()
            .filter(|t| !self.config.core || filter(t))
            .collect::<Vec<_>>();
        if self.config.core {
            println!(
                "    - cc_time: {:?}, #cc: {}",
                cc_time / num_items as u32,
                num_items
            );
        }
        if self.config.core {
            println!(
                "    - Generated {} core trees with {} nodes",
                filtered.len(),
                order
            );
        }

        Ok(filtered)
    }
}

/// Connects two rooted trees by adding an edge between their roots.
/// `child` becomes the rightmost child of `tree`.
///
/// If the two trees happen to be the same, we only add an edge once.
fn connect_by_edge(tree: &Arc<Node>, child: &Arc<Node>) -> Vec<Node> {
    let connect = |dir| {
        tree.iter()
            .chain(std::iter::once((child.clone(), dir)))
            .collect()
    };

    if *tree == *child {
        [true].iter().map(|&dir| connect(dir)).collect()
    } else {
        [true, false].iter().map(|&dir| connect(dir)).collect()
    }
}

/// Connects an arbitrary number of rooted trees by adding a new vertex that is
/// adjacent to each of their roots.
fn connect_by_vertex(children: &[Arc<Node>]) -> Vec<Node> {
    (0..children.len())
        .map(|_| [true, false].into_iter())
        .multi_cartesian_product()
        .map(|edges| children.iter().cloned().zip(edges.into_iter()))
        .filter(|v| v.clone().tuple_windows().all(|(a, b)| a <= b)) // excludes permutations
        .map(|t| t.collect())
        .collect()
}

/// Returns every set of `n` integers that sum up to `sum` sorted in ascending order.
///
/// E.g. `addends(6, 3)` yields [[1, 1, 4], [1, 2, 3], [2, 2, 2]].
fn addends(sum: usize, n: usize) -> Vec<Vec<usize>> {
    fn inner(sum: usize, n: usize, last: usize) -> Vec<Vec<usize>> {
        if n == 0 {
            return vec![vec![]];
        }

        let mut result = Vec::new();
        let start = ((sum) as f32 / n as f32).ceil() as usize;
        let end = std::cmp::min(sum - n + 1, last);

        for i in start..=end {
            for mut child in inner(sum - i, n - 1, i) {
                child.push(i);
                result.push(child);
            }
        }

        result
    }

    if n > sum {
        return vec![];
    }

    inner(sum, n, usize::MAX)
}

#[derive(Debug)]
pub enum TreenumError {
    /// The number of nodes is too small
    TreeNumNodes(usize),
    /// The number of nodes is too small
    TriadNumNodes(usize),
}

impl std::fmt::Display for TreenumError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            TreenumError::TreeNumNodes(n) => write!(f, "There is no tree with {} nodes", n),
            TreenumError::TriadNumNodes(n) => write!(f, "There is no triad with {} nodes", n),
        }
    }
}

impl std::error::Error for TreenumError {}
