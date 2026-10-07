//! binary rep note.diff.pack
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

use crate::standards::v1::subsets::any::schema::diff::RasterDiff;
use crate::standards::v1::subsets::any::io::text::diff::{raster_diff_record_spec,raster_diff_to_record,raster_diff_from_record};
impl protocol::DiffBinary for RasterDiff {
 fn encode_diff(&self)->Result<Vec<u8>,protocol::ProtocolError> {store::pack_rt::encode_document(&raster_diff_record_spec(),&raster_diff_to_record(self),&store::PackEncodeOptions::default()).map_err(protocol::ProtocolError::from)}
 fn decode_diff(bytes:&[u8])->Result<Self,protocol::ProtocolError> {
  let(record,_)=store::pack_rt::decode_document(bytes,&raster_diff_record_spec(),&store::PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
  raster_diff_from_record(&record).map_err(|error|protocol::ProtocolError::Malformed{what:"diff record",offset:0,detail:error.to_string()})
 }
}
