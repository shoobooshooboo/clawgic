pub mod node;
pub mod expression_var;
pub mod universe;
mod token;

use token::Token;
use node::Node;
use node::operator::Operator;
use std::cell::Cell;
use std::collections::HashMap;

use crate::expression_tree::node::negation::Negation;
use crate::expression_tree::universe::Universe;
use crate::operator_notation::OperatorNotation;
use crate::{ClawgicError, utils};
use crate::prelude::{Predicate, Sentence};

/// Expression tree for logical expressions in SL.
#[derive(Debug, Clone)]
pub struct ExpressionTree{
    /// All the unique variables in the tree and their current value.
    uni: Universe,
    /// Root node of the expression Tree.
    root: Node,
    /// Cached previous result of `evaluate()`
    value: Cell<Option<bool>>
}

impl ExpressionTree{
    ///returns a tree that is just a true node
    #[allow(non_snake_case)]
    pub fn TRUE() -> Self{
        Self { uni: Universe::new(), root: Node::Constant(Negation::default(), true), value: Cell::new(Some(true)) }
    }

    /// Returns a tree that is just a false node
    #[allow(non_snake_case)]
    pub fn FALSE() -> Self{
        Self { uni: Universe::new(), root: Node::Constant(Negation::default(), false), value: Cell::new(Some(false)) }
        
    }

    // Constructs a tree with a single constant node of the given value.
    pub fn constant(b: bool) -> Self{
        Self { uni: Universe::new(), root: Node::Constant(Negation::default(), b), value: Cell::new(Some(b)) }
    }

    /// Constructs a new expression tree given a string representation of an infix logical expression.
    pub fn new(expression: &str) -> Result<Self, ClawgicError>{
        let shells = &mut Self::shunting_yard(Self::tokenize_expression(expression, &OperatorNotation::default())?)?;
        let root = Self::construct_tree(shells)?;
        let vars = Self::create_uni(&root, Universe::new());
        if !shells.is_empty(){
            return Err(ClawgicError::NotEnoughOperators);
        }
        Ok(Self{
            uni: vars,
            root,
            value: Cell::new(None),
        })
    }

    /// Constructs a new expression tree given a string representation of an infix logical expression and an 
    /// `OperatorNotation` detailing the accepted operators.
    pub fn new_with_notation(expression: &str, notation: &OperatorNotation) -> Result<Self, ClawgicError>{
        let shells = &mut Self::shunting_yard(Self::tokenize_expression(expression, notation)?)?;
        let root = Self::construct_tree(shells)?;
        let vars = Self::create_uni(&root, Universe::new());
        if !shells.is_empty(){
            return Err(ClawgicError::NotEnoughOperators);
        }
        Ok(Self{
            uni: vars,
            root,
            value: Cell::new(None),
        })
    }

