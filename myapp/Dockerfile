# Use an official Rust runtime as a parent image
FROM rust:1.54-buster as builder

RUN rustup target add x86_64-unknown-linux-musl
# Set the working directory in the container to /app
WORKDIR /app

# Copy the current directory contents into the container at /app
COPY . /app

# Update Rust and Cargo, and update the dependencies
RUN rustup update && cargo update

# Switch to the nightly version of Rust
RUN rustup default nightly

# Install musl-tools and set the linker to musl-gcc
RUN apt-get update && apt-get install -y musl-tools || true
ENV RUSTFLAGS="-C linker=musl-gcc"
ENV CC=musl-gcc

# Build the application without the --release flag
RUN cargo build --target x86_64-unknown-linux-musl

# Use an official Python runtime as a parent image
FROM python:3.9-slim

# Set the working directory in the container to /app
WORKDIR /app

# Install the transformers library
RUN pip install transformers