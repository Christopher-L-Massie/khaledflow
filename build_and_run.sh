#!/bin/bash

# Step 1: Build the Docker image
echo "Building the Docker image..."
docker build -t khaledflow_image .

# Check if the build was successful
if [ $? -ne 0 ]; then
    echo "Docker build failed. Exiting."
    exit 1
fi

# Step 2: Run the Docker container
echo "Running the Docker container..."
docker run --rm khaledflow_image

# Check if the container run was successful
if [ $? -ne 0 ]; then
    echo "Docker run failed. Exiting."
    exit 1
fi

echo "Docker container ran successfully."
