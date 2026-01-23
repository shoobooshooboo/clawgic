#![cfg(test)]

use clawgic::expression_tree::{ExpressionTree, expression_var::ExpressionVar};

#[test]
fn op_construction(){
    let expected = ExpressionTree::new("~(((~A v B) & C) -> D <-> E)").unwrap();
    let a = ExpressionVar::new("A").unwrap();
    let b = ExpressionVar::new("B").unwrap();
    let c = ExpressionVar::new("C").unwrap();
    let d = ExpressionVar::new("D").unwrap();
    let e = ExpressionVar::new("E").unwrap();
    let expression = (((!&a | &b) & &c) >> &d) ^ &e;

    assert_eq!(expression.infix(None), expected.infix(None));
}

#[test]
fn assignop_construction(){
    let expected = ExpressionTree::new("~(((~A v B) & C) -> D <-> E)").unwrap();
    let a = ExpressionVar::new("A").unwrap();
    let b = ExpressionVar::new("B").unwrap();
    let c = ExpressionVar::new("C").unwrap();
    let d = ExpressionVar::new("D").unwrap();
    let e = ExpressionVar::new("E").unwrap();
    let mut expression = !&a;
    expression |= &b;
    expression &= &c;
    expression >>= &d;
    expression ^= &e;

    assert_eq!(expression.infix(None), expected.infix(None));
}