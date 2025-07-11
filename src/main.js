//import "https://raw.githubusercontent.com/SCLorentz/useless-ts-scripts/refs/heads/main/weirdo.js";

if (std.args(1) == "-f" || std.args(1) == "--file" && std.args(2))
{
    const something = await read_txt(std.args(2)).catch(_ => std.error("couldn't read file"));
    //
    console.log(`${something}\n`);
    std.exit(0);
}

if (std.args(1) == "-d")
{
    const value = "\x1b[32mWhat have you done?\x1b[39m\n";
    console.clear();

    for (let i=0; i<10; i++)
        value.split("").forEach(char => (print(char), 1) && delay(Math.floor(Math.random() * 300)))
    //
    std.exit(0);
}

var repl = (m,l=console.log,t=process.tokenize) => {while(1)l(t(input(m)))}

console.log("My repl with window! -", std.os(), std.arch());
let window = new Window("My Window").create();

repl("> ");