use std::{collections::HashMap, ops::Index};

use crate::expression_tree::node::operator::Operator;

/// Fake HashMap for OperatorNotation.
struct NotationMap{
    map: [Vec<String> ; 5],
}

impl NotationMap{
    pub fn new(map: HashMap<Operator, (String, Vec<String>)>) -> NotationMap{
        let mut nm = Self { map: [const {Vec::new()} ; 5] };
        for (op, (first, mut rest)) in map{
            rest.insert(0, first);
            nm.map[op as usize] = rest;
        }
        nm
    }
}

impl Index<Operator> for NotationMap{
    type Output = Vec<String>;

    fn index(&self, index: Operator) -> &Self::Output {
        &self.map[index as usize]
    }
}

///Contains a set of symbols for printing `ExpressionTree`s. Used in certain `ExpressionTree` functions to customize expression printing.
pub struct OperatorNotation{
    map: NotationMap,
}

impl OperatorNotation{
    /// Constructs the ascii version of the default `OperatorNotation`.
    /// 
    /// * conjunction &
    /// * disjunction v
    /// * negation ~
    /// * conditional ->
    /// * biconditional <->
    pub fn ascii() -> Self{
        Self { map: NotationMap::new([
            (Operator::NOT, ("~".to_string(), vec!["¬".to_string(), "!".to_string()])),
            (Operator::AND, ("&".to_string(), vec!["^".to_string(), "∧".to_string(), "*".to_string(), "⋅".to_string()])),
            (Operator::OR, ("v".to_string(), vec!["∨".to_string(), "|".to_string(), "+".to_string()])),
            (Operator::CON, ("->".to_string(), vec!["➞".to_string(), ">".to_string(), "-->".to_string()])),
            (Operator::BICON, ("<->".to_string(), vec!["⟷".to_string(), "<>".to_string(), "<-->".to_string()])),
            ].into_iter().collect())
        }
    }

    /// Constructs the `OperatorNotation` based on mathematical notation.
    /// 
    /// * conjunction ∧
    /// * disjunction ∨
    /// * negation ¬
    /// * conditional ➞
    /// * biconditional ⟷
    pub fn mathematical() -> Self{
        Self { map: NotationMap::new([
            (Operator::NOT, ("¬".to_string(), vec!["~".to_string(), "!".to_string()])),
            (Operator::AND, ("∧".to_string(), vec!["^".to_string(), "&".to_string(), "*".to_string(), "⋅".to_string()])),
            (Operator::OR, ("v".to_string(), vec!["∨".to_string(), "|".to_string(), "+".to_string()])),
            (Operator::CON, ("➞".to_string(), vec!["->".to_string(), ">".to_string(), "-->".to_string()])),
            (Operator::BICON, ("⟷".to_string(), vec!["<->".to_string(), "<>".to_string(), "<-->".to_string()])),
            ].into_iter().collect())
        }
    }

    /// Constructs the ascii version of the `OperatorNotation` based on mathematical notation.
    /// 
    /// * conjunction ^
    /// * disjunction ∨
    /// * negation ~
    /// * conditional ->
    /// * biconditional <->
    pub fn mathematical_ascii() -> Self{
        Self { map: NotationMap::new([
            (Operator::NOT, ("~".to_string(), vec!["¬".to_string(), "!".to_string()])),
            (Operator::AND, ("^".to_string(), vec!["&".to_string(), "∧".to_string(), "*".to_string(), "⋅".to_string()])),
            (Operator::OR, ("v".to_string(), vec!["∨".to_string(), "|".to_string(), "+".to_string()])),
            (Operator::CON, ("->".to_string(), vec!["➞".to_string(), ">".to_string(), "-->".to_string()])),
            (Operator::BICON, ("<->".to_string(), vec!["⟷".to_string(), "<>".to_string(), "<-->".to_string()])),
            ].into_iter().collect())
        }
    }

    /// Constructs the `OperatorNotation` based on bit logic notation.
    /// 
    /// * conjunction ⋅
    /// * disjunction +
    /// * negation ¬
    /// * conditional ➞
    /// * biconditional ⟷
    pub fn bits() -> Self{
        Self { map: NotationMap::new([
            (Operator::NOT, ("¬".to_string(), vec!["~".to_string(), "!".to_string()])),
            (Operator::AND, ("⋅".to_string(), vec!["^".to_string(), "&".to_string(), "*".to_string(), "∧".to_string()])),
            (Operator::OR, ("+".to_string(), vec!["∨".to_string(), "|".to_string(), "v".to_string()])),
            (Operator::CON, ("➞".to_string(), vec!["->".to_string(), ">".to_string(), "-->".to_string()])),
            (Operator::BICON, ("⟷".to_string(), vec!["<->".to_string(), "<>".to_string(), "<-->".to_string()])),
            ].into_iter().collect())
        }
    }

