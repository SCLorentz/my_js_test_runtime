if (std.args_len() > 1 && std.arg(1) == "-f" || std.arg(1) == "--file")
{
    try
    {
        print("Reading file...\n");
        print(read_txt(std.arg(2)));
        exit(0);
    }
    catch (err)
    {
        print(err);
        print("\nopening repl...\n");
        repl();
    }
}

function repl()
{
    while (true)
    {
        print("> ");
        const value = input();
        print(`{ ${value} }\n`);
    }
}

repl();