use std::collections::{HashMap, HashSet};

use crate::{ClawgicError, prelude::{Predicate, Sentence}, utils};

/// Evaluation context for an expression tree.
///
/// Contains all:
/// * existing variables (i.e. "a", "b12", etc.), 
/// * existing predicates (i.e. ("P", 0), ("Q", 2), etc), 
/// * known values (i.e. "P", "~Q(a,b12)") 
#[derive(Debug, Clone)]
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

    /// Attempts to add the given variable into the Universe. 
    /// If the name is invalid, returns 
    pub fn insert_variable(&mut self, variable: &str) -> Result<bool, ClawgicError>{
        if !utils::is_valid_var_name(&variable){
            return Err(ClawgicError::InvalidVariableName(variable.to_string()))
        }

        Ok(self.variables.insert(variable.to_string()))
    }

    /// Attemps to add several variables into the Universe.
    pub fn insert_variables<It: Iterator<Item = String>>(&mut self, variables: It) -> Result<(), ClawgicError>{
        for var in variables{
            if !utils::is_valid_var_name(&var){
                return Err(ClawgicError::InvalidVariableName(var));
            }
            self.variables.insert(var);
        }

        Ok(())
    }

    /// Adds a predicate to the Universe.
    /// Returns false if the predicate was already in the Universe. 
    pub fn insert_predicate(&mut self, predicate: Predicate) -> bool{
        if self.predicates.contains_key(&predicate){
            return false;
        }
        self.predicates.entry(predicate).or_default();
        true
    }

    /// Adds several predicates to the universe.
    pub fn insert_predicates<It: Iterator<Item = Predicate>>(&mut self, predicates: It){
        for pred in predicates{
            self.predicates.entry(pred.clone()).or_default();
        }
    }

    /// Adds a sentence and it's truth value to the Universe.
    /// If the sentence was already in the Universe, returns the previous truth value.
    /// 
    /// If the sentence's predicate is not in the Universe already, it is added.
    pub fn insert_sentence(&mut self, sentence: Sentence, truth_value: bool) -> Option<bool>{
        self.predicates.entry(sentence.predicate().clone()).or_default().insert(sentence, truth_value)
    }

    /// Adds several sentences and their truth values to the Universe.
    pub fn insert_sentences<It: Iterator<Item = (Sentence, bool)>>(&mut self, sentences: It){
        for s in sentences{
            self.insert_sentence(s.0, s.1);
        }
    }

    ///returns the set of variables.
    pub fn variables(&self) -> &HashSet<String>{
        &self.variables
    }

    ///returns an iterator of all the predicates.
    pub fn predicates(&self) -> std::collections::hash_map::Keys<'_, Predicate, HashMap<Sentence, bool>>{
        self.predicates.keys()
    }

    ///Gets all sentences and their truth values of the given predicate.
    pub fn all_sentences(&self, predicate: &Predicate) -> Option<&HashMap<Sentence, bool>>{
        self.predicates.get(predicate)
    }

    ///Gets the truth value of the given sentence.
    pub fn get_tval(&self, sentence: &Sentence) -> Option<bool>{
        self.predicates.get(sentence.predicate()).and_then(|map| map.get(sentence)).copied()
    }

    ///Gets a mutable reference to the truth value of the given sentence.
    pub fn get_tval_mut(&mut self, sentence: &Sentence) -> Option<&mut bool>{
        self.predicates.get_mut(sentence.predicate()).and_then(|map| map.get_mut(sentence))
    }

    ///Adds all the contents of another universe to this one. 
    ///If there are conflicts, defaults to other's values.
    pub fn add_universe(&mut self, other: &Universe){
        self.insert_variables(other.variables.iter().cloned());
        self.insert_predicates(other.predicates().cloned());
        other.predicates.iter().for_each(|(_, m)| 
            m.iter().for_each(|(s, &b)| {self.insert_sentence(s.clone(), b);}));
    }

    ///Makes self entirely distinct from other.
    pub fn subtract_universe(&mut self, other: &Universe){
        todo!();
    }

    // ///Returns true if the two universes have the same constants, predicates, and concrete sentences
    // pub fn syn_eq(&self, other: &Self) -> bool{

    // }
}

impl PartialEq for Universe{
    fn eq(&self, other: &Self) -> bool {
        let mut same_vars = true;
        self.variables.iter().for_each(|name| if !other.variables.contains(name) {same_vars = false});
        if !same_vars{return false;}
        other.variables.iter().for_each(|name| if !self.variables.contains(name) {same_vars = false});
        if !same_vars{return false;}

        let mut same_sentences = true;
        self.predicates.iter().for_each(|(pred, map)| {
            if !same_sentences{return;}
            let other_map = other.all_sentences(pred);
            if other_map.is_none(){same_sentences = false; return;}
            let other_map = other_map.unwrap();
            map.iter().for_each(|(sentence, tval)| if other_map.get(sentence).is_none_or(|other_tval| *tval != *other_tval){same_sentences = false;});
        });
        if !same_sentences{return false;}
        other.predicates.iter().for_each(|(pred, map)| {
            if !same_sentences{return;}
            let self_map = self.all_sentences(pred);
            if self_map.is_none(){same_sentences = false; return;}
            let self_map = self_map.unwrap();
            map.iter().for_each(|(sentence, tval)| if self_map.get(sentence).is_none_or(|self_tval| *tval != *self_tval){same_sentences = false;});
        });
        if !same_sentences{return false;}

        true
    }

    fn ne(&self, other: &Self) -> bool {
        !(self == other)
    }
}

impl Eq for Universe{}