use async_std::io;
use futures::{prelude::*, select};
use libp2p::core::upgrade::Version;
use libp2p::kad::record::store::MemoryStore;
use libp2p::kad::{
    record::Key, AddProviderOk, GetProvidersOk, GetRecordOk, Kademlia, KademliaEvent, PeerRecord,
    PutRecordOk, QueryResult, Quorum, Record,
};
use libp2p::{
    identity, mdns, noise, quic,
    swarm::{NetworkBehaviour, SwarmBuilder, SwarmEvent},
    tcp, yamux, PeerId, Transport,
};
use std::error::Error;

pub struct NetworkService {}

impl NetworkService {
    async fn start(pid: PeerId, key: identity::Keypair) -> Result<(), Box<dyn Error>> {
        let transport_quic = quic::async_std::Transport::default()
            .upgrade(Version::V1)
            .authenticate(noise::Config::new(&local_key)?)
            .multiplex(yamux::Config::default())
            .boxed();
    }
}
