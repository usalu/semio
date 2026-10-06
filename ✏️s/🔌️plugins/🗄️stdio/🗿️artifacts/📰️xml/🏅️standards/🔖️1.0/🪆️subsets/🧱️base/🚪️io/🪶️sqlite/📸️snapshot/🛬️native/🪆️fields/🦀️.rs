//! 🪆️ Literal snapshot fields are constructed directly for an enclosing actual owner.
use super::*;
/// 📰️ Reads the six authored field stream without creating an enclosing XML Snapshot mirror.
pub fn read_xml_native_snapshot_fields(state:XmlNativeInput<'_, '_, '_>)->Result<(String,XmlDocument),ValueError>{
 let mut reader=Reader{bytes:state.bytes,position:*state.position,binary:state.binary,rows:*state.rows,limits:state.limits,control:state.control};
 let result=(||{
  if reader.binary{reader.expected(1)?;}else{reader.expected(b'[')?;}
  let schema=reader.string()?;reader.delimiter(b',')?;let mut owner=reader.document()?;reader.delimiter(b']')?;
  if reader.position!=reader.bytes.len(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"XML native snapshot fields have trailing input"))}
  reader.control.checkpoint()?;Ok((schema,owner.0.take().ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"XML native document owner is empty"))?))
 })();
 *state.position=reader.position;*state.rows=reader.rows;result
}
