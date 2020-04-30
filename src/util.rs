use tracing::Level;
use tracing_subscriber;

pub fn init_trace() {
    std::env::set_var("RUST_BACKTRACE", "true");
    let _subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();
}