    /// Tokenizes a string representation of an infix logical expression and produces a Vec of `Shell`'s
    fn tokenize_expression(expression: &str, notation: &OperatorNotation) -> Result<Vec<Token>, ClawgicError>{
        //using chars enforces exactly one pass.
        let mut chars = expression.chars().filter(|c| !c.is_whitespace());
        let mut result = Vec::new();
        let mut c = match chars.next(){
            Some(next_char) => next_char,
            None => return Err(ClawgicError::EmptyExpression)
        };

        let mut more_to_parse = true;

        while more_to_parse{
            //handle predicates
            if c.is_alphanumeric() && c != 'v'{
                let mut substring = String::new();
                while c.is_uppercase(){
                    substring.push(c);
                    c = match chars.next(){
                        Some(next_char) => next_char,
                        None => {more_to_parse = false; break;},
                    };
                }

                if substring.is_empty(){
                    return Err(ClawgicError::InvalidPredicateName(c.to_string()));
                }

                if substring == "TRUE"{
                    result.push(Token::Constant(Negation::default(), true));
                }else if substring == "FALSE"{
                    result.push(Token::Constant(Negation::default(), false));
                }else if substring.len() > 1{
                    return Err(ClawgicError::InvalidPredicateName(substring));
                }else{
                    while c.is_numeric(){
                        substring.push(c);
                        c = match chars.next(){
                            Some(next_char) => next_char,
                            None => {more_to_parse = false; break;},
                        };
                    }
                    let pred_name = substring.clone();
                    let mut variables = Vec::new();
                    if c == '('{
                        c = match chars.next(){
                            Some(next_char) => next_char,
                            None => return Err(ClawgicError::InvalidExpression),
                        };

                        while c != ')'{
                            substring.clear();
                            while c != ','{
                                substring.push(c);
                                c = match chars.next(){
                                    Some(next_char) => next_char,
                                    None => {more_to_parse = false; break;},
                                };
                            }

                            if !utils::is_valid_var_name(&substring){
                                return Err(ClawgicError::InvalidVariableName(substring));
                            }

                            variables.push(substring.clone());
                            c = match chars.next(){
                                Some(next_char) => next_char,
                                None => {more_to_parse = false; break;},
                            };
                        }
                    }
                    result.push(Token::Sentence(Negation::default(), Predicate::new(&pred_name, variables.len()).unwrap(), variables));
                }
            } else if !notation.get_potential_operators(&c.to_string()).is_empty() {
                let mut substring = String::new();
                substring.push(c);
                let op: Operator;
                while !notation.get_potential_operators(&substring).is_empty(){
                    c = match chars.next(){
                        Some(next_char) => next_char,
                        None => {substring.push(':'); more_to_parse = false; break;},
                    };
                    substring.push(c);
                }
                substring.pop();
                op = match notation.get_operator(&substring){
                    Some(o) => o,
                    None => return Err(ClawgicError::UnknownSymbol(substring)),
                };
                if op.is_not(){
                    result.push(Token::Tilde(Negation::new(1)));
                }else{
                    result.push(Token::Operator(Negation::default(), op));
                }
            }else if c == '('{
                result.push(Token::OpenParenthesis);

                c = match chars.next(){
                    Some(next_char) => next_char,
                    None => break,
                };
            }else if c == ')'{
                result.push(Token::ClosedParenthesis);

                c = match chars.next(){
                    Some(next_char) => next_char,
                    None => break,
                };
            }else{
                return Err(ClawgicError::UnknownSymbol(c.to_string()));
            }
        }

        Ok(result)
    }

