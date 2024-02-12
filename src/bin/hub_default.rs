use std::error::Error;

use metaphy_network::hub;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create the server.
    let mut server = hub::Server::new().await?;
    
    // Tell the server's swarm to listen on multiaddr.
    server.listen(None).await?;

    // Poll the server swarm in a loop.
    server.run().await;
    
    Ok(())
}
