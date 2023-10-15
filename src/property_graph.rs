//! Provides a property graph type.

use crate::errors::NodeAddressError;
use crate::node_address::NodeAddress;
use crate::node_relation::NodeRelation;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

/// A property graph type.
#[derive(Debug)]
pub struct PropertyGraph<T, R> {
    nodes: Vec<Node<T>>,
    // TODO: Use another vector here?
    // TODO: Use array-backed lists for this? Depends on the connectivity.
    outgoing: HashMap<usize, Vec<NodeRelation<R>>>,
}

#[derive(Debug)]
#[cfg(feature = "boxed-nodes")]
struct Node<T>(Box<NodeData<T>>);

#[cfg(not(feature = "boxed-nodes"))]
struct Node<T>(NodeData<T>);

impl<T> From<NodeData<T>> for Node<T> {
    fn from(value: NodeData<T>) -> Self {
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

impl<T> Deref for Node<T> {
    type Target = NodeData<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Node<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) struct NodeData<T> {
    pub(crate) id: usize,
    pub(crate) data: T,
}

impl<T, R> PropertyGraph<T, R> {
    pub fn add(&mut self, data: T) -> NodeAddress {
        let id = self.nodes.len();
        let node = NodeData { id, data };
        self.nodes.push(node.into());
        self.outgoing.insert(id, Vec::new());
        let address = NodeAddress::from_local(id);
        address
    }

    /// Iterates the neighbors of a node given its [`NodeAddress`] under the condition that the node is locally available.
    pub fn iter_local_neighbors(
        &self,
        address: &NodeAddress,
    ) -> Result<impl Iterator<Item = &NodeRelation<R>>, NodeAddressError> {
        #[allow(unreachable_patterns)]
        match address {
            NodeAddress::Local(id) => {
                if let Some(vec) = self.outgoing.get(id) {
                    Ok(vec.iter())
                } else {
                    todo!()
                }
            }
            _ => Err(NodeAddressError::NodeNotLocal(address.clone())),
        }
    }

    /// Gets a node's data given its [`NodeAddress`] under the condition that the node is locally available.
    pub(crate) fn local_node_data_ref(
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
            NodeAddress::Local(from_idx) => {
                let entry = self.outgoing.entry(*from_idx).or_insert(Vec::new());
                entry.push(NodeRelation {
                    relation,
                    address: to.clone(),
                });
            }
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

impl<T, R> Default for PropertyGraph<T, R> {
    fn default() -> Self {
        PropertyGraph {
            nodes: Vec::new(),
            outgoing: HashMap::default(),
        }
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
