use deno_core::{
    error::AnyError,
    extension
};

use std::rc::Rc;

mod methods;
use methods::*;

extension!(
    runjs,
    ops = [
        create_file,
        op_arg,
        op_input,
        read_txt_file,
        exit_program,
        op_error,
        delay,
        eval,
        tokenize
    ]
);

fn main()
{
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    if let Err(error) = runtime.block_on(run_js("./js/main.js")) {
        eprintln!("error: {}", error);
    }
}

async fn run_js(file_path: &str) -> Result<(), AnyError>
{
    let main_module = deno_core::resolve_path(file_path, &std::env::current_dir()?)?;

    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions
    {
        module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
        extensions: vec![runjs::init_ops_and_esm()],
        ..Default::default()
    });

    // Executa o código padrão (como a definição de `print`)
    js_runtime.execute_script("defaults.js", DEFAULTS)?;

    let mod_id = js_runtime.load_main_es_module(&main_module).await?;
    let result = js_runtime.mod_evaluate(mod_id);

    js_runtime.run_event_loop(Default::default()).await?;

    result.await
}