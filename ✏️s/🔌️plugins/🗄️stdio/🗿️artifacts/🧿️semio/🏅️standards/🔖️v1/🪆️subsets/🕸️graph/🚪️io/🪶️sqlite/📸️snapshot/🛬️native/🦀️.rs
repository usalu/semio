//! 🕸️ Controlled rich graph records with exact words and guarded nested property ownership.
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint2;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueEntry;
use crate::schema::snapshot::{SemioGraphSnapshot, SemioGraphPortKind, STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_semio_base::io::sqlite::snapshot::native_decoding::{self as native,workspace};
use semio_s_artifact_stdio_semio_value_model::io::sqlite::snapshot::native_decoding as values;
use semio_framework_value::{ValueError, ValueRefusalKind};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use store::sqlite_snapshot::{SqliteSnapshotControl,SqliteDatabaseLimits};
/// 🛬️ Separates complete SQL cells from the same caller's actual native allocation backing.
pub(crate)fn decode(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control:&mut NativeDecodeControl<'_>)->Result<SemioGraphSnapshot,ValueError>{
 crate::io::sqlite::snapshot::admit_layout(control.limits())?;
 native::with_body(payload,STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA,control,native_control,|body,decoder,limits|workspace::decode(decoder,empty,|output,decoder|match body{native::Body::Binary(body)=>binary_into(output,body,decoder,limits),native::Body::Text(body)=>document_into(output,body,decoder,limits)}))
}
fn invalid(error:impl std::fmt::Display)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,error.to_string())}
fn kind(tag:u8)->Result<SemioGraphPortKind,ValueError>{Ok(match tag{0=>SemioGraphPortKind::In,1=>SemioGraphPortKind::Out,2=>SemioGraphPortKind::InOut,_=>return Err(invalid("invalid Semio native port kind"))})}


