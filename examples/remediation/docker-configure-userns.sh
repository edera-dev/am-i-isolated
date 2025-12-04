#!/bin/bash
# Configure Docker daemon to use user namespace remapping
# This improves container isolation by mapping container root to an unprivileged host user
#
# WARNING: This affects all containers and may break some workloads that require true root.
# Existing containers/images may need to be recreated after enabling this.

set -euo pipefail

DAEMON_CONFIG="/etc/docker/daemon.json"

if [[ $EUID -ne 0 ]]; then
    echo "Error: This script must be run as root (sudo)"
    exit 1
fi

# Check if subuid/subgid are configured for dockremap
if ! grep -q "^dockremap:" /etc/subuid 2>/dev/null; then
    echo "Creating dockremap user and configuring subordinate IDs..."
    useradd -r -s /bin/false dockremap || true
    echo "dockremap:100000:65536" >> /etc/subuid
    echo "dockremap:100000:65536" >> /etc/subgid
fi

# Backup existing config if present
if [[ -f "$DAEMON_CONFIG" ]]; then
    cp "$DAEMON_CONFIG" "${DAEMON_CONFIG}.backup.$(date +%Y%m%d%H%M%S)"
    echo "Backed up existing $DAEMON_CONFIG"

    # Check if userns-remap is already configured
    if grep -q "userns-remap" "$DAEMON_CONFIG"; then
        echo "userns-remap already configured in $DAEMON_CONFIG"
        echo "Current config:"
        cat "$DAEMON_CONFIG"
        exit 0
    fi

    # Merge with existing config using jq if available, otherwise warn
    if command -v jq &>/dev/null; then
        jq '. + {"userns-remap": "default"}' "$DAEMON_CONFIG" > "${DAEMON_CONFIG}.tmp"
        mv "${DAEMON_CONFIG}.tmp" "$DAEMON_CONFIG"
    else
        echo "Warning: jq not found. Please manually add \"userns-remap\": \"default\" to $DAEMON_CONFIG"
        exit 1
    fi
else
    # Create new config
    echo '{"userns-remap": "default"}' > "$DAEMON_CONFIG"
fi

echo "Docker daemon configured with user namespace remapping"
echo "Config:"
cat "$DAEMON_CONFIG"

echo ""
echo "Restart Docker to apply changes:"
echo "  sudo systemctl restart docker"
echo ""
echo "To verify, run a container and check /proc/self/uid_map:"
echo "  docker run --rm alpine cat /proc/self/uid_map"
echo ""
echo "To disable, remove \"userns-remap\" from $DAEMON_CONFIG and restart Docker"
