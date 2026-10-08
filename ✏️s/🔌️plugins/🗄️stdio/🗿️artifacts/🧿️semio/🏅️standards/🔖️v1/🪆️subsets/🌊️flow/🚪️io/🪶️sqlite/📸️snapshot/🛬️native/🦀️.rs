//! 🌊️ Controlled native flow nodes, parameters, positions and port edges.
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint2};
use crate::standards::v1::subsets::flow::schema::snapshot::{SemioFlowSnapshot,FlowNode,FlowParam,FlowEdge,PortRef,STDIO_SEMIOFLOW_DOCUMENT_SCHEMA};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding as native;
use semio_framework_value::{native_decoding::NativeDecodeControl,ValueError,ValueRefusalKind};
use store::sqlite_snapshot::{SqliteSnapshotControl,SqliteDatabaseLimits};
pub(crate) fn decode(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<SemioFlowSnapshot,ValueError>{
 let limits=control.limits();crate::standards::v1::subsets::flow::io::sqlite::snapshot::admit_layout(limits)?;
 let size=match payload{store::io::IoPayload::Binary(value)=>value.len(),store::io::IoPayload::Text(value)=>value.len()};if size>limits.max_file_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio Flow native input exceeds file limit"))}
 control.allocation_stage(store::sqlite_snapshot::SqliteSnapshotPhase::DecodeNative,|remaining,checkpoint|{
  let mut callback=|event:semio_framework_value::native_decoding::NativeDecodeProgress|checkpoint(event.completed,event.total);let mut native_control=NativeDecodeControl::new(remaining,&mut callback);
  let result=(||->Result<SemioFlowSnapshot,ValueError>{let result=match payload{
   store::io::IoPayload::Binary(value)=>{let body=store::semio_format::unwrap_binary_controlled(value,STDIO_SEMIOFLOW_DOCUMENT_SCHEMA,store::semio_format::Component::Pack,1,&mut native_control).map_err(store::semio_format::SemioError::into_value_error)?;binary(body,&mut native_control,limits)?},
   store::io::IoPayload::Text(value)=>{let body=store::semio_format::split_text_preamble_controlled(value,STDIO_SEMIOFLOW_DOCUMENT_SCHEMA,store::semio_format::Component::Dsl,1,&mut native_control).map_err(store::semio_format::SemioError::into_value_error)?;document(body,&mut native_control,limits)?}
  };let result=crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::Owned::new(result);native_control.checkpoint()?;Ok(result.take())})();(result,native_control.owned_bytes())
 })?
}
pub(crate) fn binary(body:&[u8],control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<SemioFlowSnapshot,ValueError>{
 crate::standards::v1::subsets::flow::io::sqlite::snapshot::admit_binary(body,control,limits)?;
 let mut entities=0;native::entities(&mut entities,1,limits)?;let mut reader=store::ByteReader::new(body);if reader.read_u8().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unsupported Semio flow native format"))}
 let schema=native::text(&mut reader,control)?;let count=native::length(&mut reader)?;native::entities(&mut entities,count,limits)?;let mut nodes=control.allocate_vec::<FlowNode>(count)?;
 control.scoped_stage(|control|{control.begin_stage(count)?;for _ in 0..count{
  let id=native::text(&mut reader,control)?;let kind=native::text(&mut reader,control)?;let label=native::text(&mut reader,control)?;let count=native::length(&mut reader)?;native::entities(&mut entities,count,limits)?;let mut params=control.allocate_vec::<FlowParam>(count)?;control.scoped_stage(|control|{control.begin_stage(count)?;for _ in 0..count{let key=native::text(&mut reader,control)?;let value=native::text(&mut reader,control)?;params.push(FlowParam{key,value});control.step()?;}Ok::<_,ValueError>(())})?;
  let position=SemioPoint2{x:reader.read_f64_le().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?,y:reader.read_f64_le().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?};nodes.push(FlowNode{id,kind,label,params,position});control.step()?;
 }Ok::<_,ValueError>(())})?;
 let count=native::length(&mut reader)?;native::entities(&mut entities,count,limits)?;let mut edges=control.allocate_vec::<FlowEdge>(count)?;control.scoped_stage(|control|{control.begin_stage(count)?;for _ in 0..count{let id=native::text(&mut reader,control)?;let from=PortRef{node:native::text(&mut reader,control)?,port:native::text(&mut reader,control)?};let to=PortRef{node:native::text(&mut reader,control)?,port:native::text(&mut reader,control)?};let kind=native::text(&mut reader,control)?;edges.push(FlowEdge{id,from,to,kind});control.step()?;}Ok::<_,ValueError>(())})?;
 if reader.remaining()!=0{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio flow native trailing bytes"))}Ok(SemioFlowSnapshot{schema,nodes,edges})
}
fn port(value:&str,control:&mut NativeDecodeControl<'_>)->Result<PortRef,ValueError>{let[node,port]=native::record(value,control)?;Ok(PortRef{node:native::hex_text(node,control)?,port:native::hex_text(port,control)?})}
pub(crate) fn document(body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<SemioFlowSnapshot,ValueError>{
 crate::standards::v1::subsets::flow::io::sqlite::snapshot::admit_document(body,control,limits)?;
 let mut entities=0;native::entities(&mut entities,1,limits)?;let fields=native::fields(body,["schema","nodes","edges"],control)?;let schema=native::hex_text(fields[0].ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Semio flow native schema missing"))?,control)?;let mut items=native::Items::new(fields[1].unwrap_or("[]"))?;let count=items.count(control,limits.max_rows)?;native::entities(&mut entities,count,limits)?;let mut nodes=control.allocate_vec::<FlowNode>(count)?;
 control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(value)=items.next(control)?{
  let[id,kind,label,param_fields,position]=native::record(value,control)?;let id=native::hex_text(id,control)?;let kind=native::hex_text(kind,control)?;let label=native::hex_text(label,control)?;let mut param_items=native::Items::new(param_fields)?;let count=param_items.count(control,limits.max_rows)?;native::entities(&mut entities,count,limits)?;let mut params=control.allocate_vec::<FlowParam>(count)?;
  control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(value)=param_items.next(control)?{let[key,value]=native::record(value,control)?;let key=native::hex_text(key,control)?;let value=native::hex_text(value,control)?;params.push(FlowParam{key,value});control.step()?;}Ok::<_,ValueError>(())})?;
  let[x,y]=native::record(position,control)?;let position=SemioPoint2{x:native::float(x,control)?,y:native::float(y,control)?};nodes.push(FlowNode{id,kind,label,params,position});control.step()?;
 }Ok::<_,ValueError>(())})?;
 let mut items=native::Items::new(fields[2].unwrap_or("[]"))?;let count=items.count(control,limits.max_rows)?;native::entities(&mut entities,count,limits)?;let mut edges=control.allocate_vec::<FlowEdge>(count)?;control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(value)=items.next(control)?{let[id,from,to,kind]=native::record(value,control)?;let id=native::hex_text(id,control)?;let from=port(from,control)?;let to=port(to,control)?;let kind=native::hex_text(kind,control)?;edges.push(FlowEdge{id,from,to,kind});control.step()?;}Ok::<_,ValueError>(())})?;
 Ok(SemioFlowSnapshot{schema,nodes,edges})
}

