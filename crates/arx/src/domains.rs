//! A data-structure to represent the domains of a CSP.

use std::iter::FromIterator;
use std::ops::Range;

use crate::problem::{Problem, Value, Variable};

use self::list::List;

pub type Dom = List<Value>;

/// A data-structure to represent the domains of a CSP.
#[derive(Debug, Clone)]
pub struct DomMap {
    map: Vec<Dom>,
}

impl DomMap {
    pub fn new<P: Problem>(problem: &P) -> DomMap {
        let domains = (0..problem.size())
            .map(|i| List::from_iter(problem.domain(Variable(i))))
            .collect();

        DomMap { map: domains }
    }

    /// Assigns the value at index `index` to variable `x`.
    pub fn get(&self, x: Variable) -> &Dom {
        &self.map[*x]
    }

    /// Assigns the value at index `index` to variable `x`.
    pub fn get_mut(&mut self, x: Variable) -> &mut Dom {
        &mut self.map[*x]
    }

    pub fn insert(&mut self, x: Variable, dom: Dom) -> Option<Dom> {
        if let Some(d) = self.map.get_mut(*x) {
            Some(std::mem::replace(d, dom))
        } else {
            None
        }
    }

    pub fn set(&mut self, x: Variable, index: usize) -> Option<Dom> {
        self.insert(x, Dom::singleton(self.value(x, index)))
    }

    /// Removes the value at index `index` from the domain of variable `x`.
    pub fn remove(&mut self, x: Variable, index: usize) {
        self.map[*x].unlink(index);
    }

    /// Restores the value at index `index` in the domain of variable `x`.
    pub fn restore(&mut self, x: Variable, index: usize) {
        self.map[*x].relink(index);
    }

    /// Returns the value at index `index` of variable `x`.
    pub fn value(&self, x: Variable, index: usize) -> Value {
        self.map[*x][index]
    }

    pub fn vars_count(&self) -> usize {
        self.map.len()
    }

    /// Returns an iterator over all variables.
    pub fn vars(&self) -> Vars {
        Vars {
            iter: 0..self.map.len(),
        }
    }

    /// Returns an iterator over the indices of values in the domain of `x`.
    pub fn indices(&self, x: Variable) -> Indices {
        Indices {
            iter: Box::new(self.map[*x].clone().into_iter()),
        }
    }

    pub fn assignment(&self) -> Option<Vec<Value>> {
        let mut assignment = Vec::with_capacity(self.map.len());

        for x in self.vars() {
            if self.map[*x].size() == 1 {
                assignment.push(self.value(x, self.indices(x).next().unwrap()));
            } else {
                return None;
            }
        }

        Some(assignment)
    }

    /// Print a debug representation of the domains
    pub fn debug_print(&self) {
        eprintln!("Domains:");

        for x in self.vars() {
            eprintln!(
                "{:?} -> {:?}",
                *x,
                self.indices(x)
                    .map(|i| *self.value(x, i))
                    .collect::<Vec<_>>(),
            );
        }
    }
}

pub struct Vars {
    iter: Range<usize>,
}

impl Iterator for Vars {
    type Item = Variable;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|i| Variable(i))
    }
}

pub struct Indices {
    iter: Box<dyn Iterator<Item = usize>>,
}

impl Iterator for Indices {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<A: IntoIterator<Item = usize>> FromIterator<A> for DomMap {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> DomMap {
        let domains = iter
            .into_iter()
            .map(|i| List::from_iter(i.into_iter().map(|v| Value(v))))
            .collect::<Vec<_>>();
        DomMap { map: domains }
    }
}

#[cfg(test)]
mod tests {}

mod list {
    use std::iter::FromIterator;

    use Direction::*;
    use Store::*;

    /// Direction of list link
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub(crate) enum Direction {
        Prev,
        Next,
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub(crate) enum Store<T> {
        Head,
        Item(T),
    }

    /// Link node along left/right
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct Node<T> {
        /// Left, Right
        link: [usize; 2],
        pub(crate) value: Store<T>,
    }

    impl<T> Node<T> {
        /// Create a new node from the value.
        fn item(value: T) -> Node<T> {
            Node {
                value: Item(value),
                link: [!0; 2], // invalid link values to start with
            }
        }

        fn head() -> Node<T> {
            Node {
                value: Head,
                link: [!0; 2], // invalid link values to start with
            }
        }

        /// Get link in the given direction
        fn get(&self, dir: Direction) -> usize {
            self.link[dir as usize]
        }

        /// Set link in the given direction
        fn set(&mut self, dir: Direction, index: usize) -> &mut Node<T> {
            self.link[dir as usize] = index;
            self
        }

        /// Assign link in the given direction
        fn assign(&mut self, dir: Direction) -> &mut usize {
            &mut self.link[dir as usize]
        }
    }

    #[derive(Clone, Debug)]
    pub struct List<T> {
        nodes: Vec<Node<T>>,
        size: usize,
    }

    impl<T> List<T> {
        pub fn new() -> List<T> {
            List {
                nodes: vec![Node::head()],
                size: 0,
            }
        }

        pub fn singleton(value: T) -> List<T> {
            List::from_iter(vec![value])
        }

        fn len(&self) -> usize {
            self.nodes.len()
        }

        // #[inline]
        pub fn size(&self) -> usize {
            self.size
        }

        pub fn is_empty(&self) -> bool {
            self.size() == 0
        }

        fn node(&self, index: usize) -> &Node<T> {
            &self.nodes[index]
        }

        fn node_mut(&mut self, index: usize) -> &mut Node<T> {
            &mut self.nodes[index]
        }

        #[inline]
        fn head(&self) -> usize {
            0
        }

        pub fn iter<'a>(&'a self) -> Iter<'a, T> {
            Iter {
                list: self,
                index: 0,
            }
        }

