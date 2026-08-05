//! ⚖️ Imperative artifact — state-patch-representation wire codec + laws (was: constitutional
//! `protocol`).
//!
//! `ImperativeOperation` has no shared kernel crate to implement `protocol::OpBinary` for it directly (see
//! `🗿️artifacts/📜️imperative/🦀️component.rs`'s module doc), so this component owns the full mirror-struct
//! machinery: `ImperativeOperationDsl` flattens `PathRef` into bare `owner`/`slot` fields and routes
//! through `#[derive(dsl::DslOps)]` for the actual text/binary codegen.
//!
//! The app's typed `ImperativeCommand` enum — which used to share the old `📡️protocol` crate with this
//! codec — is an APP concern, not an artifact one: it now lives in `🎛️apps/📜️imperative/🦀️component.rs`,
//! assembled from the `🎮️commands/*` payload modules by `semio_framework_plugin::app_commands!`.

use crate::artifacts::imperative::dsl::{dictionary_to_value_dsl_map, step_node_dsl_to_step, step_to_step_node_dsl, value_dsl_map_to_dictionary, StepNodeDsl, ValueDsl};
use crate::artifacts::imperative::op::ImperativeOperation;
use crate::artifacts::imperative::PathRef;
use protocol::OpBinary;

//#region 🔖️OpText
/// ✂️ Local mirror of `ImperativeOperation` — flattens `PathRef` into bare `owner`/`slot`
/// `Option<String>` fields (printed bare when the value lexes as a bare ident, per the engine's
/// default `Shape::Text` behavior — no per-field opt-in needed) since a `store::Operation` grammar is
/// a genuinely tagged enum (`#[derive(dsl::DslOps)]` requires an enum), not the single generic-struct
/// shape `ImperativeOperation`/`protocol::CollectionOperation` use at the Rust level.
#[derive(Clone, Debug, PartialEq, dsl::DslOps)]
enum ImperativeOperationDsl {
    Add {
        owner: Option<String>,
        slot: Option<String>,
        index: usize,
        #[dsl(statements)]
        item: Box<StepNodeDsl>,
    },
    Remove {
        owner: Option<String>,
        slot: Option<String>,
        id: String,
    },
    Move {
        owner: Option<String>,
        slot: Option<String>,
        id: String,
        #[dsl(key = "to")]
        to_index: usize,
    },
    Patch {
        owner: Option<String>,
        slot: Option<String>,
        id: String,
        patch: std::collections::BTreeMap<String, ValueDsl>,
    },
}

fn imperative_operation_to_dsl(operation: &ImperativeOperation) -> ImperativeOperationDsl {
    let owner = operation.path_ref.owner.clone();
    let slot = operation.path_ref.slot.clone();
    match &operation.collection {
        // 🔒️ `id` is intentionally dropped in the DSL's `Add` shape (unchanged on-disk text
        // format) — `Step.id` round-trips it losslessly, recovered on the reverse conversion below.
        protocol::CollectionOperation::Add { id: _id, item, at } => ImperativeOperationDsl::Add { owner, slot, index: *at, item: Box::new(step_to_step_node_dsl(item)) },
        protocol::CollectionOperation::Remove { id } => ImperativeOperationDsl::Remove { owner, slot, id: id.clone() },
        protocol::CollectionOperation::Move { id, to } => ImperativeOperationDsl::Move { owner, slot, id: id.clone(), to_index: *to },
        protocol::CollectionOperation::Patch { id, patch } => ImperativeOperationDsl::Patch { owner, slot, id: id.clone(), patch: dictionary_to_value_dsl_map(patch) },
    }
}

