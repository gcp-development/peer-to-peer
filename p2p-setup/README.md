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
<li><a href="https://github.com/gcp-development/peer-to-peer/tree/main/p2p-setup#dockerfile" target="_self">Dokerfile</a></li>
<li><a href="https://github.com/gcp-development/peer-to-peer/tree/main/p2p-setup#pods" target="_self">Pods</a></li>  
<li><a href="https://github.com/gcp-development/peer-to-peer/tree/main/p2p-setup#how-do-we-know-the-pods-ips" target="_self">How do we know the Pods IPs?</a></li> 
</ul>
<hr>

## Dockerfile

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

## Pods

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

## How do we know the Pods IPs?

```bash
kubectl get po --all-namespaces -o wide
```

![image](https://user-images.githubusercontent.com/76512851/214622104-04577d9a-8db7-442c-b3c4-acb17429b1b5.png)

The kube-system (coredns-565d847f94-srfc5) is responsible to create [DNS records](https://kubernetes.io/docs/concepts/services-networking/dns-pod-service/) for [Services](https://minikube.sigs.k8s.io/docs/commands/service/) and [Pods](https://kubernetes.io/docs/concepts/workloads/pods/). And it will consume the first IP available in the [minikube node](https://minikube.sigs.k8s.io/docs/commands/node/#minikube-node).

Whit this information is very easy to preview which IPs will be available for the pods. In ours case 172.17.0.3 and 172.17.0.4.

Note: assigning a Pod a static IP address is an anti-pattern in Kubernetes environments. This example is only for demonstrative purposes.

<hr>

References

[Dockerfile reference](https://docs.docker.com/engine/reference/builder/)<br>
[minikube](https://minikube.sigs.k8s.io/docs/)<br>
[Kubernetes Documentation](https://kubernetes.io/docs/home/)<br>
[Rust Official Image](https://hub.docker.com/_/rust)

