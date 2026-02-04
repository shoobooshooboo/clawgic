use std::collections::{HashMap, HashSet};

use crate::{ClawgicError, prelude::{Predicate, Sentence}, utils};

/// Evaluation context for an expression tree.
///
/// Contains all:
/// * existing variables (i.e. "a", "b12", etc.), 
/// * existing predicates (i.e. ("P", 0), ("Q", 2), etc), 
/// * known values (i.e. "P", "~Q(a,b12)") 
pub struct Universe{
    //Things that exist
    /// All variables in the universe.
    variables: HashSet<String>,

    /// All predicates in the universe. 
    /// 
    /// Maps each predicate to each known sentence that uses that predicate
    predicates: HashMap<Predicate, HashMap<Sentence, bool>>,
}

impl Universe{
    /// Constructs a new `Universe`. Nothing fancy.
    pub fn new() -> Self{
        Self { variables: HashSet::new(), predicates: HashMap::new() }
    }

    /// Attempts to insert the given variable into the Universe. 
    /// If the name is invalid, returns 
    pub fn insert_variable(&mut self, variable: &str) -> Result<bool, ClawgicError>{
        if !utils::is_valid_var_name(&variable){
            return Err(ClawgicError::InvalidVariableName(variable.to_string()))
        }

        Ok(self.variables.insert(variable.to_string()))
    }

    pub fn insert_variables<'a, T: Iterator<Item = &'a String>>(&mut self, variables: T) -> Result<(), ClawgicError>{
        for var in variables{
            if !utils::is_valid_var_name(var){
                return Err(ClawgicError::InvalidVariableName(var.clone()));
            }
            self.variables.insert(var.clone());
        }

        Ok(())
    }

    pub fn insert_predicate(&mut self, predicate: &Predicate) -> bool{
        if self.predicates.contains_key(predicate){
            return false;
        }
        self.predicates.entry(predicate.clone()).or_default();
        true
    }

    pub fn insert_predicates<'a, T: Iterator<Item = &'a Predicate>>(&mut self, predicates: T){
        for pred in predicates{
            self.predicates.entry(pred.clone()).or_default();
        }
    }

}