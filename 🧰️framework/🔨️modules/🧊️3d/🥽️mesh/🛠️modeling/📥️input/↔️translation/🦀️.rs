//! ↔️ Original mesh translation clones all seven topology and metadata owners through their defining typed cursors.
use super::super::*;
use std::ptr::NonNull;
use std::mem::size_of;
use semio_framework_value::{OriginalRetirementFailure,RetirementDemand,ValueError,ValueRefusalKind,retirement::arc_custody::ArcLease,retained_clone::{OriginalEffectReply,OriginalEffectReplyComplete,OriginalProjectionReply,OriginalSourceProjection,RetainedAllocationEffect,RetainedCloneOwner,RetainedCloneStep,retained_clone_grant_admits}};

use semio_framework_value_source_hold::SourceHold;

type Reply<'r>=OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>;
type Complete<'r>=OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>;

const ADMIT:u8=0;const CLONE:u8=1;const TAKE:u8=2;const TRANSLATE:u8=3;const PUBLISH:u8=4;const DONE:u8=5;

/// 🪪️ The actual shared original mesh as one Source authority whose projection is the mesh itself.
#[derive(semio_framework_value::RetireOwned)]
pub struct MeshOriginal(ArcLease<HalfedgeMesh>);
impl MeshOriginal{fn mesh(&self)->Result<&HalfedgeMesh,ValueError>{self.0.0.as_deref().ok_or_else(||fault("original mesh lease is empty"))}}
impl OriginalSourceProjection<HalfedgeMesh> for MeshOriginal{
 fn projection_work_demand(&self)->Result<u64,ValueError>{Ok(1)}
 fn project_source<'reply,'source>(&'source self,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalProjectionReply<'reply,'source,HalfedgeMesh>)->Result<OriginalEffectReplyComplete<'reply,NonNull<HalfedgeMesh>,ValueError>,ValueError>{
  if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original mesh projection exceeded its original work"))))}
  Ok(reply.complete(self.mesh()))
 }
 fn recover_source_projection<'reply,'source>(&'source self,_:&mut RetainedAllocationEffect<'_>,reply:OriginalProjectionReply<'reply,'source,HalfedgeMesh>)->Result<OriginalEffectReplyComplete<'reply,NonNull<HalfedgeMesh>,ValueError>,ValueError>{Ok(reply.pending())}
}

/// 🧵️ Every partial clone, original alias and translated vertex remains in paid retirement custody; the finished mesh leaves only through `take_output`.
pub struct MeshTranslationProduction{hold:SourceHold<HalfedgeMesh,MeshOriginal>,owner:RetainedCloneOwner<HalfedgeMesh>,offset:[f32;3],mesh:Option<HalfedgeMesh>,output:Option<HalfedgeMesh>,ordinal:usize,atoms:usize,phase:u8,route:Option<u8>,cancelled:bool}
semio_framework_value::artifact_retire_struct!(MeshTranslationProduction{hold,owner,offset,mesh,output,ordinal,atoms,phase,route,cancelled});
impl MeshTranslationProduction{
 pub fn new(original:&std::sync::Arc<HalfedgeMesh>,offset:[f32;3])->Result<Self,ValueError>{
  if offset.iter().any(|value|!value.is_finite()){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original mesh translation requires finite offset"))}
  Ok(Self{hold:SourceHold::new(MeshOriginal(ArcLease::new(std::sync::Arc::clone(original)))),owner:Default::default(),offset,mesh:None,output:None,ordinal:0,atoms:0,phase:ADMIT,route:None,cancelled:false})
 }
 pub fn cancel(&mut self){self.cancelled=true}
 pub fn complete(&self)->bool{self.phase_now()==DONE&&!self.cancelled}
 pub fn progress(&self)->f32{let phase=self.phase_now();let detail=if phase==CLONE{self.atoms as f64/(self.atoms as f64+1024.0)}else if phase==TRANSLATE{self.mesh.as_ref().map_or(0.0,|mesh|self.ordinal as f64/mesh.vertices.len().max(1)as f64)}else{0.0};((phase as f64+detail)/5.0)as f32}
 fn phase_now(&self)->u8{match self.phase{
  ADMIT if self.hold.ready()=>CLONE,
  CLONE if self.owner.output_ready()=>TAKE,
  TAKE if self.mesh.is_some()=>TRANSLATE,
  phase=>phase}}
 pub fn demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  let phase=self.phase_now();if self.cancelled||phase>=DONE{return Ok(Default::default())}
  Ok(match phase{
   ADMIT=>self.hold.quote(body)?,
   CLONE=>self.owner.advance_demands(self.hold.original()?,body,u64::MAX)?,
   TAKE=>self.owner.take_demand()?,
   TRANSLATE=>RetirementDemand{copy_bytes:size_of::<[f32;3]>(),depth:1,..Default::default()},
   _=>RetirementDemand{copy_bytes:size_of::<HalfedgeMesh>(),depth:1,..Default::default()},
  })
 }
 pub fn advance_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{self.drive(body,effect,reply,false)}
 pub fn recover_advance_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{self.drive(body,effect,reply,true)}
 fn drive<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Complete<'r>,ValueError>{
  if self.route.is_some()!=recover{return Err(fault("original mesh translation changed its recovery route"))}
  self.phase=self.phase_now();
  let demand=if recover{Default::default()}else{
   let grant=effect.grant();if self.cancelled||self.phase>=DONE||grant.maximum_items==0{return denied(reply)}
   let demand=self.demand(body)?;if !retained_clone_grant_admits(grant,demand){return denied(reply)}demand
  };
  let state=self;let phase=state.phase;
  match phase{
   ADMIT=>{let mut scope=reply.scope(&mut state.route,phase)?;drop(state.hold.step_into(body,effect,scope.borrow()?,recover)?);Ok(scope.forwarded())},
   CLONE=>{let source=state.hold.original()?;let mut scope=reply.scope(&mut state.route,phase)?;drop(if recover{state.owner.recover_advance_into(source,body,u64::MAX,effect,scope.borrow()?)?}else{state.owner.advance_into(source,body,u64::MAX,effect,scope.borrow()?)?});state.atoms=state.atoms.saturating_add(1);Ok(scope.forwarded())},
   TAKE=>{
    if recover{return Err(fault("original mesh translation take has no unrecorded callback"))}
    let mut inner=None;drop(state.owner.take(&mut state.mesh,effect,OriginalEffectReply::borrow(&mut inner)?)?);
    match inner.take(){Some(result)=>Ok(reply.progress().complete(result.map_err(OriginalRetirementFailure::from))),None=>Ok(reply.pending())}
   },
   TRANSLATE=>local_step(effect,demand,1,reply,|_|{
    let offset=state.offset;let mesh=state.mesh.as_mut().ok_or_else(||fault("original mesh translation lost its clone"))?;
    if let Some(vertex)=mesh.vertices.get_mut(state.ordinal){
     let position:[f32;3]=std::array::from_fn(|axis|vertex.position[axis]+offset[axis]);
     if position.iter().any(|value|!value.is_finite()){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original mesh translated vertex exceeds finite coordinates"))}
     vertex.position=position;state.ordinal+=1;
    }else{state.phase=PUBLISH}
    Ok(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})
   }),
   PUBLISH=>local_step(effect,demand,1,reply,|_|{state.output=state.mesh.take();state.phase=DONE;Ok(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})}),
   _=>Err(fault("original mesh translation phase is invalid")),
  }
 }
 /// 🪜️ Runs one funded turn on the caller's effect; an unresolved callback is recovered by the next turn, never replayed.
 pub fn step_turn(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<RetainedCloneStep>,ValueError>{
  let mut slot=None;let reply=OriginalEffectReply::borrow(&mut slot)?;
  drop(if self.route.is_some(){self.recover_advance_into(body,effect,reply)?}else{self.advance_into(body,effect,reply)?});
  slot.map(|result|result.map_err(OriginalRetirementFailure::into_value_error)).transpose()
 }
 /// 📤️ Hands over the finished translated mesh once every funded phase has completed.
 pub fn take_output(&mut self)->Option<HalfedgeMesh>{if self.complete(){self.output.take()}else{None}}
}
fn fault(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
fn denied<'r>(reply:Reply<'r>)->Result<Complete<'r>,ValueError>{Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
fn local_step<'r>(effect:&mut RetainedAllocationEffect<'_>,demand:RetirementDemand,work:u64,reply:Reply<'r>,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneProgress,ValueError>)->Result<Complete<'r>,ValueError>{
 if !retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?<work{return denied(reply)}
 if !effect.performed(work)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original mesh translation transition exceeded its original work").into())))}
 Ok(reply.complete(run(effect).map(RetainedCloneStep::Progress).map_err(Into::into)))
}
