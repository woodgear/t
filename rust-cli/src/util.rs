use tracing::Level;

pub fn init_trace() {
    std::env::set_var("RUST_BACKTRACE", "true");
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();
}
