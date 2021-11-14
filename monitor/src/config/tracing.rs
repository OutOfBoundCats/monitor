use tracing::subscriber::set_global_default;
use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Registry};

pub struct HttpWriter;

impl std::io::Write for HttpWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let buf_len = buf.len();

        println!("from http wrtiter{:?}", buf);

        let s = match std::str::from_utf8(buf) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        println!("buffer in string is {}", &s);
        Ok(buf_len)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// We are using `impl Subscriber` as return type to avoid having to spell out the actual
pub fn get_subcriber() -> impl Subscriber + Sync + Send {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new("monitor".into(), std::io::stdout);

    let file_appender = tracing_appender::rolling::never("application_log", "application.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    //layer to log to standard output
    let fmt_layer = fmt::layer()
        .with_target(true) // don't include event targets when logging
        .with_level(true)
        .with_ansi(true)
        .compact()
        .pretty();

    //layer to log to file
    let file_layer = fmt::layer()
        .with_target(true) // don't include event targets when logging
        .with_level(true)
        .with_ansi(false)
        .compact()
        .pretty()
        .with_writer(non_blocking);

    //http layer
    let (non_blocking_http, _guard_http) = tracing_appender::non_blocking(HttpWriter);
    let http_layer = fmt::layer()
        .with_target(true) // don't include event targets when logging
        .with_level(true)
        .with_ansi(false)
        .compact()
        .with_writer(non_blocking_http);

    let subscriber = Registry::default()
        .with(env_filter)
        .with(fmt_layer)
        .with(file_layer)
        .with(JsonStorageLayer)
        //.with(formatting_layer)
        .with(http_layer);

    subscriber
}

/// Register a subscriber as global default to process span data.
pub fn init_subscriber(subscriber: impl Subscriber + Sync + Send) {
    set_global_default(subscriber).expect("Failed to set subscriber");
}

fn http_writer() -> impl std::io::Write {
    std::io::stdout()
}
