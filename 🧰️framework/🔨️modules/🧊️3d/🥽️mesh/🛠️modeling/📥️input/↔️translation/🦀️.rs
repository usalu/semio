//! ↔️ Original mesh translation clones all seven topology and metadata owners through their defining typed cursors.
use super::super::*;
use std::sync::Arc;
use protocol::value::{ValueError,ValueRefusalKind,RetirementDemand,retained_clone::{RetainedCloneSource,RetainedCloneOwner,RetainedCloneStep}};

/// 🧵️ Every partial clone, original alias, seam, attribute, material and texture remains in paid retirement custody.
#[derive(protocol::value::RetireOwned)]
pub struct MeshTranslationProduction{identity:usize,offset:[f32;3],alias:Option<Arc<HalfedgeMesh>>,source:Option<RetainedCloneSource<HalfedgeMesh,Arc<HalfedgeMesh>>>,cursor:RetainedCloneOwner<HalfedgeMesh>,mesh:Option<HalfedgeMesh>,phase:u8,ordinal:usize,atoms:usize}
impl MeshTranslationProduction{
 pub fn new(original:&Arc<HalfedgeMesh>,offset:[f32;3])->Result<Self,ValueError>{if offset.iter().any(|value|!value.is_finite()){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original mesh translation requires finite offset"))}Ok(Self{identity:Arc::as_ptr(original)as usize,offset,alias:None,source:None,cursor:Default::default(),mesh:None,phase:0,ordinal:0,atoms:0})}
 pub fn progress(&self)->f32{let detail=if self.phase==2{self.atoms as f64/(self.atoms as f64+1024.0)}else if self.phase==3{self.mesh.as_ref().map_or(0.0,|mesh|self.ordinal as f64/mesh.vertices.len().max(1)as f64)}else{0.0};((self.phase as f64+detail)/7.0)as f32}
 pub fn terminal_is_empty(&self)->bool{self.phase==7&&self.alias.is_none()&&self.source.is_none()&&self.cursor.terminal_is_empty()&&self.mesh.is_none()}
 pub fn demand(&self,body:usize)->Result<RetirementDemand,ValueError>{Ok(match self.phase{
  0=>RetirementDemand{copy_bytes:size_of::<Arc<HalfedgeMesh>>(),depth:1,..Default::default()},
  1=>RetirementDemand{capacity_bytes:RetainedCloneSource::<HalfedgeMesh>::borrowed_constructor_capacity_bytes::<Arc<HalfedgeMesh>>(),depth:1,..Default::default()},
  2=>self.cursor.advance_demands(self.source.as_ref().ok_or_else(changed)?.try_borrow()?,body)?,
  3=>RetirementDemand{copy_bytes:12,depth:1,..Default::default()},
  4=>self.cursor.selected_close_demand(body)?,
  5=>self.source.as_ref().ok_or_else(changed)?.next_close_demand(body)?,
  _=>RetirementDemand{copy_bytes:size_of::<HalfedgeMesh>(),depth:1,..Default::default()},
 })}
 pub fn step(&mut self,original:&Arc<HalfedgeMesh>,grant:RetainedCloneGrant)->Result<(Option<HalfedgeMesh>,RetainedCloneProgress),ValueError>{
  if self.identity!=Arc::as_ptr(original)as usize{return Err(changed())}if self.phase==7||grant.maximum_items==0{return Ok((None,Default::default()))}let demand=self.demand(grant.maximum_copy_bytes)?;if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok((None,Default::default()))}
  let metadata=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};let receipt=match self.phase{
   0=>{self.alias=Some(Arc::clone(original));self.phase=1;metadata},
   1=>{let alias=self.alias.take().ok_or_else(changed)?;match RetainedCloneSource::<HalfedgeMesh>::admit_borrowed(alias,|mesh|&**mesh,grant){Ok((source,receipt))=>{self.source=Some(source);self.phase=2;receipt},Err((error,alias))=>{self.alias=Some(alias);return Err(error)}}},
   2=>{let result=self.cursor.advance(self.source.as_ref().ok_or_else(changed)?.try_borrow()?,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth})?;let receipt=result.progress();self.atoms=self.atoms.saturating_add(receipt.copied_items);if matches!(result,RetainedCloneStep::Complete(_)){self.mesh=self.cursor.take();if self.mesh.is_none(){return Err(changed())}self.phase=3;}receipt},
   3=>{let mesh=self.mesh.as_mut().ok_or_else(changed)?;if let Some(vertex)=mesh.vertices.get_mut(self.ordinal){let position:[f32;3]=std::array::from_fn(|axis|vertex.position[axis]+self.offset[axis]);if position.iter().any(|value|!value.is_finite()){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original mesh translated vertex exceeds finite coordinates"))}vertex.position=position;self.ordinal+=1;}else{self.phase=4;}metadata},
   4=>{let receipt=self.cursor.close_step(grant)?.progress();if self.cursor.terminal_is_empty(){self.phase=5;}receipt},
   5=>{let source=self.source.as_mut().ok_or_else(changed)?;let receipt=source.close_step(grant)?.progress();if source.terminal_is_empty(){self.source=None;self.phase=6;}receipt},
   6=>{self.phase=7;return Ok((self.mesh.take(),metadata))},
   _=>return Err(changed()),
  };Ok((None,receipt))
 }
}
fn changed()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original mesh translation source or partial owner changed")}
