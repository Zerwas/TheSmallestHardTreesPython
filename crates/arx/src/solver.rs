//! A simple backtracking algorithm using MAC-3.

use std::iter::FromIterator;

use time::{Duration, OffsetDateTime};

use crate::domains::{Dom, DomMap};
use crate::problem::{Problem, Solution, Variable};

// These macros are used trace debug logging

#[cfg(feature = "trace")]
macro_rules! if_trace {
    ($($t:tt)*) => { $($t)* }
}

#[cfg(not(feature = "trace"))]
macro_rules! if_trace {
    ($($t:tt)*) => {};
}

macro_rules! trace {
    ($($t:tt)*) => { if_trace!(eprintln!($($t)*)) }
}

type Stack<T> = Vec<T>;

#[derive(Debug, Clone)]
struct Trail {
    pub(crate) removals: Vec<(Variable, usize)>,
    pub(crate) dom: Dom,
}

impl Trail {
    fn new(dom: Dom) -> Trail {
        Trail {
            removals: Stack::new(),
            dom,
        }
    }
}

/// Configuration for backtracking
#[derive(Clone, Debug)]
pub struct BTConfig {
    /// Sort the stack of variables after initial run of AC-3 (smallest domain first)
    pub sort_stack: bool,
    /// If true, stop at first solution
    pub stop_at_first: bool,
    /// If true, record statistics during the search
    pub record_stats: bool,
}

impl Default for BTConfig {
    fn default() -> BTConfig {
        BTConfig {
            sort_stack: true,
            stop_at_first: false,
            record_stats: true,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) enum BTError {
    /// Used internally to stop after the first solution (if enabled)
    RequestedStop,
}

/// Statistics from the execution of the backtracking search.
#[derive(Clone, Debug, Default)]
pub struct BTStats {
    /// Number of consistency checks
    pub ccks: u32,
    /// Number of backtracks from dead ends
    pub backtracks: u32,
    /// Number of solutions emitted
    pub solutions: u32,
    /// Duration of the arc-consistency preprocessing
    pub ac3_time: Duration,
    /// Duration of the entire solving process
    pub mac3_time: Duration,
}

impl BTStats {
    pub fn print(&self) {
        println!("{: <12} {: <10}", "consistency checks:", self.ccks);
        println!("{: <12} {: <10}", "backtracks:", self.backtracks);
        println!("{: <12} {: <10}", "solutions:", self.solutions);
        println!("{: <12} {: <10?}", "ac_time", self.ac3_time);
        println!("{: <12} {: <10?}", "search_time:", self.mac3_time);
    }
}

macro_rules! stat {
    ($c:ident . $field:ident $($t:tt)*) => {
            if let Some(ref mut st) = $c.stats {
                st . $field $($t)*;
            }
    }
}

/// Backtracking solver
#[derive(Clone, Debug)]
pub struct BTSolver<'p, P: Problem> {
    problem: &'p P,
    domains: DomMap,
    neighbors: Vec<Vec<(Variable, Variable)>>,
    variables: Stack<Variable>,
    assignments: Stack<(Variable, usize)>,
    config: BTConfig,
    stats: Option<BTStats>,
    trails: Stack<Trail>,
}

impl<'p, P: Problem> BTSolver<'p, P> {
    /// Constructs a new `BTSolver<p', P>` from `problem`.
    pub fn new(problem: &'p P) -> BTSolver<P> {
        BTSolver::with_config(problem, BTConfig::default())
    }

    /// Constructs a new `BTSolver<p', P>` from `problem`.
    pub fn with_config(problem: &'p P, config: BTConfig) -> BTSolver<P> {
        let mut neighbors = vec![Vec::new(); problem.size()];

        for arc in problem.arcs() {
            neighbors[*arc.1].push(arc);
        }

        BTSolver {
            problem,
            domains: DomMap::new(problem),
            neighbors,
            variables: Stack::from_iter((0..problem.size()).map(|x| Variable(x))),
            stats: if config.record_stats {
                Some(BTStats::default())
            } else {
                None
            },
            config,
            trails: Stack::new(),
            assignments: Stack::new(),
        }
    }

    pub fn stats(&self) -> Option<&BTStats> {
        self.stats.as_ref()
    }

    fn assign(&mut self, x: Variable, index: usize) {
        self.assignments.push((x, index));

        if let Some(old) = self.domains.set(x, index) {
            self.trails.push(Trail::new(old));
        }
    }

    fn unassign(&mut self, x: Variable) {
        let (_, i) = self.assignments.pop().unwrap();
        let trail = self.trails.pop().unwrap();

        for (y, a) in trail.removals.into_iter().rev() {
            self.domains.restore(y, a);
        }
        self.domains.insert(x, trail.dom);
        self.remove(x, i);
    }

