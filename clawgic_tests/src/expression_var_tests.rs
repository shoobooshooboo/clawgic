#![cfg(test)]
use test_case::test_case;

use clawgic::prelude::*;

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
    let a = ExpressionVars::new("A", 1..4, true).unwrap();

    let expr = (&a[1] & &a[2]) >> &a[3];

    assert!(expr.lit_eq(&expected));
}

#[test]
fn new_vars_in(){
    let expected = ExpressionTree::new("A1 & A2 -> A3").unwrap();
    let a = ExpressionVars::new("A", 1..=3, true).unwrap();

    let expr = (&a[1] & &a[2]) >> &a[3];
    
    assert!(expr.lit_eq(&expected));
}

#[test]
fn relative_index_normal(){
    let a = ExpressionVars::new("A", 1..=3, true).unwrap();
    assert_eq!(a[1].name(), "A1");
    assert_eq!(a[2].name(), "A2");
    assert_eq!(a[3].name(), "A3");
}

#[should_panic]
#[test_case( 0 ; "too low")]
#[test_case( 4 ; "too high")]
fn relative_index_panic(i: usize){
    let a = ExpressionVars::new("A", 1..=3, true).unwrap();
    let _ = &a[i];
}

#[test]
fn absolute_index_normal(){
    let a = ExpressionVars::new("A", 1..=3, false).unwrap();
    assert_eq!(a[0].name(), "A1");
    assert_eq!(a[1].name(), "A2");
    assert_eq!(a[2].name(), "A3");
}

#[test]
#[should_panic]
fn absolute_index_panic(){
    let a = ExpressionVars::new("A", 1..=3, false).unwrap();
    let _ = &a[3];
}

#[test]
fn vars_iter(){
    let a = ExpressionVars::new("A", 1..=3, false).unwrap();
    let mut iter = a.into_iter();
    assert_eq!(iter.next().unwrap().name(), "A1");
    assert_eq!(iter.next().unwrap().name(), "A2");
    assert_eq!(iter.next().unwrap().name(), "A3");
    assert!(iter.next().is_none());
}