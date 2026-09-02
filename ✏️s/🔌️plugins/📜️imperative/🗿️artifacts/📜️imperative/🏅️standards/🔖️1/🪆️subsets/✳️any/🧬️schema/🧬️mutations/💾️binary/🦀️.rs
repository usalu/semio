//! ⚖️ Imperative artifact — state-patch-representation wire codec + laws (was: constitutional
//! `protocol`).
//!
//! Each direct text leaf owns its flattened wire record and domain conversion. This aggregate
//! keeps only framing and ordered registry lookup; declaration order preserves the binary tags.
//!
//! The app's typed `ImperativeCommand` enum — which used to share the old `📡️protocol` crate with this
//! codec — is an APP concern, not an artifact one: it now lives in the sibling `✏️editor` surface's
//! `🦀️.rs`, assembled from the `🎮️commands/*` payload modules by
//! `semio_framework_plugin::app_commands!`.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::artifacts::imperative::mutations::ImperativeMutation;
use protocol::OpBinary;

pub const BINARY_TAG_REGISTRY: &[(&str, u8)] =
    &[("create-step", super::create_step::binary::BINARY_TAG), ("delete-step", super::delete_step::binary::BINARY_TAG), ("reorder-steps", super::reorder_steps::binary::BINARY_TAG), ("edit-step-params", super::edit_step_params::binary::BINARY_TAG)];

//#region 🔖️OpText
/// ✂️ Ordered wire aggregate of direct leaf-owned records.
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
pub(crate) enum ImperativeMutationDsl {
    CreateStep(super::create_step::text::CreateStepText),
    DeleteStep(super::delete_step::text::DeleteStepText),
    ReorderSteps(super::reorder_steps::text::ReorderStepsText),
    EditStepParams(super::edit_step_params::text::EditStepParamsText),
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
    let converters: &[fn(&ImperativeMutation) -> Option<ImperativeMutationDsl>] = &[super::create_step::text::to_dsl, super::delete_step::text::to_dsl, super::reorder_steps::text::to_dsl, super::edit_step_params::text::to_dsl];
    converters.iter().find_map(|convert| convert(operation)).expect("every mutation has a direct text owner")
}

fn imperative_operation_from_dsl(dsl_op: ImperativeMutationDsl) -> ImperativeMutation {
    let converters: &[fn(ImperativeMutationDsl) -> Result<ImperativeMutation, ImperativeMutationDsl>] =
        &[super::create_step::text::from_dsl, super::delete_step::text::from_dsl, super::reorder_steps::text::from_dsl, super::edit_step_params::text::from_dsl];
    converters.iter().fold(Err(dsl_op), |operation, convert| operation.or_else(convert)).expect("every wire record has a direct mutation owner")
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
    use crate::artifacts::imperative::mutations::{create_step, delete_step, edit_step_params, reorder_steps};
    use crate::artifacts::imperative::{ImperativeSnapshot, PathRef};

    #[test]
    fn direct_wire_records_preserve_keyword_fields_and_tag_order() {
        let expected = [("create-step", &["owner", "slot", "item"][..]), ("delete-step", &["owner", "slot", "id"][..]), ("reorder-steps", &["owner", "slot", "id", "to"][..]), ("edit-step-params", &["owner", "slot", "id", "params"][..])];
        let variants = <ImperativeMutationDsl as dsl::DslVariants>::variants();
        assert_eq!(variants.len(), expected.len());
        for (index, ((keyword, spec), (expected_keyword, expected_fields))) in variants.iter().zip(expected.iter()).enumerate() {
            assert_eq!(keyword.as_str(), *expected_keyword);
            assert_eq!(BINARY_TAG_REGISTRY[index], (*expected_keyword, index as u8));
            let fields: Vec<_> = spec().fields.into_iter().map(|field| field.key).collect();
            assert_eq!(fields, expected_fields.iter().map(|field| (*field).to_owned()).collect::<Vec<_>>());
        }
    }

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