    fn remove(&mut self, x: Variable, i: usize) {
        self.domains.remove(x, i);
        if let Some(trail) = self.trails.last_mut() {
            trail.removals.push((x, i));
        }
    }

    fn revise(&mut self, x: Variable, y: Variable) -> Option<bool> {
        let mut mutated = false;

        for i in self.domains.indices(x) {
            let mut is_possible = false;

            for j in self.domains.indices(y) {
                stat!(self.ccks += 1);
                if self
                    .problem
                    .check((x, self.domains.value(x, i)), (y, self.domains.value(y, j)))
                {
                    is_possible = true;
                    break;
                }
            }

            if !is_possible {
                self.remove(x, i);
                mutated = true;

                if self.domains.get(x).is_empty() {
                    return None;
                }
            }
        }

        Some(mutated)
    }

    fn mac3<I: IntoIterator<Item = (Variable, Variable)>>(&mut self, arcs: I) -> bool {
        let mut work_list = Vec::from_iter(arcs);

        while let Some((x, y)) = work_list.pop() {
            if let Some(mutated) = self.revise(x, y) {
                if mutated {
                    work_list.extend(self.neighbors[*x].iter().copied());
                }
            } else {
                return false;
            }
        }
        true
    }
}

impl<'p, P: Problem> BTSolver<'p, P> {
    /// Returns true, if the there exists a solution to the problem.
    pub fn solution_exists(&mut self) -> bool {
        self.solve_first().is_some()
    }

    /// Get the first found solution to the problem
    ///
    /// This is faster than `solve_all`, also in the case where there only is
    /// one solution: the extra work in solve all is the part needed to know
    /// that the solution is unique, in that case. This method can not say if
    /// the solution is unique or not.
    pub fn solve_first(&mut self) -> Option<Solution> {
        self.config.stop_at_first = true;
        let mut solution = None;
        self.solve(|s| solution = Some(s));
        solution
    }

    /// Get every solution to the problem
    pub fn solve_all(&mut self, out: impl FnMut(Solution)) {
        self.config.stop_at_first = false;
        self.solve(out);
    }

    fn solve(&mut self, mut out: impl FnMut(Solution)) {
        trace!("Backtracking start");
        if_trace!(self.domains.debug_print());

        if_trace!("Preprocessing with AC-3");
        let tstart = OffsetDateTime::now_utc();
        let ac = self.mac3(self.problem.arcs());
        let tend = OffsetDateTime::now_utc();
        stat!(self.ac3_time = tend - tstart);

        if_trace!(self.domains.debug_print());

        if self.config.sort_stack {
            let dom_sizes = self
                .domains
                .vars()
                .map(|x| self.domains.get(x).size())
                .collect::<Vec<_>>();

            self.variables
                .sort_unstable_by(|&a, &b| dom_sizes[*b].cmp(&dom_sizes[*a]));
        }

        if ac {
            let tstart = OffsetDateTime::now_utc();
            let _ = self.solve_inner(&mut out);
            let tend = OffsetDateTime::now_utc();
            stat!(self.mac3_time = tend - tstart);
        }
    }

    /// The solver runs through the whole search tree if not interrupted; the
    /// BTError status is used to short-circuit and exit as soon as possible if
    /// requested.
    fn solve_inner<F>(&mut self, out: &mut F) -> Result<(), BTError>
    where
        F: FnMut(Solution),
    {
        let mut backtrack = false;
        let mut depth = 0;

        loop {
            if depth == self.domains.vars_count() {
                // We have a solution
                let solution = self.domains.assignment().unwrap();
                trace!("==> Valid solution: {:?}", solution);
                stat!(self.solutions += 1);
                out(solution);

                if self.config.stop_at_first {
                    return Err(BTError::RequestedStop);
                }
                if depth == 0 {
                    return Ok(());
                } else {
                    depth -= 1;
                }
            }
            let x = self.variables[depth];
            trace!("> Selected variable = {}", x);

            if let Some(i) = self.domains.indices(x).next() {
                if backtrack {
                    self.unassign(x);
                    backtrack = false;
                    if depth == 0 {
                        return Ok(());
                    } else {
                        depth -= 1;
                    }
                } else {
                    trace!("  - Assignment: {} -> {}", x, self.domains.value(x, i));
                    self.assign(x, i);

                    if self.mac3(self.neighbors[*x].clone()) {
                        trace!("  - Propagation successful, recursing...");
                        depth += 1;
                    } else {
                        trace!("  - Detected inconsistency, backtracking...");
                        stat!(self.backtracks += 1);
                        self.unassign(x);
                    }
                }
            } else {
                trace!("  - Detected emtpy list, backtracking...");
                backtrack = true;
                if depth == 0 {
                    return Ok(());
                } else {
                    depth -= 1;
                }
            }
        }
    }
}
