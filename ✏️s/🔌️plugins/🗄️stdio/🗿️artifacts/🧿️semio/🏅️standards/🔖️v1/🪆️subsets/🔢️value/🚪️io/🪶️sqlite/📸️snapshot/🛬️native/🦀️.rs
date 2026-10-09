//! 🌳️ Controlled native typed values preserve lexemes, ordered duplicates and explicit references.
use semio_framework_value::{ValueError,ValueRefusalKind};
use crate::standards::v1::subsets::value::schema::snapshot::{SemioValue,SemioValueEntry,SemioValueNode,SemioValueSnapshot,ValueId,STDIO_SEMIOVALUE_DOCUMENT_SCHEMA};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding as native;
use semio_framework_value::native_decoding::NativeDecodeControl;
use store::sqlite_snapshot::{SqliteSnapshotControl,SqliteDatabaseLimits};
/// 🛬️ Counts complete cells before the same caller's actual native ownership allocation.
pub(crate)fn decode(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<SemioValueSnapshot,ValueError>{
 let limits=control.limits();crate::standards::v1::subsets::value::io::sqlite::snapshot::admit_layout(limits)?;
 let size=match payload{store::io::IoPayload::Binary(value)=>value.len(),store::io::IoPayload::Text(value)=>value.len()};if size>limits.max_file_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio value native input exceeds file limit"))}
 control.allocation_stage_native(store::sqlite_snapshot::SqliteSnapshotPhase::DecodeNative,|remaining,checkpoint|{
  let native_before=native_control.owned_bytes();let result=native_control.scoped_maximum(native_before.checked_add(remaining).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"native snapshot allowance overflow"))?,|native_control|{
  let result=(||->Result<SemioValueSnapshot,ValueError>{let result=match payload{
   store::io::IoPayload::Binary(value)=>{let body=store::semio_format::unwrap_binary_controlled(value,STDIO_SEMIOVALUE_DOCUMENT_SCHEMA,store::semio_format::Component::Pack,1,native_control).map_err(store::semio_format::SemioError::into_value_error)?;let body=native_control.borrow_text(body)?;document(body,native_control,limits)?},
   store::io::IoPayload::Text(value)=>{let body=store::semio_format::split_text_preamble_controlled(value,STDIO_SEMIOVALUE_DOCUMENT_SCHEMA,store::semio_format::Component::Dsl,1,native_control).map_err(store::semio_format::SemioError::into_value_error)?;document(body,native_control,limits)?}
  };let result=native::Owned::new(result);native_control.checkpoint()?;Ok(result.take())})();result});(result,native_control.owned_bytes().saturating_sub(native_before))
 })?
}
pub(crate) fn document(body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<SemioValueSnapshot,ValueError>{
 crate::standards::v1::subsets::value::io::sqlite::snapshot::admit_document(body,control,limits)?; let mut entities=0;native::entities(&mut entities,1,limits)?;let[schema,root,nodes]=native::record(body,control)?;let schema=native::hex_text(schema,control)?;let root=native::Owned::new(value_text(root,control,limits,&mut entities)?);
 let mut items=native::Items::new(nodes)?;let count=items.count(control,limits.max_rows)?;native::entities(&mut entities,count,limits)?;children(entities,count,limits)?;let mut nodes=native::Owned::new(control.allocate_vec::<SemioValueNode>(count)?);
 control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(item)=items.next(control)?{let(id,value)=item.split_once(':').ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Semio native value node requires colon"))?;let id=ValueId::new(native::hex_text(id,control)?);let value=value_text(value,control,limits,&mut entities)?;nodes.get_mut().push(SemioValueNode{id,value});control.step()?;}Ok::<_,ValueError>(())})?;
 Ok(SemioValueSnapshot{schema,root:root.take(),nodes:nodes.take()})
}
pub(crate) fn value_text(value:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits,entities:&mut usize)->Result<SemioValue,ValueError>{
 control.scoped_depth(64,|control|{
 native::entities(entities,1,limits)?;control.checkpoint()?;if value=="Z"{return Ok(SemioValue::Null)}let(tag,body)=value.split_at_checked(1).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Semio native value tag missing"))?;let body=body.strip_prefix('[').and_then(|body|body.strip_suffix(']')).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Semio native value requires brackets"))?;
 Ok(match tag{
 "B"=>SemioValue::Bool{value:match body{"0"=>false,"1"=>true,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native boolean"))}},
 "I"=>SemioValue::Int{lexeme:native::hex_text(body,control)?},"F"=>SemioValue::Float{lexeme:native::hex_text(body,control)?},"S"=>SemioValue::Str{value:native::hex_text(body,control)?},"Y"=>SemioValue::Bytes{value:native::hex(body,control)?},"R"=>SemioValue::Ref{id:ValueId::new(native::hex_text(body,control)?)},
 "L"=>{let mut items=native::Items::new(&value[1..])?;let count=items.count(control,limits.max_rows)?;native::entities(entities,count,limits)?;children(*entities,count,limits)?;let mut values=native::Owned::new(control.allocate_vec::<SemioValue>(count)?);control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(item)=items.next(control)?{values.get_mut().push(value_text(item,control,limits,entities)?);control.step()?;}Ok::<_,ValueError>(())})?;SemioValue::List{items:values.take()}},
 "M"=>{let mut items=native::Items::new(&value[1..])?;let count=items.count(control,limits.max_rows)?;native::entities(entities,count,limits)?;children(*entities,count,limits)?;let mut entries=native::Owned::new(control.allocate_vec::<SemioValueEntry>(count)?);control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(item)=items.next(control)?{let(key,value)=item.split_once(':').ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Semio native value entry requires colon"))?;let key=native::hex_text(key,control)?;let value=value_text(value,control,limits,entities)?;entries.get_mut().push(SemioValueEntry{key,value});control.step()?;}Ok::<_,ValueError>(())})?;SemioValue::Map{entries:entries.take()}},
 _=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown Semio native value tag"))})
 })
}
pub(crate) fn value_binary(reader:&mut store::ByteReader<'_>,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits,entities:&mut usize)->Result<SemioValue,ValueError>{
 control.scoped_depth(64,|control|{
 native::entities(entities,1,limits)?;control.checkpoint()?;let tag=reader.read_u8().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?;
 Ok(match tag{
 0=>SemioValue::Null,1=>SemioValue::Bool{value:match reader.read_u8().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?{0=>false,1=>true,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native binary boolean"))}},
 2=>SemioValue::Int{lexeme:native::text(reader,control)?},3=>SemioValue::Float{lexeme:native::text(reader,control)?},4=>SemioValue::Str{value:native::text(reader,control)?},5=>SemioValue::Bytes{value:control.copy_bytes(native::bytes(reader)?)?},8=>SemioValue::Ref{id:ValueId::new(native::text(reader,control)?)},
 6=>{let count=native::length(reader)?;native::entities(entities,count,limits)?;children(*entities,count,limits)?;let mut values=native::Owned::new(control.allocate_vec::<SemioValue>(count)?);control.scoped_stage(|control|{control.begin_stage(count)?;for _ in 0..count{values.get_mut().push(value_binary(reader,control,limits,entities)?);control.step()?;}Ok::<_,ValueError>(())})?;SemioValue::List{items:values.take()}},
 7=>{let count=native::length(reader)?;native::entities(entities,count,limits)?;children(*entities,count,limits)?;let mut entries=native::Owned::new(control.allocate_vec::<SemioValueEntry>(count)?);control.scoped_stage(|control|{control.begin_stage(count)?;for _ in 0..count{let key=native::text(reader,control)?;let value=value_binary(reader,control,limits,entities)?;entries.get_mut().push(SemioValueEntry{key,value});control.step()?;}Ok::<_,ValueError>(())})?;SemioValue::Map{entries:entries.take()}},
 _=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown Semio native binary value tag"))})
 })
}

fn children(entities:usize,count:usize,limits:SqliteDatabaseLimits)->Result<(),ValueError>{if count>limits.max_rows.saturating_sub(entities){Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio native values exceed remaining row limit"))}else{Ok(())}}
