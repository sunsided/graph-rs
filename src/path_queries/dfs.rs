use crate::node_address::NodeAddress;
use crate::{Graph, NodePathLink};
use std::collections::HashSet;

/// A depth-first search (DFS) solver for finding a path in a graph.
#[derive(Debug, Default)]
pub struct DepthFirstSearch;

impl DepthFirstSearch {
    /// Performs a depth-first search to find a path between nodes in the graph.
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
    pub fn find_path<N, R>(
        &self,
        graph: &Graph<N, R>,
        start: NodeAddress,
        target: NodeAddress,
    ) -> Vec<NodePathLink<R>>
    where
        R: Clone,
    {
        let mut visited = HashSet::new();
        let mut stack = Vec::new();

        stack.push(vec![NodePathLink {
            relation: None,
            address: start,
        }]);

        while let Some(path) = stack.pop() {
            let current_node = path.last().expect("path must not be empty").clone();

            if current_node.address == target {
                return path;
            }

            if visited.contains(&current_node.address) {
                continue;
            }

            visited.insert(current_node.address.clone());

            let current_node = graph
                .get_local_node_ref(&current_node.address)
                .expect("remote node lookups are not yet supported");

            for relation in &current_node.outgoing {
                let mut new_path = path.clone();
                new_path.push(relation.into());
                stack.push(new_path);
            }
        }

        Vec::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::examples::london_graph::london_graph;

    #[test]
    fn it_works() {
        let solver = DepthFirstSearch::default();
        let graph = london_graph();
        let path = solver.find_path(
            &graph,
            NodeAddress::from_local(0),
            NodeAddress::from_local(198),
        );

        assert_eq!(path.len(), 45);
    }

    #[test]
    fn unreachable() {
        let solver = DepthFirstSearch::default();
        let graph = london_graph();
        let path = solver.find_path(
            &graph,
            NodeAddress::from_local(0),
            NodeAddress::from_local(199),
        );

        assert_eq!(path.len(), 0);
    }
}
