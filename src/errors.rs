use crate::node_address::NodeAddress;

#[derive(Debug, Clone, thiserror::Error)]
pub enum NodeAddressError {
    #[error("The specified node address does not represent a local node: {0}")]
    NodeNotLocal(NodeAddress),
}
