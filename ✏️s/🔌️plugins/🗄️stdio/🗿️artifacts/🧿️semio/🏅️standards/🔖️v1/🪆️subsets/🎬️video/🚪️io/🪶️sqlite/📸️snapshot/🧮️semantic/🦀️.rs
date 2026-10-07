//! 🎬️ Borrowed stream primitives admit full canonical timestamp and raw sample cells before typed construction.
use super::{SemioVideoSnapshot,ValueError,ValueRefusalKind};
use store::{ArtifactSqliteSnapshot,sqlite_snapshot::SqliteDatabaseLimits};
use semio_framework_value::native_decoding::NativeDecodeControl;
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding as native;
type Result<T>=std::result::Result<T,ValueError>;
fn invalid()->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,"Semio Video native field differs from its declared shape")}
fn add(a:usize,b:usize)->Result<usize>{a.checked_add(b).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio Video complete cell extent overflow"))}
/// 🏛️ Admits the full authored three-table layout before an Video owner is allocated.
pub(super)fn layout(limits:SqliteDatabaseLimits)->Result<()>{
 let mut bytes=0;for statement in SemioVideoSnapshot::SQLITE_SCHEMA.split(';'){bytes=add(bytes,statement.trim().len())?;}for name in["semio_video_document","semio_video_stream","semio_video_sample"]{bytes=add(bytes,name.len())?;}
 if bytes.max(SemioVideoSnapshot::SQLITE_SCHEMA.len())>limits.max_schema_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio Video schema exceeds caller bytes"))}
 if limits.max_tables<3||limits.max_columns<9||limits.max_rows<1{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio Video authored layout exceeds copied limits"))}Ok(())
}
struct Census{limits:SqliteDatabaseLimits,rows:usize,bytes:usize}
impl Census{
 fn row(&mut self,bytes:usize)->Result<()>{self.rows=add(self.rows,1)?;self.bytes=add(self.bytes,bytes)?;if self.rows>self.limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio Video complete rows exceed caller limit"))}if self.bytes>self.limits.max_value_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio Video complete cells exceed caller bytes"))}Ok(())}
 fn future(&self,count:usize)->Result<()>{if add(self.rows,count)?>self.limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio Video declared rows exceed caller limit"))}Ok(())}
}
fn text(reader:&mut store::ByteReader<'_>,control:&mut NativeDecodeControl<'_>)->Result<usize>{Ok(control.borrow_text(native::bytes(reader)?)?.len())}
fn digits(mut value:u64)->usize{let mut digits=1;while value>=10{value/=10;digits+=1;}digits}
fn kind(value:u8)->Result<usize>{match value{0|1=>Ok(5),2=>Ok(8),_=>Err(invalid())}}
fn signed(reader:&mut store::ByteReader<'_>)->Result<()>{reader.read_bytes(8).map_err(|_|invalid())?;Ok(())}
fn hex_bytes(value:&str,control:&mut NativeDecodeControl<'_>)->Result<usize>{if !value.len().is_multiple_of(2){return Err(invalid())}control.scoped_stage(|control|{let count=value.len()/2;control.begin_stage(count)?;for pair in value.as_bytes().chunks_exact(2){if !pair.iter().all(u8::is_ascii_hexdigit){return Err(invalid())}control.step()?;}control.checkpoint()?;Ok(count)})}
/// 🎞️ Counts actual binary stream words, unsigned timestamps and borrowed BLOB extents.
pub(super)fn binary(body:&[u8],control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<()>{
 layout(limits)?;control.scoped_stage(|control|{let mut reader=store::ByteReader::new(body);if reader.read_u8().map_err(|_|invalid())?!=1{return Err(invalid())}let schema=text(&mut reader,control)?;let mut census=Census{limits,rows:0,bytes:0};census.row(add(8,schema)?)?;let count=native::length(&mut reader)?;census.future(count)?;control.begin_stage(count)?;
 for _ in 0..count{let kind=kind(reader.read_u8().map_err(|_|invalid())?)?;let codec=text(&mut reader,control)?;reader.read_u32_le().map_err(|_|invalid())?;reader.read_u32_le().map_err(|_|invalid())?;signed(&mut reader)?;signed(&mut reader)?;census.row(add(add(56,kind)?,codec)?)?;let count=native::length(&mut reader)?;census.future(count)?;control.scoped_stage(|control|{control.begin_stage(count)?;for _ in 0..count{let pts=reader.read_u64_le().map_err(|_|invalid())?;if reader.read_u8().map_err(|_|invalid())?>1{return Err(invalid())}let data=native::bytes(&mut reader)?;control.scoped_stage(|control|{control.begin_stage(data.len())?;for part in data.chunks(256){control.advance(part.len())?;}Ok::<_,ValueError>(())})?;census.row(add(add(32,digits(pts))?,data.len())?)?;control.step()?;}Ok::<_,ValueError>(())})?;control.step()?;}if reader.remaining()!=0{return Err(invalid())}control.checkpoint()})
}
/// 📝️ Preserves original lexical widths, required records and omitted-stream defaults.
pub(super)fn document(body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<()>{
 layout(limits)?;control.scoped_stage(|control|{let fields=native::fields(body,["schema","streams"],control)?;let schema=native::hex_text_extent(fields[0].ok_or_else(invalid)?,control)?;let mut census=Census{limits,rows:0,bytes:0};census.row(add(8,schema)?)?;let mut streams=native::Items::new(fields[1].unwrap_or("[]"))?;let count=streams.count(control,limits.max_rows)?;census.future(count)?;control.begin_stage(count)?;
 while let Some(value)=streams.next(control)?{let[kind,codec,width,height,rate,samples]=native::record(value,control)?;let kind=match kind{"V"|"A"=>5,"S"=>8,_=>return Err(invalid())};let codec=native::hex_text_extent(codec,control)?;width.parse::<u32>().map_err(|_|invalid())?;height.parse::<u32>().map_err(|_|invalid())?;let[num,den]=native::record(rate,control)?;num.parse::<i64>().map_err(|_|invalid())?;den.parse::<i64>().map_err(|_|invalid())?;census.row(add(add(56,kind)?,codec)?)?;let mut samples=native::Items::new(samples)?;let count=samples.count(control,limits.max_rows)?;census.future(count)?;control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(value)=samples.next(control)?{let[pts,key,data]=native::record(value,control)?;let pts=pts.parse::<u64>().map_err(|_|invalid())?;if !matches!(key,"0"|"1"){return Err(invalid())}let data=hex_bytes(data,control)?;census.row(add(add(32,digits(pts))?,data)?)?;control.step()?;}Ok::<_,ValueError>(())})?;control.step()?;}control.checkpoint()})
}
