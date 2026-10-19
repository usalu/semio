//! 🔺️ Controlled native geometry buffers, complete PBR material fields and texture octets.
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint3;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioUv;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioRgba;
use crate::schema::snapshot::{SemioMeshSnapshot, SemioTopology, STDIO_SEMIOMESH_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_semio_base::io::sqlite::snapshot::native_decoding::{self as native,workspace};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use semio_framework_value::{ValueError,ValueRefusalKind};
use store::sqlite_snapshot::{SqliteSnapshotControl,SqliteDatabaseLimits};
/// 🛬️ Separates complete SQL cells from the same caller's actual native allocation backing.
pub(crate)fn decode(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control:&mut NativeDecodeControl<'_>)->Result<SemioMeshSnapshot,ValueError>{
 crate::io::sqlite::snapshot::admit_layout(control.limits())?;
 native::with_body(payload,STDIO_SEMIOMESH_DOCUMENT_SCHEMA,control,native_control,|body,decoder,limits|workspace::decode(decoder,empty,|output,decoder|match body{native::Body::Binary(body)=>binary_into(output,body,decoder,limits),native::Body::Text(body)=>document_into(output,body,decoder,limits)}))
}
fn topology(tag:u8)->Result<SemioTopology,ValueError>{Ok(match tag{0=>SemioTopology::Points,1=>SemioTopology::Lines,2=>SemioTopology::LineStrip,3=>SemioTopology::Triangles,4=>SemioTopology::TriangleStrip,5=>SemioTopology::TriangleFan,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native topology"))})}
fn f64(reader:&mut store::ByteReader<'_>)->Result<f64,ValueError>{reader.read_f64_le().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))}
fn f32(reader:&mut store::ByteReader<'_>)->Result<f32,ValueError>{Ok(f32::from_bits(reader.read_u32_le().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?))}
fn point(reader:&mut store::ByteReader<'_>)->Result<SemioPoint3,ValueError>{Ok(SemioPoint3{x:f64(reader)?,y:f64(reader)?,z:f64(reader)?})}
fn rgba(reader:&mut store::ByteReader<'_>)->Result<SemioRgba,ValueError>{Ok(SemioRgba{r:f32(reader)?,g:f32(reader)?,b:f32(reader)?,a:f32(reader)?})}


/// 🔘️ Fills one optional texture reference in its original slot from borrowed binary octets.
fn optional_text_into(output:&mut Option<String>,reader:&mut store::ByteReader<'_>,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{match reader.read_u8().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?{0=>Ok(()),1=>{*output=Some(String::new());native::reader_text_into(output.as_mut().unwrap(),reader,control)},_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native texture reference presence"))}}
/// ☑️ Fills one optional texture reference in its original slot from a bracketed hexadecimal record.
fn optional_text_record_into(output:&mut Option<String>,value:&str,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{if value=="[0]"{return Ok(())}let[tag,value]=native::record(value,control)?;if tag!="1"{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native texture reference presence"))}*output=Some(String::new());native::hex_text_into(output.as_mut().unwrap(),value,control)}
/// 🧩️ Fills the retained mesh root from borrowed binary octets, inserting every mesh, primitive, material and texture before its fields.
pub fn binary_into(output:&mut SemioMeshSnapshot,body:&[u8],control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<(),ValueError>{
 crate::io::sqlite::snapshot::admit_binary(body,control,limits)?;
 let mut entities=0;native::entities(&mut entities,1,limits)?;let mut reader=store::ByteReader::new(body);if reader.read_u8().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unsupported Semio mesh native format"))}native::reader_text_into(&mut output.schema,&mut reader,control)?;
 native::binary_list_into(&mut output.meshes,&mut reader,control,limits,&mut entities,Default::default,|mesh,reader,control,entities|{
  native::reader_text_into(&mut mesh.id,reader,control)?;
  native::binary_list_into(&mut mesh.primitives,reader,control,limits,entities,Default::default,|primitive,reader,control,entities|{
   native::reader_text_into(&mut primitive.id,reader,control)?;primitive.topology=topology(reader.read_u8().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?)?;
   native::binary_list_into(&mut primitive.positions,reader,control,limits,entities,Default::default,|slot,reader,_,_|{*slot=point(reader)?;Ok(())})?;native::binary_list_into(&mut primitive.normals,reader,control,limits,entities,Default::default,|slot,reader,_,_|{*slot=point(reader)?;Ok(())})?;native::binary_list_into(&mut primitive.uvs,reader,control,limits,entities,Default::default,|slot,reader,_,_|{*slot=SemioUv{u:f64(reader)?,v:f64(reader)?};Ok(())})?;native::binary_list_into(&mut primitive.colors,reader,control,limits,entities,Default::default,|slot,reader,_,_|{*slot=rgba(reader)?;Ok(())})?;native::binary_list_into(&mut primitive.indices,reader,control,limits,entities,Default::default,|slot,reader,_,_|{*slot=reader.read_u32_le().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?;Ok(())})?;
   match reader.read_u8().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?{0=>{},1=>{primitive.material_id=Some(String::new());native::reader_text_into(primitive.material_id.as_mut().unwrap(),reader,control)?;},_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native material presence"))}
   Ok(())
  })
 })?;
 native::binary_list_into(&mut output.materials,&mut reader,control,limits,&mut entities,Default::default,|material,reader,control,_|{native::reader_text_into(&mut material.id,reader,control)?;material.base_color=rgba(reader)?;material.metallic=f32(reader)?;material.roughness=f32(reader)?;optional_text_into(&mut material.base_color_texture,reader,control)?;optional_text_into(&mut material.metallic_roughness_texture,reader,control)?;optional_text_into(&mut material.normal_texture,reader,control)?;optional_text_into(&mut material.occlusion_texture,reader,control)?;optional_text_into(&mut material.emissive_texture,reader,control)})?;
 native::binary_list_into(&mut output.textures,&mut reader,control,limits,&mut entities,Default::default,|texture,reader,control,_|{native::reader_text_into(&mut texture.id,reader,control)?;native::reader_text_into(&mut texture.mime,reader,control)?;native::reader_bytes_into(&mut texture.bytes,reader,control)})?;
 if reader.remaining()!=0{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio mesh native trailing bytes"))}Ok(())
}
fn point_text(value:&str,control:&mut NativeDecodeControl<'_>)->Result<SemioPoint3,ValueError>{let[x,y,z]=native::record(value,control)?;Ok(SemioPoint3{x:native::float(x,control)?,y:native::float(y,control)?,z:native::float(z,control)?})}
fn rgba_text(value:&str,control:&mut NativeDecodeControl<'_>)->Result<SemioRgba,ValueError>{let[r,g,b,a]=native::record(value,control)?;Ok(SemioRgba{r:native::float32(r,control)?,g:native::float32(g,control)?,b:native::float32(b,control)?,a:native::float32(a,control)?})}
/// 📝️ Fills the retained mesh root from the borrowed document body, inserting every mesh, primitive, material and texture before its fields.
pub fn document_into(output:&mut SemioMeshSnapshot,body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<(),ValueError>{
 crate::io::sqlite::snapshot::admit_document(body,control,limits)?;
 let mut entities=0;native::entities(&mut entities,1,limits)?;let fields=native::fields(body,["schema","meshes","materials","textures"],control)?;native::hex_text_into(&mut output.schema,fields[0].ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Semio mesh native schema missing"))?,control)?;
 native::text_list_into(&mut output.meshes,fields[1].unwrap_or("[]"),control,limits,&mut entities,Default::default,|mesh,value,control,entities|{
  let[id,primitives]=native::record(value,control)?;native::hex_text_into(&mut mesh.id,id,control)?;
  native::text_list_into(&mut mesh.primitives,primitives,control,limits,entities,Default::default,|primitive,value,control,entities|{
   let[id,topology_fields,positions,normals,uvs,colors,indices,material_fields]=native::record(value,control)?;native::hex_text_into(&mut primitive.id,id,control)?;let tag=match topology_fields{"P"=>0,"L"=>1,"S"=>2,"T"=>3,"X"=>4,"F"=>5,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native topology"))};primitive.topology=topology(tag)?;
   native::text_list_into(&mut primitive.positions,positions,control,limits,entities,Default::default,|slot,value,control,_|{*slot=point_text(value,control)?;Ok(())})?;native::text_list_into(&mut primitive.normals,normals,control,limits,entities,Default::default,|slot,value,control,_|{*slot=point_text(value,control)?;Ok(())})?;native::text_list_into(&mut primitive.uvs,uvs,control,limits,entities,Default::default,|slot,value,control,_|{let[u,v]=native::record(value,control)?;*slot=SemioUv{u:native::float(u,control)?,v:native::float(v,control)?};Ok(())})?;native::text_list_into(&mut primitive.colors,colors,control,limits,entities,Default::default,|slot,value,control,_|{*slot=rgba_text(value,control)?;Ok(())})?;native::text_list_into(&mut primitive.indices,indices,control,limits,entities,Default::default,|slot,value,control,_|{*slot=control.scoped_stage(|control|{control.begin_stage(value.len())?;for _ in value.bytes(){control.step()?;}value.parse::<u32>().map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native index"))})?;Ok(())})?;
   if material_fields!="[0]"{let[tag,value]=native::record(material_fields,control)?;if tag!="1"{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native material presence"))}primitive.material_id=Some(String::new());native::hex_text_into(primitive.material_id.as_mut().unwrap(),value,control)?;}
   Ok(())
  })
 })?;
 native::text_list_into(&mut output.materials,fields[2].unwrap_or("[]"),control,limits,&mut entities,Default::default,|material,value,control,_|{let[id,color,metallic,roughness,base_color_texture,metallic_roughness_texture,normal_texture,occlusion_texture,emissive_texture]=native::record(value,control)?;native::hex_text_into(&mut material.id,id,control)?;material.base_color=rgba_text(color,control)?;material.metallic=native::float32(metallic,control)?;material.roughness=native::float32(roughness,control)?;optional_text_record_into(&mut material.base_color_texture,base_color_texture,control)?;optional_text_record_into(&mut material.metallic_roughness_texture,metallic_roughness_texture,control)?;optional_text_record_into(&mut material.normal_texture,normal_texture,control)?;optional_text_record_into(&mut material.occlusion_texture,occlusion_texture,control)?;optional_text_record_into(&mut material.emissive_texture,emissive_texture,control)})?;
 native::text_list_into(&mut output.textures,fields[3].unwrap_or("[]"),control,limits,&mut entities,Default::default,|texture,value,control,_|{let[id,mime,bytes]=native::record(value,control)?;native::hex_text_into(&mut texture.id,id,control)?;native::hex_text_into(&mut texture.mime,mime,control)?;native::hex_bytes_into(&mut texture.bytes,bytes,control)})
}
/// 🕳️ The empty typed root every native prefix starts from.
pub fn empty()->SemioMeshSnapshot{SemioMeshSnapshot{schema:String::new(),meshes:Vec::new(),materials:Vec::new(),textures:Vec::new()}}
