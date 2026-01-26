# clawgic
A rust engine for Sentential (Propositional) Logic. Will later be expanded to deal with predicate logic as well.

I highly recommend that you include `use clawgic::prelude::*;` because it gives you all the types you're likely to ever need while working with this library. And that's only like 6 things, so it shouldn't bloat anything too much.
## Things you can do right now:

The main data structure you'll be using is the `ExpressionTree`. `ExpressionTree`s represent a boolean expression with constants, variables, and operators. The operators currently supported are the following:
* conjunction (&, ^, ∧, *, ⋅)
* disjunction (v, ∨, +, |),
* conditional (->, ➞),
* biconditional (<->, ⟷)
* negation (~, ¬, !)

As of v0.7.1, there are now also the `ExpressionVar` and `ExpressionVars` types that you can use to construct `ExpressionTree`s in a nicer way. I've gone into them in detail further into this document.

You can also make and use your own set of operators with the `OperatorNotation` struct.
```rs
//There's several hard-coded notations like ascii(), mathematical(), bits(), and boolean()
//which represent many common standard notation.
let mut notation = OperatorNotation::default();
assert_eq!(notation.get_notation(Operator::AND), "&");
notation.set_notation(Operator::AND, "#".to_string());
assert_eq!(notation.get_notation(Operator::AND), "#");
```

