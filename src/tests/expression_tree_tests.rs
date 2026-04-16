#![cfg(test)]
use std::collections::HashMap;

use test_case::test_case;
use crate::{expression_tree::universe::Universe, prelude::*};

fn sen0(name: &str) -> Sentence{
    Sentence::new(&Predicate::new(name, 0).unwrap(), &vec![]).unwrap()
}

fn senx(name: &str, vars: Vec<&str>) -> Sentence{
    Sentence::new_from_strings(&Predicate::new(name, vars.len()).unwrap(), &vars.iter().map(|v| v.to_string()).collect()).unwrap()
}

#[test_case("A" ; "single predicate0")]
#[test_case("A()" ; "single predicate0 w parentheses")]
#[test_case("A(a)" ; "single predicate1")]
#[test_case("A23(a, b1)" ; "single predicate2")]
#[test_case("A23(a, b1, c23124)" ; "single predicate3")]
#[test_case("A&B" ; "one connective")]
#[test_case("@(x)L(x,c)" ; "simple quantifier")]
#[test_case("@(x, y, z)L(x,y)" ; "multi var quantifier")]
#[test_case("#(x)@(y)L(x,y)" ; "multiple quantifiers")]
#[test_case("@(x)#(y, z)(F(x,y)&F(y,z)->F(x,z))" ; "multi-quan complex expression")]
#[test_case("~~@(x)~#(y, z)(F(x,y)&F(y,z)->F(x,z))" ; "mult-quan complex expression with negs")]
#[test_case("A(a1, b)&B(x300, y585)" ; "one connective w predicates")]
#[test_case("(A&B)vC" ; "two connectives")]
#[test_case("(A(a, b)&B(v5, w6))vC" ; "two connectives w predicates")]
#[test_case("A->B<->C" ; "two arrows")]
#[test_case("(~(A&B)vC->~D<->~~E)" ; "many connectives")]
#[test_case("(~(A(a1, b2)&B1())vC2(x)->~D()<->~~E3)" ; "many connectives w predicates")]
#[test_case("TRUE" ; "r#true")]
#[test_case("FALSE" ; "r#false")]
#[test_case("TRUE&FALSE" ; "true and false")]
fn new_ok(expression: &str){
    let t = ExpressionTree::new(expression);
    
    assert!(t.is_ok(), "{:#?}", t);
}

#[test_case("(A&B", ClawgicError::InvalidParentheses ; "missing close parentheses")]
#[test_case("A&B)", ClawgicError::InvalidParentheses ; "missing open parentheses")]
#[test_case("A&b", ClawgicError::InvalidPredicateName("b".to_string()) ; "lowercase predicate")]
#[test_case("A&BC", ClawgicError::InvalidPredicateName("BC".to_string()) ; "multi-letter predicate")]
#[test_case("A(B)", ClawgicError::InvalidVariableName("B".to_string()) ; "uppercase variables")]
#[test_case("A(bc)", ClawgicError::InvalidVariableName("bc".to_string()) ; "multi-letter variable")]
#[test_case("A(b4c)", ClawgicError::InvalidVariableName("b4c".to_string()) ; "ill-formed variable")]
#[test_case("A&B4C", ClawgicError::NotEnoughOperators ; "ill-formed predicate")]
#[test_case("(A&B)&", ClawgicError::TooManyOperators ; "Too many operators")]
#[test_case("(A)B", ClawgicError::NotEnoughOperators ; "Not enough operators")]
#[test_case("A&~", ClawgicError::InvalidExpression ; "tilde nothing")]
#[test_case("A&<-", ClawgicError::UnknownSymbol("<-".to_string()); "bad double arrow")]
#[test_case("A&-", ClawgicError::UnknownSymbol("-".to_string()); "bad single arrow")]
#[test_case("A&?", ClawgicError::UnknownSymbol("?".to_string()); "random symbol")]
#[test_case("A&B&C", ClawgicError::AmbiguousExpression ; "ambiguous conjunctions")]
fn new_err(expression: &str, err: ClawgicError){
    let t = ExpressionTree::new(expression);
    assert_eq!(t.unwrap_err(), err);
}

#[test]
fn set_variable(){
    let mut t = ExpressionTree::new("A&B->A").unwrap();
    assert!(t.evaluate().is_err());
    t.set_tval(&sen0("A"), true);
    assert!(t.evaluate().is_err());
    t.set_tval(&sen0("B"), true);
    assert!(t.evaluate().is_ok());
}