    /// Takes a tokenized version of an infix logical expression and converts to postfix.
    fn shunting_yard(expression: Vec<Token>) -> Result<Vec<Token>, ClawgicError>{

        let mut postfix = Vec::new();
        let mut operators = Vec::new();

        for token in expression{
            match token{
                Token::Tilde(negation) => operators.push(Token::Tilde(negation)),
                Token::OpenParenthesis => operators.push(Token::OpenParenthesis),
                Token::Constant(mut negation, value) => {
                    while operators.last().is_some_and(|op| op.is_tilde()){
                        negation.negate();
                        operators.pop();
                    }
                    postfix.push(Token::Constant(negation, value));
                },
                Token::Sentence(mut negation, predicate, vars) => {
                    while operators.last().is_some_and(|op| op.is_tilde()){
                        negation.negate();
                        operators.pop();
                    }
                    postfix.push(Token::Sentence(negation, predicate, vars));
                },
                Token::Operator(mut negation, op) => {
                    if !operators.is_empty(){
                        while let Some(Token::Operator(_, o)) = operators.last(){
                            if o.precedence() < op.precedence(){
                                break;
                            }else if o.precedence() == op.precedence(){
                                return Err(ClawgicError::AmbiguousExpression);
                            }
                            postfix.push(operators.pop().unwrap());
                        }
                        while operators.last().is_some_and(|op| op.is_tilde()){
                            negation.negate();
                            operators.pop();
                        }
                    }
                    operators.push(Token::Operator(negation, op));
                },
                Token::ClosedParenthesis => {
                    while operators.last().is_some_and(|op| !op.is_open_parentheses()){
                        postfix.push(operators.pop().unwrap());
                    }
                    if operators.pop().is_none_or(|x| !x.is_open_parentheses()){
                        return Err(ClawgicError::InvalidParentheses);
                    }
                    if operators.last().is_some_and(|t| t.is_tilde()){
                        match postfix.pop().unwrap(){
                            Token::Constant(mut negation, val) => {
                                while operators.last().is_some_and(|op| op.is_tilde()){
                                    negation.negate();
                                    operators.pop();
                                }

                                postfix.push(Token::Constant(negation, val))
                            },
                            Token::Operator(mut negation, op) => {
                                while operators.last().is_some_and(|op| op.is_tilde()){
                                    negation.negate();
                                    operators.pop();
                                }

                                postfix.push(Token::Operator(negation, op));
                            },
                            Token::Sentence(mut negation, pred, vars) => {
                                while operators.last().is_some_and(|op| op.is_tilde()){
                                    negation.negate();
                                    operators.pop();
                                }

                                postfix.push(Token::Sentence(negation, pred, vars))
                            },
                            Token::ClosedParenthesis | Token::OpenParenthesis | Token::Tilde(_) => panic!("this should be impossible"),

                        }
                    }
                }
            }
        }

        while !operators.is_empty(){
            postfix.push(operators.pop().unwrap());
        }

        Ok(postfix)
    }

    /// Takes a Vec of `Shell`s, constructs a subtree of `Node`s and returns the root node of that subtree. 
    fn construct_tree(shells: &mut Vec<Token>) -> Result<Node, ClawgicError>{
        let node = match shells.pop(){
            Some(s) => {
                match s {
                    Token::Operator(denied, op) => {
                        let right = Self::construct_tree(shells)?;
                        let left = Self::construct_tree(shells)?;
                        Node::Operator { neg: denied, op, left: Box::new(left), right: Box::new(right) }
                    },
                    Token::Sentence(denied, predicate, vars) => Node::Sentence { neg: denied, sen: predicate.inst(&vars)?},
                    Token::Constant(neg, value) => Node::Constant(neg, value),
                    Token::OpenParenthesis | Token::ClosedParenthesis => return Err(ClawgicError::InvalidParentheses),
                    Token::Tilde(_) => return Err(ClawgicError::InvalidExpression),
                }
            },
            None => return Err(ClawgicError::TooManyOperators),
        };

        Ok(node)
    }

    //OPTIMIZATION: create vars at the same time as construct_tree to avoid excessive work.
    /// Takes a `Node` and the `Universe` and does a depth-first-search for every variable, inserting them into the map as they are found.
    fn create_uni(node: & Node, mut uni: Universe) -> Universe{
        let vars = match node{
            Node::Operator { neg: _, op: _, left, right } =>{
                let vars = Self::create_uni(left, uni);
                Self::create_uni(right, vars)
            },
            Node::Constant(..) => uni,
            Node::Sentence { neg: _, sen} => {
                uni.insert_predicate(sen.predicate().clone());
                uni
            },
        };

        vars
    }

    /// Sets the truth value of the given sentence.
    pub fn set_tval(&mut self, sentence: &Sentence, value: bool){
        if let Some(tval) = self.uni.get_tval_mut(sentence){
            self.value.replace(None);
            *tval = value;
        }
    }

    /// Updates the values of multiple .
    pub fn set_tvals(&mut self, sentences: &HashMap<Sentence, bool>){
        for (sen, b) in sentences.iter(){
            if let Some(tval) = self.uni.get_tval_mut(sen){
                *tval = *b;
            }
            self.value.replace(None);
        }
    }

