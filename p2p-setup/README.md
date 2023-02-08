# p2p setup

For this setup its assume that these software are installed and running:
<ul>
  <li><a href="https://docs.docker.com/engine/install/ubuntu/" target="_blank">docker</a></li>
  <li><a href="https://minikube.sigs.k8s.io/docs/start/" target="_blank">minikube</a></li>
  <li><a href="https://kubernetes.io/docs/tasks/tools/install-kubectl-linux/" target="_blank">kubectl</a></li>
</ul>
<hr>

## Table of Contents<br>
<ul>
  <li><a href="https://github.com/gcp-development/peer-to-peer/blob/main/p2p-setup/README.md#minikube-setup" target="_self">minikube setup</a></li>
  <ii>p2p-app</li>
  <ul>
    <li><a href="https://github.com/gcp-development/peer-to-peer/tree/main/p2p-setup#dockerfile" target="_self">Dokerfile</a></li>
    <li><a href="https://github.com/gcp-development/peer-to-peer/tree/main/p2p-setup#pods" target="_self">Pods</a></li>
  </ul>
  <ii>mdns-app</li>
  <ul>
    <li></li>
    <li></li>
  </ul>
  <ii>dht-app</li>
  <ul>
    <li></li>
    <li></li>
  </ul>
</ul>
<hr>

## minikube setup

![image](https://user-images.githubusercontent.com/76512851/217585412-3467872a-4101-4453-9c9e-34ec32402ca1.png)

Minikube setup with [two nodes](https://minikube.sigs.k8s.io/docs/tutorials/multi_node/).

```bash
minikube start --driver=docker --cpus=4 --memory=8192 --nodes 2 -p minikube-node
```

```bash
kubectl get nodes
```

![image](https://user-images.githubusercontent.com/76512851/217586844-d342c20e-76a0-46e3-a4fa-e63f0a94ed6a.png)

Add [labels](https://kubernetes.io/docs/tasks/configure-pod-container/assign-pods-nodes/#add-a-label-to-a-node) to both nodes.
```bash
kubectl label nodes minikube-node nodetype=control-plane
```

```bash
kubectl label nodes minikube-node nodetype=worker
```

```bash
kubectl label nodes minikube-node nodetype=worker
```

```bash
kubectl get nodes --show-labels
```

![image](https://user-images.githubusercontent.com/76512851/217620938-634d61ca-31c4-45a3-9967-8b68c6e52e9e.png)

## Scenario p2p-app

### Dockerfile

![image](https://user-images.githubusercontent.com/76512851/214598654-10fe08b3-0297-4122-a26f-d12c894f7491.png)

```bash
docker build -f /p2p-setup/pod-a-dockerfile.dev -t p2p-pod-a:1.0 .
```
Note : to list images just run "docker image ls"

![image](https://user-images.githubusercontent.com/76512851/214587480-350a7121-ea38-4603-8923-17caf8b91683.png)

```bash
docker tag p2p-pod-a:1.0 {docker.hub}/p2p-pod-a:1.0
```
![image](https://user-images.githubusercontent.com/76512851/214598064-5c73f619-9dfe-412f-8622-3ec136fd8a9f.png)

```bash
docker push {docker.hub}/p2p-pod-a:1.0
```

![image](https://user-images.githubusercontent.com/76512851/214589087-f62ee2b0-d66c-4f52-9001-a417094935fe.png)


```bash
docker build -f /p2p-setup/pod-b-dockerfile.dev -t p2p-pod-b:1.0 .
```

```bash
docker tag p2p-pod-b:1.0 {docker.hub}/p2p-pod-b:1.0
```

```bash
docker push {docker.hub}/p2p-pod-b:1.0
```

![image](https://user-images.githubusercontent.com/76512851/214597726-33990b1c-45d7-4d7b-ba21-30b5cac7f17a.png)

<hr>

### Pods

![image](https://user-images.githubusercontent.com/76512851/214600031-fd6627f0-f848-461a-9555-6f0b113a3bb8.png)

```bash
kubectl apply -f 1_namespace.yml
```
Note: to list the namespaces just run "kubectl get namespaces"

![image](https://user-images.githubusercontent.com/76512851/214600792-700afd92-3553-4f1d-a3fe-cebd304d89b2.png)


```bash
kubectl apply -f 2_pod-a.yml
```

```bash
kubectl logs -f pod-a --namespace=peer-to-peer-platform
```

![image](https://user-images.githubusercontent.com/76512851/214621392-270750a6-fc6d-4dc4-9108-237ce5bd045d.png)


```bash
kubectl apply -f 3_pod-b.yml
```

```bash
kubectl logs -f pod-b --namespace=peer-to-peer-platform
```

![image](https://user-images.githubusercontent.com/76512851/214621152-36dd79a8-8007-4509-a56e-8bdc0678bf66.png)

<hr>

## Scenario mdns-app

### Dockerfile

### Pods

<hr>

## Scenario dht-app

### Dockerfile

### Pods

<hr>

References

[Dockerfile reference](https://docs.docker.com/engine/reference/builder/)<br>
[minikube](https://minikube.sigs.k8s.io/docs/)<br>
[Kubernetes Documentation](https://kubernetes.io/docs/home/)<br>
[Rust Official Image](https://hub.docker.com/_/rust)

