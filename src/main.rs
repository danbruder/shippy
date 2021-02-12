mod domain;
mod infra;
mod prelude;
mod result;

#[tokio::main]
async fn main() {
    infra::cli::run().await.expect("Failed");
}
