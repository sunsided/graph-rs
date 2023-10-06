use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum NodeAddress {
    Local(usize),
}

impl NodeAddress {
    pub const fn from_local(node_id: usize) -> Self {
        NodeAddress::Local(node_id)
    }
}

impl Display for NodeAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeAddress::Local(id) => write!(f, "local node #{id}"),
        }
    }
}

impl Hash for NodeAddress {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            NodeAddress::Local(id) => id.hash(state),
        }
    }
}
