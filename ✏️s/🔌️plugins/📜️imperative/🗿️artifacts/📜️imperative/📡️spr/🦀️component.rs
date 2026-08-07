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


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


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

//#region 🔖️OpCodec
/// 🎞️ Handcrafted OpText (P6).
impl protocol::OpText for ImperativeOperationDsl {
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
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl protocol::OpBinary for ImperativeOperationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(protocol::ProtocolError::Malformed {
            what: "op variant",
            offset: 0,
            detail: format!("keyword {keyword:?} is not a declared variant"),
        })?;
        let spec = (variants[ordinal].1)();
        let body = store::pack_rt::encode_record_body(&spec, &record, &store::PackEncodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        store::pack_rt::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = store::pack_rt::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as dsl::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(protocol::ProtocolError::Malformed {
            what: "op variant",
            offset: 1,
            detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()),
        })?;
        let spec = spec_fn();
        let body = &bytes[reader.position()..];
        let (record, _report) = store::pack_rt::decode_record_body(body, &spec, &store::PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        <Self as dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| protocol::ProtocolError::Malformed {
            what: "op record",
            offset: reader.position() as u64,
            detail: error.to_string(),
        })
    }
}
//#endregion 🔖️OpCodec


fn imperative_operation_to_dsl(operation: &ImperativeOperation) -> ImperativeOperationDsl {
    let owner = operation.path_ref.owner.clone();
    let slot = operation.path_ref.slot.clone();
    match &operation.collection {
        // 🔒️ `id` is intentionally dropped in the DSL's `Add` shape (unchanged on-disk text
        // format) — `Step.id` round-trips it losslessly, recovered on the reverse conversion below.
        protocol::CollectionOperation::Add { index: at, item } => ImperativeOperationDsl::Add { owner, slot, index: *at, item: Box::new(step_to_step_node_dsl(item)) },
        protocol::CollectionOperation::Remove { id } => ImperativeOperationDsl::Remove { owner, slot, id: id.clone() },
        protocol::CollectionOperation::Move { id, to_index: to } => ImperativeOperationDsl::Move { owner, slot, id: id.clone(), to_index: *to },
        protocol::CollectionOperation::Patch { id, patch } => ImperativeOperationDsl::Patch { owner, slot, id: id.clone(), patch: dictionary_to_value_dsl_map(patch) },
    }
}

fn imperative_operation_from_dsl(dsl_op: ImperativeOperationDsl) -> ImperativeOperation {
    match dsl_op {
        ImperativeOperationDsl::Add { owner, slot, index, item } => {
            let item = step_node_dsl_to_step(*item);
            let id = item.id.clone();
            ImperativeOperation { path_ref: PathRef { owner, slot }, collection: protocol::CollectionOperation::Add { index: index, item } }
        }
        ImperativeOperationDsl::Remove { owner, slot, id } => ImperativeOperation { path_ref: PathRef { owner, slot }, collection: protocol::CollectionOperation::Remove { id } },
        ImperativeOperationDsl::Move { owner, slot, id, to_index } => ImperativeOperation { path_ref: PathRef { owner, slot }, collection: protocol::CollectionOperation::Move { id, to_index: to_index } },
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
        let operation = ImperativeOperation { path_ref: PathRef::default(), collection: protocol::CollectionOperation::Add { index: 0, item: step } };
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
        let operation = ImperativeOperation { path_ref: PathRef { owner: Some("step-if".into()), slot: Some("then".into()) }, collection: protocol::CollectionOperation::Add { index: 0, item } };
        let printed = <ImperativeOperation as protocol::OpText>::print_op(&operation);
        assert!(printed.contains("owner=step-if"), "printed: {printed}");
        assert!(printed.contains("slot=then"), "printed: {printed}");
        let parsed = <ImperativeOperation as protocol::OpText>::parse_op(&printed).expect("round trips");
        assert_eq!(parsed, operation);
    }
}
//#endregion 🧪️Tests
