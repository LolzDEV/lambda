pub struct Lexer;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Number(f64),
    Bool(bool), 
    Lambda,
    Let,
    Dot,
    Comma,
    SemiColon,
    LeftSquared,
    RightSquared,
    LeftParen,
    RightParen,
    Define,
    DefineType,
    EqualEqual,
    Plus,
    Minus,
    Star,
    Slash,
    Power,
    Arrow,
}

impl Lexer {
    pub fn lex(s: String) -> Vec<Token> {
        let mut result = Vec::new();
        let chars: Vec<char> = s.chars().collect();
        let mut skip = 0;
        for mut i in 0..chars.len() {
            if skip > 0 {
                skip -= 1;
                continue;
            }

            let mut c = *chars.get(i).unwrap();
            if c == '\n' || c == ' ' {
                continue;
            }

            match c {
                'λ' => {
                    result.push(Token::Lambda);
                    continue;
                }
                '.' => {
                    result.push(Token::Dot);
                    continue;
                }
                ',' => {
                    result.push(Token::Comma);
                    continue;
                }
                ';' => {
                    result.push(Token::SemiColon);
                    continue;
                }
                '(' => {
                    result.push(Token::LeftParen);
                    continue;
                }
                ')' => {
                    result.push(Token::RightParen);
                    continue;
                }
                '[' => {
                    result.push(Token::LeftSquared);
                    continue;
                }
                ']' => {
                    result.push(Token::RightSquared);
                    continue;
                }
                '+' => {
                    result.push(Token::Plus);
                    continue;
                }
                '-' => {
                    if *chars.get(i+1).unwrap() == '>' {
                        result.push(Token::Arrow);
                        continue;
                    }
                    result.push(Token::Minus);
                    continue;
                }
                '*' => {
                    result.push(Token::Star);
                    continue;
                }
                '/' => {
                    result.push(Token::Slash);
                    continue;
                }
                '^' => {
                    result.push(Token::Power);
                    continue;
                }
                ':' => {
                    match chars.get(i+1).unwrap() {
                        ':' => result.push(Token::DefineType),
                        '=' => result.push(Token::Define),
                        _ => (),
                    }
                    continue;
                }
                '=' => {
                    match chars.get(i+1).unwrap() {
                        '=' => result.push(Token::EqualEqual),
                        _ => (),
                    }
                    continue;
                }
                _ => (),
            }

            if c.is_ascii_digit() {
                let mut number = String::new();
                while c.is_ascii_digit() {
                    number.push(c);
                    i += 1;
                    if let Some(ch) = chars.get(i) {
                        c = *ch; 
                    } else {
                        break;
                    }
                }

                skip = number.len() - 1;

                result.push(Token::Number(number.parse::<f64>().unwrap()));
                continue;
            }

            if c.is_alphabetic() {
                let mut identifier = String::new();
                while c.is_alphabetic() {
                    identifier.push(c);
                    i += 1;
                    if let Some(ch) = chars.get(i) {
                        c = *ch; 
                    } else {
                        break;
                    }
                }

                skip = identifier.len() - 1;

                result.push(Token::Identifier(identifier));
                continue;
            }
        }

        result
    }
}
