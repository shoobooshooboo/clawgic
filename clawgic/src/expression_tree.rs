pub mod node;
mod shell;

use shell::Shell;
use node::Node;
use node::operator::Operator;
use std::collections::HashMap;

/// All the errors that can occur in making and managing an `ExpressionTree`. 
#[derive(Debug, PartialEq, Eq)]
pub enum ExpressionTreeError{
    UninitializedVariable,
    InvalidExpression,
    UnknownSymbol,
    InvalidParentheses,
    TooManyOperators,
    NotEnoughOperators,
    LowercaseVariables,
    AmbiguousExpression,
}

/// Expression tree for logical expressions in SL.
#[derive(Debug)]
pub struct ExpressionTree{
    /// All the unique variables in the tree and their current value.
    vars: HashMap<String, Option<bool>>,
    /// Root node of the expression Tree.
    root: Node,
}

impl ExpressionTree{
    /// Constructs a new expression tree given a string representation of an infix logical expression.
    pub fn new(expression: &str) -> Result<Self, ExpressionTreeError>{
        let shells = &mut Self::shunting_yard(expression)?;
        let root = Self::construct_tree(shells)?;
        let vars = Self::create_vars(&root, HashMap::new());
        if !shells.is_empty(){
            return Err(ExpressionTreeError::NotEnoughOperators);
        }
        Ok(Self{
            vars,
            root,
        })
    }

    /// # Shunting yard algorithm.
    /// 
    /// Takes a string representation of an infix logical expression and produces a Vec of `Shell`s.
    fn shunting_yard(mut expression: &str) -> Result<Vec<Shell>, ExpressionTreeError>{
        expression = expression.trim();
        let mut shells = Vec::<Shell>::new();
        let mut operators = Vec::<Shell>::new();

        while !expression.is_empty(){
            let mut denied = false;
            while expression.starts_with("~"){
                denied = !denied;
                expression = &expression[1..];
            }
            if denied{
                operators.push(Shell::Tilde);
            }

            let mut chars = expression.chars();
            let mut cur_char = match chars.next(){
                Some(c) => c,
                None => return Err(ExpressionTreeError::InvalidExpression),
            };
            let mut chars_consumed = 1;

            if cur_char.is_uppercase(){
                loop{
                    cur_char = match chars.next(){
                        Some(c) => c,
                        None => break,
                    };
                    if !cur_char.is_numeric(){
                        break;
                    }
                    chars_consumed += 1;
                }
                if denied{
                    operators.pop();
                }
                shells.push(Shell::Variable(denied, expression[0..chars_consumed].to_string()));
            }
            else if cur_char == '&' || cur_char == 'v' || cur_char == '<' || cur_char == '-' || cur_char == '>'{
                let op: Operator;
                if cur_char == '&' {
                    op = Operator::AND;
                }else if cur_char == 'v'{
                    op = Operator::OR;
                }else if cur_char == '<'{
                    op = Operator::BICON;
                    chars_consumed += 1;
                    loop{
                        cur_char = match chars.next(){
                            Some(c) => c,
                            None => return Err(ExpressionTreeError::UnknownSymbol),
                        };
                        if cur_char != '-'{
                            break;
                        }
                        chars_consumed += 1
                    }
                    if cur_char != '>'{
                        return Err(ExpressionTreeError::UnknownSymbol);
                    }
                }else/*if cur_char == '-' || cur_char == '>'*/{
                    op = Operator::CON;
                    while cur_char == '-'{
                        cur_char = match chars.next(){
                            Some(c) => c,
                            None => return Err(ExpressionTreeError::UnknownSymbol),
                        };
                        chars_consumed += 1;
                    }
                    if cur_char != '>'{
                        return Err(ExpressionTreeError::UnknownSymbol);
                    }
                }
                match operators.last(){
                    None => operators.push(Shell::Operator(false, op)),
                    Some(_) => {
                        while let Some(Shell::Operator(_, o)) = operators.last(){
                            if o.precedence() < op.precedence(){
                                break;
                            }else if o.precedence() == op.precedence(){
                                return Err(ExpressionTreeError::AmbiguousExpression);
                            }
                            shells.push(operators.pop().unwrap());
                        }
                        if let Some(Shell::Tilde) = operators.last(){
                            denied = true;
                            operators.pop();
                        }
                        operators.push(Shell::Operator(denied, op));
                    },
                }
            }
            else if cur_char == '('{
                operators.push(Shell::Parentheses);
            }
            else if cur_char == ')'{
                while operators.last().is_some_and(|op| !op.is_parentheses()){
                    shells.push(operators.pop().unwrap());
                }
                if operators.pop().is_none_or(|x| !x.is_parentheses()){
                    return Err(ExpressionTreeError::InvalidParentheses);
                }
                if operators.last().is_some_and(|x| x.is_tilde()){
                    operators.pop();
                    match shells.pop(){
                        Some(s) => {
                            if let Shell::Operator(_, op) = s{
                                shells.push(Shell::Operator(true, op));
                            }else{
                                return Err(ExpressionTreeError::InvalidExpression)
                            }
                        },
                        None => return Err(ExpressionTreeError::InvalidExpression),
                    }
                }
            }
            else{
                if cur_char.is_lowercase(){
                    return Err(ExpressionTreeError::LowercaseVariables);
                }
                return Err(ExpressionTreeError::UnknownSymbol);
            }

            expression = &expression[chars_consumed..];
        }

        while !operators.is_empty(){
            shells.push(operators.pop().unwrap());
        }

        Ok(shells)
    }

