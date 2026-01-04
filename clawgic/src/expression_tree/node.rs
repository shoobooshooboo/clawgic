pub mod operator;
pub mod negation;

use std::{collections::HashMap};

use operator::Operator;
use crate::{expression_tree::{ExpressionTreeError, node::negation::Negation}, operator_notation::OperatorNotation};

/// Nodes for regular logical expression tree.
/// 
/// Can be a binary operator, a variable, or a constant.
/// 
/// Since there is only one unary operator in SL (~ - denial operator), it doesn't
/// get its own enum type and instead is imbedded as a boolean value in operators and variables.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub enum Node{
    /// Binary operator node.
    Operator{
        /// Whether there is an odd number of tildes preceding the operator.
        denied: Negation,
        /// the type of operator.
        op: Operator,
        /// left operand.
        left: Box<Node>,
        /// right operand.
        right: Box<Node>,
    },
    /// Variable node.
    Variable{
        /// Whether there is an odd number of tildes preceding the variable.
        denied: Negation,
        /// Identifier of the variable. Ex: "A", "G", "B3".
        name: String,
    },
    /// Constant node. True or False.
    Constant(Negation, bool),
}

impl Node{
    /// Whether it is an operator node.
    pub fn is_operator(&self) -> bool{
        match self{
            Self::Operator{..} => true,
            _ => false,
        }
    }

    /// Whether it is a variable node.
    pub fn is_variable(&self) -> bool{
        match self{
            Self::Variable{..} => true,
            _ => false,
        }
    }

    /// Whether it is a constant node.
    pub fn is_constant(&self) -> bool{
        match self{
            Self::Constant(..) => true,
            _ => false,
        }
    }

    /// Attempts to get the boolean value of the node.
    /// 
    /// A constant node will just return it's value
    /// 
    /// If a variable node contains a `Some`, it will return that inner value.
    /// Otherwise it will return an ExpressionTreeError.
    /// 
    /// An operator node will attempt to perform its operation on it's left and right operands. 
    /// Will return an ExpressionTreeError if the evaluation of the left or right results in an `Err` value. 
    pub fn evaluate(&self, vars: &HashMap<String, Option<bool>>) -> Result<bool, ExpressionTreeError>{
        match self{
            Self::Operator{op, denied, left, right} => {
                let result = op.execute(left.evaluate(vars)?, right.evaluate(vars)?);
                if denied.tval() {Ok(!result)}
                else {Ok(result)}
            }
            Self::Variable { denied, name} =>{
                let result = match vars.get(name){
                    Some(b) => {
                        if b.is_none(){
                            return Err(ExpressionTreeError::UninitializedVariable(name.clone()))
                        }
                        b.unwrap()
                    },
                    None => return Err(ExpressionTreeError::UninitializedVariable(name.clone())),
                };
                Ok(denied.tval() != result)
            }
            Self::Constant(denied, value) => Ok(denied.tval() != *value),
        }
    }

    ///evaluates the tree with a specific set of variables.
    /// 
    /// If some variable is not present in the map, returns `ExpressionTreeError::UninitualizedVariable`
    pub fn evaluate_with_vars(&self, vars: &HashMap<String, bool>) -> Result<bool, ExpressionTreeError>{
        match self{
            Self::Operator{op, denied, left, right} => {
                let result = op.execute(left.evaluate_with_vars(vars)?, right.evaluate_with_vars(vars)?);
                Ok(result != denied.tval())
            }
            Self::Variable { denied, name} =>{
                let result = match vars.get(name){
                    Some(b) => b.clone(),
                    None => return Err(ExpressionTreeError::UninitializedVariable(name.clone())),
                };
                Ok (result != denied.tval())
            }
            Self::Constant(denied, value) => Ok(denied.tval() != *value),
        }
    }

    /// Negates the node; returns a mutable reference.
    pub fn deny(&mut self) -> &mut Self{
        match self{
            Node::Constant(denied, ..) => denied.deny(),
            Node::Variable { denied, ..} => denied.deny(),
            Node::Operator { denied, ..} => denied.deny(),
        };
        self
    }

    /// Applies demorgan's law to the node if it is
    /// a conjunction or a disjunction; returns a mutable reference. 
    /// 
    /// Otherwise, does nothing and returns `None`.
    pub fn demorgans(&mut self) -> Option<&mut Self>{
        match self{
            Node::Operator { denied, op, left, right } => {
                if op.is_and() || op.is_or(){
                    *op = if op.is_and() {Operator::OR} else {Operator::AND};
                    denied.deny();
                    left.deny();
                    right.deny();
                    return Some(self);
                }
            },
            _ => (),
        }
        None
    }

