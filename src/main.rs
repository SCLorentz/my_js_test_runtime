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

extension!(
    runjs,
    ops = [
        create_file,
        op_arg,
    ]
);

fn main()
{
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    if let Err(error) = runtime.block_on(run_js("./src/main.js")) {
        eprintln!("error: {}", error);
    }
}

static DEFAULTS: &str = r#"(
    function init() {
        globalThis.print = (...args) => {
            Deno.core.print(args, false);
        }
        globalThis.std = {
            arg: (pos) => {
                return Deno.core.ops.op_arg(pos);
            }
        }
        globalThis.file = (arg) => {
            return Deno.core.ops.create_file(arg);
        }
    }
)()"#;

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

#[op2(fast)]
fn create_file(#[string] path: String) -> Result<(), AnyError>
{
    println!("create file: {}", path);
    File::create(path)?;
    //
    Ok(())
}

#[op2()]
#[string]
fn op_arg(arg: i32) -> Result<String, AnyError>
{
    Ok(env::args().nth(arg as usize).unwrap())
}