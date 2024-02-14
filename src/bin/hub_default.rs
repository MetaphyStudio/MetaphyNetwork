use std::error::Error;

use metaphy_network::hub;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create the server.
    let mut server = hub::Server::new()?;

    // Tell the server's swarm to listen on multiaddr.
    server.listen(None)?;

    // Poll the server swarm.
    server.run_loop().await?;

    Ok(())
}
