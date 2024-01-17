use bpaf::Bpaf;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
struct Cli {
    /// JS file to execute
    #[bpaf(positional("file"))]
    pub file: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let cli = cli().run();
    runjs::run_js(&cli.file).await?;
    Ok(())
}
