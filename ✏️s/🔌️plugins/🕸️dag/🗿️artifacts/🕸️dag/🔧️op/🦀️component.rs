//! 🔧 DAG artifact — Op facet re-exports `DagMutation`.
pub use crate::artifacts::dag::mutations::{apply_dag_mutation, inverse_dag_mutation, DagMutation};

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::Mutation;

    #[test]
    fn set_nodes_inverse_restores_pre_state() {
        let document = infinite_board_port_directed_dag::default_dag_document();
        let mutation = DagMutation::SetNodes { nodes: Vec::new() };
        let inverse = mutation.inverse(&document);
        assert_eq!(inverse, vec![DagMutation::SetNodes { nodes: document.nodes }]);
    }
}
//#endregion 🧪️Tests
