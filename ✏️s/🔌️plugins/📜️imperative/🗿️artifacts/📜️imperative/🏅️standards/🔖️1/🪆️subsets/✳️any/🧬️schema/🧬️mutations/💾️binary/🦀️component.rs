//! ⚖️ Imperative artifact — state-patch-representation wire codec + laws (was: constitutional
//! `protocol`).
//!
//! `ImperativeMutation` has no shared kernel crate to implement `protocol::OpBinary` for it directly (see
//! `🗿️artifacts/📜️imperative/🦀️component.rs`'s module doc), so this component owns the full mirror-struct
//! machinery: `ImperativeMutationDsl` flattens `PathRef` into bare `owner`/`slot` fields (and the step
//! payload through the existing `StepNodeDsl`/`ValueDsl` mirrors) and routes through
//! `#[derive(dsl::DslEnum)]` for the actual text/binary codegen — `Step`/`Dictionary` are foreign kernel
//! types with no `dsl::DslRecord` support, so `ImperativeMutation`'s own payload structs (see the
//! `🧬️mutations/<slug>/🦠️mutation/` leaves) cannot derive it either.
//!
//! The app's typed `ImperativeCommand` enum — which used to share the old `📡️protocol` crate with this
//! codec — is an APP concern, not an artifact one: it now lives in the sibling `✏️editor` surface's
//! `🦀️component.rs`, assembled from the `🎮️commands/*` payload modules by
//! `semio_framework_plugin::app_commands!`.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::artifacts::imperative::dsl::{dictionary_to_value_dsl_map, step_node_dsl_to_step, step_to_step_node_dsl, value_dsl_map_to_dictionary, StepNodeDsl, ValueDsl};
use crate::artifacts::imperative::mutations::{create_step, delete_step, edit_step_params, reorder_steps, ImperativeMutation};
use crate::artifacts::imperative::PathRef;
use protocol::OpBinary;
use std::collections::BTreeMap;

//#region 🔖️OpText
/// ✂️ Local mirror of `ImperativeMutation` — flattens `PathRef` into bare `owner`/`slot`
/// `Option<String>` fields (printed bare when the value lexes as a bare ident, per the engine's
/// default `Shape::Text` behavior — no per-field opt-in needed) since a `store::Mutation` grammar is
/// a genuinely tagged enum (`#[derive(dsl::DslEnum)]` requires an enum), not the single generic
/// struct-plus-op-payload shape the old pre-migration mutation type used.
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
enum ImperativeMutationDsl {
    CreateStep {
        owner: Option<String>,
        slot: Option<String>,
        #[dsl(statements)]
        item: Box<StepNodeDsl>,
    },
    DeleteStep {
        owner: Option<String>,
        slot: Option<String>,
        id: String,
    },
    ReorderSteps {
        owner: Option<String>,
        slot: Option<String>,
        id: String,
        #[dsl(key = "to")]
        to_index: usize,
    },
    EditStepParams {
        owner: Option<String>,
        slot: Option<String>,
        id: String,
        params: BTreeMap<String, ValueDsl>,
    },
}

//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
impl protocol::OpText for ImperativeMutationDsl {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown mutation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl OpBinary for ImperativeMutationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs

fn imperative_operation_to_dsl(operation: &ImperativeMutation) -> ImperativeMutationDsl {
    match operation {
        ImperativeMutation::CreateStep(payload) => ImperativeMutationDsl::CreateStep { owner: payload.path_ref.owner.clone(), slot: payload.path_ref.slot.clone(), item: Box::new(step_to_step_node_dsl(&payload.step)) },
        ImperativeMutation::DeleteStep(payload) => ImperativeMutationDsl::DeleteStep { owner: payload.path_ref.owner.clone(), slot: payload.path_ref.slot.clone(), id: payload.id.clone() },
        ImperativeMutation::ReorderSteps(payload) => ImperativeMutationDsl::ReorderSteps { owner: payload.path_ref.owner.clone(), slot: payload.path_ref.slot.clone(), id: payload.id.clone(), to_index: payload.to_index },
        ImperativeMutation::EditStepParams(payload) => {
            ImperativeMutationDsl::EditStepParams { owner: payload.path_ref.owner.clone(), slot: payload.path_ref.slot.clone(), id: payload.id.clone(), params: dictionary_to_value_dsl_map(&payload.new_params) }
        }
    }
}

fn imperative_operation_from_dsl(dsl_op: ImperativeMutationDsl) -> ImperativeMutation {
    match dsl_op {
        ImperativeMutationDsl::CreateStep { owner, slot, item } => create_step(PathRef { owner, slot }, step_node_dsl_to_step(*item)),
        ImperativeMutationDsl::DeleteStep { owner, slot, id } => delete_step(PathRef { owner, slot }, id),
        ImperativeMutationDsl::ReorderSteps { owner, slot, id, to_index } => reorder_steps(PathRef { owner, slot }, id, to_index),
        ImperativeMutationDsl::EditStepParams { owner, slot, id, params } => edit_step_params(PathRef { owner, slot }, id, value_dsl_map_to_dictionary(&params)),
    }
}

impl protocol::OpText for ImperativeMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        Ok(imperative_operation_from_dsl(<ImperativeMutationDsl as protocol::OpText>::parse_op(line)?))
    }

    fn print_op(&self) -> String {
        <ImperativeMutationDsl as protocol::OpText>::print_op(&imperative_operation_to_dsl(self))
    }
}

