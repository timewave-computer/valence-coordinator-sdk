# Quickstart

This guide will walk you through the process of building a simple coordinator
using the Valence Coordinator SDK.

We will be building a "ticker bot" that periodically calls the `tick`
function on a Valence Protocol processor contract.

The end result of this guide can be found in [`examples/minimal/`](../examples/minimal/README.md).
It also includes the instructions for running the project.

## Prerequisites

* Rust and Cargo installed
* A Neutron GRPC endpoint
* A deployed Valence Protocol processor contract on Neutron

## 1. Setting up the project

First, create a new Rust project:

```bash
cargo new my-ticker-bot
cd my-ticker-bot
```

Next, add the `valence-coordinator-sdk` and other necessary dependencies to your `Cargo.toml` file:

```toml
[dependencies]
anyhow = "1"
async-trait = "0.1.88"
log = "0.4.22"
tokio = { version = "1.45.1", features = ["macros", "rt-multi-thread"] }
valence-coordinator-sdk = "0.1.0"
valence-domain-clients = "1.1.0"
dotenv = "0.15"
```

## 2. Defining the Coordinator struct

Now, let's define the structure of our coordinator in `src/main.rs`.

The coordinator will hold the necessary state, such as the client for
interacting with the blockchain and the address of the processor contract.

Coordinator is a runtime object that implements the actor model, where each
coordinator is an independent entity that communicates with other actors
(such as blockchain nodes, indexers, ZK coprocessor, etc.) through async
messages. It encapsulates its own state and logic, and its behavior is
defined by the `cycle` method, which is executed repeatedly in a non-blocking
manner.

```rust
use anyhow::Result;
use async_trait::async_trait;
use log::info;
use std::{env, time::Duration};
use valence_coordinator_sdk::{coordinator::ValenceCoordinator, telemetry};
use valence_domain_clients::clients::neutron::NeutronClient;

// top level coordinator struct holding the neutron client
// and any information relevant to its functionality
struct Ticker {
    coordinator_label: String,
    client: NeutronClient,
    processor_addr: String,
    tick_interval_secs: u64,
}
```

## 3. Implementing the `ValenceCoordinator` Trait

The core logic of the coordinator is implemented through the `ValenceCoordinator`
trait. This trait has two methods that must be implemented: `get_name` and `cycle`,
and one method that comes with a default implementation: `start`.

* `get_name`: Returns the name of the coordinator (for logging purposes)
* `cycle`: This is where the main logic of the coordinator resides.
  It will be called repeatedly in a loop
* `start`: A helper method that starts the coordinator in its own thread
  and enters into an infinite loop that calls `cycle`. You can override the
  default implementation if it does not suit your needs.

```rust
#[async_trait]
impl ValenceCoordinator for Ticker {
    fn get_name(&self) -> String {
        self.coordinator_label.to_string()
    }

    async fn cycle(&mut self) -> Result<()> {
        info!("{} cycle about to tick...", self.get_name());

        match valence_coordinator_sdk::core::cw::tick(&self.client, &self.processor_addr).await {
            Ok(_) => info!("tock! successfully ticked the processor"),
            Err(e) => log::warn!("ticking the processor failed: {e}"),
        };

        info!("sleeping for {}sec...", self.tick_interval_secs);
        tokio::time::sleep(Duration::from_secs(self.tick_interval_secs)).await;

        Ok(())
    }
}
```

## 4. The `main` Function

Finally, we need to write the `main` function to initialize and run the coordinator. This involves:

1.  Loading environment variables.
2.  Setting up logging.
3.  Initializing the `NeutronClient`.
4.  Creating an instance of our `Ticker`.
5.  Calling the `start` method on the `Ticker` instance to run the coordinator.

```rust
#[tokio::main]
async fn main() -> Result<()> {
    // load the examples env file
    dotenv::from_filename(".env").ok();

    // set up logging with no telemetry
    telemetry::setup_logging(None)?;

    // fetch the env variables
    let coordinator_label = env::var("LABEL")?;
    let processor_addr = env::var("NEUTRON_PROCESSOR_ADDR")?;
    let tick_interval_secs: u64 = env::var("TICK_INTERVAL_SECS")?.parse()?;

    // initialize a neutron client
    let client = NeutronClient::new(
        &env::var("NEUTRON_GRPC_URL")?,
        &env::var("NEUTRON_GRPC_PORT")?,
        &env::var("MNEMONIC")?,
        &env::var("NEUTRON_CHAIN_ID")?,
    )
    .await?;

    // construct the ticker in order to start the coordinator
    let mut ticker = Ticker {
        coordinator_label,
        client,
        processor_addr,
        tick_interval_secs,
    };

    // get the join handle of the ticker thread
    let handle = ticker.start();

    // wait for the ticker thread to finish (which should not happen
    // because it enters an infinite loop)
    let _ = handle.join();

    Ok(())
}
```
