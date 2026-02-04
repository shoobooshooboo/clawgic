use crate::{ClawgicError, utils};

/// Predicate from prediccate (first order) logic.
/// Has a name and an arity (number of vars that it takes).
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
    pub fn new(name: &str, arity: usize) -> Result<Self, ClawgicError>{
        if !utils::is_valid_predicate_name(name){
            return Err(ClawgicError::InvalidVariableName(name.to_string()))
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

/// A predicate logic atomic sentence.
/// The combination of a predicate and a set of variables.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sentence{
    ///The identifying name and arity of the predicate
    predicate: Predicate,
    ///The variables associated with this instantiation of the predicate.
    vars: Vec<String>,
}

impl Sentence{
    /// Creates a new sentence iff all vars have valid names (and there are the right number of vars).
    pub fn new(predicate: &Predicate, vars: &Vec<String>) -> Result<Self, ClawgicError>{
        if vars.len() < predicate.arity{
            return Err(ClawgicError::TooFewVariables);
        }
        if vars.len() > predicate.arity{
            return Err(ClawgicError::TooManyVariables);
        }

        for v in vars{
            if !utils::is_valid_var_name(v){
                return Err(ClawgicError::InvalidVariableName(v.clone()))
            }
        }

        Ok(Self{predicate: predicate.clone(), vars: vars.clone()})
    }

    ///Gets the predicate.
    pub fn predicate(&self) -> &Predicate{
        &self.predicate
    }

    ///Gets the predicates name.
    pub fn name(&self) -> &str{
        &self.predicate.name
    }

    ///Gets the predicates arity.
    pub fn arity(&self) -> usize{
        self.predicate.arity
    }

    ///Gets all the vars.
    pub fn vars(&self) -> &Vec<String>{
        &self.vars
    }
}

impl ToString for Sentence{
    fn to_string(&self) -> String {
        let mut s = self.predicate.name.clone();
        s += "(";
        for v in self.vars.iter(){
            s += v;
            s += ",";
        }
        if self.vars.len() > 0{
            s.pop();
        }
        s += ")";
        s
    }
}