//! 🧰️ Borrowed Kit primitives count relational type identities, exact placements and history pin cells.
use super::{SemioKitSnapshot,ValueError,ValueRefusalKind};
use store::{ArtifactSqliteSnapshot,sqlite_snapshot::SqliteDatabaseLimits};
use semio_framework_value::native_decoding::NativeDecodeControl;
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding as native;
type Result<T>=std::result::Result<T,ValueError>;
fn invalid()->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,"Semio Kit native field differs from its authored shape")}
fn add(a:usize,b:usize)->Result<usize>{a.checked_add(b).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio Kit complete cell extent overflow"))}
/// 🏛️ Admits every declared child, exact placement and history pin table before ownership.
pub(super)fn layout(limits:SqliteDatabaseLimits)->Result<()>{
 let mut bytes=0;for statement in SemioKitSnapshot::SQLITE_SCHEMA.split(';'){bytes=add(bytes,statement.trim().len())?;}for name in["semio_kit_document","semio_kit_type","semio_kit_design","semio_kit_piece","semio_kit_connection","semio_kit_reference","semio_kit_object_child","semio_kit_model_child","semio_kit_value_child","semio_kit_representation","semio_kit_blob"]{bytes=add(bytes,name.len())?;}
 if bytes.max(SemioKitSnapshot::SQLITE_SCHEMA.len())>limits.max_schema_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio Kit schema exceeds caller bytes"))}
 if limits.max_tables<11||limits.max_columns<35||limits.max_rows<1{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio Kit authored layout exceeds copied limits"))}Ok(())
}
struct Census{limits:SqliteDatabaseLimits,rows:usize,bytes:usize}
impl Census{
 fn row(&mut self,bytes:usize)->Result<()>{self.rows=add(self.rows,1)?;self.bytes=add(self.bytes,bytes)?;if self.rows>self.limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio Kit complete rows exceed caller limit"))}if self.bytes>self.limits.max_value_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio Kit complete cells exceed caller bytes"))}Ok(())}
 fn future(&self,count:usize)->Result<()>{if add(self.rows,count)?>self.limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio Kit declared rows exceed caller limit"))}Ok(())}
}
fn text(reader:&mut store::ByteReader<'_>,control:&mut NativeDecodeControl<'_>)->Result<usize>{Ok(control.borrow_text(native::bytes(reader)?)?.len())}
fn byte(reader:&mut store::ByteReader<'_>)->Result<u8>{reader.read_u8().map_err(|_|invalid())}
fn scalar(value:f64)->usize{if value.is_nan(){11}else if value.is_infinite(){32}else{22}}
fn digits(mut value:u64)->usize{let mut count=1;while value>=10{value/=10;count+=1;}count}
fn binary_list(reader:&mut store::ByteReader<'_>,census:&mut Census,control:&mut NativeDecodeControl<'_>,mut visit:impl FnMut(&mut store::ByteReader<'_>,&mut Census,&mut NativeDecodeControl<'_>)->Result<()>)->Result<()>{
 let count=native::length(reader)?;census.future(count)?;control.scoped_stage(|control|{control.begin_stage(count)?;for _ in 0..count{visit(reader,census,control)?;control.step()?;}control.checkpoint()})
}
fn binary_reference(reader:&mut store::ByteReader<'_>,census:&mut Census,control:&mut NativeDecodeControl<'_>)->Result<()>{
 let mut bytes=8;for _ in 0..4{bytes=add(bytes,text(reader,control)?)?;}census.row(bytes)
}
fn binary_child(reader:&mut store::ByteReader<'_>,census:&mut Census,control:&mut NativeDecodeControl<'_>,ordered:bool)->Result<()>{
 let id=text(reader,control)?;binary_reference(reader,census,control)?;census.row(add(if ordered{32}else{24},id)?)
}
/// 📦️ Counts the actual catalog, design, child and pin field order without materializing a Kit.
pub(super)fn binary(body:&[u8],control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<()>{
 layout(limits)?;control.scoped_stage(|control|{
  let mut reader=store::ByteReader::new(body);if byte(&mut reader)?!=1{return Err(invalid())}let mut census=Census{limits,rows:0,bytes:0};census.row(add(8,text(&mut reader,control)?)?)?;
  binary_list(&mut reader,&mut census,control,|reader,census,control|{let mut bytes=24;for _ in 0..3{bytes=add(bytes,text(reader,control)?)?;}census.row(bytes)})?;
  binary_list(&mut reader,&mut census,control,|reader,census,control|{
   census.row(add(add(24,text(reader,control)?)?,text(reader,control)?)?)?;
   binary_list(reader,census,control,|reader,census,control|{
    let mut bytes=add(32,text(reader,control)?)?;text(reader,control)?;for _ in 0..10{bytes=add(bytes,scalar(reader.read_f64_le().map_err(|_|invalid())?))?;}census.row(bytes)
   })?;
   binary_list(reader,census,control,|reader,census,control|{
    let mut bytes=add(40,text(reader,control)?)?;text(reader,control)?;bytes=add(bytes,text(reader,control)?)?;text(reader,control)?;bytes=add(bytes,text(reader,control)?)?;census.row(bytes)
   })
  })?;
  binary_list(&mut reader,&mut census,control,|reader,census,control|binary_child(reader,census,control,true))?;
  binary_list(&mut reader,&mut census,control,|reader,census,control|binary_child(reader,census,control,true))?;
  match byte(&mut reader)?{0=>{},1=>binary_child(&mut reader,&mut census,control,false)?,_=>return Err(invalid())}
  binary_list(&mut reader,&mut census,control,|reader,census,control|{
   binary_reference(reader,census,control)?;let bytes=match byte(reader)?{
    0=>44,1=>add(50,text(reader,control)?)?,2=>{let hash=text(reader,control)?;let size=reader.read_varint_u64().map_err(|_|invalid())?;let media=text(reader,control)?;census.row(add(add(add(8,hash)?,digits(size))?,media)?)?;56},_=>return Err(invalid())
   };text(reader,control)?;census.row(bytes)
  })?;
  if reader.remaining()!=0{return Err(invalid())}control.checkpoint()
 })
}
fn document_list(value:&str,census:&mut Census,control:&mut NativeDecodeControl<'_>,mut visit:impl FnMut(&str,&mut Census,&mut NativeDecodeControl<'_>)->Result<()>)->Result<()>{
 let mut items=native::Items::new(value)?;let count=items.count(control,census.limits.max_rows)?;census.future(count)?;control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(value)=items.next(control)?{visit(value,census,control)?;control.step()?;}control.checkpoint()})
}
fn document_reference(value:&str,census:&mut Census,control:&mut NativeDecodeControl<'_>)->Result<()>{
 let[id,kind,standard,subset]=native::record(value,control)?;let mut bytes=8;for value in[id,kind,standard,subset]{bytes=add(bytes,native::hex_text_extent(value,control)?)?;}census.row(bytes)
}
fn document_child(value:&str,census:&mut Census,control:&mut NativeDecodeControl<'_>,ordered:bool)->Result<()>{
 let[id,target]=native::record(value,control)?;let id=native::hex_text_extent(id,control)?;document_reference(target,census,control)?;census.row(add(if ordered{32}else{24},id)?)
}
fn document_pin(value:&str,census:&mut Census,control:&mut NativeDecodeControl<'_>)->Result<usize>{
 if value=="[h]"{return Ok(44)}let inner=value.strip_prefix('[').and_then(|value|value.strip_suffix(']')).ok_or_else(invalid)?;
 match inner.split_once(',').map(|(tag,_)|tag){
  Some("c")=>{let[_,id]=native::record(value,control)?;add(50,native::hex_text_extent(id,control)?)},
  Some("s")=>{let[_,hash,size,media]=native::record(value,control)?;let size=control.scoped_stage(|control|{control.begin_stage(size.len())?;for _ in size.bytes(){control.step()?;}let value=size.parse::<u64>().map_err(|_|invalid())?;control.checkpoint()?;Ok::<usize,ValueError>(digits(value))})?;census.row(add(add(add(8,native::hex_text_extent(hash,control)?)?,size)?,native::hex_text_extent(media,control)?)?)?;Ok(56)},
  _=>Err(invalid())
 }
}
/// 📝️ Preserves original field defaults, optional value child and exact pin grammar.
pub(super)fn document(body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<()>{
 layout(limits)?;control.scoped_stage(|control|{
  let fields=native::fields(body,["schema","types","designs","objects","models","properties","representations"],control)?;let mut census=Census{limits,rows:0,bytes:0};census.row(add(8,native::hex_text_extent(fields[0].ok_or_else(invalid)?,control)?)?)?;
  document_list(fields[1].unwrap_or("[]"),&mut census,control,|value,census,control|{let[id,name,category]=native::record(value,control)?;let mut bytes=24;for value in[id,name,category]{bytes=add(bytes,native::hex_text_extent(value,control)?)?;}census.row(bytes)})?;
  document_list(fields[2].unwrap_or("[]"),&mut census,control,|value,census,control|{
   let[id,name,pieces,connections]=native::record(value,control)?;census.row(add(add(24,native::hex_text_extent(id,control)?)?,native::hex_text_extent(name,control)?)?)?;
   document_list(pieces,census,control,|value,census,control|{
    let[id,kind,transform]=native::record(value,control)?;let mut bytes=add(32,native::hex_text_extent(id,control)?)?;native::hex_text_extent(kind,control)?;let[tx,ty,tz,rx,ry,rz,rw,sx,sy,sz]=native::record(transform,control)?;for value in[tx,ty,tz,rx,ry,rz,rw,sx,sy,sz]{bytes=add(bytes,scalar(native::float(value,control)?))?;}census.row(bytes)
   })?;
   document_list(connections,census,control,|value,census,control|{let[id,a,ap,b,bp]=native::record(value,control)?;native::hex_text_extent(a,control)?;native::hex_text_extent(b,control)?;census.row(add(add(add(40,native::hex_text_extent(id,control)?)?,native::hex_text_extent(ap,control)?)?,native::hex_text_extent(bp,control)?)?)?;Ok(())})
  })?;
  document_list(fields[3].unwrap_or("[]"),&mut census,control,|value,census,control|document_child(value,census,control,true))?;
  document_list(fields[4].unwrap_or("[]"),&mut census,control,|value,census,control|document_child(value,census,control,true))?;
  if let Some(value)=fields[5]{if value!="[]"{document_child(value,&mut census,control,false)?;}}
  document_list(fields[6].unwrap_or("[]"),&mut census,control,|value,census,control|{let[target,pin,role]=native::record(value,control)?;document_reference(target,census,control)?;let bytes=document_pin(pin,census,control)?;native::hex_text_extent(role,control)?;census.row(bytes)})?;
  control.checkpoint()
 })
}
