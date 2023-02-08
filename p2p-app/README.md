
# p2p application 

This Rust application is based in the [ping example](https://github.com/libp2p/rust-libp2p/blob/master/examples/ping.rs) for the libp2p. It was developed using the [Intellij Community](https://www.jetbrains.com/idea/download/#section=linux) with the [Rust plugin](https://www.jetbrains.com/rust/).

To compile in release mode.

```bash
cargo build --release
```

To run the application with the argument "a".

```bash
cargo run a
```

To run the application with the argument "b".


```bash
cargo run b
```

Remove al artifacts from the target directory generated in the past.

```bash
cargo clean
```

Where the pods IPs are defined.

![image](https://user-images.githubusercontent.com/76512851/214858437-b54f1b3f-ed59-48cb-b593-594285527c59.png)

How do we know the Pods IPs?

```bash
kubectl get po --all-namespaces -o wide
```

![image](https://user-images.githubusercontent.com/76512851/214622104-04577d9a-8db7-442c-b3c4-acb17429b1b5.png)

The kube-system (coredns-565d847f94-srfc5) is responsible to create [DNS records](https://kubernetes.io/docs/concepts/services-networking/dns-pod-service/) for [Services](https://minikube.sigs.k8s.io/docs/commands/service/) and [Pods](https://kubernetes.io/docs/concepts/workloads/pods/). And it will consume the first IP available in the [minikube node](https://minikube.sigs.k8s.io/docs/commands/node/#minikube-node).

With this information is very easy to preview which IPs will be available for the pods. In ours case 172.17.0.3 and 172.17.0.4.

Note: assigning a Pod a static IP address is an anti-pattern in Kubernetes environments. This example is only for demonstrative purposes.

<hr>

References:<br>
[ping protocol](https://github.com/libp2p/specs/blob/master/ping/ping.md)<br>
[identify protocol](https://github.com/libp2p/specs/blob/master/identify/README.md)<br>
[Peer Ids and Keys](https://github.com/libp2p/specs/blob/master/peer-ids/peer-ids.md)<br>
[Addressing in libp2p ](https://github.com/libp2p/specs/blob/master/addressing/README.md)<br>
