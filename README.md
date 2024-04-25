# MetaphyNetwork

This project will be the backbone of a new vr nexus that supports open source and federated communities. The net code will be built with `rust-libp2p` and provide an open API for developers to implement into their own game engines that support low level languages _(like rust)_.

## Structure

This section will breifly describe the structure of MetaphyNetwork nodes and their use case.

- Hub _(The entry point to an instance)_
    - In `libp2p` this would be a Rendezvous node that allows user nodes to connect to the instance of interest and start communicating with other users that are connected to the instnace.
    - Hubs are able to maintane lists of available relay, and data nodes for users to query when ever they need to make a connection through a relay, or need to access public, and/or restricted instance data.instance.
- Relay _(To get around those pesky NATs)_
    - A relay, using `relay` in combination with another `libp2p` protocol, `dcutr`, allows user nodes behind NATs to be able to connect to one another without a middleman.
- Data _(Persistant instance data)_
    - A Data node, using `libp2p`'s `kad` protocol, allows each instance to store persistant user data _(i.e; Profiles, Avatars, Worlds, Assets, and Props)_.
- User _(you)_
    - A node that takes advantage of `libp2p` under the hood to connect to a decentralized and federated network designed around the user, their creativity, and social connections in a digital world.
    - Makes use of `mDNS` to find local network nodes _(this is really useful in situations there is no connection to the outside internet)_.
    - Makes use of `rzv` to connect to either the default Hub, or a prefered Hub hosted by a 3rd-party of MetaphyStudio.
    - Users keep a list of friends and their "home" instance, as well as what status they have set _(Online, AFH, Do Not Disturb, or Offline)_
