//! 🚪️ Native artifact representation codecs.
use super::super::super::*;

/// 📦️ Handcrafted ArtifactPack (P6).
impl pack::snapshot::ArtifactPack for FlowHostSnapshot {
    fn encode_pack_with(&self, options: &crate::os_store::PackEncodeOptions) -> Result<Vec<u8>, crate::os_store::PackError> {
        let inner = crate::os_pack::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope =
            crate::os_store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), crate::os_store::semio_format::Component::Pack, 1).map_err(|e| crate::os_store::PackError::from(e.into_value_error()))?;
        Ok(crate::os_store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &crate::os_store::PackDecodeOptions) -> Result<Self, crate::os_store::PackError> {
        let (envelope, inner) = crate::os_store::semio_format::unwrap_binary(bytes).map_err(|e| crate::os_store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), crate::os_store::semio_format::Component::Pack, 1) {
            return Err(crate::os_store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as pack::snapshot::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = crate::os_pack::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(crate::os_store::serialization::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for FlowHostSnapshot {
    fn reference_snapshot_codec()->Option<::semio_framework_os_kernel::os_store::serialization::ArtifactReferenceSnapshotCodec<Self>>{Some(::semio_framework_os_kernel::os_store::serialization::reference_snapshot_codec::<Self>())}

fn native_snapshot_registration() -> Option<(semio_framework_artifact_reference::Dialect, crate::os_store::serialization::ArtifactCodec)> {
        Some((semio_framework_artifact_reference::Dialect { artifact_kind: "flow.host_snapshot", standard: semio_framework_artifact_reference::StandardId("1"), subset: semio_framework_artifact_reference::SubsetId("*") }, crate::os_store::bare_artifact_codec::<Self, FlowMutation>(FLOW_DOCUMENT_SCHEMA)))
    }
fn sqlite_snapshot_codec() -> Option<crate::os_store::serialization::ArtifactSqliteSnapshotCodec> { Some(<Self as semio_framework_os_kernel::os_store::ArtifactSqliteSnapshotOperations>::sqlite_codec()) }
}

/// 🌊️ Child-owned persisted Flow record codec and original field construction.
#[derive(Clone,Copy)]
pub struct FlowSnapshotPackProducer;
impl FlowSnapshotPackProducer {
 fn encode_snapshot(snapshot:&FlowHostSnapshot,options:&crate::os_store::PackEncodeOptions)->Result<Vec<u8>,crate::os_store::PackError>{<FlowHostSnapshot as pack::snapshot::ArtifactPack>::encode_pack_with(snapshot,options)}
 fn decode_snapshot(bytes:&[u8],options:&crate::os_store::PackDecodeOptions)->Result<FlowHostSnapshot,crate::os_store::PackError>{
  <FlowHostSnapshot as pack::snapshot::ArtifactPack>::decode_pack_with(bytes,options)
 }
}
impl crate::os_store::MemberSnapshotDecoder<FlowHostSnapshot> for FlowSnapshotPackProducer {
 fn decode_snapshot(bytes:&[u8])->Result<FlowHostSnapshot,crate::os_store::PackError>{Self::decode_snapshot(bytes,&crate::os_store::PackDecodeOptions::default())}
}
impl ::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactGenesisProducer<FlowHostSnapshot> for FlowSnapshotPackProducer {
 fn encode_genesis_pack(&self,snapshot:&FlowHostSnapshot)->Result<Vec<u8>,semio_framework_value::ValueError>{Self::encode_snapshot(snapshot,&crate::os_store::PackEncodeOptions::default()).map_err(|error|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error.to_string()))}
 fn decode_genesis_pack(&self,bytes:&[u8])->Result<FlowHostSnapshot,semio_framework_value::ValueError>{Self::decode_snapshot(bytes,&crate::os_store::PackDecodeOptions::default()).map_err(|error|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error.to_string()))}
}

impl crate::os_store::ArtifactStorePublication<FlowHostSnapshot> for FlowSnapshotPackProducer {
 fn publish_native_snapshot()->Result<(),::semio_framework_os_kernel::io::ArtifactAssemblyRegistryError>{
  let dialect=semio_framework_artifact_reference::Dialect{artifact_kind:"flow.host_snapshot",standard:semio_framework_artifact_reference::StandardId("1"),subset:semio_framework_artifact_reference::SubsetId("*")};
  let codec=crate::os_store::bare_artifact_codec::<FlowHostSnapshot,FlowMutation>(FLOW_DOCUMENT_SCHEMA);
  ::semio_framework_os_kernel::io::register_native_snapshot_codec(dialect,codec)
 }
}
