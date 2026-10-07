//! 🖊️ Borrowed Drawing primitives count actual canvas, style, layer, path and scene cells.
use super::{SemioDrawingSnapshot,ValueError,ValueRefusalKind};
use store::{ArtifactSqliteSnapshot,sqlite_snapshot::SqliteDatabaseLimits};
use semio_framework_value::native_decoding::NativeDecodeControl;
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding as native;
type Result<T>=std::result::Result<T,ValueError>;
fn invalid()->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,"Semio Drawing native field differs from its authored shape")}
fn add(a:usize,b:usize)->Result<usize>{a.checked_add(b).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio Drawing complete cell extent overflow"))}
/// 🏛️ Admits the full independently authored Drawing layout before native ownership.
pub(super)fn layout(limits:SqliteDatabaseLimits)->Result<()>{
 let mut bytes=0;for statement in SemioDrawingSnapshot::SQLITE_SCHEMA.split(';'){bytes=add(bytes,statement.trim().len())?;}
 for name in["semio_drawing_document","semio_drawing_style","semio_drawing_layer","semio_drawing_node","semio_drawing_path","semio_drawing_segment","semio_drawing_move","semio_drawing_line","semio_drawing_cubic","semio_drawing_quad","semio_drawing_arc","semio_drawing_close","semio_drawing_text","semio_drawing_group","semio_drawing_child","semio_drawing_image"]{bytes=add(bytes,name.len())?;}
 if bytes.max(SemioDrawingSnapshot::SQLITE_SCHEMA.len())>limits.max_schema_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio Drawing schema exceeds caller bytes"))}
 if limits.max_tables<16||limits.max_columns<34||limits.max_rows<1{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio Drawing authored layout exceeds copied limits"))}Ok(())
}
struct Census{limits:SqliteDatabaseLimits,rows:usize,bytes:usize}
impl Census{
 fn row(&mut self,bytes:usize)->Result<()>{let rows=add(self.rows,1)?;let bytes=add(self.bytes,bytes)?;if rows>self.limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio Drawing complete row count exceeds copied limit"))}if bytes>self.limits.max_value_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio Drawing complete cell bytes exceed copied limit"))}self.rows=rows;self.bytes=bytes;Ok(())}
 fn future(&self,count:usize)->Result<()>{if add(self.rows,count)?>self.limits.max_rows{Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio Drawing native list exceeds copied row limit"))}else{Ok(())}}
}
fn byte(reader:&mut store::ByteReader<'_>)->Result<u8>{reader.read_u8().map_err(|_|invalid())}
fn text(reader:&mut store::ByteReader<'_>,control:&mut NativeDecodeControl<'_>)->Result<usize>{Ok(control.borrow_text(native::bytes(reader)?)?.len())}
fn scalar(value:f64)->usize{if value.is_nan(){11}else if value.is_infinite(){32}else{22}}
fn binary_f64(reader:&mut store::ByteReader<'_>)->Result<usize>{Ok(scalar(reader.read_f64_le().map_err(|_|invalid())?))}
fn binary_f32(reader:&mut store::ByteReader<'_>)->Result<usize>{let bytes=reader.read_bytes(4).map_err(|_|invalid())?;Ok(scalar(f64::from(f32::from_le_bytes(bytes.try_into().map_err(|_|invalid())?))))}
fn binary_words(reader:&mut store::ByteReader<'_>,count:usize)->Result<usize>{let mut bytes=0;for _ in 0..count{bytes=add(bytes,binary_f64(reader)?)?;}Ok(bytes)}
fn binary_rgba(reader:&mut store::ByteReader<'_>,_:&mut NativeDecodeControl<'_>)->Result<usize>{let mut bytes=0;for _ in 0..4{bytes=add(bytes,binary_f32(reader)?)?;}Ok(bytes)}
fn binary_option(reader:&mut store::ByteReader<'_>,control:&mut NativeDecodeControl<'_>,read:impl FnOnce(&mut store::ByteReader<'_>,&mut NativeDecodeControl<'_>)->Result<usize>)->Result<usize>{match byte(reader)?{0=>Ok(0),1=>read(reader,control),_=>Err(invalid())}}
fn binary_reference(reader:&mut store::ByteReader<'_>,control:&mut NativeDecodeControl<'_>)->Result<usize>{binary_option(reader,control,|reader,control|{text(reader,control)?;Ok(8)})}
fn binary_boolean(reader:&mut store::ByteReader<'_>)->Result<()>{match byte(reader)?{0|1=>Ok(()),_=>Err(invalid())}}
fn binary_list(reader:&mut store::ByteReader<'_>,census:&mut Census,control:&mut NativeDecodeControl<'_>,mut visit:impl FnMut(&mut store::ByteReader<'_>,&mut Census,&mut NativeDecodeControl<'_>)->Result<()>)->Result<()>{
 let count=native::length(reader)?;census.future(count)?;control.scoped_stage(|control|{control.begin_stage(count)?;for _ in 0..count{visit(reader,census,control)?;control.step()?;}control.checkpoint()})
}
fn binary_segment(reader:&mut store::ByteReader<'_>,census:&mut Census,_:&mut NativeDecodeControl<'_>)->Result<()>{
 let tag=byte(reader)?;let(kind,words)=match tag{0=>(4,2),1=>(4,2),2=>(5,6),3=>(4,4),4=>(3,5),5=>(5,0),_=>return Err(invalid())};census.row(add(24,kind)?)?;
 let bytes=if tag==4{let mut bytes=binary_words(reader,3)?;binary_boolean(reader)?;binary_boolean(reader)?;bytes=add(bytes,16)?;add(bytes,binary_words(reader,2)?)?}else{binary_words(reader,words)?};census.row(add(8,bytes)?)
}
fn binary_node(reader:&mut store::ByteReader<'_>,census:&mut Census,control:&mut NativeDecodeControl<'_>)->Result<()>{
 control.scoped_depth(64,|control|control.scoped_stage(|control|{control.begin_stage(0)?;let tag=byte(reader)?;let kind=match tag{0|1=>4,2|3=>5,_=>return Err(invalid())};census.row(add(8,kind)?)?;
 match tag{
  0=>{binary_list(reader,census,control,binary_segment)?;census.row(add(8,binary_reference(reader,control)?)?)?;},
  1=>{let mut bytes=add(8,text(reader,control)?)?;bytes=add(bytes,binary_words(reader,2)?)?;bytes=add(bytes,binary_reference(reader,control)?)?;census.row(bytes)?;},
  2=>{census.row(add(8,binary_words(reader,10)?)?)?;binary_list(reader,census,control,|reader,census,control|{census.row(32)?;binary_node(reader,census,control)})?;},
  3=>{let mut bytes=add(8,binary_words(reader,4)?)?;bytes=add(bytes,text(reader,control)?)?;bytes=add(bytes,native::bytes(reader)?.len())?;census.row(bytes)?;},_=>return Err(invalid())
 }control.step()?;Ok(())}))
}
/// 📦️ Counts original Drawing binary records before typed construction.
pub(super)fn binary(body:&[u8],control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<()>{
 layout(limits)?;control.scoped_stage(|control|{
  let mut reader=store::ByteReader::new(body);if byte(&mut reader)?!=1{return Err(invalid())}let mut census=Census{limits,rows:0,bytes:0};
  let mut bytes=add(8,text(&mut reader,control)?)?;bytes=add(bytes,binary_words(&mut reader,2)?)?;bytes=add(bytes,binary_option(&mut reader,control,binary_rgba)?)?;census.row(bytes)?;
  binary_list(&mut reader,&mut census,control,|reader,census,control|{let mut bytes=add(24,text(reader,control)?)?;for _ in 0..2{bytes=add(bytes,binary_option(reader,control,binary_rgba)?)?;}bytes=add(bytes,binary_option(reader,control,|reader,_|binary_f64(reader))?)?;bytes=add(bytes,binary_option(reader,control,|reader,_|binary_f32(reader))?)?;census.row(bytes)})?;
  binary_list(&mut reader,&mut census,control,|reader,census,control|{let bytes=add(add(40,text(reader,control)?)?,text(reader,control)?)?;binary_boolean(reader)?;census.row(bytes)?;binary_node(reader,census,control)})?;
  if reader.remaining()!=0{return Err(invalid())}control.checkpoint()
 })
}
fn document_f64(value:&str,control:&mut NativeDecodeControl<'_>)->Result<usize>{Ok(scalar(native::float(value,control)?))}
fn document_f32(value:&str,control:&mut NativeDecodeControl<'_>)->Result<usize>{Ok(scalar(f64::from(native::float32(value,control)?)))}
fn document_rgba(value:&str,control:&mut NativeDecodeControl<'_>)->Result<usize>{let[r,g,b,a]=native::record(value,control)?;let mut bytes=0;for value in[r,g,b,a]{bytes=add(bytes,document_f32(value,control)?)?;}Ok(bytes)}
fn document_point(value:&str,control:&mut NativeDecodeControl<'_>)->Result<usize>{let[x,y]=native::record(value,control)?;add(document_f64(x,control)?,document_f64(y,control)?)}
fn document_point3(value:&str,control:&mut NativeDecodeControl<'_>)->Result<usize>{let[x,y,z]=native::record(value,control)?;let mut bytes=0;for value in[x,y,z]{bytes=add(bytes,document_f64(value,control)?)?;}Ok(bytes)}
fn document_transform(value:&str,control:&mut NativeDecodeControl<'_>)->Result<usize>{let[translation,rotation,scale]=native::record(value,control)?;let[x,y,z,w]=native::record(rotation,control)?;let mut bytes=document_point3(translation,control)?;for value in[x,y,z,w]{bytes=add(bytes,document_f64(value,control)?)?;}add(bytes,document_point3(scale,control)?)}
fn document_option(value:&str,control:&mut NativeDecodeControl<'_>,read:impl FnOnce(&str,&mut NativeDecodeControl<'_>)->Result<usize>)->Result<usize>{if value=="[0]"{return Ok(0)}let[tag,value]=native::record(value,control)?;if tag!="1"{return Err(invalid())}read(value,control)}
fn document_reference(value:&str,control:&mut NativeDecodeControl<'_>)->Result<usize>{document_option(value,control,|value,control|{native::hex_text_extent(value,control)?;Ok(8)})}
fn document_blob(value:&str,control:&mut NativeDecodeControl<'_>)->Result<usize>{if !value.len().is_multiple_of(2){return Err(invalid())}control.scoped_stage(|control|{control.begin_stage(value.len())?;for value in value.bytes(){if !value.is_ascii_hexdigit(){return Err(invalid())}control.step()?;}control.checkpoint()?;Ok(value.len()/2)})}
fn document_boolean(value:&str)->Result<()>{match value{"0"|"1"=>Ok(()),_=>Err(invalid())}}
fn document_list(value:&str,census:&mut Census,control:&mut NativeDecodeControl<'_>,mut visit:impl FnMut(&str,&mut Census,&mut NativeDecodeControl<'_>)->Result<()>)->Result<()>{
 let mut items=native::Items::new(value)?;let count=items.count(control,census.limits.max_rows)?;census.future(count)?;control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(value)=items.next(control)?{visit(value,census,control)?;control.step()?;}control.checkpoint()})
}
fn document_segment(value:&str,census:&mut Census,control:&mut NativeDecodeControl<'_>)->Result<()>{
 if value=="Z"{census.row(29)?;return census.row(8)}
 let(tag,value)=value.split_at_checked(1).ok_or_else(invalid)?;let(kind,bytes)=match tag{
  "M"|"L"=>{let[to]=native::record(value,control)?;(4,document_point(to,control)?)},
  "C"=>{let[c1,c2,to]=native::record(value,control)?;(5,add(add(document_point(c1,control)?,document_point(c2,control)?)?,document_point(to,control)?)?)},
  "Q"=>{let[c,to]=native::record(value,control)?;(4,add(document_point(c,control)?,document_point(to,control)?)?)},
  "A"=>{let[rx,ry,x_rotation,large_arc,sweep,to]=native::record(value,control)?;document_boolean(large_arc)?;document_boolean(sweep)?;let mut bytes=16;for value in[rx,ry,x_rotation]{bytes=add(bytes,document_f64(value,control)?)?;}(3,add(bytes,document_point(to,control)?)?)},_=>return Err(invalid())
 };census.row(add(24,kind)?)?;census.row(add(8,bytes)?)
}
fn document_node(value:&str,census:&mut Census,control:&mut NativeDecodeControl<'_>)->Result<()>{
 control.scoped_depth(64,|control|control.scoped_stage(|control|{control.begin_stage(0)?;let(tag,value)=value.split_at_checked(1).ok_or_else(invalid)?;let kind=match tag{"P"|"T"=>4,"G"|"I"=>5,_=>return Err(invalid())};census.row(add(8,kind)?)?;
 match tag{
  "P"=>{let[segments,style]=native::record(value,control)?;document_list(segments,census,control,document_segment)?;census.row(add(8,document_reference(style,control)?)?)?;},
  "T"=>{let[value,at,style]=native::record(value,control)?;census.row(add(add(add(8,native::hex_text_extent(value,control)?)?,document_point(at,control)?)?,document_reference(style,control)?)?)?;},
  "G"=>{let[transform,children]=native::record(value,control)?;census.row(add(8,document_transform(transform,control)?)?)?;document_list(children,census,control,|value,census,control|{census.row(32)?;document_node(value,census,control)})?;},
  "I"=>{let[at,width,height,mime,bytes]=native::record(value,control)?;let mut extent=add(8,document_point(at,control)?)?;extent=add(extent,document_f64(width,control)?)?;extent=add(extent,document_f64(height,control)?)?;extent=add(extent,native::hex_text_extent(mime,control)?)?;census.row(add(extent,document_blob(bytes,control)?)?)?;},_=>return Err(invalid())
 }control.step()?;Ok(())}))
}
/// 📝️ Counts Drawing native field records using the actual binary32 and binary64 parsers.
pub(super)fn document(body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<()>{
 layout(limits)?;control.scoped_stage(|control|{
  let fields=native::fields(body,["schema","canvas","styles","layers"],control)?;let mut census=Census{limits,rows:0,bytes:0};
  let mut bytes=add(8,native::hex_text_extent(fields[0].ok_or_else(invalid)?,control)?)?;let[width,height,background]=native::record(fields[1].ok_or_else(invalid)?,control)?;bytes=add(bytes,document_f64(width,control)?)?;bytes=add(bytes,document_f64(height,control)?)?;bytes=add(bytes,document_option(background,control,document_rgba)?)?;census.row(bytes)?;
  document_list(fields[2].unwrap_or("[]"),&mut census,control,|value,census,control|{let[name,fill,stroke,stroke_width,opacity]=native::record(value,control)?;let mut bytes=add(24,native::hex_text_extent(name,control)?)?;for value in[fill,stroke]{bytes=add(bytes,document_option(value,control,document_rgba)?)?;}bytes=add(bytes,document_option(stroke_width,control,document_f64)?)?;bytes=add(bytes,document_option(opacity,control,document_f32)?)?;census.row(bytes)})?;
  document_list(fields[3].unwrap_or("[]"),&mut census,control,|value,census,control|{let[id,name,visible,root]=native::record(value,control)?;document_boolean(visible)?;census.row(add(add(40,native::hex_text_extent(id,control)?)?,native::hex_text_extent(name,control)?)?)?;document_node(root,census,control)})?;control.checkpoint()
 })
}
