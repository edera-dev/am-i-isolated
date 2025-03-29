# Kubernetes

## curl | bash

To curl | bash the image, you can use the following command:

```sh
curl -sSL https://raw.githubusercontent.com/edera-dev/am-i-isolated/main/examples/kubernetes/run.sh | bash
```

## Kubernetes Job

To run `am-i-isolated` as a Kubernetes Job:

```sh
kubectl apply -f job.yaml
kubectl logs job/am-i-isolated
```
