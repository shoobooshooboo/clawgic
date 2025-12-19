#[cfg(test)]
mod test{
    use std::collections::HashMap;

    use test_case::test_case;
    use clawgic::expression_tree::{ExpressionTree, ExpressionTreeError};

    #[test_case("A" ; "single variable")]
    #[test_case("A&B" ; "one connective")]
    #[test_case("(A&B)vC" ; "two connectives")]
    #[test_case("A->B<->C" ; "two arrows")]
    #[test_case("(~(A&B)vC->~D<->~~E)" ; "many connectives")]
    fn new_ok(expression: &str){
        let t = ExpressionTree::new(expression);
        
        assert!(t.is_ok(), "{:#?}", t);
    }

    #[test_case("(A&B", ExpressionTreeError::InvalidParentheses ; "missing close parentheses")]
    #[test_case("A&B)", ExpressionTreeError::InvalidParentheses ; "missing open parentheses")]
    #[test_case("A&b", ExpressionTreeError::LowercaseVariables ; "lowercase variable")]
    #[test_case("(A&B)&", ExpressionTreeError::TooManyOperators ; "Too many operators")]
    #[test_case("AB", ExpressionTreeError::NotEnoughOperators ; "Not enough operators")]
    #[test_case("A&~", ExpressionTreeError::InvalidExpression ; "tilde nothing")]
    #[test_case("A&<-", ExpressionTreeError::UnknownSymbol ; "bad double arrow")]
    #[test_case("A&-", ExpressionTreeError::UnknownSymbol ; "bad single arrow")]
    #[test_case("A&?", ExpressionTreeError::UnknownSymbol ; "random symbol")]
    #[test_case("A&B&C", ExpressionTreeError::AmbiguousExpression ; "ambiguous conjunctions")]
    fn new_err(expression: &str, err: ExpressionTreeError){
        let t = ExpressionTree::new(expression);
        assert_eq!(t.unwrap_err(), err);
    }

    #[test]
    fn set_variable(){
        let mut t = ExpressionTree::new("A&B->A").unwrap();
        assert!(t.evaluate().is_err());
        t.set_variable("A", true);
        assert!(t.evaluate().is_err());
        t.set_variable("B", true);
        assert!(t.evaluate().is_ok());
    }

    #[test_case("~(A&B)", false, true, true, true ; "negated conjunction")]
    #[test_case("A&B", true, false, false, false ; "conjunction")]
    #[test_case("AvB", true, true, false, true ; "disjunction")]
    #[test_case("A->B", true, false, true, true ; "conditional")]
    #[test_case("A<->B", true, false, true, false ; "biconditional")]
    fn evaluate(expression: &str, ex1: bool, ex2: bool, ex3: bool, ex4: bool){
        let mut t = ExpressionTree::new(expression).unwrap();
        t.set_variable("A", true);
        t.set_variable("B", true);
        assert_eq!(t.evaluate().unwrap(), ex1, "failed true true");

        t.set_variable("B", false);
        assert_eq!(t.evaluate().unwrap(), ex2, "failed true false");

        t.set_variable("A", false);
        assert_eq!(t.evaluate().unwrap(), ex3, "failed false false");

        t.set_variable("B", true);
        assert_eq!(t.evaluate().unwrap(), ex4, "failed false true");
    }

    #[test_case("~(A&B)", false, true, true, true ; "negated conjunction")]
    #[test_case("A&B", true, false, false, false ; "conjunction")]
    #[test_case("AvB", true, true, false, true ; "disjunction")]
    #[test_case("A->B", true, false, true, true ; "conditional")]
    #[test_case("A<->B", true, false, true, false ; "biconditional")]
    fn evaluate_with_vars(expression: &str, ex1: bool, ex2: bool, ex3: bool, ex4: bool){
        let t = ExpressionTree::new(expression).unwrap();
        let mut v = HashMap::new();
        v.insert("A".to_string(), true);
        v.insert("B".to_string(), true);
        println!("{:#?}", v);
        assert_eq!(t.evaluate_with_vars(&v).unwrap(), ex1, "failed true true");

        v.insert("B".to_string(), false);
        assert_eq!(t.evaluate_with_vars(&v).unwrap(), ex2, "failed true false");

        v.insert("A".to_string(), false);
        assert_eq!(t.evaluate_with_vars(&v).unwrap(), ex3, "failed false false");

        v.insert("B".to_string(), true);
        assert_eq!(t.evaluate_with_vars(&v).unwrap(), ex4, "failed false true");
    }

