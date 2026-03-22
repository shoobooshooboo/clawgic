#![cfg(test)]
use test_case::test_case;

use crate::prelude::*;

#[test]
fn new_vars_ex(){
    let a = ExpressionVars::new("A", 1..4, true).unwrap();

    assert_eq!(a[1].name(), "A1");
    assert_eq!(a[2].name(), "A2");
    assert_eq!(a[3].name(), "A3");
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