    /// Constructs the ascii version of the `OperatorNotation` based on bit logic notation.
    /// 
    /// * conjunction *
    /// * disjunction +
    /// * negation ~
    /// * conditional ->
    /// * biconditional <->
    pub fn bits_ascii() -> Self{
        Self { map: NotationMap::new([
            (Operator::NOT, ("~".to_string(), vec!["¬".to_string(), "!".to_string()])),
            (Operator::AND, ("*".to_string(), vec!["&".to_string(), "∧".to_string(), "^".to_string(), "⋅".to_string()])),
            (Operator::OR, ("+".to_string(), vec!["∨".to_string(), "|".to_string(), "v".to_string()])),
            (Operator::CON, ("->".to_string(), vec!["➞".to_string(), ">".to_string(), "-->".to_string()])),
            (Operator::BICON, ("<->".to_string(), vec!["⟷".to_string(), "<>".to_string(), "<-->".to_string()])),
            ].into_iter().collect())
        }
    }

    /// Constructs the `OperatorNotation` based on boolean logic notation.
    /// 
    /// * conjunction &
    /// * disjunction |
    /// * negation !
    /// * conditional ➞
    /// * biconditional ⟷
    pub fn boolean() -> Self{
        Self { map: NotationMap::new([
            (Operator::NOT, ("!".to_string(), vec!["~".to_string(), "¬".to_string()])),
            (Operator::AND, ("&".to_string(), vec!["^".to_string(), "⋅".to_string(), "*".to_string(), "∧".to_string()])),
            (Operator::OR, ("|".to_string(), vec!["∨".to_string(), "+".to_string(), "v".to_string()])),
            (Operator::CON, ("➞".to_string(), vec!["->".to_string(), ">".to_string(), "-->".to_string()])),
            (Operator::BICON, ("⟷".to_string(), vec!["<->".to_string(), "<>".to_string(), "<-->".to_string()])),
            ].into_iter().collect())
        }
    }

    /// Constructs the ascii version of the `OperatorNotation` based on boolean logic notation.
    /// 
    /// * conjunction &
    /// * disjunction |
    /// * negation !
    /// * conditional ->
    /// * biconditional <->
    pub fn boolean_ascii() -> Self{
        Self { map: NotationMap::new([
            (Operator::NOT, ("!".to_string(), vec!["~".to_string(), "¬".to_string()])),
            (Operator::AND, ("&".to_string(), vec!["^".to_string(), "⋅".to_string(), "*".to_string(), "∧".to_string()])),
            (Operator::OR, ("|".to_string(), vec!["∨".to_string(), "+".to_string(), "v".to_string()])),
            (Operator::CON, ("->".to_string(), vec!["➞".to_string(), ">".to_string(), "-->".to_string()])),
            (Operator::BICON, ("<->".to_string(), vec!["⟷".to_string(), "<>".to_string(), "<-->".to_string()])),
            ].into_iter().collect())
        }
    }

    ///Returns the notation of the given operator.
    pub fn get_default_notation(&self, op: Operator) -> &str{
        &self.map[op][0]
    }

    ///Returns all notations of the given operator.
    pub fn get_all_notations(&self, op: Operator) -> &Vec<String>{
        &self.map[op]
    }

    ///Returns the operator that matches the given notation (if there is any)
    pub fn get_operator(&self, notation: &str) -> Option<Operator>{
        for op in [Operator::NOT, Operator::AND, Operator::OR, Operator::CON, Operator::BICON]{
            for n in self.map[op].iter(){
                if n == notation{
                    return Some(op)
                }
            }
        }

        None
    }

    ///Returns all operators that have partial matches with the given string 
    /// 
    /// The map it returns has the key-value pair of (operator, # of partially-matching notations)
    pub fn get_potential_operators(&self, prefix: &str) -> HashMap<Operator, usize>{
        let mut counts = HashMap::new();
        for op in [Operator::NOT, Operator::AND, Operator::OR, Operator::CON, Operator::BICON]{
            for n in self.map[op].iter(){
                if n.starts_with(prefix){
                    *counts.entry(op).or_insert(0) += 1;
                }
            }
        }

        counts
    }
}

impl Default for OperatorNotation{
    /// Constructs the default `OperatorNotation`:
    /// 
    /// * conjunction &
    /// * disjunction ∨
    /// * negation ¬
    /// * conditional ➞
    /// * biconditional ⟷
    fn default() -> Self {
        Self { map: NotationMap::new([
            (Operator::NOT, ("¬".to_string(), vec!["~".to_string(), "!".to_string()])),
            (Operator::AND, ("&".to_string(), vec!["^".to_string(), "∧".to_string(), "*".to_string(), "⋅".to_string()])),
            (Operator::OR, ("∨".to_string(), vec!["v".to_string(), "|".to_string(), "+".to_string()])),
            (Operator::CON, ("➞".to_string(), vec!["->".to_string(), ">".to_string(), "-->".to_string()])),
            (Operator::BICON, ("⟷".to_string(), vec!["<->".to_string(), "<>".to_string(), "<-->".to_string()])),
            ].into_iter().collect())
        }
    }
}