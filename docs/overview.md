# Overview

The Valence Coordinator SDK is a Rust crate meant for building off-chain
coordinators for the Valence Protocol.

It is primarily designed for developers building Valence Protocol programs
that need some flavor of runtime management. This may be as simple as a ticker
bot that attempts to advance the program state, or more complicated, i.e. a
circuit breaker that gets triggered by some cross-domain condition.

This SDK provides the necessary tools and libraries to build robust and efficient
coordinators that can operate on blockchains supported by the Valence Protocol.

## Key Features

* **Off-chain Coordination:** Build and run custom off-chain logic that interacts
  with on-chain smart contracts, ZK co-processor, indexers, or any other elements
  of the system
* **Cross-Chain Compatibility:** Interact with CosmWasm, EVM, and SVM (soon)
* **Valence Protocol Integration:** Seamlessly interact with the Valence Protocol,
  utilizing its core (authorization + processor) and integration libraries
* **Valence Coprocessor:** Leverage the Valence Coprocessor for high-performance,
  off-chain computation
* **Telemetry:** Built-in support for telemetry using OpenTelemetry, allowing
  for robust monitoring and observability of your coordinator
* **Asynchronous:** Built with `tokio` to enable flexible management of sync/async
  transmissions

## Getting Started

To get started with the Valence Coordinator SDK, check out the [Quickstart](./quickstart.md) guide.
