use crate::ClawgicError;

/// Predicate from prediccate (first order) logic.
/// Has a name and an arity (number of vars that it takes).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Predicate{
    ///Name of the predicate
    name: String,
    ///How many variables this predicate takes.
    arity: usize,
}

impl Predicate{
    /// Constructs a new `Predicate` iff the provided name is valid.
    /// 
    /// Valid names are one uppercase letter followed by any number of digits.
    /// (i.e. "A", "B0", "C123") 
    pub fn new(name: &str, arity: usize) -> Result<Self, ()>{
        let name = name.trim().to_string();
        let mut chars = name.chars();
        let first = chars.next();
        if first.is_none_or(|c| !c.is_uppercase()){
            return Err(());
        }

        for c in chars{
            if !c.is_numeric(){
                return Err(());
            }
        }

        Ok(Self{name: name.to_string(), arity})
    }

    ///Gets the name of the predicate.
    pub fn name(&self) -> &str{
        &self.name
    }

    ///Gets the arity of the predicate
    pub fn arity(&self) -> usize{
        self.arity
    }

    /// Alternative to Sentence::new(). More readable in some cases. 
    /// Wins at code golf in all cases.
    pub fn inst(&self, vars: &Vec<String>) -> Result<Sentence, ClawgicError>{
        Sentence::new(self, vars)
    }
}

/// The combination of a predicate and a set of variables.
pub struct Sentence{
    ///The identifying name and arity of the predicate
    predicate: Predicate,
    ///The variables associated with this instantiation of the predicate.
    vars: Vec<String>,
}

impl Sentence{
    /// Creates a new sentence iff all vars have valid names (and there are the right number of vars).
    /// 
    /// If a variable has an invalid name, will return Err(n) where
    /// the variable with the invalid name is vars[n].
    pub fn new(predicate: &Predicate, vars: &Vec<String>) -> Result<Self, ClawgicError>{
        if vars.len() != predicate.arity{

        }

        Ok(Self{predicate: predicate.clone(), vars: vars.clone()})
    }
}