    /// Replaces all instances of var in the tree with new_expression. Adds all variables from new_expression to self as they are.
    pub fn replace_sentence(&mut self, sentence: &Sentence, new_expression: &ExpressionTree) -> &mut Self{
        if self.uni.contains_sentence(sentence){
            self.uni.remove_sentence(sentence);
            self.uni.add_universe(new_expression.uni.clone());
            Self::replace_sentence_rec(&mut self.root, sentence, new_expression);
            self.value.replace(None);
        }

        self
    }

    /// Recursive helper function for `ExpressionTree::replace_variable()`
    fn replace_sentence_rec(cur_node: &mut Node, sentence: &Sentence, new_expression: &ExpressionTree){
        if cur_node.is_sentence(){
            let Node::Sentence { neg: denied, sen} = cur_node.clone()
                else{panic!("this should never happen (in replace_variable_rec())")};
            if *sentence == sen{
                *cur_node = new_expression.root.clone();
                if denied.is_denied(){
                    cur_node.deny();
                }
            }
        }else if cur_node.is_operator(){
            let Node::Operator { neg: _, op: _, left, right } = cur_node 
                else{panic!("this should never happen (in replace_variable_rec())")};
            Self::replace_sentence_rec(left, sentence, new_expression);
            Self::replace_sentence_rec(right, sentence, new_expression);
        }
    }

    /// Replaces all instances of each sentence in the tree the correlating expression new_expression. Adds all variables from new_expression to self as they are.
    pub fn replace_sentences(&mut self, sentences: &HashMap<Sentence, &ExpressionTree>) -> &mut Self{
        // //gotta remove all vars before adding the new ones.
        // let mut something_in_vars = false;
        // let mut was_in_vars = Vec::with_capacity(sentences.len());
        // for (sen, _) in sentences.iter(){
        //     if self.uni.remove_sentence(sen){
        //         was_in_vars.push(true);
        //         something_in_vars = true;
        //     }else{
        //         was_in_vars.push(false);
        //     }
        // }
        // for (i, (_, new_expression)) in sentences.iter().enumerate(){
        //     if was_in_vars[i]{
        //         for (name, val) in new_expression.uni.all_sentences().iter(){
        //             if !self.uni.contains_key(name){
        //                 self.uni.insert(name.clone(), val.clone());
        //             }
        //         }
        //     }
        // }
        // if something_in_vars{
        Self::replace_sentences_rec(&mut self.root, sentences);
        self.value.replace(None);
        self.uni = Self::create_uni(&self.root, Universe::new());
        // }

        self
    }

    /// Recursive helper function for `ExpressionTree::replace_variable()`
    fn replace_sentences_rec(cur_node: &mut Node, sentences: &HashMap<Sentence, &ExpressionTree>){
        if cur_node.is_sentence(){
            let Node::Sentence { neg: denied, sen} = cur_node.clone()
                else{panic!("this should never happen (in replace_variable_rec())")};
            if let Some(new_expression) = sentences.get(&sen){
                *cur_node = new_expression.root.clone();
                if denied.is_denied(){
                    cur_node.deny();
                }
            }
        }else if cur_node.is_operator(){
            let Node::Operator { neg: _, op: _, left, right } = cur_node 
                else{panic!("this should never happen (in replace_variable_rec())")};
            Self::replace_sentences_rec(left, sentences);
            Self::replace_sentences_rec(right, sentences);
        }
    }

    ///replaces all instances of old expression in the tree with new expression.
    pub fn replace_expression(&mut self, old: &ExpressionTree, new: &ExpressionTree){
        Self::replace_expression_rec(&mut self.root, old, new);
        self.uni = Self::create_uni(&self.root, Universe::new());
    }

