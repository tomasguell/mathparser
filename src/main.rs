use core::panic;
use std::{env, iter::Peekable, ops::Not, vec::{self, IntoIter}};


// --------Lexer-------------
struct  Lexer <'a> {
    src:&'a str,
    
}




#[derive(Debug, PartialEq)]
enum Token {
    NUMBER(f64),
    ID(String),
    PLUS, // "+"
    MINUS,
    STAR,
    SLASH,
    LPAR, //PARENTECIS IZQUIERDO
    RPAR,
    OTHER(char),

}

impl <'a> Lexer<'a>{
    fn new ( src: &'a str)-> Self {
       
        Lexer{
            src,
        }

    }

    fn tokenize(mut self) -> Vec<Token>{
           // str -> [char] 
        let mut tokens: Vec<Token> = vec![];   
        let mut iterador = self.src.chars().peekable();
        while let Some(&c) = iterador.peek(){
//[383+]
            match c {
                c if c.is_whitespace() => {iterador.next();}, //whitespace
                '+' =>{ tokens.push(Token::PLUS); iterador.next();},
                '-' =>{ tokens.push(Token::MINUS); iterador.next();},
                '*' =>{ tokens.push(Token::STAR); iterador.next();},
                '/' =>{ tokens.push(Token::SLASH); iterador.next();},
                '(' =>{ tokens.push(Token::LPAR); iterador.next();},
                ')' =>{ tokens.push(Token::RPAR); iterador.next();},
                

                // 383413.1414+3
                '0'..='9' => {
                    let mut num_string:String = String::new();
                    while let Some (&digit) = iterador.peek(){
                        if digit.is_ascii_digit() || digit == '.'{

                             num_string.push(digit);
                             //println!("{num_string}");
                             iterador.next();
                        
                    }
                    else{
                        break;

                    }
                    

                    }
                    let num:f64 = num_string.parse().unwrap();
                    tokens.push(Token::NUMBER(num));
                    
                },


                _ => { tokens.push(Token::OTHER(c)); iterador.next();}
                

            }




            //println!("{}", c);
            
        }
        tokens


    }


}





// --------Parser-------------
#[derive(Debug)]
enum Operator{
    ADD,
    SUB,
    MUL,
    DIV,
    NEG,
    SQR,
    POW
}


#[derive(Debug)]
enum Node {
    number(f64),
    binary{
        op:Operator,
        left:Box<Node>,
        right:Box<Node>,
    }
}



struct Parser {
    iterador_tokens : Peekable<IntoIter<Token>>,

}

impl Parser{

    fn new(vector: Vec<Token>) -> Self {
        Parser{
            iterador_tokens: vector.into_iter().peekable() 
        }
    }
// (15+15) * 2 + 15 + 15
    fn parse_expression(&mut self) -> Node{
            let mut left = self.parse_term(); // Va a consumir toda la parte de la izquierda
            
            while let Some(token) = self.iterador_tokens.peek() {
                let op = match token {
                     Token::PLUS => Operator::ADD,
                     Token::MINUS => Operator::SUB,
                     Token::RPAR => {println!("Llegamos al parentesis derecho y cortamos la expresion interna"); break},
                     _ => break,

                };
                self.iterador_tokens.next(); //consume el operador
                let right = self.parse_term();

                left = Node::binary { op:op, left:Box::new(left), right:Box::new(right) };

            }

            left


    }

// (15+15) * 2 + 15 + 15

        fn parse_term(&mut self)-> Node {
        let mut left = self.parse_factor();

         while let Some(token) = self.iterador_tokens.peek() {
                let op: Operator = match token {
                     Token::STAR => Operator::MUL,
                     Token::SLASH => Operator::DIV,
                     _ => break,

                };
                self.iterador_tokens.next();
                let right = self.parse_factor();

                left = Node::binary { op, left:Box::new(left),right: Box::new(right) };

        }
    
        left
    }


    // 15 + 15                  {Binary{Operator::ADD,Binary{Operator::ADD,Number(15), Number(15), Number(15)}
    // (15+15) * 2 + 15 + 15
     fn parse_factor(&mut self)-> Node {

        match self.iterador_tokens.next() {
            Some(Token::NUMBER(x)) => Node::number(x),
            Some(Token::LPAR) => {
                let expr = self.parse_expression();
                
                if self.iterador_tokens.next() == Some(Token::RPAR) {
                    return expr
                    
                    
                }
                panic!("Sintaxis incorrecta")
                
            } 
            _ => panic!("algo anduvo raro")
            
        }
        

    }

}




fn main() {
    let arg: String = env::args().nth(1).unwrap(); 
    let l = Lexer::new(&arg);
    let tokens = l.tokenize();
    println!("Tokens:\n {:?}\n ",tokens);
    let ast = Parser::new(tokens).parse_expression();
    println!("AST: \n{:?}",ast);
}
