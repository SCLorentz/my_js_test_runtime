use deno_core::{
    error::AnyError, extension, JsRuntime
};

use std::{borrow::Cow, rc::Rc};

mod methods;
mod tokenize;
mod custom_module_loader;

use methods::*;
use custom_module_loader::*;

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
        tokenize,
        get_os,
        get_arch
    ]
);

const PROGRAM: &[u8] = include_bytes!("../js/main.js");

fn main()
{
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    if let Err(error) = runtime.block_on(run_js()) {
        eprintln!("error: {}", error);
    }
}

async fn run_js() -> Result<(), AnyError>
{
    let program: Cow<'static, str> = String::from_utf8_lossy(PROGRAM).into_owned().into();

    let main_module_url = deno_core::resolve_url_or_path("main.js", &std::env::current_dir()?)?;

    let mut js_runtime = JsRuntime::new(deno_core::RuntimeOptions
    {
        module_loader: Some(Rc::new(SimpleModuleLoader)),
        extensions: vec![runjs::init_ops_and_esm()],
        ..Default::default()
    });

    let defaults: Cow<'static, str> = String::from_utf8_lossy(DEFAULTS).into_owned().into();

    js_runtime.execute_script("defaults.js", defaults.clone().into_owned())?;

    let mod_id = js_runtime.load_main_es_module_from_code(&main_module_url, program.clone().into_owned()).await?;
    let result = js_runtime.mod_evaluate(mod_id);

    js_runtime.run_event_loop(Default::default()).await?;

    result.await
}