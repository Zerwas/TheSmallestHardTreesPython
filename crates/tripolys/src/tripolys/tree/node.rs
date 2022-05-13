use std::cmp::max;
use std::str::FromStr;
use std::sync::Arc;

use itertools::Itertools;

use crate::digraph::{AdjMap, ToGraph};

use super::{Balanced, Rooted, Tree, Triad};

/// A recursive tree data-structure
#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct Node {
    pub num_nodes: usize,
    pub height: usize,
    pub max_arity: usize,
    children: Vec<(Arc<Node>, bool)>,
}

impl Node {
    /// Returns a tree with only one node, also referred to as 'leaf'.
    pub const fn leaf() -> Node {
        Node {
            num_nodes: 1,
            height: 0,
            max_arity: 0,
            children: Vec::new(),
        }
    }

    /// Returns the number of childern of the root vertex, also referred to as
    /// its 'arity'.
    pub fn arity(&self) -> usize {
        self.children.len()
    }

    /// Returns `true` if the tree is centered.
    ///
    /// A centered tree is a rooted tree where at least two children of the root
    /// have height d−1.
    pub fn is_centered(&self) -> bool {
        let mut child_found = false;

        for (child, _) in &self.children {
            if child.height == self.height - 1 {
                if child_found {
                    return true;
                }
                child_found = true;
            }
        }
        false
    }

    /// Returns `true` if the tree is a path.
    ///
    /// A path is an orientation of a tree which has only vertices of degree 2
    /// and 1.
    pub const fn is_path(&self) -> bool {
        self.max_arity < 3
    }

    /// Returns `true` if the tree is a triad.
    ///
    /// A triad is an orientation of a tree which has a single vertex of degree
    /// 3 and otherwise only vertices of degree 2 and 1.
    pub fn is_triad(&self) -> bool {
        let mut root_found = false;

        match self.arity() {
            4.. => return false,
            3 => {
                root_found = true;
            }
            _ => {}
        }

        let mut stack = self.children.iter().map(|(t, _)| t.as_ref()).collect_vec();

        while let Some(tree) = stack.pop() {
            match tree.arity() {
                3.. => return false,
                2 => {
                    if root_found {
                        return false;
                    }
                    root_found = true;
                }
                _ => {}
            }
            stack.extend(tree.children.iter().map(|(t, _)| t.as_ref()));
        }
        root_found
    }

    pub fn iter(&self) -> impl Iterator<Item = (Arc<Node>, bool)> + '_ {
        self.children.iter().cloned()
    }
}

impl Tree for Node {}

impl ToGraph for Node {
    type V = u32;

    fn to_graph(&self) -> AdjMap<Self::V> {
        fn inner(id: &mut u32, tree: &Node, graph: &mut AdjMap<u32>) {
            let id_parent = *id;

            for (child, dir) in &tree.children {
                *id += 1;
                graph.add_vertex(*id);

                if *dir {
                    graph.add_edge(&id_parent, id);
                } else {
                    graph.add_edge(id, &id_parent);
                }
                inner(id, child, graph);
            }
        }

        let mut id = 0;
        let mut graph = AdjMap::new();
        graph.add_vertex(id);
        inner(&mut id, self, &mut graph);

        graph
    }
}

impl From<Triad> for Node {
    fn from(triad: Triad) -> Node {
        todo!()
    }
}

impl From<Node> for AdjMap<u32> {
    fn from(tree: Node) -> AdjMap<u32> {
        (&tree).to_graph()
    }
}

impl Rooted for Node {}

impl Balanced for Node {
    fn level(&self, id: &u32) -> Option<u32> {
        let mut rank = 0;
        let mut count = *id as usize;
        let mut node = self;

        while count > 0 {
            for (child, dir) in &node.children {
                if count <= child.num_nodes {
                    if *dir {
                        rank -= 1;
                    } else {
                        rank += 1;
                    }
                    count -= 1;
                    node = child.as_ref();
                    break;
                }
                count -= child.num_nodes;
            }
        }
        Some(rank)
    }
}

impl FromIterator<(Arc<Node>, bool)> for Node {
    fn from_iter<T: IntoIterator<Item = (Arc<Node>, bool)>>(iter: T) -> Node {
        let mut num_nodes = 1;
        let mut height = 0;
        let mut max_arity = 0;
        let mut children = Vec::new();

        for (child, dir) in iter {
            num_nodes += child.num_nodes;
            height = max(height, child.height);
            max_arity = max(max_arity, child.children.len() + 1);
            children.push((child, dir));
        }

        Node {
            num_nodes,
            height: height + 1,
            max_arity: max(max_arity, children.len()),
            children,
        }
    }
}

impl FromStr for Node {
    type Err = ParseNodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut children_stack = Vec::<Vec<(Arc<Node>, bool)>>::new();
        let mut dir_stack = Vec::new();

        let mut chars = s.chars().tuple_windows();
        let mut dir = false;

        while let Some((c, d)) = chars.next() {
            match (c, d) {
                ('0', e) => {
                    dir = false;
                    match e {
                        '0' | '1' | ']' => {
                            children_stack
                                .last_mut()
                                .unwrap()
                                .push((Arc::new(Node::leaf()), dir));
                        }
                        _ => {}
                    }
                }
                ('1', e) => {
                    dir = true;
                    match e {
                        '0' | '1' | ']' => {
                            children_stack
                                .last_mut()
                                .unwrap()
                                .push((Arc::new(Node::leaf()), dir));
                        }
                        _ => {}
                    }
                }
                ('[', _) => {
                    children_stack.push(Vec::new());
                    dir_stack.push(dir);
                }
                (']', _) => {
                    let children = children_stack.pop().unwrap();
                    let dir = dir_stack.pop().unwrap();
                    children_stack
                        .last_mut()
                        .unwrap()
                        .push((Arc::new(Node::from_iter(children)), dir));
                }
                (e, _) => {
                    return Err(ParseNodeError::InvalidCharacter(e));
                }
            }
        }
        if let Some(v) = children_stack.pop() {
            Ok(Node::from_iter(v))
        } else {
            return Err(ParseNodeError::InvalidCharacter('a'));
        }
    }
}

#[derive(Debug)]
pub enum ParseNodeError {
    InvalidCharacter(char),
    // Delimiter
}

impl std::fmt::Display for ParseNodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseNodeError::InvalidCharacter(c) => write!(f, "Could not parse: {}", c),
        }
    }
}

impl std::error::Error for ParseNodeError {
    fn description(&self) -> &str {
        match self {
            ParseNodeError::InvalidCharacter(_) => "Only 0, 1, [, ] allowed!",
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        for (child, dir) in &self.children {
            let d = if *dir { '1' } else { '0' };
            s.push_str(&format!("{}{}", d, child));
        }
        if self.arity() != 0 {
            s = "[".to_string() + s.as_str() + "]";
        }

        write!(f, "{}", s)
    }
}
