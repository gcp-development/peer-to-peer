use futures::StreamExt;
use libp2p::{
    identity,
    mdns,
    swarm::SwarmEvent,
    PeerId,
    Swarm,
};
use std::error::Error;

static POD_PORT: &str = "4242";

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());
    let transport = libp2p::development_transport(id_keys).await?;
    let behaviour = mdns::async_io::Behaviour::new(mdns::Config::default())?;
    let mut swarm = Swarm::with_async_std_executor(transport, behaviour, peer_id);
    let mut multi_address_pod: String = "/ip4/0.0.0.0/tcp/".to_owned();

    multi_address_pod.push_str(&POD_PORT);
    println!("Local peer id: {:?}", peer_id);
    println!("Pod Multi address: {}", multi_address_pod);
    swarm.listen_on(multi_address_pod.parse()?)?;
    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Listening on local address {:?}", address)
            }
            SwarmEvent::Behaviour(mdns::Event::Discovered(peers)) => {
                for (peer, addr) in peers {
                    println!("discovered {} {}", peer, addr);
                }
            }
            SwarmEvent::Behaviour(mdns::Event::Expired(expired)) => {
                for (peer, addr) in expired {
                    println!("expired {} {}", peer, addr);
                }
            }
            _ => {}
        }
    }
}