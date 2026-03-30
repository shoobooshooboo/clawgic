#![cfg(test)]

use std::collections::HashMap;

use crate::expression_tree::universe::Universe;
use crate::prelude::*;
use crate::expression_tree::node::{Node, negation::Negation};
use test_case::test_case;

fn sen0(name: &str) -> Sentence{
    Sentence::new(&Predicate::new(name, 0).unwrap(), &vec![]).unwrap()
}

#[test_case(true ; "true node")]
#[test_case(false ; "false node")]
fn constant_node(value: bool){
    let n = Node::Constant(Negation::default(), value);
    assert_eq!(n.evaluate(&Universe::new(), &mut HashMap::new()).unwrap(), value);
}

#[test_case(Negation::new(0), true, true ; "true, not denied")]
#[test_case(Negation::new(0), false, false ; "false, not denied")]
#[test_case(Negation::new(1), true, false ; "true, denied")]
#[test_case(Negation::new(1), false, true ; "false, denied")]
fn variable_node(neg: Negation, value: bool, expected: bool){
    let n = Node::Sentence { neg, sen: sen0("A") };
    let mut uni = Universe::new();
    uni.insert_sentence(sen0("A"), value);
    assert_eq!(n.evaluate(&uni, &mut HashMap::new()).unwrap(), expected);
}

#[test]
fn variable_node_empty(){
    let n = Node::Sentence { neg: Negation::new(0), sen: sen0("A")};
    let uni = Universe::new();
    assert!(n.evaluate(&uni, &mut HashMap::new()).is_err());
}

#[test_case(Operator::AND, true, false, false, false ; "AND OPERATOR")]
#[test_case(Operator::OR, true, true, true, false ; "OR OPERATOR")]
#[test_case(Operator::CON, true, false, true, true ; "CON OPERATOR")]
#[test_case(Operator::BICON, true, false, false, true ; "BICON OPERATOR")]
fn operator_nodes(operator: Operator, ex1: bool, ex2: bool, ex3: bool, ex4: bool){
    let uni = Universe::new();
    let op = Node::Operator {
        neg: Negation::new(0),
        op: operator,
        left: Box::new(Node::Constant(Negation::new(0), true)),
        right: Box::new(Node::Constant(Negation::new(0), true)) 
    };
    assert_eq!(op.evaluate(&uni, &mut HashMap::new()).unwrap(), ex1, "true true failed");

    let op = Node::Operator {
        neg: Negation::new(0),
        op: operator,
        left: Box::new(Node::Constant(Negation::new(0), true)),
        right: Box::new(Node::Constant(Negation::new(0), false)) 
    };
    assert_eq!(op.evaluate(&uni, &mut HashMap::new()).unwrap(), ex2, "true false failed");

    let op = Node::Operator {
        neg: Negation::new(0),
        op: operator,
        left: Box::new(Node::Constant(Negation::new(0), false)),
        right: Box::new(Node::Constant(Negation::new(0), true)) 
    };
    assert_eq!(op.evaluate(&uni, &mut HashMap::new()).unwrap(), ex3, "false true failed");

    let op = Node::Operator {
        neg: Negation::new(0),
        op: operator,
        left: Box::new(Node::Constant(Negation::new(0), false)),
        right: Box::new(Node::Constant(Negation::new(0), false)) 
    };
    assert_eq!(op.evaluate(&uni, &mut HashMap::new()).unwrap(), ex4, "false false failed");
}

#[test_case(Node::Sentence{neg: Negation::new(0), sen: sen0("A")}, "A".to_string() ; "Variable")]
#[test_case(Node::Sentence{neg: Negation::new(1), sen: sen0("A")}, "¬A".to_string() ; "Denied Variable")]
#[test_case(Node::Constant(Negation::new(0), true), "TRUE".to_string() ; "True Constant")]
#[test_case(Node::Constant(Negation::new(0), false), "FALSE".to_string() ; "False Constant")]
#[test_case(Node::Operator{neg: Negation::new(0), op: Operator::AND, left: Box::new(Node::Constant(Negation::new(0), true)), right: Box::new(Node::Constant(Negation::new(0), true))}, "&".to_string() ; "And Operator")]
#[test_case(Node::Operator{neg: Negation::new(1), op: Operator::AND, left: Box::new(Node::Constant(Negation::new(0), true)), right: Box::new(Node::Constant(Negation::new(0), true))}, "¬&".to_string() ; "Denied Operator")]
#[test_case(Node::Operator{neg: Negation::new(0), op: Operator::OR, left: Box::new(Node::Constant(Negation::new(0), true)), right: Box::new(Node::Constant(Negation::new(0), true))}, "∨".to_string() ; "Or Operator")]
#[test_case(Node::Operator{neg: Negation::new(0), op: Operator::CON, left: Box::new(Node::Constant(Negation::new(0), true)), right: Box::new(Node::Constant(Negation::new(0), true))}, "➞".to_string() ; "Con Operator")]
#[test_case(Node::Operator{neg: Negation::new(0), op: Operator::BICON, left: Box::new(Node::Constant(Negation::new(0), true)), right: Box::new(Node::Constant(Negation::new(0), true))}, "⟷".to_string() ; "Bicon Operator")]
fn to_string(node: Node, expected: String){
    assert_eq!(node.to_string(), expected);
}

