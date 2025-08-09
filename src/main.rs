use moth_budget::{app, logging::init};

#[tokio::main]
async fn main() {
    init().expect("Failed to initialize logging");
    app::serve().await;
}
