use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn init_logging() {
    let format = tracing_subscriber::fmt::format()
        .with_level(true) // don't include levels in formatted output
        .with_target(true) // don't include targets
        .with_thread_names(true) // include the name of the current thread
        .with_source_location(false)
        .pretty()
        .with_source_location(false)
        .compact(); // use the `Compact` formatting style.

    // Create a `fmt` subscriber that uses our custom event format, and set it
    // as the default.
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer().event_format(format), /*.with_span_events(FmtSpan::ENTER)*/
        )
        .with(tracing_error::ErrorLayer::default())
        .with(
            EnvFilter::builder()
                //                .parse("INFO,hyperlit_core=DEBUG,hyperlit_engine=DEBUG")
                .parse("INFO")
                .unwrap(),
        )
        .init();
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        $crate::logging::error!($($arg)*);
    };
}

pub use log_error;
pub use tracing::debug;
pub use tracing::error;
pub use tracing::info;
pub use tracing::info_span;
pub use tracing::warn;
