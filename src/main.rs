mod core;
mod engine;
mod api;

#[tokio::main]
async fn main() {
    println!("Starting AD-Pathfinder-Engine server...");
    if let Err(e) = api::server::run().await {
        eprintln!("Server error: {}", e);
    }
}