    /// Takes a Vec of `Shell`s, constructs a subtree of `Node`s and returns the root node of that subtree. 
    fn construct_tree(shells: &mut Vec<Shell>) -> Result<Node, ExpressionTreeError>{
        let node = match shells.pop(){
            Some(s) => {
                match s {
                    Shell::Operator(denied, op) => {
                        let right = Self::construct_tree(shells)?;
                        let left = Self::construct_tree(shells)?;
                        Node::Operator { denied, op, left: Box::new(left), right: Box::new(right) }
                    },
                    Shell::Variable(denied, name) => Node::Variable { denied, name, value: None },
                    Shell::Constant(value) => Node::Constant(value),
                    Shell::Parentheses => return Err(ExpressionTreeError::InvalidParentheses),
                    Shell::Tilde => return Err(ExpressionTreeError::InvalidExpression),
                }
            },
            None => return Err(ExpressionTreeError::TooManyOperators),
        };

        Ok(node)
    }

    /// Takes a `Node` and the vars map and does a depth-first-search for every variable, inserting them into the map as they are found.
    fn create_vars(node: & Node, mut vars: HashMap<String, Option<bool>>) -> HashMap<String, Option<bool>>{
        let vars = match node{
            Node::Operator { denied: _, op: _, left, right } =>{
                let vars = Self::create_vars(left, vars);
                Self::create_vars(right, vars)
            },
            Node::Constant(_) => vars,
            Node::Variable { denied: _, name, value } => {
                vars.insert(name.clone(), *value);
                vars
            },
        };

        vars
    }

    /// Searches for every variable with the given name and updates it's value.
    pub fn set_variable(&mut self, name: &str, value: bool){
        if self.vars.contains_key(name){
            self.vars.insert(name.to_string(), Some(value));
            Self::set_variable_rec(name, value, &mut self.root);
        }
    }

    /// Recursive helper function for `ExpressionTree::set_variable().`
    fn set_variable_rec(target: &str, val: bool, cur_node: &mut Node){
        match cur_node{
            Node::Constant(_) => (),
            Node::Operator{ denied: _, op: _, left, right } => {
                Self::set_variable_rec(target, val, left);
                Self::set_variable_rec(target, val, right);
            }
            Node::Variable { denied: _, name, value } => {
                if *name == target{
                    *value = Some(val);
                }
            }
        }
    }

    /// Attempts to evaluate the tree.
    pub fn evaluate(&self) -> Result<bool, ExpressionTreeError>{
        self.root.evaluate()
    }

    /// Gets the prefix representation of the tree.
    pub fn prefix(&self) -> String{
        let mut prefix = String::new();
        Self::prefix_rec(&self.root, &mut prefix);
        prefix
    }

