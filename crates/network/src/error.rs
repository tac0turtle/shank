use libp2p::swarm::DialError;

pub enum NetworkError {
  DialError{
    error: DialError
  },
}
