//! 🎥️ Camera projection variants retain optional IEEE values and independent metadata.
use super::*;
const FLOATS:&[FloatColumn]=&[FloatColumn::Binary64(1),FloatColumn::Binary64(2),FloatColumn::Binary64(3),FloatColumn::Binary64(4)];
pub(super) fn project(write:&mut Write<'_,'_>,cameras:&[GltfCamera])->Result<(),String>{
 write.check(cameras.len())?;for(position,camera)in cameras.iter().enumerate(){let kind=match camera.projection{GltfCameraProjection::Perspective(_)=>"perspective",GltfCameraProjection::Orthographic(_)=>"orthographic"};let [extension,extra]=write.extras(&camera.extensions,&camera.extras)?;let key=write.insert("gltf_camera",&[Cell::Integer(1),ordinal(position)?,Cell::Text(kind),text(&camera.name),extension,extra])?;
  match &camera.projection{GltfCameraProjection::Perspective(value)=>{let [extension,extra]=write.extras(&value.extensions,&value.extras)?;write.float_key("gltf_camera_perspective",key,&[value.aspect_ratio.map_or(Cell::Null,Cell::Real),Cell::Real(value.yfov),value.zfar.map_or(Cell::Null,Cell::Real),Cell::Real(value.znear),extension,extra],FLOATS)?;},GltfCameraProjection::Orthographic(value)=>{let [extension,extra]=write.extras(&value.extensions,&value.extras)?;write.float_key("gltf_camera_orthographic",key,&[Cell::Real(value.xmag),Cell::Real(value.ymag),Cell::Real(value.zfar),Cell::Real(value.znear),extension,extra],FLOATS)?;}}
 }Ok(())
}
pub(super) fn reconstruct(read:&mut Read<'_,'_,'_>)->Result<Vec<GltfCamera>,String>{
 let rows=read.rows("gltf_camera",1,1,2)?;let mut result=Vec::with_capacity(rows.len());for row in rows{let projection=match row.text(3)?{
  "perspective"=>{let row=read.key("gltf_camera_perspective",row.rowid)?;let value=FloatRow::new(row,FLOATS)?;GltfCameraProjection::Perspective(GltfPerspective{aspect_ratio:if value.is_null(1)?{None}else{Some(value.real(1)?)},yfov:value.real(2)?,zfar:if value.is_null(3)?{None}else{Some(value.real(3)?)},znear:value.real(4)?,extensions:read.json(row,5)?,extras:read.json(row,6)?})},
  "orthographic"=>{let row=read.key("gltf_camera_orthographic",row.rowid)?;let value=FloatRow::new(row,FLOATS)?;GltfCameraProjection::Orthographic(GltfOrthographic{xmag:value.real(1)?,ymag:value.real(2)?,zfar:value.real(3)?,znear:value.real(4)?,extensions:read.json(row,5)?,extras:read.json(row,6)?})},
  _=>return Err("GLTF camera projection differs".into())
 };result.push(GltfCamera{projection,name:read.optional_text(row,4)?,extensions:read.json(row,5)?,extras:read.json(row,6)?});}Ok(result)
}
