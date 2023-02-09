# Kubernetes Setup

It's assumed that these software are installed and running:

<ul>
  <li><a href="https://docs.docker.com/engine/install/ubuntu/" target="_blank">docker</a></li>
  <li><a href="https://minikube.sigs.k8s.io/docs/start/" target="_blank">minikube</a></li>
  <li><a href="https://kubernetes.io/docs/tasks/tools/install-kubectl-linux/" target="_blank">kubectl</a></li>
</ul>
<hr>

## Table of Contents<br>
<ul>
  <li><a href="https://github.com/gcp-development/peer-to-peer/blob/main/kubernetes-setup/README.md#minikube-setup" target="_self">minikube setup</a></li>
  <li>p2p-app</li>
  <ul>
    <li><a href="https://github.com/gcp-development/peer-to-peer/tree/main/p2p-setup#dockerfile" target="_self">Dokerfile</a></li>
    <li><a href="https://github.com/gcp-development/peer-to-peer/tree/main/p2p-setup#pods" target="_self">Pods</a></li>
  </ul>
  <li>mdns-app</li>
  <ul>
    <li>Dokerfile</li>
    <li>Pods</li>
  </ul>
  <li>dht-app</li>
  <ul>
    <li>Dokerfile</li>
    <li>Pods</li>
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
kubectl label nodes minikube-node-m02 nodetype=worker
```

```bash
kubectl get nodes --show-labels
```

![image](https://user-images.githubusercontent.com/76512851/217620938-634d61ca-31c4-45a3-9967-8b68c6e52e9e.png)

<hr>
