//! 🕸️ Controlled rich graph records with exact words and guarded nested property ownership.
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint2};
use crate::standards::v1::subsets::value::schema::snapshot::SemioValueEntry;
use crate::standards::v1::subsets::graph::schema::snapshot::{SemioGraphSnapshot,SemioGraphNode,SemioGraphEdge,SemioGraphPort,SemioGraphPortKind,GraphNodeId,GraphEdgeId,STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding as native;
use crate::standards::v1::subsets::value::io::sqlite::snapshot::native_decoding as values;
use semio_framework_value::{native_decoding::NativeDecodeControl,ValueError,ValueRefusalKind};
use store::sqlite_snapshot::{SqliteSnapshotControl,SqliteDatabaseLimits};
pub(crate) fn decode(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<SemioGraphSnapshot,ValueError>{
 let limits=control.limits();crate::standards::v1::subsets::graph::io::sqlite::snapshot::admit_layout(limits)?;
 let size=match payload{store::io::IoPayload::Binary(value)=>value.len(),store::io::IoPayload::Text(value)=>value.len()};if size>limits.max_file_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio Graph native input exceeds file limit"))}
 control.allocation_stage_native(store::sqlite_snapshot::SqliteSnapshotPhase::DecodeNative,|remaining,checkpoint|{
  let native_before=native_control.owned_bytes();let result=native_control.scoped_maximum(native_before.checked_add(remaining).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"native snapshot allowance overflow"))?,|native_control|{
  let result=(||->Result<SemioGraphSnapshot,ValueError>{let result=match payload{
   store::io::IoPayload::Binary(value)=>{let body=store::semio_format::unwrap_binary_controlled(value,STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA,store::semio_format::Component::Pack,1,native_control).map_err(store::semio_format::SemioError::into_value_error)?;binary(body,native_control,limits)?},
   store::io::IoPayload::Text(value)=>{let body=store::semio_format::split_text_preamble_controlled(value,STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA,store::semio_format::Component::Dsl,1,native_control).map_err(store::semio_format::SemioError::into_value_error)?;document(body,native_control,limits)?}
  };let result=crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::Owned::new(result);native_control.checkpoint()?;Ok(result.take())})();result});(result,native_control.owned_bytes().saturating_sub(native_before))
 })?
}
fn invalid(error:impl std::fmt::Display)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,error.to_string())}
fn kind(tag:u8)->Result<SemioGraphPortKind,ValueError>{Ok(match tag{0=>SemioGraphPortKind::In,1=>SemioGraphPortKind::Out,2=>SemioGraphPortKind::InOut,_=>return Err(invalid("invalid Semio native port kind"))})}
fn binary_properties(reader:&mut store::ByteReader<'_>,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits,entities:&mut usize)->Result<Vec<SemioValueEntry>,ValueError>{
 let count=native::length(reader)?;native::entities(entities,count,limits)?;let mut properties=native::Owned::new(control.allocate_vec::<SemioValueEntry>(count)?);
 control.scoped_stage(|control|{control.begin_stage(count)?;for _ in 0..count{let key=native::text(reader,control)?;let value=values::value_binary(reader,control,limits,entities)?;properties.get_mut().push(SemioValueEntry{key,value});control.step()?;}Ok::<_,ValueError>(())})?;Ok(properties.take())
}
fn binary_optional(reader:&mut store::ByteReader<'_>,control:&mut NativeDecodeControl<'_>)->Result<Option<String>,ValueError>{match reader.read_u8().map_err(invalid)?{0=>Ok(None),1=>Ok(Some(native::text(reader,control)?)),_=>Err(invalid("invalid Semio optional text tag"))}}
pub(crate) fn binary(body:&[u8],control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<SemioGraphSnapshot,ValueError>{
 crate::standards::v1::subsets::graph::io::sqlite::snapshot::admit_binary(body,control,limits)?;
 let mut entities=0;native::entities(&mut entities,1,limits)?;let mut reader=store::ByteReader::new(body);if reader.read_u8().map_err(invalid)?!=1{return Err(invalid("unsupported Semio graph native format"))}
 let schema=native::text(&mut reader,control)?;let count=native::length(&mut reader)?;native::entities(&mut entities,count,limits)?;let mut nodes=native::Owned::new(control.allocate_vec::<SemioGraphNode>(count)?);
 control.scoped_stage(|control|{control.begin_stage(count)?;for _ in 0..count{
  let id=GraphNodeId::new(native::text(&mut reader,control)?);let kind=native::text(&mut reader,control)?;let label=native::text(&mut reader,control)?;let position=SemioPoint2{x:reader.read_f64_le().map_err(invalid)?,y:reader.read_f64_le().map_err(invalid)?};let width=reader.read_f64_le().map_err(invalid)?;let height=reader.read_f64_le().map_err(invalid)?;
  let count=native::length(&mut reader)?;native::entities(&mut entities,count,limits)?;let mut ports=native::Owned::new(control.allocate_vec::<SemioGraphPort>(count)?);
  control.scoped_stage(|control|{control.begin_stage(count)?;for _ in 0..count{let name=native::text(&mut reader,control)?;let kind=self::kind(reader.read_u8().map_err(invalid)?)?;let category=native::text(&mut reader,control)?;let properties=binary_properties(&mut reader,control,limits,&mut entities)?;ports.get_mut().push(SemioGraphPort{name,kind,category,properties});control.step()?;}Ok::<_,ValueError>(())})?;
  let properties=binary_properties(&mut reader,control,limits,&mut entities)?;nodes.get_mut().push(SemioGraphNode{id,kind,label,position,width,height,ports:ports.take(),properties});control.step()?;
 }Ok::<_,ValueError>(())})?;
 let count=native::length(&mut reader)?;native::entities(&mut entities,count,limits)?;let mut edges=native::Owned::new(control.allocate_vec::<SemioGraphEdge>(count)?);
 control.scoped_stage(|control|{control.begin_stage(count)?;for _ in 0..count{let id=GraphEdgeId::new(native::text(&mut reader,control)?);let source=GraphNodeId::new(native::text(&mut reader,control)?);let target=GraphNodeId::new(native::text(&mut reader,control)?);let kind=native::text(&mut reader,control)?;let label=native::text(&mut reader,control)?;let source_port=binary_optional(&mut reader,control)?;let target_port=binary_optional(&mut reader,control)?;let properties=binary_properties(&mut reader,control,limits,&mut entities)?;edges.get_mut().push(SemioGraphEdge{id,source,target,kind,label,source_port,target_port,properties});control.step()?;}Ok::<_,ValueError>(())})?;
 if reader.remaining()!=0{return Err(invalid("Semio graph native trailing bytes"))}Ok(SemioGraphSnapshot{schema,nodes:nodes.take(),edges:edges.take()})
}
fn number(value:&str,control:&mut NativeDecodeControl<'_>)->Result<f64,ValueError>{let value=native::hex_text(value,control)?;native::float(&value,control)}
fn text_optional(value:&str,control:&mut NativeDecodeControl<'_>)->Result<Option<String>,ValueError>{if value=="-"{Ok(None)}else{let[value]=native::record(value,control)?;Ok(Some(native::hex_text(value,control)?))}}
fn text_properties(body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits,entities:&mut usize)->Result<Vec<SemioValueEntry>,ValueError>{
 let mut items=native::Items::new(body)?;let count=items.count(control,limits.max_rows)?;native::entities(entities,count,limits)?;let mut properties=native::Owned::new(control.allocate_vec::<SemioValueEntry>(count)?);
 control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(item)=items.next(control)?{let(key,value)=item.split_once(':').ok_or_else(||invalid("Semio graph native property requires colon"))?;let key=native::hex_text(key,control)?;let value=values::value_text(value,control,limits,entities)?;properties.get_mut().push(SemioValueEntry{key,value});control.step()?;}Ok::<_,ValueError>(())})?;Ok(properties.take())
}
pub(crate) fn document(body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<SemioGraphSnapshot,ValueError>{
 crate::standards::v1::subsets::graph::io::sqlite::snapshot::admit_document(body,control,limits)?;
 let mut entities=0;native::entities(&mut entities,1,limits)?;let fields=native::fields(body,["schema","nodes","edges"],control)?;let schema=native::hex_text(fields[0].ok_or_else(||invalid("Semio graph native schema missing"))?,control)?;
 let mut items=native::Items::new(fields[1].unwrap_or("[]"))?;let count=items.count(control,limits.max_rows)?;native::entities(&mut entities,count,limits)?;let mut nodes=native::Owned::new(control.allocate_vec::<SemioGraphNode>(count)?);
 control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(item)=items.next(control)?{
  let[id,kind,label,x,y,width,height,port_fields,property_fields]=native::record(item,control)?;let id=GraphNodeId::new(native::hex_text(id,control)?);let kind=native::hex_text(kind,control)?;let label=native::hex_text(label,control)?;let position=SemioPoint2{x:number(x,control)?,y:number(y,control)?};let width=number(width,control)?;let height=number(height,control)?;
  let mut port_items=native::Items::new(port_fields)?;let count=port_items.count(control,limits.max_rows)?;native::entities(&mut entities,count,limits)?;let mut ports=native::Owned::new(control.allocate_vec::<SemioGraphPort>(count)?);
  control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(item)=port_items.next(control)?{let[name,tag,category,properties]=native::record(item,control)?;let name=native::hex_text(name,control)?;let tag=match tag{"i"=>0,"o"=>1,"x"=>2,_=>return Err(invalid("invalid Semio native port kind"))};let category=native::hex_text(category,control)?;let properties=text_properties(properties,control,limits,&mut entities)?;ports.get_mut().push(SemioGraphPort{name,kind:self::kind(tag)?,category,properties});control.step()?;}Ok::<_,ValueError>(())})?;
  let properties=text_properties(property_fields,control,limits,&mut entities)?;nodes.get_mut().push(SemioGraphNode{id,kind,label,position,width,height,ports:ports.take(),properties});control.step()?;
 }Ok::<_,ValueError>(())})?;
 let mut items=native::Items::new(fields[2].unwrap_or("[]"))?;let count=items.count(control,limits.max_rows)?;native::entities(&mut entities,count,limits)?;let mut edges=native::Owned::new(control.allocate_vec::<SemioGraphEdge>(count)?);
 control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(item)=items.next(control)?{let[id,source,target,kind,label,source_port,target_port,properties]=native::record(item,control)?;let id=GraphEdgeId::new(native::hex_text(id,control)?);let source=GraphNodeId::new(native::hex_text(source,control)?);let target=GraphNodeId::new(native::hex_text(target,control)?);let kind=native::hex_text(kind,control)?;let label=native::hex_text(label,control)?;let source_port=text_optional(source_port,control)?;let target_port=text_optional(target_port,control)?;let properties=text_properties(properties,control,limits,&mut entities)?;edges.get_mut().push(SemioGraphEdge{id,source,target,kind,label,source_port,target_port,properties});control.step()?;}Ok::<_,ValueError>(())})?;
 Ok(SemioGraphSnapshot{schema,nodes:nodes.take(),edges:edges.take()})
}
