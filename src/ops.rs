// https://github.com/denoland/deno_core/blob/main/ops/op2/README.md

use deno_core::op2;
use tokio::fs;

#[op2(async)]
#[string]
pub async fn op_read_file(#[string] path: String) -> anyhow::Result<String> {
    let content = fs::read_to_string(path).await?;
    Ok(content)
}

#[op2(async)]
#[string]
pub async fn op_write_file(
    #[string] path: String,
    #[string] contents: String,
) -> anyhow::Result<()> {
    fs::write(path, contents).await?;
    Ok(())
}

#[op2(async)]
#[string]
pub async fn op_remove_file(#[string] path: String) -> anyhow::Result<()> {
    fs::remove_file(path).await?;
    Ok(())
}
