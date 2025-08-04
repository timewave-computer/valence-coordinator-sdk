use async_trait::async_trait;
use log::{error, info};

/// label for logging targets originating from the coordinator
const COORDINATOR: &str = "COORDINATOR";
const ERROR_BACKOFF_SECS: u64 = 1;

/// Base trait for Valence coordinators.
#[async_trait]
pub trait ValenceCoordinator {
    /// label of the implementing coordinator
    fn get_name(&self) -> String;

    /// cycle implementation will define the core business logic
    /// of a Valence Coordinator. this should contain the steps
    /// needed to carry out the desired set of actions.
    /// this implementation gets called by `fn start(self)` function
    /// below.
    async fn cycle(&mut self) -> anyhow::Result<()>;

    /// convenience helper to own a runtime on a new thread
    fn start(self) -> std::thread::JoinHandle<()>
    where
        Self: Sized + Send + 'static,
    {
        let name = self.get_name();
        info!(target: COORDINATOR, "starting coordinator: {name}");

        // start the coordinator in its own thread to own the runtime
        std::thread::spawn(move || {
            // create the tokio runtime on the current thread
            let rt = tokio::runtime::Runtime::new().expect("tokio runtime");

            // start looping inside this runtime
            rt.block_on(async {
                let mut worker = self;

                info!(target: COORDINATOR, "{name}: coordinator started in new runtime");

                loop {
                    match worker.cycle().await {
                        Ok(_) => {
                            info!(target: COORDINATOR, "{name}: cycle completed successfully");
                        }
                        Err(e) => {
                            error!(target: COORDINATOR, "{name}: error in cycle: {:?}", e);
                            // sleep a little just in case
                            tokio::time::sleep(tokio::time::Duration::from_secs(ERROR_BACKOFF_SECS)).await;
                        }
                    }
                }
            });
        })
    }
}