#[test_case("~(A&B)", false, true, true, true ; "negated conjunction")]
#[test_case("A&B", true, false, false, false ; "conjunction")]
#[test_case("AvB", true, true, false, true ; "disjunction")]
#[test_case("A->B", true, false, true, true ; "conditional")]
#[test_case("A<->B", true, false, true, false ; "biconditional")]
fn evaluate(expression: &str, ex1: bool, ex2: bool, ex3: bool, ex4: bool){
    let mut t = ExpressionTree::new(expression).unwrap();
    t.set_tval(&sen0("A"), true);
    t.set_tval(&sen0("B"), true);
    assert_eq!(t.evaluate().unwrap(), ex1, "failed true true");

    t.set_tval(&sen0("B"), false);
    assert_eq!(t.evaluate().unwrap(), ex2, "failed true false");

    t.set_tval(&sen0("A"), false);
    assert_eq!(t.evaluate().unwrap(), ex3, "failed false false");

    t.set_tval(&sen0("B"), true);
    assert_eq!(t.evaluate().unwrap(), ex4, "failed false true");
}

#[test_case("~(A(a1)&B(x, y))", false, true, true, true ; "negated conjunction")]
#[test_case("A(a1)&B(x, y)", true, false, false, false ; "conjunction")]
#[test_case("A(a1)vB(x, y)", true, true, false, true ; "disjunction")]
#[test_case("A(a1)->B(x, y)", true, false, true, true ; "conditional")]
#[test_case("A(a1)<->B(x, y)", true, false, true, false ; "biconditional")]
fn evaluate_multi_var_pred(expression: &str, ex1: bool, ex2: bool, ex3: bool, ex4: bool){
    let mut t = ExpressionTree::new(expression).unwrap();
    t.set_tval(&senx("A", vec!["a1"]), true);
    t.set_tval(&senx("B", vec!["x", "y"]), true);
    assert_eq!(t.evaluate().unwrap(), ex1, "failed true true");

    t.set_tval(&senx("B", vec!["x", "y"]), false);
    assert_eq!(t.evaluate().unwrap(), ex2, "failed true false");

    t.set_tval(&senx("A", vec!["a1"]), false);
    assert_eq!(t.evaluate().unwrap(), ex3, "failed false false");

    t.set_tval(&senx("B", vec!["x", "y"]), true);
    assert_eq!(t.evaluate().unwrap(), ex4, "failed false true");
}

#[test]
fn evaluate_irrelevant_tvalue(){
    let mut t = ExpressionTree::new("A&B(x)").unwrap();
    t.set_tval(&sen0("A"), true);
    t.set_tval(&senx("B", vec!["x"]), true);
    assert_eq!(t.evaluate().unwrap(), true);
    t.set_tval(&senx("B", vec!["x1"]), false);
    assert_eq!(t.evaluate().unwrap(), true);
}

#[test_case("~(A&B)", false, true, true, true ; "negated conjunction")]
#[test_case("A&B", true, false, false, false ; "conjunction")]
#[test_case("AvB", true, true, false, true ; "disjunction")]
#[test_case("A->B", true, false, true, true ; "conditional")]
#[test_case("A<->B", true, false, true, false ; "biconditional")]
fn evaluate_with_uni(expression: &str, ex1: bool, ex2: bool, ex3: bool, ex4: bool){
    let t = ExpressionTree::new(expression).unwrap();
    let mut v = Universe::new();
    v.insert_sentence(sen0("A"), true);
    v.insert_sentence(sen0("B"), true);
    // println!("{:#?}", v);
    assert_eq!(t.evaluate_with_uni(&v).unwrap(), ex1, "failed true true");

    v.insert_sentence(sen0("B"), false);
    assert_eq!(t.evaluate_with_uni(&v).unwrap(), ex2, "failed true false");

    v.insert_sentence(sen0("A"), false);
    assert_eq!(t.evaluate_with_uni(&v).unwrap(), ex3, "failed false false");

    v.insert_sentence(sen0("B"), true);
    assert_eq!(t.evaluate_with_uni(&v).unwrap(), ex4, "failed false true");
}

