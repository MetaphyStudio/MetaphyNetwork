/// # Hub
/// This will be the main node on the network, responsible for being a gathering place, or townhall,
/// for all nodes on the network to share data, like player data, available relays, public worlds, etc...
pub mod hub;

/// # Node
/// This will be your different nodes that interact with each other on the network, users, hash tabels,
/// relays, everything you'll need for a federated/decentralized network.
pub mod node;

pub fn get_rzv_address_var() -> String {
    let Ok(var) = std::env::var("RZV_ADDRESS") else {
        panic!("There is no funcitonality without an RZV server!")
    };

    var
}