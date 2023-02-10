# Peer To Peer (Work In Progress)

## Motivation
Peer to Peer networks are fundamental to the blockchain technology, they provide the base layer for a decentralized communication model between two peers , which can communicate with each other without the need for a central server. Using the correct technologies to implement this types of networks is essential for the success of any blockchain solution. As always [Kubernetes](https://kubernetes.io/docs/concepts/overview/), due its scalability and flexibility, is a must. Associated to a library, like [rust-libp2p](https://github.com/libp2p/rust-libp2p), that can support the requirements of a p2p application.

<hr>

### Peer To Peer(p2p)

Peer to peer networks(P2P) is defined as the group of devices that are connected together to create a network. [libp2p](https://libp2p.io/) is a modular peer-to-peer networking framework.

![image](https://user-images.githubusercontent.com/76512851/214358618-6d3895ff-2da4-4169-840c-200499844b58.png)

Two [Pods](https://kubernetes.io/docs/concepts/workloads/pods/) were created to accommodate the p2p-app. Which is a Rust application using the [libp2p ping protocol](https://docs.libp2p.io/concepts/introduction/protocols/ping/) to test the connectivity and performance between these two Pods. For each Pod a [image](https://docs.docker.com/engine/reference/commandline/image/) is created and deployed into the [container](https://kubernetes.io/docs/concepts/containers/) inside of the corresponding Pod.

Although libp2p was originally developed to work with [IPFS](https://ipfs.tech/), we want to use it to create p2p applications that have no relationship to IPFS at all.

In order to do that we need:
<ul>
 <li>A discovery service that is able to find peers.<a href="https://github.com/libp2p/specs/blob/master/discovery/mdns.md"> Multicast DNS (mDNS) protocol.</a></li>
 <li>A register service that is able store SHA256 (<a href="https://docs.libp2p.io/concepts/fundamentals/peers/#peer-id">PeerID</a>) in a data structure.<a href="https://docs.ipfs.tech/concepts/dht/"> Kademlia Distributed Hash Table.</a></li>
</ul>

The Multicast DNS (mDNS) protocol only can be used in local networks ([containers](https://kubernetes.io/docs/concepts/containers/) within the same [Pod](https://kubernetes.io/docs/concepts/workloads/pods/)). Multicast DNS (mDNS) does not process hostnames with other top-level domains (TLDs). Meaning we will need a implementation like [Kademlia](https://docs.ipfs.tech/concepts/dht/#kademlia) that is a data structure stored on multiple computers, scalable and fault-tolerant.

A [distributed hash table (DHT)](https://docs.ipfs.tech/concepts/dht/) is a distributed system for mapping keys to values:
<ul>
 <li>PUT(key, value), which inserts a new element.</li>
 <li>GET(key), which returns the value of the element corresponding to that key.</li>
</ul>

There are some fundamental limitations here. If all computers leave at once, we have nowhere to store anything. We will need to replicate keys across different computers so that key-value pairs will be recoverable even if some of those computers leave at once.

### Communication between pods on the same node

A network bridge connects two networks together. When a request hits the bridge, the bridge asks all the connected devices (i.e. pods) if they have the right IP address to handle the original request. 
 
![image](https://user-images.githubusercontent.com/76512851/216921321-ec8ff596-73bb-4215-9aec-cf8a1d874902.png)

### Communication between pods on different nodes

At the cluster level, there’s a table that maps IP address ranges to various nodes. Pods on those nodes will have been assigned IP addresses from those ranges.<br>

![image](https://user-images.githubusercontent.com/76512851/216921925-85ff702b-690e-4c80-8b07-d8068a34c36c.png)

### Conclusion

The underlying objective is to leverages the power of p2p networks via the [libp2p-rust](https://github.com/libp2p/rust-libp2p) to provide a shared and trusted ledger of transactions (blockchain technology).

Source code:
<ul>
  <li><a href="https://github.com/gcp-development/peer-to-peer/tree/main/p2p-app" target="_self">p2p-app</a> ping example.</li>
  <li><a href="https://github.com/gcp-development/peer-to-peer/tree/main/mdns-app" target="_self">mdns-app</a> mDNS example.</li>
  <li><a href="https://github.com/gcp-development/peer-to-peer/tree/main/dht-app" target="_self">dht-app</a> mDNS/Kademlia(DHT) example.</li>
  <li><a href="https://github.com/gcp-development/peer-to-peer/tree/main/kubernetes-setup" target="_self">Kubernetes setup.</a></li>
</ul>

<hr>

## libp2p

libp2p was designed around the philosophy of creating small components that are easy to understand and test. These components should also be able to be swapped in order to accommodate different technologies or scenarios and also make it feasible to upgrade them over time. ([Microservices](https://microservices.io/), anyone?!?)

## Architecture

![image](https://user-images.githubusercontent.com/76512851/214889597-8540ce06-66e6-4739-b8df-22823a92fb78.png)

The libp2p interface acts as a thin veneer over a multitude of subsystems that are required in order for peers to be able to communicate. The main areas where these subsystems fit are:
<ul>
  <li>Peer Routing - Mechanism to decide which peers to use for routing particular messages. This routing can be done recursively, iteratively or even in a broadcast/multicast mode.</li>
  <li>Swarm - Handles everything that touches the 'opening a stream' part of libp2p, from protocol muxing, stream muxing, NAT traversal and connection relaying, while being multi-transport.</li>
  <li>Distributed Record Store - A system to store and distribute records. Records are small entries used by other systems for signaling, establishing links, announcing peers or content, and so on. They have a similar role to DNS in the broader Internet.</li>
  <li>Discovery - Finding or identifying other peers in the network.</li>
</ul>

<hr>

References:<br>
[Rust libp2p](https://github.com/libp2p/rust-libp2p)
[libp2p documentation](https://docs.libp2p.io/concepts/introduction/overview/)<br>
