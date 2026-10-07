//! 🌊️ Borrowed Flow primitives count exact coordinates, ordered parameters and port-addressed edges.
use super::{SemioFlowSnapshot,ValueError,ValueRefusalKind};
use store::{ArtifactSqliteSnapshot,sqlite_snapshot::SqliteDatabaseLimits};
use semio_framework_value::native_decoding::NativeDecodeControl;
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding as native;
type Result<T>=std::result::Result<T,ValueError>;
fn invalid()->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,"Semio Flow native field differs from its authored shape")}
fn add(a:usize,b:usize)->Result<usize>{a.checked_add(b).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio Flow complete cell extent overflow"))}
/// 🏛️ Admits the exact four-table coordinate layout before native ownership.
pub(super)fn layout(limits:SqliteDatabaseLimits)->Result<()>{
 let mut bytes=0;for statement in SemioFlowSnapshot::SQLITE_SCHEMA.split(';'){bytes=add(bytes,statement.trim().len())?;}for name in["semio_flow_document","semio_flow_node","semio_flow_parameter","semio_flow_edge"]{bytes=add(bytes,name.len())?;}
 if bytes.max(SemioFlowSnapshot::SQLITE_SCHEMA.len())>limits.max_schema_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio Flow schema exceeds caller bytes"))}
 if limits.max_tables<4||limits.max_columns<12||limits.max_rows<1{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio Flow authored layout exceeds copied limits"))}Ok(())
}
struct Census{limits:SqliteDatabaseLimits,rows:usize,bytes:usize}
impl Census{
 fn row(&mut self,bytes:usize)->Result<()>{self.rows=add(self.rows,1)?;self.bytes=add(self.bytes,bytes)?;if self.rows>self.limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio Flow complete rows exceed caller limit"))}if self.bytes>self.limits.max_value_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio Flow complete cells exceed caller bytes"))}Ok(())}
 fn future(&self,count:usize)->Result<()>{if add(self.rows,count)?>self.limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio Flow declared rows exceed caller limit"))}Ok(())}
}
fn text(reader:&mut store::ByteReader<'_>,control:&mut NativeDecodeControl<'_>)->Result<usize>{Ok(control.borrow_text(native::bytes(reader)?)?.len())}
fn coordinate(value:f64)->usize{if value.is_nan(){11}else if value.is_infinite(){32}else{22}}
/// 📦️ Counts the real binary field order without allocating a node or parameter.
pub(super)fn binary(body:&[u8],control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<()>{
 layout(limits)?;control.scoped_stage(|control|{
  let mut reader=store::ByteReader::new(body);if reader.read_u8().map_err(|_|invalid())?!=1{return Err(invalid())}
  let mut census=Census{limits,rows:0,bytes:0};census.row(add(8,text(&mut reader,control)?)?)?;let count=native::length(&mut reader)?;census.future(count)?;control.begin_stage(count)?;
  for _ in 0..count{
   let mut bytes=add(add(add(24,text(&mut reader,control)?)?,text(&mut reader,control)?)?,text(&mut reader,control)?)?;let count=native::length(&mut reader)?;census.future(add(count,1)?)?;
   control.scoped_stage(|control|{control.begin_stage(count)?;for _ in 0..count{census.row(add(add(24,text(&mut reader,control)?)?,text(&mut reader,control)?)?)?;control.step()?;}Ok::<_,ValueError>(())})?;
   bytes=add(bytes,coordinate(reader.read_f64_le().map_err(|_|invalid())?))?;bytes=add(bytes,coordinate(reader.read_f64_le().map_err(|_|invalid())?))?;census.row(bytes)?;control.step()?;
  }
  let count=native::length(&mut reader)?;census.future(count)?;control.begin_stage(count)?;
  for _ in 0..count{
   let mut bytes=add(40,text(&mut reader,control)?)?;text(&mut reader,control)?;bytes=add(bytes,text(&mut reader,control)?)?;text(&mut reader,control)?;bytes=add(bytes,text(&mut reader,control)?)?;bytes=add(bytes,text(&mut reader,control)?)?;census.row(bytes)?;control.step()?;
  }
  if reader.remaining()!=0{return Err(invalid())}control.checkpoint()
 })
}
/// 📝️ Counts actual hexadecimal text and exact IEEE spellings with unchanged field defaults.
pub(super)fn document(body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<()>{
 layout(limits)?;control.scoped_stage(|control|{
  let fields=native::fields(body,["schema","nodes","edges"],control)?;let mut census=Census{limits,rows:0,bytes:0};census.row(add(8,native::hex_text_extent(fields[0].ok_or_else(invalid)?,control)?)?)?;
  let mut nodes=native::Items::new(fields[1].unwrap_or("[]"))?;let count=nodes.count(control,limits.max_rows)?;census.future(count)?;control.begin_stage(count)?;
  while let Some(value)=nodes.next(control)?{
   let[id,kind,label,params,position]=native::record(value,control)?;let[x,y]=native::record(position,control)?;
   let mut bytes=add(add(add(24,native::hex_text_extent(id,control)?)?,native::hex_text_extent(kind,control)?)?,native::hex_text_extent(label,control)?)?;bytes=add(bytes,coordinate(native::float(x,control)?))?;bytes=add(bytes,coordinate(native::float(y,control)?))?;census.row(bytes)?;
   let mut params=native::Items::new(params)?;let count=params.count(control,limits.max_rows)?;census.future(count)?;
   control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(value)=params.next(control)?{let[key,value]=native::record(value,control)?;census.row(add(add(24,native::hex_text_extent(key,control)?)?,native::hex_text_extent(value,control)?)?)?;control.step()?;}Ok::<_,ValueError>(())})?;control.step()?;
  }
  let mut edges=native::Items::new(fields[2].unwrap_or("[]"))?;let count=edges.count(control,limits.max_rows)?;census.future(count)?;control.begin_stage(count)?;
  while let Some(value)=edges.next(control)?{
   let[id,from,to,kind]=native::record(value,control)?;let[from_node,from_port]=native::record(from,control)?;let[to_node,to_port]=native::record(to,control)?;native::hex_text_extent(from_node,control)?;native::hex_text_extent(to_node,control)?;
   census.row(add(add(add(add(40,native::hex_text_extent(id,control)?)?,native::hex_text_extent(from_port,control)?)?,native::hex_text_extent(to_port,control)?)?,native::hex_text_extent(kind,control)?)?)?;control.step()?;
  }control.checkpoint()
 })
}
