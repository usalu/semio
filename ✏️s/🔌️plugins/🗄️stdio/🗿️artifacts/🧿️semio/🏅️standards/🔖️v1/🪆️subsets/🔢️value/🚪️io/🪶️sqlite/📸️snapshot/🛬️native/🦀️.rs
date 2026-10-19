//! 🌳️ Controlled native typed values preserve lexemes, ordered duplicates and explicit references.
use semio_framework_value::{ValueError,ValueRefusalKind};
use crate::schema::snapshot::{SemioValue, SemioValueEntry, SemioValueNode, SemioValueSnapshot, ValueId, STDIO_SEMIOVALUE_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_semio_base::io::sqlite::snapshot::native_decoding::{self as native,workspace};
use semio_framework_value::native_decoding::NativeDecodeControl;
use store::sqlite_snapshot::{SqliteSnapshotControl,SqliteDatabaseLimits};
/// 🛬️ Separates complete SQL cells from the same caller's actual native allocation backing.
pub(crate)fn decode(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control:&mut NativeDecodeControl<'_>)->Result<SemioValueSnapshot,ValueError>{
 crate::io::sqlite::snapshot::admit_layout(control.limits())?;
 native::with_body(payload,STDIO_SEMIOVALUE_DOCUMENT_SCHEMA,control,native_control,|body,decoder,limits|workspace::decode(decoder,empty,|output,decoder|match body{native::Body::Binary(body)=>binary_into(output,body,decoder,limits),native::Body::Text(body)=>document_into(output,body,decoder,limits)}))
}
/// 🧩️ Validates the borrowed binary body as the same text document and fills the retained value root.
pub fn binary_into(output:&mut SemioValueSnapshot,body:&[u8],control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<(),ValueError>{let body=control.borrow_text(body)?;document_into(output,body,control,limits)}
/// 📝️ Fills the retained value root from the borrowed document body, inserting every graph node before its identity and value.
pub fn document_into(output:&mut SemioValueSnapshot,body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<(),ValueError>{
 crate::io::sqlite::snapshot::admit_document(body,control,limits)?; let mut entities=0;native::entities(&mut entities,1,limits)?;let[schema,root,nodes]=native::record(body,control)?;native::hex_text_into(&mut output.schema,schema,control)?;value_text_into(&mut output.root,root,control,limits,&mut entities)?;
 let mut items=native::Items::new(nodes)?;let count=items.count(control,limits.max_rows)?;native::entities(&mut entities,count,limits)?;children(entities,count,limits)?;output.nodes=control.allocate_vec::<SemioValueNode>(count)?;
 control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(item)=items.next(control)?{let(id,value)=item.split_once(':').ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Semio native value node requires colon"))?;output.nodes.push(SemioValueNode::default());let node=output.nodes.last_mut().unwrap();native::hex_text_into(&mut node.id.value,id,control)?;value_text_into(&mut node.value,value,control,limits,&mut entities)?;control.step()?;}Ok::<_,ValueError>(())})
}
/// 🔷️ Selects each text value variant in its retained slot before copying its lexeme, key or child.
pub fn value_text_into(output:&mut SemioValue,value:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits,entities:&mut usize)->Result<(),ValueError>{
 control.scoped_depth(64,|control|{
 native::entities(entities,1,limits)?;control.checkpoint()?;if value=="Z"{*output=SemioValue::Null;return Ok(())}let(tag,body)=value.split_at_checked(1).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Semio native value tag missing"))?;let body=body.strip_prefix('[').and_then(|body|body.strip_suffix(']')).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Semio native value requires brackets"))?;
 match tag{
 "B"=>*output=SemioValue::Bool{value:match body{"0"=>false,"1"=>true,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native boolean"))}},
 "I"=>{*output=SemioValue::Int{lexeme:String::new()};let SemioValue::Int{lexeme}=&mut *output else{unreachable!()};native::hex_text_into(lexeme,body,control)?;},
 "F"=>{*output=SemioValue::Float{lexeme:String::new()};let SemioValue::Float{lexeme}=&mut *output else{unreachable!()};native::hex_text_into(lexeme,body,control)?;},
 "S"=>{*output=SemioValue::Str{value:String::new()};let SemioValue::Str{value}=&mut *output else{unreachable!()};native::hex_text_into(value,body,control)?;},
 "Y"=>{*output=SemioValue::Bytes{value:Vec::new()};let SemioValue::Bytes{value}=&mut *output else{unreachable!()};native::hex_bytes_into(value,body,control)?;},
 "R"=>{*output=SemioValue::Ref{id:ValueId{value:String::new()}};let SemioValue::Ref{id}=&mut *output else{unreachable!()};native::hex_text_into(&mut id.value,body,control)?;},
 "L"=>{*output=SemioValue::List{items:Vec::new()};let SemioValue::List{items}=&mut *output else{unreachable!()};let mut source=native::Items::new(&value[1..])?;let count=source.count(control,limits.max_rows)?;native::entities(entities,count,limits)?;children(*entities,count,limits)?;*items=control.allocate_vec::<SemioValue>(count)?;control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(item)=source.next(control)?{items.push(SemioValue::Null);value_text_into(items.last_mut().unwrap(),item,control,limits,entities)?;control.step()?;}Ok::<_,ValueError>(())})?;},
 "M"=>{*output=SemioValue::Map{entries:Vec::new()};let SemioValue::Map{entries}=&mut *output else{unreachable!()};let mut source=native::Items::new(&value[1..])?;let count=source.count(control,limits.max_rows)?;native::entities(entities,count,limits)?;children(*entities,count,limits)?;*entries=control.allocate_vec::<SemioValueEntry>(count)?;control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(item)=source.next(control)?{let(key,value)=item.split_once(':').ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Semio native value entry requires colon"))?;entries.push(SemioValueEntry::default());let entry=entries.last_mut().unwrap();native::hex_text_into(&mut entry.key,key,control)?;value_text_into(&mut entry.value,value,control,limits,entities)?;control.step()?;}Ok::<_,ValueError>(())})?;},
 _=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown Semio native value tag"))}
 Ok(())
 })
}
/// 💠️ Selects each binary value variant in its retained slot before copying its lexeme, key or child.
pub fn value_binary_into(output:&mut SemioValue,reader:&mut store::ByteReader<'_>,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits,entities:&mut usize)->Result<(),ValueError>{
 control.scoped_depth(64,|control|{
 native::entities(entities,1,limits)?;control.checkpoint()?;let tag=reader.read_u8().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?;
 match tag{
 0=>*output=SemioValue::Null,
 1=>*output=SemioValue::Bool{value:match reader.read_u8().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?{0=>false,1=>true,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native binary boolean"))}},
 2=>{*output=SemioValue::Int{lexeme:String::new()};let SemioValue::Int{lexeme}=&mut *output else{unreachable!()};native::reader_text_into(lexeme,reader,control)?;},
 3=>{*output=SemioValue::Float{lexeme:String::new()};let SemioValue::Float{lexeme}=&mut *output else{unreachable!()};native::reader_text_into(lexeme,reader,control)?;},
 4=>{*output=SemioValue::Str{value:String::new()};let SemioValue::Str{value}=&mut *output else{unreachable!()};native::reader_text_into(value,reader,control)?;},
 5=>{*output=SemioValue::Bytes{value:Vec::new()};let SemioValue::Bytes{value}=&mut *output else{unreachable!()};native::reader_bytes_into(value,reader,control)?;},
 8=>{*output=SemioValue::Ref{id:ValueId{value:String::new()}};let SemioValue::Ref{id}=&mut *output else{unreachable!()};native::reader_text_into(&mut id.value,reader,control)?;},
 6=>{*output=SemioValue::List{items:Vec::new()};let SemioValue::List{items}=&mut *output else{unreachable!()};let count=native::length(reader)?;native::entities(entities,count,limits)?;children(*entities,count,limits)?;*items=control.allocate_vec::<SemioValue>(count)?;control.scoped_stage(|control|{control.begin_stage(count)?;for _ in 0..count{items.push(SemioValue::Null);value_binary_into(items.last_mut().unwrap(),reader,control,limits,entities)?;control.step()?;}Ok::<_,ValueError>(())})?;},
 7=>{*output=SemioValue::Map{entries:Vec::new()};let SemioValue::Map{entries}=&mut *output else{unreachable!()};let count=native::length(reader)?;native::entities(entities,count,limits)?;children(*entities,count,limits)?;*entries=control.allocate_vec::<SemioValueEntry>(count)?;control.scoped_stage(|control|{control.begin_stage(count)?;for _ in 0..count{entries.push(SemioValueEntry::default());let entry=entries.last_mut().unwrap();native::reader_text_into(&mut entry.key,reader,control)?;value_binary_into(&mut entry.value,reader,control,limits,entities)?;control.step()?;}Ok::<_,ValueError>(())})?;},
 _=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown Semio native binary value tag"))}
 Ok(())
 })
}

fn children(entities:usize,count:usize,limits:SqliteDatabaseLimits)->Result<(),ValueError>{if count>limits.max_rows.saturating_sub(entities){Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio native values exceed remaining row limit"))}else{Ok(())}}
/// 🕳️ The empty typed root every native prefix starts from.
pub fn empty()->SemioValueSnapshot{SemioValueSnapshot{schema:String::new(),root:SemioValue::Null,nodes:Vec::new()}}
