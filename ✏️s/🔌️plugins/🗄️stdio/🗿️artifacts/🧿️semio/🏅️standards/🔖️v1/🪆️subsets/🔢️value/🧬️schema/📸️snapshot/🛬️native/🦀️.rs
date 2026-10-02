//! 🌳️ Controlled native typed values preserve lexemes, ordered duplicates and explicit references.
use super::{SemioValue,SemioValueEntry,SemioValueNode,SemioValueSnapshot,ValueId,STDIO_SEMIOVALUE_DOCUMENT_SCHEMA};
use crate::standards::v1::subsets::base::schema::snapshot::native_decoding as native;
use semio_framework_value::native_decoding::NativeDecodeControl;
use store::sqlite_snapshot::{SqliteSnapshotControl,SqliteDatabaseLimits};
pub(super) fn decode(payload:&store::os_io::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<SemioValueSnapshot,String>{native::decode(payload,STDIO_SEMIOVALUE_DOCUMENT_SCHEMA,control,|body,control,limits|{let body=control.borrow_text(body)?;document(body,control,limits)},document)}
pub(crate) fn document(body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<SemioValueSnapshot,String>{
 let mut entities=0;native::entities(&mut entities,1,limits)?;let[schema,root,nodes]=native::record(body,control)?;let schema=native::hex_text(schema,control)?;let root=native::Owned::new(value_text(root,control,limits,&mut entities)?);
 let mut items=native::Items::new(nodes)?;let count=items.count(control,limits.max_rows)?;native::entities(&mut entities,count,limits)?;children(entities,count,limits)?;let mut nodes=native::Owned::new(control.allocate_vec::<SemioValueNode>(count)?);
 control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(item)=items.next(control)?{let(id,value)=item.split_once(':').ok_or("Semio native value node requires colon")?;let id=ValueId::new(native::hex_text(id,control)?);let value=value_text(value,control,limits,&mut entities)?;nodes.get_mut().push(SemioValueNode{id,value});control.step()?;}Ok::<_,String>(())})?;
 Ok(SemioValueSnapshot{schema,root:root.take(),nodes:nodes.take()})
}
pub(crate) fn value_text(value:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits,entities:&mut usize)->Result<SemioValue,String>{
 control.scoped_depth(64,|control|{
 native::entities(entities,1,limits)?;control.checkpoint()?;if value=="Z"{return Ok(SemioValue::Null)}let(tag,body)=value.split_at_checked(1).ok_or("Semio native value tag missing")?;let body=body.strip_prefix('[').and_then(|body|body.strip_suffix(']')).ok_or("Semio native value requires brackets")?;
 Ok(match tag{
 "B"=>SemioValue::Bool{value:match body{"0"=>false,"1"=>true,_=>return Err("invalid Semio native boolean".into())}},
 "I"=>SemioValue::Int{lexeme:native::hex_text(body,control)?},"F"=>SemioValue::Float{lexeme:native::hex_text(body,control)?},"S"=>SemioValue::Str{value:native::hex_text(body,control)?},"Y"=>SemioValue::Bytes{value:native::hex(body,control)?},"R"=>SemioValue::Ref{id:ValueId::new(native::hex_text(body,control)?)},
 "L"=>{let mut items=native::Items::new(&value[1..])?;let count=items.count(control,limits.max_rows)?;native::entities(entities,count,limits)?;children(*entities,count,limits)?;let mut values=native::Owned::new(control.allocate_vec::<SemioValue>(count)?);control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(item)=items.next(control)?{values.get_mut().push(value_text(item,control,limits,entities)?);control.step()?;}Ok::<_,String>(())})?;SemioValue::List{items:values.take()}},
 "M"=>{let mut items=native::Items::new(&value[1..])?;let count=items.count(control,limits.max_rows)?;native::entities(entities,count,limits)?;children(*entities,count,limits)?;let mut entries=native::Owned::new(control.allocate_vec::<SemioValueEntry>(count)?);control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(item)=items.next(control)?{let(key,value)=item.split_once(':').ok_or("Semio native value entry requires colon")?;let key=native::hex_text(key,control)?;let value=value_text(value,control,limits,entities)?;entries.get_mut().push(SemioValueEntry{key,value});control.step()?;}Ok::<_,String>(())})?;SemioValue::Map{entries:entries.take()}},
 _=>return Err("unknown Semio native value tag".into())})
 })
}
pub(crate) fn value_binary(reader:&mut store::ByteReader<'_>,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits,entities:&mut usize)->Result<SemioValue,String>{
 control.scoped_depth(64,|control|{
 native::entities(entities,1,limits)?;control.checkpoint()?;let tag=reader.read_u8().map_err(|error|error.to_string())?;
 Ok(match tag{
 0=>SemioValue::Null,1=>SemioValue::Bool{value:match reader.read_u8().map_err(|error|error.to_string())?{0=>false,1=>true,_=>return Err("invalid Semio native binary boolean".into())}},
 2=>SemioValue::Int{lexeme:native::text(reader,control)?},3=>SemioValue::Float{lexeme:native::text(reader,control)?},4=>SemioValue::Str{value:native::text(reader,control)?},5=>SemioValue::Bytes{value:control.copy_bytes(native::bytes(reader)?)?},8=>SemioValue::Ref{id:ValueId::new(native::text(reader,control)?)},
 6=>{let count=native::length(reader)?;native::entities(entities,count,limits)?;children(*entities,count,limits)?;let mut values=native::Owned::new(control.allocate_vec::<SemioValue>(count)?);control.scoped_stage(|control|{control.begin_stage(count)?;for _ in 0..count{values.get_mut().push(value_binary(reader,control,limits,entities)?);control.step()?;}Ok::<_,String>(())})?;SemioValue::List{items:values.take()}},
 7=>{let count=native::length(reader)?;native::entities(entities,count,limits)?;children(*entities,count,limits)?;let mut entries=native::Owned::new(control.allocate_vec::<SemioValueEntry>(count)?);control.scoped_stage(|control|{control.begin_stage(count)?;for _ in 0..count{let key=native::text(reader,control)?;let value=value_binary(reader,control,limits,entities)?;entries.get_mut().push(SemioValueEntry{key,value});control.step()?;}Ok::<_,String>(())})?;SemioValue::Map{entries:entries.take()}},
 _=>return Err("unknown Semio native binary value tag".into())})
 })
}

fn children(entities:usize,count:usize,limits:SqliteDatabaseLimits)->Result<(),String>{if count>limits.max_rows.saturating_sub(entities){Err("Semio native values exceed remaining row limit".into())}else{Ok(())}}
