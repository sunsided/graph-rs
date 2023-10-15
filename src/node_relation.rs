//! Provides the [`NodePathLink`] and [`NodeRelation`] types.

use crate::node_address::NodeAddress;

/// Describes a path link between two nodes used during graph traversals.
/// It describes a node, as well as the relation that was traversed in order to get to it.
#[derive(Debug, Clone)]
pub struct NodePathLink<R>
where
    R: Clone,
{
    /// The type of relation. This value is virtually always set, except for pathfinding
    /// cases where the start node does not have a means of "getting to it".
    pub relation: Option<R>,
    /// The address of the target node.
    pub address: NodeAddress,
}

/// An internal representation of a node relation.
#[derive(Debug, Clone)]
pub struct NodeRelation<R> {
    /// The relation that leads to the addressed node.
    pub relation: R,
    /// The address of the targeted node.
    pub address: NodeAddress,
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
