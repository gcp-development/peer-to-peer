# peer-to-peer (Work In Progress)
Peer To Peer 

![image](https://user-images.githubusercontent.com/76512851/212056259-67520e4b-703f-44a7-bf2f-2eb06d11f81b.png)


![image](https://user-images.githubusercontent.com/76512851/214309185-c9134ca0-1c2c-4c1e-8706-bd8e932c09b1.png)


```bash
docker build -f /p2p-setup/pod-a-dockerfile.dev -t p2p-pod-a:1.0 .
```

```bash
docker tag p2p-pod-a:1.0 {docker.hub}/p2p-pod-a:1.0
```

```bash
docker push {docker.hub}/p2p-pod-a:1.0
```

```bash
docker build -f /p2p-setup/pod-b-dockerfile.dev -t p2p-pod-b:1.0 .
```

```bash
docker tag p2p-pod-b:1.0 {docker.hub}/p2p-pod-b:1.0
```

```bash
docker push {docker.hub}/p2p-pod-b:1.0
```

```bash
kubectl apply -f 1_namespace.yml
```

```bash
kubectl apply -f 2_pod-a.yml
```

```bash
kubectl apply -f 3_pod-b.yml
```


References:

[Rust libp2p](https://github.com/libp2p/rust-libp2p)<br>
[Rust Official Image](https://hub.docker.com/_/rust)
