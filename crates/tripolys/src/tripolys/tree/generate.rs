use std::sync::Arc;

use crate::tree::{is_core_tree, is_rooted_core_tree, Tree};
use itertools::Itertools;
use rayon::prelude::*;

/// Returns every set of `k` integers that sum up to `n` sorted in ascending order.
///
/// E.g. `calculate_addends(6, 3)` yields [[1, 1, 4], [1, 2, 3], [2, 2, 2]].
fn addends(n: usize, k: usize) -> Vec<Vec<usize>> {
    fn inner(pos: usize, left: usize, k: usize, last: usize) -> Vec<Vec<usize>> {
        // Base Case
        if pos == k {
            if left == 0 {
                return vec![vec![]];
            } else {
                return vec![];
            }
        }

        if left == 0 {
            return vec![];
        }

        let mut addends = Vec::new();

        for i in 1..=left {
            if i > last {
                break;
            }
            for mut sub in inner(pos + 1, left - i, k, i) {
                sub.push(i);
                addends.push(sub);
            }
        }

        addends
    }

    inner(0, n, k, n)
}

#[derive(Clone, Copy)]
pub struct TreeGenSettings {
    /// Number of vertices to start at
    pub start: usize,
    /// Number of vertices to end at (inclusive)
    pub end: usize,
    /// Maximal degree of each node
    pub max_arity: usize,
    /// Constrain to cores
    pub core: bool,
    /// Only enumerate triads
    pub triad: bool,
    /// Record statistics
    pub stats: Option<TreeGenStats>,
}

/// Statistics from tree generation
#[derive(Clone, Copy, Default)]
pub struct TreeGenStats {
    /// Time for rooted core checks
    pub rcc_time: f32,
    /// Number of generated rooted trees
    pub num_rcc: usize,
    /// Time for core checks
    pub cc_time: f32,
    /// Number of generated trees
    pub num_cc: usize,
}

pub struct TreeGenerator {
    rooted_trees: Vec<Vec<Arc<Tree>>>,
    settings: TreeGenSettings,
    nvertices: usize,
}

impl TreeGenerator {
    pub fn new(config: TreeGenSettings) -> TreeGenerator {
        assert!(
            !(config.triad && config.start < 4),
            "There is no triad with {} nodes",
            config.start
        );
        assert!(
            config.start != 0,
            "There is no tree with {} nodes",
            config.start
        );

        TreeGenerator {
            rooted_trees: vec![vec![Arc::new(Tree::leaf())]],
            nvertices: config.start,
            settings: config,
        }
    }

    /// Returns all unique sets of `k` rooted trees whose number of nodes sum up
    /// to `n`. The trees are sorted by their number of nodes in ascending
    /// order.
    fn rooted_trees(&self, n: usize, k: usize) -> impl Iterator<Item = Vec<Arc<Tree>>> + '_ {
        addends(n, k)
            .into_iter()
            .flat_map(|set| {
                set.into_iter()
                    .map(|nvertices| self.rooted_trees[nvertices - 1].iter().cloned())
                    .multi_cartesian_product()
            })
            .filter(|vec| vec.windows(2).all(|w| w[0] <= w[1])) // excludes permutations
    }

    fn generate_rooted_trees(&mut self) {
        for step in self.rooted_trees.len() + 1..self.nvertices {
            let mut trees = Vec::<Vec<Arc<Tree>>>::new();
            // let mut rcc_time = time::OffsetDateTime::now_utc();
            let mut num_rcc = 0;

            for arity in 1..self.settings.max_arity {
                let treenagers = self
                    .rooted_trees(step - 1, arity)
                    .par_bridge()
                    .flat_map(|children| connect_by_vertex(&children))
                    .collect::<Vec<_>>();

                num_rcc += treenagers.len();
                // let start = Instant::now();
                let filtered = treenagers
                    .into_par_iter()
                    .filter_map(|child| {
                        if !self.settings.core || is_rooted_core_tree(&child) {
                            Some(Arc::new(child))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                // rc_time += start.elapsed();
                trees.push(filtered);
            }
            let trees = trees.into_iter().flatten().collect_vec();

            if let Some(mut stats) = self.settings.stats {
                // stats.rcc_time = rcc_time; TODO
                stats.num_rcc = num_rcc;
            }
            self.rooted_trees.push(trees);
        }
    }

    fn generate_trees(&mut self) -> Vec<Tree> {
        self.generate_rooted_trees();

        if self.settings.triad {
            self.rooted_trees(self.nvertices - 1, 3)
                .filter(|arms| arms.iter().all(|arm| arm.is_path()))
                .flat_map(|arms| connect_by_vertex(&arms))
                .filter(|tree| tree.is_triad())
                .collect()
        } else {
            // A tree with centre is a rooted tree where at least two children of the root
            // have height d−1
            let centered = (2..=self.settings.max_arity)
                .flat_map(|arity| {
                    self.rooted_trees(self.nvertices - 1, arity)
                        .flat_map(|children| connect_by_vertex(&children))
                })
                .filter(|tree| tree.is_centered());

            // A bicentered tree is formed by taking two rooted trees of equal
            // height and adding an edge between their roots
            let bicentered = self
                .rooted_trees(self.nvertices, 2)
                .filter(|c| c[0].height == c[1].height)
                .flat_map(|v| connect_by_edge(&v[0], &v[1]));

            centered.chain(bicentered).collect()
        }
    }

    pub fn generate(&mut self) -> Vec<Tree> {
        if self.nvertices == 1 {
            return vec![Tree::leaf()];
        }

        let trees = self.generate_trees();

        let num_cc = trees.len();
        let filtered = trees
            .into_par_iter()
            .filter(|t| !self.settings.core || is_core_tree(t))
            .collect::<Vec<_>>();
        if let Some(mut stats) = self.settings.stats {
            // stats.cc_time = cc_time; TODO
            stats.num_cc = num_cc;
        }
        self.nvertices += 1;

        filtered
    }
}

/// Connects two rooted trees by adding an edge between their roots.
/// `child` becomes the rightmost child of `tree`.
///
/// If the two trees happen to be the same, we only add an edge once.
fn connect_by_edge(tree: &Arc<Tree>, child: &Arc<Tree>) -> Vec<Tree> {
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
fn connect_by_vertex(children: &[Arc<Tree>]) -> Vec<Tree> {
    (0..children.len())
        .map(|_| [true, false])
        .multi_cartesian_product()
        .map(|edges| children.iter().cloned().zip(edges))
        .filter(|v| v.clone().tuple_windows().all(|(a, b)| a <= b)) // excludes permutations
        .map(|t| t.collect())
        .collect()
}
