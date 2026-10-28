//! 💾️ Authored chart binary representation.
use crate::ChartSnapshot;
use protocol as store;
use semio_framework_value::{ValueError,ValueRefusalKind as K};
impl pack::snapshot::ArtifactPack for ChartSnapshot{
 fn record_spec()->Option<semio_framework_dsl_record::RecordSpec>{Some(Self::__dsl_spec())}
 
 fn encode_pack_with(&self,options:&store::PackEncodeOptions)->Result<Vec<u8>,store::PackError>{let body=store::os_pack::record::encode_document(&Self::__dsl_spec(),&self.__dsl_to_record(),options).map_err(store::PackError::from)?;let envelope=store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(),store::semio_format::Component::Pack,1).map_err(|error|store::PackError::from(error.into_value_error()))?;Ok(store::semio_format::wrap_binary(&envelope,&body))}
 fn decode_pack_with(bytes:&[u8],options:&store::PackDecodeOptions)->Result<Self,store::PackError>{let(envelope,body)=store::semio_format::unwrap_binary(bytes).map_err(|error|store::PackError::from(error.into_value_error()))?;if !envelope.matches_identity(<Self as pack::snapshot::ArtifactDsl>::envelope_id(),store::semio_format::Component::Pack,1){return Err(store::PackError::from(ValueError::new(K::InvalidValue,"Chart logical Pack envelope mismatch")))}Self::__dsl_from_record(&store::os_pack::record::decode_document(&body,&Self::__dsl_spec(),options).map_err(store::PackError::from)?.0).map_err(store::PackError::from)}
}
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for ChartSnapshot {
    fn reference_snapshot_codec()->Option<::semio_framework_os_kernel::os_store::serialization::ArtifactReferenceSnapshotCodec<Self>>{Some(::semio_framework_os_kernel::os_store::serialization::reference_snapshot_codec::<Self>())}

fn sqlite_snapshot_codec()->Option<store::os_store::serialization::ArtifactSqliteSnapshotCodec>{Some(<Self as store::os_store::ArtifactSqliteSnapshotOperations>::sqlite_codec())}
}