    /// Performs the logical rule of implication on a node if it is a conditional operator or a disjunction operator; returns a mut reference.
    /// 
    /// Otherwise, does nothing and returns None.. 
    pub fn implication(&mut self) -> Option<&mut Self>{
        match self{
            Node::Operator { denied: _, op, left, right: _ } => {
                if op.is_con() || op.is_or(){
                    *op =  if op.is_con() {Operator::OR} else {Operator::CON};
                    left.deny();
                    return Some(self);
                }
            },
            _ => (),
        }
        None
    }

    /// Performs the logical rule of Negated Conditional on a node if it is
    /// a conditional or a conjuction; returns a mut reference. 
    /// 
    /// Otherwise does nothing and returns `None`.
    pub fn ncon(&mut self) -> Option<&mut Self>{
        match self{
            Node::Operator { denied, op, left: _, right } => {
                if op.is_con() || op.is_and(){
                    *op = if op.is_con() {Operator::AND} else {Operator::CON};
                    denied.deny();
                    right.deny();
                    return Some(self);
                }
            },
            _ => (),
        }
        None
    }

    /// Performs the logical rule of Material Equivalence on a node
    /// if it is a biconditional or a conjunction of conditionals; returns a mut reference. 
    /// Otherwise, does nothing and returns `None`.
    pub fn mat_eq(&mut self) -> Option<&mut Self>{
        match self{
            Node::Operator { denied: _, op, left, right } => {
                if op.is_bicon(){
                    *op = Operator::AND;
                    let old_left = left.clone();
                    let old_right = right.clone();
                    *left = Box::new(Node::Operator { denied: Negation::default(), op: Operator::CON, left: old_left.clone(), right: old_right.clone() });
                    *right = Box::new(Node::Operator { denied: Negation::default(), op: Operator::CON, left: old_right, right: old_left });

                    return Some(self);
                }else if op.is_and(){
                    if let Node::Operator{denied: ld, op: l_op, left: ll, right: lr} = *left.clone(){
                        if let Node::Operator { denied: rd, op: r_op, left: rl, right: rr } = *right.clone(){
                            if l_op.is_con() && r_op.is_con() && !ld.tval() && !rd.tval() && ll == rr && lr == rl{
                                *op = Operator::BICON;
                                *left = ll;
                                *right = lr;
                            }
                        }
                    }
                    return Some(self);
                }
            },
            _ => (),
        }
        None
    }

    /// Performs the logical rule of Material Equivalence on a node
    /// and turns it monotonous if it is a biconditional; returns a mut reference. 
    /// Otherwise, does nothing and returns `None`.
    /// 
    /// Also if operator is denied, consumes the denial
    /// and handles it accordingly.
    pub fn mat_eq_mono(&mut self) -> Option<&mut Self>{
        match self{
            Node::Operator { denied, op, left, right } => {
                if op.is_bicon(){
                    *op = Operator::OR;
                    let mut old_left = left.clone();
                    let mut old_right = right.clone();
                    if denied.tval(){
                        denied.deny();
                        if old_left < old_right{
                            old_left.deny();
                        }
                        else{
                            old_right.deny();
                        }
                    }
                    *left = Box::new(Node::Operator { denied: Negation::default(), op: Operator::AND, left: old_left.clone(), right: old_right.clone() });
                    old_left.deny();
                    old_right.deny();
                    *right = Box::new(Node::Operator { denied: Negation::default(), op: Operator::AND, left: old_left, right: old_right });
                    return Some(self);
                }
            },
            _ => (),
        }
        None
    }

    ///Returns a string representation of the current node based on the given notation.
    pub fn print(&self, notation: &OperatorNotation) -> String{
        match self{
            Self::Operator { denied, op, .. } => {
                let mut s = String::new();
                if denied.tval(){
                    s.push_str(notation.neg());
                }
                match op{
                    Operator::AND => s.push_str(notation.and()),
                    Operator::OR => s.push_str(notation.or()),
                    Operator::CON => s.push_str(notation.con()),
                    Operator::BICON => s.push_str(notation.bicon()),
                }

                s
            }
            Self::Variable { denied, name, .. } => {
                let mut s = String::new();
                if denied.tval(){
                    s.push_str(notation.neg());
                }
                s.push_str(name);
                s
            }
            Self::Constant(denied, b) => {
                let mut s = String::new();
                for _ in 0..denied.count(){
                    s.push_str(notation.neg())
                }
                s + 
                if *b{
                    "TRUE"
                }else{
                    "FALSE"
                }
            }
        }
    }

    ///Returns a string representation of the current node based on `OperationNotation::ascii()`.
    pub fn to_ascii(&self) -> String{
        self.print(&OperatorNotation::ascii())
    }
}

///Returns a string representation of the current node based on `OperationNotation::default()`.
impl ToString for Node{
    fn to_string(&self) -> String {
        self.print(&OperatorNotation::default())
    }
}

impl std::ops::Not for Node{
    type Output = Self;
    fn not(mut self) -> Self::Output {
        self.deny();
        self
    }
}