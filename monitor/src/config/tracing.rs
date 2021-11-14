use tracing::subscriber::set_global_default;
use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Registry};

/// We are using `impl Subscriber` as return type to avoid having to spell out the actual
pub fn get_subcriber() -> impl Subscriber + Sync + Send {
    //initialize thetracing crate
    LogTracer::init().expect("Failed to set logger");

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // gives json output og log which we utilize to write to db
    let formatting_layer = BunyanFormattingLayer::new("zero2prod".into(), std::io::stdout);

    // specify how the log file should get created
    let file_appender = tracing_appender::rolling::never("application_log", "application.log");

    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    //make subscriber which  logs tracing events to console as well as file
    Registry::default()
        .with(env_filter)
        .with(fmt::Layer::default().with_writer(non_blocking))
        .with(JsonStorageLayer)
}

/// Register a subscriber as global default to process span data.
pub fn init_subscriber(subscriber: impl Subscriber + Sync + Send) {
    set_global_default(subscriber).expect("Failed to set subscriber");
}
