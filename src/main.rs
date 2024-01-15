use std::rc::Rc;

use deno_core::error::AnyError;

async fn run_js(file_path: &str) -> Result<(), AnyError> {
    let current_dir = std::env::current_dir()?;
    let main_module = deno_core::resolve_path(file_path, &current_dir)?;
    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
        ..Default::default()
    });

    let mod_id = js_runtime.load_main_module(&main_module, None).await?;
    let result = js_runtime.mod_evaluate(mod_id);
    js_runtime.run_event_loop(Default::default()).await?;
    result.await
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run_js("example.js").await?;
    Ok(())
}
