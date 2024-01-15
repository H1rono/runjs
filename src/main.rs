#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    runjs::run_js("example.js").await?;
    Ok(())
}
