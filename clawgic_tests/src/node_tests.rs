#![cfg(test)]

use std::collections::HashMap;

use clawgic::expression_tree::node::{Node, negation::Negation, operator::Operator};
use test_case::test_case;

#[test_case(true ; "true node")]
#[test_case(false ; "false node")]
fn constant_node(value: bool){
    let n = Node::Constant(Negation::default(), value);
    assert_eq!(n.evaluate(&HashMap::new()).unwrap(), value);
}

#[test_case(Negation::new(0), true, true ; "true, not denied")]
#[test_case(Negation::new(0), false, false ; "false, not denied")]
#[test_case(Negation::new(1), true, false ; "true, denied")]
#[test_case(Negation::new(1), false, true ; "false, denied")]
fn variable_node(denied: Negation, value: bool, expected: bool){
    let n = Node::Variable { denied, name: "A".into()};
    let vars = HashMap::from([("A".to_string(), Some(value))]);
    assert_eq!(n.evaluate(&vars).unwrap(), expected);
}

#[test]
fn variable_node_empty(){
    let n = Node::Variable { denied: Negation::new(0), name: "A".into()};
    let vars = HashMap::new();
    assert!(n.evaluate(&vars).is_err());
}

#[test_case(Operator::AND, true, false, false, false ; "AND OPERATOR")]
#[test_case(Operator::OR, true, true, true, false ; "OR OPERATOR")]
#[test_case(Operator::CON, true, false, true, true ; "CON OPERATOR")]
#[test_case(Operator::BICON, true, false, false, true ; "BICON OPERATOR")]
fn operator_nodes(operator: Operator, ex1: bool, ex2: bool, ex3: bool, ex4: bool){
    let vars = HashMap::new();
    let op = Node::Operator {
        denied: Negation::new(0),
        op: operator,
        left: Box::new(Node::Constant(Negation::new(0), true)),
        right: Box::new(Node::Constant(Negation::new(0), true)) 
    };
    assert_eq!(op.evaluate(&vars).unwrap(), ex1, "true true failed");

    let op = Node::Operator {
        denied: Negation::new(0),
        op: operator,
        left: Box::new(Node::Constant(Negation::new(0), true)),
        right: Box::new(Node::Constant(Negation::new(0), false)) 
    };
    assert_eq!(op.evaluate(&vars).unwrap(), ex2, "true false failed");

    let op = Node::Operator {
        denied: Negation::new(0),
        op: operator,
        left: Box::new(Node::Constant(Negation::new(0), false)),
        right: Box::new(Node::Constant(Negation::new(0), true)) 
    };
    assert_eq!(op.evaluate(&vars).unwrap(), ex3, "false true failed");

    let op = Node::Operator {
        denied: Negation::new(0),
        op: operator,
        left: Box::new(Node::Constant(Negation::new(0), false)),
        right: Box::new(Node::Constant(Negation::new(0), false)) 
    };
    assert_eq!(op.evaluate(&vars).unwrap(), ex4, "false false failed");
}

#[test_case(Node::Variable{denied: Negation::new(0), name: "A".to_string()}, "A".to_string() ; "Variable")]
#[test_case(Node::Variable{denied: Negation::new(1), name: "A".to_string()}, "¬A".to_string() ; "Denied Variable")]
#[test_case(Node::Constant(Negation::new(0), true), "TRUE".to_string() ; "True Constant")]
#[test_case(Node::Constant(Negation::new(0), false), "FALSE".to_string() ; "False Constant")]
#[test_case(Node::Operator{denied: Negation::new(0), op: Operator::AND, left: Box::new(Node::Constant(Negation::new(0), true)), right: Box::new(Node::Constant(Negation::new(0), true))}, "&".to_string() ; "And Operator")]
#[test_case(Node::Operator{denied: Negation::new(1), op: Operator::AND, left: Box::new(Node::Constant(Negation::new(0), true)), right: Box::new(Node::Constant(Negation::new(0), true))}, "¬&".to_string() ; "Denied Operator")]
#[test_case(Node::Operator{denied: Negation::new(0), op: Operator::OR, left: Box::new(Node::Constant(Negation::new(0), true)), right: Box::new(Node::Constant(Negation::new(0), true))}, "∨".to_string() ; "Or Operator")]
#[test_case(Node::Operator{denied: Negation::new(0), op: Operator::CON, left: Box::new(Node::Constant(Negation::new(0), true)), right: Box::new(Node::Constant(Negation::new(0), true))}, "➞".to_string() ; "Con Operator")]
#[test_case(Node::Operator{denied: Negation::new(0), op: Operator::BICON, left: Box::new(Node::Constant(Negation::new(0), true)), right: Box::new(Node::Constant(Negation::new(0), true))}, "⟷".to_string() ; "Bicon Operator")]
fn to_string(node: Node, expected: String){
    assert_eq!(node.to_string(), expected);
}

