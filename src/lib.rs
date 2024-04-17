pub use libp2p;
use libp2p::{
    identify, identity::Keypair, noise, ping, swarm::NetworkBehaviour, tcp, yamux, Multiaddr,
    Swarm, SwarmBuilder,
};
use log::debug;
use std::{
    error::Error,
    sync::{Arc, Weak},
};
use tokio::sync::Mutex;

pub fn init_debug_interface() {
    let _ = env_logger::init();
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .try_init();

    debug!("Debug interface has been initialized!");
}

pub struct Phylosopher {
    swarm: Arc<Mutex<Swarm<Phylosophy>>>,
}

impl Phylosopher {
    /// Create a new node.
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

        #[cfg(not(any(feature = "user", feature = "relay", feature = "hub", feature = "data")))]
        let swarm = swarm_builder
            .with_behaviour(|_key| Phylosophy { ping, protocol })?
            .build();

        #[cfg(feature = "user")]
        let swarm = swarm_builder
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
            .build();

        #[cfg(feature = "relay")]
        let swarm = swarm_builder
            .with_behaviour(|_key| Phylosophy {
                ping,
                protocol,
                rzv,
                relay,
            })?
            .build();

        #[cfg(feature = "hub")]
        let swarm = swarm_builder
            .with_behaviour(|_key| Phylosophy {
                ping,
                protocol,
                rzv,
            })?
            .build();

        #[cfg(feature = "data")]
        let swarm = swarm_builder
            .with_behaviour(|_key| Phylosophy {
                ping,
                protocol,
                rzv,
                kad,
            })?
            .build();

        Ok(Self {
            swarm: Arc::new(Mutex::new(swarm)),
        })
    }

    /// Bind a listening address to the swarm.
    pub async fn bind(&self, address: Option<Multiaddr>) {
        let swarm = self.get_swarm();
        let mut swarm = swarm.lock().await;

        swarm
            .listen_on(address.unwrap_or_else(|| "/ip4/0.0.0.0/tcp/0".parse().unwrap()))
            .expect("Failed to bind a listening address!");
    }

    /// Get a strong reference to the swarm mutex.
    pub fn get_swarm(&self) -> Arc<Mutex<Swarm<Phylosophy>>> {
        Arc::clone(&self.swarm)
    }

    /// Get a weak reference to the swar mutex.
    pub fn get_swarm_weak(&self) -> Weak<Mutex<Swarm<Phylosophy>>> {
        Arc::downgrade(&self.swarm)
    }
}

#[derive(NetworkBehaviour)]
#[behaviour(to_swarm = "Logic")]
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

unsafe impl Send for Phylosophy {}

#[derive(Debug)]
pub enum Logic {
    Ping(ping::Event),
    Protocol(identify::Event),

    #[cfg(feature = "user")]
    Mdns(libp2p::mdns::Event),

    #[cfg(feature = "user")]
    ClientRelay(libp2p::relay::client::Event),

    #[cfg(feature = "user")]
    Dcutr(libp2p::dcutr::Event),

    #[cfg(any(feature = "user", feature = "relay", feature = "data"))]
    ClientRzv(libp2p::rendezvous::client::Event),

    #[cfg(any(feature = "user", feature = "data"))]
    Kad(libp2p::kad::Event),

    #[cfg(feature = "hub")]
    ServerRzv(libp2p::rendezvous::server::Event),

    #[cfg(feature = "relay")]
    ServerRelay(libp2p::relay::Event),
}

impl From<ping::Event> for Logic {
    fn from(value: ping::Event) -> Self {
        Self::Ping(value)
    }
}
impl From<identify::Event> for Logic {
    fn from(value: identify::Event) -> Self {
        Self::Protocol(value)
    }
}

#[cfg(feature = "user")]
impl From<libp2p::mdns::Event> for Logic {
    fn from(value: libp2p::mdns::Event) -> Self {
        Self::Mdns(value)
    }
}

#[cfg(feature = "user")]
impl From<libp2p::relay::client::Event> for Logic {
    fn from(value: libp2p::relay::client::Event) -> Self {
        Self::ClientRelay(value)
    }
}

#[cfg(feature = "user")]
impl From<libp2p::dcutr::Event> for Logic {
    fn from(value: libp2p::dcutr::Event) -> Self {
        Self::Dcutr(value)
    }
}

#[cfg(any(feature = "user", feature = "relay", feature = "data"))]
impl From<libp2p::rendezvous::client::Event> for Logic {
    fn from(value: libp2p::rendezvous::client::Event) -> Self {
        Self::ClientRzv(value)
    }
}

#[cfg(any(feature = "user", feature = "data"))]
impl From<libp2p::kad::Event> for Logic {
    fn from(value: libp2p::kad::Event) -> Self {
        Self::Kad(value)
    }
}

#[cfg(feature = "hub")]
impl From<libp2p::rendezvous::server::Event> for Logic {
    fn from(value: libp2p::rendezvous::server::Event) -> Self {
        Self::ServerRzv(value)
    }
}

#[cfg(feature = "relay")]
impl From<libp2p::relay::Event> for Logic {
    fn from(value: libp2p::relay::Event) -> Self {
        Self::ServerRelay(value)
    }
}
