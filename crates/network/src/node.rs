#[allow(dead_code)]
use libp2p::{identity::Keypair, Multiaddr, PeerId};
use std::collections::HashSet;
use serde::{Deserialize, Serialize};


// NodeType defines the type of node
#[derive(Clone, Copy, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub enum NodeType {
  // BootstrapNode is a node that is used to bootstrap the network, it can have a higher amount of peers connected 
  BootstrapNode,
  // Fullnode defines a peer that is participating in the network (Consensus, etc..)
  FullNode,
}

impl Default for NodeType {
  fn default() -> Self {
    NodeType::FullNode
  }
}


// Config defines the configuration for a node on 
#[derive(Clone, Default, Debug, )]
pub struct NodeConfig {
  // Specifies the type of node
  pub node_type: NodeType,
  // Specifies the identity of the node
  pub identity: Option<Keypair>,
  // Specifies the multiaddress of the node
  pub multi_addr: Option<Multiaddr>,
  // peers is a list of peers to connect to
  pub peers: HashSet<(Option<PeerId>, Multiaddr)>
}
