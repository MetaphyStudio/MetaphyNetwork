use std::{error::Error, time::Duration};

use async_std::stream::StreamExt;
use libp2p::{core::transport::ListenerId, swarm::NetworkBehaviour, Multiaddr};

pub struct Client {
    id: libp2p::identity::Keypair,
    swarm: libp2p::Swarm<Behaviour>,
}

impl Client {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let id = libp2p::identity::Keypair::generate_ed25519();

        let swarm = libp2p::SwarmBuilder::with_existing_identity(id.clone())
            .with_async_std()
            .with_tcp(
                libp2p::tcp::Config::default(),
                libp2p::noise::Config::new,
                libp2p::yamux::Config::default,
            )?
            .with_relay_client(libp2p::noise::Config::new, libp2p::yamux::Config::default)?
            .with_behaviour(|key, relay| Behaviour::new(key.public(), relay))?
            .build();

        Ok(Self { id, swarm })
    }

    pub fn listen(&mut self, multiaddr: Option<Multiaddr>) -> Result<ListenerId, Box<dyn Error>> {
        let id = match multiaddr {
            Some(addr) => self.swarm.listen_on(addr)?,
            None => self.swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?,
        };
        Ok(id)
    }

    pub async fn run_loop(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            self.poll().await?
        }
    }

    pub async fn poll(&mut self) -> Result<(), Box<dyn Error>> {
        match self.swarm.next().await.unwrap() {
            libp2p::swarm::SwarmEvent::NewListenAddr {
                listener_id,
                address,
            } => println!("Swarm {listener_id}, listening on: {address}"),
            libp2p::swarm::SwarmEvent::Behaviour(e) => println!("{e:?}"),
            _ => todo!(),
        }

        Ok(())
    }
}

#[derive(NetworkBehaviour)]
#[behaviour(to_swarm = "Event")]
pub struct Behaviour {
    ping: libp2p::ping::Behaviour,
    identify: libp2p::identify::Behaviour,
    relay_client: libp2p::relay::client::Behaviour,
    dcutr: libp2p::dcutr::Behaviour,
}

impl Behaviour {
    pub fn new(
        key: libp2p::identity::PublicKey,
        relay_client: libp2p::relay::client::Behaviour,
    ) -> Self {
        Self {
            ping: libp2p::ping::Behaviour::new(
                libp2p::ping::Config::default().with_interval(Duration::from_secs(1)),
            ),
            identify: libp2p::identify::Behaviour::new(libp2p::identify::Config::new(
                "/Phylosopher/0.1.0".into(),
                key.clone(),
            )),
            relay_client,
            dcutr: libp2p::dcutr::Behaviour::new(key.to_peer_id()),
        }
    }
}

#[derive(Debug)]
pub enum Event {
    Ping(libp2p::ping::Event),
    Identify(libp2p::identify::Event),
    Relay(libp2p::relay::client::Event),
    Dcutr(libp2p::dcutr::Event),
}

impl From<libp2p::ping::Event> for Event {
    fn from(value: libp2p::ping::Event) -> Self {
        Self::Ping(value)
    }
}

impl From<libp2p::identify::Event> for Event {
    fn from(value: libp2p::identify::Event) -> Self {
        Self::Identify(value)
    }
}

impl From<libp2p::relay::client::Event> for Event {
    fn from(value: libp2p::relay::client::Event) -> Self {
        Self::Relay(value)
    }
}

impl From<libp2p::dcutr::Event> for Event {
    fn from(value: libp2p::dcutr::Event) -> Self {
        Self::Dcutr(value)
    }
}
