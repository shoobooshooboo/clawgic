/// Types of logical operators that exist in Sentential Logic (SL)
/// "~" (the denial operator) is excluded because, as a unary operator,
/// it's simpler to handle it within each node rather than have it take up a whole node on it's own.
/// 
/// The Negation operator is not actually supported in operator nodes. It's inclusion is just so that
/// `Operator` is all encompassing and can be used for extra things.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operator{
    /// Conjunction. &, ^
    AND,
    /// Disjunction. v
    OR,
    /// Conditional. ->
    CON,
    /// Biconditional. <->
    BICON,
    /// Negation. ~
    NOT,
}

impl Operator{
    /// Checks if the current node is a conjunction.
    pub fn is_and(&self) -> bool{
        match self{
            Self::AND => true,
            _ => false,
        }
    }

    /// Checks if the current node is a disjunction.
    pub fn is_or(&self) -> bool{
        match self{
            Self::OR => true,
            _ => false,
        }
    }

    /// Checks if the current node is a conditional.
    pub fn is_con(&self) -> bool{
        match self{
            Self::CON => true,
            _ => false,
        }
    }

    /// Checks if the current node is a biconditional.
    pub fn is_bicon(&self) -> bool{
        match self{
            Self::BICON => true,
            _ => false,
        }
    }

    /// Checks if the current node is a negation.
    pub fn is_not(&self) -> bool{
        match self{
            Self::NOT => true,
            _ => false,
        }
    }

    /// Returns the precedence of the node.
    /// Higher number is higher precedence.
    /// Precedence is as follows:
    /// * AND (conjunction): 3
    /// * OR (disjunction): 3
    /// * CON (conditional): 2
    /// * BICON (biconditional): 1 
    pub fn precedence(&self) -> u8{
        match self{
            Self::AND => 3,
            Self::OR => 3,
            Self::CON => 2,
            Self::BICON => 1,
            Self::NOT => 0,
        }
    }

    /// Takes two booleans and performs the appropriate evaluation with the given operator.
    /// 
    /// # ex
    /// ```
    /// use clawgic::expression_tree::node::operator::Operator;
    /// let op = Operator::AND;
    /// assert!(op.execute(true, true));
    /// assert!(!op.execute(true, false));
    /// assert!(!op.execute(false, true));
    /// assert!(!op.execute(false, false));
    /// ```
    pub fn execute(&self, left: bool, right: bool) -> bool{
        match self{
            Self::AND => left && right,
            Self::OR => left || right,
            Self::CON => !left || right,
            Self::BICON => left == right,
            Self::NOT => panic!("Operator nodes cannot be Negation nodes"),
        }
    }
}