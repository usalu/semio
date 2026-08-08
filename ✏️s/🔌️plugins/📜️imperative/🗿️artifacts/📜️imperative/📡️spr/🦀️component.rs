//! ⚖️ Imperative artifact — state-patch-representation wire codec + laws (was: constitutional
//! `protocol`).
//!
//! `ImperativeMutation` has no shared kernel crate to implement `protocol::OpBinary` for it directly (see
//! `🗿️artifacts/📜️imperative/🦀️component.rs`'s module doc), so this component owns the full mirror-struct
//! machinery: `ImperativeMutationDsl` flattens `PathRef` into bare `owner`/`slot` fields and routes
//! through `#[derive(dsl::DslEnum)]` for the actual text/binary codegen.
//!
//! The app's typed `ImperativeCommand` enum — which used to share the old `📡️protocol` crate with this
//! codec — is an APP concern, not an artifact one: it now lives in `🎛️apps/📜️imperative/🦀️component.rs`,
//! assembled from the `🎮️commands/*` payload modules by `semio_framework_plugin::app_commands!`.


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::imperative::dsl::{dictionary_to_value_dsl_map, step_node_dsl_to_step, step_to_step_node_dsl, value_dsl_map_to_dictionary, StepNodeDsl, ValueDsl};
use crate::artifacts::imperative::mutations::ImperativeMutation;
use crate::artifacts::imperative::PathRef;
use protocol::OpBinary;

//#region 🔖️OpText
/// ✂️ Local mirror of `ImperativeMutation` — flattens `PathRef` into bare `owner`/`slot`
/// `Option<String>` fields (printed bare when the value lexes as a bare ident, per the engine's
/// default `Shape::Text` behavior — no per-field opt-in needed) since a `store::Mutation` grammar is
/// a genuinely tagged enum (`#[derive(dsl::DslEnum)]` requires an enum), not the single generic-struct
/// shape `ImperativeMutation`/`protocol::CollectionMutation` use at the Rust level.
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
enum ImperativeMutationDsl {
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
//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
impl protocol::OpText for ImperativeMutationDsl {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(
                    line,
                    &spec_fn(),
                    &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline },
                )?;
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

impl protocol::OpBinary for ImperativeMutationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs




fn imperative_operation_to_dsl(operation: &ImperativeMutation) -> ImperativeMutationDsl {
    let owner = operation.path_ref.owner.clone();
    let slot = operation.path_ref.slot.clone();
    match &operation.collection {
        // 🔒️ `id` is intentionally dropped in the DSL's `Add` shape (unchanged on-disk text
        // format) — `Step.id` round-trips it losslessly, recovered on the reverse conversion below.
        protocol::CollectionMutation::Add { index: at, item } => ImperativeMutationDsl::Add { owner, slot, index: *at, item: Box::new(step_to_step_node_dsl(item)) },
        protocol::CollectionMutation::Remove { id } => ImperativeMutationDsl::Remove { owner, slot, id: id.clone() },
        protocol::CollectionMutation::Move { id, to_index: to } => ImperativeMutationDsl::Move { owner, slot, id: id.clone(), to_index: *to },
        protocol::CollectionMutation::Patch { id, patch } => ImperativeMutationDsl::Patch { owner, slot, id: id.clone(), patch: dictionary_to_value_dsl_map(patch) },
    }
}

fn imperative_operation_from_dsl(dsl_op: ImperativeMutationDsl) -> ImperativeMutation {
    match dsl_op {
        ImperativeMutationDsl::Add { owner, slot, index, item } => {
            let item = step_node_dsl_to_step(*item);
            let id = item.id.clone();
            ImperativeMutation { path_ref: PathRef { owner, slot }, collection: protocol::CollectionMutation::Add { index: index, item } }
        }
        ImperativeMutationDsl::Remove { owner, slot, id } => ImperativeMutation { path_ref: PathRef { owner, slot }, collection: protocol::CollectionMutation::Remove { id } },
        ImperativeMutationDsl::Move { owner, slot, id, to_index } => ImperativeMutation { path_ref: PathRef { owner, slot }, collection: protocol::CollectionMutation::Move { id, to_index: to_index } },
        ImperativeMutationDsl::Patch { owner, slot, id, patch } => ImperativeMutation { path_ref: PathRef { owner, slot }, collection: protocol::CollectionMutation::Patch { id, patch: value_dsl_map_to_dictionary(&patch) } },
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
    use crate::artifacts::imperative::ImperativeDocument;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = ImperativeMutation { path_ref: PathRef::default(), collection: protocol::CollectionMutation::Remove { id: "step-1".into() } };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn document_text_round_trip_with_applied_operation() {
        use crate::artifacts::imperative::{Dictionary, Step};
        use std::collections::BTreeMap;

        let document = crate::artifacts::imperative::engine::default_document();
        let envelope = store::create_document_envelope::<ImperativeDocument, ImperativeMutation>("imperative.document/v1", "test", document, None);
        let mut doc_store = store::DocumentStore::new(envelope);
        let step = Step { id: "step-x".into(), kind: "log.print".into(), params: Dictionary::new(), bodies: BTreeMap::new() };
        let operation = ImperativeMutation { path_ref: PathRef::default(), collection: protocol::CollectionMutation::Add { index: 0, item: step } };
        doc_store.dispatch(store::DocumentCommand::Apply { mutations: vec![operation], description: None }).expect("apply");
        store::test_support::assert_document_text_round_trip(&doc_store);
        store::test_support::assert_document_pack_round_trip(&doc_store);
    }

    #[test]
    fn op_text_rejects_unknown_operation_keyword() {
        let line = r#"frobnicate owner=- slot=- id="step-1""#;
        assert!(<ImperativeMutation as protocol::OpText>::parse_op(line).is_err());
    }

    #[test]
    fn op_text_round_trips_add_with_owner_and_slot() {
        use crate::artifacts::imperative::{Dictionary, Step};
        use std::collections::BTreeMap;
        let item = Step { id: "step-nested".into(), kind: "log.print".into(), params: Dictionary::new(), bodies: BTreeMap::new() };
        let operation = ImperativeMutation { path_ref: PathRef { owner: Some("step-if".into()), slot: Some("then".into()) }, collection: protocol::CollectionMutation::Add { index: 0, item } };
        let printed = <ImperativeMutation as protocol::OpText>::print_op(&operation);
        assert!(printed.contains("owner=step-if"), "printed: {printed}");
        assert!(printed.contains("slot=then"), "printed: {printed}");
        let parsed = <ImperativeMutation as protocol::OpText>::parse_op(&printed).expect("round trips");
        assert_eq!(parsed, operation);
    }
}
//#endregion 🧪️Tests
