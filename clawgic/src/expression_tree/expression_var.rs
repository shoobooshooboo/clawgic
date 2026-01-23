use crate::expression_tree::ExpressionTree;

#[derive(Clone, Debug)]
pub struct ExpressionVar{
    name: String,
    expr: ExpressionTree,
}

impl ExpressionVar{
    ///Constructs and returns an ExpressionVar iff a valid name is given.
    pub fn new(name: String) -> Result<ExpressionVar, ()>{
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
}