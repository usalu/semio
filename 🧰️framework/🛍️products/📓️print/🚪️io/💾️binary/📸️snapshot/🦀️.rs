//! 💾️ Authored chart binary representation.
use crate::ChartSnapshot;
use protocol as store;
use semio_framework_value::{ValueError,ValueRefusalKind as K};
impl store::ArtifactPack for ChartSnapshot{
 fn record_spec()->Option<semio_framework_dsl_record::RecordSpec>{Some(Self::__dsl_spec())}
 fn sqlite_snapshot_codec()->Option<store::ArtifactSqliteSnapshotCodec>{Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())}
 fn encode_pack_with(&self,options:&store::PackEncodeOptions)->Result<Vec<u8>,store::PackError>{let body=store::os_pack::value::encode_document(&Self::__dsl_spec(),&self.__dsl_to_record(),options).map_err(store::PackError::from)?;let envelope=store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(),store::semio_format::Component::Pack,1).map_err(|error|store::PackError::from(error.into_value_error()))?;Ok(store::semio_format::wrap_binary(&envelope,&body))}
 fn decode_pack_with(bytes:&[u8],options:&store::PackDecodeOptions)->Result<Self,store::PackError>{let(envelope,body)=store::semio_format::unwrap_binary(bytes).map_err(|error|store::PackError::from(error.into_value_error()))?;if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(),store::semio_format::Component::Pack,1){return Err(store::PackError::from(ValueError::new(K::InvalidValue,"Chart logical Pack envelope mismatch")))}Self::__dsl_from_record(&store::os_pack::value::decode_document(&body,&Self::__dsl_spec(),options).map_err(store::PackError::from)?.0).map_err(store::PackError::from)}
}
