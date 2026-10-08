//! 🎵️ Borrowed MP3 literals admit every authored tag, header and INTEGER octet cell before ownership.
use super::*;
use semio_framework_dsl_record::{FieldValue as F,RecordValue};
use store::sqlite_snapshot::SqliteDatabaseLimits;
use semio_framework_value::NativeDecodeControl;
const TABLES:[(&str,usize);8]=[("mp3_document",2),("mp3_id3v2_tag",1),("mp3_id3v2_frame",11),("mp3_id3_text_value",4),("mp3_id3_content_octet",4),("mp3_id3v1_tag",8),("mp3_audio_frame",15),("mp3_audio_payload_octet",4)];
pub(super) fn extent(limits:SqliteDatabaseLimits)->Result<(),ValueError>{
 let mut bytes=0usize;for statement in Mp3Snapshot::SQLITE_SCHEMA.split(';'){bytes=bytes.checked_add(statement.trim().len()).ok_or_else(||work("MP3 authored schema extent overflow"))?;}for(name,_)in TABLES{bytes=bytes.checked_add(name.len()).ok_or_else(||work("MP3 authored schema name extent overflow"))?;}
 if bytes.max(Mp3Snapshot::SQLITE_SCHEMA.len())>limits.max_schema_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"MP3 authored schema exceeds caller bytes"))}if limits.max_tables<TABLES.len()||TABLES.iter().any(|(_,width)|*width>limits.max_columns)||limits.max_rows<1{return Err(work("MP3 authored relational extent exceeds caller limits"))}Ok(())
}
pub(super) fn typed(snapshot:&Mp3Snapshot,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{extent(control.limits())?;let mut writer=RowWriter::borrowed(control,phase)?;super::visit_rows(snapshot,&mut writer)?;writer.finish_borrowed()}
fn field(record:&RecordValue,id:u16)->Result<&F,ValueError>{record.get(id).ok_or_else(||invalid("MP3 native authored field is missing"))}
fn record(value:&F,count:usize)->Result<&RecordValue,ValueError>{let F::Record(record)=value else{return Err(invalid("MP3 literal Record required"))};if record.fields.len()!=count{return Err(invalid("MP3 native Record has undeclared fields"))}Ok(record)}
fn optional(value:&F,count:usize)->Result<Option<&RecordValue>,ValueError>{match value{F::Absent=>Ok(None),_=>record(value,count).map(Some)}}
fn list(value:&F)->Result<&[F],ValueError>{match value{F::List(values)=>Ok(values),_=>Err(invalid("MP3 literal List required"))}}
fn octets(value:&F)->Result<&[u8],ValueError>{match value{F::Bytes64(bytes)=>Ok(bytes),_=>Err(invalid("MP3 intrinsic octets require Bytes64"))}}
fn uint(value:&F,maximum:u64)->Result<(),ValueError>{if matches!(value,F::UInt(number)if *number<=maximum){Ok(())}else{Err(invalid("MP3 unsigned field exceeds its authored width"))}}
fn boolean(value:&F)->Result<(),ValueError>{if matches!(value,F::Bool(_)){Ok(())}else{Err(invalid("MP3 header flag requires Bool"))}}
fn add(bytes:&mut usize,amount:usize,maximum:usize)->Result<(),ValueError>{*bytes=bytes.checked_add(amount).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"MP3 semantic value byte count overflow"))?;if *bytes>maximum{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"MP3 semantic cells exceed caller bytes"))}Ok(())}
fn text(value:&F,bytes:&mut usize,maximum:usize,native:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{let F::Text(value)=value else{return Err(invalid("MP3 lexical field requires Text"))};add(bytes,value.len(),maximum)?;native.scoped_stage(|native|{native.begin_stage(value.len())?;for part in value.as_bytes().chunks(256){native.advance(part.len())?;}Ok::<_,ValueError>(())})}
fn bytes_rows(value:&F,bytes:&mut usize,maximum:usize,native:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{let value=octets(value)?;add(bytes,value.len().checked_mul(32).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"MP3 INTEGER octet cell extent overflow"))?,maximum)?;for part in value.chunks(256){native.advance(part.len())?;}Ok(())}
fn count(rows:&mut usize,amount:usize,limits:SqliteDatabaseLimits)->Result<(),ValueError>{*rows=rows.checked_add(amount).ok_or_else(||work("MP3 semantic row count overflow"))?;if *rows>limits.max_rows{return Err(work("MP3 semantic rows exceed caller limit"))}Ok(())}
pub(super) fn borrowed(root:&RecordValue,limits:SqliteDatabaseLimits,native:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{
 extent(limits)?;native.scoped_stage(|native|{
  if root.fields.len()!=4{return Err(invalid("MP3 native root requires four authored fields"));}native.begin_stage(0)?;let mut rows=1;let mut bytes=0;add(&mut bytes,8,limits.max_value_bytes)?;text(field(root,0)?,&mut bytes,limits.max_value_bytes,native)?;
  if let Some(tag)=optional(field(root,1)?,1)?{count(&mut rows,1,limits)?;add(&mut bytes,8,limits.max_value_bytes)?;for frame in list(field(tag,0)?)?{
   let frame=record(frame,2)?;let F::Text(id)=field(frame,0)? else{return Err(invalid("MP3 frame identifier text"));};if id.len()!=4||!id.chars().all(|c|c.is_ascii_uppercase()||c.is_ascii_digit()){return Err(invalid("MP3 frame identifier"));}
   let F::Statements(variants)=field(frame,1)? else{return Err(invalid("MP3 metadata tagged variant required"));};let[(keyword,content)]=variants.as_slice()else{return Err(invalid("MP3 metadata one tagged variant required"));};let kind=match keyword.as_str(){"text"=>"text","user-text"=>"userText","comment"=>"comment","lyrics"=>"lyrics","url"=>"url","user-url"=>"userUrl","picture"=>"picture","opaque"=>"opaque",_=>return Err(invalid("MP3 metadata variant"))};if kind!=id3_content_kind(id){return Err(invalid("MP3 identifier/content binding"));}
   count(&mut rows,1,limits)?;add(&mut bytes,24+kind.len(),limits.max_value_bytes)?;text(field(frame,0)?,&mut bytes,limits.max_value_bytes,native)?;
   let width=match kind{"text"|"url"|"opaque"=>1,"userText"|"userUrl"=>2,"comment"|"lyrics"=>3,"picture"=>4,_=>unreachable!()};if content.fields.len()!=width{return Err(invalid("MP3 metadata variant fields"));}
   let mut text_fields: &[u16]=&[];let values=match kind{"text"=>Some(field(content,0)?),"userText"=>{text_fields=&[0];Some(field(content,1)?)},"comment"|"lyrics"=>{text_fields=&[0,1,2];None},"url"=>{text_fields=&[0];None},"userUrl"=>{text_fields=&[0,1];None},"picture"=>{text_fields=&[0,2];uint(field(content,1)?,20)?;add(&mut bytes,8,limits.max_value_bytes)?;None},"opaque"=>None,_=>unreachable!()};
   for &id in text_fields{text(field(content,id)?,&mut bytes,limits.max_value_bytes,native)?;}
   if let Some(values)=values{let values=list(values)?;if values.is_empty(){return Err(invalid("MP3 nonempty metadata text values"));}for value in values{count(&mut rows,1,limits)?;add(&mut bytes,24,limits.max_value_bytes)?;text(value,&mut bytes,limits.max_value_bytes,native)?;native.step()?;}}
   if matches!(kind,"opaque"|"picture"){let body=field(content,if kind=="picture"{3}else{0})?;count(&mut rows,octets(body)?.len(),limits)?;bytes_rows(body,&mut bytes,limits.max_value_bytes,native)?;}native.step()?;
  }}
  if let Some(trailer)=optional(field(root,3)?,7)?{count(&mut rows,1,limits)?;add(&mut bytes,8,limits.max_value_bytes)?;for id in 0..5{text(field(trailer,id)?,&mut bytes,limits.max_value_bytes,native)?;}for id in 5..7{let value=field(trailer,id)?;if !matches!(value,F::Absent){uint(value,if id==5{255}else{254})?;add(&mut bytes,8,limits.max_value_bytes)?;}}native.step()?;}
  for frame in list(field(root,2)?)?{count(&mut rows,1,limits)?;let frame=record(frame,2)?;let header=record(field(frame,0)?,12)?;for id in [0,1,3,4,7,8,11]{uint(field(header,id)?,u64::from(u8::MAX))?;}for id in [2,5,6,9,10]{boolean(field(header,id)?)?;}add(&mut bytes,120,limits.max_value_bytes)?;count(&mut rows,octets(field(frame,1)?)?.len(),limits)?;bytes_rows(field(frame,1)?,&mut bytes,limits.max_value_bytes,native)?;native.step()?;}native.checkpoint()
 })
}
