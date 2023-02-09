use futures::StreamExt;
use libp2p::{
    identity, mdns,
    swarm::{Swarm, SwarmEvent},
    PeerId,
};
use std::error::Error;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());
    println!("Local peer id: {peer_id:?}");
    let transport = libp2p::development_transport(id_keys).await?;
    let behaviour = mdns::async_io::Behaviour::new(mdns::Config::default())?;
    let mut swarm = Swarm::with_async_std_executor(transport, behaviour, peer_id);
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Listening on local address {:?}", address)
            }
            SwarmEvent::Behaviour(mdns::Event::Discovered(peers)) => {
                for (peer, addr) in peers {
                    println!("discovered {peer} {addr}");
                }
            }
            SwarmEvent::Behaviour(mdns::Event::Expired(expired)) => {
                for (peer, addr) in expired {
                    println!("expired {peer} {addr}");
                }
            }
            _ => {}
        }
    }
}