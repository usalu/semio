//! 🚪️ Native artifact representation codecs.
use super::super::super::*;

/// 📦️ Direct actual persisted fields provide ordinary and paid native endpoints.
impl crate::os_store::serialization::ArtifactPack for DagSnapshot {
 /// 📣️ Publishes this actual owner through ordinary Store construction and hydration.
 fn native_snapshot_registration() -> Option<(semio_framework_artifact_reference::Dialect, crate::os_store::serialization::ArtifactCodec)> {
     Some((semio_framework_artifact_reference::Dialect { artifact_kind: "dag.host_snapshot", standard: semio_framework_artifact_reference::StandardId("1"), subset: semio_framework_artifact_reference::SubsetId("*") }, crate::os_store::bare_artifact_codec::<Self, DagMutation>(DAG_DOCUMENT_SCHEMA)))
 }
 fn sqlite_snapshot_codec()->Option<crate::os_store::serialization::ArtifactSqliteSnapshotCodec>{Some(<Self as crate::os_store::serialization::ArtifactSqliteSnapshot>::sqlite_codec())}
 fn encode_pack_with(&self,options:&crate::os_store::PackEncodeOptions)->Result<Vec<u8>,crate::os_store::PackError>{
  let inner=crate::os_pack::pack_rt::encode_document(&Self::__dsl_spec(),&self.__dsl_to_record(),options)?;
  let envelope=crate::os_store::semio_format::SemioEnvelope::from_envelope_id(<Self as crate::os_store::serialization::ArtifactDsl>::envelope_id(),crate::os_store::semio_format::Component::Pack,1).map_err(|error| crate::os_store::PackError::from(error.into_value_error()))?;
  Ok(crate::os_store::semio_format::wrap_binary(&envelope,&inner))
 }
 fn decode_pack_with(bytes:&[u8],options:&crate::os_store::PackDecodeOptions)->Result<Self,crate::os_store::PackError>{
  let(envelope,inner)=crate::os_store::semio_format::unwrap_binary(bytes).map_err(|error| crate::os_store::PackError::from(error.into_value_error()))?;
  if !envelope.matches_identity(<Self as crate::os_store::serialization::ArtifactDsl>::envelope_id(),crate::os_store::semio_format::Component::Pack,1){return Err(crate::os_store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "actual DAG pack envelope mismatch")))}
  let(record,_)=crate::os_pack::pack_rt::decode_document(&inner,&Self::__dsl_spec(),options)?;
  Self::__dsl_from_record(&record).map_err(crate::os_store::serialization::text_error_to_pack_error)
 }
 fn record_spec()->Option<semio_framework_dsl_record::RecordSpec>{Some(Self::__dsl_spec())}
}
