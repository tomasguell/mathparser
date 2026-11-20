use std::ops::Neg;

use crate::parser::Node::{self, number};
use crate::parser::Operator;
///binary { op: ADD, left: binary
///{ op: MUL, left: binary
///{{ op: ADD, left: number(18.0), right: number(10.0) }
///{ right: number(5.0) }
///{,right: number(1.0) }

pub fn evaluate(nodo: &Node) -> f64 {
    match nodo {
        Node::number(x) => *x,
        Node::binary {
            op,
            left: l,
            right: r,
        } => {
            let left = evaluate(l);
            let right = evaluate(r);
            match op {
                Operator::ADD => left + right,
                Operator::MUL => left * right,
                Operator::SUB => left - right,
                Operator::DIV => left / right,
                Operator::EXP => left.powf(right),
                Operator::ROOT => left.powf(1.0 / right),
                _ => panic!("No esta programada la logica de este Operador"),
            }
        }
        Node::unary { op, node } => {
            let value = evaluate(node);
            match op {
                Operator::NEG => value.neg(),
                Operator::ADD => value,
                _ => panic!("No tendria que estar aca"),
            }
        }
    }
}
