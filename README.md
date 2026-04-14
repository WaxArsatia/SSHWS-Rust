# SSHWS-Rust

Minimal asynchronous TCP proxy written in Rust with Tokio.

## Overview

This project listens for client connections on port 8880, then opens a backend connection to localhost:22 and forwards traffic in both directions.

Before forwarding starts, it writes an HTTP 101 response to the client and consumes one initial client read.

## Features

- Asynchronous server built with Tokio runtime.
- Accepts connections on 0.0.0.0:8880.
- Connects each client session to localhost:22.
- Sends HTTP 101 Switching Protocols response to the client.
- Proxies traffic both ways using tokio::io::copy.

## Tech Stack

- Rust (edition 2021)
- Tokio (full feature set)

## Setup and Run

### Prerequisites

- Rust toolchain (rustc and cargo)

### Run

1. From the repository root, build and run:

   cargo run

2. The proxy will listen on:

   0.0.0.0:8880

3. Ensure an SSH service (or other TCP service you intend to proxy) is available at:

   localhost:22

## Project Structure

- Cargo.toml: package metadata, dependencies, release profile.
- Cargo.lock: dependency lockfile.
- src/main.rs: proxy server implementation.

## Contributing

No formal contribution guidelines are currently included. Open an issue or pull request if you want to propose improvements.
