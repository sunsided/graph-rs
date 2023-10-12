mod astar;
mod bfs;
mod dfs;
mod examples;
mod node_address;

pub use crate::bfs::BreadthFirstSearch;
pub use crate::dfs::DepthFirstSearch;
use crate::node_address::NodeAddress;
use std::borrow::Borrow;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct Graph<T, R> {
    nodes: Vec<Node<T, R>>,
}

#[derive(Debug)]
#[cfg(feature = "boxed-nodes")]
struct Node<T, R>(Box<NodeData<T, R>>);

#[cfg(not(feature = "boxed-nodes"))]
struct Node<T>(NodeData<T>);

#[derive(Debug, Clone)]
struct NodeRelation<R> {
    relation: R,
    address: NodeAddress,
}

#[derive(Debug, Clone)]
pub struct NodePathLink<R>
where
    R: Clone,
{
    /// The type of relation. This value is virtually always set, except for pathfinding
    /// cases where the start node does not have a means of "getting to it".
    relation: Option<R>,
    /// The address of the target node.
    address: NodeAddress,
}

impl<R> From<&NodeRelation<R>> for NodePathLink<R>
where
    R: Clone,
{
    fn from(value: &NodeRelation<R>) -> Self {
        NodePathLink {
            relation: Some(value.relation.clone()),
            address: value.address.clone(),
        }
    }
}

impl<T, R> From<NodeData<T, R>> for Node<T, R> {
    fn from(value: NodeData<T, R>) -> Self {
        #[cfg(feature = "boxed-nodes")]
        {
            Self(Box::new(value))
        }
        #[cfg(not(feature = "boxed-nodes"))]
        {
            Self(value)
        }
    }
}

impl<T, R> Deref for Node<T, R> {
    type Target = NodeData<T, R>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, R> DerefMut for Node<T, R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct NodeData<T, R> {
    id: usize,
    data: T,
    // TODO: Use array-backed lists for this? Depends on the connectivity.
    outgoing: Vec<NodeRelation<R>>,
}

impl<T, R> Graph<T, R> {
    pub fn add(&mut self, data: T) -> NodeAddress {
        let id = self.nodes.len();
        let node = NodeData {
            id,
            data,
            outgoing: Vec::new(),
        };
        self.nodes.push(node.into());
        NodeAddress::from_local(id)
    }

    /// Gets a node given its [`NodeAddress`] under the condition that the node is locally available.
    fn get_local_node_ref(
        &self,
        address: &NodeAddress,
    ) -> Result<&NodeData<T, R>, NodeAddressError> {
        #[allow(unreachable_patterns)]
        match address {
            NodeAddress::Local(id) => Ok(&self.nodes[*id]),
            _ => Err(NodeAddressError::NodeNotLocal(address.clone())),
        }
    }

    /// Creates a connection between the nodes at the `from` address and the `to` address.
    ///
    /// * The `from` node will receive an outgoing connection to the `to` node.
    /// * The `to` node will receive an incoming connection from the `from` node.
    ///
    /// ## Arguments
    /// * `from` - The node from which to link to another node.
    /// * `to` The node to link to.
    /// * `relation` - The type of relation.
    pub fn link_to<A: Borrow<NodeAddress>>(&mut self, from: A, to: A, relation: R) {
        let from = from.borrow();
        let to = to.borrow();
        match from {
            NodeAddress::Local(from_idx) => self.nodes[*from_idx].outgoing.push(NodeRelation {
                relation,
                address: to.clone(),
            }),
        }
    }

    /// Creates a bidirectional connection between the nodes at the `from` address and the `to` address.
    ///
    /// * The `from` node will receive an outgoing connection to the `to` node and vice versa.
    /// * The `to` node will receive an incoming connection from the `from` node and vice versa.
    ///
    /// ## Arguments
    /// * `from` - The node from which to link to another node.
    /// * `to` The node to link to.
    /// * `relation` - The type of relation.
    pub fn link_bidir<A: Borrow<NodeAddress>>(&mut self, from: A, to: A, relation: R)
    where
        R: Clone,
    {
        self.link_to(from.borrow(), to.borrow(), relation.clone());
        self.link_to(to.borrow(), from.borrow(), relation.clone());
    }
}

impl<T, R> Default for Graph<T, R> {
    fn default() -> Self {
        Graph { nodes: Vec::new() }
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum NodeAddressError {
    #[error("The specified node address does not represent a local node: {0}")]
    NodeNotLocal(NodeAddress),
}

#[cfg(test)]
mod tests {
    use super::examples::london_graph::*;
    use super::examples::movie_graph::*;

    #[test]
    #[cfg(feature = "examples-movies")]
    fn movie_examples() {
        let graph = movie_graph();
        println!("{:?}", graph);
    }

    #[test]
    #[cfg(feature = "examples-london")]
    fn london_examples() {
        let graph = london_graph();
        println!("{:?}", graph);
    }
}
