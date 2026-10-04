//! ⚡️ TrinityGraph mutation text codec and operation-runtime bridge.

use crate::JackSnapshot;

pub use crate::standards::v1::subsets::any::schema::mutations::TrinityGraphMutation;
pub use crate::standards::v1::subsets::any::schema::operations::{
    apply_trinity_graph_mutation, apply_trinity_graph_mutations, create_trinity_graph_envelope, dispatch_trinity_graph_mutations, inverse_trinity_graph_mutation, new_trinity_graph_store, validate_trinity_graph_operation, OwnedTrinityGraphStore, TrinityGraphEnvelope, TrinityGraphStore,
};

//#region 🧾️DerivedRegistry
/// 🧾️ Direct-owner text opcodes in aggregate declaration order.
pub const TEXT_OPCODE_REGISTRY: &[(&str, &str)] = &[("SetQuery", super::set_query::text::TEXT_OPCODE)];
//#endregion 🧾️DerivedRegistry

//#region 🌉️ExternalCodecBridge
/// 📥️ Decodes the internally tagged JSON projection.
pub fn decode_trinity_graph_mutation_json(text: &str) -> Result<TrinityGraphMutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// ▶️ Applies one mutation and returns its diagnostic code/severity pairs.
pub fn apply_trinity_graph_mutation_reporting(snapshot: &mut JackSnapshot, mutation: &TrinityGraphMutation) -> Vec<(String, String)> {
    let outcome = <TrinityGraphMutation as protocol::Mutation<JackSnapshot>>::diff(mutation, snapshot).apply_to(snapshot);
    outcome.messages().iter().map(|message| (message.code.0.clone(), format!("{:?}", message.level))).collect()
}

/// ↩️ Computes the mutation's own undo steps.
pub fn inverse_trinity_graph_mutation_steps(mutation: &TrinityGraphMutation, base: &JackSnapshot) -> Result<Vec<TrinityGraphMutation>, semio_framework_value::ValueError> {
    Ok({
    <TrinityGraphMutation as protocol::Mutation<JackSnapshot>>::inverse(mutation, base)?

    })
}
//#endregion 🌉️ExternalCodecBridge

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar
