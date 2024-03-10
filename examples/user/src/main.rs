use log::debug;
use metaphy_network::{init_debug_interface, Logic, Phylosopher};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_debug_interface();

    let mut node = Phylosopher::new(None)?;
    let _ = node.listen();
    
    loop {
        match node.poll().await {
            Some(e) => match e {
                Logic::Ping(event) => debug!("Ping -> {event:?}"),
                Logic::Protocol(event) => debug!("Protocol -> {event:?}"),
                Logic::Mdns(event) => debug!("Mdns -> {event:?}"),
                Logic::ClientRelay(event) => debug!("Client Relay -> {event:?}"),
                Logic::Dcutr(event) => debug!("Dcutr -> {event:?}"),
                Logic::ClientRzv(event) => debug!("Client Rzv -> {event:?}"),
                Logic::Kad(event) => debug!("Kad -> {event:?}"),
            },
            None => (),
        }
    }
}