/// 📇️ Fills one ordered property list in its original vector, inserting each entry before its key and value.
fn binary_properties_into(output:&mut Vec<SemioValueEntry>,reader:&mut store::ByteReader<'_>,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits,entities:&mut usize)->Result<(),ValueError>{
 native::binary_list_into(output,reader,control,limits,entities,Default::default,|entry,reader,control,entities|{native::reader_text_into(&mut entry.key,reader,control)?;values::value_binary_into(&mut entry.value,reader,control,limits,entities)})
}
/// ☑️ Fills one optional port name in its original slot.
fn binary_optional_into(output:&mut Option<String>,reader:&mut store::ByteReader<'_>,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{match reader.read_u8().map_err(invalid)?{0=>Ok(()),1=>{*output=Some(String::new());native::reader_text_into(output.as_mut().unwrap(),reader,control)},_=>Err(invalid("invalid Semio optional text tag"))}}
/// 🧩️ Fills the retained graph root from borrowed binary octets, inserting every node, port and edge before its fields.
pub fn binary_into(output:&mut SemioGraphSnapshot,body:&[u8],control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<(),ValueError>{
 crate::io::sqlite::snapshot::admit_binary(body,control,limits)?;
 let mut entities=0;native::entities(&mut entities,1,limits)?;let mut reader=store::ByteReader::new(body);if reader.read_u8().map_err(invalid)?!=1{return Err(invalid("unsupported Semio graph native format"))}
 native::reader_text_into(&mut output.schema,&mut reader,control)?;
 native::binary_list_into(&mut output.nodes,&mut reader,control,limits,&mut entities,Default::default,|node,reader,control,entities|{
  native::reader_text_into(&mut node.id.value,reader,control)?;native::reader_text_into(&mut node.kind,reader,control)?;native::reader_text_into(&mut node.label,reader,control)?;node.position=SemioPoint2{x:reader.read_f64_le().map_err(invalid)?,y:reader.read_f64_le().map_err(invalid)?};node.width=reader.read_f64_le().map_err(invalid)?;node.height=reader.read_f64_le().map_err(invalid)?;
  native::binary_list_into(&mut node.ports,reader,control,limits,entities,Default::default,|port,reader,control,entities|{native::reader_text_into(&mut port.name,reader,control)?;port.kind=self::kind(reader.read_u8().map_err(invalid)?)?;native::reader_text_into(&mut port.category,reader,control)?;binary_properties_into(&mut port.properties,reader,control,limits,entities)})?;
  binary_properties_into(&mut node.properties,reader,control,limits,entities)
 })?;
 native::binary_list_into(&mut output.edges,&mut reader,control,limits,&mut entities,Default::default,|edge,reader,control,entities|{native::reader_text_into(&mut edge.id.value,reader,control)?;native::reader_text_into(&mut edge.source.value,reader,control)?;native::reader_text_into(&mut edge.target.value,reader,control)?;native::reader_text_into(&mut edge.kind,reader,control)?;native::reader_text_into(&mut edge.label,reader,control)?;binary_optional_into(&mut edge.source_port,reader,control)?;binary_optional_into(&mut edge.target_port,reader,control)?;binary_properties_into(&mut edge.properties,reader,control,limits,entities)})?;
 if reader.remaining()!=0{return Err(invalid("Semio graph native trailing bytes"))}Ok(())
}



/// 🔘️ Fills one optional port name in its original slot from a bracketed hexadecimal record.
fn text_optional_into(output:&mut Option<String>,value:&str,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{if value=="-"{Ok(())}else{let[value]=native::record(value,control)?;*output=Some(String::new());native::hex_text_into(output.as_mut().unwrap(),value,control)}}
/// 🧾️ Fills one ordered property list in its original vector, inserting each entry before its key and value.
fn text_properties_into(output:&mut Vec<SemioValueEntry>,body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits,entities:&mut usize)->Result<(),ValueError>{
 native::text_list_into(output,body,control,limits,entities,Default::default,|entry,item,control,entities|{let(key,value)=item.split_once(':').ok_or_else(||invalid("Semio graph native property requires colon"))?;native::hex_text_into(&mut entry.key,key,control)?;values::value_text_into(&mut entry.value,value,control,limits,entities)})
}
/// 📝️ Fills the retained graph root from the borrowed document body, inserting every node, port and edge before its fields.
pub fn document_into(output:&mut SemioGraphSnapshot,body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<(),ValueError>{
 crate::io::sqlite::snapshot::admit_document(body,control,limits)?;
 let mut entities=0;native::entities(&mut entities,1,limits)?;let fields=native::fields(body,["schema","nodes","edges"],control)?;native::hex_text_into(&mut output.schema,fields[0].ok_or_else(||invalid("Semio graph native schema missing"))?,control)?;
 native::text_list_into(&mut output.nodes,fields[1].unwrap_or("[]"),control,limits,&mut entities,Default::default,|node,item,control,entities|{
  let[id,kind,label,x,y,width,height,port_fields,property_fields]=native::record(item,control)?;native::hex_text_into(&mut node.id.value,id,control)?;native::hex_text_into(&mut node.kind,kind,control)?;native::hex_text_into(&mut node.label,label,control)?;node.position=SemioPoint2{x:native::hex_float(x,control)?,y:native::hex_float(y,control)?};node.width=native::hex_float(width,control)?;node.height=native::hex_float(height,control)?;
  native::text_list_into(&mut node.ports,port_fields,control,limits,entities,Default::default,|port,item,control,entities|{let[name,tag,category,properties]=native::record(item,control)?;native::hex_text_into(&mut port.name,name,control)?;let tag=match tag{"i"=>0,"o"=>1,"x"=>2,_=>return Err(invalid("invalid Semio native port kind"))};native::hex_text_into(&mut port.category,category,control)?;text_properties_into(&mut port.properties,properties,control,limits,entities)?;port.kind=self::kind(tag)?;Ok(())})?;
  text_properties_into(&mut node.properties,property_fields,control,limits,entities)
 })?;
 native::text_list_into(&mut output.edges,fields[2].unwrap_or("[]"),control,limits,&mut entities,Default::default,|edge,item,control,entities|{let[id,source,target,kind,label,source_port,target_port,properties]=native::record(item,control)?;native::hex_text_into(&mut edge.id.value,id,control)?;native::hex_text_into(&mut edge.source.value,source,control)?;native::hex_text_into(&mut edge.target.value,target,control)?;native::hex_text_into(&mut edge.kind,kind,control)?;native::hex_text_into(&mut edge.label,label,control)?;text_optional_into(&mut edge.source_port,source_port,control)?;text_optional_into(&mut edge.target_port,target_port,control)?;text_properties_into(&mut edge.properties,properties,control,limits,entities)})
}
/// 🕳️ The empty typed root every native prefix starts from.
pub fn empty()->SemioGraphSnapshot{SemioGraphSnapshot{schema:String::new(),nodes:Vec::new(),edges:Vec::new()}}
