/// Types of logical operators that exist in Sentential Logic (SL)
/// "~" (the denial operator) is excluded because, as a unary operator,
/// it's simpler to handle it within each node rather than have it take up a whole node on it's own.
/// 
/// The Negation operator is not actually supported in operator nodes. It's inclusion is just so that
/// `Operator` is all encompassing and can be used for extra things.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Operator{
    /// Negation. ~
    NOT,
    /// Conjunction. &, ^
    AND,
    /// Disjunction. v
    OR,
    /// Conditional. ->
    CON,
    /// Biconditional. <->
    BICON,
    /// Universal. @
    UNI,
    /// Existential #
    EXI,
}

impl Operator{
    /// Checks if the operator is a conjunction.
    pub fn is_and(&self) -> bool{
        match self{
            Self::AND => true,
            _ => false,
        }
    }

    /// Checks if the operator is a disjunction.
    pub fn is_or(&self) -> bool{
        match self{
            Self::OR => true,
            _ => false,
        }
    }

    /// Checks if the operator is a conditional.
    pub fn is_con(&self) -> bool{
        match self{
            Self::CON => true,
            _ => false,
        }
    }

    /// Checks if the operator is a biconditional.
    pub fn is_bicon(&self) -> bool{
        match self{
            Self::BICON => true,
            _ => false,
        }
    }

    /// Checks if the operator is a negation.
    pub fn is_not(&self) -> bool{
        match self{
            Self::NOT => true,
            _ => false,
        }
    }

    /// Checks if the operator is an universal
    pub fn is_uni(&self) -> bool{
        match self{
            Self::UNI => true,
            _ => false,
        }
    }

    /// Checks if the operator is an existential
    pub fn is_exi(&self) -> bool{
        match self{
            Self::EXI => true,
            _ => false,
        }
    }

    /// Whether the operator is binary.
    pub fn is_binary(&self) -> bool{
        self.arity() == 2
    }

    /// Whether the operator is unary.
    pub fn is_unary(&self) -> bool{
        self.arity() == 1
    }

    /// Whether the operator is a quantifier.
    pub fn is_quantifier(&self) -> bool{
        self.is_uni() || self.is_exi()
    }

    /// Returns the precedence of the operator.
    /// 
    /// Lower number is higher precedence.
    /// 
    /// Precedence is as follows:
    /// * AND (conjunction): 3
    /// * OR (disjunction): 3
    /// * CON (conditional): 2
    /// * BICON (biconditional): 1 
    /// * UNI (universal): 0
    /// * EXI (existential): 0
    /// * NOT (negation): 0
    pub fn precedence(&self) -> u8{
        match self{
            Self::AND => 3,
            Self::OR => 3,
            Self::CON => 2,
            Self::BICON => 1,
            Self::NOT => 0,
            Self::UNI => 0,
            Self::EXI => 0,
        }
    }

    /// Returns the arity of the operator.
    /// 
    /// Binary operators return 2, unary return 1.
    /// 
    /// Arity is as follows:
    /// * AND (conjunction): 2
    /// * OR (disjunction): 2
    /// * CON (conditional): 2
    /// * BICON (biconditional): 2 
    /// * UNI (universal): 1
    /// * EXI (existential): 1
    /// * NOT (negation): 1
    pub fn arity(&self) -> u8{
        match self{
            Self::AND |
            Self::OR |
            Self::CON | 
            Self::BICON => 2,
            Self::NOT |
            Self::UNI |
            Self::EXI => 1,
        }
    }

    /// Takes two booleans and performs the appropriate evaluation with the given binary operator. 
    /// 
    /// panics if a unary operator is given.
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
    pub fn execute_binary(&self, left: bool, right: bool) -> bool{
        match self{
            Self::AND => left && right,
            Self::OR => left || right,
            Self::CON => !left || right,
            Self::BICON => left == right,
            Self::NOT | Self::UNI | Self::EXI => panic!("Attempting to evaluate a unary operator as a binary operator"),
        }
    }

    /// Attempts short-circuit evaluation with only one boolean with the given binary operator.
    /// 
    /// panics if unary operator is given
    /// 
    /// # ex
    /// ```
    /// use clawgic::expression_tree::node::operator::Operator;
    /// let op = Operator::AND;
    /// assert_eq!(op.short_circuit(false), Some(false));
    /// assert_eq!(op.short_circuit(true), None);
    /// let op = Operator::OR;
    /// assert_eq!(op.short_circuit(false), None);
    /// assert_eq!(op.short_circuit(true), Some(true));
    /// let op = Operator::CON;
    /// assert_eq!(op.short_circuit(false), Some(true));
    /// assert_eq!(op.short_circuit(true), None);
    /// let op = Operator::BICON;
    /// assert_eq!(op.short_circuit(false), None);
    /// assert_eq!(op.short_circuit(true), None);
    /// ```
    pub fn short_circuit_bin(&self, left: bool) -> Option<bool>{
        match self{
            Self::AND => if !left {Some(false)} else {None},
            Self::OR => if left {Some(true)} else {None},
            Self::CON => if !left {Some(true)} else {None} ,
            Self::BICON => None,
            Self::NOT | Self::UNI | Self::EXI => panic!("Attempting to evaluate a unary operator as a binary operator"),
        }
    }
}