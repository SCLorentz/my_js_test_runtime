use deno_core::serde;

use std::result::Result;

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum TokenType
{
    String,
    Number,
    Boolean,
    Null,
    //Json,
    //
    Identifier,
    If,
    Else,
    Var,
    Fn,
    //
    OpenParenthesis,
    CloseParenthesis,
    OpenBrace,
    CloseBrace,
    //
    Colon,
    EqualTo,
    Equals
}

#[derive(Debug, serde::Serialize, Clone)]
pub struct Token
{
    token_type: TokenType,
    value: String,
    args: Option<Vec<Token>>,
    children: Option<Vec<Token>>
}

fn get_token_type(token: &str) -> TokenType
{
    // ! this is ineffective and shouldn't be used for production
    match token
    {
        //c if c.parse::<serde_json::Value>().is_ok() => return TokenType::Json,
        c if c.parse::<f64>().is_ok() => return TokenType::Number,
        c if c.parse::<bool>().is_ok() => return TokenType::Boolean,
        "nil" => return TokenType::Null,
        "if" => return TokenType::If,
        "else" => return TokenType::Else,
        "(" => return TokenType::OpenParenthesis,
        ")" => return TokenType::CloseParenthesis,
        "var" => return TokenType::Var,
        ":" => return TokenType::Colon,
        "def" => return TokenType::Fn,
        "==" => return TokenType::EqualTo,
        "=" => return TokenType::Equals,
        "{" => return TokenType::OpenBrace,
        "}" => return TokenType::CloseBrace,
        c if c.parse::<String>().is_ok() => return TokenType::String,
        // TODO: add a method of sub-tokenization for more precise results
        _ => return TokenType::Identifier
    }
}

pub enum TokenizeKind<'a>
{
    String(&'a [&'a str]),
    Splited(std::slice::Iter<'a, &'a str>)
}

pub fn tokenize_loop(tokens: TokenizeKind) -> Result<Vec<Token>, &'static str>
{
    let mut tokens = match tokens
    {
        TokenizeKind::String(tokens) => tokens.iter(),
        TokenizeKind::Splited(tokens) => tokens
    };

    let mut result: Vec<Token> = Vec::new();

    // TODO: make changes to the code for more efficence, less redundancy and more abstraction
    while let Some(token) = tokens.next()
    {
        let token_type = get_token_type(token);

        if token_type == TokenType::CloseParenthesis
        {
            return Ok(result);
        }

        // TODO: make this work with braces and brackets as well
        if token_type == TokenType::OpenParenthesis
        {
            let args = tokenize_loop(TokenizeKind::Splited(tokens.clone()))?;

            if let Some(last) = result.last_mut()
            {
                last.args = Some(args);
            }

            return Ok(result);
        }

        result.push(Token { token_type, value: token.to_string(), args: None, children: None });
    }

    return Ok(result);
}

/* how a print function token should look

print("Hello world", 42)

{
    type: "NativeFunction",
    name: "print",
    args: Some([
        Token {
            type: "String",
            value: "Hello world"
            args: None
        },
        Token {
            type: "Number",
            value: "42"
            args: None
        }
    ])
}

notice that '()' is ignored at this part
*/

/*
writeln("hello world");
write("hello world\n");

readln("> ");

*/