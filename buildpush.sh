#!/bin/bash -e

# Builds distroless Docker images with the application and pushes them to Docker Hub
# Currently, intended to be run from a local machine

APP_NAME="hello_world"
DOCKER_HUB="YourDockerHubUsername"

# Step 1: Build the Docker image and push to Docker Hub
# This builds and pushes ARM32, ARM64, and X64 images
docker buildx create --use --name mybuilder || true
docker buildx use mybuilder
docker buildx inspect --bootstrap
docker buildx build . --platform linux/amd64,linux/arm64,linux/arm/v7 -t $DOCKER_HUB/$APP_NAME:latest --push

