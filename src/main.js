//import "https://raw.githubusercontent.com/SCLorentz/useless-ts-scripts/refs/heads/main/weirdo.js";

/*if (std.args(1) == "-f" || std.args(1) == "--file" && std.args(2))
{
    const something = await read_txt(std.args(2)).catch(_ => std.error("couldn't read file"));
    //
    print(`${something}\n`);
    std.exit(0);
}*/

if (std.args(1) == "-d")
{
    const value = "this is a text printed with delay!\n";
    value.split("").forEach(char => (print(char), 1) && delay(Math.floor(Math.random() * 300)))
    //
    std.exit(0);
}

function repl()
{
    while (true)
    {
        const value = input("> ");
        //
        const tokens = tokenize(value);
        //
        console.log(tokens);
    }
}

console.log("My repl with window!");
let window = new Window("My Window").create();

repl();