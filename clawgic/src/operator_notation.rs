//this enum is so I can avoid the overhead of a hashmap.
enum OpType{
    Neg,
    And,
    Or,
    Con,
    Bicon,

    N, //number of OpType enums
}

///Contains a set of symbols for printing `ExpressionTree`s. Used in certain `ExpressionTree` functions to customize expression printing.
pub struct OperatorNotation{
    map: [String; OpType::N as usize],
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
        Self { map: [
            "~".to_string(),
            "&".to_string(),
            "v".to_string(),
            "->".to_string(),
            "<->".to_string(),
            ]
        }
    }

    /// Constructs the `OperatorNotation` based on mathematical notation.
    /// 
    /// * conjunction ^
    /// * disjunction ∨
    /// * negation ¬
    /// * conditional ➞
    /// * biconditional ⟷
    pub fn mathematical() -> Self{
        Self { map: [
            "¬".to_string(),
            "^".to_string(),
            "∨".to_string(),
            "➞".to_string(),
            "⟷".to_string(),
            ]
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
        Self { map: [
            "~".to_string(),
            "^".to_string(),
            "v".to_string(),
            "->".to_string(),
            "<->".to_string(),
            ]
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
        Self { map: [
            "¬".to_string(),
            "⋅".to_string(),
            "+".to_string(),
            "➞".to_string(),
            "⟷".to_string(),
            ]
        }
    }

    /// Constructs the ascii version of the `OperatorNotation` based on bit logic notation.
    /// 
    /// * conjunction *
    /// * disjunction +
    /// * negation ~
    /// * conditional ➞
    /// * biconditional ⟷
    pub fn bits_ascii() -> Self{
        Self { map: [
            "~".to_string(),
            "*".to_string(),
            "+".to_string(),
            "->".to_string(),
            "<->".to_string(),
            ]
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
        Self { map: [
            "!".to_string(),
            "&".to_string(),
            "|".to_string(),
            "➞".to_string(),
            "⟷".to_string(),
            ]
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
        Self { map: [
            "!".to_string(),
            "&".to_string(),
            "|".to_string(),
            "->".to_string(),
            "<->".to_string(),
            ]
        }
    }

    ///Gets the symbol for the conjunction operator.
    pub fn and(&self) -> &str{
        &self.map[OpType::And as usize]
    }

    ///Sets the symbol for the conjunction operator.
    pub fn set_and(&mut self, symbol: String){
        self.map[OpType::And as usize] = symbol;
    }

    //Gets the symbol for the disjunction operator.
    pub fn or(&self) -> &str{
        &self.map[OpType::Or as usize]
    }

    ///Sets the symbol for the disjunction operator.
    pub fn set_or(&mut self, symbol: String){
        self.map[OpType::Or as usize] = symbol;
    }

    //Gets the symbol for the negation operator.
    pub fn neg(&self) -> &str{
        &self.map[OpType::Neg as usize]
    }

    ///Sets the symbol for the negation operator.
    pub fn set_neg(&mut self, symbol: String){
        self.map[OpType::Neg as usize] = symbol;
    }

    //Gets the symbol for the conditional operator.
    pub fn con(&self) -> &str{
        &self.map[OpType::Con as usize]
    }

    ///Sets the symbol for the conditional operator.
    pub fn set_con(&mut self, symbol: String){
        self.map[OpType::Con as usize] = symbol;
    }

    //Gets the symbol for the biconditional operator.
    pub fn bicon(&self) -> &str{
        &self.map[OpType::Bicon as usize]
    }

    ///Sets the symbol for the biconditional operator.
    pub fn set_bicon(&mut self, symbol: String){
        self.map[OpType::Bicon as usize] = symbol;
    }
}

/// Constructs the default `OperatorNotation`:
/// 
/// * conjunction &
/// * disjunction ∨
/// * negation ¬
/// * conditional ➞
/// * biconditional ⟷
impl Default for OperatorNotation{
    fn default() -> Self {
        Self { map: [
            "¬".to_string(),
            "&".to_string(),
            "∨".to_string(),
            "➞".to_string(),
            "⟷".to_string(),
            ]
        }
    }
}