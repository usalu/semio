//! 🌊️ Controlled native flow nodes, parameters, positions and port edges.
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint2;
use crate::schema::snapshot::{SemioFlowSnapshot, PortRef, STDIO_SEMIOFLOW_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_semio_base::io::sqlite::snapshot::native_decoding::{self as native,workspace};
use semio_framework_value::{ValueError, ValueRefusalKind};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use store::sqlite_snapshot::{SqliteSnapshotControl,SqliteDatabaseLimits};
/// 🛬️ Separates complete SQL cells from the same caller's actual native allocation backing.
pub(crate)fn decode(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control:&mut NativeDecodeControl<'_>)->Result<SemioFlowSnapshot,ValueError>{
 crate::io::sqlite::snapshot::admit_layout(control.limits())?;
 native::with_body(payload,STDIO_SEMIOFLOW_DOCUMENT_SCHEMA,control,native_control,|body,decoder,limits|workspace::decode(decoder,empty,|output,decoder|match body{native::Body::Binary(body)=>binary_into(output,body,decoder,limits),native::Body::Text(body)=>document_into(output,body,decoder,limits)}))
}
/// 🧩️ Fills the retained flow root from borrowed binary octets, inserting every node, parameter and edge before its fields.
pub fn binary_into(output:&mut SemioFlowSnapshot,body:&[u8],control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<(),ValueError>{
 crate::io::sqlite::snapshot::admit_binary(body,control,limits)?;
 let mut entities=0;native::entities(&mut entities,1,limits)?;let mut reader=store::ByteReader::new(body);if reader.read_u8().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unsupported Semio flow native format"))}
 native::reader_text_into(&mut output.schema,&mut reader,control)?;
 native::binary_list_into(&mut output.nodes,&mut reader,control,limits,&mut entities,Default::default,|node,reader,control,entities|{
  native::reader_text_into(&mut node.id,reader,control)?;native::reader_text_into(&mut node.kind,reader,control)?;native::reader_text_into(&mut node.label,reader,control)?;
  native::binary_list_into(&mut node.params,reader,control,limits,entities,Default::default,|param,reader,control,_|{native::reader_text_into(&mut param.key,reader,control)?;native::reader_text_into(&mut param.value,reader,control)})?;
  node.position=SemioPoint2{x:reader.read_f64_le().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?,y:reader.read_f64_le().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?};Ok(())
 })?;
 native::binary_list_into(&mut output.edges,&mut reader,control,limits,&mut entities,Default::default,|edge,reader,control,_|{native::reader_text_into(&mut edge.id,reader,control)?;native::reader_text_into(&mut edge.from.node,reader,control)?;native::reader_text_into(&mut edge.from.port,reader,control)?;native::reader_text_into(&mut edge.to.node,reader,control)?;native::reader_text_into(&mut edge.to.port,reader,control)?;native::reader_text_into(&mut edge.kind,reader,control)})?;
 if reader.remaining()!=0{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio flow native trailing bytes"))}Ok(())
}

/// 📝️ Fills the retained flow root from the borrowed document body, inserting every node, parameter and edge before its fields.
pub fn document_into(output:&mut SemioFlowSnapshot,body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<(),ValueError>{
 crate::io::sqlite::snapshot::admit_document(body,control,limits)?;
 let mut entities=0;native::entities(&mut entities,1,limits)?;let fields=native::fields(body,["schema","nodes","edges"],control)?;native::hex_text_into(&mut output.schema,fields[0].ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Semio flow native schema missing"))?,control)?;
 native::text_list_into(&mut output.nodes,fields[1].unwrap_or("[]"),control,limits,&mut entities,Default::default,|node,value,control,entities|{
  let[id,kind,label,param_fields,position]=native::record(value,control)?;native::hex_text_into(&mut node.id,id,control)?;native::hex_text_into(&mut node.kind,kind,control)?;native::hex_text_into(&mut node.label,label,control)?;
  native::text_list_into(&mut node.params,param_fields,control,limits,entities,Default::default,|param,value,control,_|{let[key,value]=native::record(value,control)?;native::hex_text_into(&mut param.key,key,control)?;native::hex_text_into(&mut param.value,value,control)})?;
  let[x,y]=native::record(position,control)?;node.position=SemioPoint2{x:native::float(x,control)?,y:native::float(y,control)?};Ok(())
 })?;
 native::text_list_into(&mut output.edges,fields[2].unwrap_or("[]"),control,limits,&mut entities,Default::default,|edge,value,control,_|{let[id,from,to,kind]=native::record(value,control)?;native::hex_text_into(&mut edge.id,id,control)?;port_into(&mut edge.from,from,control)?;port_into(&mut edge.to,to,control)?;native::hex_text_into(&mut edge.kind,kind,control)})
}
fn port_into(output:&mut PortRef,value:&str,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{let[node,port]=native::record(value,control)?;native::hex_text_into(&mut output.node,node,control)?;native::hex_text_into(&mut output.port,port,control)}
/// 🕳️ The empty typed root every native prefix starts from.
pub fn empty()->SemioFlowSnapshot{SemioFlowSnapshot{schema:String::new(),nodes:Vec::new(),edges:Vec::new()}}
