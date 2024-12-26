use deno_core::serde;

use deno_core::{
    error::AnyError,
    op2,
    serde_json
};

//use std::collections::HashMap;

use std::{
    env, fs::File
};

use crate::runjs;

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
                Deno.core.ops.exit_program(arg),
            error: arg =>
                Deno.core.ops.op_error(arg),
            eval: arg =>
                Deno.core.ops.eval(arg)
        }

        globalThis.new_file = arg => Deno.core.ops.create_file(arg);

        globalThis.read_txt = arg => Deno.core.ops.read_txt_file(arg);

        globalThis.delay = arg => Deno.core.ops.delay(arg);

        globalThis.tokenize = arg => Deno.core.ops.tokenize(arg);
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

// create an eval function
#[op2(fast, reentrant)]
pub fn eval(#[string] arg: String) -> Result<(), AnyError>
{
    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions
    {
        module_loader: Some(std::rc::Rc::new(deno_core::FsModuleLoader)),
        extensions: vec![runjs::init_ops_and_esm()],
        ..Default::default()
    });

    js_runtime.execute_script("defaults.js", DEFAULTS)?;

    js_runtime.execute_script("eval.js", arg)?;

    Ok(())
}

#[derive(Debug, Clone, serde::Serialize)]
enum TokenType
{
    String,
    Number,
    Boolean,
    Null,
    Json,
    //
    Identifier,
    If,
    Else,
}

#[derive(Debug, serde::Serialize)]
struct Token
{
    token_type: TokenType,
    value: String
}

fn get_token_type(token: &str) -> TokenType
{
    // ! this is ineffective and shouldn't be used for production
    match token
    {
        c if c.parse::<serde_json::Value>().is_ok() => return TokenType::Json,
        c if c.parse::<f64>().is_ok() => return TokenType::Number,
        c if c.parse::<bool>().is_ok() => return TokenType::Boolean,
        "null" => return TokenType::Null,
        "if" => return TokenType::If,
        "else" => return TokenType::Else,
        c if c.parse::<String>().is_ok() => return TokenType::String,
        // TODO: add a method of sub-tokenization for more precise results
        _ => return TokenType::Identifier
    }
}

#[op2()]
#[serde]
pub fn tokenize(#[string] arg: String) -> Result<serde_json::Value, AnyError>
{
    let tokens = arg.split_whitespace().collect::<Vec<&str>>();

    let mut result: Vec<Token> = Vec::new();
    
    // TODO: create a recursive function here
    for token in tokens.clone()
    {
        let token_type = get_token_type(token);
        //
        result.push(Token { token_type, value: token.to_string() });
    }

    Ok(serde_json::json!(result))
}