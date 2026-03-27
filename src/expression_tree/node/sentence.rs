use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr, ShrAssign};

use crate::{ClawgicError, prelude::{ExpressionTree, ExpressionVar}, utils};

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
    pub fn inst_strings(&self, vars: &Vec<String>) -> Result<Sentence, ClawgicError>{
        Sentence::new_from_strings(self, vars)
    }

    pub fn inst(&self, vars: &Vec<ExpressionVar>) -> Result<Sentence, ClawgicError>{
        Sentence::new(self, vars)
    }
}

/// A predicate logic atomic sentence.
/// The combination of a predicate and a set of variables.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sentence{
    ///The identifying name and arity of the predicate
    predicate: Predicate,
    ///The variables associated with this instantiation of the predicate.
    vars: Vec<ExpressionVar>,
}

impl Sentence{
    /// Creates a new sentence iff all vars have valid names (and there are the right number of vars).
    pub fn new_from_strings(predicate: &Predicate, vars: &Vec<String>) -> Result<Self, ClawgicError>{
        if vars.len() < predicate.arity{
            return Err(ClawgicError::TooFewVariables);
        }
        if vars.len() > predicate.arity{
            return Err(ClawgicError::TooManyVariables);
        }

        let mut expr_vars = Vec::new();
        for v in vars{
            expr_vars.push(ExpressionVar::new(v)?);
        }

        Ok(Self{predicate: predicate.clone(), vars: expr_vars})
    }

    /// Creates a new Sentence iff there are the proper number of vars
    pub fn new(predicate: &Predicate, vars: &Vec<ExpressionVar>) -> Result<Self, ClawgicError>{
        if vars.len() < predicate.arity{
            Err(ClawgicError::TooFewVariables)
        } else if vars.len() > predicate.arity{
            Err(ClawgicError::TooManyVariables)
        } else{
            Ok(Self{predicate: predicate.clone(), vars: vars.clone()})
        }
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
    pub fn vars(&self) -> &Vec<ExpressionVar>{
        &self.vars
    }

    pub fn expr(&self) -> ExpressionTree{
        ExpressionTree::new(&self.to_string()).unwrap()
    }
}

impl ToString for Sentence{
    fn to_string(&self) -> String {
        let vars: String = format!("{:?}", self.vars).chars().filter(|c| *c != '[' && *c != ']' && *c != '"').collect();
        if vars.is_empty(){
            format!("{}", self.name())
        }else{
            format!("{}({})", self.name(), vars)
        }
    }
}

impl BitAnd<&Sentence> for &Sentence{
    type Output = ExpressionTree;

    fn bitand(self, rhs: &Sentence) -> Self::Output {
        self.expr() & rhs.expr()
    }
}

impl BitAnd<ExpressionTree> for &Sentence{
    type Output = ExpressionTree;

    fn bitand(self, rhs: ExpressionTree) -> Self::Output {
        self.expr() & rhs
    }
}

impl BitAnd<&Sentence> for ExpressionTree{
    type Output = ExpressionTree;

    fn bitand(self, rhs: &Sentence) -> Self::Output {
        self & rhs.expr()
    }
}

impl BitAndAssign<&Sentence> for ExpressionTree{
    fn bitand_assign(&mut self, rhs: &Sentence) {
        *self &= rhs.expr();
    }
}

impl BitOr<&Sentence> for &Sentence{
    type Output = ExpressionTree;

    fn bitor(self, rhs: &Sentence) -> Self::Output {
        self.expr() | rhs.expr()
    }
}

impl BitOr<ExpressionTree> for &Sentence{
    type Output = ExpressionTree;

    fn bitor(self, rhs: ExpressionTree) -> Self::Output {
        self.expr() | rhs
    }
}

impl BitOr<&Sentence> for ExpressionTree{
    type Output = ExpressionTree;

    fn bitor(self, rhs: &Sentence) -> Self::Output {
        self | rhs.expr()
    }
}

impl BitOrAssign<&Sentence> for ExpressionTree{
    fn bitor_assign(&mut self, rhs: &Sentence) {
        *self |= rhs.expr();
    }
}

impl BitXor<&Sentence> for &Sentence{
    type Output = ExpressionTree;

    fn bitxor(self, rhs: &Sentence) -> Self::Output {
        self.expr() ^ rhs.expr()
    }
}

impl BitXor<ExpressionTree> for &Sentence{
    type Output = ExpressionTree;

    fn bitxor(self, rhs: ExpressionTree) -> Self::Output {
        self.expr() ^ rhs
    }
}

impl BitXor<&Sentence> for ExpressionTree{
    type Output = ExpressionTree;

    fn bitxor(self, rhs: &Sentence) -> Self::Output {
        self ^ rhs.expr()
    }
}

impl BitXorAssign<&Sentence> for ExpressionTree{
    fn bitxor_assign(&mut self, rhs: &Sentence) {
        *self ^= rhs.expr();
    }
}

impl Not for &Sentence{
    type Output = ExpressionTree;

    fn not(self) -> Self::Output {
        !self.expr()
    }
}

impl Shl<&Sentence> for &Sentence{
    type Output = ExpressionTree;

    fn shl(self, rhs: &Sentence) -> Self::Output {
        self.expr() << rhs.expr()
    }
}

impl Shl<ExpressionTree> for &Sentence{
    type Output = ExpressionTree;

    fn shl(self, rhs: ExpressionTree) -> Self::Output {
        self.expr() << rhs
    }
}

impl Shl<&Sentence> for ExpressionTree{
    type Output = ExpressionTree;

    fn shl(self, rhs: &Sentence) -> Self::Output {
        self << rhs.expr()
    }
}

impl ShlAssign<&Sentence> for ExpressionTree{
    fn shl_assign(&mut self, rhs: &Sentence) {
        *self <<= rhs.expr();
    }
}

impl Shr<&Sentence> for &Sentence{
    type Output = ExpressionTree;

    fn shr(self, rhs: &Sentence) -> Self::Output {
        self.expr() >> rhs.expr()
    }
}

impl Shr<ExpressionTree> for &Sentence{
    type Output = ExpressionTree;

    fn shr(self, rhs: ExpressionTree) -> Self::Output {
        self.expr() >> rhs
    }
}

impl Shr<&Sentence> for ExpressionTree{
    type Output = ExpressionTree;

    fn shr(self, rhs: &Sentence) -> Self::Output {
        self >> rhs.expr()
    }
}

impl ShrAssign<&Sentence> for ExpressionTree{
    fn shr_assign(&mut self, rhs: &Sentence) {
        *self >>= rhs.expr();
    }
}