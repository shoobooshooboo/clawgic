pub mod node;
mod shell;

use shell::Shell;
use node::Node;
use node::operator::Operator;
use std::cell::Cell;
use std::collections::HashMap;

use crate::expression_tree::node::negation::Negation;
use crate::operator_notation::OperatorNotation;

/// All the errors that can occur in making and managing an `ExpressionTree`. 
#[derive(Debug, PartialEq, Eq)]
pub enum ExpressionTreeError{
    UninitializedVariable(String),
    InvalidExpression,
    UnknownSymbol,
    InvalidParentheses,
    TooManyOperators,
    NotEnoughOperators,
    LowercaseVariables(char),
    AmbiguousExpression,
}

impl std::fmt::Display for ExpressionTreeError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self{
            Self::UninitializedVariable(s) => format!("Uninitialized variable \"{s}\""),
            Self::InvalidExpression => "Invalid expression".to_string(),
            Self::UnknownSymbol => "Unknown symbol".to_string(),
            Self::InvalidParentheses => "Invalid parenthesis".to_string(),
            Self::TooManyOperators => "Too many operators".to_string(),
            Self::NotEnoughOperators => "Not enough operators".to_string(),
            Self::LowercaseVariables(c) => format!("Lowercase variable \"{c}\""),
            Self::AmbiguousExpression => "Ambiguous expression".to_string(),
        })
    }
}

impl std::error::Error for ExpressionTreeError{}

/// Expression tree for logical expressions in SL.
#[derive(Debug, Clone)]
pub struct ExpressionTree{
    /// All the unique variables in the tree and their current value.
    vars: HashMap<String, Option<bool>>,
    /// Root node of the expression Tree.
    root: Node,
    /// Cached previous result of `evaluate()`
    value: Cell<Option<bool>>
}

impl ExpressionTree{
    ///returns a tree that is just a true node
    #[allow(non_snake_case)]
    pub fn TRUE() -> Self{
        Self { vars: HashMap::new(), root: Node::Constant(Negation::default(), true), value: Cell::new(Some(true)) }
    }

    /// Returns a tree that is just a false node
    #[allow(non_snake_case)]
    pub fn FALSE() -> Self{
        Self { vars: HashMap::new(), root: Node::Constant(Negation::default(), false), value: Cell::new(Some(false)) }
        
    }

