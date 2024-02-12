use std::{error::Error, time::Duration};

use futures::StreamExt;
use libp2p::{core::transport::ListenerId, swarm::{NetworkBehaviour, SwarmEvent}, Multiaddr};

pub struct Server(libp2p::Swarm<Behaviour>);

impl Server {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let swarm = libp2p::SwarmBuilder::with_new_identity()
            .with_async_std()
            .with_tcp(
                libp2p::tcp::Config::default(),
                libp2p::noise::Config::new,
                libp2p::yamux::Config::default,
            )?
            .with_behaviour(|key| Behaviour::new(key.public()))?
            .with_swarm_config(|cfg| cfg.with_idle_connection_timeout(Duration::from_secs(30)))
            .build();

        Ok(Self(swarm))
    }

    pub async fn listen(
        &mut self,
        multiaddr: Option<Multiaddr>,
    ) -> Result<ListenerId, Box<dyn Error>> {
        let id = match multiaddr {
            Some(addr) => self.0.listen_on(addr)?,
            None => self.0.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?,
        };
        Ok(id)
    }

    pub async fn run(&mut self) {
        loop {
            match self.0.select_next_some().await {
                SwarmEvent::NewListenAddr { listener_id, address } => println!("Listener {listener_id}, listening on {address:?}"),
                SwarmEvent::Behaviour(event) => println!("{event:?}"), 
                _ => {}
            }
        }
    }
}

#[derive(NetworkBehaviour)]
#[behaviour(to_swarm = "Event")]
pub struct Behaviour {
    ping : libp2p::ping::Behaviour,
    identify: libp2p::identify::Behaviour,
    rzv: libp2p::rendezvous::server::Behaviour,
}

impl Behaviour {
    fn new(key: libp2p::identity::PublicKey) -> Self {
        Self {
            ping: libp2p::ping::Behaviour::new(libp2p::ping::Config::new().with_interval(Duration::from_secs(1))),
            identify: libp2p::identify::Behaviour::new(libp2p::identify::Config::new(
                "HubServer | 0.1.0".into(),
                key,
            )),
            rzv: libp2p::rendezvous::server::Behaviour::new(libp2p::rendezvous::server::Config::default()),
        }
    }
}

#[derive(Debug)]
pub enum Event {
    Ping(libp2p::ping::Event),
    Identify(libp2p::identify::Event),
    Rzv(libp2p::rendezvous::server::Event),
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

impl From<libp2p::rendezvous::server::Event> for Event {
    fn from(value: libp2p::rendezvous::server::Event) -> Self {
        Self::Rzv(value)
    }
}
