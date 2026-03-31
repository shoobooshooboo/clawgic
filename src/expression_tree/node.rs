pub mod operator;
pub mod negation;
pub mod sentence;

use std::{collections::HashMap, mem::swap};

use operator::Operator;
use crate::{expression_tree::{ClawgicError, node::negation::Negation, universe::Universe}, operator_notation::OperatorNotation, prelude::{ExpressionVar, Sentence}};

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
        /// preceding negations
        neg: Negation,
        /// the type of operator. (exclusively a binary operator)
        op: Operator,
        /// left operand.
        left: Box<Node>,
        /// right operand.
        right: Box<Node>,
    },
    /// Quantifier node.
    Quantifier{
        /// preceding negations
        neg: Negation,
        /// the type of operator (strictly universal or existential).
        op: Operator,
        /// variables bound by the quantifier.
        vars: Vec<ExpressionVar>,
        /// subexpression contained within quantifier.
        subexpr: Box<Node>,
    },
    /// Sentence node.
    Sentence{
        /// preceding negations
        neg: Negation,
        /// The actual sentence
        sen: Sentence,
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
    pub fn is_sentence(&self) -> bool{
        match self{
            Self::Sentence{..} => true,
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
    pub fn evaluate(&self, uni: &Universe, varsubs: &mut HashMap<ExpressionVar, ExpressionVar>) -> Result<bool, ClawgicError>{
        match self{
            Self::Operator{op, neg, left, right} => {
                let left_result = left.evaluate(uni, varsubs)?;
                let result = match op.short_circuit(left_result){
                    Some(b) => b,
                    None => op.execute_binary(left_result, right.evaluate(uni, varsubs)?),
                };
                Ok(result != neg.is_denied())
            },
            Self::Quantifier { neg, op, vars, subexpr } => {
                //first, make sure there are no multi-captured vars
                for v in uni.variables().iter(){
                    if vars.contains(v){
                        return Err(ClawgicError::MultiBoundVar(v.name().to_string()))
                    }
                }

                //enumerate all concrete vars in the universe
                let uni_vars: Vec<&ExpressionVar> = uni.variables().iter().collect();
                let max = uni_vars.len();
                //store all captured vars in an easily accessible way
                let mut quant_vars: Vec<(&ExpressionVar, usize)> = vars.iter().map(|v| (v,0)).collect();
                //If the op is universal and reaches the end of the loop without short-circuting, then the result is true.
                //If it's an existential, the default is false.
                let mut result = op.is_uni();
                
                //while all posibilities have not been covered
                while quant_vars.last().unwrap().1 < max{
                    for v in quant_vars.iter(){
                        varsubs.insert(v.0.clone(), uni_vars[v.1].clone());
                    }

                    //short circuit
                    match op.short_circuit(subexpr.evaluate(uni, varsubs)?){
                        Some(b) => {result = b; break;},
                        None => (),
                    }

                    //update quant_vars
                    let mut i = 0;
                    quant_vars[i].1 += 1;
                    while i < quant_vars.len() - 1 && quant_vars[i].1 >= max{
                        quant_vars[i].1 = 0;
                        i += 1;
                    }
                }

                Ok(result != neg.is_denied())
            },
            Self::Sentence { neg, sen} =>{
                let result = match uni.get_tval(sen){
                    Some(b) => {
                        b
                    },
                    None => return Err(ClawgicError::UninitializedSentence(sen.name().to_string())),
                };
                Ok(neg.is_denied() != result)
            },
            Self::Constant(neg, value) => Ok(neg.is_denied() != *value),
        }
    }

    /// If the node has at least one tilde, remove one. otherwise, add one. returns a mutable reference.
    pub fn deny(&mut self) -> &mut Self{
        match self{
            Node::Constant(neg, ..) => neg.deny(),
            Node::Sentence { neg, ..} => neg.deny(),
            Node::Operator { neg, ..} => neg.deny(),
            Node::Quantifier { neg, .. } => neg.deny(),
        };
        self
    }

    /// If the node has more than 1 tilde, remove two. otherwise add two. returns a mutable reference.
    pub fn double_deny(&mut self) -> &mut Self{
        match self{
            Node::Constant(neg, ..) => neg.double_deny(),
            Node::Sentence { neg, ..} => neg.double_deny(),
            Node::Operator { neg, ..} => neg.double_deny(),
            Node::Quantifier { neg, .. } => neg.double_deny(),
        };
        self
    }

    /// Adds a tilde to the node; returns a mutable reference
    pub fn negate(&mut self) -> &mut Self{
        match self{
            Node::Constant(neg, ..) => neg.negate(),
            Node::Sentence { neg, ..} => neg.negate(),
            Node::Operator { neg, ..} => neg.negate(),
            Node::Quantifier { neg, .. } => neg.negate(),
        };
        self
    }

    // Adds two tildes to the node; returns a mutable reference
    pub fn double_negate(&mut self) -> &mut Self{
        match self{
            Node::Constant(neg, ..) => neg.double_negate(),
            Node::Sentence { neg, ..} => neg.double_negate(),
            Node::Operator { neg, ..} => neg.double_negate(),
            Node::Quantifier { neg, .. } => neg.double_negate(),
        };
        self
    }

    /// Reduces the number of tildes to 0 or 1, retaining the truth value of the node; returns a mutable reference.
    pub fn reduce_negation(&mut self) -> &mut Self{
        match self{
            Node::Constant(neg, ..) => neg.reduce(),
            Node::Sentence { neg, ..} => neg.reduce(),
            Node::Operator { neg, ..} => neg.reduce(),
            Node::Quantifier { neg, .. } => neg.reduce(),
        };
        self
    }

    /// Applies demorgan's law to the node if it is
    /// a conjunction or a disjunction; returns a mutable reference. 
    /// 
    /// Otherwise, does nothing and returns `None`.
    pub fn demorgans(&mut self) -> Option<&mut Self>{
        match self{
            Node::Operator { neg: denied, op, left, right } => {
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

    /// Applies demorgan's law to the node if it is
    /// a conjunction or a disjunction; returns a mutable reference.
    /// 
    /// Otherwise, does nothing and returns `None`.
    /// 
    /// Opts for negating instead of denying
    pub fn demorgans_neg(&mut self) -> Option<&mut Self>{
        match self{
            Node::Operator { neg: denied, op, left, right } => {
                if op.is_and() || op.is_or(){
                    *op = if op.is_and() {Operator::OR} else {Operator::AND};
                    denied.negate();
                    left.negate();
                    right.negate();
                    return Some(self);
                }
            },
            _ => (),
        }
        None
    }

    /// Applies transposition if the main connective (barring tildes)
    /// is a conditional and then returns a mutable reference.
    /// 
    /// otherwise, does nothing and returns `None`.
    pub fn transposition(&mut self) -> Option<&mut Self>{
        let Node::Operator { neg: _, op, left, right } = self
            else {return None};
        if op.is_con(){
            left.deny();
            right.deny();
            swap(left, right);
            return Some(self);
        }
        None
    }

    /// Applies transposition if the main connective (barring tildes)
    /// is a conditional and then returns a mutable reference.
    /// 
    /// otherwise, does nothing and returns `None`.
    /// 
    /// Opts for negating instead of denying
    pub fn transposition_neg(&mut self) -> Option<&mut Self>{
        let Node::Operator { neg: _, op, left, right } = self
            else {return None};
        if op.is_con(){
            left.negate();
            right.negate();
            swap(left, right);
            return Some(self);
        }
        None
    }

    /// Performs the logical rule of implication on a node if it is a conditional operator or a disjunction operator; returns a mut reference.
    /// 
    /// Otherwise, does nothing and returns None.. 
    pub fn implication(&mut self) -> Option<&mut Self>{
        match self{
            Node::Operator { neg: _, op, left, right: _ } => {
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

    /// Performs the logical rule of implication on a node if it is a conditional operator or a disjunction operator; returns a mut reference.
    /// 
    /// Otherwise, does nothing and returns None.. 
    /// 
    /// Opts for negating instead of denying
    pub fn implication_neg(&mut self) -> Option<&mut Self>{
        match self{
            Node::Operator { neg: _, op, left, right: _ } => {
                if op.is_con() || op.is_or(){
                    *op =  if op.is_con() {Operator::OR} else {Operator::CON};
                    left.negate();
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
            Node::Operator { neg: denied, op, left: _, right } => {
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

    /// Performs the logical rule of Negated Conditional on a node if it is
    /// a conditional or a conjuction; returns a mut reference. 
    /// 
    /// Otherwise does nothing and returns `None`.
    /// 
    /// Opts for negating instead of denying
    pub fn ncon_neg(&mut self) -> Option<&mut Self>{
        match self{
            Node::Operator { neg: denied, op, left: _, right } => {
                if op.is_con() || op.is_and(){
                    *op = if op.is_con() {Operator::AND} else {Operator::CON};
                    denied.negate();
                    right.negate();
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
            Node::Operator { neg: _, op, left, right } => {
                if op.is_bicon(){
                    *op = Operator::AND;
                    let old_left = left.clone();
                    let old_right = right.clone();
                    *left = Box::new(Node::Operator { neg: Negation::default(), op: Operator::CON, left: old_left.clone(), right: old_right.clone() });
                    *right = Box::new(Node::Operator { neg: Negation::default(), op: Operator::CON, left: old_right, right: old_left });

                    return Some(self);
                }else if op.is_and(){
                    if let Node::Operator{neg: ld, op: l_op, left: ll, right: lr} = *left.clone(){
                        if let Node::Operator { neg: rd, op: r_op, left: rl, right: rr } = *right.clone(){
                            if l_op.is_con() && r_op.is_con() && !ld.is_denied() && !rd.is_denied() && ll == rr && lr == rl{
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
            Node::Operator { neg: denied, op, left, right } => {
                if op.is_bicon(){
                    *op = Operator::OR;
                    let mut old_left = left.clone();
                    let mut old_right = right.clone();
                    if denied.is_denied(){
                        denied.deny();
                        if old_left < old_right{
                            old_left.deny();
                        }
                        else{
                            old_right.deny();
                        }
                    }
                    *left = Box::new(Node::Operator { neg: Negation::default(), op: Operator::AND, left: old_left.clone(), right: old_right.clone() });
                    old_left.deny();
                    old_right.deny();
                    *right = Box::new(Node::Operator { neg: Negation::default(), op: Operator::AND, left: old_left, right: old_right });
                    return Some(self);
                }
            },
            _ => (),
        }
        None
    }

    ///Returns a string representation of the current node based on the given notation.
    pub fn print(&self, notation: &OperatorNotation) -> String{
        let mut s = String::new();
        match self{
            Self::Operator { neg, op, .. } => {
                s.push_str(&notation[Operator::NOT].repeat(neg.count() as usize));
                s.push_str(&notation[*op]);
            }
            Self::Sentence { neg, sen, .. } => {
                s.push_str(&notation[Operator::NOT].repeat(neg.count() as usize));
                s.push_str(&sen.to_string());
            }
            Self::Constant(neg, b) => {
                s.push_str(&notation[Operator::NOT].repeat(neg.count() as usize));
                s +=
                if *b{
                    "TRUE"
                }else{
                    "FALSE"
                };
            }
            Self::Quantifier { neg, op, vars, .. } => {
                s.push_str(&notation[Operator::NOT].repeat(neg.count() as usize));
                s.push_str(&notation[*op]);
                let var_string: String = format!("({:?})", vars).chars().filter(|c| *c != '[' && *c != ']' && *c != '"').collect();
                s.push_str(&var_string);
            }
        }
        s
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