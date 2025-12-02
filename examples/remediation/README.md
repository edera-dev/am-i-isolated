# Remediation

Scripts and configurations to fix security issues detected by `am-i-isolated`.

## AII2220: RWX Memory Mappings

**Issue:** The container allows creating memory regions that are simultaneously readable, writable, and executable (RWX). This is a key primitive for exploitation techniques like shellcode injection and JIT spraying.

**Remediation:** Apply a custom seccomp profile that blocks `mmap`/`mprotect` syscalls requesting RWX permissions.

See [`seccomp-no-rwx.json`](seccomp-no-rwx.json) for the seccomp profile and usage instructions.

### Quick Start

```bash
docker run --rm -it --security-opt seccomp=./examples/remediation/seccomp-no-rwx.json <image>
```

### Caveats

- May break applications requiring JIT compilation (some JavaScript engines, Java JIT)
- For production, consider extending Docker's [default seccomp profile](https://github.com/moby/moby/blob/master/profiles/seccomp/default.json) with these rules

---

## AII2240: Host Namespaces

**Issue:** The container shares one or more namespaces (pid, net, ipc, user) with the host. This reduces isolation and increases the impact of container escapes. When the user namespace is shared, container root is the same as host root.

**Remediation:** Configure Docker daemon to use user namespace remapping, which maps container UID 0 to an unprivileged host UID.

See [`docker-configure-userns.sh`](docker-configure-userns.sh) for the configuration script and usage instructions.

### Quick Start

```bash
sudo ./examples/remediation/docker-configure-userns.sh
sudo systemctl restart docker
```

### Verification

```bash
docker run --rm alpine cat /proc/self/uid_map
# Should show non-zero offset, e.g.: 0 100000 65536
```

### Caveats

- Affects all containers on the daemon
- Some workloads requiring true root access may break
- Existing containers and images may need to be recreated
