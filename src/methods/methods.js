const { print } = Deno.core;

globalThis.std =
{
    args: pos =>
        Deno.core.ops.op_arg(pos),
    exit: arg =>
        Deno.core.ops.exit_program(arg),
    eval: arg =>
        Deno.core.ops.eval(arg),
    os: _ =>
        Deno.core.ops.get_os(),
    arch: _ =>
        Deno.core.ops.get_arch(),
}

var log_op = (err, fn, ...args) =>
    print((typeof fn == "function" ? fn(args.flat().join(" ")) : [fn, ...args].flat().join(" ")), err || 0);

globalThis.console =
{
    clear: _ =>
        (Deno.core.ops.op_clear(), 1) && globalThis.console,
    log: (fn, ...args) =>
        (log_op(0, fn, ...args), 1) && globalThis.console,
    err: (fn, ...args) =>
        (log_op(1, fn, ...args), 1) && globalThis.console,
    input: (fn, ...args) =>
        (log_op(0, fn, ...args), new Promise(resolve => resolve(Deno.core.ops.op_input())))
}

globalThis.process =
{
    tokenize: arg => Deno.core.ops.tokenize(arg),
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

    create = () =>
        Deno.core.ops.new_window(this.title)
}