/// ⚡️ Binary mirror of the `OpText` impl above — `ImperativeMutationDsl` already derives
/// `OpBinary` via `#[derive(dsl::DslEnum)]`, so this is a pure to/from-dsl forward.
impl OpBinary for ImperativeMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        imperative_operation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(imperative_operation_from_dsl(ImperativeMutationDsl::decode_op(bytes)?))
    }
}
//#endregion 🔖️OpText

//#region 🔖️Api
/// 📦️ Encodes an `ImperativeMutation` to its binary state-patch form.
pub fn encode_op(operation: &ImperativeMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes an `ImperativeMutation` from its binary state-patch form.
pub fn decode_op(bytes: &[u8]) -> Result<ImperativeMutation, protocol::ProtocolError> {
    ImperativeMutation::decode_op(bytes)
}
//#endregion 🔖️Api

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::imperative::ImperativeSnapshot;

    #[semio_framework_async_macros::async_test]
    async fn op_binary_round_trips_and_agrees_with_text() {
        let operation = delete_step(PathRef::default(), "step-1".into());
        store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[semio_framework_async_macros::async_test]
    async fn document_text_round_trip_with_applied_operation() {
        use crate::artifacts::imperative::{Dictionary, Step};
        use std::collections::BTreeMap;

        let document = crate::artifacts::imperative::schema::default_snapshot();
        let envelope = store::create_document_envelope::<ImperativeSnapshot, ImperativeMutation>("imperative.document/v1", "test", document, None);
        let mut doc_store = store::ArtifactStore::new(envelope).await.expect("valid artifact store fixture");
        let step = Step { id: "step-x".into(), kind: "log.print".into(), params: Dictionary::new(), bodies: BTreeMap::new() };
        let operation = create_step(PathRef::default(), step);
        doc_store.dispatch(store::ArtifactCommand::Apply { mutations: vec![operation], description: None }).await.expect("apply");
        store::os_store::test_support::assert_document_text_round_trip(&doc_store);
        store::os_store::test_support::assert_document_pack_round_trip(&doc_store);
    }

    #[semio_framework_async_macros::async_test]
    async fn op_text_rejects_unknown_operation_keyword() {
        let line = r#"frobnicate owner=- slot=- id="step-1""#;
        assert!(<ImperativeMutation as protocol::OpText>::parse_op(line).is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn op_text_round_trips_create_step_with_owner_and_slot() {
        use crate::artifacts::imperative::{Dictionary, Step};
        use std::collections::BTreeMap;
        let step = Step { id: "step-nested".into(), kind: "log.print".into(), params: Dictionary::new(), bodies: BTreeMap::new() };
        let operation = create_step(PathRef { owner: Some("step-if".into()), slot: Some("then".into()) }, step);
        let printed = <ImperativeMutation as protocol::OpText>::print_op(&operation);
        assert!(printed.contains("owner=step-if"), "printed: {printed}");
        assert!(printed.contains("slot=then"), "printed: {printed}");
        let parsed = <ImperativeMutation as protocol::OpText>::parse_op(&printed).expect("round trips");
        assert_eq!(parsed, operation);
    }

    #[semio_framework_async_macros::async_test]
    async fn op_text_round_trips_reorder_steps() {
        let operation = reorder_steps(PathRef::default(), "step-2".into(), 0);
        let printed = <ImperativeMutation as protocol::OpText>::print_op(&operation);
        let parsed = <ImperativeMutation as protocol::OpText>::parse_op(&printed).expect("round trips");
        assert_eq!(parsed, operation);
    }

    #[semio_framework_async_macros::async_test]
    async fn op_text_round_trips_edit_step_params() {
        use crate::artifacts::imperative::Dictionary;
        use neural_engine::{Atom, Value};
        let operation = edit_step_params(PathRef::default(), "step-2".into(), Dictionary::new().insert("message", Value::Atom(Atom::String("hi".into()))));
        let printed = <ImperativeMutation as protocol::OpText>::print_op(&operation);
        let parsed = <ImperativeMutation as protocol::OpText>::parse_op(&printed).expect("round trips");
        assert_eq!(parsed, operation);
    }
}
//#endregion 🧪️Tests
