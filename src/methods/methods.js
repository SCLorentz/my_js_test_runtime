globalThis.print = (...args) => Deno.core.print(args, false);

globalThis.input = (...args) => ((args.length > 0) && Deno.core.print(args, true), Deno.core.ops.op_input());

globalThis.std =
{
    args: pos =>
        Deno.core.ops.op_arg(pos),
    exit: arg =>
        Deno.core.ops.exit_program(arg),
    error: arg =>
        Deno.core.ops.op_error(arg),
    eval: arg =>
        Deno.core.ops.eval(arg),
    os: _ =>
        Deno.core.ops.get_os(),
    arch: _ =>
        Deno.core.ops.get_arch()
}

globalThis.new_file = arg => Deno.core.ops.create_file(arg);

globalThis.read_txt = arg => Deno.core.ops.read_txt_file(arg);

globalThis.delay = arg => Deno.core.ops.delay(arg);

globalThis.tokenize = arg => Deno.core.ops.tokenize(arg);

globalThis.Window = class
{
    constructor(title)
    {
        this.title = title;
    }

    create()
    {
        Deno.core.ops.new_window(this.title)
    }
}