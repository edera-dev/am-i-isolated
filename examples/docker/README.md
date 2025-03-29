# Docker

To use `am-i-isolated` in a Docker container, you can pull the image from the
GitHub Container Registry and run it directly. This is the recommended way to
use `am-i-isolated` for most users.

```sh
docker run --rm -it ghcr.io/edera-dev/am-i-isolated:nightly
```

## Script file

An example of this is available within the `run.sh` script within this
directory. This script will run the `am-i-isolated` container and execute the
tests within the container. The results will be printed to the console.

```sh
./run.sh
```
