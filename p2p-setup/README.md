# p2p setup

For this setup its assume that these software are installed and running:
<ul>
  <li><a href="https://docs.docker.com/engine/install/ubuntu/" target="_blank">docker</a></li>
  <li><a href="https://minikube.sigs.k8s.io/docs/start/" target="_blank">minikube</a></li>
  <li><a href="https://kubernetes.io/docs/tasks/tools/install-kubectl-linux/" target="_blank">kubectl</a></li>
</ul>

<hr>

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

```bash
kubectl apply -f 1_namespace.yml
```

```bash
kubectl apply -f 2_pod-a.yml
```

```bash
kubectl apply -f 3_pod-b.yml
```
