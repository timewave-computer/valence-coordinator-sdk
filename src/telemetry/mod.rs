use opentelemetry_appender_log::OpenTelemetryLogBridge;
use opentelemetry_otlp::{Protocol, WithExportConfig};
use opentelemetry_sdk::{
    Resource,
    logs::{BatchLogProcessor, SdkLoggerProvider},
};

const DEFAULT_LOG_SERVICE_NAME: &str = "valence-coordinator";

/// utility method to configure coordinator logging.
/// if `otlp_endpoint` is provided, a multi-logger gets configured
/// so that logs are pushed both to the specified opentelemetry endpoint
/// and std.
/// otherwise, a simple env_logger is initialized.
pub fn setup_logging(otlp_endpoint: Option<String>) -> anyhow::Result<()> {
    if let Some(otlp_endpoint) = otlp_endpoint {
        // set up a opentelemetry log exporter with the given otlp_endpoint
        let otlp_exporter = opentelemetry_otlp::LogExporter::builder()
            .with_http()
            .with_protocol(Protocol::HttpBinary)
            .with_endpoint(otlp_endpoint)
            .build()?;

        // configure a logger resource with the given service name
        let logger_resource = Resource::builder()
            .with_service_name(DEFAULT_LOG_SERVICE_NAME)
            .build();

        let otlp_logger_provider = SdkLoggerProvider::builder()
            .with_resource(logger_resource)
            .with_log_processor(BatchLogProcessor::builder(otlp_exporter).build())
            .build();

        let otlp_logger = Box::new(OpenTelemetryLogBridge::new(&otlp_logger_provider));

        let std_logger = Box::new(env_logger::Builder::from_default_env().build());

        multi_log::MultiLogger::init(vec![otlp_logger, std_logger], log::Level::Trace)?;
    } else {
        env_logger::init();
    };

    Ok(())
}
