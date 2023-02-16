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
  <li><a href="https://github.com/gcp-development/peer-to-peer/blob/main/kubernetes-setup/README.md#scenario-p2p-app" target="_self">Scenario p2p-app</a></li>
  <ul>
    <li><a href="https://github.com/gcp-development/peer-to-peer/tree/main/p2p-setup#dockerfile" target="_self">Dokerfile</a></li>
    <li><a href="https://github.com/gcp-development/peer-to-peer/tree/main/p2p-setup#pods" target="_self">Pods</a></li>
  </ul>
  <li><a href="https://github.com/gcp-development/peer-to-peer/blob/main/kubernetes-setup/README.md#scenario-mdns-app">Scenario mdns-app</a></li>
  <ul>
    <li><a href="https://github.com/gcp-development/peer-to-peer/blob/main/kubernetes-setup/README.md#dockerfile" target="_self">Dokerfile</a></li>
    <li><a href="https://github.com/gcp-development/peer-to-peer/blob/main/kubernetes-setup/README.md#pods" target="_self">Pods</a></li>
  </ul>
  <li><a href="https://github.com/gcp-development/peer-to-peer/blob/main/kubernetes-setup/README.md#scenario-dht-app" target="_self">Scenario dht-app</a></li>
  <ul>
    <li><a href="https://github.com/gcp-development/peer-to-peer/blob/main/kubernetes-setup/README.md#dockerfile-1" target="_self">Dokerfile</a></li>
    <li><a href="https://github.com/gcp-development/peer-to-peer/blob/main/kubernetes-setup/README.md#pods-1" target="_self">Pods</a></li>
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

![image](https://user-images.githubusercontent.com/76512851/219067968-88c9c1ac-849f-463f-b679-e1181020d9f9.png)

Add [labels](https://kubernetes.io/docs/tasks/configure-pod-container/assign-pods-nodes/#add-a-label-to-a-node) to both nodes.

```bash
kubectl label nodes demo nodelabel=node-01
```

```bash
kubectl label nodes demo-m02 nodelabel=node-02
```

```bash
kubectl get nodes --show-labels
```
![image](https://user-images.githubusercontent.com/76512851/219067571-4a184c2c-b1cb-4e45-8853-412be3a2a64e.png)

<hr>

## Scenario p2p-app

### Dockerfile

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

For this scenario we are going to create a node with two pods. Each pod will have two containers. The mDNS protocol is not able to discover devices outside the pod local network. 

![image](https://user-images.githubusercontent.com/76512851/219295216-5307945f-7760-4129-9af0-9782b4ae22e1.png)

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

For this scenario we are going to create a cluster with two nodes, with two pods each. For the mDNS protocol to discover devices outside the pod local network. The HostNetwork is set to true(hostNetwork: true) to allow a pod to use the node network namespace. This was done for pod-a and pod-c. The network behavior is composed by the mDNS protocol for discovery and [Kademlia](https://docs.ipfs.tech/concepts/dht/#kademlia) for storing the IPs of the peers found and providers.

![image](https://user-images.githubusercontent.com/76512851/219296386-dbe19ca5-c1c1-496b-b430-137b8428a0e0.png)

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

Create a pod "pod-a" with one container "container-a".

```bash
kubectl apply -f 2_pod-a.yml
```
Note:HostNetwork allows a pod to use the node-01 network namespace.This is done to allow the mDNS to discover all other pods in the node. This is not a [best practice](https://kubernetes.io/docs/concepts/configuration/overview/) and is used only for demonstration proposes.

Create a pod "pod-b" with one container "container-a".

```bash
kubectl apply -f 3_pod-b.yml
```

Create a pod "pod-c" with one container "container-a".

```bash
kubectl apply -f 4_pod-c.yml
```
Note:HostNetwork allows a pod to use the node-02 network namespace.This is done to allow the mDNS to discover all other pods in the node. This is not a [best practice](https://kubernetes.io/docs/concepts/configuration/overview/) and is used only for demonstration proposes.

Create a pod "pod-d" with one container "container-a".

```bash
kubectl apply -f 5_pod-d.yml
```

Get all pods created.

```bash
kubectl get pods -o wide --namespace=peer-to-peer-platform
```

![image](https://user-images.githubusercontent.com/76512851/219071341-c1508a1e-1d18-400b-8298-70601a713869.png)

Login into the pod-a, execute the command "cargo run" and insert the IP value for the pod-a into the Distributed Hash Tables/Kademlia "PUT pod-a 192.168.49.2".

```bash
kubectl exec -it pod-a --namespace=peer-to-peer-platform -- /bin/bash
```

![image](https://user-images.githubusercontent.com/76512851/219317769-2a37a881-cb2b-4bf9-be3a-bceea9a5292b.png)

Login into the pod-d, execute the command "cargo run" and get the IP value for the pod-a from the Distributed Hash Tables/Kademlia "GET pod-a".

```bash
kubectl exec -it pod-d --namespace=peer-to-peer-platform -- /bin/bash
```

![image](https://user-images.githubusercontent.com/76512851/219321657-d6c6ea58-2c7b-4b61-8450-e032be2db3fc.png)

Login into the pod-b, execute the command "cargo run", insert the IP value for the pod-b from the Distributed Hash Tables/Kademlia "PUT pod-b 10.244.0.3" and insert pod-b as a provider "PUT_PROVIDER pod-b".

```bash
kubectl exec -it pod-b --namespace=peer-to-peer-platform -- /bin/bash
```
![image](https://user-images.githubusercontent.com/76512851/219108166-ca110ab8-7c27-4364-bcd4-4e5b83c068cd.png)

Login into the pod-c, execute the command "cargo run", get the IP value for the pod-b from the Distributed Hash Tables/Kademlia "GET pod-b" and GET pod-b as a provider "GET_PROVIDER pod-b".

```bash
kubectl exec -it pod-c --namespace=peer-to-peer-platform -- /bin/bash
```
![image](https://user-images.githubusercontent.com/76512851/219112492-f8c308f7-dd5d-40cc-a321-ed5d37ba8ae3.png)

Delete all resources belonging to the namespace.
```bash
kubectl delete namespace peer-to-peer-platform
```

<hr>

References:

[Dockerfile reference](https://docs.docker.com/engine/reference/builder/)<br>
[minikube](https://minikube.sigs.k8s.io/docs/)<br>
[Kubernetes Documentation](https://kubernetes.io/docs/home/)<br>
[Rust Official Image](https://hub.docker.com/_/rust)
