use tracing::subscriber::set_global_default;
use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Registry};

/// We are using `impl Subscriber` as return type to avoid having to spell out the actual
pub fn get_subcriber() -> impl Subscriber + Sync + Send {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new("monitor".into(), std::io::stdout);

    let file_appender = tracing_appender::rolling::never("application_log", "application.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let fmt_layer = fmt::layer()
        .with_target(true) // don't include event targets when logging
        .with_level(true)
        .with_ansi(true)
        .compact()
        .pretty();

    let file_layer = fmt::layer()
        .with_target(true) // don't include event targets when logging
        .with_level(true)
        .with_ansi(true)
        .compact()
        .pretty()
        .with_writer(non_blocking);

    let subscriber = Registry::default()
        .with(env_filter)
        .with(fmt_layer)
        //.with(tracing_subscriber::fmt::layer())
        //.with(fmt::Layer::default().with_writer(non_blocking))
        .with(file_layer)
        //.with(JsonStorageLayer)
        //.with(formatting_layer)
        ;
    subscriber
}

/// Register a subscriber as global default to process span data.
pub fn init_subscriber(subscriber: impl Subscriber + Sync + Send) {
    set_global_default(subscriber).expect("Failed to set subscriber");
}
