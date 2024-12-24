if (std.args(1) == "-f" || std.args(1) == "--file" && std.args(2))
{
    const something = await read_txt(std.args(2)).catch(err => {
        print(`${err}\n`);
        repl();
    });
    print(something);
    std.exit(0);
}

function repl()
{
    while (true)
    {
        const value = input("> ");
        print(`{ ${value} }\n`);
    }
}

repl();