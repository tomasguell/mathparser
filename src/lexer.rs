// --------Lexer-------------
pub struct Lexer<'a> {
    src: &'a str,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    NUMBER(f64),
    ID(String),
    PLUS, // "+"
    MINUS,
    STAR,
    SLASH,
    DSLASH, // double slash
    CARET,
    LPAR, //PARENTECIS IZQUIERDO
    RPAR,

    OTHER(char),
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Lexer { src }
    }

    pub fn tokenize(mut self) -> Vec<Token> {
        // str -> [char]
        let mut tokens: Vec<Token> = vec![];
        let mut iterador = self.src.chars().peekable();
        while let Some(&c) = iterador.peek() {
            //[383+]
            match c {
                c if c.is_whitespace() => {
                    iterador.next();
                } //whitespace
                '+' => {
                    tokens.push(Token::PLUS);
                    iterador.next();
                }
                '-' => {
                    tokens.push(Token::MINUS);
                    iterador.next();
                }
                '*' => {
                    tokens.push(Token::STAR);
                    iterador.next();
                }
                '/' => {
                    iterador.next();
                    if let Some('/') = iterador.peek() {
                        tokens.push(Token::DSLASH);
                        iterador.next();
                    } else {
                        tokens.push(Token::SLASH);
                    }
                }
                '^' => {
                    tokens.push(Token::CARET);
                    iterador.next();
                }

                '(' => {
                    tokens.push(Token::LPAR);
                    iterador.next();
                }
                ')' => {
                    tokens.push(Token::RPAR);
                    iterador.next();
                }

                // 383413.1414+3
                '0'..='9' => {
                    let mut num_string: String = String::new();
                    while let Some(&digit) = iterador.peek() {
                        if digit.is_ascii_digit() || digit == '.' {
                            num_string.push(digit);
                            //println!("{num_string}");
                            iterador.next();
                        } else {
                            break;
                        }
                    }
                    let num: f64 = num_string.parse().unwrap();
                    tokens.push(Token::NUMBER(num));
                }

                _ => {
                    tokens.push(Token::OTHER(c));
                    iterador.next();
                }
            }

            //println!("{}", c);
        }
        tokens
    }
}