#[test_case("A&B", "&AB" ; "One connective")]
#[test_case("(A&B)vC", "∨&ABC" ; "Two connectives")]
#[test_case("(A&B)vC->D", "➞∨&ABCD" ; "Three connectives")]
#[test_case("(A&B)vC->(D<->E)", "➞∨&ABC⟷DE" ; "four connectives")]
#[test_case("(A1&~B)v~C3->~(D<->E)", "➞∨&A1¬B¬C3¬⟷DE" ; "four connectives with funny symbols")]
fn prefix(expression: &str, expected: &str){
    let t = ExpressionTree::new(expression).unwrap();
    assert_eq!(t.prefix(None), expected);
}

#[test_case("A", "A" ; "no connectives")]
#[test_case("A&B", "A&B" ; "One connective")]
#[test_case("~(A&B)vC", "¬(A&B)∨C" ; "Two connectives")]
#[test_case("(A&B)vC->D", "((A&B)∨C)➞D" ; "Three connectives")]
#[test_case("(A&B)vC->(D<->E)", "((A&B)∨C)➞(D⟷E)" ; "four connectives")]
#[test_case("(A1&~B)v~C3->~(D<->E)", "((A1&¬B)∨¬C3)➞¬(D⟷E)" ; "four connectives with funny symbols")]
fn infix(expression: &str, expected: &str){
    let t = ExpressionTree::new(expression).unwrap();
    assert_eq!(t.infix(None), expected);
}

#[test_case("A&B", "A&B" ; "no expected changes")]
#[test_case("~(A&B)", "¬A∨¬B" ; "just demorgans")]
#[test_case("A->B", "¬A∨B" ; "just implication")]
#[test_case("~(A->B)", "A&¬B" ; "just ncon")]
#[test_case("A<->B", "(A&B)∨(¬A&¬B)" ; "just mat_eq")]
#[test_case("~(A&~B)v~C->~(D<->E)", "((A&¬B)&C)∨((¬D&E)∨(D&¬E))" ; "lots of stuff")]
fn monotenize(expression: &str, expected: &str){
    let mut t = ExpressionTree::new(expression).unwrap();
    t.monotenize();

    assert_eq!(t.infix(None), expected);
}

#[test]
fn func_construction(){
    let expected = ExpressionTree::new("~(A&(BvC->D<->E))").unwrap();
    let a = ExpressionTree::new("A").unwrap();
    let b = ExpressionTree::new("B").unwrap();
    let c = ExpressionTree::new("C").unwrap();
    let d = ExpressionTree::new("D").unwrap();
    let e = ExpressionTree::new("E").unwrap();
    let expression = a.and(b.or(c).con(d).bicon(e)).not();

    assert_eq!(expression.infix(None), expected.infix(None));
}

#[test]
fn op_construction(){
    let expected = ExpressionTree::new("~(((~A v B) & C) -> D <-> E)").unwrap();
    let a = ExpressionTree::new("A").unwrap();
    let b = ExpressionTree::new("B").unwrap();
    let c = ExpressionTree::new("C").unwrap();
    let d = ExpressionTree::new("D").unwrap();
    let e = ExpressionTree::new("E").unwrap();
    let expression = (((!a | b) & c) >> d) ^ e;

    assert_eq!(expression.infix(None), expected.infix(None));
}

#[test]
fn assignop_construction(){
    let expected = ExpressionTree::new("~(((~A v B) & C) -> D <-> E)").unwrap();
    let a = ExpressionTree::new("A").unwrap();
    let b = ExpressionTree::new("B").unwrap();
    let c = ExpressionTree::new("C").unwrap();
    let d = ExpressionTree::new("D").unwrap();
    let e = ExpressionTree::new("E").unwrap();
    let mut expression = !a;
    expression |= b;
    expression &= c;
    expression >>= d;
    expression ^= e;

    assert_eq!(expression.infix(None), expected.infix(None));
}

#[test_case("A&B", "B&A", true ; "swapped operands")]
#[test_case("A&B", "~~(A&B)", true ; "double negation")]
#[test_case("A&B", "A&B", true ; "same expression")]
#[test_case("A&~A", "B&~B", true ; "inconsistencies")]
#[test_case("A&B", "A&C", false ; "completely different")]
fn log_eq(expr1: &str, expr2: &str, expected: bool){
    let t1 = ExpressionTree::new(expr1).unwrap();
    let t2 = ExpressionTree::new(expr2).unwrap();

    assert_eq!(t1.log_eq(&t2), expected);
}

