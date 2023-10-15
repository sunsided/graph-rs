use crate::embedded_property_graph::EmbeddedPropertyGraph;
use crate::node_address::NodeAddress;
use crate::node_relation::NodePathLink;
use std::collections::{HashSet, VecDeque};

/// A breadth-first search (BFS) solver for shortest path queries.
#[derive(Debug, Default)]
pub struct BreadthFirstSearch;

impl BreadthFirstSearch {
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
    pub fn shortest_path<N, R>(
        &self,
        graph: &EmbeddedPropertyGraph<N, R>,
        start: NodeAddress,
        target: NodeAddress,
    ) -> Vec<NodePathLink<R>>
    where
        R: Clone,
    {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back(vec![NodePathLink {
            relation: None,
            address: start,
        }]);

        while let Some(path) = queue.pop_front() {
            let current_node = path.last().expect("path must not be empty").clone();

            if current_node.address == target {
                return path;
            }

            if visited.contains(&current_node.address) {
                continue;
            }

            visited.insert(current_node.address.clone());

            let neighbors = graph
                .iter_local_neighbors(&current_node.address)
                .expect("remote node lookups are not yet supported");

            for relation in neighbors {
                let mut new_path = path.clone();
                new_path.push(relation.into());
                queue.push_back(new_path);
            }
        }

        Vec::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::examples::london_graph::{london_graph, ConnectionType};

    #[test]
    fn it_works() {
        let solver = BreadthFirstSearch::default();
        let graph = london_graph();
        let path = solver.shortest_path(
            &graph,
            NodeAddress::from_local(0),
            NodeAddress::from_local(198),
        );

        assert_eq!(path.len(), 6);
        assert_eq!(path[0].address, NodeAddress::Local(0));

        assert_eq!(path[1].address, NodeAddress::Local(45));
        assert_eq!(path[1].relation, Some(ConnectionType::Bus));

        assert_eq!(path[2].address, NodeAddress::Local(12));
        assert_eq!(path[2].relation, Some(ConnectionType::Underground));

        assert_eq!(path[3].address, NodeAddress::Local(88));
        assert_eq!(path[3].relation, Some(ConnectionType::Underground));

        assert_eq!(path[4].address, NodeAddress::Local(127));
        assert_eq!(path[4].relation, Some(ConnectionType::Underground));

        assert_eq!(path[5].address, NodeAddress::Local(198));
        assert_eq!(path[5].relation, Some(ConnectionType::Bus));
    }

    #[test]
    fn unreachable() {
        let solver = BreadthFirstSearch::default();
        let graph = london_graph();
        let path = solver.shortest_path(
            &graph,
            NodeAddress::from_local(0),
            NodeAddress::from_local(199),
        );

        assert_eq!(path.len(), 0);
    }
}
