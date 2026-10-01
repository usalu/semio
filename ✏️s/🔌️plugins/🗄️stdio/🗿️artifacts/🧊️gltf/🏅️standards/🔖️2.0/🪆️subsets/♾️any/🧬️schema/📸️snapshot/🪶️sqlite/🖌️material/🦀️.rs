//! 🖌️ Authored material factors and texture slots retain exact IEEE words and optional presence.
use super::*;
const MATERIAL:&[FloatColumn]=&[FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(8)];
const PBR:&[FloatColumn]=&[FloatColumn::Binary64(1),FloatColumn::Binary64(2),FloatColumn::Binary64(3),FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6)];
const SLOT:&[FloatColumn]=&[FloatColumn::Binary64(5)];
fn texture(write:&mut Write<'_,'_>,table:&'static str,key:i64,value:&Option<GltfTextureInfo>)->Result<(),String>{if let Some(value)=value{let [high,low]=index(value.index);let [coord_high,coord_low]=word(value.tex_coord);let [extension,extra]=write.extras(&value.extensions,&value.extras)?;write.insert_key(table,key,&[high,low,coord_high,coord_low,extension,extra])?;}Ok(())}
fn read_texture(read:&mut Read<'_,'_,'_>,table:&'static str,key:i64)->Result<Option<GltfTextureInfo>,String>{read.optional_key(table,key)?.map(|row|Ok(GltfTextureInfo{index:read.index(row,1)?,tex_coord:read.word(row,3)?,extensions:read.json(row,5)?,extras:read.json(row,6)?})).transpose()}
pub(super) fn project(write:&mut Write<'_,'_>,materials:&[GltfMaterial])->Result<(),String>{
 write.check(materials.len())?;for(position,material)in materials.iter().enumerate(){
  let [extension,extra]=write.extras(&material.extensions,&material.extras)?;let mode=match material.alpha_mode{GltfAlphaMode::Opaque=>"OPAQUE",GltfAlphaMode::Mask=>"MASK",GltfAlphaMode::Blend=>"BLEND"};
  let key=write.floats("gltf_material",&[Cell::Integer(1),ordinal(position)?,text(&material.name),Cell::Real(material.emissive_factor[0]),Cell::Real(material.emissive_factor[1]),Cell::Real(material.emissive_factor[2]),Cell::Text(mode),Cell::Real(material.alpha_cutoff),Cell::Integer(i64::from(material.double_sided)),extension,extra],MATERIAL)?;
  if let Some(pbr)=&material.pbr_metallic_roughness{let [extension,extra]=write.extras(&pbr.extensions,&pbr.extras)?;write.float_key("gltf_pbr_metallic_roughness",key,&[Cell::Real(pbr.base_color_factor[0]),Cell::Real(pbr.base_color_factor[1]),Cell::Real(pbr.base_color_factor[2]),Cell::Real(pbr.base_color_factor[3]),Cell::Real(pbr.metallic_factor),Cell::Real(pbr.roughness_factor),extension,extra],PBR)?;texture(write,"gltf_base_color_texture",key,&pbr.base_color_texture)?;texture(write,"gltf_metallic_roughness_texture",key,&pbr.metallic_roughness_texture)?;}
  texture(write,"gltf_emissive_texture",key,&material.emissive_texture)?;
  if let Some(value)=&material.normal_texture{let [high,low]=index(value.index);let [coord_high,coord_low]=word(value.tex_coord);let [extension,extra]=write.extras(&value.extensions,&value.extras)?;write.float_key("gltf_normal_texture",key,&[high,low,coord_high,coord_low,Cell::Real(value.scale),extension,extra],SLOT)?;}
  if let Some(value)=&material.occlusion_texture{let [high,low]=index(value.index);let [coord_high,coord_low]=word(value.tex_coord);let [extension,extra]=write.extras(&value.extensions,&value.extras)?;write.float_key("gltf_occlusion_texture",key,&[high,low,coord_high,coord_low,Cell::Real(value.strength),extension,extra],SLOT)?;}
 }Ok(())
}
pub(super) fn reconstruct(read:&mut Read<'_,'_,'_>)->Result<Vec<GltfMaterial>,String>{
 let rows=read.rows("gltf_material",1,1,2)?;let mut result=Vec::with_capacity(rows.len());for row in rows{let key=row.rowid;let floating=FloatRow::new(row,MATERIAL)?;
  let pbr=read.optional_key("gltf_pbr_metallic_roughness",key)?.map(|row|{let floating=FloatRow::new(row,PBR)?;Ok::<_,String>(GltfPbrMetallicRoughness{base_color_factor:[floating.real(1)?,floating.real(2)?,floating.real(3)?,floating.real(4)?],metallic_factor:floating.real(5)?,roughness_factor:floating.real(6)?,base_color_texture:read_texture(read,"gltf_base_color_texture",key)?,metallic_roughness_texture:read_texture(read,"gltf_metallic_roughness_texture",key)?,extensions:read.json(row,7)?,extras:read.json(row,8)?})}).transpose()?;
  let normal=read.optional_key("gltf_normal_texture",key)?.map(|row|{let floating=FloatRow::new(row,SLOT)?;Ok::<_,String>(GltfNormalTextureInfo{index:read.index(row,1)?,tex_coord:read.word(row,3)?,scale:floating.real(5)?,extensions:read.json(row,6)?,extras:read.json(row,7)?})}).transpose()?;
  let occlusion=read.optional_key("gltf_occlusion_texture",key)?.map(|row|{let floating=FloatRow::new(row,SLOT)?;Ok::<_,String>(GltfOcclusionTextureInfo{index:read.index(row,1)?,tex_coord:read.word(row,3)?,strength:floating.real(5)?,extensions:read.json(row,6)?,extras:read.json(row,7)?})}).transpose()?;
  let mode=match row.text(7)?{"OPAQUE"=>GltfAlphaMode::Opaque,"MASK"=>GltfAlphaMode::Mask,"BLEND"=>GltfAlphaMode::Blend,_=>return Err("GLTF alpha mode differs".into())};
  result.push(GltfMaterial{name:read.optional_text(row,3)?,emissive_factor:[floating.real(4)?,floating.real(5)?,floating.real(6)?],alpha_mode:mode,alpha_cutoff:floating.real(8)?,double_sided:read.boolean(row,9)?,pbr_metallic_roughness:pbr,normal_texture:normal,occlusion_texture:occlusion,emissive_texture:read_texture(read,"gltf_emissive_texture",key)?,extensions:read.json(row,10)?,extras:read.json(row,11)?});
 }Ok(result)
}
