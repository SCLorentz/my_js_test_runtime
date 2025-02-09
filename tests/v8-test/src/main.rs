use std::{fs::File, io::Read};

use v8;

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let isolate = &mut v8::Isolate::new(Default::default());

    let scope = &mut v8::HandleScope::new(isolate);
    let context = v8::Context::new(scope, Default::default());
    let scope = &mut v8::ContextScope::new(scope, context);

    let code = v8::String::new(scope, &read_js_file()?).unwrap();

    let script = v8::Script::compile(scope, code, None).unwrap();
    let result = script.run(scope).unwrap();

    let function = result.to_object(scope).unwrap();
    let function = v8::Local::<v8::Function>::try_from(function)?;

    let undefined = v8::undefined(scope);

    let result = function.call(scope, undefined.into(), &[]).unwrap();
    let result = result.to_string(scope).unwrap();

    println!("{}", result.to_rust_string_lossy(scope));

    Ok(())
}

fn read_js_file() -> Result<String, std::io::Error>
{
    let mut file = File::open(".test.js")?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    Ok(contents)
}