//! ☁️ Borrowed LAS roles admit every handcrafted semantic cell before actual ownership.
use super::*;
use semio_framework_dsl_record::{FieldValue as F,RecordValue as R};
use semio_framework_value::NativeDecodeControl;
use semio_framework_os_kernel::sqlite_snapshot::SqliteDatabaseLimits;
type Result<T>=std::result::Result<T,ValueError>;
const TABLES:[(&str,usize);8]=[("las_document",2),("las_header",50),("las_return_histogram",4),("las_vlr",6),("las_vlr_octet",4),("las_point",21),("las_point_gps",4),("las_point_rgb",4)];
fn invalid()->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,"LAS native field differs from its authored literal role")}
fn sum(left:usize,right:usize)->Result<usize>{left.checked_add(right).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"LAS semantic byte extent overflow"))}
pub(super)fn extent(limits:SqliteDatabaseLimits)->Result<()>{
 let bytes=LasSnapshot::SQLITE_SCHEMA.split(';').map(str::trim).try_fold(0usize,|sum,sql|super::semantic::sum(sum,sql.len()))?;
 let bytes=TABLES.iter().try_fold(bytes,|sum,(name,_)|super::semantic::sum(sum,name.len()))?;
 if bytes.max(LasSnapshot::SQLITE_SCHEMA.len())>limits.max_schema_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"LAS authored schema exceeds caller bytes"))}
 if TABLES.len()>limits.max_tables||TABLES.iter().any(|(_,width)|*width>limits.max_columns)||limits.max_rows<7{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"LAS authored relational extent exceeds caller limits"))}Ok(())
}
fn exact(value:&R,count:usize)->Result<()>{if value.fields.len()!=count||value.fields.keys().any(|id|usize::from(*id)>=count){return Err(invalid())}Ok(())}
fn field(value:&R,id:u16)->Result<&F>{value.get(id).ok_or_else(invalid)}
fn record(value:&F,count:usize)->Result<&R>{let F::Record(value)=value else{return Err(invalid())};exact(value,count)?;Ok(value)}
fn list(value:&F)->Result<&[F]>{let F::List(value)=value else{return Err(invalid())};Ok(value)}
fn uint(value:&F,maximum:u64)->Result<u64>{match value{F::UInt(value)if *value<=maximum=>Ok(*value),_=>Err(invalid())}}
fn word(value:&F)->Result<usize>{Ok(super::scalar_bytes(f64::from_bits(uint(value,u64::MAX)?)))}
fn text(value:&F,native:&mut NativeDecodeControl<'_>)->Result<usize>{let F::Text(value)=value else{return Err(invalid())};native.scoped_stage(|native|{native.begin_stage(value.len())?;for part in value.as_bytes().chunks(256){native.advance(part.len())?;}Ok::<_,ValueError>(())})?;Ok(value.len())}
struct Census{limits:SqliteDatabaseLimits,rows:usize,bytes:usize}
impl Census{
 fn row(&mut self,bytes:usize,native:&mut NativeDecodeControl<'_>)->Result<()>{let rows=self.rows.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"LAS semantic row extent overflow"))?;let total=sum(self.bytes,bytes)?;if rows>self.limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"LAS semantic rows exceed caller limit"))}if total>self.limits.max_value_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"LAS semantic values exceed caller limit"))}native.step()?;self.rows=rows;self.bytes=total;Ok(())}
}
/// 🫳️ Visits original raw word records without a typed Snapshot or owned SQL mirror.
pub(super)fn admit_record(value:&R,limits:SqliteDatabaseLimits,native:&mut NativeDecodeControl<'_>)->Result<()>{
 extent(limits)?;native.scoped_stage(|native|{
  native.begin_stage(0)?;exact(value,4)?;let mut census=Census{limits,rows:0,bytes:0};census.row(sum(8,text(field(value,0)?,native)?)?,native)?;
  let header=record(field(value,1)?,25)?;for(id,maximum)in[(0,u8::MAX as u64),(1,u8::MAX as u64),(4,u16::MAX as u64),(5,u16::MAX as u64),(6,u16::MAX as u64),(7,u32::MAX as u64),(8,u32::MAX as u64),(9,u8::MAX as u64),(10,u16::MAX as u64),(11,u32::MAX as u64)]{uint(field(header,id)?,maximum)?;native.step()?;}
  let mut bytes=sum(sum(96,text(field(header,2)?,native)?)?,text(field(header,3)?,native)?)?;for id in 13..25{bytes=sum(bytes,word(field(header,id)?)?)?;native.step()?;}census.row(bytes,native)?;
  let F::Tuple(histogram)=field(header,12)?else{return Err(invalid())};if histogram.len()!=5{return Err(invalid())}for count in histogram{uint(count,u32::MAX as u64)?;census.row(32,native)?;}
  for value in list(field(value,2)?)?{let value=record(value,4)?;uint(field(value,1)?,u16::MAX as u64)?;census.row(sum(sum(32,text(field(value,0)?,native)?)?,text(field(value,2)?,native)?)?,native)?;for byte in list(field(value,3)?)?{uint(byte,u8::MAX as u64)?;census.row(32,native)?;}}
  for value in list(field(value,3)?)?{let value=record(value,14)?;let mut bytes=96usize;for id in 0..3{bytes=sum(bytes,word(field(value,id)?)?)?;}
   for(id,maximum)in[(3,u16::MAX as u64),(4,u8::MAX as u64),(5,u8::MAX as u64),(8,u8::MAX as u64),(10,u8::MAX as u64),(11,u16::MAX as u64)]{uint(field(value,id)?,maximum)?;native.step()?;}for id in[6,7]{if !matches!(field(value,id)?,F::Bool(_)){return Err(invalid())}native.step()?;}if !matches!(field(value,9)?,F::Int(value)if i8::try_from(*value).is_ok()){return Err(invalid())}census.row(bytes,native)?;
   match field(value,12)?{F::Absent=>(),value=>census.row(sum(8,word(value)?)?,native)?}
   match field(value,13)?{F::Absent=>(),value=>{let value=record(value,3)?;for id in 0..3{uint(field(value,id)?,u16::MAX as u64)?;}census.row(32,native)?;}}
  }native.checkpoint()
 })
}
