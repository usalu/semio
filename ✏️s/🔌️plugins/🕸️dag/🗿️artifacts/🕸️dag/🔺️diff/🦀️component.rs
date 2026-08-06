//! 🔺️ DAG artifact — the operation diff (constitutional: diff).
//!
//! `OperationDiff<DagDocument> for DagDiff` and its `apply`/`absorb` logic are implemented directly in
//! the DAG kernel crate (`infinite_board_port_directed_dag`, `🔖️DocumentVcs` region) alongside
//! `DagDocument`/`DagOperation` themselves — see `crate::artifacts::dag::op`'s doc for why. This module
//! only re-exports the kernel's `DagDiff` type under this crate's taxonomy node so sibling components
//! depend on a stable app-owned path instead of reaching into the kernel path directly, mirroring
//! `dsl`/`pack`/`spr`'s equivalent re-export pattern.

//#region 🔖️Types
pub use infinite_board_port_directed_dag::DagDiff;
//#endregion 🔖️Types

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dag_diff_default_has_no_pending_writes() {
        let diff = DagDiff::default();
        assert_eq!(diff, DagDiff { document: None, nodes: None, edges: None, set_nodes: None, set_edges: None });
    }
}
//#endregion 🧪️Tests

#[cfg(test)]
mod semio_grammar_conformance {
    use super::*;

    #[test]
    fn component_grammar_semio_is_grammar_dialect() {
        let g = ::dsl::parse_grammar(COMPONENT_GRAMMAR_SEMIO).expect("parse grammar.semio");
        assert_eq!(g.dialect, ::dsl::SemioDialect::Grammar);
        assert!(!COMPONENT_GRAMMAR_SEMIO.is_empty());
        let _ = COMPONENT_GRAMMAR_PATH;
    }
}

