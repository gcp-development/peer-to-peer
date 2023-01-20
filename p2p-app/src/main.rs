use futures::prelude::*;
use libp2p::swarm::{Swarm, SwarmEvent};
use libp2p::{identity, Multiaddr, PeerId, ping};
use std::error::Error;
use chrono::{Datelike, Timelike, Utc};
use std::{thread, time};

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    match args[1].as_str() {
        "default" => {
            let delay = time::Duration::from_secs(60);
            // Infinite loop
            loop {
                let now = Utc::now();
                let (is_pm, hour) = now.hour12();
                println!(
                    "The current UTC time is {:02}:{:02}:{:02} {}",
                    hour,
                    now.minute(),
                    now.second(),
                    if is_pm { "PM" } else { "AM" }
                );
                let (is_common_era, year) = now.year_ce();
                println!(
                    "The current UTC date is {}-{:02}-{:02} {:?} ({})",
                    year,
                    now.month(),
                    now.day(),
                    now.weekday(),
                    if is_common_era { "CE" } else { "BCE" }
                );
                println!("Waiting for a new cycle....");
                thread::sleep(delay);
            }
        },
        _ => {
            let local_key = identity::Keypair::generate_ed25519();
            let local_peer_id = PeerId::from(local_key.public());
            let transport = libp2p::development_transport(local_key).await?;
            let behaviour = ping::Behaviour::new(ping::Config::new().with_keep_alive(true));
            let mut swarm = Swarm::new(transport, behaviour, local_peer_id);

            swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
            println!("Local peer id: {:?}", local_peer_id);

            if args[1].parse::<Multiaddr>().is_ok() {
                let multi_address:Multiaddr= args[1].parse::<Multiaddr>().unwrap();
                swarm.dial(multi_address)?;
                println!("Dialed {}", args[1].as_str())
            }
            loop {
                match swarm.select_next_some().await {
                    SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {:?}", address),
                    SwarmEvent::Behaviour(event) => println!("{:?}", event),
                    _ => {}
                }
            }
        },
    }
}