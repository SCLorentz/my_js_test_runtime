use deno_core::serde;

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
    //
    Parenthesis,
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
        "null" => return TokenType::Null,
        "if" => return TokenType::If,
        "else" => return TokenType::Else,
        "(" => return TokenType::Parenthesis,
        ")" => return TokenType::Parenthesis,
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

pub fn tokenize_loop(tokens: &[&str]) -> Vec<Token>
{
    let mut result: Vec<Token> = Vec::new();

    while let Some(token) = tokens.iter().next()
    {
        let token_type = get_token_type(token);

        result.push(Token { token_type, value: token.to_string(), args: None});
    }

    return result;
}

/* how a print function token should look

{
    type: "NativeFunction",
    name: "print",
    args: [
        {
            type: "String",
            value: "Hello world"
        }
    ]
}

notice that '()' is ignored at this part
*/