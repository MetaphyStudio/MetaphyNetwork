use log::{debug, info, warn};
use metaphy_network::{init_debug_interface, libp2p::{Multiaddr, PeerId}, user};
use std::{collections::{HashMap, VecDeque}, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_debug_interface();

    let mut user = user::Node::new()?;
    let _ = user.listen()?;

    let mut events = VecDeque::new();
    let mut discovered_peers: HashMap<PeerId, Multiaddr> = HashMap::new();

    loop {
        user.run_once(&mut events).await;
        if let Some(event) = events.pop_front() {
            match event {
                user::BehaviourEvent::Ping(e) => info!("{e:?}"),
                user::BehaviourEvent::Relay(e) => match e {
                    metaphy_network::libp2p::relay::client::Event::ReservationReqAccepted { relay_peer_id, renewal, limit } => info!("Relay Reservation Request Accepted!\nRelay Peer: {relay_peer_id}\nRenewal: {renewal}\nReservation Limit: {:?}\n", limit.unwrap()),
                    metaphy_network::libp2p::relay::client::Event::OutboundCircuitEstablished { relay_peer_id, limit } => info!("Outbount Circuit Established!\nRelay Peer: {relay_peer_id}\nReservation Limit: {:?}\n", limit.unwrap()),
                    metaphy_network::libp2p::relay::client::Event::InboundCircuitEstablished { src_peer_id, limit } => info!("Inbound Circuit Established!\nPeer: {src_peer_id}\nReservation Limit: {:?}\n", limit.unwrap()),
                },
                user::BehaviourEvent::Mdns(e) => match e {
                    metaphy_network::libp2p::mdns::Event::Discovered(discoveries) => {
                        for (peer_id, addr) in discoveries {
                            if discovered_peers.contains_key(&peer_id) {
                                // If this peer discovery already exists, and is rediscovered with a new address,
                                // we want to update that entry in our hashmap.
                                discovered_peers.entry(peer_id).or_insert(addr.clone());
                            } else {
                                // Else, if this peer discovery does not already exist, we add it to the hashmap.
                                discovered_peers.insert(peer_id, addr.clone());
                            }
                            info!("Discovered new peer...\nPeer: {peer_id}\nAddress: {addr}\nAdding to list of discovered peers.\n")
                        }
                        debug!("Discovered peers: {discovered_peers:?}")
                    },
                    metaphy_network::libp2p::mdns::Event::Expired(expirations) => {
                        for (peer_id, addr) in expirations {
                            discovered_peers.remove(&peer_id);
                            warn!("Mdns expiration...\nPeer: {peer_id}\nAddress: {addr}\nPeer has been removed from the hashmap.")
                        }
                    },
                },
            }
        } else {
            continue;
        };
    }
}