    // Constructs a tree with a single constant node of the given value.
    pub fn constant(b: bool) -> Self{
        Self { vars: HashMap::new(), root: Node::Constant(Negation::default(), b), value: Cell::new(Some(b)) }
    }

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
            value: Cell::new(None),
        })
    }

    /// Constructs a new expression tree given a string representation of an infix logical expression and an 
    /// `OperatorNotation` detailing the accepted operators.
    pub fn new_with_notation(expression: &str, notation: &OperatorNotation) -> Result<Self, ExpressionTreeError>{
        let shells = &mut Self::shunting_yard_with_notation(expression, notation)?;
        let root = Self::construct_tree(shells)?;
        let vars = Self::create_vars(&root, HashMap::new());
        if !shells.is_empty(){
            return Err(ExpressionTreeError::NotEnoughOperators);
        }
        Ok(Self{
            vars,
            root,
            value: Cell::new(None),
        })
    }

    /// Takes a string representation of an infix logical expression and an `OperatorNotation` and produces a Vec of `Shell`s.
    fn shunting_yard_with_notation(mut expression: &str, notation: &OperatorNotation) -> Result<Vec<Shell>, ExpressionTreeError>{
        expression = expression.trim();
        let mut shells = Vec::<Shell>::new();
        let mut operators = Vec::<Shell>::new();

        while !expression.is_empty(){
            expression = expression.trim_start();
            let mut negation = Negation::default();
            while expression.starts_with(notation.neg()){
                negation.negate();
                expression = &expression[notation.neg().as_bytes().len()..];
            }

            if expression.starts_with("TRUE"){
                shells.push(Shell::Constant(negation, true));
                expression = &expression[4..];
                continue;
            }else if expression.starts_with("FALSE"){
                shells.push(Shell::Constant(negation, false));
                expression = &expression[5..];
                continue;
            }

            if negation.count() > 0{
                operators.push(Shell::Tilde(negation));
            }

            let mut chars = expression.chars();
            let mut cur_char = match chars.next(){
                Some(c) => c,
                None => return Err(ExpressionTreeError::InvalidExpression),
            };
            let mut chars_consumed = 0;

            if cur_char.is_uppercase(){
                loop{
                    chars_consumed += cur_char.len_utf8();
                    cur_char = match chars.next(){
                        Some(c) => c,
                        None => break,
                    };
                    if !cur_char.is_numeric(){
                        break;
                    }
                }
                if negation.count() > 0{
                    operators.pop();
                }
                shells.push(Shell::Variable(negation, expression[0..chars_consumed].to_string()));
            }
            else if expression.starts_with(notation.and()) || expression.starts_with(notation.or()) || 
                    expression.starts_with(notation.con()) || expression.starts_with(notation.bicon()){
                let op: Operator = 
                    if expression.starts_with(notation.and()) {chars_consumed = notation.and().as_bytes().len(); Operator::AND} 
                    else if expression.starts_with(notation.or()) {chars_consumed = notation.or().as_bytes().len(); Operator::OR}
                    else if expression.starts_with(notation.con()) {chars_consumed = notation.con().as_bytes().len(); Operator::CON}
                    else /*if expression.starts_with(notation.bicon())*/{chars_consumed = notation.bicon().as_bytes().len(); Operator::BICON};
                    
                match operators.last(){
                    None => operators.push(Shell::Operator(Negation::default(), op)),
                    Some(_) => {
                        while let Some(Shell::Operator(_, o)) = operators.last(){
                            if o.precedence() < op.precedence(){
                                break;
                            }else if o.precedence() == op.precedence(){
                                return Err(ExpressionTreeError::AmbiguousExpression);
                            }
                            shells.push(operators.pop().unwrap());
                        }
                        if let Some(Shell::Tilde(n)) = operators.last(){
                            negation = *n;
                            operators.pop();
                        }
                        operators.push(Shell::Operator(negation, op));
                    },
                }
            }
            else if cur_char == '('{
                operators.push(Shell::Parentheses);
                chars_consumed = 1;
            }
            else if cur_char == ')'{
                while operators.last().is_some_and(|op| !op.is_parentheses()){
                    shells.push(operators.pop().unwrap());
                }
                if operators.pop().is_none_or(|x| !x.is_parentheses()){
                    return Err(ExpressionTreeError::InvalidParentheses);
                }
                if let Some(Shell::Tilde(n)) = operators.pop_if(|s| s.is_tilde()){
                    match shells.pop(){
                        Some(s) => {
                            if let Shell::Operator(_, op) = s{
                                shells.push(Shell::Operator(n, op));
                            }else{
                                return Err(ExpressionTreeError::InvalidExpression)
                            }
                        },
                        None => return Err(ExpressionTreeError::InvalidExpression),
                    }
                }
                chars_consumed = 1;
            }
            else{
                if cur_char.is_lowercase(){
                    return Err(ExpressionTreeError::LowercaseVariables(cur_char));
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

    /// # Shunting yard algorithm.
    /// 
    /// Takes a string representation of an infix logical expression and produces a Vec of `Shell`s.
    fn shunting_yard(mut expression: &str) -> Result<Vec<Shell>, ExpressionTreeError>{
        expression = expression.trim();
        let mut shells = Vec::<Shell>::new();
        let mut operators = Vec::<Shell>::new();

        while !expression.is_empty(){
            expression = expression.trim_start();
            let mut negation = Negation::default();
            while expression.starts_with('~') || expression.starts_with('!') || expression.starts_with('¬'){
                negation.negate();
                expression = if expression.starts_with('¬') {&expression[2..]} else {&expression[1..]};
            }

            if expression.starts_with("TRUE"){
                shells.push(Shell::Constant(negation, true));
                expression = &expression[4..];
                continue;
            }else if expression.starts_with("FALSE"){
                shells.push(Shell::Constant(negation, false));
                expression = &expression[5..];
                continue;
            }

            if negation.count() > 0{
                operators.push(Shell::Tilde(negation));
            }

            let mut chars = expression.chars();
            let mut cur_char = match chars.next(){
                Some(c) => c,
                None => return Err(ExpressionTreeError::InvalidExpression),
            };
            let mut chars_consumed = cur_char.len_utf8();

            if cur_char.is_uppercase(){
                loop{
                    cur_char = match chars.next(){
                        Some(c) => c,
                        None => break,
                    };
                    if !cur_char.is_numeric(){
                        break;
                    }
                    chars_consumed += cur_char.len_utf8();
                }
                if negation.count() > 0{
                    operators.pop();
                }
                shells.push(Shell::Variable(negation, expression[0..chars_consumed].to_string()));
            }
            else if cur_char == '&' || cur_char == '*' || cur_char == '∧' || cur_char == '^' || cur_char == '⋅' ||
                    cur_char == 'v' || cur_char == '∨' || cur_char == '|' || cur_char == '+' || 
                    cur_char == '<' || cur_char == '-' || cur_char == '>' || cur_char == '➞' || cur_char == '⟷' {
                let op: Operator;
                match cur_char{
                    '&' | '*' | '∧' | '^' | '⋅' => op = Operator::AND,
                    'v' | '|' | '+' | '∨' => op = Operator::OR,
                    '➞' => op = Operator::CON,
                    '⟷' => op = Operator::BICON,
                    '<' => {
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
                    }
                    _ /*'-' | '>' */ => {
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
                }
                match operators.last(){
                    None => operators.push(Shell::Operator(Negation::default(), op)),
                    Some(_) => {
                        while let Some(Shell::Operator(_, o)) = operators.last(){
                            if o.precedence() < op.precedence(){
                                break;
                            }else if o.precedence() == op.precedence(){
                                return Err(ExpressionTreeError::AmbiguousExpression);
                            }
                            shells.push(operators.pop().unwrap());
                        }
                        if let Some(Shell::Tilde(n)) = operators.last(){
                            negation = *n;
                            operators.pop();
                        }
                        operators.push(Shell::Operator(negation, op));
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
                if let Some(Shell::Tilde(n)) = operators.pop_if(|s| s.is_tilde()){
                    match shells.pop(){
                        Some(s) => {
                            if let Shell::Operator(_, op) = s{
                                shells.push(Shell::Operator(n, op));
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
                    return Err(ExpressionTreeError::LowercaseVariables(cur_char));
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
                    Shell::Variable(denied, name) => Node::Variable { denied, name},
                    Shell::Constant(neg, value) => Node::Constant(neg, value),
                    Shell::Parentheses => return Err(ExpressionTreeError::InvalidParentheses),
                    Shell::Tilde(_) => return Err(ExpressionTreeError::InvalidExpression),
                }
            },
            None => return Err(ExpressionTreeError::TooManyOperators),
        };

        Ok(node)
    }

    //OPTIMIZATION: create vars at the same time as construct_tree to avoid excessive work.
    /// Takes a `Node` and the vars map and does a depth-first-search for every variable, inserting them into the map as they are found.
    fn create_vars(node: & Node, mut vars: HashMap<String, Option<bool>>) -> HashMap<String, Option<bool>>{
        let vars = match node{
            Node::Operator { denied: _, op: _, left, right } =>{
                let vars = Self::create_vars(left, vars);
                Self::create_vars(right, vars)
            },
            Node::Constant(..) => vars,
            Node::Variable { denied: _, name} => {
                vars.insert(name.clone(), None);
                vars
            },
        };

        vars
    }

    /// Searches for every variable with the given name and updates it's value.
    pub fn set_variable(&mut self, name: &str, value: bool){
        if self.vars.get(name).is_some_and(|v| v.is_none_or(|b| value != b)){
            self.vars.insert(name.to_string(), Some(value));
            self.value.replace(None);
        }
    }

    /// Updates the values of all the variables in `vars`.
    pub fn set_variables(&mut self, vars: &HashMap<String, bool>){
        for (name, b) in vars.iter(){
            match self.vars.get_mut(name){
                Some(v) => v.replace(*b),
                None => continue,
            };
            self.value.replace(None);
        }
    }

    /// Replaces all instances of var in the tree with new_expression. Adds all variables from new_expression to self as they are.
    pub fn replace_variable(&mut self, var: &str, new_expression: &ExpressionTree) -> &mut Self{
        if self.vars.contains_key(var){
            self.vars.remove(var);
            for (name, val) in new_expression.vars.iter(){
                if !self.vars.contains_key(name){
                    self.vars.insert(name.clone(), val.clone());
                }
            }
            Self::replace_variable_rec(&mut self.root, var, new_expression);
            self.value.replace(None);
        }

        self
    }

    /// Recursive helper function for `ExpressionTree::replace_variable()`
    fn replace_variable_rec(cur_node: &mut Node, var: &str, new_expression: &ExpressionTree){
        if cur_node.is_variable(){
            let Node::Variable { denied, name} = cur_node.clone()
                else{panic!("this should never happen (in replace_variable_rec())")};
            if var == name{
                *cur_node = new_expression.root.clone();
                if denied.is_denied(){
                    cur_node.deny();
                }
            }
        }else if cur_node.is_operator(){
            let Node::Operator { denied: _, op: _, left, right } = cur_node 
                else{panic!("this should never happen (in replace_variable_rec())")};
            Self::replace_variable_rec(left, var, new_expression);
            Self::replace_variable_rec(right, var, new_expression);
        }
    }

    /// Replaces all instances of var in the tree with new_expression. Adds all variables from new_expression to self as they are.
    pub fn replace_variables(&mut self, vars: &HashMap<String, &ExpressionTree>) -> &mut Self{
        //gotta remove all vars before adding the new ones.
        let mut something_in_vars = false;
        let mut was_in_vars = Vec::with_capacity(vars.len());
        for (var, _) in vars.iter(){
            if self.vars.remove(var).is_some(){
                was_in_vars.push(true);
                something_in_vars = true;
            }else{
                was_in_vars.push(false);
            }
        }
        for (i, (_, new_expression)) in vars.iter().enumerate(){
            if was_in_vars[i]{
                for (name, val) in new_expression.vars.iter(){
                    if !self.vars.contains_key(name){
                        self.vars.insert(name.clone(), val.clone());
                    }
                }
            }
        }
        if something_in_vars{
            Self::replace_variables_rec(&mut self.root, vars);
            self.value.replace(None);
        }

        self
    }

    /// Recursive helper function for `ExpressionTree::replace_variable()`
    fn replace_variables_rec(cur_node: &mut Node, vars: &HashMap<String, &ExpressionTree>){
        if cur_node.is_variable(){
            let Node::Variable { denied, name} = cur_node.clone()
                else{panic!("this should never happen (in replace_variable_rec())")};
            match vars.get(&name){
                Some(new_expression) => {
                    *cur_node = new_expression.root.clone();
                    if denied.is_denied(){
                        cur_node.deny();
                    }
                },
                None => (),
            }
        }else if cur_node.is_operator(){
            let Node::Operator { denied: _, op: _, left, right } = cur_node 
                else{panic!("this should never happen (in replace_variable_rec())")};
            Self::replace_variables_rec(left, vars);
            Self::replace_variables_rec(right, vars);
        }
    }

    ///replaces all instances of old expression in the tree with new expression.
    pub fn replace_expression(&mut self, old: &ExpressionTree, new: &ExpressionTree){
        Self::replace_expression_rec(&mut self.root, old, new);
        let mut new_vars= Self::create_vars(&self.root, HashMap::new());

        for (name, val) in self.vars.iter(){
           if let Some(var) = new_vars.get_mut(name){
                *var = *val; 
            }
        }
        for (name, val) in new.vars.iter(){
            if let Some(var) = new_vars.get_mut(name){
                if var.is_none(){
                    *var = *val;
                }
            }
        }
    }

    fn replace_expression_rec(cur_node: &mut Node, old: &ExpressionTree, new: &ExpressionTree){
        if *cur_node == old.root || (cur_node.is_constant() && old.root.is_constant()){
            *cur_node = new.root.clone();
            return;
        }
        if cur_node.is_variable() && old.root.is_variable(){
            let Node::Variable { denied: cur_denied, name: cur_name } = cur_node 
                else {panic!("this shouldn't be possible (replace_expression_rec)")};
            let Node::Variable { denied: old_denied, name: old_name } = &old.root
                else {panic!("this shouldn't be possible (replace_expression_rec)")};
            if old_name == cur_name{
                let deny = *cur_denied != *old_denied;
                *cur_node = new.root.clone();
                if deny{
                    cur_node.deny();
                }
            }
        }else if cur_node.is_operator() && old.root.is_operator(){
            let Node::Operator { denied: cur_denied, op: cur_op, left: cur_left, right: cur_right } = cur_node
                else {panic!("this shouldn't be possible (replace_expression_rec)")};
            let Node::Operator { denied: old_denied, op: old_op, left: old_left, right: old_right } = &old.root
                else {panic!("this shouldn't be possible (replace_expression_rec)")};

            if *cur_op == *old_op && cur_left == old_left && cur_right == old_right{
                let deny = *cur_denied != *old_denied;
                *cur_node = new.root.clone();
                if deny{
                    cur_node.deny();
                }
            }else{
                Self::replace_expression_rec(cur_left, old, new);
                Self::replace_expression_rec(cur_right, old, new);
            }
        }
    }

    /// Attempts to evaluate the tree.
    pub fn evaluate(&self) -> Result<bool, ExpressionTreeError>{
        match self.value.get(){
            Some(v) => Ok(v),
            None => {
                let result = self.root.evaluate(&self.vars);
                match result{
                    Ok(b) => {
                        self.value.replace(Some(b));
                        Ok(b)
                    },
                    Err(e) => Err(e),
                }
            }
        }
    }

    /// Attempts to evaluate the tree with the given set of variables.
    pub fn evaluate_with_vars(&self, vars: &HashMap<String, bool>) -> Result<bool, ExpressionTreeError>{
        self.root.evaluate_with_vars(vars)
    }

    /// Gets the prefix representation of the tree.
    pub fn prefix(&self, notation: Option<&OperatorNotation>) -> String{
        let mut prefix = String::new();
        Self::prefix_rec(&self.root, &mut prefix, notation.unwrap_or(&OperatorNotation::default()));
        prefix
    }

    /// Recurseive helper function for `ExpressionTree::prefix().`
    fn prefix_rec(node: &Node, prefix: &mut String, notation: &OperatorNotation){
        prefix.push_str(&node.print(notation));
        match node{
            Node::Operator { denied: _, op: _, left, right } => {
                Self::prefix_rec(left, prefix, notation);
                Self::prefix_rec(right, prefix, notation);
            }
            _ => (),
        }
    }

    /// Gets the infix representation of the tree.
    pub fn infix(&self, notation: Option<&OperatorNotation>) -> String{
        let mut infix = String::new();
        Self::infix_rec(&self.root, &mut infix, notation.unwrap_or(&OperatorNotation::default()));
        //remove outer-most parenthesis
        if infix.starts_with('('){
            infix.remove(0);
            infix.pop();
        }
        infix
    }

    /// Recursive helper function for `ExpressionTree::infix().`
    fn infix_rec(node: &Node, infix: &mut String, notation: &OperatorNotation){
        match node{
            Node::Operator { denied, op: _, left, right } => {
                let mut op = node.print(notation);
                if denied.is_denied(){
                    //TODO!: make this less ugly
                    infix.push_str(&notation.neg().repeat(denied.count() as usize));
                    
                    op = op.chars().skip(notation.neg().chars().count() * denied.count() as usize).collect();
                }
                infix.push('(');
                Self::infix_rec(left, infix, notation);
                infix.push_str(&op);
                Self::infix_rec(right, infix, notation);
                infix.push(')');
            }
            _ => infix.push_str(&node.print(notation)),
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

    //OPTIMIZE: make monotenization work from the bottom up (monotenization expands the tree)
    /// Recursive helper function for `ExpressionTree::monotenize()`.
    fn monotenize_rec(node: &mut Node){
        match &*node{
            Node::Operator { denied, op, left: _, right: _ } => {
                if (op.is_and() || op.is_or()) && denied.is_denied(){
                    node.demorgans();
                }else if op.is_con(){
                    if denied.is_denied(){
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
    /// 
    /// If you find yourself needing this, chances are that 
    /// there's probably just a feature I have yet to add.
    pub fn into_node(self) -> Node{
        self.root
    }

    ///consumes two trees and returns a tree in the form of self & second.
    pub fn and(mut self, second: Self) -> Self{
        for (name, val) in second.vars{
            self.vars.entry(name).or_insert(val);
        }

        Self { 
            vars: self.vars, 
            root: Node::Operator{denied: Negation::default(), op: node::operator::Operator::AND, left: Box::new(self.root), right: Box::new(second.root)},
            value: Cell::new(None),
        }
    }

    ///consumes two trees and returns a tree in the form of self v (wedge) second.
    pub fn or(mut self, second: Self) -> Self{
        for (name, val) in second.vars{
            self.vars.entry(name).or_insert(val);
        }

        Self { 
            vars: self.vars, 
            root: Node::Operator{denied: Negation::default(), op: node::operator::Operator::OR, left: Box::new(self.root), right: Box::new(second.root)},
            value: Cell::new(None),
        }
    }

    ///consumes two trees and returns a tree in the form of self->consequent.
    pub fn con(mut self, consequent: Self) -> Self{
        for (name, val) in consequent.vars{
            self.vars.entry(name).or_insert(val);
        }

        Self { 
            vars: self.vars, 
            root: Node::Operator{denied: Negation::default(), op: node::operator::Operator::CON, left: Box::new(self.root), right: Box::new(consequent.root)},
            value: Cell::new(None),
        }
    }

    ///consumes two trees and returns a tree in the form of self->second.
    pub fn bicon(mut self: Self, second: Self) -> Self{
        for (name, val) in second.vars{
            self.vars.entry(name).or_insert(val);
        }

        Self { 
            vars: self.vars, 
            root: Node::Operator{denied: Negation::default(), op: node::operator::Operator::BICON, left: Box::new(self.root), right: Box::new(second.root)},
            value: Cell::new(None),
        }
    }

    ///consumes the tree and produces a tree in the form of ~self.
    pub fn not(mut self) -> Self{
        self.root.negate();
        match self.value.get_mut(){
            Some(v) => *v = !*v,
            None => (),
        };
        self
    }

    ///checks if the two expressions are logically equivalent (produce the same truth tables). Very expensive function.
    pub fn log_eq(&self, other: &Self) -> bool{
        !Self::is_satisfiable(&!self.clone().bicon(other.clone()))
    }

    ///checks if the two expressions are literally exactly the same (ignoring double negations).
    pub fn lit_eq(&self, other: &Self) -> bool{
        //this can be optimized later, but for now, it's fine.
        self.root == other.root
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

    ///checks if the expression is satisfiable. Very expensive function.
    pub fn is_satisfiable(&self) -> bool{
        let mut vars: HashMap<String, bool> = self.vars.iter().map(|(n, _)| (n.to_owned(), false)).collect();

        'outer: loop{
            if self.evaluate_with_vars(&vars).unwrap(){
                return true;
            }

            for (_, b) in vars.iter_mut(){
                *b = !*b;
                if *b{
                    continue 'outer;
                }
            }

            break;
        }

        false
    }

    ///checks if the expression is satisfiable given the auxiliary expression. Very expensive function.
    pub fn is_satisfiable_with(&self, aux: &ExpressionTree) -> bool{
        Self::is_satisfiable(&(self.clone() & aux.clone()))
    }

    ///returns a set of variables that satisfies the expression if one exists. Very expensive function.
    pub fn satisfy_one(&self) -> Option<HashMap<String, bool>>{
        let mut vars: HashMap<String, bool> = self.vars.iter().map(|(n, _)| (n.to_owned(), false)).collect();

        'outer: loop{
            if self.evaluate_with_vars(&vars).unwrap(){
                return Some(vars);
            }

            for (_, b) in vars.iter_mut(){
                *b = !*b;
                if *b{
                    continue 'outer;
                }
            }

            break;
        }

        None
    }

    ///returns a set of variables that satisfies the expression and the auxiliary expression if one exists. Very expensive function.
    pub fn satisfy_one_with(&self, aux: &ExpressionTree) -> Option<HashMap<String, bool>>{
        Self::satisfy_one(&(self.clone() & aux.clone()))
    }

    ///returns a vector of all sets of variables that satisfy the expression. Extremely expensive function.
    pub fn satisfy_all(&self) -> Vec<HashMap<String, bool>>{
        let mut vars: HashMap<String, bool> = self.vars.iter().map(|(n, _)| (n.to_owned(), false)).collect();
        let mut maps = Vec::new();

        'outer: loop{
            if self.evaluate_with_vars(&vars).unwrap(){
                maps.push(vars.clone());
            }

            for (_, b) in vars.iter_mut(){
                *b = !*b;
                if *b{
                    continue 'outer;
                }
            }

            break;
        }

        maps
    }

    ///returns a vector of all sets of variables that satisfy the expression and the auxiliary expression. Extremely expensive function.
    pub fn satisfy_all_with(&self, aux: &ExpressionTree) -> Vec<HashMap<String, bool>>{
        Self::satisfy_all(&(self.clone() & aux.clone()))
    }

    ///returns the total number of ways the expression can be satisfied. very expensive function.
    pub fn satisfy_count(&self) -> Vec<u128>{
        let mut vars: HashMap<String, bool> = self.vars.iter().map(|(n, _)| (n.to_owned(), false)).collect();
        let len = 1 + vars.len() / 128;
        let mut count = vec![0 ; len];

        'outer: loop{
            if self.evaluate_with_vars(&vars).unwrap(){
                for c in count.iter_mut(){
                    if *c != std::u128::MAX{
                        *c += 1;
                        break;
                    }
                    *c = 0;
                }
            }

            for (_, b) in vars.iter_mut(){
                *b = !*b;
                if *b{
                    continue 'outer;
                }
            }

            break;
        }

        count
    }

    ///returns the total number if ways the expression can be satisfied with the auxiliary expression. very expensive function.
    pub fn satisfy_count_with(&self, aux: &ExpressionTree) -> Vec<u128>{
        Self::satisfy_count(&(self.clone() & aux.clone()))        
    }

    ///returns whether the expression is a tautology (always true). Very expensive function.
    pub fn is_tautology(&self) -> bool{
        let mut vars: HashMap<String, bool> = self.vars.iter().map(|(n, _)| (n.to_owned(), false)).collect();

        'outer: loop{
            if !self.evaluate_with_vars(&vars).unwrap(){
                return false;
            }

            for (_, b) in vars.iter_mut(){
                *b = !*b;
                if *b{
                    continue 'outer;
                }
            }

            break;
        }

        true
    }

    ///returns whether the expression is tautological with the auxiliary expression. Very expensive function.
    pub fn is_tautology_with(&self, aux: &ExpressionTree) -> bool{
        Self::is_inconsistency(&(self.clone() & aux.clone()))
    }

    ///returns whether the expression is an inconsistency (always false). Very expensive function.
    pub fn is_inconsistency(&self) -> bool{
        let mut vars: HashMap<String, bool> = self.vars.iter().map(|(n, _)| (n.to_owned(), false)).collect();

        'outer: loop{
            if self.evaluate_with_vars(&vars).unwrap(){
                return false;
            }

            for (_, b) in vars.iter_mut(){
                *b = !*b;
                if *b{
                    continue 'outer;
                }
            }

            break;
        }

        true
    }

    ///returns whether the expression is inconsistent with the auxiliary expression. Very expensive function.
    pub fn is_inconsistency_with(&self, aux: &ExpressionTree) -> bool{
        Self::is_inconsistency(&(self.clone() & aux.clone()))
    }

    ///returns whether the expression is a contingency (sometimes true, sometimes false). Very expensive function.
    pub fn is_contingency(&self) -> bool{
        let mut vars: HashMap<String, bool> = self.vars.iter().map(|(n, _)| (n.to_owned(), false)).collect();
        let mut can_be_false = false;
        let mut can_be_true = false;

        'outer: loop{
            if self.evaluate_with_vars(&vars).unwrap(){
                can_be_true = true;
            }else{
                can_be_false = true;
            }

            if can_be_false && can_be_true{
                return true;
            }

            for (_, b) in vars.iter_mut(){
                *b = !*b;
                if *b{
                    continue 'outer;
                }
            }

            break;
        }

        false
    }

    ///returns whether the expression is contingent with the auxiliary expression. Very expensive function.
    pub fn is_contingency_with(&self, aux: &ExpressionTree) -> bool{
        Self::is_contingency(&(self.clone() & aux.clone()))
    }

    /// Negates the expression tree; returns a mutable reference.
    pub fn deny(&mut self) -> &mut Self{
        self.root.deny();
        match self.value.get_mut(){
            Some(v) => *v = !*v,
            None => (),
        };
        self
    }

    /// Applies demorgan's law to the expression tree if its main connective is
    /// a conjunction or a disjunction; returns a mutable reference. 
    /// 
    /// Otherwise, does nothing and returns `None`.
    pub fn demorgans(&mut self) -> Option<&mut Self>{
        match self.root.demorgans(){
            Some(_) => Some(self),
            None => None,
        }
    }

    /// Performs the logical rule of implication on an expression tree
    /// if its main connective is a conditional operator
    /// or a disjunction operator; returns a mut reference.
    /// 
    /// Otherwise, does nothing and returns None.. 
    pub fn implication(&mut self) -> Option<&mut Self>{
        match self.root.implication(){
            Some(_) => Some(self),
            None => None,
        }
    }

    /// Performs the logical rule of Negated Conditional on an expression tree if its
    /// main connective a conditional or a conjuction; returns a mut reference. 
    /// 
    /// Otherwise does nothing and returns `None`.
    pub fn ncon(&mut self) -> Option<&mut Self>{
        match self.root.ncon(){
            Some(_) => Some(self),
            None => None,
        }
    }

    /// Performs the logical rule of Material Equivalence on an expression tree
    /// if its main connective is a biconditional or a conjunction of conditionals; returns a mut reference. 
    /// Otherwise, does nothing and returns `None`.
    pub fn mat_eq(&mut self) -> Option<&mut Self>{
        match self.root.mat_eq(){
            Some(_) => Some(self),
            None => None,
        }
    }

    /// Performs the logical rule of Material Equivalence on an expression tree
    /// and turns it monotonous if its main connective is a biconditional; returns a mut reference. 
    /// Otherwise, does nothing and returns `None`.
    /// 
    /// Also if operator is denied, consumes the denial
    /// and handles it accordingly.
    pub fn mat_eq_mono(&mut self) -> Option<&mut Self>{
        match self.root.mat_eq_mono(){
            Some(_) => Some(self),
            None => None,
        }
    }
}

impl Default for ExpressionTree{
    /// Default value is just a constant false node.
    fn default() -> Self {
        Self { 
            vars: HashMap::new(), 
            root: Node::Constant(Negation::default(), false),
            value: Cell::new(None),
        }
    }
}

impl From<Node> for ExpressionTree{
    fn from(n: Node) -> Self{
        Self { 
            vars: Self::create_vars(&n, HashMap::new()), 
            root: n,
            value: Cell::new(None),
        }
    }
}

impl From<&str> for ExpressionTree{
    fn from(value: &str) -> Self {
        ExpressionTree::new(value).unwrap()
    }
}

impl From<String> for ExpressionTree{
    fn from(value: String) -> Self {
        ExpressionTree::new(&value).unwrap()
    }
}

///produces the denial of the expression tree.
impl std::ops::Not for ExpressionTree{
    type Output = ExpressionTree;

    fn not(self) -> Self::Output {
        self.not()
    }
}

///produces the expression lhs v rhs
impl std::ops::BitOr for ExpressionTree{
    type Output = ExpressionTree;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.or(rhs)
    }
}

///produces the expression lhs & rhs
impl std::ops::BitAnd for ExpressionTree{
    type Output = ExpressionTree;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.and(rhs)
    }
}

///produces the expression ~(lhs <-> rhs)
impl std::ops::BitXor for ExpressionTree{
    type Output = ExpressionTree;
    
    fn bitxor(self, rhs: Self) -> Self::Output {
        self.bicon(rhs).not()
    }
}

///produces the expression lhs -> rhs
impl std::ops::Shr for ExpressionTree{
    type Output = ExpressionTree;

    fn shr(self, rhs: Self) -> Self::Output {
        self.con(rhs)
    }
}

///produces the expression rhs -> lhs
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

impl std::ops::ShrAssign for ExpressionTree{
    fn shr_assign(&mut self, rhs: Self) {
        *self = self.clone().con(rhs);
    }
}

impl std::ops::ShlAssign for ExpressionTree{
    fn shl_assign(&mut self, rhs: Self) {
        *self = rhs.con(self.clone());
    }
} 