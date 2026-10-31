//! 🔺️ Borrowed Mesh primitives count each exact numeric buffer row and literal material reference.
use pack::snapshot::ArtifactSqliteSnapshot as _;
use super::{SemioMeshSnapshot,ValueError,ValueRefusalKind};
use store::{os_store::serialization::ArtifactSqliteSnapshotHost,sqlite_snapshot::SqliteDatabaseLimits};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use semio_s_artifact_stdio_semio_base::io::sqlite::snapshot::native_decoding as native;
type Result<T>=std::result::Result<T,ValueError>;
fn invalid()->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,"Semio Mesh native field differs from its authored shape")}
fn add(a:usize,b:usize)->Result<usize>{a.checked_add(b).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio Mesh complete cell extent overflow"))}
/// 🏛️ Admits all ten declared geometry and exact word tables before native ownership.
pub(super)fn layout(limits:SqliteDatabaseLimits)->Result<()>{
 let mut bytes=0;for statement in SemioMeshSnapshot::SQLITE_SCHEMA.split(';'){bytes=add(bytes,statement.trim().len())?;}for name in["semio_mesh_document","semio_mesh_mesh","semio_mesh_primitive","semio_mesh_position","semio_mesh_normal","semio_mesh_uv","semio_mesh_color","semio_mesh_index","semio_mesh_material","semio_mesh_texture"]{bytes=add(bytes,name.len())?;}
 if bytes.max(SemioMeshSnapshot::SQLITE_SCHEMA.len())>limits.max_schema_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio Mesh schema exceeds caller bytes"))}
 if limits.max_tables<10||limits.max_columns<27||limits.max_rows<1{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio Mesh authored layout exceeds copied limits"))}Ok(())
}
struct Census{limits:SqliteDatabaseLimits,rows:usize,bytes:usize}
impl Census{
 fn row(&mut self,bytes:usize)->Result<()>{self.rows=add(self.rows,1)?;self.bytes=add(self.bytes,bytes)?;if self.rows>self.limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio Mesh complete rows exceed caller limit"))}if self.bytes>self.limits.max_value_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio Mesh complete cells exceed caller bytes"))}Ok(())}
 fn future(&self,count:usize)->Result<()>{if add(self.rows,count)?>self.limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio Mesh declared rows exceed caller limit"))}Ok(())}
}
fn text(reader:&mut store::ByteReader<'_>,control:&mut NativeDecodeControl<'_>)->Result<usize>{Ok(control.borrow_text(native::bytes(reader)?)?.len())}
fn byte(reader:&mut store::ByteReader<'_>)->Result<u8>{reader.read_u8().map_err(|_|invalid())}
fn scalar(value:f64)->usize{if value.is_nan(){11}else if value.is_infinite(){32}else{22}}
fn topology(tag:u8)->Result<usize>{match tag{0=>Ok(6),1=>Ok(5),2=>Ok(10),3=>Ok(9),4=>Ok(14),5=>Ok(12),_=>Err(invalid())}}
fn octets(reader:&mut store::ByteReader<'_>,control:&mut NativeDecodeControl<'_>)->Result<usize>{let bytes=native::bytes(reader)?;control.scoped_stage(|control|{control.begin_stage(bytes.len())?;for _ in bytes{control.step()?;}control.checkpoint()?;Ok(bytes.len())})}
fn document_optional(value:&str,control:&mut NativeDecodeControl<'_>)->Result<usize>{if value=="[0]"{return Ok(0)}let[tag,value]=native::record(value,control)?;if tag!="1"{return Err(invalid())}native::hex_text_extent(value,control)}
fn document_list(value:&str,census:&mut Census,control:&mut NativeDecodeControl<'_>,mut visit:impl FnMut(&str,&mut Census,&mut NativeDecodeControl<'_>)->Result<()>)->Result<()>{
 let mut items=native::Items::new(value)?;let count=items.count(control,census.limits.max_rows)?;census.future(count)?;control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(value)=items.next(control)?{visit(value,census,control)?;control.step()?;}control.checkpoint()})
}
fn document_numbers(value:&str,census:&mut Census,control:&mut NativeDecodeControl<'_>,width:usize,bits32:bool)->Result<()>{
 document_list(value,census,control,|value,census,control|{
  let values=match width{2=>{let[x,y]=native::record(value,control)?;[x,y,"",""]},3=>{let[x,y,z]=native::record(value,control)?;[x,y,z,""]},4=>native::record(value,control)?,_=>return Err(invalid())};
  let mut bytes=24;for value in values.into_iter().take(width){bytes=add(bytes,if bits32{scalar(f64::from(native::float32(value,control)?))}else{scalar(native::float(value,control)?)})?;}census.row(bytes)
 })
}
fn hex(value:&str,control:&mut NativeDecodeControl<'_>)->Result<usize>{if !value.len().is_multiple_of(2){return Err(invalid())}control.scoped_stage(|control|{control.begin_stage(value.len()/2)?;for pair in value.as_bytes().chunks_exact(2){if !pair.iter().all(u8::is_ascii_hexdigit){return Err(invalid())}control.step()?;}control.checkpoint()?;Ok(value.len()/2)})}
/// 📝️ Keeps every original list, optional literal reference and accepted u32 index grammar.
pub fn document(body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<()>{
 layout(limits)?;control.scoped_stage(|control|{
  let fields=native::fields(body,["schema","meshes","materials","textures"],control)?;let mut census=Census{limits,rows:0,bytes:0};census.row(add(8,native::hex_text_extent(fields[0].ok_or_else(invalid)?,control)?)?)?;
  document_list(fields[1].unwrap_or("[]"),&mut census,control,|value,census,control|{
   let[id,primitives]=native::record(value,control)?;census.row(add(24,native::hex_text_extent(id,control)?)?)?;
   document_list(primitives,census,control,|value,census,control|{
    let[id,kind,positions,normals,uvs,colors,indices,material]=native::record(value,control)?;
    let tag=match kind{"P"=>0,"L"=>1,"S"=>2,"T"=>3,"X"=>4,"F"=>5,_=>return Err(invalid())};
    census.row(add(add(add(24,native::hex_text_extent(id,control)?)?,topology(tag)?)?,document_optional(material,control)?)?)?;
    document_numbers(positions,census,control,3,false)?;document_numbers(normals,census,control,3,false)?;document_numbers(uvs,census,control,2,false)?;document_numbers(colors,census,control,4,true)?;
    document_list(indices,census,control,|value,census,control|{control.scoped_stage(|control|{control.begin_stage(value.len())?;for _ in value.bytes(){control.step()?;}value.parse::<u32>().map_err(|_|invalid())?;control.checkpoint()})?;census.row(32)})
   })
  })?;
  document_list(fields[2].unwrap_or("[]"),&mut census,control,|value,census,control|{
   let[id,color,metallic,roughness,base_texture,metallic_texture,normal_texture,occlusion_texture,emissive_texture]=native::record(value,control)?;let[r,g,b,a]=native::record(color,control)?;
   let mut bytes=add(24,native::hex_text_extent(id,control)?)?;for value in[r,g,b,a,metallic,roughness]{bytes=add(bytes,scalar(f64::from(native::float32(value,control)?)))?;}for value in[base_texture,metallic_texture,normal_texture,occlusion_texture,emissive_texture]{bytes=add(bytes,document_optional(value,control)?)?;}census.row(bytes)
  })?;
  document_list(fields[3].unwrap_or("[]"),&mut census,control,|value,census,control|{let[id,mime,bytes]=native::record(value,control)?;census.row(add(add(add(24,native::hex_text_extent(id,control)?)?,native::hex_text_extent(mime,control)?)?,hex(bytes,control)?)?)?;Ok(())})?;
  control.checkpoint()
 })
}