    #[test_case("A&B", "&AB" ; "One connective")]
    #[test_case("(A&B)vC", "v&ABC" ; "Two connectives")]
    #[test_case("(A&B)vC->D", "->v&ABCD" ; "Three connectives")]
    #[test_case("(A&B)vC->(D<->E)", "->v&ABC<->DE" ; "four connectives")]
    #[test_case("(A1&~B)v~C3->~(D<->E)", "->v&A1~B~C3~<->DE" ; "four connectives with funny symbols")]
    fn prefix(expression: &str, expected: &str){
        let t = ExpressionTree::new(expression).unwrap();
        assert_eq!(t.prefix(), expected);
    }

    #[test_case("A&B", "(A&B)" ; "no expected changes")]
    #[test_case("~(A&B)", "(~Av~B)" ; "just demorgans")]
    #[test_case("A->B", "(~AvB)" ; "just implication")]
    #[test_case("~(A->B)", "(A&~B)" ; "just ncon")]
    #[test_case("A<->B", "((A&B)v(~A&~B))" ; "just mat_eq")]
    #[test_case("~(A&~B)v~C->~(D<->E)", "(((A&~B)&C)v((~D&E)v(D&~E)))" ; "lots of stuff")]
    fn monotenize(expression: &str, expected: &str){
        let mut t = ExpressionTree::new(expression).unwrap();
        t.monotenize();

        assert_eq!(t.infix(), expected);
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

        assert_eq!(expression.infix(), expected.infix());
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

        assert_eq!(expression.infix(), expected.infix());
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

        assert_eq!(expression.infix(), expected.infix());
    }

    #[test_case("A&B", "B&A", true ; "swapped operands")]
    #[test_case("A&B", "A&B", true ; "same expression")]
    #[test_case("A&~A", "B&~B", true ; "inconsistencies")]
    #[test_case("A&B", "A&C", false ; "completely different")]
    fn log_eq(expr1: &str, expr2: &str, expected: bool){
        let t1 = ExpressionTree::new(expr1).unwrap();
        let t2 = ExpressionTree::new(expr2).unwrap();

        assert_eq!(t1.log_eq(&t2), expected);
    }

    #[test_case("A&B", "B&A", false ; "swapped operands")]
    #[test_case("A&B", "A&B", true ; "same expression")]
    #[test_case("A&~A", "B&~B", false ; "inconsistencies")]
    #[test_case("A&B", "A&C", false ; "completely different")]
    fn lit_eq(expr1: &str, expr2: &str, expected: bool){
        let t1 = ExpressionTree::new(expr1).unwrap();
        let t2 = ExpressionTree::new(expr2).unwrap();

        assert_eq!(t1.lit_eq(&t2), expected);
    }

    #[test_case("A&B", "B&A", true ; "swapped operands")]
    #[test_case("A&B", "A&B", true ; "same expression")]
    #[test_case("A&~A", "B&~B", false ; "inconsistencies")]
    #[test_case("A&B", "A&C", false ; "completely different")]
    fn syn_eq(expr1: &str, expr2: &str, expected: bool){
        let t1 = ExpressionTree::new(expr1).unwrap();
        let t2 = ExpressionTree::new(expr2).unwrap();

        assert_eq!(t1.syn_eq(&t2), expected);
    }

    #[test_case("A&B", Ok(true) ; "over-populating")]
    #[test_case("A&B->C", Ok(true) ; "correct number of vars")]
    #[test_case("A&B->C&D", Err(ExpressionTreeError::UninitializedVariable("D".to_string())) ; "under-populating")]
    fn set_variables(expr: &str, expected: Result<bool, ExpressionTreeError>){
        let mut t = ExpressionTree::new(expr).unwrap();
        let mut vars = HashMap::new();
        vars.insert("A".to_string(), true);
        vars.insert("B".to_string(), true);
        vars.insert("C".to_string(), true);
        t.set_variables(&vars);

        assert_eq!(t.evaluate(), expected);
    }

    #[test]
    fn chaining_functions(){
        let mut t1 = ExpressionTree::new("~(A<->B)").unwrap();
        let t2 = ExpressionTree::new("~(~(A->B)v~(B->A))").unwrap();

        t1.deny().mat_eq().unwrap().demorgans();

        assert!(t1.lit_eq(&t2));
    }
}