fn imperative_operation_from_dsl(dsl_op: ImperativeOperationDsl) -> ImperativeOperation {
    match dsl_op {
        ImperativeOperationDsl::Add { owner, slot, index, item } => {
            let item = step_node_dsl_to_step(*item);
            let id = item.id.clone();
            ImperativeOperation { path_ref: PathRef { owner, slot }, collection: protocol::CollectionOperation::Add { id, item, at: index } }
        }
        ImperativeOperationDsl::Remove { owner, slot, id } => ImperativeOperation { path_ref: PathRef { owner, slot }, collection: protocol::CollectionOperation::Remove { id } },
        ImperativeOperationDsl::Move { owner, slot, id, to_index } => ImperativeOperation { path_ref: PathRef { owner, slot }, collection: protocol::CollectionOperation::Move { id, to: to_index } },
        ImperativeOperationDsl::Patch { owner, slot, id, patch } => ImperativeOperation { path_ref: PathRef { owner, slot }, collection: protocol::CollectionOperation::Patch { id, patch: value_dsl_map_to_dictionary(&patch) } },
    }
}

impl protocol::OpText for ImperativeOperation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        Ok(imperative_operation_from_dsl(<ImperativeOperationDsl as protocol::OpText>::parse_op(line)?))
    }

    fn print_op(&self) -> String {
        <ImperativeOperationDsl as protocol::OpText>::print_op(&imperative_operation_to_dsl(self))
    }
}

/// ⚡️ Binary mirror of the `OpText` impl above — `ImperativeOperationDsl` already derives
/// `OpBinary` via `#[derive(dsl::DslOps)]`, so this is a pure to/from-dsl forward.
impl OpBinary for ImperativeOperation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        imperative_operation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(imperative_operation_from_dsl(ImperativeOperationDsl::decode_op(bytes)?))
    }
}
//#endregion 🔖️OpText

//#region 🔖️Api
/// 📦️ Encodes an `ImperativeOperation` to its binary state-patch form.
pub fn encode_op(operation: &ImperativeOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes an `ImperativeOperation` from its binary state-patch form.
pub fn decode_op(bytes: &[u8]) -> Result<ImperativeOperation, protocol::ProtocolError> {
    ImperativeOperation::decode_op(bytes)
}
//#endregion 🔖️Api

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::imperative::ImperativeDocument;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = ImperativeOperation { path_ref: PathRef::default(), collection: protocol::CollectionOperation::Remove { id: "step-1".into() } };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn document_text_round_trip_with_applied_operation() {
        use crate::artifacts::imperative::{Dictionary, Step};
        use std::collections::BTreeMap;

        let document = crate::artifacts::imperative::engine::default_document();
        let envelope = store::create_document_envelope::<ImperativeDocument, ImperativeOperation>("imperative.document/v1", "test", document, None);
        let mut doc_store = store::DocumentStore::new(envelope);
        let step = Step { id: "step-x".into(), kind: "log.print".into(), params: Dictionary::new(), bodies: BTreeMap::new() };
        let operation = ImperativeOperation { path_ref: PathRef::default(), collection: protocol::CollectionOperation::Add { id: "step-x".to_string(), item: step, at: 0 } };
        doc_store.dispatch(store::DocumentCommand::Apply { operations: vec![operation], description: None }).expect("apply");
        store::test_support::assert_document_text_round_trip(&doc_store);
        store::test_support::assert_document_pack_round_trip(&doc_store);
    }

    #[test]
    fn op_text_rejects_unknown_operation_keyword() {
        let line = r#"frobnicate owner=- slot=- id="step-1""#;
        assert!(<ImperativeOperation as protocol::OpText>::parse_op(line).is_err());
    }

    #[test]
    fn op_text_round_trips_add_with_owner_and_slot() {
        use crate::artifacts::imperative::{Dictionary, Step};
        use std::collections::BTreeMap;
        let item = Step { id: "step-nested".into(), kind: "log.print".into(), params: Dictionary::new(), bodies: BTreeMap::new() };
        let operation = ImperativeOperation { path_ref: PathRef { owner: Some("step-if".into()), slot: Some("then".into()) }, collection: protocol::CollectionOperation::Add { id: "step-nested".to_string(), item, at: 0 } };
        let printed = <ImperativeOperation as protocol::OpText>::print_op(&operation);
        assert!(printed.contains("owner=step-if"), "printed: {printed}");
        assert!(printed.contains("slot=then"), "printed: {printed}");
        let parsed = <ImperativeOperation as protocol::OpText>::parse_op(&printed).expect("round trips");
        assert_eq!(parsed, operation);
    }
}
//#endregion 🧪️Tests
