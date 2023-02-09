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

## Scenario mdns-app

For this scenario we are going to create a node with two pods. Each pod will have two containers. The mDNS protocol can only discover devices in the local network inside of the pod. 

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