#[test_case("A&B", "B&A", false ; "swapped operands")]
#[test_case("A&B", "~~(A&B)", false ; "double negation")]
#[test_case("A&B", "A&B", true ; "same expression")]
#[test_case("A&~A", "B&~B", false ; "inconsistencies")]
#[test_case("A&B", "A&C", false ; "completely different")]
fn lit_eq(expr1: &str, expr2: &str, expected: bool){
    let t1 = ExpressionTree::new(expr1).unwrap();
    let t2 = ExpressionTree::new(expr2).unwrap();

    assert_eq!(t1.lit_eq(&t2), expected);
}

#[test_case("A&B", "B&A", true ; "swapped operands")]
#[test_case("A&B", "~~(A&B)", true ; "double negation")]
#[test_case("A&B", "A&B", true ; "same expression")]
#[test_case("A&~A", "B&~B", false ; "inconsistencies")]
#[test_case("A&B", "A&C", false ; "completely different")]
fn syn_eq(expr1: &str, expr2: &str, expected: bool){
    let t1 = ExpressionTree::new(expr1).unwrap();
    let t2 = ExpressionTree::new(expr2).unwrap();

    assert_eq!(t1.syn_eq(&t2), expected);
}

#[test_case("A&B", Ok(true) ; "over-populating")]
#[test_case("A&B->C", Ok(true) ; "correct number of uni")]
#[test_case("A&B->C&D", Err(ClawgicError::UninitializedSentence("D".to_string())) ; "under-populating")]
fn set_variables(expr: &str, expected: Result<bool, ClawgicError>){
    let mut t = ExpressionTree::new(expr).unwrap();
    let mut uni = HashMap::new();
    uni.insert(sen0("A"), true);
    uni.insert(sen0("B"), true);
    uni.insert(sen0("C"), true);
    t.set_tvals(&uni);

    assert_eq!(t.evaluate(), expected);
}

#[test]
fn chaining_functions(){
    let mut t1 = ExpressionTree::new("~(A<->B)").unwrap();
    let t2 = ExpressionTree::new("~(~(A->B)v~(B->A))").unwrap();

    t1.deny().mat_eq().unwrap().demorgans();

    assert!(t1.lit_eq(&t2));
}

#[test_case("Av~A", true ; "tautology")]
#[test_case("A&~A", false ; "inconsistency")]
#[test_case("A", true ; "contingency")]
fn is_satisfiable(expr: &str, expected: bool){
    assert_eq!(ExpressionTree::new(expr).unwrap().is_satisfiable(), expected);
}

#[test_case("Av~A", true ; "tautology")]
#[test_case("A&~A", false ; "inconsistency")]
#[test_case("A", true ; "contingency")]
fn satisfy_one(expr: &str, expected: bool){
    let mut tree = ExpressionTree::new(expr).unwrap();

    match tree.satisfy_one(){
        Some(v) => {
            tree.set_tvals(&v);
            assert!(tree.evaluate().unwrap() && expected)
        },
        None => assert!(!expected),
    };
}

#[test_case("Av~A", 2 ; "tautology")]
#[test_case("A&~A", 0 ; "inconsistency")]
#[test_case("A", 1 ; "contingency")]
fn satisfy_all(expr: &str, count: usize){
    let mut tree = ExpressionTree::new(expr).unwrap();
    let var_maps = tree.satisfy_all();
    assert_eq!(var_maps.len(), count);
    
    for uni in var_maps{
        tree.set_tvals(&uni);
        if !tree.evaluate().unwrap(){
            assert!(false);
        }
    }
    assert!(true);
}

#[test_case("Av~A", 2 ; "tautology")]
#[test_case("A&~A", 0 ; "inconsistency")]
#[test_case("A", 1 ; "contingency")]
fn satisfy_count(expr: &str, count: u128){
    let tree = ExpressionTree::new(expr).unwrap();

    assert_eq!(tree.satisfy_count()[0], count);
}

#[test_case("Av~A", true ; "tautology")]
#[test_case("A&~A", false ; "inconsistency")]
#[test_case("A", false ; "contingency")]
fn is_tautology(expr: &str, expected: bool){
    let tree = ExpressionTree::new(expr).unwrap();

    assert_eq!(tree.is_tautology(), expected);
}

#[test_case("Av~A", false ; "tautology")]
#[test_case("A&~A", true ; "inconsistency")]
#[test_case("A", false ; "contingency")]
fn is_inconsistency(expr: &str, expected: bool){
    let tree = ExpressionTree::new(expr).unwrap();

    assert_eq!(tree.is_inconsistency(), expected);
}

#[test_case("Av~A", false ; "tautology")]
#[test_case("A&~A", false ; "inconsistency")]
#[test_case("A", true ; "contingency")]
fn is_contingency(expr: &str, expected: bool){
    let tree = ExpressionTree::new(expr).unwrap();

    assert_eq!(tree.is_contingency(), expected);
}

#[test_case("A&B", sen0("A"), "CvD", "(CvD)&B" ; "normal")]
#[test_case("A&B", sen0("C"), "CvD", "A&B" ; "no variable to replace")]
#[test_case("A", sen0("A"), "CvD", "CvD" ; "single variable")]
#[test_case("~A&A", sen0("A"), "CvD", "~(CvD)&(CvD)" ; "denied")]
fn replace_variable(expr1: &str, var: Sentence, subexpr: &str, expected: &str){
    let mut t1 = ExpressionTree::new(expr1).unwrap();
    let st = ExpressionTree::new(subexpr).unwrap();
    let res = ExpressionTree::new(expected).unwrap();

    t1.replace_sentence(&var, &st);
    assert!(t1.lit_eq(&res));
}

#[test]
fn replace_variables(){
    let mut tree = ExpressionTree::new("~A&B->Cv~D").unwrap();
    let mut uni = HashMap::new();
    let a_subtree = ExpressionTree::new("BvD").unwrap();
    uni.insert(sen0("A"), &a_subtree);
    let b_subtree = ExpressionTree::new("E->F").unwrap();
    uni.insert(sen0("B"), &b_subtree);
    let e_subtree = ExpressionTree::new("H").unwrap();
    uni.insert(sen0("E"), &e_subtree);

    let expected = ExpressionTree::new("~(BvD)&(E->F)->Cv~D").unwrap();

    tree.replace_sentences(&uni);

    assert_eq!(tree.infix(None), expected.infix(None));
}

#[test]
fn evaluate_after_deny(){
    let mut tree = ExpressionTree::new("A").unwrap();
    tree.set_tval(&sen0("A"), true);
    assert!(tree.evaluate().unwrap());
    tree.deny();
    assert!(!tree.evaluate().unwrap());
    assert!(tree.not().evaluate().unwrap());
}

#[test_case("¬(A∧B)∨(C➞TRUE⟷E)", "~(A&B)v(C->TRUE<->E)" ; "mathematical")]
#[test_case("¬(A⋅B)+(C➞TRUE⟷E)", "~(A&B)v(C->TRUE<->E)" ; "logic gates")]
#[test_case("~(A*B)+(C->TRUE<->E)", "~(A&B)v(C->TRUE<->E)" ; "logic gates ascii")]
#[test_case("!(A&B)|(C➞TRUE⟷E)", "~(A&B)v(C->TRUE<->E)" ; "coding")]
#[test_case("!(A&B)|(C->TRUE<->E)", "~(A&B)v(C->TRUE<->E)" ; "coding ascii")]
fn new_with_weird_ops(expression: &str, expected: &str){
    let t1 = ExpressionTree::new(expression).unwrap();
    let t2 = ExpressionTree::new(expected).unwrap();
    assert!(t1.lit_eq(&t2));
}

#[test_case("A&B", "A&B", "CvD", "CvD" ; "complete replacement")]
#[test_case("A&(BvC)", "BvC", "CvD", "A&(CvD)" ; "subexpression")]
#[test_case("A&~(BvC)", "BvC", "CvD", "A&~(CvD)" ; "old denied")]
#[test_case("A&~(BvC)", "BvC", "~(CvD)", "A&(CvD)" ; "both denied")]
#[test_case("A&(BvC)", "BvC", "~(CvD)", "A&~(CvD)" ; "new denied")]

fn replace_expression(expression: &str, old: &str, new: &str, expected: &str){
    let mut tree = ExpressionTree::new(expression).unwrap();
    let old = ExpressionTree::new(old).unwrap();
    let new = ExpressionTree::new(new).unwrap();
    let expected = ExpressionTree::new(expected).unwrap();
    tree.replace_expression(&old, &new);
    // println!("{}", tree.prefix(None));
    // println!("{}", expected.prefix(None));

    assert!(tree.lit_eq(&expected));
}

#[allow(non_snake_case)]
#[test]
fn TRUE(){
    assert!(ExpressionTree::TRUE().evaluate().unwrap());
}

#[allow(non_snake_case)]
#[test]
fn FALSE(){
    assert!(!ExpressionTree::FALSE().evaluate().unwrap());
}

#[test_case(true ; "r#true")]
#[test_case(false ; "r#false")]
fn constant(b: bool){
    assert_eq!(ExpressionTree::constant(b).evaluate().unwrap(), b);
}

#[test_case("TRUE", true ; "r#true")]
#[test_case("FALSE", false ; "r#false")]
#[test_case("TRUE&FALSE", false ; "true and false")]
#[test_case("TRUEvFALSE", true ; "true or false")]
#[test_case("~TRUE", false ; "denied true")]
#[test_case("~FALSE", true ; "denied false")]
fn new_with_constants(expression: &str, expected: bool){
    let tree = ExpressionTree::new(expression).unwrap();
    assert_eq!(tree.evaluate().unwrap(), expected);
}

//this (as well as all the tests for the original functions) should cover all of the "_with" functions 
#[test_case("Av~A->B", "Bv~B", true ; "tautology")]
#[test_case("A&B", "B&~A", false ; "inconsistency")]
#[test_case("A&B", "A", true ; "contingency")]
#[test_case("A", "B&!B", false ; "completely irrelevent")]
fn is_satisfiable_with(expr: &str, aux: &str, expected: bool){
    let tree = ExpressionTree::new(expr).unwrap();
    let aux = ExpressionTree::new(aux).unwrap();

    assert_eq!(tree.is_satisfiable_with(&aux), expected);
}

#[test]
fn notation_printing(){
    let tree = ExpressionTree::new("(A1&~B)v~C->(D<->E)").unwrap();
    let notation = OperatorNotation::bits_ascii();
    assert_eq!(tree.infix(Some(&notation)), "((A1*~B)+~C)->(D<->E)", "1");
    let notation = OperatorNotation::new(HashMap::from([
        (Operator::AND, ("&&".to_string(), vec![])),
        (Operator::NOT, ("?".to_string(), vec![])),
        (Operator::OR, ("||".to_string(), vec![])),
        (Operator::CON, (".-.".to_string(), vec![])),
        (Operator::BICON, (":".to_string(), vec![])),
    ])).unwrap();
    assert_eq!(tree.infix(Some(&notation)), "((A1&&?B)||?C).-.(D:E)", "2");
}

#[test_case("(A1<-B)>-C#(D@E)", "(A1&~B)v~C->(D<->E)", ["-", "<", ">", "#", "@"] ; "unique symbols")]
//#[test_case("(A1 and notB)or notC if(D bicon E)", "(A1&~B)v~C->(D<->E)", ["not", "and", "or", "if", "bicon"] ; "lowercase words")]
fn new_with_notation(expr: &str, expected: &str, operators: [&str ; 5]){
    let notation = OperatorNotation::new(HashMap::from([
        (Operator::NOT, (operators[0].to_string(), vec![])),
        (Operator::AND, (operators[1].to_string(), vec![])),
        (Operator::OR, (operators[2].to_string(), vec![])),
        (Operator::CON, (operators[3].to_string(), vec![])),
        (Operator::BICON, (operators[4].to_string(), vec![])),
    ])).unwrap();
    let t1 = ExpressionTree::new_with_notation(expr, &notation).unwrap();
    let t2 = ExpressionTree::new(expected).unwrap();

    assert!(t1.lit_eq(&t2));
}

#[test_case("Av~A", ExpressionTree::or, true; "tautology")]
#[test_case("A&~A", ExpressionTree::and, false; "inconsistency")]
#[test_case("A", ExpressionTree::and, true; "contingency")]
#[ignore]
fn large_tree_sat<F>(center: &str, func: F, expected: bool)
    where F: Fn(ExpressionTree, ExpressionTree) -> ExpressionTree{
    let mut tree = ExpressionTree::new(center).unwrap();
    for i in 0..128{
        tree = func(tree, ExpressionTree::new(&("A".to_string() + &i.to_string())).unwrap());
    }

    assert_eq!(tree.is_satisfiable(), expected);
}

