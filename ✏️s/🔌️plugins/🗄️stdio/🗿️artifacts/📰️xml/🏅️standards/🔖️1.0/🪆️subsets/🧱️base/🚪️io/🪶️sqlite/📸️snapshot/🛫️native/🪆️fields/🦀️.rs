//! 🪆️ Borrowed enclosing owner fields retain the exact structured XML state grammar.
use super::*;
/// 📰️ Counts or emits only literal snapshot fields into the caller's admitted stream.
pub fn emit_xml_native_snapshot_fields(schema:&str,doc:super::super::sqlite::XmlDocumentView<'_>,state:XmlNativeEmission<'_, '_, '_>)->Result<(),ValueError>{
 let mut writer=Writer{control:state.control,output:state.output,count:*state.count,rows:*state.rows,limits:state.limits,encoding:state.encoding};
 let result=(||{
  if writer.encoding==SnapshotEncoding::Binary{writer.byte(1)?;}else{writer.bytes(b"[")?;}
  writer.string(schema)?;writer.delimiter(b",")?;writer.document(doc)?;writer.delimiter(b"]")
 })();
 *state.count=writer.count;*state.rows=writer.rows;result
}
