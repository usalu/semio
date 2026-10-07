//! 🎞️ Borrowed Animation primitives count nullable timelines and every exact IEEE detail row.
use super::{SemioAnimationSnapshot,ValueError,ValueRefusalKind};
use store::{ArtifactSqliteSnapshot,sqlite_snapshot::SqliteDatabaseLimits};
use semio_framework_value::native_decoding::NativeDecodeControl;
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding as native;
type Result<T>=std::result::Result<T,ValueError>;
fn invalid()->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,"Semio Animation native field differs from its authored shape")}
fn add(a:usize,b:usize)->Result<usize>{a.checked_add(b).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio Animation complete cell extent overflow"))}
/// 🏛️ Admits all eight declared timeline and scalar detail tables before ownership.
pub(super)fn layout(limits:SqliteDatabaseLimits)->Result<()>{
 let mut bytes=0;for statement in SemioAnimationSnapshot::SQLITE_SCHEMA.split(';'){bytes=add(bytes,statement.trim().len())?;}for name in["semio_animation_document","semio_animation_timeline","semio_animation_channel","semio_animation_keyframe","semio_animation_scalar","semio_animation_vector3","semio_animation_quaternion","semio_animation_weight"]{bytes=add(bytes,name.len())?;}
 if bytes.max(SemioAnimationSnapshot::SQLITE_SCHEMA.len())>limits.max_schema_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio Animation schema exceeds caller bytes"))}
 if limits.max_tables<8||limits.max_columns<13||limits.max_rows<1{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio Animation authored layout exceeds copied limits"))}Ok(())
}
struct Census{limits:SqliteDatabaseLimits,rows:usize,bytes:usize}
impl Census{
 fn row(&mut self,bytes:usize)->Result<()>{self.rows=add(self.rows,1)?;self.bytes=add(self.bytes,bytes)?;if self.rows>self.limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio Animation complete rows exceed caller limit"))}if self.bytes>self.limits.max_value_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio Animation complete cells exceed caller bytes"))}Ok(())}
 fn future(&self,count:usize)->Result<()>{if add(self.rows,count)?>self.limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio Animation declared rows exceed caller limit"))}Ok(())}
}
fn text(reader:&mut store::ByteReader<'_>,control:&mut NativeDecodeControl<'_>)->Result<usize>{Ok(control.borrow_text(native::bytes(reader)?)?.len())}
fn scalar(value:f64)->usize{if value.is_nan(){11}else if value.is_infinite(){32}else{22}}
fn binary_scalar(reader:&mut store::ByteReader<'_>)->Result<usize>{Ok(scalar(reader.read_f64_le().map_err(|_|invalid())?))}
fn interpolation(tag:u8)->Result<usize>{match tag{0=>Ok(6),1=>Ok(4),2=>Ok(12),_=>Err(invalid())}}
fn kind(tag:u8)->Result<usize>{match tag{0=>Ok(6),1|3=>Ok(7),2=>Ok(10),_=>Err(invalid())}}
fn binary_value(tag:u8,reader:&mut store::ByteReader<'_>,census:&mut Census,control:&mut NativeDecodeControl<'_>)->Result<()>{
 match tag{
  0..=2=>{let count=match tag{0=>1,1=>3,_=>4};let mut bytes=8;for _ in 0..count{bytes=add(bytes,binary_scalar(reader)?)?;}census.row(bytes)?;},
  3=>{let count=native::length(reader)?;census.future(count)?;control.scoped_stage(|control|{control.begin_stage(count)?;for _ in 0..count{census.row(add(24,binary_scalar(reader)?)?)?;control.step()?;}control.checkpoint()})?;},
  _=>return Err(invalid())
 }Ok(())
}
/// 📦️ Counts each real discriminant and source word without materializing a timeline.
pub(super)fn binary(body:&[u8],control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<()>{
 layout(limits)?;control.scoped_stage(|control|{
  let mut reader=store::ByteReader::new(body);if reader.read_u8().map_err(|_|invalid())?!=1{return Err(invalid())}
  let mut census=Census{limits,rows:0,bytes:0};census.row(add(8,text(&mut reader,control)?)?)?;let count=native::length(&mut reader)?;census.future(count)?;control.begin_stage(count)?;
  for _ in 0..count{
   let name=match reader.read_u8().map_err(|_|invalid())?{0=>0,1=>text(&mut reader,control)?,_=>return Err(invalid())};census.row(add(24,name)?)?;let count=native::length(&mut reader)?;census.future(count)?;
   control.scoped_stage(|control|{control.begin_stage(count)?;for _ in 0..count{
    let node=text(&mut reader,control)?;let property=match reader.read_u8().map_err(|_|invalid())?{0=>11,1=>8,2=>5,3=>7,4=>add(6,text(&mut reader,control)?)?,_=>return Err(invalid())};let interpolation=interpolation(reader.read_u8().map_err(|_|invalid())?)?;census.row(add(add(add(24,node)?,property)?,interpolation)?)?;let count=native::length(&mut reader)?;census.future(count)?;
    control.scoped_stage(|control|{control.begin_stage(count)?;for _ in 0..count{let time=binary_scalar(&mut reader)?;let tag=reader.read_u8().map_err(|_|invalid())?;census.row(add(add(24,time)?,kind(tag)?)?)?;binary_value(tag,&mut reader,&mut census,control)?;control.step()?;}control.checkpoint()})?;control.step()?;
   }control.checkpoint()})?;control.step()?;
  }if reader.remaining()!=0{return Err(invalid())}control.checkpoint()
 })
}
fn document_value(value:&str,time:usize,census:&mut Census,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<()>{
 let(tag,body)=value.split_once(':').ok_or_else(invalid)?;let tag=match tag{"S"=>0,"V"=>1,"Q"=>2,"W"=>3,_=>return Err(invalid())};census.row(add(add(24,time)?,kind(tag)?)?)?;
 match tag{
  0=>census.row(add(8,scalar(native::float(body,control)?))?)?,
  1=>{let[x,y,z]=native::record(body,control)?;let mut bytes=8;for value in[x,y,z]{bytes=add(bytes,scalar(native::float(value,control)?))?;}census.row(bytes)?;},
  2=>{let[x,y,z,w]=native::record(body,control)?;let mut bytes=8;for value in[x,y,z,w]{bytes=add(bytes,scalar(native::float(value,control)?))?;}census.row(bytes)?;},
  _=>{let mut items=native::Items::new(body)?;let count=items.count(control,limits.max_rows)?;census.future(count)?;control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(value)=items.next(control)?{census.row(add(24,scalar(native::float(value,control)?))?)?;control.step()?;}control.checkpoint()})?;}
 }Ok(())
}
/// 📝️ Preserves optional names, all target tags and empty weight keyframes in the actual grammar.
pub(super)fn document(body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<()>{
 layout(limits)?;control.scoped_stage(|control|{
  let fields=native::fields(body,["schema","timelines"],control)?;let mut census=Census{limits,rows:0,bytes:0};census.row(add(8,native::hex_text_extent(fields[0].ok_or_else(invalid)?,control)?)?)?;let mut timelines=native::Items::new(fields[1].unwrap_or("[]"))?;let count=timelines.count(control,limits.max_rows)?;census.future(count)?;control.begin_stage(count)?;
  while let Some(value)=timelines.next(control)?{
   let[name,channels]=native::record(value,control)?;let name=if name=="[0]"{0}else{let[tag,name]=native::record(name,control)?;if tag!="1"{return Err(invalid())}native::hex_text_extent(name,control)?};census.row(add(24,name)?)?;let mut channels=native::Items::new(channels)?;let count=channels.count(control,limits.max_rows)?;census.future(count)?;
   control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(value)=channels.next(control)?{
    let[target,interpolation_fields,keyframes]=native::record(value,control)?;let[node,property]=native::record(target,control)?;let node=native::hex_text_extent(node,control)?;let property=match property{"t"=>11,"r"=>8,"s"=>5,"w"=>7,value=>add(6,native::hex_text_extent(value.strip_prefix("c:").ok_or_else(invalid)?,control)?)?};let interpolation=match interpolation_fields{"l"=>6,"s"=>4,"c"=>12,_=>return Err(invalid())};census.row(add(add(add(24,node)?,property)?,interpolation)?)?;let mut keyframes=native::Items::new(keyframes)?;let count=keyframes.count(control,limits.max_rows)?;census.future(count)?;
    control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(value)=keyframes.next(control)?{let[t,value]=native::record(value,control)?;let time=scalar(native::float(t,control)?);document_value(value,time,&mut census,control,limits)?;control.step()?;}control.checkpoint()})?;control.step()?;
   }control.checkpoint()})?;control.step()?;
  }control.checkpoint()
 })
}
