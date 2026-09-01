//! ⚡️ TrinityGraph mutation text codec and operation-runtime bridge.

use crate::artifacts::jack::JackSnapshot;

pub use crate::artifacts::jack::schema::mutations::TrinityGraphMutation;
pub use crate::artifacts::jack::schema::operations::{
    apply_trinity_graph_mutation, apply_trinity_graph_mutations, create_trinity_graph_envelope, dispatch_trinity_graph_mutations, inverse_trinity_graph_mutation, validate_trinity_graph_operation, TrinityGraphEnvelope, TrinityGraphStore,
};

//#region 🧾️DerivedRegistry
/// 🧾️ Direct-owner text opcodes in aggregate declaration order.
pub const TEXT_OPCODE_REGISTRY: &[(&str, &str)] = &[
    ("CreateNode", super::create_node::text::TEXT_OPCODE),
    ("DeleteNode", super::delete_node::text::TEXT_OPCODE),
    ("CreateEdge", super::create_edge::text::TEXT_OPCODE),
    ("DeleteEdge", super::delete_edge::text::TEXT_OPCODE),
    ("RenameNode", super::rename_node::text::TEXT_OPCODE),
    ("MoveNode", super::move_node::text::TEXT_OPCODE),
    ("ChangeDataProperty", super::change_data_property::text::TEXT_OPCODE),
    ("RemoveDataProperty", super::remove_data_property::text::TEXT_OPCODE),
];
//#endregion 🧾️DerivedRegistry

//#region 🌉️ExternalCodecBridge
/// 📥️ Decodes the internally tagged JSON projection.
pub fn decode_trinity_graph_mutation_json(text: &str) -> Result<TrinityGraphMutation, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

/// ▶️ Applies one mutation and returns its diagnostic code/severity pairs.
pub fn apply_trinity_graph_mutation_reporting(snapshot: &mut JackSnapshot, mutation: &TrinityGraphMutation) -> Vec<(String, String)> {
    let outcome = <TrinityGraphMutation as protocol::Mutation<JackSnapshot>>::diff(mutation, snapshot).apply_to(snapshot);
    outcome.messages().iter().map(|message| (message.code.0.clone(), format!("{:?}", message.level))).collect()
}

/// ↩️ Computes the mutation's own undo steps.
pub fn inverse_trinity_graph_mutation_steps(mutation: &TrinityGraphMutation, base: &JackSnapshot) -> Vec<TrinityGraphMutation> {
    <TrinityGraphMutation as protocol::Mutation<JackSnapshot>>::inverse(mutation, base)
}
//#endregion 🌉️ExternalCodecBridge

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar
