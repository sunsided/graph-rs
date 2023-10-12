use crate::node_address::NodeAddress;
use crate::{Graph, NodePathLink};
use std::collections::HashMap;

/// An A* search solver for shortest path queries.
#[derive(Debug, Default)]
pub struct AStarSearch;

/// Trait for heuristics.
pub trait Heuristic<N> {
    /// Provides a heuristic value for the cost to the target node.
    ///
    /// ## Admissible Heuristics
    /// The heuristic function must be admissible, i.e. never overestimate the true distance.
    fn heuristic(&self, from: &N, to: &N) -> f32;
}

impl AStarSearch {
    /// Performs a shortest path query on the specified graph,
    /// starting at the `start` node, attempting to reach the `target` node.
    ///
    /// ## Arguments
    ///
    /// * `graph` - The graph to search on.
    /// * `start` - The starting address in the graph.
    /// * `target` - The target address in the graph.
    ///
    /// ## Returns
    ///
    /// A path from `start` to `target` or an empty vector if no such path exists.
    pub fn shortest_path<N, R, H>(
        &self,
        graph: &Graph<N, R>,
        start: NodeAddress,
        target: NodeAddress,
        heuristic: &H,
    ) -> Vec<NodePathLink<R>>
    where
        R: Clone,
        H: Heuristic<N>,
    {
        // The set of nodes to be evaluated
        let mut open_set = OpenSet::default();

        // Cost from start along best known path
        let mut g_scores: HashMap<NodeAddress, f32> = HashMap::new();

        // For each node, which node it can most efficiently be reached from
        let mut came_from: HashMap<NodeAddress, Option<NodePathLink<R>>> = HashMap::new();

        // Cost from start (to start) along best known path is zero
        g_scores.insert(start.clone(), 0.0);

        // Cost from start to goal, estimated by heuristic
        open_set.insert(
            start.clone(),
            heuristic.heuristic(
                &graph
                    .get_local_node_ref(&start)
                    .expect("the start node does not exist in the graph")
                    .data,
                &graph
                    .get_local_node_ref(&target)
                    .expect("the target node does not exist in the graph")
                    .data,
            ),
        );

        // There is no path towards the start node; it just is.
        came_from.insert(start.clone(), None);

        // Fetch the node with the lowest f-score from the open set.
        while let Some(current_addr) = open_set.pop() {
            if current_addr == target {
                return reconstruct_path(came_from, current_addr);
            }

            // Get the current node's g-score to avoid later lookups.
            let current_g_score = *g_scores
                .get(&current_addr)
                .expect("current node has no g-score");

            // Process all neighbors of the current node.
            let current_node = graph.get_local_node_ref(&current_addr).unwrap();
            for neighbor in current_node.outgoing.iter() {
                // TODO: Provide proper distance between the current node and the neighbor; right now, we consider the cost of
                //       moving from the current node to the neighbor to be one unit step.
                let distance_cost = 1.0;

                // Determine the true distance to the neighbor node from the current node.
                let tentative_g_score = current_g_score + distance_cost;

                // Determine the true cost to the neighbor node if it was already visited before.
                let mut neighbor_g_score =
                    *g_scores.get(&neighbor.address).unwrap_or(&f32::INFINITY);

                // Only update the neighbor node if we found a shorter path to it.
                if tentative_g_score < neighbor_g_score {
                    neighbor_g_score = tentative_g_score;

                    // Insert the path to the neighbor along the current node's outgoing relation.
                    // TODO: Insert step count here to simplify buffer creation later on?
                    came_from.insert(
                        neighbor.address.clone(),
                        Some(NodePathLink {
                            address: current_addr.clone(),
                            relation: Some(neighbor.relation.clone()),
                        }),
                    );

                    // Update the g-score with the better value.
                    g_scores.insert(neighbor.address.clone(), neighbor_g_score);

                    // Calculate the f-score of the neighbor with the heuristic from the neighbor
                    // node towards the goal node.
                    let neighbor_f_score = neighbor_g_score
                        + heuristic.heuristic(
                            &graph
                                .get_local_node_ref(&neighbor.address)
                                .expect("the neighbor node does not exist in the graph")
                                .data,
                            &graph
                                .get_local_node_ref(&target)
                                .expect("the target node does not exist in the graph")
                                .data,
                        );

                    // Update the open set.
                    open_set.insert(neighbor.address.clone(), neighbor_f_score);
                }
            }
        }

        // Failure; no path found.
        Vec::default()
    }
}

/// The open set of nodes; maintains a priority queue of nodes sorted by
/// their f-score in ascending order.
#[derive(Debug, Default)]
struct OpenSet {
    f_scores: HashMap<NodeAddress, f32>,
}

impl OpenSet {
    /// Inserts a node into the open set, overwriting any existing values.
    pub fn insert(&mut self, address: NodeAddress, f_score: f32) {
        self.f_scores.insert(address, f_score);
    }

    /// Returns the node with the smallest f-score from the set, if any.
    pub fn pop(&mut self) -> Option<NodeAddress> {
        let mut smallest_score = f32::INFINITY;
        let mut best_node = None;
        for (address, &score) in self.f_scores.iter() {
            if score < smallest_score {
                smallest_score = score;
                best_node = Some(address.clone())
            }
        }

        if let Some(best_node) = best_node {
            self.f_scores.remove(&best_node);
            Some(best_node)
        } else {
            None
        }
    }
}

fn reconstruct_path<R>(
    came_from: HashMap<NodeAddress, Option<NodePathLink<R>>>,
    mut current_addr: NodeAddress,
) -> Vec<NodePathLink<R>>
where
    R: Clone,
{
    // TODO: Track the number of steps to directly allocate a vector of the correct size?
    let mut path = Vec::default();
    while let Some(parent) = came_from.get(&current_addr).cloned() {
        if let Some(relation) = parent {
            path.push(NodePathLink {
                address: current_addr,
                relation: relation.relation,
            });
            current_addr = relation.address.clone();
        } else {
            path.push(NodePathLink {
                address: current_addr,
                relation: None,
            });

            path.reverse();
            return path;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::examples::london_graph::{
        london_graph, ConnectionType, LondonGraphLocationHeuristic,
    };

    #[test]
    fn it_works() {
        let solver = AStarSearch::default();
        let graph = london_graph();
        let heuristic = LondonGraphLocationHeuristic::default();
        let path = solver.shortest_path(
            &graph,
            NodeAddress::from_local(0),
            NodeAddress::from_local(198),
            &heuristic,
        );

        assert_eq!(path.len(), 9);
        assert_eq!(path[0].address, NodeAddress::Local(0));

        assert_eq!(path[1].address, NodeAddress::Local(45));
        assert_eq!(path[1].relation, Some(ConnectionType::Bus));

        assert_eq!(path[2].address, NodeAddress::Local(78));
        assert_eq!(path[2].relation, Some(ConnectionType::Underground));

        assert_eq!(path[3].address, NodeAddress::Local(110));
        assert_eq!(path[3].relation, Some(ConnectionType::Underground));

        assert_eq!(path[4].address, NodeAddress::Local(152));
        assert_eq!(path[4].relation, Some(ConnectionType::Underground));

        assert_eq!(path[5].address, NodeAddress::Local(184));
        assert_eq!(path[5].relation, Some(ConnectionType::Underground));

        assert_eq!(path[6].address, NodeAddress::Local(186));
        assert_eq!(path[6].relation, Some(ConnectionType::Bus));

        assert_eq!(path[7].address, NodeAddress::Local(187));
        assert_eq!(path[7].relation, Some(ConnectionType::Taxi));

        assert_eq!(path[8].address, NodeAddress::Local(198));
        assert_eq!(path[8].relation, Some(ConnectionType::Taxi));
    }

    #[test]
    fn unreachable() {
        let solver = AStarSearch::default();
        let graph = london_graph();
        let heuristic = LondonGraphLocationHeuristic::default();
        let path = solver.shortest_path(
            &graph,
            NodeAddress::from_local(0),
            NodeAddress::from_local(199),
            &heuristic,
        );

        assert_eq!(path.len(), 0);
    }
}
