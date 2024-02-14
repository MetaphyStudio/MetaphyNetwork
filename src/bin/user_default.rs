use std::error::Error;

use metaphy_network::node::user;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create a user client.
    let mut client = user::Client::new()?;

    // Tell the client's swarm to listen on multiaddress.
    client.listen(None)?;

    // Poll the client swarm.
    client.run_loop().await?;

    Ok(())
}
