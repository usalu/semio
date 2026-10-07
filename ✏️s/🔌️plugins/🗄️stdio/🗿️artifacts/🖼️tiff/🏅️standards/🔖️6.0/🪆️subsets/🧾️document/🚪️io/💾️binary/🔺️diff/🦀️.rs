//! 💾️ Length-framed owned TIFF value payloads.
use crate::schema::{diff::TiffDiff,snapshot::{TiffIfd,TiffValues}};
use semio_framework_value::{FromValue,ToValue};
pub const COMPONENT_PROTOCOL_SEMIO:&str=include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH:&str=concat!(module_path!(),"::📡️.protocol.semio");
pub(crate) fn write_bytes_lp(out:&mut Vec<u8>,bytes:&[u8]){store::pack_rt::write_varint_u64(out,bytes.len() as u64);out.extend_from_slice(bytes)}
pub(crate) fn read_bytes_lp(reader:&mut store::ByteReader<'_>)->Result<Vec<u8>,String>{let count=usize::try_from(reader.read_varint_u64().map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;reader.read_bytes(count).map(|b|b.to_vec()).map_err(|e|e.to_string())}
fn write<T:ToValue>(value:&T,out:&mut Vec<u8>){write_bytes_lp(out,semio_framework_pack_json::to_json_string(value).as_bytes())}
fn read<T:FromValue>(reader:&mut store::ByteReader<'_>)->Result<T,String>{let bytes=read_bytes_lp(reader)?;semio_framework_pack_json::from_json_str(std::str::from_utf8(&bytes).map_err(|e|e.to_string())?,semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e|e.to_string())}
pub(crate) fn enc_values_bin(v:&TiffValues,out:&mut Vec<u8>){write(v,out)}
pub(crate) fn dec_values_bin(r:&mut store::ByteReader<'_>)->Result<TiffValues,String>{read(r)}
pub(crate) fn enc_ifd_bin(v:&TiffIfd,out:&mut Vec<u8>){write(v,out)}
pub(crate) fn dec_ifd_bin(r:&mut store::ByteReader<'_>)->Result<TiffIfd,String>{read(r)}
pub(crate) fn write_str_lp(out:&mut Vec<u8>,s:&str){write_bytes_lp(out,s.as_bytes())}
pub(crate) fn read_str_lp(r:&mut store::ByteReader<'_>)->Result<String,String>{String::from_utf8(read_bytes_lp(r)?).map_err(|e|e.to_string())}
impl protocol::DiffBinary for TiffDiff{
 fn encode_diff(&self)->Result<Vec<u8>,protocol::ProtocolError>{let mut out=vec![store::pack_rt::OP_BINARY_FORMAT];write(self,&mut out);Ok(out)}
 fn decode_diff(bytes:&[u8])->Result<Self,protocol::ProtocolError>{let mut reader=store::ByteReader::new(bytes);let value=(||{if reader.read_u8().map_err(|e|e.to_string())?!=store::pack_rt::OP_BINARY_FORMAT{return Err("TIFF diff format".into())}let value=read(&mut reader)?;if reader.position()!=bytes.len(){return Err("TIFF diff trailing payload".into())}Ok(value)})();value.map_err(|detail|protocol::ProtocolError::Malformed{what:"TIFF diff",offset:reader.position() as u64,detail})}
}
