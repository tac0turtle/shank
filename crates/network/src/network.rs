/*
network module defines the instantisation of the network
*/

use libp2p::{PeerId,  identity::Keypair,};

use crate::node::NodeConfig;
use crate::error::NetworkError;


// Network is the main struct for the network
pub struct Network {
    pub node_config: NodeConfig,
    pub peer_id: PeerId,
}



// NewNetwork defines a new instance of a network
impl Network {
    #[allow(dead_code)]
    pub fn new(node_config: NodeConfig) -> Result<Self, NetworkError> {
        let identity = if let Some(ref kp) = node_config.identity {
            kp.clone()
        } else {
            Keypair::generate_ed25519()
        };
        let peer_id = PeerId::from(identity.public());

        Ok(Network {
            node_config,
            peer_id
        })
    }

}
