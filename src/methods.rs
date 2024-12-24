use deno_core::{
    anyhow,
    error::AnyError,
    op2
};

use crate::methods::anyhow::anyhow;

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
            arg: (pos) => {
                return Deno.core.ops.op_arg(pos);
            },
            args_len: () => Deno.core.ops.arg_len()
        }

        globalThis.new_file = arg => Deno.core.ops.create_file(arg);

        globalThis.read_txt = arg => Deno.core.ops.read_txt_file(arg);

        globalThis.exit = arg => Deno.core.ops.exit_program(arg);

    }
)()"#;

#[op2(fast)]
pub fn create_file(#[string] path: String) -> Result<(), AnyError>
{
    File::create(path)?;
    Ok(())
}

#[op2()]
#[string]
pub fn op_arg(arg: i32) -> Result<Option<String>, AnyError>
{
    let args = match env::args().nth(arg as usize) {
        Some(arg) => Some(arg),
        None => return Ok(None)
    };
    Ok(Some(args.unwrap()))
}

#[op2(fast)]
pub fn arg_len() -> Result<i32, AnyError>
{
    Ok((env::args().len() - 1) as i32)
}

#[op2()]
#[string]
pub fn op_input() -> Result<String, AnyError>
{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    //
    Ok(input.trim().to_string())
}

#[op2()]
#[string]
pub fn read_txt_file(#[string] path: String) -> Result<String, AnyError>
{
    Ok(std::fs::read_to_string(path)?)
}

#[op2(fast)]
pub fn exit_program(path: i32) -> Result<(), AnyError>
{
    std::process::exit(path)
}