#[test_case(Node::Variable{denied: Negation::new(0), name: "A".to_string()}, "A".to_string() ; "Variable")]
#[test_case(Node::Variable{denied: Negation::new(1), name: "A".to_string()}, "~A".to_string() ; "Denied Variable")]
#[test_case(Node::Constant(Negation::new(0), true), "TRUE".to_string() ; "True Constant")]
#[test_case(Node::Constant(Negation::new(0), false), "FALSE".to_string() ; "False Constant")]
#[test_case(Node::Operator{denied: Negation::new(0), op: Operator::AND, left: Box::new(Node::Constant(Negation::new(0), true)), right: Box::new(Node::Constant(Negation::new(0), true))}, "&".to_string() ; "And Operator")]
#[test_case(Node::Operator{denied: Negation::new(1), op: Operator::AND, left: Box::new(Node::Constant(Negation::new(0), true)), right: Box::new(Node::Constant(Negation::new(0), true))}, "~&".to_string() ; "Denied Operator")]
#[test_case(Node::Operator{denied: Negation::new(0), op: Operator::OR, left: Box::new(Node::Constant(Negation::new(0), true)), right: Box::new(Node::Constant(Negation::new(0), true))}, "v".to_string() ; "Or Operator")]
#[test_case(Node::Operator{denied: Negation::new(0), op: Operator::CON, left: Box::new(Node::Constant(Negation::new(0), true)), right: Box::new(Node::Constant(Negation::new(0), true))}, "->".to_string() ; "Con Operator")]
#[test_case(Node::Operator{denied: Negation::new(0), op: Operator::BICON, left: Box::new(Node::Constant(Negation::new(0), true)), right: Box::new(Node::Constant(Negation::new(0), true))}, "<->".to_string() ; "Bicon Operator")]
fn to_ascii(node: Node, expected: String){
    assert_eq!(node.to_ascii(), expected);
}

#[test_case(
    Node::Operator{denied: Negation::new(1), op: Operator::AND, left: Box::new(Node::Constant(Negation::new(1), true)), right: Box::new(Node::Variable{denied: Negation::new(0), name: "A".to_string()})},
    Node::Operator{denied: Negation::new(0), op: Operator::OR, left: Box::new(Node::Constant(Negation::new(0), true)), right: Box::new(Node::Variable{denied: Negation::new(1), name: "A".to_string()})}
    ; "AND")]
#[test_case(
    Node::Operator{denied: Negation::new(0), op: Operator::OR, left: Box::new(Node::Constant(Negation::new(1), true)), right: Box::new(Node::Variable{denied: Negation::new(0), name: "A".to_string()})},
    Node::Operator{denied: Negation::new(1), op: Operator::AND, left: Box::new(Node::Constant(Negation::new(0), true)), right: Box::new(Node::Variable{denied: Negation::new(1), name: "A".to_string()})}
    ; "OR")]
fn demorgans(mut node: Node, expected: Node){
    node.demorgans();
    assert_eq!(node, expected);
}

#[test_case(
    Node::Operator { denied: Negation::new(0), op: Operator::BICON, left: Box::new(Node::Variable{denied: Negation::new(0), name: "A".to_string()}), right:  Box::new(Node::Variable{denied: Negation::new(0), name: "B".to_string()})},
    Node::Operator { denied: Negation::new(0), op: Operator::AND, 
        left: Box::new(Node::Operator{denied: Negation::new(0), op: Operator::CON, left: Box::new(Node::Variable{denied: Negation::new(0), name: "A".to_string()}), right: Box::new(Node::Variable{denied: Negation::new(0), name: "B".to_string()})}), 
        right: Box::new(Node::Operator{denied: Negation::new(0), op: Operator::CON, left: Box::new(Node::Variable{denied: Negation::new(0), name: "B".to_string()}), right: Box::new(Node::Variable{denied: Negation::new(0), name: "A".to_string()})})} 
    ; "BICON")]
