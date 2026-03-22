use std::ops::{Index, RangeBounds};

use crate::{ClawgicError, expression_tree::ExpressionTree};

/// Variable constant for an ExpressionTree. Not necessary for constructing a tree, but very helpful.
/// 
/// Because an ExpressionVar is immutable and un-consumable, you cannot use them directly in operations.
#[derive(Clone, Debug)]
pub struct ExpressionVar{
    name: String,
    expr: ExpressionTree,
}

impl ExpressionVar{
    ///Constructs and returns an ExpressionVar iff a valid name is given.
    pub fn new(name: &str) -> Result<ExpressionVar, ClawgicError>{
        let name = name.trim().to_string();
        let mut chars = name.chars();
        let first = chars.next();
        if first.is_none_or(|c| !c.is_uppercase()){
            return Err(ClawgicError::InvalidVariableName(name.to_string()));
        }

        for c in chars{
            if !c.is_numeric(){
                return Err(ClawgicError::InvalidVariableName(name.to_string()));
            }
        }

        Ok(Self {expr:  ExpressionTree::new(&name).unwrap(), name})
    }

    ///Returns a reference to the name of the ExpressionVar
    pub fn name(&self) -> &str{
        &self.name
    }
}

///List of enumerated ExpressionVar's. 
/// 
/// Can be indexed two different ways depending on construction.
/// ```
/// use clawgic::prelude::*;
/// //relative indexing
/// let a = ExpressionVars::new("A", 1..=3, true).unwrap();
/// assert_eq!(a[1].name(), "A1");
/// assert_eq!(a[2].name(), "A2");
/// assert_eq!(a[3].name(), "A3");
/// 
/// //absolute indexing
/// let a = ExpressionVars::new("A", 1..=3, false).unwrap();
/// assert_eq!(a[0].name(), "A1");
/// assert_eq!(a[1].name(), "A2");
/// assert_eq!(a[2].name(), "A3");
/// ```
#[derive(Clone, Debug)]
pub struct ExpressionVars{
    vars: Vec<ExpressionVar>,
    bounds: Option<(usize, usize)>,
}

impl ExpressionVars{
    ///Constructs a new ExpressionVars. Type of indexing is decided by 
    /// `relative_index` parameter.
    pub fn new<R>(name: &str, range: R, relative_index: bool) -> Result<Self, ClawgicError>
    where R: RangeBounds<usize>{
        let start = match range.start_bound(){
            std::ops::Bound::Included(s) => *s,
            std::ops::Bound::Excluded(s) => *s + 1,
            std::ops::Bound::Unbounded => return Err(ClawgicError::InvalidVarBounds),
        };
        let end = match range.end_bound(){
            std::ops::Bound::Included(s) => *s,
            std::ops::Bound::Excluded(s) => *s - 1,
            std::ops::Bound::Unbounded => return Err(ClawgicError::InvalidVarBounds),
        };
        let mut vars = Vec::with_capacity(end - start);
        for i in start..=end{
            match ExpressionVar::new(&(name.to_string() + &i.to_string())){
                Ok(v) => vars.push(v),
                Err(e) => return Err(e)
            }
        }

        Ok(Self{
            vars, 
            bounds: if relative_index{Some((start, end))} else {None},
        })
    }

    ///Gets lowest index.
    pub fn start(&self) -> usize{
        self.bounds.unwrap_or((0,0)).0
    }

    ///Gets highest index.
    pub fn end(&self) -> usize{
        self.bounds.unwrap_or((0, self.vars.len() - 1)).1
    }

    ///creates an iterator of all ExpressionVars.
    pub fn iter(&self) -> std::slice::Iter<'_, ExpressionVar>{
        self.vars.iter()
    }
}

impl Index<usize> for ExpressionVars{
    type Output = ExpressionVar;

    fn index(&self, index: usize) -> &Self::Output {
        match self.bounds{
            Some((start, _)) => &self.vars[index - start],
            None => &self.vars[index],
        }
    }
}

impl IntoIterator for ExpressionVars{
    type Item = ExpressionVar;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.vars.into_iter()
    }
}

impl TryFrom<String> for ExpressionVar{
    type Error = ClawgicError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(&value)
    }
}

impl TryFrom<&str> for ExpressionVar{
    type Error = ClawgicError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}