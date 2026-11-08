//! ↔️ Original mesh translation clones all seven topology and metadata owners through their defining typed cursors.
use super::super::*;
use std::sync::Arc;
use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand,retained_clone::{RetainedCloneSource,RetainedCloneOwner,RetainedCloneStep,StagedSourceAdmission}};

/// 🧵️ Every partial clone, original alias, seam, attribute, material and texture remains in paid retirement custody.
#[derive(semio_framework_value::RetireOwned)]
#[value(send_only)]
pub struct MeshTranslationProduction{identity:usize,offset:[f32;3],admission:Option<StagedSourceAdmission<HalfedgeMesh,Arc<HalfedgeMesh>>>,work:u64,source:Option<RetainedCloneSource<HalfedgeMesh,Arc<HalfedgeMesh>>>,cursor:RetainedCloneOwner<HalfedgeMesh>,mesh:Option<HalfedgeMesh>,phase:u8,ordinal:usize,atoms:usize}
impl MeshTranslationProduction{
 pub fn new(original:&Arc<HalfedgeMesh>,offset:[f32;3])->Result<Self,ValueError>{if offset.iter().any(|value|!value.is_finite()){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original mesh translation requires finite offset"))}Ok(Self{identity:Arc::as_ptr(original)as usize,offset,admission:None,work:0,source:None,cursor:Default::default(),mesh:None,phase:0,ordinal:0,atoms:0})}
 pub fn last_turn_work(&self)->u64{self.work}
 pub fn progress(&self)->f32{let detail=if self.phase==2{self.atoms as f64/(self.atoms as f64+1024.0)}else if self.phase==3{self.mesh.as_ref().map_or(0.0,|mesh|self.ordinal as f64/mesh.vertices.len().max(1)as f64)}else{0.0};((self.phase as f64+detail)/7.0)as f32}
 pub fn terminal_is_empty(&self)->bool{self.phase==7&&self.admission.is_none()&&self.source.is_none()&&self.cursor.terminal_is_empty()&&self.mesh.is_none()}
 pub fn demand(&self,body:usize)->Result<RetirementDemand,ValueError>{Ok(match self.phase{
  0=>RetirementDemand{copy_bytes:size_of::<Arc<HalfedgeMesh>>(),depth:1,..Default::default()},
  1=>self.admission.as_ref().ok_or_else(changed)?.next_demand()?,
  2=>self.cursor.advance_demands(self.source.as_ref().ok_or_else(changed)?.try_borrow()?,body,u64::MAX)?,
  3=>RetirementDemand{copy_bytes:12,depth:1,..Default::default()},
  4=>self.cursor.selected_close_demand(body)?,
  5=>self.source.as_ref().ok_or_else(changed)?.next_close_demand(body)?,
  _=>RetirementDemand{copy_bytes:size_of::<HalfedgeMesh>(),depth:1,..Default::default()},
 })}
 pub fn step(&mut self,original:&Arc<HalfedgeMesh>,body:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<(Option<HalfedgeMesh>,RetainedCloneProgress),ValueError>{
   let before=effect.performed_work();let result=self.step_in(original,body,effect);self.work=self.work.saturating_add(u64::try_from(effect.performed_work().saturating_sub(before)).unwrap_or(u64::MAX));result
  }
  fn step_in(&mut self,original:&Arc<HalfedgeMesh>,body:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<(Option<HalfedgeMesh>,RetainedCloneProgress),ValueError>{
   let grant=effect.grant();
  if self.identity!=Arc::as_ptr(original)as usize{return Err(changed())}if self.phase==7||grant.maximum_items==0{return Ok((None,Default::default()))}let demand=self.demand(body)?;if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok((None,Default::default()))}
  let metadata=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};let receipt=match self.phase{
   0=>{self.admission=Some(StagedSourceAdmission::borrowed(Arc::clone(original),|mesh|&**mesh));self.phase=1;metadata},
   1=>{let admission=self.admission.as_mut().ok_or_else(changed)?;let turn=admission.advance(&mut self.source,effect);let progress=turn.result?.progress();if admission.terminal_is_empty(){self.admission=None;}if self.source.is_some(){self.phase=2;}progress},
   2=>{let maximum=effect.remaining_work_units()?;let turn=self.cursor.advance(self.source.as_ref().ok_or_else(changed)?.try_borrow()?,body,maximum,&mut effect.nested(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}));let result=turn.result.map_err(semio_framework_value::OriginalRetirementFailure::into_value_error)?;let receipt=result.progress();self.atoms=self.atoms.saturating_add(receipt.copied_items);if matches!(result,RetainedCloneStep::Complete(_)){let mut output=None;let mut reply=None;let proof=self.cursor.take(&mut output,effect,semio_framework_value::retained_clone::OriginalEffectReply::borrow(&mut reply)?)?;drop(proof);match reply{Some(Ok(_))=>{},Some(Err(error))=>return Err(error),None=>return Ok((None,receipt))}self.mesh=output;if self.mesh.is_none(){return Err(changed())}self.phase=3;}receipt},
   3=>{let mesh=self.mesh.as_mut().ok_or_else(changed)?;if let Some(vertex)=mesh.vertices.get_mut(self.ordinal){let position:[f32;3]=std::array::from_fn(|axis|vertex.position[axis]+self.offset[axis]);if position.iter().any(|value|!value.is_finite()){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original mesh translated vertex exceeds finite coordinates"))}vertex.position=position;self.ordinal+=1;}else{self.phase=4;}metadata},
   4=>{let turn=self.cursor.close_step(effect);let receipt=match turn{semio_framework_value::RetainedWorkPoll::Complete(turn)=>turn.result.map_err(semio_framework_value::OriginalRetirementFailure::into_value_error)?.progress(),semio_framework_value::RetainedWorkPoll::Pending{..}=>Default::default()};if self.cursor.terminal_is_empty(){self.phase=5;}receipt},
   5=>{let source=self.source.as_mut().ok_or_else(changed)?;let turn=source.close_step(effect);let receipt=match turn{semio_framework_value::RetainedWorkPoll::Complete(turn)=>turn.result.map_err(semio_framework_value::OriginalRetirementFailure::into_value_error)?.progress(),semio_framework_value::RetainedWorkPoll::Pending{..}=>Default::default()};if source.terminal_is_empty(){self.source=None;self.phase=6;}receipt},
   6=>{self.phase=7;return Ok((self.mesh.take(),metadata))},
   _=>return Err(changed()),
  };Ok((None,receipt))
 }
}
fn changed()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original mesh translation source or partial owner changed")}
