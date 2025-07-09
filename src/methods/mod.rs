use crate::runjs;

use deno_core::{
    error::AnyError,
    op2,
    serde_json
};

use std::borrow::Cow;
use std::{
    env::{self, consts::{OS, ARCH}}, fs::File
};

pub mod tokenize;
pub mod window;

use tokenize::{tokenize_loop, TokenizeKind};

pub const DEFAULTS: &[u8] = include_bytes!("./methods.js");

#[op2(fast)]
pub fn create_file(#[string] path: String) -> Result<(), AnyError>
{
    File::create(path)?;
    Ok(())
}

#[op2()]
#[serde]
pub fn op_arg(arg: Option<i32>) -> Result<Option<serde_json::Value>, AnyError>
{
    let arg =  match arg
    {
        Some(arg) => arg,
        _ => return Ok(Some(env::args().into_iter().map(serde_json::Value::String).collect())),
    };

    match env::args().nth(arg as usize)
    {
        Some(arg) => return Ok(Some(serde_json::Value::String(arg))),
        _ => return Ok(None)
    };
}

#[op2()]
#[string]
pub fn op_input(#[string] prompt: String) -> Result<String, AnyError>
{
    print!("{}", prompt);
    //
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    //
    Ok(input.trim().to_string())
}

#[op2(async)]
#[string]
pub async fn read_txt_file(#[string] path: String) -> Result<String, AnyError>
{
    match tokio::fs::read_to_string(&path).await
    {
        Ok(content) => Ok(content),
        Err(_) => Err(deno_core::error::custom_error(
            "FileNotFound",
            format!("File not found at path: {}", path),
        )),
    }
}

#[op2(fast)]
pub fn exit_program(arg: i32) -> Result<(), AnyError>
{
    //println!("\nProgram exited with code: {}", arg);
    std::process::exit(arg)
}

#[op2(fast)]
pub fn op_error(#[string] arg: String) -> Result<(), AnyError>
{
    // painc!("{}", arg);
    println!("{}", arg);
    std::process::exit(1)
}

#[op2(async)]
pub async fn delay(arg: i32) -> Result<(), AnyError>
{
    std::thread::sleep(std::time::Duration::from_millis(arg as u64));
    Ok(())
}

#[op2(fast, reentrant)]
pub fn eval(#[string] arg: String) -> Result<(), AnyError>
{
    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions
    {
        module_loader: Some(std::rc::Rc::new(deno_core::FsModuleLoader)),
        extensions: vec![runjs::init_ops_and_esm()],
        ..Default::default()
    });

    let defaults: Cow<'static, str> = String::from_utf8_lossy(DEFAULTS).into_owned().into();

    js_runtime.execute_script("defaults.js", defaults.clone().into_owned())?;

    js_runtime.execute_script("eval.js", arg)?;

    Ok(())
}

#[op2()]
#[serde]
pub fn tokenize(#[string] arg: String) -> Result<serde_json::Value, AnyError>
{
    /*let result = tokenize_recursive(
        &arg.split_whitespace().collect::<Vec<&str>>(),
        None
    );*/
    let Some(result) = tokenize_loop(TokenizeKind::String(&arg.split_whitespace().collect::<Vec<&str>>())).ok() else
    {
        return Err(deno_core::error::custom_error("SyntaxError", "Invalid syntax"))
    };

    Ok(serde_json::json!(result))
}

#[op2()]
#[string]
pub fn get_os() -> Result<String, AnyError>
{
    return Ok(OS.to_string());
}

#[op2()]
#[string]
pub fn get_arch() -> Result<String, AnyError>
{
    return Ok(ARCH.to_string());
}