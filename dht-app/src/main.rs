use async_std::io;
use futures::{prelude::*, select};
use libp2p::kad::record::store::MemoryStore;
use libp2p::kad::{
    record::Key,
    AddProviderOk,
    Kademlia,
    KademliaEvent,
    PeerRecord,
    PutRecordOk,
    QueryResult,
    Quorum,
    Record,
};
use libp2p::{
    development_transport,
    identity,
    mdns,
    swarm::{NetworkBehaviour, SwarmEvent},
    PeerId,
    Swarm,
};
use libp2p_kad::{GetProvidersOk, GetRecordOk};
use std::error::Error;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    let transport = development_transport(local_key).await?;

    #[derive(NetworkBehaviour)]
    #[behaviour(out_event = "DiscoveryAndStoreEvent")]
    struct DiscoveryAndStoreBehaviour {
        kademlia: Kademlia<MemoryStore>,
        mdns: mdns::async_io::Behaviour,
    }

    #[allow(clippy::large_enum_variant)]
    enum DiscoveryAndStoreEvent {
        Kademlia(KademliaEvent),
        Mdns(mdns::Event),
    }

    impl From<KademliaEvent> for DiscoveryAndStoreEvent {
        fn from(event: KademliaEvent) -> Self {
            DiscoveryAndStoreEvent::Kademlia(event)
        }
    }

    impl From<mdns::Event> for DiscoveryAndStoreEvent {
        fn from(event: mdns::Event) -> Self {
            DiscoveryAndStoreEvent::Mdns(event)
        }
    }

    let mut swarm = {
        let store = MemoryStore::new(local_peer_id);
        let kademlia = Kademlia::new(local_peer_id, store);
        let mdns = mdns::async_io::Behaviour::new(mdns::Config::default())?;
        let behaviour = DiscoveryAndStoreBehaviour { kademlia, mdns };
        Swarm::with_async_std_executor(transport, behaviour, local_peer_id)
    };
    let mut stdin = io::BufReader::new(io::stdin()).lines().fuse();

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
    loop {
        select! {
        line = stdin.select_next_some() => handle_input_line(&mut swarm.behaviour_mut().kademlia, line.expect("Stdin not to close")),
        event = swarm.select_next_some() => match event {
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Listening in {address:?}");
            },
            SwarmEvent::Behaviour(DiscoveryAndStoreEvent::Mdns(mdns::Event::Discovered(list))) => {
                for (peer_id, multiaddr) in list {
                    swarm.behaviour_mut().kademlia.add_address(&peer_id, multiaddr);
                }
            }
            SwarmEvent::Behaviour(DiscoveryAndStoreEvent::Kademlia(KademliaEvent::OutboundQueryProgressed { result, ..})) => {
                match result {
                    QueryResult::GetProviders(Ok(GetProvidersOk::FoundProviders { key, providers, .. })) => {
                        for peer in providers {
                            println!(
                                "Peer {peer:?} provides key {:?}",
                                std::str::from_utf8(key.as_ref()).unwrap()
                            );
                        }
                    }
                    QueryResult::GetProviders(Err(err)) => {
                        eprintln!("Failed to get providers: {err:?}");
                    }
                    QueryResult::GetRecord(Ok(
                        GetRecordOk::FoundRecord(PeerRecord {
                            record: Record { key, value, .. },
                            ..
                        })
                    )) => {
                        println!(
                            "Got record {:?} {:?}",
                            std::str::from_utf8(key.as_ref()).unwrap(),
                            std::str::from_utf8(&value).unwrap(),
                        );
                    }
                    QueryResult::GetRecord(Ok(_)) => {}
                    QueryResult::GetRecord(Err(err)) => {
                        eprintln!("Failed to get record: {err:?}");
                    }
                    QueryResult::PutRecord(Ok(PutRecordOk { key })) => {
                        println!(
                            "Successfully put record {:?}",
                            std::str::from_utf8(key.as_ref()).unwrap()
                        );
                    }
                    QueryResult::PutRecord(Err(err)) => {
                        eprintln!("Failed to put record: {err:?}");
                    }
                    QueryResult::StartProviding(Ok(AddProviderOk { key })) => {
                        println!(
                            "Successfully put provider record {:?}",
                            std::str::from_utf8(key.as_ref()).unwrap()
                        );
                    }
                    QueryResult::StartProviding(Err(err)) => {
                        eprintln!("Failed to put provider record: {err:?}");
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        }
    }
}

fn handle_input_line(kademlia: &mut Kademlia<MemoryStore>, line: String) {
    let mut args = line.split(' ');

    match args.next() {
        Some("GET") => {
            let key = {
                match args.next() {
                    Some(key) => Key::new(&key),
                    None => {
                        eprintln!("Expected key");
                        return;
                    }
                }
            };
            kademlia.get_record(key);
        }
        Some("GET_PROVIDERS") => {
            let key = {
                match args.next() {
                    Some(key) => Key::new(&key),
                    None => {
                        eprintln!("Expected key");
                        return;
                    }
                }
            };
            kademlia.get_providers(key);
        }
        Some("PUT") => {
            let key = {
                match args.next() {
                    Some(key) => Key::new(&key),
                    None => {
                        eprintln!("Expected key");
                        return;
                    }
                }
            };
            let value = {
                match args.next() {
                    Some(value) => value.as_bytes().to_vec(),
                    None => {
                        eprintln!("Expected value");
                        return;
                    }
                }
            };
            let record = Record {
                key,
                value,
                publisher: None,
                expires: None,
            };
            kademlia
                .put_record(record, Quorum::One)
                .expect("Failed to store record locally.");
        }
        Some("PUT_PROVIDER") => {
            let key = {
                match args.next() {
                    Some(key) => Key::new(&key),
                    None => {
                        eprintln!("Expected key");
                        return;
                    }
                }
            };

            kademlia
                .start_providing(key)
                .expect("Failed to start providing key");
        }
        _ => {
            eprintln!("expected GET, GET_PROVIDERS, PUT or PUT_PROVIDER");
        }
    }
}