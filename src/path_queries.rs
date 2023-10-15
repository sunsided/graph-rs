//! Provides graph traversal and path queries.

pub mod astar;
pub mod bfs;
pub mod dfs;

/// Trait for heuristics.
pub trait AdmissibleHeuristic<N> {
    /// Provides a heuristic value for the estimated cost to the target node.
    ///
    /// ## Admissible Heuristics
    /// The heuristic function must be admissible, i.e. never overestimate the true distance.
    fn heuristic(&self, from: &N, to: &N) -> f32;
}

/// Trait for path costs.
pub trait PathCost<N, R> {
    /// Provides a value for the actual cost to the target node.
    ///
    /// This function is mainly used to determine the cost to move to a neighbor node.
    /// If there is no path to the target node, the returned cost should be [`f32::INFINITY`].
    fn path_cost(&self, from: &N, to: &N, relation: &R) -> f32;
}
