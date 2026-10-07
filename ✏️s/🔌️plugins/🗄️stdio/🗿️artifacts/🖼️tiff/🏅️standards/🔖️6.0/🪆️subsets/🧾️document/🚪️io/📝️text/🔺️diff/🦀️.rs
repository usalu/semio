//! 📝️ Owned TIFF value text framing.
use crate::schema::{diff::TiffDiff,snapshot::{TiffIfd,TiffValues}};
use semio_framework_value::{FromValue,ToValue};
pub const COMPONENT_GRAMMAR_SEMIO:&str=include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH:&str=concat!(module_path!(),"::📖️.grammar.semio");
pub(crate) fn hex_encode(bytes:&[u8])->String{bytes.iter().map(|v|format!("{v:02x}")).collect()}
pub(crate) fn hex_decode(text:&str)->Result<Vec<u8>,String>{if text.len()%2!=0||!text.is_ascii(){return Err("invalid hex framing".into())}text.as_bytes().chunks_exact(2).map(|b|u8::from_str_radix(std::str::from_utf8(b).unwrap(),16).map_err(|e|e.to_string())).collect()}
fn owned<T:ToValue>(value:&T)->String{hex_encode(semio_framework_pack_json::to_json_string(value).as_bytes())}
fn read<T:FromValue>(text:&str)->Result<T,String>{let bytes=hex_decode(text)?;let text=std::str::from_utf8(&bytes).map_err(|e|e.to_string())?;semio_framework_pack_json::from_json_str(text,semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e|e.to_string())}
pub(crate) fn enc_values(v:&TiffValues)->String{owned(v)}
pub(crate) fn dec_values(s:&str)->Result<TiffValues,String>{read(s)}
pub(crate) fn enc_ifd(v:&TiffIfd)->String{owned(v)}
pub(crate) fn dec_ifd(s:&str)->Result<TiffIfd,String>{read(s)}
pub(crate) fn enc_str(s:&str)->String{hex_encode(s.as_bytes())}
pub(crate) fn dec_str(s:&str)->Result<String,String>{String::from_utf8(hex_decode(s)?).map_err(|e|e.to_string())}
impl protocol::DiffText for TiffDiff{
 fn print_diff(&self)->String{format!("tiff-diff payload={}",owned(self))}
 fn parse_diff(line:&str)->Result<Self,semio_framework_diagnostic::TextError>{line.strip_prefix("tiff-diff payload=").ok_or_else(||"expected owned TIFF diff payload".to_string()).and_then(read).map_err(|e|semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,e,semio_framework_diagnostic::TextSpan::at(1,1)))}
}
