use anyhow::Result;
use async_trait::async_trait;
use dotenv::dotenv;
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

// implementing the ValenceCoordinator trait on the above struct
// will provide us with the runtime helper functions like `start()`
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

#[tokio::main]
async fn main() -> Result<()> {
    // load environment variables
    let env_path = env::current_dir()?.join(".env");
    dotenv::from_path(env_path.as_path())?;
    dotenv().ok();

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
    let ticker = Ticker {
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
