use deno_core::{error::AnyError, extension, JsRuntime, PollEventLoopOptions};
use winit::event_loop::EventLoop;
use std::sync::mpsc::Sender;
use std::{borrow::Cow, rc::Rc, sync::mpsc};

mod methods;
mod module;

use methods::*;
use module::*;

use crate::methods::window::{new_window, App};
use crate::window::Command;

extension!(
    runjs,
    ops = [
        create_file,
        op_arg,
        op_input,
        read_txt_file,
        exit_program,
        delay,
        eval,
        get_os,
        get_arch,
        new_window,
        op_clear
    ]
);

const PROGRAM: &[u8] = include_bytes!("./main.js");

fn main()
{
    use tokio::runtime;
    use std::thread;

    let (tx, rx) = mpsc::channel::<Command>();

    let tx_clone = tx.clone();

    let _script = thread::spawn(move ||
    {
        let runtime = runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        if let Err(err) = runtime.block_on(run_js(tx_clone)) { panic!("{}", err) }
    });

    let event_loop = EventLoop::new().unwrap();
    let mut app = App::new(rx);

    let _ = event_loop.run_app(&mut app);
}

async fn run_js(tx: Sender<Command>) -> Result<(), AnyError>
{
    let program: Cow<'static, str> = String::from_utf8_lossy(PROGRAM).into_owned().into();

    let main_module_path = deno_core::resolve_url_or_path("main.js", &std::env::current_dir()?)?;

    let mut js_runtime = JsRuntime::new(deno_core::RuntimeOptions {
        extensions: vec![runjs::init_ops_and_esm()],
        module_loader: Some(Rc::new(SimpleModuleLoader)),
        startup_snapshot: None,
        ..Default::default()
    });

    js_runtime.op_state().borrow_mut().put::<Sender<Command>>(tx);

    let defaults: Cow<'static, str> = String::from_utf8_lossy(DEFAULTS).into_owned().into();

    js_runtime.execute_script("defaults.js", defaults.clone().into_owned())?;

    let mod_id = js_runtime.load_main_es_module_from_code(&main_module_path, program.clone().into_owned()).await?;
    js_runtime.mod_evaluate(mod_id).await?;
    js_runtime.run_event_loop(PollEventLoopOptions
    {
        pump_v8_message_loop: true,
        wait_for_inspector: false,
    }).await?;

    Ok(())
}