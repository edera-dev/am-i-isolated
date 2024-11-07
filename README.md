# *Am I Isolated*

*Am I Isolated* is a security posture benchmarking tool.

It evaluates a given runtime environment and attempts to look for things
which may be a security problem, as well as providing suggestions for
solving the security problem.

Security is a rapidly evolving space: it is intended that *Am I Isolated* is
updated over time to incorporate new and relevant security research
relating to jailing containers.

*Am I Isolated* is also still a work in progress and does not yet incorporate
tests for all possible container security problems.  Current work is
focused on providing enough data in a digestable format, rather than
overwhelming security engineers and CISOs with too much data.

## Using *Am I Isolated*

In general you will want to use the OCI image:

```sh
docker run --rm -it ghcr.io/edera-dev/am-i-isolated:nightly
```

To detect isolation gaps in your Kubernetes environments, you can run it as a Pod

```sh
apiVersion: v1
kind: Pod
metadata:
  name: am-i-isolated
spec:
  containers:
  - name: am-i-isolated
    image: "ghcr.io/edera-dev/am-i-isolated:nightly"
```

And fetch the logs for the results

```sh
kubectl logs am-i-isolated
```

You can also build and run directly with Cargo.