    /// Recurseive helper function for `ExpressionTree::prefix().`
    fn prefix_rec(node: &Node, prefix: &mut String){
        prefix.push_str(&node.to_string());
        match node{
            Node::Operator { denied: _, op: _, left, right } => {
                Self::prefix_rec(left, prefix);
                Self::prefix_rec(right, prefix);
            }
            _ => (),
        }
    }

    /// Gets the infix representation of the tree.
    pub fn infix(&self) -> String{
        let mut infix = String::new();
        Self::infix_rec(&self.root, &mut infix);
        infix
    }

    /// Recursive helper function for `ExpressionTree::infix().`
    fn infix_rec(node: &Node, infix: &mut String){
        match node{
            Node::Operator { denied: _, op: _, left, right } => {
                infix.push('(');
                Self::infix_rec(left, infix);
                infix.push_str(&node.to_string());
                Self::infix_rec(right, infix);
                infix.push(')');
            }
            _ => infix.push_str(&node.to_string()),
        }
    }

    /// Gets the variables map of the tree.
    pub fn vars(&self) -> &HashMap<String, Option<bool>>{
        &self.vars
    }

    /// Converts all operators in the tree into conjunctions and disjunctions with no leading denials.
    pub fn monotenize(&mut self){
        Self::monotenize_rec(&mut self.root);
    }

    /// Recursive helper function for `ExpressionTree::monotenize()`.
    fn monotenize_rec(node: &mut Node){
        match &*node{
            Node::Operator { denied, op, left: _, right: _ } => {
                if (op.is_and() || op.is_or()) && *denied{
                    node.demorgans();
                }else if op.is_con(){
                    if *denied{
                        node.ncon();
                    }else{
                        node.implication();
                    }
                }else if op.is_bicon(){
                    node.mat_eq_mono();
                }
            }
            _ => (),
        }

        match node{
            Node::Operator { denied: _, op: _, left, right } => {
                Self::monotenize_rec(left);
                Self::monotenize_rec(right);
            },
            _ => (),
        }
    }

    /// Consumes tree and returns the root node.
    pub fn into_node(self) -> Node{
        self.root
    }

    ///consumes two trees and returns a tree in the form of first & second.
    pub fn and(mut first: Self, second: Self) -> Self{
        for (name, val) in second.vars{
            first.vars.entry(name).or_insert(val);
        }

        Self { vars: first.vars, root: Node::Operator{denied: false, op: node::operator::Operator::AND, left: Box::new(first.root), right: Box::new(second.root)} }
    }

    ///consumes two trees and returns a tree in the form of forst v (wedge) second.
    pub fn or(mut first: Self, second: Self) -> Self{
        for (name, val) in second.vars{
            first.vars.entry(name).or_insert(val);
        }

        Self { vars: first.vars, root: Node::Operator{denied: false, op: node::operator::Operator::OR, left: Box::new(first.root), right: Box::new(second.root)} }
    }

    ///consumes two trees and returns a tree in the form of antecedent->consequent.
    pub fn con(mut antecedent: Self, consequent: Self) -> Self{
        for (name, val) in consequent.vars{
            antecedent.vars.entry(name).or_insert(val);
        }

        Self { vars: antecedent.vars, root: Node::Operator{denied: false, op: node::operator::Operator::CON, left: Box::new(antecedent.root), right: Box::new(consequent.root)} }
    }

    ///consumes two trees and returns a tree in the form of first->second.
    pub fn bicon(mut first: Self, second: Self) -> Self{
        for (name, val) in second.vars{
            first.vars.entry(name).or_insert(val);
        }

        Self { vars: first.vars, root: Node::Operator{denied: false, op: node::operator::Operator::BICON, left: Box::new(first.root), right: Box::new(second.root)} }
    }

    ///consumes the tree and produces a tree in the form of ~self.
    pub fn not(mut self) -> Self{
        self.root.deny();
        self
    }
}

impl Default for ExpressionTree{
    /// Default value is just a constant false node.
    fn default() -> Self {
        Self { vars: HashMap::new(), root: Node::Constant(false) }
    }
}

impl From<Node> for ExpressionTree{
    fn from(n: Node) -> Self{
        Self { vars: Self::create_vars(&n, HashMap::new()), root: n }
    }
}

#[cfg(test)]
mod test{
    use test_case::test_case;
    use crate::expression_tree::{ExpressionTree, ExpressionTreeError};

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
}