#[test_case(
    Node::Operator { denied: Negation::new(0), op: Operator::AND, 
        left: Box::new(Node::Operator{denied: Negation::new(0), op: Operator::CON, left: Box::new(Node::Variable{denied: Negation::new(0), name: "A".to_string()}), right: Box::new(Node::Variable{denied: Negation::new(0), name: "B".to_string()})}), 
        right: Box::new(Node::Operator{denied: Negation::new(0), op: Operator::CON, left: Box::new(Node::Variable{denied: Negation::new(0), name: "B".to_string()}), right: Box::new(Node::Variable{denied: Negation::new(0), name: "A".to_string()})})}, 
    Node::Operator { denied: Negation::new(0), op: Operator::BICON, left: Box::new(Node::Variable{denied: Negation::new(0), name: "A".to_string()}), right:  Box::new(Node::Variable{denied: Negation::new(0), name: "B".to_string()})}
    ; "AND")]
fn mat_eq(mut node: Node, expected: Node){
    node.mat_eq();
    assert_eq!(node, expected);
}

#[test_case(
    Node::Operator{denied: Negation::new(0), op: Operator::CON, left: Box::new(Node::Constant(Negation::new(0), true)), right: Box::new(Node::Variable{denied: Negation::new(0), name: "A".to_string()})},
    Node::Operator{denied: Negation::new(0), op: Operator::OR, left: Box::new(Node::Constant(Negation::new(1), true)), right: Box::new(Node::Variable{denied: Negation::new(0), name: "A".to_string()})}
    ; "CON")]
#[test_case(
    Node::Operator{denied: Negation::new(0), op: Operator::OR, left: Box::new(Node::Constant(Negation::new(0), true)), right: Box::new(Node::Variable{denied: Negation::new(0), name: "A".to_string()})},
    Node::Operator{denied: Negation::new(0), op: Operator::CON, left: Box::new(Node::Constant(Negation::new(1), true)), right: Box::new(Node::Variable{denied: Negation::new(0), name: "A".to_string()})}
    ; "OR")]
fn implication(mut node: Node, expected: Node){
    node.implication();
    assert_eq!(node, expected);
}

#[test_case(
    Node::Operator{denied: Negation::new(1), op: Operator::AND, left: Box::new(Node::Constant(Negation::new(0), true)), right: Box::new(Node::Variable{denied: Negation::new(0), name: "A".to_string()})},
    Node::Operator{denied: Negation::new(0), op: Operator::CON, left: Box::new(Node::Constant(Negation::new(0), true)), right: Box::new(Node::Variable{denied: Negation::new(1), name: "A".to_string()})}
    ; "AND")]
#[test_case(
    Node::Operator{denied: Negation::new(0), op: Operator::CON, left: Box::new(Node::Constant(Negation::new(0), true)), right: Box::new(Node::Variable{denied: Negation::new(0), name: "A".to_string()})},
    Node::Operator{denied: Negation::new(1), op: Operator::AND, left: Box::new(Node::Constant(Negation::new(0), true)), right: Box::new(Node::Variable{denied: Negation::new(1), name: "A".to_string()})}
    ; "CON")]
fn ncon(mut node: Node, expected: Node){
    node.ncon();
    assert_eq!(node, expected);
}

#[test_case(
    Node::Operator { denied: Negation::new(0), op: Operator::BICON, left: Box::new(Node::Variable{denied: Negation::new(0), name: "A".to_string()}), right:  Box::new(Node::Variable{denied: Negation::new(0), name: "B".to_string()})},
    Node::Operator { denied: Negation::new(0), op: Operator::OR, 
        left: Box::new(Node::Operator{denied: Negation::new(0), op: Operator::AND, left: Box::new(Node::Variable{denied: Negation::new(0), name: "A".to_string()}), right: Box::new(Node::Variable{denied: Negation::new(0), name: "B".to_string()})}), 
        right: Box::new(Node::Operator{denied: Negation::new(0), op: Operator::AND, left: Box::new(Node::Variable{denied: Negation::new(1), name: "A".to_string()}), right: Box::new(Node::Variable{denied: Negation::new(1), name: "B".to_string()})})} 
    ; "BICON")]
fn mat_eq_mono(mut node: Node, expected: Node){
    node.mat_eq_mono();
    assert_eq!(node, expected);
}