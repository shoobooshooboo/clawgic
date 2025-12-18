pub mod node;
mod shell;

use shell::Shell;
use node::Node;
use node::operator::Operator;
use std::collections::HashMap;

/// All the errors that can occur in making and managing an `ExpressionTree`. 
#[derive(Debug, PartialEq, Eq)]
pub enum ExpressionTreeError{
    UninitializedVariable(String),
    InvalidExpression,
    UnknownSymbol,
    InvalidParentheses,
    TooManyOperators,
    NotEnoughOperators,
    LowercaseVariables,
    AmbiguousExpression,
}

/// Expression tree for logical expressions in SL.
#[derive(Debug, Clone)]
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
            expression = expression.trim_start();
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

    /// Attempts to evaluate the tree with the given set of variables.
    pub fn evaluate_with_vars(&self, vars: &HashMap<String, bool>) -> Result<bool, ExpressionTreeError>{
        self.root.evaluate_with_vars(vars)
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

    ///consumes two trees and returns a tree in the form of self & second.
    pub fn and(mut self, second: Self) -> Self{
        for (name, val) in second.vars{
            self.vars.entry(name).or_insert(val);
        }

        Self { vars: self.vars, root: Node::Operator{denied: false, op: node::operator::Operator::AND, left: Box::new(self.root), right: Box::new(second.root)} }
    }

    ///consumes two trees and returns a tree in the form of self v (wedge) second.
    pub fn or(mut self, second: Self) -> Self{
        for (name, val) in second.vars{
            self.vars.entry(name).or_insert(val);
        }

        Self { vars: self.vars, root: Node::Operator{denied: false, op: node::operator::Operator::OR, left: Box::new(self.root), right: Box::new(second.root)} }
    }

    ///consumes two trees and returns a tree in the form of self->consequent.
    pub fn con(mut self, consequent: Self) -> Self{
        for (name, val) in consequent.vars{
            self.vars.entry(name).or_insert(val);
        }

        Self { vars: self.vars, root: Node::Operator{denied: false, op: node::operator::Operator::CON, left: Box::new(self.root), right: Box::new(consequent.root)} }
    }

    ///consumes two trees and returns a tree in the form of self->second.
    pub fn bicon(mut self: Self, second: Self) -> Self{
        for (name, val) in second.vars{
            self.vars.entry(name).or_insert(val);
        }

        Self { vars: self.vars, root: Node::Operator{denied: false, op: node::operator::Operator::BICON, left: Box::new(self.root), right: Box::new(second.root)} }
    }

    ///consumes the tree and produces a tree in the form of ~self.
    pub fn not(mut self) -> Self{
        self.root.deny();
        self
    }

    ///checks if the two expressions are logically equivalent (produce the same truth tables). Very expensive function. 
    /// 
    /// Currently supports up to 127 different variables.
    pub fn log_eq(&self, other: &Self) -> bool{
        let mut vars = HashMap::new();

        for (name, _) in self.vars.iter(){
            vars.insert(name.clone(), false);
        }
        for (name, _) in other.vars.iter(){
            vars.insert(name.clone(), false);
        }

        let max: u128 = 2 << vars.len();
        for cur in 0..max{
            //this loop is technically const time, since the function currently only supports up to 127 variables.
            for (i, (_, b)) in vars.iter_mut().enumerate(){
                let i = i as u8;
                *b = cur >> i & 1 == 1;
            }
            

            if self.evaluate_with_vars(&vars) != other.evaluate_with_vars(&vars){
                return false;
            }
        }

        true
    }

    ///checks if the two expressions are literally exactly the same (ignoring double negations).
    pub fn lit_eq(&self, other: &Self) -> bool{
        //this can be optimized later, but for now, it's fine.
        self.prefix() == other.prefix()
    }

    ///checks if the two expressions are syntactically the same (one can be transformed into the other with primitive logic rules). Very expensive function.
    pub fn syn_eq(&self, other: &Self) -> bool{
        //check if they use only the same variables.
        let mut same_vars = true;
        self.vars().iter().for_each(|(name, _)| if !other.vars.contains_key(name) {same_vars = false});
        other.vars().iter().for_each(|(name, _)| if !self.vars.contains_key(name) {same_vars = false});
        if !same_vars{
            return false;
        }
        //check for logical equivalence
        self.log_eq(other)
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

impl std::ops::Not for ExpressionTree{
    type Output = ExpressionTree;

    fn not(self) -> Self::Output {
        self.not()
    }
}

impl std::ops::BitOr for ExpressionTree{
    type Output = ExpressionTree;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.or(rhs)
    }
}

impl std::ops::BitAnd for ExpressionTree{
    type Output = ExpressionTree;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.and(rhs)
    }
}

impl std::ops::BitXor for ExpressionTree{
    type Output = ExpressionTree;
    
    fn bitxor(self, rhs: Self) -> Self::Output {
        self.bicon(rhs).not()
    }
}

///equivalent to lhs.con(rhs)
impl std::ops::Shr for ExpressionTree{
    type Output = ExpressionTree;

    fn shr(self, rhs: Self) -> Self::Output {
        self.con(rhs)
    }
}

///equivalent to rhs.con(lhs)
impl std::ops::Shl for ExpressionTree{
    type Output = ExpressionTree;

    fn shl(self, rhs: Self) -> Self::Output {
        rhs.con(self)
    }
}

impl std::ops::BitOrAssign for ExpressionTree{
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.clone().or(rhs);
    }
}

impl std::ops::BitAndAssign for ExpressionTree{
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.clone().and(rhs);
    }
}

impl std::ops::BitXorAssign for ExpressionTree{
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.clone().bicon(rhs).not();
    }
}

///equivalent to lhs = lhs.con(rhs)
impl std::ops::ShrAssign for ExpressionTree{
    fn shr_assign(&mut self, rhs: Self) {
        *self = self.clone().con(rhs);
    }
}

///equivalent to rhs = rhs.con(lhs)
impl std::ops::ShlAssign for ExpressionTree{
    fn shl_assign(&mut self, rhs: Self) {
        *self = rhs.con(self.clone());
    }
} 