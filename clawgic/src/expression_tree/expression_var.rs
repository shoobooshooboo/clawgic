use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Index, Not, RangeBounds, Shl, ShlAssign, Shr, ShrAssign};

use crate::expression_tree::ExpressionTree;

/// Atomic Variable for an ExpressionTree. Not necessary for constructing a tree, but very helpful.
/// 
/// Because an ExpressionVar is immutable and un-consumable, you cannot use them directly in operations.
/// Instead you have two options to choose between:
/// ```
/// use clawgic::expression_tree::{ExpressionTree, expression_var::ExpressionVar};
/// 
/// let a = ExpressionVar::new("A").unwrap();
/// let b = ExpressionVar::new("B").unwrap();
/// let expr1 = &a & &b;
/// let expr2 = a.expr() & b.expr();
/// assert!(expr1.lit_eq(&expr2) && ExpressionTree::new("A&B").unwrap().lit_eq(&expr1))
/// ```
#[derive(Clone, Debug)]
pub struct ExpressionVar{
    name: String,
    expr: ExpressionTree,
}

impl ExpressionVar{
    ///Constructs and returns an ExpressionVar iff a valid name is given.
    pub fn new(name: &str) -> Result<ExpressionVar, ()>{
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

        Ok(Self {expr:  ExpressionTree::new(&name).unwrap(), name})
    }

    ///Returns a reference to the name of the ExpressionVar
    pub fn name(&self) -> &str{
        &self.name
    }

    /// Returns an ExpressionTree containing only the variable.
    /// 
    /// Clones an ExpressionTree with each call, but it's a very inexpensive clone.
    pub fn expr(&self) -> ExpressionTree{
        self.expr.clone()
    }
}

impl BitAnd<&ExpressionVar> for &ExpressionVar{
    type Output = ExpressionTree;

    fn bitand(self, rhs: &ExpressionVar) -> Self::Output {
        self.expr() & rhs.expr()
    }
}

impl BitAnd<ExpressionTree> for &ExpressionVar{
    type Output = ExpressionTree;

    fn bitand(self, rhs: ExpressionTree) -> Self::Output {
        self.expr() & rhs
    }
}

impl BitAnd<&ExpressionVar> for ExpressionTree{
    type Output = ExpressionTree;

    fn bitand(self, rhs: &ExpressionVar) -> Self::Output {
        self & rhs.expr()
    }
}

impl BitAndAssign<&ExpressionVar> for ExpressionTree{
    fn bitand_assign(&mut self, rhs: &ExpressionVar) {
        *self &= rhs.expr();
    }
}

impl BitOr<&ExpressionVar> for &ExpressionVar{
    type Output = ExpressionTree;

    fn bitor(self, rhs: &ExpressionVar) -> Self::Output {
        self.expr() | rhs.expr()
    }
}

impl BitOr<ExpressionTree> for &ExpressionVar{
    type Output = ExpressionTree;

    fn bitor(self, rhs: ExpressionTree) -> Self::Output {
        self.expr() | rhs
    }
}

impl BitOr<&ExpressionVar> for ExpressionTree{
    type Output = ExpressionTree;

    fn bitor(self, rhs: &ExpressionVar) -> Self::Output {
        self | rhs.expr()
    }
}

impl BitOrAssign<&ExpressionVar> for ExpressionTree{
    fn bitor_assign(&mut self, rhs: &ExpressionVar) {
        *self |= rhs.expr();
    }
}

impl BitXor<&ExpressionVar> for &ExpressionVar{
    type Output = ExpressionTree;

    fn bitxor(self, rhs: &ExpressionVar) -> Self::Output {
        self.expr() ^ rhs.expr()
    }
}

impl BitXor<ExpressionTree> for &ExpressionVar{
    type Output = ExpressionTree;

    fn bitxor(self, rhs: ExpressionTree) -> Self::Output {
        self.expr() ^ rhs
    }
}

impl BitXor<&ExpressionVar> for ExpressionTree{
    type Output = ExpressionTree;

    fn bitxor(self, rhs: &ExpressionVar) -> Self::Output {
        self ^ rhs.expr()
    }
}

impl BitXorAssign<&ExpressionVar> for ExpressionTree{
    fn bitxor_assign(&mut self, rhs: &ExpressionVar) {
        *self ^= rhs.expr();
    }
}

impl Not for &ExpressionVar{
    type Output = ExpressionTree;

    fn not(self) -> Self::Output {
        !self.expr()
    }
}

impl Shl<&ExpressionVar> for &ExpressionVar{
    type Output = ExpressionTree;

    fn shl(self, rhs: &ExpressionVar) -> Self::Output {
        self.expr() << rhs.expr()
    }
}

impl Shl<ExpressionTree> for &ExpressionVar{
    type Output = ExpressionTree;

    fn shl(self, rhs: ExpressionTree) -> Self::Output {
        self.expr() << rhs
    }
}

impl Shl<&ExpressionVar> for ExpressionTree{
    type Output = ExpressionTree;

    fn shl(self, rhs: &ExpressionVar) -> Self::Output {
        self << rhs.expr()
    }
}

impl ShlAssign<&ExpressionVar> for ExpressionTree{
    fn shl_assign(&mut self, rhs: &ExpressionVar) {
        *self <<= rhs.expr();
    }
}

impl Shr<&ExpressionVar> for &ExpressionVar{
    type Output = ExpressionTree;

    fn shr(self, rhs: &ExpressionVar) -> Self::Output {
        self.expr() >> rhs.expr()
    }
}

impl Shr<ExpressionTree> for &ExpressionVar{
    type Output = ExpressionTree;

    fn shr(self, rhs: ExpressionTree) -> Self::Output {
        self.expr() >> rhs
    }
}

impl Shr<&ExpressionVar> for ExpressionTree{
    type Output = ExpressionTree;

    fn shr(self, rhs: &ExpressionVar) -> Self::Output {
        self >> rhs.expr()
    }
}

impl ShrAssign<&ExpressionVar> for ExpressionTree{
    fn shr_assign(&mut self, rhs: &ExpressionVar) {
        *self >>= rhs.expr();
    }
}

///List of enumerated ExpressionVar's
#[derive(Clone, Debug)]
pub struct ExpressionVars{
    vars: Vec<ExpressionVar>,
    bounds: Option<(usize, usize)>,
}

impl ExpressionVars{
    pub fn new<R>(name: &str, range: R, relative_index: bool) -> Result<Self, ()>
    where R: RangeBounds<usize>{
        let start = match range.start_bound(){
            std::ops::Bound::Included(s) => *s,
            std::ops::Bound::Excluded(s) => *s + 1,
            std::ops::Bound::Unbounded => return Err(()),
        };
        let end = match range.end_bound(){
            std::ops::Bound::Included(s) => *s,
            std::ops::Bound::Excluded(s) => *s - 1,
            std::ops::Bound::Unbounded => return Err(()),
        };
        let mut vars = Vec::with_capacity(end - start);
        for i in start..=end{
            match ExpressionVar::new(&(name.to_string() + &i.to_string())){
                Ok(v) => vars.push(v),
                Err(()) => return Err(())
            }
        }

        Ok(Self{
            vars, 
            bounds: if relative_index{Some((start, end))} else {None},
        })
    }

    pub fn start(&self) -> usize{
        self.bounds.unwrap_or((0,0)).0
    }

    pub fn end(&self) -> usize{
        self.bounds.unwrap_or((0, self.vars.len() - 1)).1
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