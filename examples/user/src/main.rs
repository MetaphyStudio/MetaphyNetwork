use log::debug;
use metaphy_network::{init_debug_interface, Phylosopher};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_debug_interface();

    let mut node = Phylosopher::new(None)?;
    let _ = node.listen();
    
    loop {
        match node.poll().await {
            // NOTE: Ignore "missing match arm..." error as this will not effect compilation at all,
            // it's only happening due to `rust-analyzer` not noticing the conditional compilation.
            Some(e) => match e {
                metaphy_network::PhylosophyEvent::Ping(e) => debug!("{e:?}"),
                metaphy_network::PhylosophyEvent::Protocol(e) => debug!("{e:?}"),
                metaphy_network::PhylosophyEvent::Mdns(e) => debug!("{e:?}"),
                metaphy_network::PhylosophyEvent::ClientRelay(e) => debug!("{e:?}"),
                metaphy_network::PhylosophyEvent::Dcutr(e) => debug!("{e:?}"),
                metaphy_network::PhylosophyEvent::ClientRzv(e) => debug!("{e:?}"),
                metaphy_network::PhylosophyEvent::Kad(e) => debug!("{e:?}"),
            },
            None => (),
        }
    }
}
