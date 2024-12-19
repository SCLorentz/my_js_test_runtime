use deno_core::{
    error::AnyError,
    op2,
    extension
};

use std::{
    rc::Rc,
    fs::File,
    env
};

use deno_ast::MediaType;
use deno_ast::ParseParams;
use deno_core::ModuleLoadResponse;
use deno_core::ModuleSourceCode;

extension!(
    runjs,
    ops = [
        create_file,
    ],
    esm_entry_point = "ext:runjs/runtime.js",
    esm = [dir "src", "runtime.js"],
);

fn main()
{
    let args: Vec<String> = env::args().collect();
    //
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    if let Err(error) = runtime.block_on(run_js(&args[1])) {
        eprintln!("error: {}", error);
    }
}

struct TsModuleLoader;

impl deno_core::ModuleLoader for TsModuleLoader
{
    fn resolve(
        &self,
        specifier: &str,
        referrer: &str,
        _kind: deno_core::ResolutionKind,
    ) -> Result<deno_core::ModuleSpecifier, deno_core::error::AnyError>
    {
        deno_core::resolve_import(specifier, referrer).map_err(|e| e.into())
    }
}

async fn run_js(file_path: &str) -> Result<(), AnyError>
{
    let main_module =
    deno_core::resolve_path(file_path, &std::env::current_dir()?)?;
    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions
    {
        module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
        extensions: vec![runjs::init_ops_and_esm()],
        ..Default::default()
    });

    let mod_id = js_runtime.load_main_es_module(&main_module).await?;
    let result = js_runtime.mod_evaluate(mod_id);

    js_runtime.run_event_loop(Default::default()).await?;
    result.await
}

#[op2(fast)]
fn create_file(#[string] path: String) -> Result<(), AnyError>
{
    println!("create file: {}", path);
    File::create(path)?;
    //
    Ok(())
}