        /// Remove `x` from the list where the list is doubly linked.
        ///
        /// x.left.right ← x.right;
        /// x.right.left ← x.left;
        pub(crate) fn unlink(&mut self, index: usize) {
            let x = &self.nodes[index];
            let xn = x.get(Next);
            let xp = x.get(Prev);

            self.nodes[xp].set(Next, xn);
            self.nodes[xn].set(Prev, xp);
            self.size -= 1;
        }

        /// Restore `x` to the list, reversing a leftious removal.
        ///
        /// x.left.right ← x;
        /// x.right.left ← x;
        pub(crate) fn relink(&mut self, index: usize) {
            let x = &self.nodes[index];
            let xn = x.get(Next);
            let xp = x.get(Prev);

            self.nodes[xp].set(Next, index);
            self.nodes[xn].set(Prev, index);
            self.size += 1;
        }
    }

    pub struct Iter<'a, T> {
        list: &'a List<T>,
        index: usize,
    }

    impl<'a, T> Iterator for Iter<'a, T> {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            let right = self.list.node(self.index).get(Next);
            self.index = right;

            if right == self.list.head() {
                None
            } else {
                Some(right)
            }
        }
    }

    fn enumerate<T>(it: impl IntoIterator<Item = T>) -> impl Iterator<Item = (usize, T)> {
        it.into_iter().enumerate()
    }

    impl<T> FromIterator<T> for List<T> {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> List<T> {
            let mut list = List::new();
            for value in iter {
                list.nodes.push(Node::item(value));
                list.size += 1;
            }

            for (index, node) in enumerate(&mut list.nodes) {
                *node.assign(Prev) = index.wrapping_sub(1);
                *node.assign(Next) = index + 1;
            }
            // fixup begin/end
            let len = list.len();
            *list.node_mut(0).assign(Prev) = len - 1;
            *list.node_mut(len - 1).assign(Next) = 0;

            list
        }
    }

    impl<T> std::fmt::Display for List<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "[")?;
            for v in self.iter() {
                write!(f, "{:?} ", v)?;
            }
            write!(f, "]")
        }
    }

    impl<T> std::ops::Index<usize> for List<T> {
        type Output = T;

        fn index(&self, index: usize) -> &Self::Output {
            if let Item(ref x) = self.nodes[index].value {
                x
            } else {
                panic!("Trying to access head node");
            }
        }
    }

    impl<T> std::ops::IndexMut<usize> for List<T> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            if let Item(ref mut x) = self.nodes[index].value {
                x
            } else {
                panic!("Trying to access head node");
            }
        }
    }

    pub struct IntoIter<T> {
        list: List<T>,
        index: usize,
    }

    impl<T> Iterator for IntoIter<T> {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            let right = self.list.node(self.index).get(Next);
            self.index = right;

            if right == self.list.head() {
                None
            } else {
                Some(right)
            }
        }
    }

    impl<T> IntoIterator for List<T> {
        type Item = usize;
        type IntoIter = IntoIter<T>;

        fn into_iter(self) -> Self::IntoIter {
            IntoIter {
                list: self,
                index: 0,
            }
        }
    }

    impl<'a, T> IntoIterator for &'a List<T> {
        type Item = usize;
        type IntoIter = Iter<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn from_iter() {
            let list = List::from_iter(vec![3, 5, 6]);
            let mut iter = list.iter();

            assert_eq!(iter.next(), Some(1));
            assert_eq!(iter.next(), Some(2));
            assert_eq!(iter.next(), Some(3));
            assert_eq!(iter.next(), None);
        }

        #[test]
        fn index() {
            let list = List::from_iter(vec![3, 5, 6]);

            assert_eq!(list[1], 3);
            assert_eq!(list[3], 6);
        }

        #[test]
        fn link() {
            let mut list = List::from_iter(vec![3, 5, 6]);

            list.unlink(2);
            let mut iter = list.iter();

            assert_eq!(iter.next(), Some(1));
            assert_eq!(iter.next(), Some(3));
            assert_eq!(iter.next(), None);

            list.relink(2);
            let mut iter = list.iter();

            assert_eq!(iter.next(), Some(1));
            assert_eq!(iter.next(), Some(2));
            assert_eq!(iter.next(), Some(3));
            assert_eq!(iter.next(), None);
        }
    }
}
