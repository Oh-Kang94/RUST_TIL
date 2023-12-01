mod run;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    run::run().await
}