#[test_case(Node::Sentence{neg: Negation::new(0), sen: sen0("A")}, "A".to_string() ; "Variable")]
#[test_case(Node::Sentence{neg: Negation::new(1), sen: sen0("A")}, "~A".to_string() ; "Denied Variable")]
#[test_case(Node::Constant(Negation::new(0), true), "TRUE".to_string() ; "True Constant")]
#[test_case(Node::Constant(Negation::new(0), false), "FALSE".to_string() ; "False Constant")]
#[test_case(Node::Operator{neg: Negation::new(0), op: Operator::AND, left: Box::new(Node::Constant(Negation::new(0), true)), right: Box::new(Node::Constant(Negation::new(0), true))}, "&".to_string() ; "And Operator")]
#[test_case(Node::Operator{neg: Negation::new(1), op: Operator::AND, left: Box::new(Node::Constant(Negation::new(0), true)), right: Box::new(Node::Constant(Negation::new(0), true))}, "~&".to_string() ; "Denied Operator")]
#[test_case(Node::Operator{neg: Negation::new(0), op: Operator::OR, left: Box::new(Node::Constant(Negation::new(0), true)), right: Box::new(Node::Constant(Negation::new(0), true))}, "v".to_string() ; "Or Operator")]
#[test_case(Node::Operator{neg: Negation::new(0), op: Operator::CON, left: Box::new(Node::Constant(Negation::new(0), true)), right: Box::new(Node::Constant(Negation::new(0), true))}, "->".to_string() ; "Con Operator")]
#[test_case(Node::Operator{neg: Negation::new(0), op: Operator::BICON, left: Box::new(Node::Constant(Negation::new(0), true)), right: Box::new(Node::Constant(Negation::new(0), true))}, "<->".to_string() ; "Bicon Operator")]
fn to_ascii(node: Node, expected: String){
    assert_eq!(node.to_ascii(), expected);
}

#[test_case(
    Node::Operator{neg: Negation::new(1), op: Operator::AND, left: Box::new(Node::Constant(Negation::new(1), true)), right: Box::new(Node::Sentence{neg: Negation::new(0), sen: sen0("A")})},
    Node::Operator{neg: Negation::new(0), op: Operator::OR, left: Box::new(Node::Constant(Negation::new(0), true)), right: Box::new(Node::Sentence{neg: Negation::new(1), sen: sen0("A")})}
    ; "AND")]
#[test_case(
    Node::Operator{neg: Negation::new(0), op: Operator::OR, left: Box::new(Node::Constant(Negation::new(1), true)), right: Box::new(Node::Sentence{neg: Negation::new(0), sen: sen0("A")})},
    Node::Operator{neg: Negation::new(1), op: Operator::AND, left: Box::new(Node::Constant(Negation::new(0), true)), right: Box::new(Node::Sentence{neg: Negation::new(1), sen: sen0("A")})}
    ; "OR")]
fn demorgans(mut node: Node, expected: Node){
    node.demorgans();
    assert_eq!(node, expected);
}

#[test_case(
    Node::Operator { neg: Negation::new(0), op: Operator::BICON, left: Box::new(Node::Sentence{neg: Negation::new(0), sen: sen0("A")}), right:  Box::new(Node::Sentence{neg: Negation::new(0), sen: sen0("B")})},
    Node::Operator { neg: Negation::new(0), op: Operator::AND, 
        left: Box::new(Node::Operator{neg: Negation::new(0), op: Operator::CON, left: Box::new(Node::Sentence{neg: Negation::new(0), sen: sen0("A")}), right: Box::new(Node::Sentence{neg: Negation::new(0), sen: sen0("B")})}), 
        right: Box::new(Node::Operator{neg: Negation::new(0), op: Operator::CON, left: Box::new(Node::Sentence{neg: Negation::new(0), sen: sen0("B")}), right: Box::new(Node::Sentence{neg: Negation::new(0), sen: sen0("A")})})} 
    ; "BICON")]
