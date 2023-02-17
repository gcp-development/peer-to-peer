use futures::prelude::*;
use libp2p::swarm::{keep_alive, NetworkBehaviour, Swarm, SwarmEvent};
use libp2p::{identity, ping, Multiaddr, PeerId};
use std::error::Error;

static POD_IP_A: &str = "172.17.0.3/tcp/";
static POD_IP_B: &str = "172.17.0.4/tcp/";
static POD_PORT: &str = "4242";

#[derive(NetworkBehaviour, Default)]
struct Behaviour {
    keep_alive: keep_alive::Behaviour,
    ping: ping::Behaviour,
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    let transport = libp2p::development_transport(local_key).await?;
    let behaviour = Behaviour::default();
    let mut swarm =Swarm::with_async_std_executor(transport, behaviour, local_peer_id);
    let mut multi_address_pod_a: String = "/ip4/".to_owned();
    let mut multi_address_pod_b: String = "/ip4/".to_owned();

    match args[1].as_str() {
        "a" => {
            multi_address_pod_a.push_str(&POD_IP_A);
            multi_address_pod_a.push_str(&POD_PORT);
            println!("Multiaddress: {}", multi_address_pod_a);
            swarm.listen_on(multi_address_pod_a.parse()?)?;
        },
        "b" => {
            multi_address_pod_b.push_str(&POD_IP_B);
            multi_address_pod_b.push_str(&POD_PORT);
            println!("Multiaddress: {}", multi_address_pod_b);
            swarm.listen_on(multi_address_pod_b.parse()?)?;

            multi_address_pod_a.push_str(&POD_IP_A);
            multi_address_pod_a.push_str(&POD_PORT);

            if multi_address_pod_a.parse::<Multiaddr>().is_ok() {
                let remote_node: Multiaddr = multi_address_pod_a.parse::<Multiaddr>().unwrap();
                swarm.dial(remote_node)?;
                println!("Dialed {}", multi_address_pod_a.as_str())
            }
        },
        _ => {
            println!("Invalid option its must be a or b.");
        },
    }
    println!("Local peer id: {:?}", local_peer_id);

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {:?}", address),
            SwarmEvent::Behaviour(event) => println!("{:?}", event),
            _ => {}
        }
    }
}