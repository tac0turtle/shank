use super::score::PeerScore;
pub use libp2p::core::Multiaddr;
use serde::{
    ser::{SerializeStruct, Serializer},
    Serialize,
};
use std::collections::HashSet;
use std::net::{IpAddr, SocketAddr};
use std::time::Instant;

/// Information about a given connected peer.
#[derive(Clone, Debug, Serialize)]
pub struct PeerInfo {
    /// The peers reputation, represented as a integer
    score: PeerScore,
    /// Connection status of this peer
    connection_status: PeerConnectionStatus,
    /// The known listening addresses of this peer. This is given by identify and can be arbitrary
    /// (including local IPs).
    listening_addresses: Vec<Multiaddr>,
    /// This is addresses we have physically seen and this is what we use for banning/un-banning
    /// peers.
    seen_addresses: HashSet<SocketAddr>,
    /// Is the peer a trusted peer.
    is_trusted: bool,
    /// Direction of the first connection of the last (or current) connected session with this peer.
    /// None if this peer was never connected.
    connection_direction: Option<ConnectionDirection>,
}

impl Default for PeerInfo {
    fn default() -> Self {
        PeerInfo {
            score: PeerScore::default(),
            connection_status: PeerConnectionStatus::Unknown,
            listening_addresses: Vec::new(),
            seen_addresses: HashSet::new(),
            is_trusted: false,
            connection_direction: None,
        }
    }
}

impl PeerInfo {
    pub fn trusted_peer() -> Self {
        PeerInfo {
            score: PeerScore::max(),
            connection_status: PeerConnectionStatus::Unknown,
            listening_addresses: Vec::new(),
            seen_addresses: HashSet::new(),
            is_trusted: true,
            connection_direction: None,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum ConnectionDirection {
    /// The connection was established by a peer dialing us.
    Incoming,
    /// The connection was established by us dialing a peer.
    Outgoing,
}

/// Connection Status of the peer.
#[derive(Debug, Clone, Default)]
pub enum PeerConnectionStatus {
    /// The peer is connected.
    Connected {
        /// number of ingoing connections.
        n_in: u8,
        /// number of outgoing connections.
        n_out: u8,
    },
    /// The peer is being disconnected.
    Disconnecting {
        // After the disconnection the peer will be considered banned.
        to_ban: bool,
    },
    /// The peer has disconnected.
    Disconnected {
        /// last time the peer was connected or discovered.
        since: Instant,
    },
    /// The peer has been banned and is disconnected.
    Banned {
        /// moment when the peer was banned.
        since: Instant,
    },
    /// We are currently dialing this peer.
    Dialing {
        /// time since we last communicated with the peer.
        since: Instant,
    },
    /// The connection status has not been specified.
    #[default]
    Unknown,
}

/// Serialization for http requests.
impl Serialize for PeerConnectionStatus {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_struct("connection_status", 6)?;
        match self {
            PeerConnectionStatus::Connected { n_in, n_out } => {
                s.serialize_field("status", "connected")?;
                s.serialize_field("connections_in", n_in)?;
                s.serialize_field("connections_out", n_out)?;
                s.serialize_field("last_seen", &0)?;
                s.end()
            }
            PeerConnectionStatus::Disconnecting { .. } => {
                s.serialize_field("status", "disconnecting")?;
                s.serialize_field("connections_in", &0)?;
                s.serialize_field("connections_out", &0)?;
                s.serialize_field("last_seen", &0)?;
                s.end()
            }
            PeerConnectionStatus::Disconnected { since } => {
                s.serialize_field("status", "disconnected")?;
                s.serialize_field("connections_in", &0)?;
                s.serialize_field("connections_out", &0)?;
                s.serialize_field("last_seen", &since.elapsed().as_secs())?;
                s.serialize_field("banned_ips", &Vec::<IpAddr>::new())?;
                s.end()
            }
            PeerConnectionStatus::Banned { since } => {
                s.serialize_field("status", "banned")?;
                s.serialize_field("connections_in", &0)?;
                s.serialize_field("connections_out", &0)?;
                s.serialize_field("last_seen", &since.elapsed().as_secs())?;
                s.end()
            }
            PeerConnectionStatus::Dialing { since } => {
                s.serialize_field("status", "dialing")?;
                s.serialize_field("connections_in", &0)?;
                s.serialize_field("connections_out", &0)?;
                s.serialize_field("last_seen", &since.elapsed().as_secs())?;
                s.end()
            }
            PeerConnectionStatus::Unknown => {
                s.serialize_field("status", "unknown")?;
                s.serialize_field("connections_in", &0)?;
                s.serialize_field("connections_out", &0)?;
                s.serialize_field("last_seen", &0)?;
                s.end()
            }
        }
    }
}
