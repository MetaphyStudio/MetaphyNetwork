use std::error::Error;

use futures::StreamExt;
pub use libp2p;
use libp2p::{
    core::transport::ListenerId, identify, identity::Keypair, noise, ping, swarm::NetworkBehaviour,
    tcp, yamux, Swarm, SwarmBuilder,
};
use log::{info, warn};

pub fn init_debug_interface() {
    let _ = env_logger::init();
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .try_init();
}

pub struct Phylosopher {
    id: Keypair,
    swarm: Option<Swarm<Phylosophy>>,
}

impl Phylosopher {
    pub fn new(keygen: Option<Keypair>) -> Result<Self, Box<dyn Error>> {
        let id = match keygen {
            Some(id) => id,
            None => Keypair::generate_ed25519(),
        };

        let ping = ping::Behaviour::new(ping::Config::default());
        let protocol = identify::Behaviour::new(identify::Config::new(
            "/Phylosophy/0.1.0".into(),
            id.public(),
        ));

        #[cfg(feature = "user")]
        let mdns = libp2p::mdns::tokio::Behaviour::new(
            libp2p::mdns::Config::default(),
            id.public().to_peer_id(),
        )?;

        #[cfg(feature = "user")]
        let dcutr = libp2p::dcutr::Behaviour::new(id.public().to_peer_id());

        #[cfg(any(feature = "user", feature = "relay", feature = "data"))]
        let rzv = libp2p::rendezvous::client::Behaviour::new(id.clone());

        #[cfg(any(feature = "user", feature = "data"))]
        let kad = libp2p::kad::Behaviour::new(
            id.public().to_peer_id(),
            libp2p::kad::store::MemoryStore::new(id.public().to_peer_id()),
        );

        #[cfg(feature = "hub")]
        let rzv = libp2p::rendezvous::server::Behaviour::new(
            libp2p::rendezvous::server::Config::default(),
        );

        #[cfg(feature = "relay")]
        let relay = libp2p::relay::Behaviour::new(
            id.public().to_peer_id(),
            libp2p::relay::Config::default(),
        );

        let swarm_builder = SwarmBuilder::with_existing_identity(id.clone())
            .with_tokio()
            .with_tcp(
                tcp::Config::default(),
                noise::Config::new,
                yamux::Config::default,
            )?;

        let swarm: Option<Swarm<Phylosophy>> = None;

        #[cfg(feature = "user")]
        let swarm = Some(
            swarm_builder
                .with_relay_client(noise::Config::new, yamux::Config::default)?
                .with_behaviour(|_key, relay| Phylosophy {
                    ping,
                    protocol,
                    mdns,
                    relay,
                    dcutr,
                    rzv,
                    kad,
                })?
                .build(),
        );

        #[cfg(feature = "relay")]
        let swarm = Some(
            swarm_builder
                .with_behaviour(|_key| Phylosophy {
                    ping,
                    protocol,
                    rzv,
                    relay,
                })?
                .build(),
        );

        #[cfg(feature = "hub")]
        let swarm = Some(
            swarm_builder
                .with_behaviour(|_key| Phylosophy {
                    ping,
                    protocol,
                    rzv,
                })?
                .build(),
        );

        #[cfg(feature = "data")]
        let swarm = Some(
            swarm_builder
                .with_behaviour(|_key| Phylosophy {
                    ping,
                    protocol,
                    rzv,
                    kad,
                })?
                .build(),
        );

        Ok(Self { id, swarm })
    }

    pub fn listen(&mut self) -> Result<ListenerId, Box<dyn Error>> {
        Ok(self.get_swarm().listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?)
    }

    pub async fn poll(&mut self) -> Option<PhylosophyEvent> {
        match self.get_swarm().select_next_some().await {
            libp2p::swarm::SwarmEvent::Behaviour(event) => Some(event),
            libp2p::swarm::SwarmEvent::NewListenAddr { listener_id: _, address } => {
                info!("New listen address -> {address}");
                None
            },
            _ => {
                warn!("Unhandled swarm event occured...");
                None
            }
        }
    }

    pub fn get_id(&self) -> &Keypair {
        &self.id
    }

    pub fn get_swarm(&mut self) -> &mut Swarm<Phylosophy> {
        self.swarm.as_mut().expect("No swarm found!")
    }
}

#[derive(NetworkBehaviour)]
pub struct Phylosophy {
    ping: ping::Behaviour,
    protocol: identify::Behaviour,

    #[cfg(feature = "user")]
    mdns: libp2p::mdns::tokio::Behaviour,

    #[cfg(feature = "user")]
    relay: libp2p::relay::client::Behaviour,

    #[cfg(feature = "user")]
    dcutr: libp2p::dcutr::Behaviour,

    #[cfg(any(feature = "user", feature = "relay", feature = "data"))]
    rzv: libp2p::rendezvous::client::Behaviour,

    #[cfg(any(feature = "user", feature = "data"))]
    kad: libp2p::kad::Behaviour<libp2p::kad::store::MemoryStore>,

    #[cfg(feature = "hub")]
    rzv: libp2p::rendezvous::server::Behaviour,

    #[cfg(feature = "relay")]
    relay: libp2p::relay::Behaviour,
}
