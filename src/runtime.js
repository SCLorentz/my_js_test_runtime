const { core } = Deno;

function argsToMessage(...args) {
    return args.map((arg) => JSON.stringify(arg)).join(" ");
}

globalThis.console = {
    log: (...args) => {
        core.print(`[out]: ${argsToMessage(...args)}\n`, false);
    },
    error: (...args) => {
        core.print(`[err]: ${argsToMessage(...args)}\n`, true);
    },
};

globalThis.print = (...args) => {
    core.print(args, false);
};


globalThis.file = (arg) => {
    core.ops.create_file(arg);
}

globalThis.std = {
    arg: (pos) => {
        return core.ops.op_arg(pos);
    }
}