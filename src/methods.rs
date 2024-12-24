use deno_core::{
    error::AnyError,
    op2,
    serde_json
};

use std::{
    env, fs::File
};

pub static DEFAULTS: &str = r#"(
    function init()
    {
        globalThis.print = (...args) => Deno.core.print(args, false);

        globalThis.input = (...args) => {
            if (args.length > 0) Deno.core.print(args, true);
            return Deno.core.ops.op_input();
        }

        globalThis.std = {
            args: pos =>
                Deno.core.ops.op_arg(pos),
            exit: arg =>
                Deno.core.ops.exit_program(arg)
        }

        globalThis.new_file = arg => Deno.core.ops.create_file(arg);

        globalThis.read_txt = arg => Deno.core.ops.read_txt_file(arg);

        globalThis.get_array = () => Deno.core.ops.get_array();
    }
)()"#;

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
        None => return Ok(Some(env::args().into_iter().map(serde_json::Value::String).collect())),
        Some(arg) => arg
    };

    match env::args().nth(arg as usize)
    {
        Some(arg) => return Ok(Some(serde_json::Value::String(arg))),
        None => return Ok(None)
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
    println!("\nProgram exited with code: {}", arg);
    std::process::exit(arg)
}

#[op2]
#[serde]
pub fn get_array() -> Result<Vec<String>, AnyError> {
    let values = vec![
        "Valor1".to_string(),
        "Valor2".to_string(),
        "Valor3".to_string(),
    ];
    Ok(values)
}