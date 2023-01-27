# Peer To Peer (Work In Progress)

## Motivation


<hr>

### Peer To Peer(p2p)

![image](https://user-images.githubusercontent.com/76512851/214358618-6d3895ff-2da4-4169-840c-200499844b58.png)

Peer to peer networks(P2P) is defined as the group of devices that are connected together to create a network. [libp2p](https://libp2p.io/) is a modular peer-to-peer networking framework. The example provided was done using the Rust implementation of the libp2p networking stack with a Kubernetes infrastructure. The purpose of this little demo is to show how to implement a p2p network with Kubernetes and libp2p.

This source code is composed by:
<ul>
  <li>><a href="https://github.com/gcp-development/peer-to-peer/tree/main/p2p-app" target="_self">p2p-app</a>,This project is associate to the Rust application.</li>
  <li><a href="https://github.com/gcp-development/peer-to-peer/tree/main/p2p-setup" target="_self"p2p-setup</a>,This project is associate to the Kubernetes setup.</li>
</ul>

<hr>

## libp2p

libp2p was designed around the philosophy of creating small components that are easy to understand and test. These components should also be able to be swapped in order to accommodate different technologies or scenarios and also make it feasible to upgrade them over time. (Microservices, anyone?!?)

## Architecture

![image](https://user-images.githubusercontent.com/76512851/214889597-8540ce06-66e6-4739-b8df-22823a92fb78.png)

The libp2p interface acts as a thin veneer over a multitude of subsystems that are required in order for peers to be able to communicate. The main areas where these subsystems fit are:
<ul>
  <li>Peer Routing - Mechanism to decide which peers to use for routing particular messages. This routing can be done recursively, iteratively or even in a broadcast/multicast mode.</li>
  <li>Swarm - Handles everything that touches the 'opening a stream' part of libp2p, from protocol muxing, stream muxing, NAT traversal and connection relaying, while being multi-transport.</li>
  <li>Distributed Record Store - A system to store and distribute records. Records are small entries used by other systems for signaling, establishing links, announcing peers or content, and so on. They have a similar role to DNS in the broader Internet.</li>
  <li>Discovery - Finding or identifying other peers in the network.</li>
</ul>

References:<br>
[libp2p documentation](https://docs.libp2p.io/concepts/introduction/overview/)<br>


