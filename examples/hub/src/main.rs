use std::error::Error;

use log::info;
use metaphy_network::{init_debug_interface, Phylosopher};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_debug_interface();

    let mut node = Phylosopher::new(None)?;
    let _ = node.listen();

    loop {
        match node.poll().await {
            Some(event) => info!("{:?}", event),
            None => (),
        }
    }
}