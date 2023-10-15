//! Provides a property graph type that embeds node relations into the node itself.

use crate::errors::NodeAddressError;
use crate::node_address::NodeAddress;
use crate::node_relation::NodeRelation;
use std::borrow::Borrow;
use std::ops::{Deref, DerefMut};

/// A property graph type that embeds node relations into the node itself.
#[derive(Debug)]
pub struct EmbeddedPropertyGraph<T, R> {
    nodes: Vec<Node<T, R>>,
}

#[derive(Debug)]
#[cfg(feature = "boxed-nodes")]
struct Node<T, R>(Box<NodeData<T, R>>);

#[cfg(not(feature = "boxed-nodes"))]
struct Node<T>(NodeData<T>);

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
pub(crate) struct NodeData<T, R> {
    pub(crate) id: usize,
    pub(crate) data: T,
    // TODO: Use array-backed lists for this? Depends on the connectivity.
    pub(crate) outgoing: Vec<NodeRelation<R>>,
}

impl<T, R> EmbeddedPropertyGraph<T, R> {
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
    pub(crate) fn get_local_node_ref(
        &self,
        address: &NodeAddress,
    ) -> Result<&NodeData<T, R>, NodeAddressError> {
        #[allow(unreachable_patterns)]
        match address {
            NodeAddress::Local(id) => Ok(&self.nodes[*id]),
            _ => Err(NodeAddressError::NodeNotLocal(address.clone())),
        }
    }

    /// Gets a node's data given its [`NodeAddress`] under the condition that the node is locally available.
    pub(crate) fn get_local_node_data_ref(
        &self,
        address: &NodeAddress,
    ) -> Result<&T, NodeAddressError> {
        #[allow(unreachable_patterns)]
        match address {
            NodeAddress::Local(id) => Ok(&self.nodes[*id].data),
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
        self.link_to(to.borrow(), from.borrow(), relation);
    }
}

impl<T, R> Default for EmbeddedPropertyGraph<T, R> {
    fn default() -> Self {
        EmbeddedPropertyGraph { nodes: Vec::new() }
    }
}

#[cfg(test)]
mod tests {
    use crate::examples::london_graph::*;
    use crate::examples::movie_graph::*;

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
