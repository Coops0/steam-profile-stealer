use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new()
        .route("/", get(root))
        .route("/check", get(check))
        .fallback(fallback);

    info!("Attempting to bind server...");
    let builder = Server::bind(&"0.0.0.0:8000".parse()?);

    success!("Successfully bound to port {}", 8000);
    builder.serve(app.into_make_service()).await?;

    Ok(())
}