//! 🎬️ Controlled native stream and sample ownership keeps full integer widths.
use super::{SemioVideoSnapshot,SemioVideoStream,SemioVideoStreamKind,SemioVideoSample,SemioRational,STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA};
use crate::standards::v1::subsets::base::schema::snapshot::native_decoding as native;
use semio_framework_value::native_decoding::NativeDecodeControl;
use store::sqlite_snapshot::{SqliteSnapshotControl,SqliteDatabaseLimits};
pub(super) fn decode(payload:&store::os_io::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<SemioVideoSnapshot,String>{native::decode(payload,STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA,control,binary,document)}
fn integer(reader:&mut store::ByteReader<'_>)->Result<i64,String>{Ok(i64::from_le_bytes(reader.read_bytes(8).map_err(|e|e.to_string())?.try_into().map_err(|_|"Semio video native rational truncated")?))}
pub(crate) fn binary(body:&[u8],control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<SemioVideoSnapshot,String>{
 let mut entities=0;native::entities(&mut entities,1,limits)?;let mut reader=store::ByteReader::new(body);if reader.read_u8().map_err(|e|e.to_string())?!=1{return Err("unsupported Semio video native format".into())}
 let schema=native::text(&mut reader,control)?;let count=native::length(&mut reader)?;native::entities(&mut entities,count,limits)?;let mut streams=control.allocate_vec::<SemioVideoStream>(count)?;
 control.scoped_stage(|control|{control.begin_stage(count)?;for _ in 0..count{
  let kind=super::kind_from_tag(reader.read_u8().map_err(|e|e.to_string())?)?;let codec=native::text(&mut reader,control)?;let width=reader.read_u32_le().map_err(|e|e.to_string())?;let height=reader.read_u32_le().map_err(|e|e.to_string())?;let rate=SemioRational{num:integer(&mut reader)?,den:integer(&mut reader)?};
  let count=native::length(&mut reader)?;native::entities(&mut entities,count,limits)?;let mut samples=control.allocate_vec::<SemioVideoSample>(count)?;
  control.scoped_stage(|control|{control.begin_stage(count)?;for _ in 0..count{let pts=reader.read_u64_le().map_err(|e|e.to_string())?;let key=match reader.read_u8().map_err(|e|e.to_string())?{0=>false,1=>true,_=>return Err("invalid Semio video native key flag".into())};let data=control.copy_bytes(native::bytes(&mut reader)?)?;samples.push(SemioVideoSample{pts,key,data});control.step()?;}Ok::<_,String>(())})?;
  streams.push(SemioVideoStream{kind,codec,width,height,rate,samples});control.step()?;
 }Ok::<_,String>(())})?;
 if reader.remaining()!=0{return Err("Semio video native trailing bytes".into())}Ok(SemioVideoSnapshot{schema,streams})
}
pub(crate) fn document(body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<SemioVideoSnapshot,String>{
 let mut entities=0;native::entities(&mut entities,1,limits)?;let fields=native::fields(body,["schema","streams"],control)?;let schema=native::hex_text(fields[0].ok_or("Semio video native schema missing")?,control)?;let mut items=native::Items::new(fields[1].unwrap_or("[]"))?;let count=items.count(control,limits.max_rows)?;native::entities(&mut entities,count,limits)?;let mut streams=control.allocate_vec::<SemioVideoStream>(count)?;
 control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(value)=items.next(control)?{
  let[kind,codec,width,height,rate,sample_fields]=native::record(value,control)?;let kind=match kind{"V"=>SemioVideoStreamKind::Video,"A"=>SemioVideoStreamKind::Audio,"S"=>SemioVideoStreamKind::Subtitle,_=>return Err("invalid Semio video native stream kind".into())};let codec=native::hex_text(codec,control)?;let width=width.parse::<u32>().map_err(|e|e.to_string())?;let height=height.parse::<u32>().map_err(|e|e.to_string())?;let[num,den]=native::record(rate,control)?;let rate=SemioRational{num:num.parse::<i64>().map_err(|e|e.to_string())?,den:den.parse::<i64>().map_err(|e|e.to_string())?};
  let mut sample_items=native::Items::new(sample_fields)?;let count=sample_items.count(control,limits.max_rows)?;native::entities(&mut entities,count,limits)?;let mut samples=control.allocate_vec::<SemioVideoSample>(count)?;
  control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(value)=sample_items.next(control)?{let[pts,key,data]=native::record(value,control)?;let pts=pts.parse::<u64>().map_err(|e|e.to_string())?;let key=match key{"0"=>false,"1"=>true,_=>return Err("invalid Semio video native key flag".into())};let data=native::hex(data,control)?;samples.push(SemioVideoSample{pts,key,data});control.step()?;}Ok::<_,String>(())})?;
  streams.push(SemioVideoStream{kind,codec,width,height,rate,samples});control.step()?;
 }Ok::<_,String>(())})?;Ok(SemioVideoSnapshot{schema,streams})
}

