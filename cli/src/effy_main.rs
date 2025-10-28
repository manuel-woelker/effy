use effy_base::logging::{debug, error, info, info_span, warn};
use tracing_error::SpanTrace;

fn main() {
    effy_base::logging::init_logging();
    color_eyre::install().unwrap();
    info!("🛈 effy");
    warn!("This is a warning");
    debug!("This is a debug message");
    error!("This is an error message");
    info_span!("foo").in_scope(|| {
        info_span!("bar").in_scope(|| {
            info!("fizzbuzz");
            dbg!(SpanTrace::capture());
            //let err: Result<(), _> = Err(anyhow::anyhow!("This is an example error")).context("This is a context");
            let err = color_eyre::eyre::anyhow!("This is an example error");
            println!("{:?}", err);
        });
    });
}
