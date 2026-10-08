//! 🎬️ Controlled native stream and sample ownership keeps full integer widths.
use crate::standards::v1::subsets::video::schema::snapshot::{SemioVideoSnapshot,SemioVideoStream,SemioVideoStreamKind,SemioVideoSample,SemioRational,STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding as native;
use semio_framework_value::{native_decoding::NativeDecodeControl,ValueError,ValueRefusalKind};
use store::sqlite_snapshot::{SqliteSnapshotControl,SqliteDatabaseLimits};
/// 🛬️ Separates complete SQL cells from the same caller's actual native allocation backing.
pub(crate)fn decode(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<SemioVideoSnapshot,ValueError>{
 let limits=control.limits();crate::standards::v1::subsets::video::io::sqlite::snapshot::admit_layout(limits)?;
 let size=match payload{store::io::IoPayload::Binary(value)=>value.len(),store::io::IoPayload::Text(value)=>value.len()};if size>limits.max_file_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio Video native input exceeds file limit"))}
 control.allocation_stage(store::sqlite_snapshot::SqliteSnapshotPhase::DecodeNative,|remaining,checkpoint|{
  let mut callback=|event:semio_framework_value::native_decoding::NativeDecodeProgress|checkpoint(event.completed,event.total);let mut native_control=NativeDecodeControl::new(remaining,&mut callback);
  let result=(||->Result<SemioVideoSnapshot,ValueError>{let result=match payload{
   store::io::IoPayload::Binary(value)=>{let body=store::semio_format::unwrap_binary_controlled(value,STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA,store::semio_format::Component::Pack,1,&mut native_control).map_err(store::semio_format::SemioError::into_value_error)?;binary(body,&mut native_control,limits)?},
   store::io::IoPayload::Text(value)=>{let body=store::semio_format::split_text_preamble_controlled(value,STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA,store::semio_format::Component::Dsl,1,&mut native_control).map_err(store::semio_format::SemioError::into_value_error)?;document(body,&mut native_control,limits)?}
  };let result=native::Owned::new(result);native_control.checkpoint()?;Ok(result.take())})();(result,native_control.owned_bytes())
 })?
}

fn integer(reader:&mut store::ByteReader<'_>)->Result<i64,ValueError>{Ok(i64::from_le_bytes(reader.read_bytes(8).map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?.try_into().map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"Semio video native rational truncated"))?))}
pub(crate) fn binary(body:&[u8],control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<SemioVideoSnapshot,ValueError>{
 crate::standards::v1::subsets::video::io::sqlite::snapshot::admit_binary(body,control,limits)?;let mut entities=0;native::entities(&mut entities,1,limits)?;let mut reader=store::ByteReader::new(body);if reader.read_u8().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unsupported Semio video native format"))}
 let schema=native::text(&mut reader,control)?;let count=native::length(&mut reader)?;native::entities(&mut entities,count,limits)?;let mut streams=control.allocate_vec::<SemioVideoStream>(count)?;
 control.scoped_stage(|control|{control.begin_stage(count)?;for _ in 0..count{
  let kind=crate::standards::v1::subsets::video::io::binary::snapshot::kind_from_tag(reader.read_u8().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?).map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error))?;let codec=native::text(&mut reader,control)?;let width=reader.read_u32_le().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?;let height=reader.read_u32_le().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?;let rate=SemioRational{num:integer(&mut reader)?,den:integer(&mut reader)?};
  let count=native::length(&mut reader)?;native::entities(&mut entities,count,limits)?;let mut samples=control.allocate_vec::<SemioVideoSample>(count)?;
  control.scoped_stage(|control|{control.begin_stage(count)?;for _ in 0..count{let pts=reader.read_u64_le().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?;let key=match reader.read_u8().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?{0=>false,1=>true,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio video native key flag"))};let data=control.copy_bytes(native::bytes(&mut reader)?)?;samples.push(SemioVideoSample{pts,key,data});control.step()?;}Ok::<_,ValueError>(())})?;
  streams.push(SemioVideoStream{kind,codec,width,height,rate,samples});control.step()?;
 }Ok::<_,ValueError>(())})?;
 if reader.remaining()!=0{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio video native trailing bytes"))}Ok(SemioVideoSnapshot{schema,streams})
}
pub(crate) fn document(body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<SemioVideoSnapshot,ValueError>{
 crate::standards::v1::subsets::video::io::sqlite::snapshot::admit_document(body,control,limits)?;let mut entities=0;native::entities(&mut entities,1,limits)?;let fields=native::fields(body,["schema","streams"],control)?;let schema=native::hex_text(fields[0].ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Semio video native schema missing"))?,control)?;let mut items=native::Items::new(fields[1].unwrap_or("[]"))?;let count=items.count(control,limits.max_rows)?;native::entities(&mut entities,count,limits)?;let mut streams=control.allocate_vec::<SemioVideoStream>(count)?;
 control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(value)=items.next(control)?{
  let[kind,codec,width,height,rate,sample_fields]=native::record(value,control)?;let kind=match kind{"V"=>SemioVideoStreamKind::Video,"A"=>SemioVideoStreamKind::Audio,"S"=>SemioVideoStreamKind::Subtitle,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio video native stream kind"))};let codec=native::hex_text(codec,control)?;let width=width.parse::<u32>().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?;let height=height.parse::<u32>().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?;let[num,den]=native::record(rate,control)?;let rate=SemioRational{num:num.parse::<i64>().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?,den:den.parse::<i64>().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?};
  let mut sample_items=native::Items::new(sample_fields)?;let count=sample_items.count(control,limits.max_rows)?;native::entities(&mut entities,count,limits)?;let mut samples=control.allocate_vec::<SemioVideoSample>(count)?;
  control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(value)=sample_items.next(control)?{let[pts,key,data]=native::record(value,control)?;let pts=pts.parse::<u64>().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?;let key=match key{"0"=>false,"1"=>true,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio video native key flag"))};let data=native::hex(data,control)?;samples.push(SemioVideoSample{pts,key,data});control.step()?;}Ok::<_,ValueError>(())})?;
  streams.push(SemioVideoStream{kind,codec,width,height,rate,samples});control.step()?;
 }Ok::<_,ValueError>(())})?;Ok(SemioVideoSnapshot{schema,streams})
}

