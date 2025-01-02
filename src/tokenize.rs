use deno_core::serde;

use std::result::Result;
use std::slice::Iter;

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
    //
    Parenthesis,
    Colon,
    Fn,
}

#[derive(Debug, serde::Serialize, Clone)]
pub struct Token
{
    token_type: TokenType,
    value: String,
    args: Option<Vec<Token>>
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
        "(" => return TokenType::Parenthesis,
        ")" => return TokenType::Parenthesis,
        "var" => return TokenType::Var,
        ":" => return TokenType::Colon,
        "def" => return TokenType::Fn,
        c if c.parse::<String>().is_ok() => return TokenType::String,
        // TODO: add a method of sub-tokenization for more precise results
        _ => return TokenType::Identifier
    }
}

/*pub fn tokenize_recursive(tokens: &[&str], _last: Option<TokenType>) -> Vec<Token>
{
    let mut result: Vec<Token> = Vec::new();
    //
    if let Some((first, rest)) = tokens.split_first()
    {
        let token_type = get_token_type(first);
        let extend_recursion = tokenize_recursive(rest, Some(token_type.clone()));

        result.push(Token { token_type, value: first.to_string(), args: None});
        result.extend(extend_recursion);
    }

    return result;
}*/

pub fn tokenize_loop(tokens: &[&str]) -> Result<Vec<Token>, &'static str>
{
    let mut result: Vec<Token> = Vec::new();
    let mut tokens = tokens.iter();

    while let Some(token) = tokens.next()
    {
        let token_type = get_token_type(token);

        if !tokenize_args(token_type.clone(), tokens.to_owned(), &mut result)
        {
            result.push(Token { token_type, value: token.to_string(), args: None});
        };
    }

    return Ok(result);
}

fn tokenize_args(token_type: TokenType, mut tokens: Iter<'_, &str>, result: &mut Vec<Token>) -> bool
{
    if token_type != TokenType::Parenthesis
    {
        return false
    }

    let mut args: Vec<Token> = Vec::new();

    while let Some(token) = tokens.next()
    {
        args.push(Token { token_type: get_token_type(token), value: token.to_string(), args: None});

        if token_type == TokenType::Parenthesis
        {
            break;
        }
    }

    tokens.next();

    if let Some(last) = result.last_mut() {
        last.args = Some(args);
    }

    true
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