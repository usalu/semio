//! ⚡️ TrinityGraph artifact — grammar + store/mutation re-exports for `TrinityGraphMutation`.

pub use crate::artifacts::jack::mutations::{
    apply_trinity_graph_mutation, apply_trinity_graph_mutations, create_trinity_graph_envelope,
    dispatch_trinity_graph_mutations, inverse_trinity_graph_mutation, TrinityGraphEnvelope,
    TrinityGraphMutation, TrinityGraphStore, validate_trinity_graph_operation,
};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar
