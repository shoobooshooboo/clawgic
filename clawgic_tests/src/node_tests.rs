#[cfg(test)]
mod test{
    use clawgic::expression_tree::{node::{Node, operator::Operator}};
    use test_case::test_case;

    #[test_case(true ; "true node")]
    #[test_case(false ; "false node")]
    fn constant_node(value: bool){
        let n = Node::Constant(value);
        assert_eq!(n.evaluate().unwrap(), value);
    }

    #[test_case(false, Some(true), true ; "true, not denied")]
    #[test_case(false, Some(false), false ; "false, not denied")]
    #[test_case(true, Some(true), false ; "true, denied")]
    #[test_case(true, Some(false), true ; "false, denied")]
    fn variable_node(denied: bool, value: Option<bool>, expected: bool){
        let n = Node::Variable { denied, name: "A".into(), value};
        assert_eq!(n.evaluate().unwrap(), expected);
    }

    #[test]
    fn variable_node_empty(){
        let n = Node::Variable { denied: false, name: "A".into(), value: None };
        assert!(n.evaluate().is_err());
    }

    #[test_case(Operator::AND, true, false, false, false ; "AND OPERATOR")]
    #[test_case(Operator::OR, true, true, true, false ; "OR OPERATOR")]
    #[test_case(Operator::CON, true, false, true, true ; "CON OPERATOR")]
    #[test_case(Operator::BICON, true, false, false, true ; "BICON OPERATOR")]
    fn operator_nodes(operator: Operator, ex1: bool, ex2: bool, ex3: bool, ex4: bool){
        let op = Node::Operator {
            denied: false,
            op: operator,
            left: Box::new(Node::Constant(true)),
            right: Box::new(Node::Constant(true)) 
        };
        assert_eq!(op.evaluate().unwrap(), ex1, "true true failed");

        let op = Node::Operator {
            denied: false,
            op: operator,
            left: Box::new(Node::Constant(true)),
            right: Box::new(Node::Constant(false)) 
        };
        assert_eq!(op.evaluate().unwrap(), ex2, "true false failed");

        let op = Node::Operator {
            denied: false,
            op: operator,
            left: Box::new(Node::Constant(false)),
            right: Box::new(Node::Constant(true)) 
        };
        assert_eq!(op.evaluate().unwrap(), ex3, "false true failed");

        let op = Node::Operator {
            denied: false,
            op: operator,
            left: Box::new(Node::Constant(false)),
            right: Box::new(Node::Constant(false)) 
        };
        assert_eq!(op.evaluate().unwrap(), ex4, "false false failed");
    }

    #[test_case(Node::Variable{denied: false, name: "A".to_string(), value: None}, "A".to_string() ; "Variable")]
    #[test_case(Node::Variable{denied: true, name: "A".to_string(), value: None}, "~A".to_string() ; "Denied Variable")]
    #[test_case(Node::Constant(true), "True".to_string() ; "True Constant")]
    #[test_case(Node::Constant(false), "False".to_string() ; "False Constant")]
    #[test_case(Node::Operator{denied: false, op: Operator::AND, left: Box::new(Node::Constant(true)), right: Box::new(Node::Constant(true))}, "&".to_string() ; "And Operator")]
    #[test_case(Node::Operator{denied: true, op: Operator::AND, left: Box::new(Node::Constant(true)), right: Box::new(Node::Constant(true))}, "~&".to_string() ; "Denied Operator")]
    #[test_case(Node::Operator{denied: false, op: Operator::OR, left: Box::new(Node::Constant(true)), right: Box::new(Node::Constant(true))}, "v".to_string() ; "Or Operator")]
    #[test_case(Node::Operator{denied: false, op: Operator::CON, left: Box::new(Node::Constant(true)), right: Box::new(Node::Constant(true))}, "->".to_string() ; "Con Operator")]
    #[test_case(Node::Operator{denied: false, op: Operator::BICON, left: Box::new(Node::Constant(true)), right: Box::new(Node::Constant(true))}, "<->".to_string() ; "Bicon Operator")]
    fn to_string(node: Node, expected: String){
        assert_eq!(node.to_string(), expected);
    }

