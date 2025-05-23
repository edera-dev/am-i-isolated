#!/bin/bash

# Define pod name and image
POD_NAME="i-am-not-isolated"
IMAGE="ghcr.io/edera-dev/am-i-isolated:nightly"

# Check if the pod already exists
if kubectl get pod $POD_NAME &> /dev/null; then
  read -p "Pod $POD_NAME already exists. Do you want to delete it? (y/n): " choice
  if [[ "$choice" != "y" ]]; then
    echo "Exiting without creating a new pod."
    exit 0
  fi
  echo "Deleting existing pod $POD_NAME..."
  kubectl delete pod $POD_NAME
fi

# Run the pod with the specified parameters
echo "Creating pod $POD_NAME with hostPID and privileged container..."
kubectl run $POD_NAME \
  --image=$IMAGE \
  --image-pull-policy=Always \
  --overrides='
{
  "apiVersion": "v1",
  "spec": {
  "hostPID": true,
  "containers": [
    {
    "name": "'"$POD_NAME"'",
    "image": "'"$IMAGE"'",
    "securityContext": {
      "privileged": true
    },
    "tty": true
    }
  ]
  }
}' \
  --restart=Never

# Check if the run command succeeded
if [ $? -ne 0 ]; then
  echo "Failed to create pod $POD_NAME."
  exit 1
fi

echo "Pod created successfully. Waiting for it to be Completed..."

# Wait for 5 seconds before checking the pod status
sleep 5

# Attempt to check the pod status up to 5 times
for i in {1..5}; do
  status=$(kubectl get pod $POD_NAME -o jsonpath='{.status.phase}')
  if [ "$status" == "Succeeded" ]; then
  echo "Pod $POD_NAME has succeeded."
  break
  else
  echo "Attempt $i: Pod $POD_NAME has not succeeded. Current status: $status"
  sleep 5
  fi
done

# Final check after attempts
if [ "$status" != "Succeeded" ]; then
  echo "Pod has not Succeeded after multiple attempts. Check its status."
  kubectl get pod $POD_NAME -o wide
  exit 1
fi

# Display logs of the pod
echo "---- Logs for Pod: $POD_NAME ----"
kubectl logs $POD_NAME

# Delete the pod after showing logs
echo "Deleting pod $POD_NAME..."
kubectl delete pod $POD_NAME
