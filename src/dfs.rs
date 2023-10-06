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
                .get_local_node(&current_node.address)
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
    use crate::examples::london_graph::{london_graph, ConnectionType};

    #[test]
    fn it_works() {
        let solver = DepthFirstSearch::default();
        let graph = london_graph();
        let path = solver.find_path(
            &graph,
            NodeAddress::from_local(0),
            NodeAddress::from_local(198),
        );

        assert_eq!(path.len(), 16);
        assert_eq!(path[0].address, NodeAddress::Local(0));

        assert_eq!(path[1].address, NodeAddress::Local(45));
        assert_eq!(path[1].relation, Some(ConnectionType::Underground));

        assert_eq!(path[2].address, NodeAddress::Local(78));
        assert_eq!(path[2].relation, Some(ConnectionType::Underground));

        assert_eq!(path[3].address, NodeAddress::Local(110));
        assert_eq!(path[3].relation, Some(ConnectionType::Underground));

        assert_eq!(path[4].address, NodeAddress::Local(162));
        assert_eq!(path[4].relation, Some(ConnectionType::Underground));

        assert_eq!(path[5].address, NodeAddress::Local(164));
        assert_eq!(path[5].relation, Some(ConnectionType::Bus));

        assert_eq!(path[6].address, NodeAddress::Local(190));
        assert_eq!(path[6].relation, Some(ConnectionType::Bus));

        assert_eq!(path[7].address, NodeAddress::Local(189));
        assert_eq!(path[7].relation, Some(ConnectionType::Bus));

        assert_eq!(path[8].address, NodeAddress::Local(179));
        assert_eq!(path[8].relation, Some(ConnectionType::Bus));

        assert_eq!(path[9].address, NodeAddress::Local(183));
        assert_eq!(path[9].relation, Some(ConnectionType::Bus));

        assert_eq!(path[10].address, NodeAddress::Local(184));
        assert_eq!(path[10].relation, Some(ConnectionType::Bus));

        assert_eq!(path[11].address, NodeAddress::Local(152));
        assert_eq!(path[11].relation, Some(ConnectionType::Underground));

        assert_eq!(path[12].address, NodeAddress::Local(139));
        assert_eq!(path[12].relation, Some(ConnectionType::Underground));

        assert_eq!(path[13].address, NodeAddress::Local(88));
        assert_eq!(path[13].relation, Some(ConnectionType::Underground));

        assert_eq!(path[14].address, NodeAddress::Local(127));
        assert_eq!(path[14].relation, Some(ConnectionType::Underground));

        assert_eq!(path[15].address, NodeAddress::Local(198));
        assert_eq!(path[15].relation, Some(ConnectionType::Bus));
    }
}