    #[test_case(
        Node::Operator{denied: true, op: Operator::AND, left: Box::new(Node::Constant(true)), right: Box::new(Node::Variable{denied: false, name: "A".to_string(), value: None})},
        Node::Operator{denied: false, op: Operator::OR, left: Box::new(Node::Constant(false)), right: Box::new(Node::Variable{denied: true, name: "A".to_string(), value: None})}
        ; "AND")]
    #[test_case(
        Node::Operator{denied: false, op: Operator::OR, left: Box::new(Node::Constant(true)), right: Box::new(Node::Variable{denied: false, name: "A".to_string(), value: None})},
        Node::Operator{denied: true, op: Operator::AND, left: Box::new(Node::Constant(false)), right: Box::new(Node::Variable{denied: true, name: "A".to_string(), value: None})}
        ; "OR")]
    fn demorgans(mut node: Node, expected: Node){
        node.demorgans();
        assert_eq!(node, expected);
    }

    #[test_case(
        Node::Operator { denied: false, op: Operator::BICON, left: Box::new(Node::Variable{denied: false, name: "A".to_string(), value: None}), right:  Box::new(Node::Variable{denied: false, name: "B".to_string(), value: None})},
        Node::Operator { denied: false, op: Operator::AND, 
            left: Box::new(Node::Operator{denied: false, op: Operator::CON, left: Box::new(Node::Variable{denied: false, name: "A".to_string(), value: None}), right: Box::new(Node::Variable{denied: false, name: "B".to_string(), value: None})}), 
            right: Box::new(Node::Operator{denied: false, op: Operator::CON, left: Box::new(Node::Variable{denied: false, name: "B".to_string(), value: None}), right: Box::new(Node::Variable{denied: false, name: "A".to_string(), value: None})})} 
        ; "BICON")]
    #[test_case(
        Node::Operator { denied: false, op: Operator::AND, 
            left: Box::new(Node::Operator{denied: false, op: Operator::CON, left: Box::new(Node::Variable{denied: false, name: "A".to_string(), value: None}), right: Box::new(Node::Variable{denied: false, name: "B".to_string(), value: None})}), 
            right: Box::new(Node::Operator{denied: false, op: Operator::CON, left: Box::new(Node::Variable{denied: false, name: "B".to_string(), value: None}), right: Box::new(Node::Variable{denied: false, name: "A".to_string(), value: None})})}, 
        Node::Operator { denied: false, op: Operator::BICON, left: Box::new(Node::Variable{denied: false, name: "A".to_string(), value: None}), right:  Box::new(Node::Variable{denied: false, name: "B".to_string(), value: None})}
        ; "AND")]
    fn mat_eq(mut node: Node, expected: Node){
        node.mat_eq();
        assert_eq!(node, expected);
    }

    #[test_case(
        Node::Operator{denied: false, op: Operator::CON, left: Box::new(Node::Constant(true)), right: Box::new(Node::Variable{denied: false, name: "A".to_string(), value: None})},
        Node::Operator{denied: false, op: Operator::OR, left: Box::new(Node::Constant(false)), right: Box::new(Node::Variable{denied: false, name: "A".to_string(), value: None})}
        ; "CON")]
    #[test_case(
        Node::Operator{denied: false, op: Operator::OR, left: Box::new(Node::Constant(true)), right: Box::new(Node::Variable{denied: false, name: "A".to_string(), value: None})},
        Node::Operator{denied: false, op: Operator::CON, left: Box::new(Node::Constant(false)), right: Box::new(Node::Variable{denied: false, name: "A".to_string(), value: None})}
        ; "OR")]
    fn implication(mut node: Node, expected: Node){
        node.implication();
        assert_eq!(node, expected);
    }

    #[test_case(
        Node::Operator{denied: true, op: Operator::AND, left: Box::new(Node::Constant(true)), right: Box::new(Node::Variable{denied: false, name: "A".to_string(), value: None})},
        Node::Operator{denied: false, op: Operator::CON, left: Box::new(Node::Constant(true)), right: Box::new(Node::Variable{denied: true, name: "A".to_string(), value: None})}
        ; "AND")]
    #[test_case(
        Node::Operator{denied: false, op: Operator::CON, left: Box::new(Node::Constant(true)), right: Box::new(Node::Variable{denied: false, name: "A".to_string(), value: None})},
        Node::Operator{denied: true, op: Operator::AND, left: Box::new(Node::Constant(true)), right: Box::new(Node::Variable{denied: true, name: "A".to_string(), value: None})}
        ; "CON")]
    fn ncon(mut node: Node, expected: Node){
        node.ncon();
        assert_eq!(node, expected);
    }

    #[test_case(
        Node::Operator { denied: false, op: Operator::BICON, left: Box::new(Node::Variable{denied: false, name: "A".to_string(), value: None}), right:  Box::new(Node::Variable{denied: false, name: "B".to_string(), value: None})},
        Node::Operator { denied: false, op: Operator::OR, 
            left: Box::new(Node::Operator{denied: false, op: Operator::AND, left: Box::new(Node::Variable{denied: false, name: "A".to_string(), value: None}), right: Box::new(Node::Variable{denied: false, name: "B".to_string(), value: None})}), 
            right: Box::new(Node::Operator{denied: false, op: Operator::AND, left: Box::new(Node::Variable{denied: true, name: "A".to_string(), value: None}), right: Box::new(Node::Variable{denied: true, name: "B".to_string(), value: None})})} 
        ; "BICON")]
    fn mat_eq_mono(mut node: Node, expected: Node){
        node.mat_eq_mono();
        assert_eq!(node, expected);
    }
}