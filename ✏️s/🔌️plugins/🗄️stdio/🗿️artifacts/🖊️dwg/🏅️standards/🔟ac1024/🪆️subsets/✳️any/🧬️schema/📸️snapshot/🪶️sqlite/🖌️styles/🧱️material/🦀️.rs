//! 🧱️ DWG materials and multiline styles retain owned maps, colors and transforms.
use semio_framework_value::{ValueError,ValueRefusalKind};
use super::super::*;
use super::number::Projection;
use super::color;
use super::evaluation::{project_coordinates as coords,reconstruct_coordinates as read_coords};
use super::reader::{boolean,optional_integer,ordinal,Reader};
use semio_framework_os_kernel::sqlite_snapshot::artifact::Cell::{Integer as I,Text as T,Real as R,Null as N};

fn projection(v:DwgMaterialProjection)->&'static str{match v{DwgMaterialProjection::Inherit=>"inherit",DwgMaterialProjection::Planar=>"planar",DwgMaterialProjection::Box=>"box",DwgMaterialProjection::Cylinder=>"cylinder",DwgMaterialProjection::Sphere=>"sphere"}}
fn tiling(v:DwgMaterialTiling)->&'static str{match v{DwgMaterialTiling::Inherit=>"inherit",DwgMaterialTiling::Tile=>"tile",DwgMaterialTiling::Crop=>"crop",DwgMaterialTiling::Clamp=>"clamp",DwgMaterialTiling::Mirror=>"mirror"}}
pub(super) fn project_material(p:&mut Projection<'_,'_>,id:i64,v:&DwgMaterial)->Result<(),ValueError>{
 p.insert_key("dwg_material",id,&[T(&v.name),T(&v.description),R(v.specular_gloss),R(v.opacity),R(v.refraction_index),R(v.translucence),R(v.self_illumination),R(v.reflectivity)])?;
 let c=&v.enabled_channels;p.insert_key("dwg_material_channels",id,&[I(i64::from(c.diffuse)),I(i64::from(c.specular)),I(i64::from(c.reflection)),I(i64::from(c.opacity)),I(i64::from(c.bump)),I(i64::from(c.refraction))])?;
 for(index,(channel,color))in [("ambient",&v.ambient),("diffuse",&v.diffuse),("specular",&v.specular)].into_iter().enumerate(){p.insert("dwg_material_color",&[I(id),I(ordinal(index)?),T(channel),R(color.factor),color.override_rgb.map_or(N,|value|I(i64::from(value)))])?;}
 for(index,(channel,map))in [("diffuse",&v.diffuse_map),("specular",&v.specular_map),("reflection",&v.reflection_map),("opacity",&v.opacity_map),("bump",&v.bump_map),("refraction",&v.refraction_map)].into_iter().enumerate(){
 let source=match map.source{DwgMaterialMapSource::None=>"none",DwgMaterialMapSource::CurrentScene=>"current_scene"};
 let map_id=p.insert("dwg_material_map",&[I(id),I(ordinal(index)?),T(channel),R(map.blend_factor),T(projection(map.projection)),T(tiling(map.tiling)),I(i64::from(map.scale_to_entity)),I(i64::from(map.use_current_block_transform)),T(source)])?;coords(p,"dwg_material_map_transform_coordinate",map_id,&map.transform)?;}Ok(())
}
pub(super) fn reconstruct_material(r:&mut Reader<'_,'_,'_>,id:i64)->Result<DwgMaterial,ValueError>{
 let row=r.component("dwg_material",id)?;let channels=r.component("dwg_material_channels",id)?;
 let colors=r.list("dwg_material_color",1,id,2)?;if colors.len()!=3{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG material requires three typed color channels"));}
 let color=|index:usize,channel:&str|->Result<DwgMaterialColor,ValueError>{let row=colors[index];if row.text(3)?!=channel{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG material color channel is out of order"));}Ok(DwgMaterialColor{factor:row.real(4)?,override_rgb:optional_integer(row,5)?})};
 let maps=r.list("dwg_material_map",1,id,2)?;if maps.len()!=6{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG material requires six typed map channels"));}
 let mut read_map=|index:usize,channel:&str|->Result<DwgMaterialMap,ValueError>{
 let row=maps[index];if row.text(3)?!=channel{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG material map channel is out of order"));}
 Ok(DwgMaterialMap{blend_factor:row.real(4)?,projection:match row.text(5)?{"inherit"=>DwgMaterialProjection::Inherit,"planar"=>DwgMaterialProjection::Planar,"box"=>DwgMaterialProjection::Box,"cylinder"=>DwgMaterialProjection::Cylinder,"sphere"=>DwgMaterialProjection::Sphere,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG material projection is unknown"))},tiling:match row.text(6)?{"inherit"=>DwgMaterialTiling::Inherit,"tile"=>DwgMaterialTiling::Tile,"crop"=>DwgMaterialTiling::Crop,"clamp"=>DwgMaterialTiling::Clamp,"mirror"=>DwgMaterialTiling::Mirror,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG material tiling is unknown"))},scale_to_entity:boolean(row,7)?,use_current_block_transform:boolean(row,8)?,transform:read_coords(r,"dwg_material_map_transform_coordinate",row.rowid)?,source:match row.text(9)?{"none"=>DwgMaterialMapSource::None,"current_scene"=>DwgMaterialMapSource::CurrentScene,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG material map source is unknown"))}})
 };
 Ok(DwgMaterial{name:row.text(1)?.into(),description:row.text(2)?.into(),ambient:color(0,"ambient")?,diffuse:color(1,"diffuse")?,specular:color(2,"specular")?,diffuse_map:read_map(0,"diffuse")?,specular_map:read_map(1,"specular")?,reflection_map:read_map(2,"reflection")?,opacity_map:read_map(3,"opacity")?,bump_map:read_map(4,"bump")?,refraction_map:read_map(5,"refraction")?,specular_gloss:row.real(3)?,opacity:row.real(4)?,refraction_index:row.real(5)?,translucence:row.real(6)?,self_illumination:row.real(7)?,reflectivity:row.real(8)?,enabled_channels:DwgMaterialChannels{diffuse:boolean(channels,1)?,specular:boolean(channels,2)?,reflection:boolean(channels,3)?,opacity:boolean(channels,4)?,bump:boolean(channels,5)?,refraction:boolean(channels,6)?}})
}
pub(super) fn project_mline(p:&mut Projection<'_,'_>,id:i64,v:&DwgMlineStyle)->Result<(),ValueError>{
 p.insert_key("dwg_mline_style",id,&[T(&v.name),T(&v.description),I(i64::from(v.fill_enabled)),I(i64::from(v.display_miters)),I(i64::from(v.start_caps.square)),I(i64::from(v.start_caps.inner_arcs)),I(i64::from(v.start_caps.round_outer_arcs)),I(i64::from(v.end_caps.square)),I(i64::from(v.end_caps.inner_arcs)),I(i64::from(v.end_caps.round_outer_arcs)),R(v.start_angle),R(v.end_angle)])?;color::project(p,"dwg_mline_fill_color",id,&v.fill_color)?;
 for(index,v)in v.elements.iter().enumerate(){let kind=match v.linetype{DwgMlineLinetype::ByLayer=>"by_layer",DwgMlineLinetype::ByBlock=>"by_block",DwgMlineLinetype::Continuous=>"continuous"};let element=p.insert("dwg_mline_style_element",&[I(id),I(ordinal(index)?),R(v.offset),T(kind)])?;color::project(p,"dwg_mline_element_color",element,&v.color)?;}Ok(())
}
pub(super) fn reconstruct_mline(r:&mut Reader<'_,'_,'_>,id:i64)->Result<DwgMlineStyle,ValueError>{
 let row=r.component("dwg_mline_style",id)?;
 let elements=r.list("dwg_mline_style_element",1,id,2)?.into_iter().map(|row|Ok(DwgMlineStyleElement{offset:row.real(3)?,color:color::reconstruct(r,"dwg_mline_element_color",row.rowid)?,linetype:match row.text(4)?{"by_layer"=>DwgMlineLinetype::ByLayer,"by_block"=>DwgMlineLinetype::ByBlock,"continuous"=>DwgMlineLinetype::Continuous,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG multiline linetype is unknown"))}})).collect::<Result<_,ValueError>>()?;
 Ok(DwgMlineStyle{name:row.text(1)?.into(),description:row.text(2)?.into(),fill_enabled:boolean(row,3)?,display_miters:boolean(row,4)?,start_caps:DwgMlineCaps{square:boolean(row,5)?,inner_arcs:boolean(row,6)?,round_outer_arcs:boolean(row,7)?},end_caps:DwgMlineCaps{square:boolean(row,8)?,inner_arcs:boolean(row,9)?,round_outer_arcs:boolean(row,10)?},fill_color:color::reconstruct(r,"dwg_mline_fill_color",id)?,start_angle:row.real(11)?,end_angle:row.real(12)?,elements})
}
