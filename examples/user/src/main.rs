use futures::StreamExt;
use log::{info, warn};
use metaphy_network::{init_debug_interface, Phylosopher};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_debug_interface();

    // Here we simply create a new user node with a default keypair.
    let node = Phylosopher::new(None)?;

    // Then we are telling our node to bind a new listening address to the swarm,
    // again using the deafult address to bind to.
    node.bind(None).await;

    // Now we can poll our swarm for events.
    // First we get a strong pointer to the swarm's mutex, then
    // we lock the mutex so we can mutate the swarm safely between threads.
    let swarm = node.get_swarm();
    let mut swarm = swarm.lock().await;

    loop {
        match swarm.select_next_some().await {
            metaphy_network::libp2p::swarm::SwarmEvent::Behaviour(event) => info!("{event:?}"),
            _ => warn!("Unhandled swarm event(s)..."),
        }
    }
}
