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

/// 📨️ Decodes one text mutation or returns its unclaimed wire record.
type ProcedureMutationDecoder = fn(ProcedureMutationDsl) -> Result<ProcedureMutation, ProcedureMutationDsl>;

use crate::mutations::ProcedureMutation;
use protocol::OpBinary;

pub const BINARY_TAG_REGISTRY: &[(&str, u8)] =
    &[("create-step", super::create_step::binary::BINARY_TAG), ("delete-step", super::delete_step::binary::BINARY_TAG), ("reorder-steps", super::reorder_steps::binary::BINARY_TAG), ("edit-step-params", super::edit_step_params::binary::BINARY_TAG)];

//#region 🔖️OpText
/// ✂️ Ordered wire aggregate of direct leaf-owned records.
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
pub(crate) enum ProcedureMutationDsl {
    CreateStep(super::create_step::text::CreateStepText),
    DeleteStep(super::delete_step::text::DeleteStepText),
    ReorderSteps(super::reorder_steps::text::ReorderStepsText),
    EditStepParams(super::edit_step_params::text::EditStepParamsText),
}

//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
impl protocol::OpText for ProcedureMutationDsl {
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

impl OpBinary for ProcedureMutationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs

fn procedure_operation_to_dsl(operation: &ProcedureMutation) -> ProcedureMutationDsl {
    let converters: &[fn(&ProcedureMutation) -> Option<ProcedureMutationDsl>] = &[super::create_step::text::to_dsl, super::delete_step::text::to_dsl, super::reorder_steps::text::to_dsl, super::edit_step_params::text::to_dsl];
    converters.iter().find_map(|convert| convert(operation)).expect("every mutation has a direct text owner")
}

fn procedure_operation_from_dsl(dsl_op: ProcedureMutationDsl) -> ProcedureMutation {
    let converters: &[ProcedureMutationDecoder] = &[super::create_step::text::from_dsl, super::delete_step::text::from_dsl, super::reorder_steps::text::from_dsl, super::edit_step_params::text::from_dsl];
    let mut wire = dsl_op;
    for convert in converters {
        match convert(wire) {
            Ok(operation) => return operation,
            Err(unclaimed) => wire = unclaimed,
        }
    }
    unreachable!("every wire record has a direct mutation owner")
}

impl protocol::OpText for ProcedureMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        Ok(procedure_operation_from_dsl(<ProcedureMutationDsl as protocol::OpText>::parse_op(line)?))
    }

    fn print_op(&self) -> String {
        <ProcedureMutationDsl as protocol::OpText>::print_op(&procedure_operation_to_dsl(self))
    }
}

/// ⚡️ Binary mirror of the `OpText` impl above — `ProcedureMutationDsl` already derives
/// `OpBinary` via `#[derive(dsl::DslEnum)]`, so this is a pure to/from-dsl forward.
impl OpBinary for ProcedureMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        procedure_operation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(procedure_operation_from_dsl(ProcedureMutationDsl::decode_op(bytes)?))
    }
}
//#endregion 🔖️OpText

//#region 🔖️Api
/// 📦️ Encodes an `ProcedureMutation` to its binary state-patch form.
pub fn encode_op(operation: &ProcedureMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes an `ProcedureMutation` from its binary state-patch form.
pub fn decode_op(bytes: &[u8]) -> Result<ProcedureMutation, protocol::ProtocolError> {
    ProcedureMutation::decode_op(bytes)
}
//#endregion 🔖️Api

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
