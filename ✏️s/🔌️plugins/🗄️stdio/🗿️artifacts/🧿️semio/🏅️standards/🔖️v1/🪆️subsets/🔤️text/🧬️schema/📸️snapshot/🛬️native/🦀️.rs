//! 🔤️ Explicit controlled native text run and inline mark construction.
use super::{SemioTextSnapshot,SemioTextRun,SemioTextMark,SemioTextMarkKind,STDIO_SEMIOTEXT_DOCUMENT_SCHEMA};
use crate::standards::v1::subsets::base::schema::snapshot::native_decoding as native;
use semio_framework_value::native_decoding::NativeDecodeControl;
use store::sqlite_snapshot::{SqliteSnapshotControl,SqliteDatabaseLimits};

pub(super) fn decode(payload:&store::os_io::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<SemioTextSnapshot,String>{native::decode(payload,STDIO_SEMIOTEXT_DOCUMENT_SCHEMA,control,binary,document)}
pub(crate) fn binary(body:&[u8],control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<SemioTextSnapshot,String>{
 let mut entities=0;native::entities(&mut entities,1,limits)?;let mut reader=store::ByteReader::new(body);if reader.read_u8().map_err(|e|e.to_string())?!=1{return Err("unsupported Semio text native format".into())}
 let schema=native::text(&mut reader,control)?;let count=native::length(&mut reader)?;native::entities(&mut entities,count,limits)?;let mut runs=control.allocate_vec::<SemioTextRun>(count)?;
 control.scoped_stage(|control|{control.begin_stage(count)?;for _ in 0..count{
  let language=native::text(&mut reader,control)?;let content=native::text(&mut reader,control)?;let count=native::length(&mut reader)?;native::entities(&mut entities,count,limits)?;let mut marks=control.allocate_vec::<SemioTextMark>(count)?;
  control.scoped_stage(|control|{control.begin_stage(count)?;for _ in 0..count{let kind=super::mark_kind_from_tag(reader.read_u8().map_err(|e|e.to_string())?)?;let href=native::text(&mut reader,control)?;marks.push(SemioTextMark{kind,href});control.step()?;}Ok::<_,String>(())})?;
  runs.push(SemioTextRun{language,content,marks});control.step()?;
 }Ok::<_,String>(())})?;
 if reader.remaining()!=0{return Err("Semio text native trailing bytes".into())}Ok(SemioTextSnapshot{schema,runs})
}
pub(crate) fn document(body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<SemioTextSnapshot,String>{
 let mut entities=0;native::entities(&mut entities,1,limits)?;let fields=native::fields(body,["schema","runs"],control)?;let schema=native::hex_text(fields[0].ok_or("Semio text native schema field missing")?,control)?;
 let mut items=native::Items::new(fields[1].unwrap_or("[]"))?;let count=items.count(control,limits.max_rows)?;native::entities(&mut entities,count,limits)?;let mut runs=control.allocate_vec::<SemioTextRun>(count)?;
 control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(value)=items.next(control)?{
  let[language,content,mark_fields]=native::record(value,control)?;let language=native::hex_text(language,control)?;let content=native::hex_text(content,control)?;let mut mark_items=native::Items::new(mark_fields)?;let count=mark_items.count(control,limits.max_rows)?;native::entities(&mut entities,count,limits)?;let mut marks=control.allocate_vec::<SemioTextMark>(count)?;
  control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(value)=mark_items.next(control)?{let[kind,href]=native::record(value,control)?;let kind=match kind{"b"=>SemioTextMarkKind::Bold,"i"=>SemioTextMarkKind::Italic,"c"=>SemioTextMarkKind::Code,"l"=>SemioTextMarkKind::Link,_=>return Err("invalid Semio text native mark kind".into())};let href=native::hex_text(href,control)?;marks.push(SemioTextMark{kind,href});control.step()?;}Ok::<_,String>(())})?;
  runs.push(SemioTextRun{language,content,marks});control.step()?;
 }Ok::<_,String>(())})?;Ok(SemioTextSnapshot{schema,runs})
}
