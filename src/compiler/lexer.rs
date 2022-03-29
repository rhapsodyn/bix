use anyhow::{bail, Result};

#[derive(Debug)]
pub(crate) enum Token {
    Reserved(Reserved),
    ///  1.1
    Number(f64),
    /// "2"
    String(String),
    /// `{`
    LeftBrace,
    /// `}`
    RightBrace,
    /// `(`
    LeftParenthesis,
    /// `)`
    RightParenthesis,
    /// `=`
    Assign,
    /// `==`
    Equal,
    /// [a-z|A-Z|0-9|_]
    Identifier(String),
    /// `+`
    OpsAdd,
    /// `-`
    OpsSub,
    /// `*`
    OpsMul,
    /// `/`
    OpsDiv,
    /// `,`
    Comma,
}

#[derive(Debug)]
pub(crate) enum Reserved {
    ///  `function`
    Function,
    /// `let`
    Let,
    /// `if`
    If,
    /// `else`
    Else,
    /// `true`
    True,
    /// `false`
    False,
    /// `return`
    Return,
    // For, // TODO loop
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Identifier(l0), Self::Identifier(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

fn is_num(c: char, at_begining: bool) -> bool {
    let is_num_or_dot = c.is_ascii_digit() || c == '.';
    if at_begining {
        is_num_or_dot || c == '-'
    } else {
        is_num_or_dot
    }
}

fn is_id(c: char, at_begining: bool) -> bool {
    let is_alpha = c.is_ascii_alphabetic() || c == '_';
    if at_begining {
        is_alpha
    } else {
        is_alpha || c.is_ascii_digit()
    }
}

// fn is_brk(c: char) -> bool {
//     c == ' ' || c == ';' || c == '\t'
// }

fn read_num(chars: &Vec<char>, begin: usize) -> Result<(Token, usize)> {
    let mut end = begin + 1;
    while end < chars.len() {
        if !is_num(chars[end], false) {
            break;
        } else {
            end += 1;
        }
    }

    let num = String::from_iter(&chars[begin..end]).parse::<f64>();
    match num {
        Ok(n) => Ok((Token::Number(n), end)),
        Err(e) => bail!(e),
    }
}

fn read_id(chars: &Vec<char>, begin: usize) -> Result<(Token, usize)> {
    let mut end = begin + 1;
    while end < chars.len() {
        if !is_id(chars[end], false) {
            break;
        } else {
            end += 1;
        }
    }

    let id_or_reserved = String::from_iter(&chars[begin..end]);
    match read_reserved(&id_or_reserved) {
        Some(t) => Ok((t, end)),
        None => Ok((Token::Identifier(id_or_reserved), end)),
    }
}

fn read_reserved(s: &str) -> Option<Token> {
    match s {
        "function" => Some(Token::Reserved(Reserved::Function)),
        "let" => Some(Token::Reserved(Reserved::Let)),
        "if" => Some(Token::Reserved(Reserved::If)),
        "else" => Some(Token::Reserved(Reserved::Else)),
        "true" => Some(Token::Reserved(Reserved::True)),
        "false" => Some(Token::Reserved(Reserved::False)),
        "return" => Some(Token::Reserved(Reserved::Return)),
        _ => None,
    }
}

pub(crate) fn tokenize(source: &str) -> Result<Vec<Token>> {
    let chars = source.chars().collect::<Vec<char>>();

    let mut result = vec![];
    let mut cursor = 0;

    while cursor < chars.len() {
        let c = chars[cursor];
        match c {
            ' ' | '\t' | ';' | '\n' => cursor += 1,
            '{' => {
                result.push(Token::LeftBrace);
                cursor += 1;
            }
            '}' => {
                result.push(Token::RightBrace);
                cursor += 1;
            }
            '(' => {
                result.push(Token::LeftParenthesis);
                cursor += 1;
            }
            ',' => {
                result.push(Token::Comma);
                cursor += 1;
            }
            ')' => {
                result.push(Token::RightParenthesis);
                cursor += 1;
            }
            '=' => {
                if chars[cursor + 1] == '=' {
                    result.push(Token::Equal);
                    cursor += 2;
                } else {
                    result.push(Token::Assign);
                    cursor += 1;
                }
            }
            '"' => {
                let mut end = cursor + 1;
                let mut nailed = false;
                while end < chars.len() {
                    if chars[end] == '"' {
                        result.push(Token::String(String::from_iter(&chars[cursor + 1..end])));
                        cursor = end + 1;
                        nailed = true;
                        break;
                    } else {
                        end += 1;
                    }
                }
                if !nailed {
                    bail!("unclosed string, at {}", cursor);
                }
            }
            '+' => {
                result.push(Token::OpsAdd);
                cursor += 1;
            }
            '-' => {
                if cursor != chars.len() - 1 && is_num(chars[cursor + 1], false) {
                    match read_num(&chars, cursor) {
                        Ok((t, end)) => {
                            result.push(t);
                            cursor = end;
                        }
                        Err(e) => bail!(e),
                    }
                } else {
                    result.push(Token::OpsSub);
                    cursor += 1;
                }
            }
            '*' => {
                result.push(Token::OpsMul);
                cursor += 1;
            }
            '/' => {
                result.push(Token::OpsDiv);
                cursor += 1;
            }
            _ => {
                if is_num(c, true) {
                    match read_num(&chars, cursor) {
                        Ok((t, n)) => {
                            result.push(t);
                            cursor = n;
                        }
                        Err(e) => bail!(e),
                    }
                } else if is_id(c, true) {
                    match read_id(&chars, cursor) {
                        Ok((t, n)) => {
                            result.push(t);
                            cursor = n;
                        }
                        Err(e) => bail!(e),
                    }
                } else {
                    bail!("unexpected char {} at {}", c, cursor)
                }
            }
        }
    }

    Ok(result)
}

#[test]
fn test_lex_single_assign() {
    let tokens = tokenize("let _a_0 = -1.2;").unwrap();
    assert_tokens(
        &tokens,
        &vec![
            Token::Reserved(Reserved::Let),
            Token::Identifier("_a_0".to_owned()),
            Token::Assign,
            Token::Number(-1.2),
        ],
    )
}

#[test]
fn test_lex_if_block() {
    assert_tokens(
        &tokenize(
            "
        if (true) }
            print(\"-a+\", 1);
        {
    ",
        )
        .unwrap(),
        &vec![
            Token::Reserved(Reserved::If),
            Token::LeftParenthesis,
            Token::Reserved(Reserved::True),
            Token::RightParenthesis,
            Token::RightBrace,
            Token::Identifier("print".into()),
            Token::LeftParenthesis,
            Token::String("-a+".into()),
            Token::Comma,
            Token::Number(1.0),
            Token::RightParenthesis,
            Token::LeftBrace,
        ],
    )
}

#[test]
fn test_lex_if_else() {
    assert_tokens(
        &tokenize(
            "
        let a = 1;
        if (a == 1) {
            a = b + 1;
        } else if (a == 2) {
            a = b * 2 - b / 2;
        }
    ",
        )
        .unwrap(),
        &vec![
            Token::Reserved(Reserved::Let),
            Token::Identifier("a".into()),
            Token::Assign,
            Token::Number(1.0),
            Token::Reserved(Reserved::If),
            Token::LeftParenthesis,
            Token::Identifier("a".into()),
            Token::Equal,
            Token::Number(1.0),
            Token::RightParenthesis,
            Token::LeftBrace,
            Token::Identifier("a".into()),
            Token::Assign,
            Token::Identifier("b".into()),
            Token::OpsAdd,
            Token::Number(1.0),
            Token::RightBrace,
            Token::Reserved(Reserved::Else),
            Token::Reserved(Reserved::If),
            Token::LeftParenthesis,
            Token::Identifier("a".into()),
            Token::Equal,
            Token::Number(2.0),
            Token::RightParenthesis,
            Token::LeftBrace,
            Token::Identifier("a".into()),
            Token::Assign,
            Token::Identifier("b".into()),
            Token::OpsMul,
            Token::Number(2.0),
            Token::OpsSub,
            Token::Identifier("b".into()),
            Token::OpsDiv,
            Token::Number(2.0),
            Token::RightBrace,
        ],
    )
}

#[test]
fn test_lex_function() {
    assert_tokens(&tokenize("
        function add(a, b) {
            return a + b;
        }

        let c = add(1, 2)
    ").unwrap(), &vec![
        Token::Reserved(Reserved::Function),
        Token::Identifier("add".into()),
        Token::LeftParenthesis,
        Token::Identifier("a".into()),
        Token::Comma,
        Token::Identifier("b".into()),
        Token::RightParenthesis,
        Token::LeftBrace,
        Token::Reserved(Reserved::Return),
        Token::Identifier("a".into()),
        Token::OpsAdd,
        Token::Identifier("b".into()),
        Token::RightBrace,
        Token::Reserved(Reserved::Let),
        Token::Identifier("c".into()),
        Token::Assign,
        Token::Identifier("add".into()),
        Token::LeftParenthesis,
        Token::Number(1.0),
        Token::Comma,
        Token::Number(2.0),
        Token::RightParenthesis,
    ])
}

#[cfg(test)]
fn assert_tokens(l: &Vec<Token>, r: &Vec<Token>) {
    assert_eq!(l.len(), r.len());
    for (i, lt) in l.into_iter().enumerate() {
        assert_eq!(lt, &r[i]);
    }
}
