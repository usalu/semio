//! 🧊️ Borrowed BRep primitives count literal topology, analytic geometry and every spline array.
use super::{SemioBrepSnapshot,ValueError,ValueRefusalKind};
use store::{ArtifactSqliteSnapshot,sqlite_snapshot::SqliteDatabaseLimits};
use semio_framework_value::native_decoding::NativeDecodeControl;
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding as native;
type Result<T>=std::result::Result<T,ValueError>;
fn invalid()->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,"Semio Brep native field differs from its authored shape")}
fn add(a:usize,b:usize)->Result<usize>{a.checked_add(b).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio Brep complete cell extent overflow"))}
/// 🏛️ Admits the full independently authored Brep layout before native ownership.
pub(super)fn layout(limits:SqliteDatabaseLimits)->Result<()>{
 let mut bytes=0;for statement in SemioBrepSnapshot::SQLITE_SCHEMA.split(';'){bytes=add(bytes,statement.trim().len())?;}
 for name in["semio_brep_document","semio_brep_vertex","semio_brep_edge","semio_brep_loop","semio_brep_loop_edge","semio_brep_face","semio_brep_inner_loop","semio_brep_shell","semio_brep_shell_face","semio_brep_solid","semio_brep_solid_shell","semio_brep_coedge","semio_brep_curve3","semio_brep_curve3_point","semio_brep_curve3_weight","semio_brep_curve3_knot","semio_brep_curve2","semio_brep_curve2_point","semio_brep_curve2_weight","semio_brep_curve2_knot","semio_brep_surface","semio_brep_surface_point","semio_brep_surface_weight","semio_brep_surface_knot_u","semio_brep_surface_knot_v"]{bytes=add(bytes,name.len())?;}
 if bytes.max(SemioBrepSnapshot::SQLITE_SCHEMA.len())>limits.max_schema_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio Brep schema exceeds caller bytes"))}
 if limits.max_tables<25||limits.max_columns<54||limits.max_rows<1{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio Brep authored layout exceeds copied limits"))}Ok(())
}
struct Census{limits:SqliteDatabaseLimits,rows:usize,bytes:usize}
impl Census{
 fn row(&mut self,bytes:usize)->Result<()>{let rows=add(self.rows,1)?;let bytes=add(self.bytes,bytes)?;if rows>self.limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio Brep complete row count exceeds copied limit"))}if bytes>self.limits.max_value_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio Brep complete cell bytes exceed copied limit"))}self.rows=rows;self.bytes=bytes;Ok(())}
 fn extra(&mut self,bytes:usize)->Result<()>{let bytes=add(self.bytes,bytes)?;if bytes>self.limits.max_value_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio Brep complete label cell exceeds copied limit"))}self.bytes=bytes;Ok(())}
 fn future(&self,count:usize)->Result<()>{if add(self.rows,count)?>self.limits.max_rows{Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio Brep native list exceeds copied row limit"))}else{Ok(())}}
}
fn byte(reader:&mut store::ByteReader<'_>)->Result<u8>{reader.read_u8().map_err(|_|invalid())}
fn text(reader:&mut store::ByteReader<'_>,control:&mut NativeDecodeControl<'_>)->Result<usize>{Ok(control.borrow_text(native::bytes(reader)?)?.len())}
fn scalar(value:f64)->usize{if value.is_nan(){11}else if value.is_infinite(){32}else{22}}
fn binary_f64(reader:&mut store::ByteReader<'_>)->Result<usize>{Ok(scalar(reader.read_f64_le().map_err(|_|invalid())?))}
fn binary_words(reader:&mut store::ByteReader<'_>,count:usize)->Result<usize>{let mut bytes=0;for _ in 0..count{bytes=add(bytes,binary_f64(reader)?)?;}Ok(bytes)}
fn boolean(reader:&mut store::ByteReader<'_>)->Result<bool>{match byte(reader)?{0=>Ok(false),1=>Ok(true),_=>Err(invalid())}}
fn uint32(reader:&mut store::ByteReader<'_>)->Result<()>{u32::try_from(reader.read_varint_u64().map_err(|_|invalid())?).map(|_|()).map_err(|_|invalid())}
fn digits(mut value:u64)->usize{let mut count=1;while value>=10{value/=10;count+=1;}count}
fn binary_list(reader:&mut store::ByteReader<'_>,census:&mut Census,control:&mut NativeDecodeControl<'_>,mut visit:impl FnMut(&mut store::ByteReader<'_>,&mut Census,&mut NativeDecodeControl<'_>)->Result<()>)->Result<()>{
 let count=native::length(reader)?;census.future(count)?;control.scoped_stage(|control|{control.begin_stage(count)?;for _ in 0..count{visit(reader,census,control)?;control.step()?;}control.checkpoint()})
}
fn binary_numbers(reader:&mut store::ByteReader<'_>,census:&mut Census,control:&mut NativeDecodeControl<'_>)->Result<()>{binary_list(reader,census,control,|reader,census,_|census.row(add(24,binary_f64(reader)?)?))}
fn binary_points(reader:&mut store::ByteReader<'_>,census:&mut Census,control:&mut NativeDecodeControl<'_>,width:usize)->Result<()>{binary_list(reader,census,control,|reader,census,_|census.row(add(24,binary_words(reader,width)?)?))}
fn binary_curve(reader:&mut store::ByteReader<'_>,census:&mut Census,control:&mut NativeDecodeControl<'_>,width:usize)->Result<()>{
 let tag=byte(reader)?;let(kind,words)=match(tag,width){(0,3)=>(4,6),(0,2)=>(4,4),(1,3)=>(6,7),(1,2)=>(6,3),(2,3)=>(7,8),(2,2)=>(7,6),(3,_)=>{census.row(21)?;binary_points(reader,census,control,width)?;binary_numbers(reader,census,control)?;uint32(reader)?;binary_numbers(reader,census,control)?;return Ok(())},_=>return Err(invalid())};census.row(add(add(8,kind)?,binary_words(reader,words)?)?)
}
fn binary_surface(reader:&mut store::ByteReader<'_>,census:&mut Census,control:&mut NativeDecodeControl<'_>)->Result<()>{
 let(kind,words)=match byte(reader)?{0=>(5,6),1=>(8,7),2=>(4,8),3=>(6,4),4=>(5,8),5=>{census.row(45)?;binary_points(reader,census,control,3)?;binary_numbers(reader,census,control)?;for _ in 0..4{uint32(reader)?;}binary_numbers(reader,census,control)?;binary_numbers(reader,census,control)?;return Ok(())},_=>return Err(invalid())};census.row(add(add(8,kind)?,binary_words(reader,words)?)?)
}
/// 📦️ Counts original BRep binary topology and geometry before typed construction.
pub(super)fn binary(body:&[u8],control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<()>{
 layout(limits)?;control.scoped_stage(|control|{
  let mut reader=store::ByteReader::new(body);if byte(&mut reader)?!=1{return Err(invalid())}let mut census=Census{limits,rows:0,bytes:0};census.row(add(8,text(&mut reader,control)?)?)?;
  binary_list(&mut reader,&mut census,control,|reader,census,control|census.row(add(add(24,text(reader,control)?)?,binary_words(reader,4)?)?))?;
  binary_list(&mut reader,&mut census,control,|reader,census,control|{let mut bytes=32;for _ in 0..3{bytes=add(bytes,text(reader,control)?)?;}binary_curve(reader,census,control,3)?;census.row(add(bytes,binary_f64(reader)?)?)})?;
  binary_list(&mut reader,&mut census,control,|reader,census,control|{census.row(add(24,text(reader,control)?)?)?;binary_list(reader,census,control,|reader,census,control|{let bytes=add(32,text(reader,control)?)?;boolean(reader)?;census.row(bytes)})})?;
  binary_list(&mut reader,&mut census,control,|reader,census,control|{let bytes=add(add(40,text(reader,control)?)?,text(reader,control)?)?;binary_list(reader,census,control,|reader,census,control|census.row(add(24,text(reader,control)?)?))?;binary_surface(reader,census,control)?;boolean(reader)?;census.row(add(bytes,binary_f64(reader)?)?)})?;
  for _ in 0..2{binary_list(&mut reader,&mut census,control,|reader,census,control|{census.row(add(24,text(reader,control)?)?)?;binary_list(reader,census,control,|reader,census,control|{let bytes=add(32,text(reader,control)?)?;boolean(reader)?;census.row(bytes)})})?;}
  binary_list(&mut reader,&mut census,control,|reader,census,control|{let mut bytes=add(add(32,text(reader,control)?)?,text(reader,control)?)?;boolean(reader)?;if boolean(reader)?{binary_curve(reader,census,control,2)?;bytes=add(bytes,8)?;}bytes=add(bytes,binary_words(reader,2)?)?;for _ in 0..3{bytes=add(bytes,text(reader,control)?)?;}census.row(bytes)})?;
  census.extra(digits(reader.read_varint_u64().map_err(|_|invalid())?))?;if reader.remaining()!=0{return Err(invalid())}control.checkpoint()
 })
}
fn document_f64(value:&str,control:&mut NativeDecodeControl<'_>)->Result<usize>{Ok(scalar(native::float(value,control)?))}
fn document_u32(value:&str,control:&mut NativeDecodeControl<'_>)->Result<()>{control.scoped_stage(|control|{control.begin_stage(value.len())?;for _ in value.bytes(){control.step()?;}value.parse::<u32>().map(|_|()).map_err(|_|invalid())})}
fn document_label(value:&str,control:&mut NativeDecodeControl<'_>)->Result<usize>{control.scoped_stage(|control|{control.begin_stage(value.len())?;for _ in value.bytes(){control.step()?;}value.parse::<u64>().map(digits).map_err(|_|invalid())})}
fn document_boolean(value:&str)->Result<()>{match value{"0"|"1"=>Ok(()),_=>Err(invalid())}}
fn document_point(value:&str,control:&mut NativeDecodeControl<'_>,width:usize)->Result<usize>{let mut bytes=0;if width==3{let[x,y,z]=native::record(value,control)?;for value in[x,y,z]{bytes=add(bytes,document_f64(value,control)?)?;}}else{let[x,y]=native::record(value,control)?;for value in[x,y]{bytes=add(bytes,document_f64(value,control)?)?;}}Ok(bytes)}
fn document_list(value:&str,census:&mut Census,control:&mut NativeDecodeControl<'_>,mut visit:impl FnMut(&str,&mut Census,&mut NativeDecodeControl<'_>)->Result<()>)->Result<()>{
 let mut items=native::Items::new(value)?;let count=items.count(control,census.limits.max_rows)?;census.future(count)?;control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(value)=items.next(control)?{visit(value,census,control)?;control.step()?;}control.checkpoint()})
}
fn document_numbers(value:&str,census:&mut Census,control:&mut NativeDecodeControl<'_>)->Result<()>{document_list(value,census,control,|value,census,control|census.row(add(24,document_f64(value,control)?)?))}
fn document_points(value:&str,census:&mut Census,control:&mut NativeDecodeControl<'_>,width:usize)->Result<()>{document_list(value,census,control,|value,census,control|census.row(add(24,document_point(value,control,width)?)?))}
fn document_curve(value:&str,census:&mut Census,control:&mut NativeDecodeControl<'_>,width:usize)->Result<()>{
 let(tag,value)=value.split_at_checked(1).ok_or_else(invalid)?;let(kind,bytes)=match tag{
  "L"=>{let[origin,direction]=native::record(value,control)?;(4,add(document_point(origin,control,width)?,document_point(direction,control,width)?)?)},
  "C"=>{if width==3{let[center,axis,radius]=native::record(value,control)?;(6,add(add(document_point(center,control,3)?,document_point(axis,control,3)?)?,document_f64(radius,control)?)?)}else{let[center,radius]=native::record(value,control)?;(6,add(document_point(center,control,2)?,document_f64(radius,control)?)?)}},
  "E"=>{let[center,axis,radius_major,radius_minor]=native::record(value,control)?;(7,add(add(add(document_point(center,control,width)?,document_point(axis,control,width)?)?,document_f64(radius_major,control)?)?,document_f64(radius_minor,control)?)?)},
  "N"=>{let[points,weights,degree,knots]=native::record(value,control)?;census.row(21)?;document_points(points,census,control,width)?;document_numbers(weights,census,control)?;document_u32(degree,control)?;document_numbers(knots,census,control)?;return Ok(())},_=>return Err(invalid())
 };census.row(add(add(8,kind)?,bytes)?)
}
fn document_surface(value:&str,census:&mut Census,control:&mut NativeDecodeControl<'_>)->Result<()>{
 let(tag,value)=value.split_at_checked(1).ok_or_else(invalid)?;let(kind,bytes)=match tag{
  "P"=>{let[origin,normal]=native::record(value,control)?;(5,add(document_point(origin,control,3)?,document_point(normal,control,3)?)?)},
  "C"=>{let[origin,axis,radius]=native::record(value,control)?;(8,add(add(document_point(origin,control,3)?,document_point(axis,control,3)?)?,document_f64(radius,control)?)?)},
  "O"=>{let[origin,axis,radius,half_angle]=native::record(value,control)?;(4,add(add(add(document_point(origin,control,3)?,document_point(axis,control,3)?)?,document_f64(radius,control)?)?,document_f64(half_angle,control)?)?)},
  "S"=>{let[center,radius]=native::record(value,control)?;(6,add(document_point(center,control,3)?,document_f64(radius,control)?)?)},
  "T"=>{let[center,axis,major_radius,minor_radius]=native::record(value,control)?;(5,add(add(add(document_point(center,control,3)?,document_point(axis,control,3)?)?,document_f64(major_radius,control)?)?,document_f64(minor_radius,control)?)?)},
  "N"=>{let[points,weights,u_count,v_count,degree_u,degree_v,knots_u,knots_v]=native::record(value,control)?;census.row(45)?;document_points(points,census,control,3)?;document_numbers(weights,census,control)?;for value in[u_count,v_count,degree_u,degree_v]{document_u32(value,control)?;}document_numbers(knots_u,census,control)?;document_numbers(knots_v,census,control)?;return Ok(())},_=>return Err(invalid())
 };census.row(add(add(8,kind)?,bytes)?)
}
/// 📝️ Counts BRep literal topology references, exact IEEE words and original spline array records.
pub(super)fn document(body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<()>{
 layout(limits)?;control.scoped_stage(|control|{
  let fields=native::fields(body,["schema","vertices","edges","loops","faces","shells","solids","coedges","nextLabel"],control)?;let mut census=Census{limits,rows:0,bytes:0};census.row(add(add(8,native::hex_text_extent(fields[0].ok_or_else(invalid)?,control)?)?,document_label(fields[8].ok_or_else(invalid)?,control)?)?)?;
  document_list(fields[1].unwrap_or("[]"),&mut census,control,|value,census,control|{let[id,point,tol]=native::record(value,control)?;census.row(add(add(add(24,native::hex_text_extent(id,control)?)?,document_point(point,control,3)?)?,document_f64(tol,control)?)?)})?;
  document_list(fields[2].unwrap_or("[]"),&mut census,control,|value,census,control|{let[id,start,end,curve,tol]=native::record(value,control)?;let mut bytes=32;for value in[id,start,end]{bytes=add(bytes,native::hex_text_extent(value,control)?)?;}document_curve(curve,census,control,3)?;census.row(add(bytes,document_f64(tol,control)?)?)})?;
  document_list(fields[3].unwrap_or("[]"),&mut census,control,|value,census,control|{let[id,edges]=native::record(value,control)?;census.row(add(24,native::hex_text_extent(id,control)?)?)?;document_list(edges,census,control,|value,census,control|{let[edge,orientation]=native::record(value,control)?;document_boolean(orientation)?;census.row(add(32,native::hex_text_extent(edge,control)?)?)})})?;
  document_list(fields[4].unwrap_or("[]"),&mut census,control,|value,census,control|{let[id,outer,inner,surface,orientation,tol]=native::record(value,control)?;let bytes=add(add(40,native::hex_text_extent(id,control)?)?,native::hex_text_extent(outer,control)?)?;document_list(inner,census,control,|value,census,control|{let[id]=native::record(value,control)?;census.row(add(24,native::hex_text_extent(id,control)?)?)})?;document_surface(surface,census,control)?;document_boolean(orientation)?;census.row(add(bytes,document_f64(tol,control)?)?)})?;
  for index in[5,6]{document_list(fields[index].unwrap_or("[]"),&mut census,control,|value,census,control|{let[id,items]=native::record(value,control)?;census.row(add(24,native::hex_text_extent(id,control)?)?)?;document_list(items,census,control,|value,census,control|{let[id,flag]=native::record(value,control)?;document_boolean(flag)?;census.row(add(32,native::hex_text_extent(id,control)?)?)})})?;}
  document_list(fields[7].unwrap_or("[]"),&mut census,control,|value,census,control|{let[id,edge,forward,pcurve,prange,loop_id,next,prev]=native::record(value,control)?;let mut bytes=add(add(32,native::hex_text_extent(id,control)?)?,native::hex_text_extent(edge,control)?)?;document_boolean(forward)?;if pcurve!="-"{document_curve(pcurve.strip_prefix('~').ok_or_else(invalid)?,census,control,2)?;bytes=add(bytes,8)?;}bytes=add(bytes,document_point(prange,control,2)?)?;for value in[loop_id,next,prev]{bytes=add(bytes,native::hex_text_extent(value,control)?)?;}census.row(bytes)})?;control.checkpoint()
 })
}