//i know this is bad convention for unit tests,
//but all of these functions are extremely simple,
//so i don't really care.
#[test]
fn negation_functions(){
    let mut tree = ExpressionTree::new("A").unwrap();
    assert!(tree.deny().lit_eq(&ExpressionTree::new("~A").unwrap()));
    assert!(tree.negate().lit_eq(&ExpressionTree::new("~~A").unwrap()));
    assert!(tree.deny().lit_eq(&ExpressionTree::new("~A").unwrap()));
    assert!(tree.double_deny().lit_eq(&ExpressionTree::new("~~~A").unwrap()));
    assert!(tree.double_negate().lit_eq(&ExpressionTree::new("~~~~~A").unwrap()));
    assert!(tree.double_deny().lit_eq(&ExpressionTree::new("~~~A").unwrap()));
    assert!(tree.reduce_negation().lit_eq(&ExpressionTree::new("~A").unwrap()));
}

#[test]
fn transposition(){
    let mut tree = ExpressionTree::new("A->B").unwrap();
    assert!(tree.transposition().unwrap().lit_eq(&ExpressionTree::new("~B->~A").unwrap()));
    assert!(tree.transposition().unwrap().lit_eq(&ExpressionTree::new("A->B").unwrap()));
}

#[test]
fn demorgans_neg(){
    let mut tree = ExpressionTree::new("~(~Av~B)").unwrap();
    assert!(tree.demorgans_neg().unwrap().lit_eq(&ExpressionTree::new("~~(~~A&~~B)").unwrap()))
}

#[test]
fn implication_neg(){
    let mut tree = ExpressionTree::new("~(~Av~B)").unwrap();
    assert!(tree.implication_neg().unwrap().lit_eq(&ExpressionTree::new("~(~~A->~B)").unwrap()))
}

#[test]
fn ncon_neg(){
    let mut tree = ExpressionTree::new("~(~A&~B)").unwrap();
    assert!(tree.ncon_neg().unwrap().lit_eq(&ExpressionTree::new("~~(~A->~~B)").unwrap()))
}

#[test]
fn transposition_neg(){
    let mut tree = ExpressionTree::new("~(~A->~B)").unwrap();
    assert!(tree.transposition_neg().unwrap().lit_eq(&ExpressionTree::new("~(~~B->~~A)").unwrap()))
}

#[test_case("A&B", Some(Operator::AND) ; "conjunction")]
#[test_case("~(A&B)", Some(Operator::NOT) ; "conjunction denied")]
#[test_case("AvB", Some(Operator::OR) ; "disjunction")]
#[test_case("~(AvB)", Some(Operator::NOT) ; "disjunction denied")]
#[test_case("A->B", Some(Operator::CON) ; "conditional")]
#[test_case("~(A->B)", Some(Operator::NOT) ; "conditional denied")]
#[test_case("(A<->B)", Some(Operator::BICON) ; "biconditional")]
#[test_case("~(A<->B)", Some(Operator::NOT) ; "biconditional denied")]
#[test_case("A", None ; "no connective")]
#[test_case("~A", Some(Operator::NOT) ; "tilde")]
fn main_connective(expr: &str, op: Option<Operator>){
    let tree = ExpressionTree::new(expr).unwrap();
    assert_eq!(tree.main_connective(), op);
}

#[test_case("A&B", Some(Operator::AND) ; "conjunction")]
#[test_case("~(A&B)", None ; "conjunction denied")]
#[test_case("AvB", Some(Operator::OR) ; "disjunction")]
#[test_case("~(AvB)", None ; "disjunction denied")]
#[test_case("A->B", Some(Operator::CON) ; "conditional")]
#[test_case("~(A->B)", None ; "conditional denied")]
#[test_case("(A<->B)", Some(Operator::BICON) ; "biconditional")]
#[test_case("~(A<->B)", None ; "biconditional denied")]
#[test_case("A", None ; "no connective")]
#[test_case("~A", None ; "tilde")]
fn main_conn_non_tilde(expr: &str, op: Option<Operator>){
    let tree = ExpressionTree::new(expr).unwrap();
    assert_eq!(tree.main_conn_non_tilde(), op);
}