# Constructing an `ExpressionTree`
There are a few ways that you can construct an `ExpressionTree`. The first and simplest is with `ExpressionTree::new()`.
```rs
//without notation.
let mut tree = ExpressionTree::new("~A&B1->Cv~(D42<->E)").unwrap();
//with a notation
let mut notation = OperatorNotation::ascii();
notation.set_notation(Operator::AND, "'literally just and'".to_string());
let tree = ExpressionTree::new_with_notation("A 'literally just and' B", &notation).unwrap();
```
(It's worth noting that all variables must be a capital letter followed by 0 or more digits)

you can also construct them step by step with some other functions.
```rs
let a = ExpressionTree::new("A").unwrap();
let b = ExpressionTree::new("B").unwrap();
let c = ExpressionTree::new("C").unwrap();
let mut tree = a.not().or(c).con(b); //equivalent to ~AvC->B
```

or with operators
```rs
let a = ExpressionTree::new("A").unwrap();
let b = ExpressionTree::new("B").unwrap();
let c = ExpressionTree::new("C").unwrap();
let mut tree = (!a | c ) >> b; //equivalent to ~AvC->B
```

And new in v0.7.1, you can now use the `ExpressionVar` and `ExpressionVars` types.
```rs
let a = ExpressionVar::new("A").unwrap();
let b = ExpressionVars::new("B", 1..=2, true).unwrap();
let mut tree = (!&a | &b[1]) >> &b[2]; //equivalent to ~AvB1->B2
```

# `ExpressionVar`/`s`
The notation for the `ExpressionVar`/`s` types are a little clunky right now, since they're internally very simple. They must either be used by referencing (this works for operators only right now) or with the `.expr()` method like so.
```rs
&a | a.expr();
```
The benefit to `ExpressionVar` is to abstract the cloning of atomic trees so you no longer have to write something like
```rs
let a = ExpressionTree::new("A").unwrap();
let tree = (!a.clone() | a.clone()) >> a;
```
instead allowing for the cleaner
```rs
let a = ExpressionVar::new("A").unwrap();
let tree = (!&a | &a) >> a; // or (!a.expr() | a.expr()) >> a.expr(), but that kinda defeats the point.
```

`ExpressionVars` (plural) allow you to create an enumerated range of variables with the same prefix all stored in one place. The in `ExpressionVars::new()`, the first parameter is the prefix (has to be a valid variable name (i.e. one capital letter followed by any amount of numbers)). The second paramter is the range to enumerate them by. The third paramter dictates whether indexing is relative or absolute.
```rs
//prefix "A", range (5,6,7), relative indexing 
let a = ExpressionVars::new("A", 5..8, true).unwrap();
//a[0] //panics!
assert_eq!(a[5].name(), "A5");
assert_eq!(a[6].name(), "A6");
assert_eq!(a[7].name(), "A7");
//prefix "B10", range (1,2,3), absolute indexing
let b10 = ExpressionVars::new("B10", 4..=6, false)
assert_eq!(b10[0].name(), "B104");
assert_eq!(b10[1].name(), "B105");
assert_eq!(b10[2].name(), "B106");
//b[4] //panics!
```
relative indexing is more readable, but absolute indexing may be more intuitive from a programming perspective.

If you are passing an `ExpressionVars` around and you're unsure if it's relatively or absolutely indexed, fear not! There are two ways of dealing with that.

The first way is by using the `.start()` and `.end()` functions which will return the first and last valid indices respectively
```rs
fn conj_vars(vars: &ExpressionVars) -> ExpressionTree{
    let mut tree = vars[vars.start()].expr();
    for i in (vars.start() + 1)..=vars.end(){
        tree &= &vars[i];
    }
    tree
}
```
But, if you don't feel like doing all of that, you can just use iterator magic!
```rs
fn conj_vars(vars: &ExpressionVars) -> ExpressionTree{
    vars.iter().skip(1).fold(vars[vars.start()], |tree, v| tree & v)
}
```

# modification
once you have an expression constructed, you can modify it in a couple ways.

There are some primitive rules you can call that don't change logical content:
```rs
let mut tree = ExpressionTree::new("A<->B").unwrap();    //A<->B
tree.mat_eq();                                  //(A->B)&(B->A)
//you can also chain these operations
tree.demorgans().implication();                 //~((A->B)->~(B->A))
```
(there's also `ExpressionTree::deny(&mut self)` which DOES change the logical content of your expression. You should not often find yourself in need of that function)

you can also replace variables with entire expressions.
```rs
let mut tree = ExpressionTree::new("~A&B->C").unwrap();
let subtree = ExpressionTree::new("DvE").unwrap(); 
tree.replace_variable("A", &subtree); // produces ~(DvE)&B->C
```

As of v0.5.4, you can now also replace entire expressions in the tree!
```rs
let mut tree = ExpressionTree::new("A&~(BvC)").unwrap();
let old_subtree = ExpressionTree::new("BvC").unwrap();
let new_subtree = ExpressionTree::new("~(C->D)");
tree.replace_expression(&old_subtree, &new_subtree); //produces A&(C->D) (the two negations canceled out)
```

# evaluating
there's two ways to evaluate an expression. 

The first way is by manually setting all of the variables and then calling `ExpressionTree::evaluate()`

This method requires your tree be mutable to set variables in it.
```rs
let mut tree = ExpressionTree::new("A&B->C").unwrap();
//tree.evaluate() will currently return an Err.
tree.set_variable("A", true);
//you can also set multiple variables at once (this is more efficient than setting them one at a time).
let mut vars = HashMap::new();
vars.insert("B", true);
vars.insert("C", false);
//tree.evaluate() will still return an Err.
tree.set_variables(vars);

assert!(!tree.evaluate());
tree.set_variable("C", true);
assert!(tree.evaluate());
```

The second method is by passing a map of all the variables into `ExpressionTree::evaluate_with_vars()`.

This method works on an immutable tree, but will come at the cost of performance (once I get around to optimizing a little).
```rs
let tree = ExpressionTree::new("A&B->C").unwrap();
let mut vars = HashMap::new();
vars.insert("A", true);
vars.insert("B", true);
//tree.evaluate_with_vars(vars) will currently return an Err.
vars.insert("C", false);
assert!(!tree.evaluate_with_vars(vars));
vars.insert("D", false); //adding extra variables is fine, but not having enough variables is not fine.
vars.insert("C", true);
assert!(tree.evaluate_with_vars(vars));
```

# printing
Printing functions are pretty simple. The most useful one is probably `ExpressionTree::infix()` (there's also `prefix()`) which just gives you string representation of the infix expression.
```rs
//without notation
let tree = ExpressionTree::new("A&B->Cv~D").unwrap();
assert_eq!(tree.infix(None), "(A&B)➞(C∨~D)");
//with notation
let mut notation = OperatorNotation::default();
notation.set(Operator::AND, "and".to_string());
notation.set(Operator::NOT, "not".to_string());
notation.set(Operator::OR, "or".to_string());
notation.set(Operator::CON, "if".to_string());
//this would make con a prefix of bicon which leads to ambiguity. Because of that, it will return an Err and not set the bicon notation.
assert_eq!(notation.set(Operator::BICON, "iff".to_string()), Err(Operator::CON));
assert_eq!(tree.infix(Some(&notation)), "(AandB)if(CornotD)");

```

# analyzing
Next, we have ways of analyzing individual expressions.

At the current moment, most of these functions are extremely unoptimized and will run pretty slow. You shouldn't really notice unless you're really pushing this package to it's limits.

Here are some functions for analyzing a single expression:
```rs
//a tautology is an expression that always evaluates to true.
let taut = ExpressionTree::new("Av~A").unwrap();
assert!(taut.is_tautology());
assert!(!taut.is_inconsistency());
assert!(!taut.is_contingency());
assert!(taut.is_satisfiable());
assert_eq!(taut.satisfy_count()[0], 2);
assert_eq!(taut.satisfy_all().len(), 2);
assert!(taut.evaluate_with_vars(taut.satisfy_one().unwrap()));

//an inconsistency is an expression that always evaluates to false.
let inco = ExpressionTree::new("Av~A").unwrap();
assert!(!inco.is_tautology());
assert!(inco.is_inconsistency());
assert!(!inco.is_contingency());
assert!(!inco.is_satisfiable());
assert_eq!(inco.satisfy_count()[0], 0);
assert_eq!(taut.satisfy_all().len(), 0);
assert!(inco.satisfy_one().is_none());

//a contingency is an expression that is sometimes true and sometimes false.
let cont = ExpressionTree::new("A").unwrap();
assert!(!cont.is_tautology());
assert!(!cont.is_inconsistency());
assert!(cont.is_contingency());
assert!(cont.is_satisfiable());
assert_eq!(cont.satisfy_count()[0], 1);
assert_eq!(taut.satisfy_all().len(), 1);
assert!(cont.evaluate_with_vars(cont.satisfy_one().unwrap()));
```

# comparison
Finally, we have ways of comparing multiple expressions. (these are also very unoptimized right now)
```rs
let t1 = ExpressionTree::new("A&B").unwrap();
let t2 = ExpressionTree::new("B&A").unwrap();
let t3 = ExpressionTree::new("A&C").unwrap();
let t4 = ExpressionTree::new("A&C").unwrap();
let t5 = ExpressionTree::new("Av~A").unwrap();
let t6 = ExpressionTree::new("Bv~B").unwrap();


//logical equivalence checks for if the expressions have the exact same logical content.
assert!(t1.log_eq(&t2));
assert!(!t1.log_eq(&t3));
assert!(t5.log_eq(&t6));
//literal equivalence checks for if the expressions are literally exactly the same.
assert!(!t1.lit_eq(&t2));
assert!(t3.lit_eq(&t4));
//syntactic equivalence checks for if the expressions are logically equivalent AND have the same variables.
//this is distinct from logical equivalence as all tautologies are logically equivalent, but not syntactically equivalent.
assert!(t1.syn_eq(&t2));
assert!(!t5.syn_eq(&t6));
```