use std::error::Error;

use clawgic::expression_tree::ExpressionTree;

mod expression_tree_tests;

mod expression_var_tests;

mod node_tests;


fn main() -> Result<(), Box<dyn Error>>{
    ExpressionTree::new("")?;
    Ok(())
}