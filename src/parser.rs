use crate::{lexer::Token, parser::Node::binary};
use std::{iter::Peekable, vec::IntoIter};

// --------Parser-------------
#[derive(Debug)]
pub enum Operator {
    ADD,
    SUB,
    MUL,
    DIV,
    NEG,
    SQR,
    ROOT,
    EXP,
}

#[derive(Debug)]
pub enum Node {
    number(f64),
    binary {
        op: Operator,
        left: Box<Node>,
        right: Box<Node>,
    },
    unary {
        op: Operator,
        node: Box<Node>,
    },
}

pub struct Parser {
    iterador_tokens: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn new(vector: Vec<Token>) -> Self {
        Parser {
            iterador_tokens: vector.into_iter().peekable(),
        }
    }
    // -(15+15) * 2 + 15 + 15
    pub fn parse_expression(&mut self) -> Node {
        let mut left = self.parse_term(); // Va a consumir toda la parte de la izquierda

        while let Some(token) = self.iterador_tokens.peek() {
            let op = match token {
                Token::PLUS => Operator::ADD,
                Token::MINUS => Operator::SUB,
                Token::RPAR => {
                    println!("Llegamos al parentesis derecho y cortamos la expresion interna");
                    break;
                }
                _ => break,
            };
            self.iterador_tokens.next(); //consume el operador
            let right = self.parse_term();

            left = Node::binary {
                op: op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        left
    }

    // -(15+15) * 2 + 15 + 15

    fn parse_term(&mut self) -> Node {
        let mut left = self.parse_factor();

        while let Some(token) = self.iterador_tokens.peek() {
            let op: Operator = match token {
                Token::STAR => Operator::MUL,
                Token::SLASH => Operator::DIV,
                Token::CARET => Operator::EXP,
                Token::DSLASH => Operator::ROOT,
                _ => break,
            };
            self.iterador_tokens.next();
            let right = self.parse_factor();

            left = Node::binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        left
    }
    // 5 +3 -(2+3)
    // 15 + 15                  {Binary{Operator::ADD,Binary{Operator::ADD,Number(15), Number(15), Number(15)}
    // -(15+15) * 2 + 15 + 15
    fn parse_factor(&mut self) -> Node {
        match self.iterador_tokens.next() {
            Some(Token::NUMBER(x)) => Node::number(x),
            Some(Token::LPAR) => {
                let expr = self.parse_expression();

                if self.iterador_tokens.next() == Some(Token::RPAR) {
                    return expr;
                }
                panic!("Sintaxis incorrecta")
            }

            Some(Token::MINUS) => {
                let expr = self.parse_factor();
                println!(" Negamos {:?}", expr);
                Node::unary {
                    op: Operator::NEG,
                    node: Box::new(expr),
                }
            }
            _ => panic!("algo anduvo raro"),
        }
    }
}
