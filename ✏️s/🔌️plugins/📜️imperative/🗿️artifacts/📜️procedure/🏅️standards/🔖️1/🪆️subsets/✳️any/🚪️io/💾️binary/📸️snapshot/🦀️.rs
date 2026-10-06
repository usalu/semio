//! binary rep for stdio.json 📸️snapshot

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::{ProcedureFlowChild, ProcedureTextChild};
use framework_schema::ArtifactSchema;

/// 🛤️ Derived pack record of a `ProcedureSnapshot`: the schema marker and both composed-child handles — the content lives in
/// the member stores (design §20.15), so text and pack carry addresses only.
#[derive(semio_framework_dsl_record_derive::DslRecord)]
#[dsl(extension = "imperative")]
pub(crate) struct ProcedurePackRecord {
    schema: String,
    flow: ProcedureFlowChild,
    text: ProcedureTextChild,
}

impl ProcedurePackRecord {
    pub(crate) fn snapshot_record_controlled(snapshot: &ProcedureSnapshot, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<semio_framework_dsl_record::RecordValue, semio_framework_value::ValueError> {
        control.scoped_depth(64, |control| control.scoped_stage(|control| {
            control.begin_stage(3)?;
            let mut record = semio_framework_dsl_record::native_encoding::EncodedRecord::new(3, control)?;
            record.insert(0, control.scoped_stage(|control| { control.begin_stage(0)?; <String as semio_framework_dsl_record::DslField>::to_value_controlled(&snapshot.schema, control) })?)?; control.step()?;
            record.insert(1, control.scoped_stage(|control| { control.begin_stage(0)?; <ProcedureFlowChild as semio_framework_dsl_record::DslField>::to_value_controlled(&snapshot.flow, control) })?)?; control.step()?;
            record.insert(2, control.scoped_stage(|control| { control.begin_stage(0)?; <ProcedureTextChild as semio_framework_dsl_record::DslField>::to_value_controlled(&snapshot.text, control) })?)?; control.step()?;
            Ok(record.take())
        }))
    }
    pub(crate) fn into_snapshot_controlled(self, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<ProcedureSnapshot, semio_framework_value::ValueError> {
        control.checkpoint()?;
        Ok(self.into_snapshot())
    }
    pub(crate) fn from_snapshot(snapshot: &ProcedureSnapshot) -> Self {
        Self { schema: snapshot.schema.clone(), flow: snapshot.flow.clone(), text: snapshot.text.clone() }
    }

    pub(crate) fn into_snapshot(self) -> ProcedureSnapshot {
        ProcedureSnapshot { schema: self.schema, flow: self.flow, text: self.text }
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
}
pub use snapshot_codec::*;
