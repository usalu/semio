//! 🔤️ Explicit controlled native text run and inline mark construction.
use crate::schema::snapshot::{SemioTextSnapshot, SemioTextMarkKind, STDIO_SEMIOTEXT_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_semio_base::io::sqlite::snapshot::native_decoding::{self as native,workspace};
use semio_framework_value::{native_decoding::NativeDecodeControl,ValueError,ValueRefusalKind};
use store::sqlite_snapshot::{SqliteSnapshotControl,SqliteDatabaseLimits};

/// 🛬️ Separates complete SQL cells from the same caller's actual native allocation backing.
pub(crate)fn decode(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control:&mut NativeDecodeControl<'_>)->Result<SemioTextSnapshot,ValueError>{
 crate::io::sqlite::snapshot::admit_layout(control.limits())?;
 native::with_body(payload,STDIO_SEMIOTEXT_DOCUMENT_SCHEMA,control,native_control,|body,decoder,limits|workspace::decode(decoder,empty,|output,decoder|match body{native::Body::Binary(body)=>binary_into(output,body,decoder,limits),native::Body::Text(body)=>document_into(output,body,decoder,limits)}))
}

/// 🧩️ Fills the retained text root from borrowed binary octets, inserting every run and mark before its fields.
pub fn binary_into(output:&mut SemioTextSnapshot,body:&[u8],control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<(),ValueError>{
 crate::io::sqlite::snapshot::admit_binary(body,control,limits)?;let mut entities=0;native::entities(&mut entities,1,limits)?;let mut reader=store::ByteReader::new(body);if reader.read_u8().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unsupported Semio text native format"))}
 native::reader_text_into(&mut output.schema,&mut reader,control)?;
 native::binary_list_into(&mut output.runs,&mut reader,control,limits,&mut entities,Default::default,|run,reader,control,entities|{
  native::reader_text_into(&mut run.language,reader,control)?;native::reader_text_into(&mut run.content,reader,control)?;
  native::binary_list_into(&mut run.marks,reader,control,limits,entities,Default::default,|mark,reader,control,_|{mark.kind=crate::io::binary::snapshot::mark_kind_from_tag(reader.read_u8().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?).map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error))?;native::reader_text_into(&mut mark.href,reader,control)})
 })?;
 if reader.remaining()!=0{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio text native trailing bytes"))}Ok(())
}
/// 📝️ Fills the retained text root from the borrowed document body, inserting every run and mark before its fields.
pub fn document_into(output:&mut SemioTextSnapshot,body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<(),ValueError>{
 crate::io::sqlite::snapshot::admit_document(body,control,limits)?;let mut entities=0;native::entities(&mut entities,1,limits)?;let fields=native::fields(body,["schema","runs"],control)?;native::hex_text_into(&mut output.schema,fields[0].ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Semio text native schema field missing"))?,control)?;
 native::text_list_into(&mut output.runs,fields[1].unwrap_or("[]"),control,limits,&mut entities,Default::default,|run,value,control,entities|{
  let[language,content,mark_fields]=native::record(value,control)?;native::hex_text_into(&mut run.language,language,control)?;native::hex_text_into(&mut run.content,content,control)?;
  native::text_list_into(&mut run.marks,mark_fields,control,limits,entities,Default::default,|mark,value,control,_|{let[kind,href]=native::record(value,control)?;mark.kind=match kind{"b"=>SemioTextMarkKind::Bold,"i"=>SemioTextMarkKind::Italic,"c"=>SemioTextMarkKind::Code,"l"=>SemioTextMarkKind::Link,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio text native mark kind"))};native::hex_text_into(&mut mark.href,href,control)})
 })
}
/// 🕳️ The empty typed root every native prefix starts from.
pub fn empty()->SemioTextSnapshot{SemioTextSnapshot{schema:String::new(),runs:Vec::new()}}
