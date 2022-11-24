pub fn parse(text: &str) -> Vec<Token> {

    let mut tokens = Vec::new();
    let mut char_iterator = text.chars().peekable(); // needs to be mutable
    let mut line_count = 1;

    while let Some(c) = char_iterator.next() {
        match c {
            // newline
            '\n' => line_count += 1,
            // whitespace
            c if c.is_whitespace() => continue,
            // comments or slash
            '/' => {
                if char_iterator.peek() == Some(&'/') {
                    char_iterator.next();
                    // line comment
                    loop {
                        match char_iterator.peek() {
                            Some(ch) => {
                                if ch == &'\n' {
                                    break
                                } else {
                                    char_iterator.next();
                                }
                            }
                            None => break
                        }
                    }

                } else if char_iterator.peek() == Some(&'*') {
                    char_iterator.next();
                    // block comment
                    loop {
                        match char_iterator.peek() {
                            Some(ch) => {
                                if ch == &'\n' {
                                    // increment line counts at newlines
                                    line_count += 1;
                                    char_iterator.next();
                                } else if ch == &'*' {
                                    // might either be end, or just a character
                                    char_iterator.next();
                                    match char_iterator.peek() {
                                        Some(ch) => {
                                            if ch == &'/' {
                                                // end of block comment
                                                char_iterator.next();
                                                break;
                                            } else if ch == &'\n' {
                                                // increment line counts at newlines
                                                line_count += 1;
                                                char_iterator.next();
                                            } else {
                                                // continue consuming chars
                                                char_iterator.next();
                                            }
                                        }
                                        None => break
                                    }
                                } else {
                                    // continue consuming chars
                                    char_iterator.next();
                                }
                            }
                            None => break
                        }
                    }

                } else {
                    // '/' symbol
                    tokens.push(Token::new(TokenType::Slash, line_count));
                }
            }
            // symbols (except '/', which is handled in comment segment)
            '{' => tokens.push(Token::new(TokenType::LeftBrace, line_count)),
            '}' => tokens.push(Token::new(TokenType::RightBrace, line_count)),
            '(' => tokens.push(Token::new(TokenType::LeftParen, line_count)),
            ')' => tokens.push(Token::new(TokenType::RightParen, line_count)),
            '[' => tokens.push(Token::new(TokenType::LeftBracket, line_count)),
            ']' => tokens.push(Token::new(TokenType::RightBracket, line_count)),
            '.' => tokens.push(Token::new(TokenType::Dot, line_count)),
            ',' => tokens.push(Token::new(TokenType::Comma, line_count)),
            ';' => tokens.push(Token::new(TokenType::Semicolon, line_count)),
            '+' => tokens.push(Token::new(TokenType::Plus, line_count)),
            '-' => tokens.push(Token::new(TokenType::Minus, line_count)),
            '*' => tokens.push(Token::new(TokenType::Star, line_count)),
            '&' => tokens.push(Token::new(TokenType::Ampersand, line_count)),
            '|' => tokens.push(Token::new(TokenType::Bar, line_count)),
            '<' => tokens.push(Token::new(TokenType::LessThan, line_count)),
            '>' => tokens.push(Token::new(TokenType::GreaterThan, line_count)),
            '=' => tokens.push(Token::new(TokenType::Equals, line_count)),
            '~' => tokens.push(Token::new(TokenType::Tilde, line_count)),
            // indentifier
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut identifier = String::new();

                identifier.push(c);

                loop {
                    match char_iterator.peek() {
                        Some(ch) => {
                            if ch.is_ascii_alphanumeric() || *ch == '_' {
                                identifier.push(char_iterator.next().unwrap());
                            } else {
                                break
                            }
                        },
                        None => break,
                    }
                }
                match identifier.as_str() {
                    "class" => tokens.push(Token::new(TokenType::Class, line_count)),
                    "constructor" => tokens.push(Token::new(TokenType::Constructor, line_count)),
                    "function" => tokens.push(Token::new(TokenType::Function, line_count)),
                    "method" => tokens.push(Token::new(TokenType::Method, line_count)),
                    "field" => tokens.push(Token::new(TokenType::Field, line_count)),
                    "static" => tokens.push(Token::new(TokenType::Static, line_count)),
                    "var" => tokens.push(Token::new(TokenType::Var, line_count)),
                    "int" => tokens.push(Token::new(TokenType::Int, line_count)),
                    "char" => tokens.push(Token::new(TokenType::Char, line_count)),
                    "boolean" => tokens.push(Token::new(TokenType::Boolean, line_count)),
                    "void" => tokens.push(Token::new(TokenType::Void, line_count)),
                    "true" => tokens.push(Token::new(TokenType::True, line_count)),
                    "false" => tokens.push(Token::new(TokenType::False, line_count)),
                    "null" => tokens.push(Token::new(TokenType::Null, line_count)),
                    "this" => tokens.push(Token::new(TokenType::This, line_count)),
                    "let" => tokens.push(Token::new(TokenType::Let, line_count)),
                    "do" => tokens.push(Token::new(TokenType::Do, line_count)),
                    "if" => tokens.push(Token::new(TokenType::If, line_count)),
                    "else" => tokens.push(Token::new(TokenType::Else, line_count)),
                    "while" => tokens.push(Token::new(TokenType::While, line_count)),
                    "return" => tokens.push(Token::new(TokenType::Return, line_count)),
                    ident => tokens.push(Token::new(TokenType::Identifier(Name(ident.to_string())), line_count)),
                }
            }
            // integer literal
            '0'..='9' => {
                let mut int_literal = String::new();

                int_literal.push(c);

                loop {
                    match char_iterator.peek() {
                        Some(ch) => {
                            if ch.is_ascii_digit() {
                                int_literal.push(char_iterator.next().unwrap());
                            } else {
                                break
                            }
                        },
                        None => break,
                    }
                }
                tokens.push(Token::new(TokenType::IntLiteral(int_literal), line_count))
            }
            // string literal
            '"' => {
                let mut string_literal = String::new();

                loop {
                    match char_iterator.peek() {
                        Some(ch) => {
                            if *ch != '"' {
                                string_literal.push(char_iterator.next().unwrap());
                            } else {
                                // consume end quote
                                char_iterator.next();
                                break
                            }
                        },
                        None => break,
                    }
                }
                tokens.push(Token::new(TokenType::StringLiteral(string_literal), line_count))
            }
            // catchall for invalid characters
            _ => println!("invalid character while parsing")
        }
    }
    // dbg!(&tokens);
    tokens
}

#[derive(Debug)]
#[derive(Eq, PartialEq, Hash)]
pub enum TokenType {
    // Symbols
    LeftBrace, RightBrace, LeftParen, RightParen, LeftBracket, RightBracket,
    Dot, Comma, Semicolon, Plus, Minus, Star, Slash, Ampersand, Bar, 
    LessThan, GreaterThan, Equals, Tilde,

    // Keywords
    Class, Constructor, Function, Method, Field, Static, Var, Int, Char, Boolean,
    Void, True, False, Null, This, Let, Do, If, Else, While, Return,

    // Literals
    Identifier(Name), StringLiteral(String), IntLiteral(String),
}

#[derive(Debug)]
#[derive(Eq, PartialEq, Hash)]
pub struct Token {
    token_type: TokenType,
    line: u32,
}

#[derive(Debug)]
#[derive(Eq, PartialEq, Hash)]
pub struct Name(pub String);

impl Token {
    pub fn new(token_type: TokenType, line: u32) -> Token {
        Token {token_type, line}
    }
}
