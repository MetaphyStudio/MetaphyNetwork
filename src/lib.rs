pub use libp2p;

pub fn init_debug_interface() {
    let _ = env_logger::init();
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .try_init();
}

#[cfg(feature = "user")]
pub mod user {
    use std::{collections::VecDeque, error::Error, time::Duration};

    use futures::StreamExt;
    use libp2p::{
        core::transport::ListenerId, identity::Keypair, mdns, noise, ping, relay,
        swarm::NetworkBehaviour, tcp, yamux, Swarm, SwarmBuilder,
    };
    use log::{error, info, warn};

    pub struct Node {
        pub id: libp2p::identity::Keypair,
        pub swarm: Swarm<Behaviour>,
    }

    impl Node {
        /// This creates a "default" struct for type `Node` that will be used
        /// as a user node for creating and accessing data and instances on the network.
        pub fn new() -> Result<Self, Box<dyn Error>> {
            let id = Keypair::generate_ed25519();
            let mdns =
                mdns::tokio::Behaviour::new(mdns::Config::default(), id.public().to_peer_id())?;
            let swarm = SwarmBuilder::with_existing_identity(id.clone())
                .with_tokio()
                .with_tcp(
                    tcp::Config::default(),
                    noise::Config::new,
                    yamux::Config::default,
                )?
                .with_relay_client(noise::Config::new, yamux::Config::default)?
                .with_behaviour(|_key, relay| Behaviour {
                    ping: ping::Behaviour::default(),
                    relay,
                    mdns,
                })?
                .with_swarm_config(|conf| {
                    conf.with_idle_connection_timeout(Duration::from_secs(60))
                })
                .build();

            Ok(Self { id, swarm })
        }

        pub fn listen(&mut self) -> Result<ListenerId, Box<dyn Error>> {
            let listen = self.swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
            Ok(listen)
        }

        /// This will run once, going through all swarm events and storing behaviour events in a `VecDeque<BehaviourEvent>`.
        pub async fn run_once(&mut self, events: &mut VecDeque<BehaviourEvent>) {
            match self.swarm.select_next_some().await {
                libp2p::swarm::SwarmEvent::Behaviour(e) => events.push_front(e),
                libp2p::swarm::SwarmEvent::ConnectionEstablished {
                    peer_id,
                    connection_id,
                    endpoint,
                    num_established: _,
                    concurrent_dial_errors,
                    established_in,
                } => info!("Connection Established...\nPeer: {peer_id}\nConnection: {connection_id}\nEndpoint: {endpoint:?}\nDial Errors: {concurrent_dial_errors:?}\nTime to establish: {established_in:?}\n"),
                libp2p::swarm::SwarmEvent::ConnectionClosed {
                    peer_id,
                    connection_id,
                    endpoint,
                    num_established: _,
                    cause,
                } => warn!("Connection Closed...\nPeer: {peer_id}\nConnection: {connection_id}\nEndpoint: {endpoint:?}\nCause: {}\n", cause.unwrap()),
                libp2p::swarm::SwarmEvent::IncomingConnection {
                    connection_id,
                    local_addr,
                    send_back_addr,
                } => info!("Incoming Connection...\nConnection: {connection_id}\nLocal Address: {local_addr}\nReturn Address: {send_back_addr}\n"),
                libp2p::swarm::SwarmEvent::IncomingConnectionError {
                    connection_id,
                    local_addr,
                    send_back_addr,
                    error,
                } => error!("Incoming Connection Error!\nConnection: {connection_id}\nLocal Address: {local_addr}\nReturn Address: {send_back_addr}\nError: {error}\n"),
                libp2p::swarm::SwarmEvent::OutgoingConnectionError {
                    connection_id,
                    peer_id,
                    error,
                } => error!("Outgoing Connection Error!\nConnection: {connection_id}\nPeer: {}\nError: {error}\n", peer_id.unwrap()),
                libp2p::swarm::SwarmEvent::NewListenAddr {
                    listener_id,
                    address,
                } => info!("New Listen Address...\nListener ID: {listener_id}\nAddress: {address}\n"),
                libp2p::swarm::SwarmEvent::ExpiredListenAddr {
                    listener_id,
                    address,
                } => warn!("Expired Listen Address...\nListener ID: {listener_id}\nAddress: {address}\n"),
                libp2p::swarm::SwarmEvent::ListenerClosed {
                    listener_id,
                    addresses,
                    reason: _,
                } => warn!("Listener Closed...\nListener ID: {listener_id}\nAddresses: {addresses:?}\n"),
                libp2p::swarm::SwarmEvent::ListenerError { listener_id, error } => error!("Listener Error!\nListener ID: {listener_id}\nError: {error}\n"),
                libp2p::swarm::SwarmEvent::Dialing {
                    peer_id,
                    connection_id,
                } => info!("Dialing...\nPeer: {}\nConnection ID: {connection_id}\n", peer_id.unwrap()),
                libp2p::swarm::SwarmEvent::NewExternalAddrCandidate { address } => info!("New External Address Candidate...\nAddress: {address}\n"),
                libp2p::swarm::SwarmEvent::ExternalAddrConfirmed { address } => info!("External Address Confirmed...\nAddress: {address}\n"),
                libp2p::swarm::SwarmEvent::ExternalAddrExpired { address } => warn!("External Address Expired...\nAddress: {address}\n"),
                _ => warn!("Unimplemented behaviour has occured, this may not effect network behaviour, but something happened and I don't know what that thing was...\n"),
            }
        }
    }

    #[derive(NetworkBehaviour)]
    pub struct Behaviour {
        ping: ping::Behaviour,
        relay: relay::client::Behaviour,
        mdns: mdns::tokio::Behaviour,
    }

    pub enum Events {
        Ping(ping::Event),
        Relay(relay::client::Event),
        Mdns(mdns::Event),
    }

    impl From<ping::Event> for Events {
        fn from(value: ping::Event) -> Self {
            Self::Ping(value)
        }
    }

    impl From<relay::client::Event> for Events {
        fn from(value: relay::client::Event) -> Self {
            Self::Relay(value)
        }
    }

    impl From<libp2p::mdns::Event> for Events {
        fn from(value: libp2p::mdns::Event) -> Self {
            Self::Mdns(value)
        }
    }
}

#[cfg(feature = "relay")]
pub mod relay {}

#[cfg(feature = "hub")]
pub mod hub {}

#[cfg(feature = "data")]
pub mod data {}
