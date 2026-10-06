//! binary rep for stdio.csv 📸️snapshot

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v_rfc4180::subsets::any::schema::snapshot::*;
use crate::STDIO_CSV_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;

impl store::ArtifactPack for CsvSnapshot{
 fn record_spec()->Option<semio_framework_dsl_record::RecordSpec>{Some(Self::__dsl_spec())}
 fn sqlite_snapshot_codec()->Option<store::ArtifactSqliteSnapshotCodec>{Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())}
 fn encode_pack_with(&self,options:&store::PackEncodeOptions)->Result<Vec<u8>,store::PackError>{let body=store::pack_rt::encode_document(&Self::__dsl_spec(),&self.__dsl_to_record(),options)?;let envelope=store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(),store::semio_format::Component::Pack,1).map_err(|error|store::PackError::from(error.into_value_error()))?;Ok(store::semio_format::wrap_binary(&envelope,&body))}
 fn decode_pack_with(bytes:&[u8],options:&store::PackDecodeOptions)->Result<Self,store::PackError>{let(envelope,body)=store::semio_format::unwrap_binary(bytes).map_err(|error|store::PackError::from(error.into_value_error()))?;if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(),store::semio_format::Component::Pack,1){return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"CSV logical Pack envelope mismatch")))}Self::__dsl_from_record(&store::pack_rt::decode_document(&body,&Self::__dsl_spec(),options)?.0).map_err(store::PackError::from)}
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::standards::v_rfc4180::subsets::any::schema::snapshot::*;
use crate::STDIO_CSV_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;
use crate::standards::v_rfc4180::subsets::any::io::text::snapshot::{read_csv_source_text};
/// 📥️ Reads the logical Pack document or authored UTF-8 CSV bytes.
pub fn read_csv_source_binary(bytes:&[u8])->Result<CsvSnapshot,store::PackError>{if bytes.starts_with(&[137,83,69,77,13,10,26,10]){<CsvSnapshot as store::ArtifactPack>::decode_pack(bytes)}else{let text=std::str::from_utf8(bytes).map_err(|error|store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error.to_string())))?;read_csv_source_text(text).map_err(store::PackError::from)}}
}
pub use snapshot_wire_codec::*;
