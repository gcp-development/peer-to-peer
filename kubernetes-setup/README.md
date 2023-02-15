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
  <li><a href="https://github.com/gcp-development/peer-to-peer/blob/main/kubernetes-setup/README.md#scenario-mdns-app">Scenario mdns-app</a></li>
  <ul>
    <li><a href="https://github.com/gcp-development/peer-to-peer/blob/main/kubernetes-setup/README.md#dockerfile" target="_self">Dokerfile</a></li>
    <li><a href="https://github.com/gcp-development/peer-to-peer/blob/main/kubernetes-setup/README.md#pods" target="_self">Pods</a></li>
  </ul>
  <li>dht-app</li>
  <ul>
    <li>Dokerfile</li>
    <li>Pods</li>
  </ul>
</ul>
<hr>

## minikube setup

minikube version

![image](https://user-images.githubusercontent.com/76512851/217585412-3467872a-4101-4453-9c9e-34ec32402ca1.png)

Create a cluster with [two nodes](https://minikube.sigs.k8s.io/docs/tutorials/multi_node/).

```bash
minikube start --driver=docker --nodes 2 -p demo
```

```bash
kubectl get nodes
```

![image](https://user-images.githubusercontent.com/76512851/219060576-b0e321e2-0b21-442f-a439-bcdd23e81c59.png)


Add [labels](https://kubernetes.io/docs/tasks/configure-pod-container/assign-pods-nodes/#add-a-label-to-a-node) to both nodes.

```bash
kubectl label nodes demo nodelabel=node-01
```

```bash
kubectl label nodes demo-m02 nodelabel=node-01
```

```bash
kubectl get nodes --show-labels
```

![image](https://user-images.githubusercontent.com/76512851/219061525-eacfc396-6789-4b2a-abad-1ab4fc2125ba.png)

<hr>

## Scenario mdns-app

For this scenario we are going to create a node with two pods. Each pod will have two containers. The mDNS protocol is not able to discover devices outside the pod local network. 

![image](https://user-images.githubusercontent.com/76512851/217934985-62b1ce28-82fe-4845-bf4b-32eb96bb2cdf.png)

### Dockerfile

```bash
docker build -f /peer-to-peer/kubernetes-setup/2_scenario_mdns-app/mdns-app-dockerfile.dev -t mdns-app:1.0 .
```

```bash
docker tag mdns-app:1.0 {docker.hub}/mdns-app:1.0
```

```bash
docker push {docker.hub}/mdns-app:1.0
```

### Pods

```bash
kubectl apply -f 1_namespace.yml
```
Note: to list the namespaces just run "kubectl get namespaces"

![image](https://user-images.githubusercontent.com/76512851/217919434-fd6201fb-e75c-4b43-a480-057119532af0.png)

Create a pod "pod-a" with two containers "container-a" & "container-b".

```bash
kubectl apply -f 2_pod-a.yml
```

Create a pod "pod-b" with two containers "container-a" & "container-b".

```bash
kubectl apply -f 3_pod-b.yml
```

```bash
kubectl get pod -o wide --namespace=peer-to-peer-platform
```

![image](https://user-images.githubusercontent.com/76512851/217921445-43279fa5-fdbd-4261-93c2-d3ce1e4fe142.png)

Verify the logs for the container "container-a" inside of the pod "pod-a".
```bash
kubectl logs pod-a -c container-a  --namespace=peer-to-peer-platform
```

![image](https://user-images.githubusercontent.com/76512851/217921067-33107029-5a88-4db3-b8d4-5e0b8d6f955e.png)

Verify the logs for the container "container-b" inside of the pod "pod-a".

```bash
kubectl logs pod-a -c container-b  --namespace=peer-to-peer-platform
```

![image](https://user-images.githubusercontent.com/76512851/217922598-087eaf74-e47d-4233-80ff-7a709137dfa7.png)

Verify the logs for the container "container-a" inside of the pod "pod-b".

```bash
kubectl logs pod-b -c container-a  --namespace=peer-to-peer-platform
```

![image](https://user-images.githubusercontent.com/76512851/217924430-b1a5ee3e-b00f-42d0-9609-a9f7812c5b5a.png)

Verify the logs for the container "container-b" inside of the pod "pod-b".

```bash
kubectl logs pod-b -c container-b  --namespace=peer-to-peer-platform
```

![image](https://user-images.githubusercontent.com/76512851/217924917-4f78508c-8c2a-4119-b831-a5f31590abed.png)

Delete all resources belonging to the namespace.
```bash
kubectl delete namespace peer-to-peer-platform
```

<hr>

## Scenario dht-app

### Dockerfile

```bash
docker build -f /peer-to-peer/kubernetes-setup/3_scenario_dht-app/dht-app-dockerfile.dev -t dht-app:1.0 .
```

```bash
docker tag mdns-app:1.0 {docker.hub}/dht-app:1.0
```

```bash
docker push {docker.hub}/dht-app:1.0
```

### Pods


```bash
kubectl apply -f 1_namespace.yml
```
Note: to list the namespaces just run "kubectl get namespaces"

![image](https://user-images.githubusercontent.com/76512851/217919434-fd6201fb-e75c-4b43-a480-057119532af0.png)

```bash
kubectl apply -f 2_pod-a.yml
```
Note:HostNetwork allows a pod to use the node network namespace.This is done to allow the mDNS to discover all other pods in the node. This is not a [best practice](https://kubernetes.io/docs/concepts/configuration/overview/) and is used only for demonstration proposes.

```bash
kubectl apply -f 3_pod-b.yml
```


<hr>

References:

[Dockerfile reference](https://docs.docker.com/engine/reference/builder/)<br>
[minikube](https://minikube.sigs.k8s.io/docs/)<br>
[Kubernetes Documentation](https://kubernetes.io/docs/home/)<br>
[Rust Official Image](https://hub.docker.com/_/rust)
