# Use the official Rust image as the base image
FROM rust:latest


# TODO - This may or may not come up as complexity grows.
# # Install necessary tools and libraries
# RUN apt-get update && apt-get install -y \
#     build-essential \
#     clang \
#     libclang-dev \
#     llvm-dev \
#     && rm -rf /var/lib/apt/lists/*

# Set the working directory inside the container
WORKDIR /usr/src/khaledflow

# Copy the project files into the container
COPY khaledflow .

# Build the project
RUN cargo build --release

# Command to run the project
CMD ["cargo", "run", "--release"]