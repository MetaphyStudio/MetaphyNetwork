use std::error::Error;

use metaphy_network::{get_rzv_address_var, node::user};

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let environtment_var = get_rzv_address_var();
    println!("{environtment_var}");
    
    // Create a user client.
    let mut client = user::Client::new()?;

    // Tell the client's swarm to listen on multiaddress.
    client.listen(None)?;

    // Poll the client swarm.
    client.run_loop().await?;

    Ok(())
}
