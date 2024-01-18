use deno_ast::MediaType;
use deno_core::{ModuleSpecifier, ModuleType};

#[derive(Clone, Copy)]
pub struct TsModuleLoader;

impl TsModuleLoader {
    pub async fn load_module(
        &self,
        specifier: &ModuleSpecifier,
    ) -> anyhow::Result<deno_core::ModuleSource> {
        use deno_core::{FastString, ModuleSource, ModuleSourceCode};

        let path = specifier.to_file_path().map_err(|_| {
            anyhow::anyhow!("could not parse as a local file path: `{}`", specifier)
        })?;

        // determine MediaType from file path extension
        let media_type = MediaType::from_path(&path);
        // whether transpiling is required
        let (module_type, should_transpile) = match media_type {
            MediaType::JavaScript | MediaType::Mjs | MediaType::Cjs => {
                (ModuleType::JavaScript, false)
            }
            MediaType::Jsx => (ModuleType::JavaScript, false),
            MediaType::TypeScript
            | MediaType::Mts
            | MediaType::Cts
            | MediaType::Dts
            | MediaType::Dmts
            | MediaType::Dcts
            | MediaType::Tsx => (ModuleType::JavaScript, true),
            MediaType::Json => (ModuleType::Json, false),
            _ => {
                anyhow::bail!(
                    "unsupported extension: {:?}",
                    path.extension().unwrap_or(std::ffi::OsStr::new(""))
                )
            }
        };

        // read the file, transpile if necessary
        let code = std::fs::read_to_string(path)?;
        if !should_transpile {
            let code = ModuleSourceCode::String(FastString::Owned(code.into_boxed_str()));
            return Ok(ModuleSource::new(module_type, code, specifier));
        }

        // transpile
        let parsed = deno_ast::parse_module(deno_ast::ParseParams {
            specifier: specifier.to_string(),
            text_info: deno_ast::SourceTextInfo::from_string(code),
            media_type,
            capture_tokens: false,
            scope_analysis: false,
            maybe_syntax: None,
        })?;
        let code = parsed.transpile(&deno_ast::EmitOptions::default())?.text;
        let code = ModuleSourceCode::String(FastString::Owned(code.into_boxed_str()));
        Ok(ModuleSource::new(module_type, code, specifier))
    }
}

impl deno_core::ModuleLoader for TsModuleLoader {
    fn resolve(
        &self,
        specifier: &str,
        referrer: &str,
        _kind: deno_core::ResolutionKind,
    ) -> anyhow::Result<ModuleSpecifier> {
        deno_core::resolve_import(specifier, referrer).map_err(anyhow::Error::from)
    }

    fn load(
        &self,
        module_specifier: &ModuleSpecifier,
        _maybe_referrer: Option<&ModuleSpecifier>,
        _is_dyn_import: bool,
        _requested_module_type: deno_core::RequestedModuleType,
    ) -> std::pin::Pin<Box<deno_core::ModuleSourceFuture>> {
        let s = *self;
        let module_specifier = module_specifier.clone();
        Box::pin(async move { s.load_module(&module_specifier).await })
    }
}