    fn replace_expression_rec(cur_node: &mut Node, old: &ExpressionTree, new: &ExpressionTree){
        if *cur_node == old.root || (cur_node.is_constant() && old.root.is_constant()){
            *cur_node = new.root.clone();
            return;
        }
        if cur_node.is_sentence() && old.root.is_sentence(){
            let Node::Sentence { neg: cur_denied, sen: cur_sen } = cur_node 
                else {panic!("this shouldn't be possible (replace_expression_rec)")};
            let Node::Sentence { neg: old_denied, sen: old_sen } = &old.root
                else {panic!("this shouldn't be possible (replace_expression_rec)")};
            if old_sen == cur_sen{
                let deny = *cur_denied != *old_denied;
                *cur_node = new.root.clone();
                if deny{
                    cur_node.deny();
                }
            }
        }else if cur_node.is_operator() && old.root.is_operator(){
            let Node::Operator { neg: cur_denied, op: cur_op, left: cur_left, right: cur_right } = cur_node
                else {panic!("this shouldn't be possible (replace_expression_rec)")};
            let Node::Operator { neg: old_denied, op: old_op, left: old_left, right: old_right } = &old.root
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
    pub fn evaluate(&self) -> Result<bool, ClawgicError>{
        match self.value.get(){
            Some(v) => Ok(v),
            None => {
                let result = self.root.evaluate(&self.uni);
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
    pub fn evaluate_with_uni(&self, uni: &Universe) -> Result<bool, ClawgicError>{
        self.root.evaluate(uni)
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
            Node::Operator { neg: _, op: _, left, right } => {
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
            Node::Operator { neg: denied, op: _, left, right } => {
                let mut op = node.print(notation);
                if denied.is_denied(){
                    //TODO!: make this less ugly
                    infix.push_str(&notation[Operator::NOT].repeat(denied.count() as usize));
                    
                    op = op.chars().skip(notation[Operator::NOT].chars().count() * denied.count() as usize).collect();
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
    pub fn universe(&self) -> &Universe{
        &self.uni
    }

    /// Converts all operators in the tree into conjunctions and disjunctions with no leading denials.
    pub fn monotenize(&mut self){
        Self::monotenize_rec(&mut self.root);
    }

    //OPTIMIZE: make monotenization work from the bottom up (monotenization expands the tree)
    /// Recursive helper function for `ExpressionTree::monotenize()`.
    fn monotenize_rec(node: &mut Node){
        match &*node{
            Node::Operator { neg: denied, op, left: _, right: _ } => {
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
            Node::Operator { neg: _, op: _, left, right } => {
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

    /// Returns a reference to the tree's root node.
    pub fn node(&self) -> &Node{
        &self.root
    }

    ///consumes two trees and returns a tree in the form of self & second.
    pub fn and(mut self, second: Self) -> Self{
        self.uni.add_universe(second.uni.clone());

        Self { 
            uni: self.uni, 
            root: Node::Operator{neg: Negation::default(), op: node::operator::Operator::AND, left: Box::new(self.root), right: Box::new(second.root)},
            value: Cell::new(None),
        }
    }

    ///consumes two trees and returns a tree in the form of self v (wedge) second.
    pub fn or(mut self, second: Self) -> Self{
                self.uni.add_universe(second.uni.clone());


        Self { 
            uni: self.uni, 
            root: Node::Operator{neg: Negation::default(), op: node::operator::Operator::OR, left: Box::new(self.root), right: Box::new(second.root)},
            value: Cell::new(None),
        }
    }

    ///consumes two trees and returns a tree in the form of self->consequent.
    pub fn con(mut self, consequent: Self) -> Self{
        self.uni.add_universe(consequent.uni.clone());


        Self { 
            uni: self.uni, 
            root: Node::Operator{neg: Negation::default(), op: node::operator::Operator::CON, left: Box::new(self.root), right: Box::new(consequent.root)},
            value: Cell::new(None),
        }
    }

    ///consumes two trees and returns a tree in the form of self->second.
    pub fn bicon(mut self: Self, second: Self) -> Self{
        self.uni.add_universe(second.uni.clone());


        Self { 
            uni: self.uni, 
            root: Node::Operator{neg: Negation::default(), op: node::operator::Operator::BICON, left: Box::new(self.root), right: Box::new(second.root)},
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
        self.root == other.root
    }

    ///checks if the two expressions are syntactically the same (one can be transformed into the other with primitive logic rules). Very expensive function.
    pub fn syn_eq(&self, other: &Self) -> bool{
        if self.uni == other.uni{
            return false;
        }
        //check for logical equivalence
        self.log_eq(other)
    }

    ///checks if the expression is satisfiable. Very expensive function.
    pub fn is_satisfiable(&self) -> bool{
        todo!()
        // let mut vars: HashMap<String, bool> = self.uni.iter().map(|(n, _)| (n.to_owned(), false)).collect();

        // 'outer: loop{
        //     if self.evaluate_with_vars(&vars).unwrap(){
        //         return true;
        //     }

        //     for (_, b) in vars.iter_mut(){
        //         *b = !*b;
        //         if *b{
        //             continue 'outer;
        //         }
        //     }

        //     break;
        // }

        // false
    }

    ///checks if the expression is satisfiable given the auxiliary expression. Very expensive function.
    pub fn is_satisfiable_with(&self, aux: &ExpressionTree) -> bool{
        Self::is_satisfiable(&(self.clone() & aux.clone()))
    }

    ///returns a set of variables that satisfies the expression if one exists. Very expensive function.
    pub fn satisfy_one(&self) -> Option<HashMap<Sentence, bool>>{
        todo!();
        // let mut vars: HashMap<String, bool> = self.uni.iter().map(|(n, _)| (n.to_owned(), false)).collect();

        // 'outer: loop{
        //     if self.evaluate_with_vars(&vars).unwrap(){
        //         return Some(vars);
        //     }

        //     for (_, b) in vars.iter_mut(){
        //         *b = !*b;
        //         if *b{
        //             continue 'outer;
        //         }
        //     }

        //     break;
        // }

        // None
    }

    ///returns a set of variables that satisfies the expression and the auxiliary expression if one exists. Very expensive function.
    pub fn satisfy_one_with(&self, aux: &ExpressionTree) -> Option<HashMap<Sentence, bool>>{
        Self::satisfy_one(&(self.clone() & aux.clone()))
    }

    ///returns a vector of all sets of variables that satisfy the expression. Extremely expensive function.
    pub fn satisfy_all(&self) -> Vec<HashMap<Sentence, bool>>{
        todo!()
        // let mut vars: HashMap<String, bool> = self.uni.iter().map(|(n, _)| (n.to_owned(), false)).collect();
        // let mut maps = Vec::new();

        // 'outer: loop{
        //     if self.evaluate_with_vars(&vars).unwrap(){
        //         maps.push(vars.clone());
        //     }

        //     for (_, b) in vars.iter_mut(){
        //         *b = !*b;
        //         if *b{
        //             continue 'outer;
        //         }
        //     }

        //     break;
        // }

        // maps
    }

    ///returns a vector of all sets of variables that satisfy the expression and the auxiliary expression. Extremely expensive function.
    pub fn satisfy_all_with(&self, aux: &ExpressionTree) -> Vec<HashMap<Sentence, bool>>{
        Self::satisfy_all(&(self.clone() & aux.clone()))
    }

    ///returns the total number of ways the expression can be satisfied. very expensive function.
    pub fn satisfy_count(&self) -> Vec<u128>{
        todo!();
        // let mut vars: HashMap<String, bool> = self.uni.iter().map(|(n, _)| (n.to_owned(), false)).collect();
        // let len = 1 + vars.len() / 128;
        // let mut count = vec![0 ; len];

        // 'outer: loop{
        //     if self.evaluate_with_vars(&vars).unwrap(){
        //         for c in count.iter_mut(){
        //             if *c != std::u128::MAX{
        //                 *c += 1;
        //                 break;
        //             }
        //             *c = 0;
        //         }
        //     }

        //     for (_, b) in vars.iter_mut(){
        //         *b = !*b;
        //         if *b{
        //             continue 'outer;
        //         }
        //     }

        //     break;
        // }

        // count
    }

    ///returns the total number if ways the expression can be satisfied with the auxiliary expression. very expensive function.
    pub fn satisfy_count_with(&self, aux: &ExpressionTree) -> Vec<u128>{
        Self::satisfy_count(&(self.clone() & aux.clone()))        
    }

    ///returns whether the expression is a tautology (always true). Very expensive function.
    pub fn is_tautology(&self) -> bool{
        todo!();
        // let mut vars: HashMap<String, bool> = self.uni.iter().map(|(n, _)| (n.to_owned(), false)).collect();

        // 'outer: loop{
        //     if !self.evaluate_with_vars(&vars).unwrap(){
        //         return false;
        //     }

        //     for (_, b) in vars.iter_mut(){
        //         *b = !*b;
        //         if *b{
        //             continue 'outer;
        //         }
        //     }

        //     break;
        // }

        // true
    }

    ///returns whether the expression is tautological with the auxiliary expression. Very expensive function.
    pub fn is_tautology_with(&self, aux: &ExpressionTree) -> bool{
        Self::is_inconsistency(&(self.clone() & aux.clone()))
    }

    ///returns whether the expression is an inconsistency (always false). Very expensive function.
    pub fn is_inconsistency(&self) -> bool{
        todo!();
        // let mut vars: HashMap<String, bool> = self.uni.iter().map(|(n, _)| (n.to_owned(), false)).collect();

        // 'outer: loop{
        //     if self.evaluate_with_vars(&vars).unwrap(){
        //         return false;
        //     }

        //     for (_, b) in vars.iter_mut(){
        //         *b = !*b;
        //         if *b{
        //             continue 'outer;
        //         }
        //     }

        //     break;
        // }

        // true
    }

    ///returns whether the expression is inconsistent with the auxiliary expression. Very expensive function.
    pub fn is_inconsistency_with(&self, aux: &ExpressionTree) -> bool{
        Self::is_inconsistency(&(self.clone() & aux.clone()))
    }

    ///returns whether the expression is a contingency (sometimes true, sometimes false). Very expensive function.
    pub fn is_contingency(&self) -> bool{
        todo!();
        // let mut vars: HashMap<String, bool> = self.uni.iter().map(|(n, _)| (n.to_owned(), false)).collect();
        // let mut can_be_false = false;
        // let mut can_be_true = false;

        // 'outer: loop{
        //     if self.evaluate_with_vars(&vars).unwrap(){
        //         can_be_true = true;
        //     }else{
        //         can_be_false = true;
        //     }

        //     if can_be_false && can_be_true{
        //         return true;
        //     }

        //     for (_, b) in vars.iter_mut(){
        //         *b = !*b;
        //         if *b{
        //             continue 'outer;
        //         }
        //     }

        //     break;
        // }

        // false
    }

    ///returns whether the expression is contingent with the auxiliary expression. Very expensive function.
    pub fn is_contingency_with(&self, aux: &ExpressionTree) -> bool{
        Self::is_contingency(&(self.clone() & aux.clone()))
    }

    /// If the tree has at least one leading tilde,
    /// remove one. otherwise, add one. returns a mutable reference.
    pub fn deny(&mut self) -> &mut Self{
        self.root.deny();
        match self.value.get_mut(){
            Some(v) => *v = !*v,
            None => (),
        };
        self
    }

    /// If the tree has at least 2 leading tildes,
    /// remove two. otherwise, add two. returns a mutable reference.
    pub fn double_deny(&mut self) -> &mut Self{
        self.root.double_deny();
        self
    }

    /// Adds a leading tilde; returns a mutable reference.
    pub fn negate(&mut self) -> &mut Self{
        self.root.negate();
        match self.value.get_mut(){
            Some(v) => *v = !*v,
            None => (),
        };
        self
    }

    /// Adds two leading tildes; returns a mutable reference.
    pub fn double_negate(&mut self) -> &mut Self{
        self.root.double_negate();
        self
    }

    /// Reduces the number of leading tildes to 0 or 1,
    /// retaining truth value; returns a mutable refernce.
    pub fn reduce_negation(&mut self) -> &mut Self{
        self.root.reduce_negation();
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

    /// Applies demorgan's law to the expression tree if its main connective is
    /// a conjunction or a disjunction; returns a mutable reference. 
    /// 
    /// Otherwise, does nothing and returns `None`.
    /// 
    /// Opts for negation over denial.
    pub fn demorgans_neg(&mut self) -> Option<&mut Self>{
        match self.root.demorgans_neg(){
            Some(_) => Some(self),
            None => None,
        }
    }

    /// Applies transposition if the main connective (barring tildes)
    /// is a conditional and then returns a mutable reference.
    /// 
    /// otherwise, does nothing and returns `None`.
    pub fn transposition(&mut self) -> Option<&mut Self>{
        match self.root.transposition(){
            Some(_) => Some(self),
            None => None,
        }
    }

    /// Applies transposition if the main connective (barring tildes)
    /// is a conditional and then returns a mutable reference.
    /// 
    /// otherwise, does nothing and returns `None`.
    /// 
    /// Opts for negation over denial.
    pub fn transposition_neg(&mut self) -> Option<&mut Self>{
        match self.root.transposition_neg(){
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

    /// Performs the logical rule of implication on an expression tree
    /// if its main connective is a conditional operator
    /// or a disjunction operator; returns a mut reference.
    /// 
    /// Otherwise, does nothing and returns None.. 
    /// 
    /// Opts for negation over denial.
    pub fn implication_neg(&mut self) -> Option<&mut Self>{
        match self.root.implication_neg(){
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

    /// Performs the logical rule of Negated Conditional on an expression tree if its
    /// main connective a conditional or a conjuction; returns a mut reference. 
    /// 
    /// Otherwise does nothing and returns `None`.
    /// 
    /// Opts for negation over denial.
    pub fn ncon_neg(&mut self) -> Option<&mut Self>{
        match self.root.ncon_neg(){
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

    /// Gets the main connective.
    pub fn main_connective(&self) -> Option<Operator>{
        match self.root{
            Node::Operator { neg, op, ..} => {
                if neg.count() > 0{
                    Some(Operator::NOT)
                }else{
                    Some(op)
                }
            },
            Node::Sentence { neg, .. } => {
                if neg.count() > 0{
                    Some(Operator::NOT)
                }else{
                    None
                }
            },
            Node::Constant(neg, ..) => {
                if neg.count() > 0{
                    Some(Operator::NOT)
                }else{
                    None
                }
            }
        }
    }

    /// Gets the main connective (ignoring tildes).
    pub fn main_conn_non_tilde(&self) -> Option<Operator>{
        match self.root{
            Node::Operator { neg, op, ..} => {
                if neg.count() > 0{
                    None
                }else{
                    Some(op)
                }
            },
           _ => None
        }
    }
}

impl Default for ExpressionTree{
    /// Default value is just a constant false node.
    fn default() -> Self {
        Self { 
            uni: Universe::new(), 
            root: Node::Constant(Negation::default(), false),
            value: Cell::new(None),
        }
    }
}

impl From<Node> for ExpressionTree{
    fn from(n: Node) -> Self{
        Self { 
            uni: Self::create_uni(&n, Universe::new()), 
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

impl From<Sentence> for ExpressionTree{
    fn from(value: Sentence) -> Self {
        value.expr()
    }
}

impl From<&Sentence> for ExpressionTree{
    fn from(value: &Sentence) -> Self {
        value.expr()
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