use libp2p::{
    identity, mdns, noise, quic,
    swarm::{NetworkBehaviour, SwarmBuilder, SwarmEvent},
    tcp, yamux, PeerId, Transport,
};

pub struct NetworkConfig {
    PeerId: PeerId,
    Identity: identity::Keypair,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        NetworkConfig {
            PeerId: local_peer_id,
            Identity: local_key,
        }
    }
}
