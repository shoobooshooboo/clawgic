# clawgic
A rust engine for Sentential (Propositional) Logic. Will later be expanded to deal with predicate logic as well.

## Things you can do right now:

The main data structure you'll be using is the `ExpressionTree`. `ExpressionTree`s represent a boolean expression with constants, variables, and operators. The operators currently supported are the following:
* conjunction (&)
* disjunction (v),
* conditional (->),
* biconditional (<->)
* negation (~)

# Constructing an `ExpressionTree`
There are a few ways that you can construct an `ExpressionTree`. The first and simplest is with `ExpressionTree::new()`.
```rs
let mut tree = ExpressionTree::new("~A&B1->Cv~(D42<->E)").unwrap();
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

# analyzing
Next, we have ways of analyzing individual expressions.

At the current moment, most of these functions are extremely unoptimized and will run pretty slow. You shouldn't really notice unless you're really pushing this package to it's limits (it's current limits being comparing two functions with a combined 127 distinct variables).

Here are some functions for analyzing a single expression:
```rs
//a tautology is an expression that always evaluates to true.
let taut = ExpressionTree::new("Av~A").unwrap();
assert!(taut.is_tautology());
assert!(!taut.is_inconsistency());
assert!(!taut.is_contingency());
assert!(taut.is_satisfiable());
assert_eq!(taut.satisfy_count(), 2);
assert_eq!(taut.satisfy_all().len(), 2);
assert!(taut.evaluate_with_vars(taut.satisfy_one().unwrap()));

//an inconsistency is an expression that always evaluates to false.
let inco = ExpressionTree::new("Av~A").unwrap();
assert!(!inco.is_tautology());
assert!(inco.is_inconsistency());
assert!(!inco.is_contingency());
assert!(!inco.is_satisfiable());
assert_eq!(inco.satisfy_count(), 0);
assert_eq!(taut.satisfy_all().len(), 0);
assert!(inco.satisfy_one().is_none());

//a contingency is an expression that is sometimes true and sometimes false.
let cont = ExpressionTree::new("A").unwrap();
assert!(!cont.is_tautology());
assert!(!cont.is_inconsistency());
assert!(cont.is_contingency());
assert!(cont.is_satisfiable());
assert_eq!(cont.satisfy_count(), 1);
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