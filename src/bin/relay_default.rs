use std::error::Error;

use metaphy_network::{get_rzv_address_var, node::relay};

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let environtment_var = get_rzv_address_var();
    println!("{environtment_var}");

    // Create a user client.
    let mut relay = relay::Server::new()?;

    // Tell the client's swarm to listen on multiaddress.
    relay.listen(None)?;

    // Poll the client swarm.
    relay.run_loop().await?;

    Ok(())
}
