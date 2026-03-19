use crate::{expression_tree::node::negation::Negation, prelude::Predicate};

use super::node::operator::Operator;

/// This is a data type made for the shunting yard algorithm. 
/// 
/// It represents the tokens of an infix logical expression. 
pub enum Token{
    /// Binary logical operator.
    Operator(Negation, Operator),
    /// Boolean Variable.
    Sentence(Negation, Predicate, Vec<String>),
    /// Boolean constant. True or False.
    Constant(Negation, bool),
    /// Open Parentheses.
    OpenParentheses,
    ///Closed Parantheses.
    ClosedParentheses,
    /// Boolean denial operator.
    Tilde(Negation),
}

impl Token{
    /// Whether the `Shell` is an `Operator`.
    pub fn is_operator(&self) -> bool{
        match self{
            Self::Operator(..) => true,
            _ => false,
        }
    }

    /// Whether the `Shell` is an `Variable`.
    pub fn is_sentence(&self) -> bool{
        match self{ 
            Self::Sentence(..) => true,
            _ => false,
        }
    }

    /// Whether the `Shell` is an `Constant`.
    pub fn is_constant(&self) -> bool{
        match self{
            Self::Constant(..) => true,
            _ => false,
        }
    }

    /// Whether the `Shell` is an `OpenParentheses`.
    pub fn is_open_parentheses(&self) -> bool{
        match self{
            Self::OpenParentheses => true,
            _ => false,
        }
    }

    /// Whether the `Shell` is a `ClosedParentheses`.
    pub fn is_closed_parentheses(&self) -> bool{
        match self{
            Self::ClosedParentheses => true,
            _ => false,
        }
    }

    /// Whether the `Shell` is an `Tilde`.
    pub fn is_tilde(&self) -> bool{
        match self{
            Self::Tilde(..) => true,
            _ => false,
        }
    }
}