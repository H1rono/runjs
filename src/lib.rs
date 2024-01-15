pub async fn run_js(file_path: &str) -> anyhow::Result<()> {
    let current_dir = std::env::current_dir()?;
    let main_module = deno_core::resolve_path(file_path, &current_dir)?;

    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(std::rc::Rc::new(deno_core::FsModuleLoader)),
        ..Default::default()
    });
    let runtimejs_src = deno_core::FastString::Static(include_str!("runtime.js"));
    js_runtime.execute_script("[runjs:runtime.js]", runtimejs_src)?;

    let mod_id = js_runtime.load_main_module(&main_module, None).await?;
    let result = js_runtime.mod_evaluate(mod_id);
    js_runtime.run_event_loop(Default::default()).await?;
    result.await
}
