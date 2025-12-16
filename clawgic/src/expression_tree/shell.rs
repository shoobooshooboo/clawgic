use super::node::operator::Operator;

/// This is a data type made for the shunting yard algorithm. 
/// 
/// It represents the tokens of an infix logical expression. 
pub enum Shell{
    /// Binary logical operator.
    Operator(bool, Operator),
    /// Boolean Variable.
    Variable(bool, String),
    /// Boolean constant. True or False.
    Constant(bool),
    /// Open Parentheses.
    Parentheses,
    /// Boolean denial operator.
    Tilde,
}

impl Shell{
    /// Whether the `Shell` is an `Operator`.
    pub fn is_operator(&self) -> bool{
        match self{
            Self::Operator(..) => true,
            _ => false,
        }
    }

    /// Whether the `Shell` is an `Variable`.
    pub fn is_variable(&self) -> bool{
        match self{ 
            Self::Variable(..) => true,
            _ => false,
        }
    }

    /// Whether the `Shell` is an `Constant`.
    pub fn is_constant(&self) -> bool{
        match self{
            Self::Constant(_) => true,
            _ => false,
        }
    }

    /// Whether the `Shell` is an `Parentheses`.
    pub fn is_parentheses(&self) -> bool{
        match self{
            Self::Parentheses => true,
            _ => false,
        }
    }

    /// Whether the `Shell` is an `Tilde`.
    pub fn is_tilde(&self) -> bool{
        match self{
            Self::Tilde => true,
            _ => false,
        }
    }
}