#[test_case(
    Node::Operator { neg: Negation::new(0), op: Operator::AND, 
        left: Box::new(Node::Operator{neg: Negation::new(0), op: Operator::CON, left: Box::new(Node::Sentence{neg: Negation::new(0), sen: sen0("A")}), right: Box::new(Node::Sentence{neg: Negation::new(0), sen: sen0("B")})}), 
        right: Box::new(Node::Operator{neg: Negation::new(0), op: Operator::CON, left: Box::new(Node::Sentence{neg: Negation::new(0), sen: sen0("B")}), right: Box::new(Node::Sentence{neg: Negation::new(0), sen: sen0("A")})})}, 
    Node::Operator { neg: Negation::new(0), op: Operator::BICON, left: Box::new(Node::Sentence{neg: Negation::new(0), sen: sen0("A")}), right:  Box::new(Node::Sentence{neg: Negation::new(0), sen: sen0("B")})}
    ; "AND")]
fn mat_eq(mut node: Node, expected: Node){
    node.mat_eq();
    assert_eq!(node, expected);
}

#[test_case(
    Node::Operator{neg: Negation::new(0), op: Operator::CON, left: Box::new(Node::Constant(Negation::new(0), true)), right: Box::new(Node::Sentence{neg: Negation::new(0), sen: sen0("A")})},
    Node::Operator{neg: Negation::new(0), op: Operator::OR, left: Box::new(Node::Constant(Negation::new(1), true)), right: Box::new(Node::Sentence{neg: Negation::new(0), sen: sen0("A")})}
    ; "CON")]
#[test_case(
    Node::Operator{neg: Negation::new(0), op: Operator::OR, left: Box::new(Node::Constant(Negation::new(0), true)), right: Box::new(Node::Sentence{neg: Negation::new(0), sen: sen0("A")})},
    Node::Operator{neg: Negation::new(0), op: Operator::CON, left: Box::new(Node::Constant(Negation::new(1), true)), right: Box::new(Node::Sentence{neg: Negation::new(0), sen: sen0("A")})}
    ; "OR")]
fn implication(mut node: Node, expected: Node){
    node.implication();
    assert_eq!(node, expected);
}

#[test_case(
    Node::Operator{neg: Negation::new(1), op: Operator::AND, left: Box::new(Node::Constant(Negation::new(0), true)), right: Box::new(Node::Sentence{neg: Negation::new(0), sen: sen0("A")})},
    Node::Operator{neg: Negation::new(0), op: Operator::CON, left: Box::new(Node::Constant(Negation::new(0), true)), right: Box::new(Node::Sentence{neg: Negation::new(1), sen: sen0("A")})}
    ; "AND")]
#[test_case(
    Node::Operator{neg: Negation::new(0), op: Operator::CON, left: Box::new(Node::Constant(Negation::new(0), true)), right: Box::new(Node::Sentence{neg: Negation::new(0), sen: sen0("A")})},
    Node::Operator{neg: Negation::new(1), op: Operator::AND, left: Box::new(Node::Constant(Negation::new(0), true)), right: Box::new(Node::Sentence{neg: Negation::new(1), sen: sen0("A")})}
    ; "CON")]
fn ncon(mut node: Node, expected: Node){
    node.ncon();
    assert_eq!(node, expected);
}

#[test_case(
    Node::Operator { neg: Negation::new(0), op: Operator::BICON, left: Box::new(Node::Sentence{neg: Negation::new(0), sen: sen0("A")}), right:  Box::new(Node::Sentence{neg: Negation::new(0), sen: sen0("B")})},
    Node::Operator { neg: Negation::new(0), op: Operator::OR, 
        left: Box::new(Node::Operator{neg: Negation::new(0), op: Operator::AND, left: Box::new(Node::Sentence{neg: Negation::new(0), sen: sen0("A")}), right: Box::new(Node::Sentence{neg: Negation::new(0), sen: sen0("B")})}), 
        right: Box::new(Node::Operator{neg: Negation::new(0), op: Operator::AND, left: Box::new(Node::Sentence{neg: Negation::new(1), sen: sen0("A")}), right: Box::new(Node::Sentence{neg: Negation::new(1), sen: sen0("B")})})} 
    ; "BICON")]
fn mat_eq_mono(mut node: Node, expected: Node){
    node.mat_eq_mono();
    assert_eq!(node, expected);
}

#[test_case(true ; "true node")]
#[test_case(false ; "false node")]
fn retaining_negations(val: bool){
    let mut node = Node::Constant(Negation::default(), val);
    let uni = Universe::new();
    assert_eq!(node.double_deny().evaluate(&uni, &mut HashMap::new()).unwrap(), val);
    assert_eq!(node.double_negate().evaluate(&uni, &mut HashMap::new()).unwrap(), val);
    assert_eq!(node.double_deny().evaluate(&uni, &mut HashMap::new()).unwrap(), val);
    assert_eq!(node.reduce_negation().evaluate(&uni, &mut HashMap::new()).unwrap(), val);
}