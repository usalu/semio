//! 🔤️ Explicit controlled native text run and inline mark construction.
use crate::standards::v1::subsets::text::schema::snapshot::{SemioTextSnapshot,SemioTextRun,SemioTextMark,SemioTextMarkKind,STDIO_SEMIOTEXT_DOCUMENT_SCHEMA};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding as native;
use semio_framework_value::{native_decoding::NativeDecodeControl,ValueError,ValueRefusalKind};
use store::sqlite_snapshot::{SqliteSnapshotControl,SqliteDatabaseLimits};

/// 🛬️ Separates complete SQL cells from the same caller's actual native allocation backing.
pub(crate)fn decode(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<SemioTextSnapshot,ValueError>{
 let limits=control.limits();crate::standards::v1::subsets::text::io::sqlite::snapshot::admit_layout(limits)?;
 let size=match payload{store::io::IoPayload::Binary(value)=>value.len(),store::io::IoPayload::Text(value)=>value.len()};if size>limits.max_file_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio text native input exceeds file limit"))}
 control.allocation_stage_native(store::sqlite_snapshot::SqliteSnapshotPhase::DecodeNative,|remaining,checkpoint|{
  let native_before=native_control.owned_bytes();let result=native_control.scoped_maximum(native_before.checked_add(remaining).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"native snapshot allowance overflow"))?,|native_control|{
  let result=(||->Result<SemioTextSnapshot,ValueError>{let result=match payload{
   store::io::IoPayload::Binary(value)=>{let body=store::semio_format::unwrap_binary_controlled(value,STDIO_SEMIOTEXT_DOCUMENT_SCHEMA,store::semio_format::Component::Pack,1,native_control).map_err(store::semio_format::SemioError::into_value_error)?;binary(body,native_control,limits)?},
   store::io::IoPayload::Text(value)=>{let body=store::semio_format::split_text_preamble_controlled(value,STDIO_SEMIOTEXT_DOCUMENT_SCHEMA,store::semio_format::Component::Dsl,1,native_control).map_err(store::semio_format::SemioError::into_value_error)?;document(body,native_control,limits)?}
  };let result=native::Owned::new(result);native_control.checkpoint()?;Ok(result.take())})();result});(result,native_control.owned_bytes().saturating_sub(native_before))
 })?
}

pub(crate) fn binary(body:&[u8],control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<SemioTextSnapshot,ValueError>{
 crate::standards::v1::subsets::text::io::sqlite::snapshot::admit_binary(body,control,limits)?;let mut entities=0;native::entities(&mut entities,1,limits)?;let mut reader=store::ByteReader::new(body);if reader.read_u8().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unsupported Semio text native format"))}
 let schema=native::text(&mut reader,control)?;let count=native::length(&mut reader)?;native::entities(&mut entities,count,limits)?;let mut runs=control.allocate_vec::<SemioTextRun>(count)?;
 control.scoped_stage(|control|{control.begin_stage(count)?;for _ in 0..count{
  let language=native::text(&mut reader,control)?;let content=native::text(&mut reader,control)?;let count=native::length(&mut reader)?;native::entities(&mut entities,count,limits)?;let mut marks=control.allocate_vec::<SemioTextMark>(count)?;
  control.scoped_stage(|control|{control.begin_stage(count)?;for _ in 0..count{let kind=crate::standards::v1::subsets::text::io::binary::snapshot::mark_kind_from_tag(reader.read_u8().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?).map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error))?;let href=native::text(&mut reader,control)?;marks.push(SemioTextMark{kind,href});control.step()?;}Ok::<_,ValueError>(())})?;
  runs.push(SemioTextRun{language,content,marks});control.step()?;
 }Ok::<_,ValueError>(())})?;
 if reader.remaining()!=0{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio text native trailing bytes"))}Ok(SemioTextSnapshot{schema,runs})
}
pub(crate) fn document(body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<SemioTextSnapshot,ValueError>{
 crate::standards::v1::subsets::text::io::sqlite::snapshot::admit_document(body,control,limits)?;let mut entities=0;native::entities(&mut entities,1,limits)?;let fields=native::fields(body,["schema","runs"],control)?;let schema=native::hex_text(fields[0].ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Semio text native schema field missing"))?,control)?;
 let mut items=native::Items::new(fields[1].unwrap_or("[]"))?;let count=items.count(control,limits.max_rows)?;native::entities(&mut entities,count,limits)?;let mut runs=control.allocate_vec::<SemioTextRun>(count)?;
 control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(value)=items.next(control)?{
  let[language,content,mark_fields]=native::record(value,control)?;let language=native::hex_text(language,control)?;let content=native::hex_text(content,control)?;let mut mark_items=native::Items::new(mark_fields)?;let count=mark_items.count(control,limits.max_rows)?;native::entities(&mut entities,count,limits)?;let mut marks=control.allocate_vec::<SemioTextMark>(count)?;
  control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(value)=mark_items.next(control)?{let[kind,href]=native::record(value,control)?;let kind=match kind{"b"=>SemioTextMarkKind::Bold,"i"=>SemioTextMarkKind::Italic,"c"=>SemioTextMarkKind::Code,"l"=>SemioTextMarkKind::Link,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio text native mark kind"))};let href=native::hex_text(href,control)?;marks.push(SemioTextMark{kind,href});control.step()?;}Ok::<_,ValueError>(())})?;
  runs.push(SemioTextRun{language,content,marks});control.step()?;
 }Ok::<_,ValueError>(())})?;Ok(SemioTextSnapshot{schema,runs})
}
