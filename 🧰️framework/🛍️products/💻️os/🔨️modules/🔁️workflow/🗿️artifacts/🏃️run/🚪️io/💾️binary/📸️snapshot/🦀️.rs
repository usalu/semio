//! 🚪️ Native artifact representation codecs.
use super::super::super::*;

/// 📦️ Handcrafted ArtifactPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::ArtifactPack for RunArtifact {
    /// 📣️ Publishes this actual owner through ordinary Store construction and hydration.
    fn native_snapshot_registration() -> Option<(semio_framework_artifact_reference::Dialect, store::ArtifactCodec)> {
        Some((semio_framework_artifact_reference::Dialect { artifact_kind: "os.run", standard: semio_framework_artifact_reference::StandardId("1"), subset: semio_framework_artifact_reference::SubsetId("*") }, store::ArtifactCodec::bare::<Self, crate::RunMutation>(S_RUN_SCHEMA)))
    }
    /// 📣️ Publishes this artifact-owned native codec at its declared lifecycle boundary.
    fn publish_native_snapshot() -> Result<(),store::io::ArtifactAssemblyRegistryError> {
        store::io::register_native_snapshot_codec(semio_framework_artifact_reference::Dialect { artifact_kind: "os.run", standard: semio_framework_artifact_reference::StandardId("1"), subset: semio_framework_artifact_reference::SubsetId("*") }, store::ArtifactCodec::bare::<Self, crate::RunMutation>(S_RUN_SCHEMA))
    }
    fn sqlite_snapshot_codec()->Option<store::ArtifactSqliteSnapshotCodec>{Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())}
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        match Self::__dsl_from_record(&record) {
            Ok(value) => Ok(value),
            Err(error) => Err(store::text_error_to_pack_error(error)),
        }
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
