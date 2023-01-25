# p2p setup

For this setup its assume that these software are installed and running:
<ul>
  <li><a href="https://docs.docker.com/engine/install/ubuntu/" target="_blank">docker</a></li>
  <li><a href="https://minikube.sigs.k8s.io/docs/start/" target="_blank">minikube</a></li>
  <li><a href="https://kubernetes.io/docs/tasks/tools/install-kubectl-linux/" target="_blank">kubectl</a></li>
</ul>

<hr>

![image](https://user-images.githubusercontent.com/76512851/214585072-0e59a212-ae67-4104-8185-8e150ddd39e3.png)

```bash
docker build -f /p2p-setup/pod-a-dockerfile.dev -t p2p-pod-a:1.0 .
```
Note : to list images just run "docker image ls"

![image](https://user-images.githubusercontent.com/76512851/214587027-dd3e709b-f498-41cc-9941-66a1ed166893.png)

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
