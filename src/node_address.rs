#[derive(Debug, Clone, Eq, PartialEq)]
pub enum NodeAddress {
    Local(usize),
}

impl NodeAddress {
    pub const fn from_local(node_id: usize) -> Self {
        NodeAddress::Local(node_id)
    }
}
