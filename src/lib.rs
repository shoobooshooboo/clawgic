#[allow(dead_code)]
pub mod expression_tree;

pub mod operator_notation;

pub mod prelude;

mod utils;

#[cfg(test)]
mod tests;

/// All the errors that can occur in making and managing an `ExpressionTree`. 
#[derive(Debug, PartialEq, Eq)]
pub enum ClawgicError{
    UninitializedSentence(String),
    InvalidExpression,
    EmptyExpression,
    UnknownSymbol(String),
    InvalidParentheses,
    TooManyOperators,
    NotEnoughOperators,
    InvalidPredicateName(String),
    InvalidVariableName(String),
    AmbiguousExpression,
    TooFewVariables,
    TooManyVariables,
}

impl std::fmt::Display for ClawgicError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self{
            Self::UninitializedSentence(s) => format!("Uninitialized variable \"{s}\""),
            Self::InvalidExpression => "Invalid expression".to_string(),
            Self::UnknownSymbol(s) => format!("Unknown symbol \"{s}\""),
            Self::InvalidParentheses => "Invalid parenthesis".to_string(),
            Self::TooManyOperators => "Too many operators".to_string(),
            Self::NotEnoughOperators => "Not enough operators".to_string(),
            Self::InvalidPredicateName(s) => format!("Invalid predicate name \"{s}\""),
            Self::InvalidVariableName(s) => format!("Invalid variable name \"{s}\""),
            Self::AmbiguousExpression => "Ambiguous expression".to_string(),
            Self::TooFewVariables => "Not enough variables for the given predicate".to_string(),
            Self::TooManyVariables => "Too many operators for the given predicate".to_string(),
            Self::EmptyExpression => "Expression is empty".to_string(),
        })
    }
}

impl std::error::Error for ClawgicError{}

//∧ ∨ ¬ ➞ ⟷ ⋅