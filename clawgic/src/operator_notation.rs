use crate::expression_tree::node::operator::Operator;

///Contains a set of symbols for printing `ExpressionTree`s. Used in certain `ExpressionTree` functions to customize expression printing.
pub struct OperatorNotation{
    map: [String; 5],
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
    /// * conjunction ∧
    /// * disjunction ∨
    /// * negation ¬
    /// * conditional ➞
    /// * biconditional ⟷
    pub fn mathematical() -> Self{
        Self { map: [
            "¬".to_string(),
            "∧".to_string(),
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
    /// * conditional ->
    /// * biconditional <->
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

    ///Returns the notation of the given operator.
    pub fn get_notation(&self, op: Operator) -> &str{
        &self.map[op as usize]
    }

    ///Returns the operator that matches the given notation (if there is any)
    pub fn get_operator(&self, notation: &str) -> Option<Operator>{
        for (i, n) in self.map.iter().enumerate(){
            if notation == n{
                return Some(match i{
                    0 => Operator::NOT,
                    1 => Operator::AND,
                    2 => Operator::OR,
                    3 => Operator::CON,
                    4 => Operator::BICON,
                    _ => panic!("Unsupported operator inside of set_notation"),
                });
            }
        }
        None
    }

    ///Returns the operator that appears at the beginning of the string.
    pub fn get_prefix_operator(&self, expression: &str) -> Option<Operator>{
        for (i, n) in self.map.iter().enumerate(){
            if expression.starts_with(n){
                return Some(match i{
                    0 => Operator::NOT,
                    1 => Operator::AND,
                    2 => Operator::OR,
                    3 => Operator::CON,
                    4 => Operator::BICON,
                    _ => panic!("Unsupported operator inside of set_notation"),
                });
            }
        }
        None
    }

    /// Sets the notation of an operator and returns Ok(()) if all goes well.
    /// 
    /// If there's a conflict with the new notation and a pre-existing notation,
    /// returns the operator there's a conflict with.
    pub fn set_notation(&mut self, op: Operator, notation: String) -> Result<(), Operator>{
        for (i, n) in self.map.iter().enumerate().skip_while(|(j, _)| *j == op as usize){
            if notation.starts_with(n) || n.starts_with(&notation){
                return Err(match i{
                    0 => Operator::NOT,
                    1 => Operator::AND,
                    2 => Operator::OR,
                    3 => Operator::CON,
                    4 => Operator::BICON,
                    _ => panic!("Unsupported operator inside of set_notation"),
                });
            }
        }
        
        self.map[op as usize] = notation;
        Ok(())
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