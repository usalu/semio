//! 🚪️ Native artifact representation codecs.
use super::super::super::*;
use crate::io::text::snapshot::*;

/// 📦️ Handcrafted ArtifactPack (P6).
impl crate::os_store::serialization::ArtifactPack for FlowHostSnapshotDsl {
    fn encode_pack_with(&self, options: &crate::os_store::PackEncodeOptions) -> Result<Vec<u8>, crate::os_store::PackError> {
        let inner = crate::os_pack::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope =
            crate::os_store::semio_format::SemioEnvelope::from_envelope_id(<Self as crate::os_store::serialization::ArtifactDsl>::envelope_id(), crate::os_store::semio_format::Component::Pack, 1).map_err(|e| crate::os_store::PackError::from(e.into_value_error()))?;
        Ok(crate::os_store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &crate::os_store::PackDecodeOptions) -> Result<Self, crate::os_store::PackError> {
        let (envelope, inner) = crate::os_store::semio_format::unwrap_binary(bytes).map_err(|e| crate::os_store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as crate::os_store::serialization::ArtifactDsl>::envelope_id(), crate::os_store::semio_format::Component::Pack, 1) {
            return Err(crate::os_store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as crate::os_store::serialization::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = crate::os_pack::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(crate::os_store::serialization::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

/// 🗜️ `FlowHostSnapshot` has no `#[derive(crate::os_dsl::DslArtifact)]` of its own (see `FlowHostSnapshotDsl`'s doc
/// comment above), so it doesn't automatically gain `crate::os_store::serialization::ArtifactPack` the way every derived type
/// does — this hand-written twin of the `crate::os_store::serialization::ArtifactDsl` impl just above delegates through the
/// same `flow_host_snapshot_to_dsl`/`flow_host_snapshot_dsl_to_host_snapshot` mirror instead of `__dsl_to_record`/
/// `__dsl_from_record`.
impl crate::os_store::serialization::ArtifactPack for FlowHostSnapshot {
    /// 🌊️ Publishes the actual persisted Flow owner when its Store opens or hydrates.
    fn native_snapshot_registration() -> Option<(semio_framework_artifact_reference::Dialect, crate::os_store::serialization::ArtifactCodec)> {
        Some((semio_framework_artifact_reference::Dialect { artifact_kind: "flow.host_snapshot", standard: semio_framework_artifact_reference::StandardId("1"), subset: semio_framework_artifact_reference::SubsetId("*") }, crate::os_store::bare_artifact_codec::<Self, FlowMutation>(FLOW_DOCUMENT_SCHEMA)))
    }
    fn sqlite_snapshot_codec() -> Option<crate::os_store::serialization::ArtifactSqliteSnapshotCodec> { Some(<Self as crate::os_store::serialization::ArtifactSqliteSnapshot>::sqlite_codec()) }
    fn encode_pack_with(&self, options: &crate::os_store::PackEncodeOptions) -> Result<Vec<u8>, crate::os_store::PackError> {
        <FlowHostSnapshotDsl as crate::os_store::serialization::ArtifactPack>::encode_pack_with(&flow_host_snapshot_to_dsl(self), options)
    }

    fn decode_pack_with(bytes: &[u8], options: &crate::os_store::PackDecodeOptions) -> Result<Self, crate::os_store::PackError> {
        let dsl_fixture = <FlowHostSnapshotDsl as crate::os_store::serialization::ArtifactPack>::decode_pack_with(bytes, options)?;
        flow_host_snapshot_dsl_to_host_snapshot(dsl_fixture).map_err(|message| crate::os_store::serialization::text_error_to_pack_error(crate::os_store::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,message, crate::os_store::TextSpan::at(1, 1))))
    }

    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        <FlowHostSnapshotDsl as crate::os_store::serialization::ArtifactPack>::record_spec()
    }
}
