//! binary rep for stdio.json 🔺️diff

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

use crate::standards::v1::subsets::any::schema::diff::JackDiff;
use crate::standards::v1::subsets::any::io::text::diff::{jack_diff_record_spec,jack_diff_to_record,jack_diff_from_record};
impl protocol::DiffBinary for JackDiff {
 fn encode_diff(&self)->Result<Vec<u8>,protocol::ProtocolError> {store::pack_rt::encode_document(&jack_diff_record_spec(),&jack_diff_to_record(self),&Default::default()).map_err(protocol::ProtocolError::from)}
 fn decode_diff(bytes:&[u8])->Result<Self,protocol::ProtocolError> {let (record,_)=store::pack_rt::decode_document(bytes,&jack_diff_record_spec(),&Default::default()).map_err(protocol::ProtocolError::from)?;jack_diff_from_record(&record).map_err(|error|protocol::ProtocolError::from(store::text_error_to_pack_error(error)))}
}

