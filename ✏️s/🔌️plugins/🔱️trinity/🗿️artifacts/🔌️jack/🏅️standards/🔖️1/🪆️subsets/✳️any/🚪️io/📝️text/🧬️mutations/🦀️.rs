//! ⚡️ TrinityGraph mutation text codec and operation-runtime bridge.

use crate::JackSnapshot;

use crate::standards::v1::subsets::any::schema::mutations::TrinityGraphMutation;
use crate::standards::v1::subsets::any::schema::operations::{
    apply_trinity_graph_mutation, apply_trinity_graph_mutations, create_trinity_graph_envelope, dispatch_trinity_graph_mutations, inverse_trinity_graph_mutation, new_trinity_graph_store, validate_trinity_graph_operation, OwnedTrinityGraphStore, TrinityGraphEnvelope, TrinityGraphStore,
};

//#region 🧾️DerivedRegistry
/// 🧾️ Direct-owner text opcodes in aggregate declaration order.
pub const TEXT_OPCODE_REGISTRY: &[(&str, &str)] = &[("SetQuery", crate::standards::v1::subsets::any::schema::mutations::set_query::TEXT_OPCODE)];
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

#[path = "🔎️set-query/🦀️.rs"]
pub mod set_query;

#[allow(unused_imports)]
mod operation_codec {
use crate::standards::v1::subsets::any::schema::mutations::TrinityGraphMutation;
use crate::standards::v1::subsets::any::schema::mutations::set_query;
use crate::executor::GraphEffect;
use crate::{Edge, EntityRef, JackSnapshot, Node, Port, PropertyBag, PropertyDef, PropertyValue};
use protocol::{Mutation, MutationDiff, OpBinary, OpText};
use semio_framework_diagnostic::TextError;
use semio_framework_value::{ValueError,ValueRefusalKind};
/// ⚡️ Local mirror of `TrinityGraphMutation` for `protocol::OpText`/`OpBinary`: each variant name IS its wire keyword
/// (`SetQuery` -> `set-query`), and the real enum's variants wrap handcrafted payload structs that cannot derive the codec.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslEnum)]
pub(crate) enum TrinityGraphOperationDsl {
    SetQuery {
        value: String,
    },
}
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
impl OpText for TrinityGraphOperationDsl {
    fn parse_op(line: &str) -> Result<Self, TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown mutation line '{line}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}
pub(crate) fn trinity_graph_operation_to_dsl(operation: &TrinityGraphMutation) -> TrinityGraphOperationDsl {
    let TrinityGraphMutation::SetQuery(payload) = operation;
    TrinityGraphOperationDsl::SetQuery { value: payload.value.clone() }
}
pub(crate) fn trinity_graph_operation_from_dsl(operation: TrinityGraphOperationDsl) -> TrinityGraphMutation {
    let TrinityGraphOperationDsl::SetQuery { value } = operation;
    set_query(value)
}
/// ⚡️ One-line textual notation for [`TrinityGraphMutation`] (`protocol::OpText`), delegating to the
/// derive-generated `TrinityGraphOperationDsl` mirror.
impl OpText for TrinityGraphMutation {
    fn parse_op(line: &str) -> Result<Self, TextError> {
        <TrinityGraphOperationDsl as OpText>::parse_op(line).map(trinity_graph_operation_from_dsl)
    }

    fn print_op(&self) -> String {
        <TrinityGraphOperationDsl as OpText>::print_op(&trinity_graph_operation_to_dsl(self))
    }
}
}
pub use operation_codec::*;
