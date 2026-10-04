//! 🧬️ Imperative snapshot schema — artifact-lane fields only.

use crate::{ProcedureFlowChild, ProcedureTextChild};
use framework_schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted imperative document snapshot (persistent fields of the artifact). Ticket
/// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`imperative→C:text,flow`): the inline `path:
/// Path` (the ordered/nested `Step` control-flow tree) and `seed: BTreeMap<String, Value>` (the
/// initial variable dictionary) content fields are replaced by two fixed composed CHILD slots —
/// this plugin no longer defines its own program-graph or seed-content model, it composes stdio's
/// `flow` and `text` subsets instead. `#[child(...)]` drives `#[derive(ArtifactSchema)]`'s
/// slot-table emission; never hand-written.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[artifact_schema(id = "s.imperative.procedure")]
pub struct ProcedureSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub flow: ProcedureFlowChild,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub text: ProcedureTextChild,
}

impl Default for ProcedureSnapshot {
    fn default() -> Self {
        crate::procedure_snapshot_naming(&crate::Path::new(), &std::collections::BTreeMap::new())
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️PackRecord
/// 🛤️ Derived pack record of a `ProcedureSnapshot`: the schema marker and both composed-child handles — the content lives in
/// the member stores (design §20.15), so text and pack carry addresses only.
#[derive(semio_framework_dsl_record_derive::DslRecord)]
#[dsl(extension = "imperative")]
struct ProcedurePackRecord {
    schema: String,
    flow: ProcedureFlowChild,
    text: ProcedureTextChild,
}

impl ProcedurePackRecord {
    fn snapshot_record_controlled(snapshot: &ProcedureSnapshot, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<semio_framework_dsl_record::RecordValue, semio_framework_value::ValueError> {
        control.scoped_depth(64, |control| control.scoped_stage(|control| {
            control.begin_stage(3)?;
            let mut record = semio_framework_dsl_record::native_encoding::EncodedRecord::new(3, control)?;
            record.insert(0, control.scoped_stage(|control| { control.begin_stage(0)?; <String as semio_framework_dsl_record::DslField>::to_value_controlled(&snapshot.schema, control) })?)?; control.step()?;
            record.insert(1, control.scoped_stage(|control| { control.begin_stage(0)?; <ProcedureFlowChild as semio_framework_dsl_record::DslField>::to_value_controlled(&snapshot.flow, control) })?)?; control.step()?;
            record.insert(2, control.scoped_stage(|control| { control.begin_stage(0)?; <ProcedureTextChild as semio_framework_dsl_record::DslField>::to_value_controlled(&snapshot.text, control) })?)?; control.step()?;
            Ok(record.take())
        }))
    }
    fn into_snapshot_controlled(self, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<ProcedureSnapshot, semio_framework_value::ValueError> {
        control.checkpoint()?;
        Ok(self.into_snapshot())
    }
    fn from_snapshot(snapshot: &ProcedureSnapshot) -> Self {
        Self { schema: snapshot.schema.clone(), flow: snapshot.flow.clone(), text: snapshot.text.clone() }
    }

    fn into_snapshot(self) -> ProcedureSnapshot {
        ProcedureSnapshot { schema: self.schema, flow: self.flow, text: self.text }
    }
}

/// 🖨️ The derived text body: the same `ProcedurePackRecord` the pack encodes, printed by the spec-driven engine.
pub(crate) fn print_pack_record_text(snapshot: &ProcedureSnapshot) -> String {
    semio_framework_dsl_record::print(&ProcedurePackRecord::from_snapshot(snapshot).__dsl_to_record(), &ProcedurePackRecord::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document)
}

/// 📖️ Parses a derived text body back through `ProcedurePackRecord`, with the same decode steps as the pack.
pub(crate) fn parse_pack_record_text(body: &str) -> Result<ProcedureSnapshot, semio_framework_diagnostic::TextError> {
    let record = semio_framework_dsl_record::parse(body, &ProcedurePackRecord::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
    Ok(ProcedurePackRecord::__dsl_from_record(&record)?.into_snapshot())
}
//#endregion 🔖️PackRecord

//#region 🔖️HandcraftedArtifactCodecs
/// 🎁 `ArtifactDsl` and `ArtifactPack` are the derived text and pack of `ProcedurePackRecord`.
impl store::ArtifactDsl for ProcedureSnapshot {
    const EXTENSION: &'static str = "imperative";
    fn envelope_id() -> &'static str {
        "imperative.imperative"
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_pack_record_text(body)
    }
    fn print_dsl(&self) -> String {
        let body = print_pack_record_text(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for ProcedureSnapshot {
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> { Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec()) }
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&ProcedurePackRecord::__dsl_spec(), &ProcedurePackRecord::from_snapshot(self).__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &ProcedurePackRecord::__dsl_spec(), options)?;
        Ok(ProcedurePackRecord::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)?.into_snapshot())
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(ProcedurePackRecord::__dsl_spec())
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🌉️ExternalCodecBridge
/// 📤️ Renders a [`ProcedureSnapshot`] as this facet's own camelCase JSON projection: the schema and the two
/// content-addressed child HANDLES, never content (first-party JSON codec behind this interface).
pub fn encode_procedure_snapshot_json(snapshot: &ProcedureSnapshot) -> String {
    semio_framework_pack_json::to_json_string(snapshot)
}

/// 📥️ The inverse of [`encode_procedure_snapshot_json`].
pub fn decode_procedure_snapshot_json(text: &str) -> Result<ProcedureSnapshot, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 📝️ Parses `.imperative.dsl.semio` text into a [`ProcedureSnapshot`] — a named pass-through of this type's own
/// `store::ArtifactDsl` impl, whose trait and error type are both unnameable outside this crate.
pub fn parse_procedure_dsl(text: &str) -> Result<ProcedureSnapshot, String> {
    <ProcedureSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

/// 📝️ Renders a [`ProcedureSnapshot`] back as `.imperative.dsl.semio` text — the inverse of [`parse_procedure_dsl`].
pub fn print_procedure_dsl(snapshot: &ProcedureSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}
//#endregion 🌉️ExternalCodecBridge


#[cfg(test)]
#[path="🧪️tests/🪶️sqlite/🦀️.rs"]
mod sqlite_snapshot_tests;

#[path="🪶️sqlite/🦀️.rs"]
pub mod sqlite;
