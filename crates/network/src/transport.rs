use libp2p::{
    bandwidth::BandwidthSinks,
    core::{muxing::StreamMuxerBox, transport::Boxed, upgrade},
    dns,
    identity::Keypair,
    noise, tcp, PeerId, Transport, TransportExt,
};
use std::{sync::Arc, time::Duration};

/// New transport with bandwidth logging. (TCP)
pub fn new_transport(
    local_key: Keypair,
    yamux_window_size: Option<u32>,
    yamux_maximum_buffer_size: usize,
) -> std::io::Result<(Boxed<(PeerId, StreamMuxerBox)>, Arc<BandwidthSinks>)> {
    let tcp_config = tcp::Config::new().nodelay(true);
    let tcp_trans = tcp::tokio::Transport::new(tcp_config.clone());
    let transport = dns::TokioDnsConfig::system(tcp_trans)?;

    #[cfg(feature = "libp2p-websocket")]
    let transport = {
        let trans_clone = transport.clone();
        transport.or_transport(libp2p::websocket::WsConfig::new(trans_clone))
    };

    let authentication_config =
        noise::Config::new(&local_key).expect("Can create noise config. qed");

    let multiplexing_config = {
        let mut yamux_config = libp2p::yamux::Config::default();
        // Enable proper flow-control: window updates are only sent when
        // buffered data has been consumed.
        yamux_config.set_window_update_mode(libp2p::yamux::WindowUpdateMode::on_read());
        yamux_config.set_max_buffer_size(yamux_maximum_buffer_size);

        if let Some(yamux_window_size) = yamux_window_size {
            yamux_config.set_receive_window_size(yamux_window_size);
        }

        yamux_config
    };

    let transport = transport
        .upgrade(upgrade::Version::V1Lazy)
        .authenticate(authentication_config)
        .multiplex(multiplexing_config)
        .timeout(Duration::from_secs(20))
        .boxed();

    let (transport, sink) = transport.with_bandwidth_logging();

    Ok((transport, sink))
}
