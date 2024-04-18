# Use an official Rust runtime as a parent image
FROM rust:1.54

# Set the working directory in the container to /app
WORKDIR /app

# Install ca-certificates
RUN apt-get update && apt-get install -y ca-certificates && update-ca-certificates

# Copy the current directory contents into the container at /app
COPY . /app

# Update Rust and Cargo, and update the dependencies
RUN rustup update && cargo update

# Switch to the nightly version of Rust
RUN rustup default nightly

# Build the application
RUN cargo build --release

# Set the startup command to run your binary
CMD ["/app/target/release/rust_python_interop"]