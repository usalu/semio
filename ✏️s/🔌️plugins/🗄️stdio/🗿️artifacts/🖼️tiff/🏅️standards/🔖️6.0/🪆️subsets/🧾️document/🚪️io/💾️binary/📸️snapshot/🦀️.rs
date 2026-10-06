//! tiff rep for stdio.tiff 📸️snapshot

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v6_0::subsets::document::schema::snapshot::*;
use crate::STDIO_TIFF_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;

impl store::ArtifactPack for TiffSnapshot {
 fn record_spec()->Option<semio_framework_dsl_record::RecordSpec>{Some(crate::standards::v6_0::subsets::document::io::text::snapshot::spec())}
 fn sqlite_snapshot_codec()->Option<store::ArtifactSqliteSnapshotCodec>{Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())}
 fn encode_pack_with(&self,options:&store::PackEncodeOptions)->Result<Vec<u8>,store::PackError>{
  let body=store::pack_rt::encode_document(&crate::standards::v6_0::subsets::document::io::text::snapshot::spec(),&crate::standards::v6_0::subsets::document::io::text::snapshot::to_record(self),options)?;
  let envelope=store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(),store::semio_format::Component::Pack,1).map_err(|error|store::PackError::from(error.into_value_error()))?;Ok(store::semio_format::wrap_binary(&envelope,&body))
 }
 fn decode_pack_with(bytes:&[u8],options:&store::PackDecodeOptions)->Result<Self,store::PackError>{
  let(envelope,body)=store::semio_format::unwrap_binary(bytes).map_err(|error|store::PackError::from(error.into_value_error()))?;
  if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(),store::semio_format::Component::Pack,1){return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "TIFF owned Pack envelope mismatch")))}
  crate::standards::v6_0::subsets::document::io::text::snapshot::from_record(store::pack_rt::decode_document(&body,&crate::standards::v6_0::subsets::document::io::text::snapshot::spec(),options)?.0).map_err(|error|store::PackError::from(error))
 }
}
}
pub use snapshot_codec::*;
