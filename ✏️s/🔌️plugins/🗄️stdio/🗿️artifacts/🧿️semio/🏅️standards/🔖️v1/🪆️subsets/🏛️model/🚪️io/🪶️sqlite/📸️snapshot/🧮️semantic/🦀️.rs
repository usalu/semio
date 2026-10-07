//! 🏛️ Borrowed Model primitives count spatial ownership, exact placements and typed property rows.
use super::{SemioModelSnapshot,ValueError,ValueRefusalKind};
use store::{ArtifactSqliteSnapshot,sqlite_snapshot::SqliteDatabaseLimits};
use semio_framework_value::native_decoding::NativeDecodeControl;
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding as native;
type Result<T>=std::result::Result<T,ValueError>;
fn invalid()->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,"Semio Model native field differs from its authored shape")}
fn add(a:usize,b:usize)->Result<usize>{a.checked_add(b).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio Model complete cell extent overflow"))}
/// 🏛️ Admits all declared entity, exact placement, property and relation tables before ownership.
pub(super)fn layout(limits:SqliteDatabaseLimits)->Result<()>{
 let mut bytes=0;for statement in SemioModelSnapshot::SQLITE_SCHEMA.split(';'){bytes=add(bytes,statement.trim().len())?;}for name in["semio_model_document","semio_model_entity","semio_model_spatial","semio_model_element","semio_model_placement","semio_model_geometry_reference","semio_model_property_set","semio_model_property","semio_model_relation"]{bytes=add(bytes,name.len())?;}
 if bytes.max(SemioModelSnapshot::SQLITE_SCHEMA.len())>limits.max_schema_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio Model schema exceeds caller bytes"))}
 if limits.max_tables<9||limits.max_columns<31||limits.max_rows<1{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio Model authored layout exceeds copied limits"))}Ok(())
}
struct Census{limits:SqliteDatabaseLimits,rows:usize,bytes:usize}
impl Census{
 fn row(&mut self,bytes:usize)->Result<()>{self.rows=add(self.rows,1)?;self.bytes=add(self.bytes,bytes)?;if self.rows>self.limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio Model complete rows exceed caller limit"))}if self.bytes>self.limits.max_value_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio Model complete cells exceed caller bytes"))}Ok(())}
 fn future(&self,count:usize)->Result<()>{if add(self.rows,count)?>self.limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio Model declared rows exceed caller limit"))}Ok(())}
}
fn text(reader:&mut store::ByteReader<'_>,control:&mut NativeDecodeControl<'_>)->Result<usize>{Ok(control.borrow_text(native::bytes(reader)?)?.len())}
fn byte(reader:&mut store::ByteReader<'_>)->Result<u8>{reader.read_u8().map_err(|_|invalid())}
fn scalar(value:f64)->usize{if value.is_nan(){11}else if value.is_infinite(){32}else{22}}
fn binary_list(reader:&mut store::ByteReader<'_>,census:&mut Census,control:&mut NativeDecodeControl<'_>,mut visit:impl FnMut(&mut store::ByteReader<'_>,&mut Census,&mut NativeDecodeControl<'_>)->Result<()>)->Result<()>{
 let count=native::length(reader)?;census.future(count)?;control.scoped_stage(|control|{control.begin_stage(count)?;for _ in 0..count{visit(reader,census,control)?;control.step()?;}control.checkpoint()})
}
fn binary_optional(reader:&mut store::ByteReader<'_>,control:&mut NativeDecodeControl<'_>)->Result<usize>{match byte(reader)?{0=>Ok(0),1=>{text(reader,control)?;Ok(8)},_=>Err(invalid())}}
fn spatial(tag:u8)->Result<usize>{match tag{0=>Ok(4),1=>Ok(8),2=>Ok(6),3=>Ok(5),_=>Err(invalid())}}
fn class(tag:u8)->Result<usize>{match tag{0|1|3|4|6=>Ok(4),2|5=>Ok(6),7|9=>Ok(5),8=>Ok(9),_=>Err(invalid())}}
fn relation(tag:u8)->Result<usize>{match tag{0|3=>Ok(10),1=>Ok(12),2=>Ok(11),4=>Ok(13),5=>Ok(5),_=>Err(invalid())}}
fn binary_placement(reader:&mut store::ByteReader<'_>,census:&mut Census)->Result<()>{
 let mut bytes=8;for _ in 0..10{bytes=add(bytes,scalar(reader.read_f64_le().map_err(|_|invalid())?))?;}census.row(bytes)
}
/// 📦️ Counts the real Model primitive order without constructing a spatial graph or property set.
pub(super)fn binary(body:&[u8],control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<()>{
 layout(limits)?;control.scoped_stage(|control|{
  let mut reader=store::ByteReader::new(body);if byte(&mut reader)?!=1{return Err(invalid())}let mut census=Census{limits,rows:0,bytes:0};census.row(add(8,text(&mut reader,control)?)?)?;
  binary_list(&mut reader,&mut census,control,|reader,census,control|{
   census.row(add(31,text(reader,control)?)?)?;let mut bytes=add(add(8,spatial(byte(reader)?)?)?,text(reader,control)?)?;bytes=add(bytes,binary_optional(reader,control)?)?;census.row(bytes)?;binary_placement(reader,census)
  })?;
  binary_list(&mut reader,&mut census,control,|reader,census,control|{
   census.row(add(31,text(reader,control)?)?)?;let tag=byte(reader)?;let mut bytes=add(8,class(tag)?)?;if tag==9{bytes=add(bytes,text(reader,control)?)?;}binary_placement(reader,census)?;
   let geometry=match byte(reader)?{0=>12,1|2=>add(12,text(reader,control)?)?,_=>return Err(invalid())};census.row(geometry)?;bytes=add(bytes,binary_optional(reader,control)?)?;census.row(bytes)?;
   binary_list(reader,census,control,|reader,census,control|{
    census.row(add(24,text(reader,control)?)?)?;
    binary_list(reader,census,control,|reader,census,control|{
     let mut bytes=add(24,text(reader,control)?)?;bytes=add(bytes,match byte(reader)?{0=>add(4,text(reader,control)?)?,1=>add(6,scalar(reader.read_f64_le().map_err(|_|invalid())?))?,2=>{if byte(reader)?>1{return Err(invalid())}15},_=>return Err(invalid())})?;census.row(bytes)
    })
   })
  })?;
  binary_list(&mut reader,&mut census,control,|reader,census,control|{
   let mut bytes=add(40,text(reader,control)?)?;let tag=byte(reader)?;bytes=add(bytes,relation(tag)?)?;if tag==5{bytes=add(bytes,text(reader,control)?)?;}text(reader,control)?;text(reader,control)?;census.row(bytes)
  })?;
  if reader.remaining()!=0{return Err(invalid())}control.checkpoint()
 })
}
fn document_list(value:&str,census:&mut Census,control:&mut NativeDecodeControl<'_>,mut visit:impl FnMut(&str,&mut Census,&mut NativeDecodeControl<'_>)->Result<()>)->Result<()>{
 let mut items=native::Items::new(value)?;let count=items.count(control,census.limits.max_rows)?;census.future(count)?;control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(value)=items.next(control)?{visit(value,census,control)?;control.step()?;}control.checkpoint()})
}
fn document_optional(value:&str,control:&mut NativeDecodeControl<'_>)->Result<usize>{if value=="[0]"{return Ok(0)}let[tag,value]=native::record(value,control)?;if tag!="1"{return Err(invalid())}native::hex_text_extent(value,control)?;Ok(8)}
fn document_placement(value:&str,census:&mut Census,control:&mut NativeDecodeControl<'_>)->Result<()>{
 let[translation,rotation,scale]=native::record(value,control)?;let[tx,ty,tz]=native::record(translation,control)?;let[rx,ry,rz,rw]=native::record(rotation,control)?;let[sx,sy,sz]=native::record(scale,control)?;
 let mut bytes=8;for value in[tx,ty,tz,rx,ry,rz,rw,sx,sy,sz]{bytes=add(bytes,scalar(native::float(value,control)?))?;}census.row(bytes)
}
fn document_class(value:&str,control:&mut NativeDecodeControl<'_>)->Result<usize>{
 let tag=match value{"WA"=>0,"SL"=>1,"CO"=>2,"BE"=>3,"DO"=>4,"WI"=>5,"RO"=>6,"ST"=>7,"FU"=>8,_=>9};let mut bytes=add(8,class(tag)?)?;if tag==9{let[name]=native::record(value.strip_prefix("OT").ok_or_else(invalid)?,control)?;bytes=add(bytes,native::hex_text_extent(name,control)?)?;}Ok(bytes)
}
fn document_geometry(value:&str,control:&mut NativeDecodeControl<'_>)->Result<usize>{
 if value=="N"{return Ok(12)}let(tag,value)=value.split_at_checked(1).ok_or_else(invalid)?;if !matches!(tag,"B"|"M"){return Err(invalid())}let[id]=native::record(value,control)?;add(12,native::hex_text_extent(id,control)?)
}
fn document_value(value:&str,control:&mut NativeDecodeControl<'_>)->Result<usize>{
 let(tag,value)=value.split_at_checked(1).ok_or_else(invalid)?;let[value]=native::record(value,control)?;match tag{"T"=>add(4,native::hex_text_extent(value,control)?),"N"=>add(6,scalar(native::float(value,control)?)),"B"=>{if !matches!(value,"0"|"1"){return Err(invalid())}Ok(15)},_=>Err(invalid())}
}
fn document_relation(value:&str,control:&mut NativeDecodeControl<'_>)->Result<usize>{
 let tag=match value{"AG"=>0,"CI"=>1,"CN"=>2,"FV"=>3,"VE"=>4,_=>5};let mut bytes=relation(tag)?;if tag==5{let[label]=native::record(value.strip_prefix("OT").ok_or_else(invalid)?,control)?;bytes=add(bytes,native::hex_text_extent(label,control)?)?;}Ok(bytes)
}
/// 📝️ Preserves exact optional placement parents, enum tags and all three typed property spellings.
pub(super)fn document(body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<()>{
 layout(limits)?;control.scoped_stage(|control|{
  let fields=native::fields(body,["schema","spatial","elements","relations"],control)?;let mut census=Census{limits,rows:0,bytes:0};census.row(add(8,native::hex_text_extent(fields[0].ok_or_else(invalid)?,control)?)?)?;
  document_list(fields[1].unwrap_or("[]"),&mut census,control,|value,census,control|{
   let[id,kind,name,parent,placement]=native::record(value,control)?;census.row(add(31,native::hex_text_extent(id,control)?)?)?;let tag=match kind{"S"=>0,"B"=>1,"T"=>2,"P"=>3,_=>return Err(invalid())};
   census.row(add(add(add(8,spatial(tag)?)?,native::hex_text_extent(name,control)?)?,document_optional(parent,control)?)?)?;document_placement(placement,census,control)
  })?;
  document_list(fields[2].unwrap_or("[]"),&mut census,control,|value,census,control|{
   let[id,class,placement,geometry,parent,psets]=native::record(value,control)?;census.row(add(31,native::hex_text_extent(id,control)?)?)?;census.row(add(document_class(class,control)?,document_optional(parent,control)?)?)?;document_placement(placement,census,control)?;census.row(document_geometry(geometry,control)?)?;
   document_list(psets,census,control,|value,census,control|{let[name,properties]=native::record(value,control)?;census.row(add(24,native::hex_text_extent(name,control)?)?)?;document_list(properties,census,control,|value,census,control|{let[key,value]=native::record(value,control)?;census.row(add(add(24,native::hex_text_extent(key,control)?)?,document_value(value,control)?)?)?;Ok(())})})
  })?;
  document_list(fields[3].unwrap_or("[]"),&mut census,control,|value,census,control|{let[id,kind,source,target]=native::record(value,control)?;native::hex_text_extent(source,control)?;native::hex_text_extent(target,control)?;census.row(add(add(40,native::hex_text_extent(id,control)?)?,document_relation(kind,control)?)?)?;Ok(())})?;
  control.checkpoint()
 })
}
