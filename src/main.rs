#[tokio::main]
async fn main() -> std::io::Result<()> {
    mto_webserver::run().await
}
