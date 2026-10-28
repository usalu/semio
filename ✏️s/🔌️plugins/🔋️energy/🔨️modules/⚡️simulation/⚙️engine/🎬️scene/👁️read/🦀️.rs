//! 👁️ Scene rows borrow domain geometry from either numerical owners or immutable document pages.
use crate::model::{EntityId,Model,SurfaceClass,Fenestration};
/// 🧱️ A rendered face borrows its actual original name and vertex array.
#[derive(Clone,Copy)]
pub struct EnergySceneSurface<'a>{pub id:EntityId,pub zone_id:EntityId,pub name:&'a str,pub class:SurfaceClass,pub vertices_m:&'a[[f64;3]]}
/// 🪟️ Window placement borrows the original polygon and reads only its authored scalar dimensions.
#[derive(Clone,Copy)]
pub struct EnergySceneWindow<'a>{pub id:EntityId,pub surface_id:EntityId,pub name:&'a str,pub area_m2:f64,pub height_m:f64,pub sill_height_m:f64,pub vertices_m:&'a[[f64;3]]}
/// 🌳️ Shading renders from the original geometry slice.
#[derive(Clone,Copy)]
pub struct EnergySceneShading<'a>{pub id:EntityId,pub name:&'a str,pub vertices_m:&'a[[f64;3]]}
/// 🏠️ Zone hulls retain document identity and labels while borrowing member faces.
#[derive(Clone,Copy)]
pub struct EnergySceneZone<'a>{pub id:EntityId,pub name:&'a str}
/// 🖼️ A model source supplies borrowed rows without constructing an intermediate numerical model.
pub trait EnergySceneModel{
 fn name(&self)->&str;
 fn surfaces(&self)->impl Iterator<Item=EnergySceneSurface<'_>>;
 fn fenestrations(&self)->impl Iterator<Item=EnergySceneWindow<'_>>;
 fn shading_surfaces(&self)->impl Iterator<Item=EnergySceneShading<'_>>;
 fn zones(&self)->impl Iterator<Item=EnergySceneZone<'_>>;
}
impl<'a> From<&'a Fenestration> for EnergySceneWindow<'a>{fn from(row:&'a Fenestration)->Self{Self{id:row.id,surface_id:row.surface_id,name:&row.name,area_m2:row.area_m2,height_m:row.height_m,sill_height_m:row.sill_height_m,vertices_m:&row.vertices_m}}}
impl EnergySceneModel for Model{
 fn name(&self)->&str{&self.name}
 fn surfaces(&self)->impl Iterator<Item=EnergySceneSurface<'_>>{self.surfaces.iter().map(|row|EnergySceneSurface{id:row.id,zone_id:row.zone_id,name:&row.name,class:row.class,vertices_m:&row.vertices_m})}
 fn fenestrations(&self)->impl Iterator<Item=EnergySceneWindow<'_>>{self.fenestrations.iter().map(Into::into)}
 fn shading_surfaces(&self)->impl Iterator<Item=EnergySceneShading<'_>>{self.shading_surfaces.iter().map(|row|EnergySceneShading{id:row.id,name:&row.name,vertices_m:&row.vertices_m})}
 fn zones(&self)->impl Iterator<Item=EnergySceneZone<'_>>{self.zones.iter().map(|row|EnergySceneZone{id:row.id,name:&row.name})}
}
