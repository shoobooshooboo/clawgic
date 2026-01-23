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

    assert!(expression.lit_eq(&expected));
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
    
    assert!(expression.lit_eq(&expected));
}

#[test]
fn new_vars_ex(){
    let expected = ExpressionTree::new("A1 & A2 -> A3").unwrap();
    let a = ExpressionVar::new_vars("A", 1..4).unwrap();

    let expr = (&a[0] & &a[1]) >> &a[2];

    assert!(expr.lit_eq(&expected));
}

#[test]
fn new_vars_in(){
    let expected = ExpressionTree::new("A1 & A2 -> A3").unwrap();
    let a = ExpressionVar::new_vars("A", 1..=3).unwrap();

    let expr = (&a[0] & &a[1]) >> &a[2];
    
    assert!(expr.lit_eq(&expected));
}