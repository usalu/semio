//! 📇️ One actual instance publishes a preborn operator catalogue for each installed registry generation.
use super::{FlowOperatorCatalogue,FlowOperatorCatalogueWork,FlowOperatorInfoMap,flow_operator_registry_source,flow_operator_registry_source_demands,flow_extension_registry_generation};
use neural_engine::SharedRegistry;
use semio_framework_job::{StepContext,OriginalTurnFailure};
use semio_framework_value::{OriginalRetirementPoll,RetainedAllocationEffect,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll,RetireOwned,RetirementDemand,ValueError,ValueRefusalKind,retirement::controlled::ControlledRetirement};
use std::mem::{ManuallyDrop,size_of};
#[derive(RetireOwned)]
struct Fields{registry:Option<(SharedRegistry,u64)>,current:Option<FlowOperatorCatalogue>,candidate:Option<FlowOperatorCatalogue>,displaced:Option<FlowOperatorCatalogue>}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original instance catalogue changed its generation or source custody")}
fn fixed(bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()}}
fn unit(bytes:usize)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}}
fn done(step:RetainedCloneStep)->OriginalRetirementPoll{RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(step),work:0})}
/// 📚️ Bootstrap maintenance and contribution publication own derivation; ordinary commands only borrow published headers.
pub struct FlowOperatorCatalogueOwner{fields:ManuallyDrop<Option<Fields>>,build:Option<FlowOperatorCatalogueWork>,retirement:Option<ControlledRetirement<FlowOperatorCatalogue>>,close:Option<ControlledRetirement<Fields>>,phase:u8,closing:bool}
impl FlowOperatorCatalogueOwner{
 pub fn new(registry:SharedRegistry,generation:u64)->Self{Self{fields:ManuallyDrop::new(Some(Fields{registry:Some((registry,generation)),current:None,candidate:None,displaced:None})),build:None,retirement:None,close:None,phase:1,closing:false}}
 pub fn generation(&self)->Option<u64>{if self.closing||self.phase!=6{return None}self.fields.as_ref()?.current.as_ref().map(|current|current.generation)}
 pub fn ready(&self)->bool{self.generation()==Some(flow_extension_registry_generation())}
 pub fn step_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{self.step_demands_within(copy,u64::MAX)}
 pub fn step_demands_within(&self,copy:usize,maximum:u64)->Result<RetirementDemand,ValueError>{
  if self.closing{return self.close_demands(copy)}let fields=self.fields.as_ref().ok_or_else(invalid)?;
  match self.phase{
   0=>Ok(flow_operator_registry_source_demands()),
   1=>Ok(fixed(size_of::<FlowOperatorCatalogueWork>()+2*size_of::<Option<(SharedRegistry,u64)>>())),
   2=>self.build.as_ref().ok_or_else(invalid)?.next_demands(copy,maximum),
   3=>{let build=self.build.as_ref().ok_or_else(invalid)?;if build.terminal_is_empty(){Ok(fixed(size_of::<usize>()))}else{build.next_close_demands(copy)}},
   4=>match self.retirement.as_ref(){Some(owner)=>owner.selected_demand(copy),None if fields.displaced.is_some()=>Ok(fixed(size_of::<ControlledRetirement<FlowOperatorCatalogue>>())),None=>Ok(fixed(1))},
   5=>Ok(fixed(2*size_of::<Option<FlowOperatorCatalogue>>())),
   6=>Ok(if self.ready(){Default::default()}else{fixed(2*size_of::<Option<FlowOperatorCatalogue>>() +size_of::<u8>())}),
   _=>Err(invalid())
  }
 }
 fn paid(cx:&mut StepContext<'_>,d:RetirementDemand)->Result<bool,OriginalTurnFailure>{if !cx.retained_work_can_enter(d){return Ok(false)}cx.consume_fuel(1);cx.consume_retained(unit(d.copy_bytes))?;Ok(true)}
 fn closed(cx:&mut StepContext<'_>,d:RetirementDemand,run:impl FnOnce(usize,&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll)->Result<bool,OriginalTurnFailure>{
  let grant=cx.retained_grant();if cx.should_yield()||!semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(false)}
  let Some((poll,_))=cx.with_original_effect(grant,|_,effect|run(grant.maximum_copy_bytes,effect))?else{return Ok(false)};
  match poll{RetainedWorkPoll::Pending{..}=>Ok(false),RetainedWorkPoll::Complete(turn)=>{let step=turn.result.map_err(|failure|OriginalTurnFailure::debit_original(failure.into_value_error(),cx))?;cx.consume_retained(step.progress())?;Ok(true)}}
 }
 pub fn step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{for _ in 0..512{let before=(self.phase,cx.retained_progress());if self.once(cx)?{return Ok(true)}if cx.should_yield()||cx.is_cancelled()||before==(self.phase,cx.retained_progress()){break}}Ok(false)}
 fn once(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.closing{return Err(invalid().into())}if cx.should_yield()||cx.is_cancelled(){return Ok(false)}if self.ready(){return Ok(true)}let d=self.step_demands_within(cx.retained_grant().maximum_copy_bytes,cx.fuel_remaining())?;if !cx.retained_work_can_enter(d){return Ok(false)}
  match self.phase{
   0=>{let grant=cx.retained_grant();let Some((turn,_))=cx.with_original_effect(grant,|_,effect|flow_operator_registry_source(effect))?else{return Ok(false)};match turn{Ok(advance)=>{let fields=self.fields.as_mut().ok_or_else(invalid)?;fields.registry=advance.source;if fields.registry.is_some(){self.phase=1}cx.consume_retained(advance.progress)?},Err(error)=>return Err(OriginalTurnFailure::debit_original(error,cx))}},
   1=>if Self::paid(cx,d)?{let(registry,generation)=self.fields.as_mut().ok_or_else(invalid)?.registry.take().ok_or_else(invalid)?;self.build=Some(FlowOperatorCatalogueWork::new(registry,generation));self.phase=2},
   2=>{let grant=cx.retained_grant();let build=self.build.as_mut().ok_or_else(invalid)?;let Some((turn,_))=cx.with_original_effect(grant,|_,effect|build.advance(grant.maximum_copy_bytes,effect))?else{return Ok(false)};match turn{Ok(output)=>{if let Some(output)=output{self.fields.as_mut().ok_or_else(invalid)?.candidate=Some(output);build.begin_close();self.phase=3}cx.consume_retained(build.step_progress())?},Err(failure)=>return Err(OriginalTurnFailure::debit_original(failure.into_value_error(),cx))}},
   3=>{let build=self.build.as_mut().ok_or_else(invalid)?;if build.terminal_is_empty(){if Self::paid(cx,d)?{self.build=None;self.phase=4}}else{Self::closed(cx,d,|body,effect|build.close_step(body,effect))?;}},
   4=>{if let Some(owner)=self.retirement.as_mut(){if Self::closed(cx,d,|body,effect|owner.step(body,effect))?&&owner.terminal_is_empty(){self.retirement=None}}else if self.fields.as_ref().ok_or_else(invalid)?.displaced.is_some(){if Self::paid(cx,d)?{let displaced=self.fields.as_mut().unwrap().displaced.take().unwrap();self.retirement=Some(ControlledRetirement::new(displaced).unwrap_or_else(|_|unreachable!("catalogue declares controlled retirement")))}}else if Self::paid(cx,d)?{self.phase=5}},
   5=>if Self::paid(cx,d)?{let fields=self.fields.as_mut().ok_or_else(invalid)?;if fields.current.is_some(){return Err(invalid().into())}fields.current=fields.candidate.take();self.phase=6},
   6=>if Self::paid(cx,d)?{let fields=self.fields.as_mut().ok_or_else(invalid)?;if fields.displaced.is_some(){return Err(invalid().into())}fields.displaced=fields.current.take();self.phase=0},
   _=>return Err(invalid().into())
  }Ok(self.ready())
 }
 /// 📏️ Only the same published registry generation can quote its already derived map alias.
 pub fn retain_infos_demands(&self,registry:&SharedRegistry,generation:u64)->Result<Option<RetirementDemand>,ValueError>{
  if self.closing{return Err(invalid())}
  Ok(self.fields.as_ref().and_then(|fields|fields.current.as_ref()).filter(|current|self.phase==6&&current.generation==generation&&current.registry.owner_identity()==registry.owner_identity()).map(|current|current.infos.alias_demands()))
 }
 /// 🫴️ This ordinary command seam only retains an already published map matching its actual registry lease.
 pub fn retain_infos(&self,registry:&SharedRegistry,generation:u64,output:&mut Option<FlowOperatorInfoMap>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.closing||output.is_some(){return Err(invalid().into())}let Some(current)=self.fields.as_ref().and_then(|fields|fields.current.as_ref()).filter(|current|self.phase==6&&current.generation==generation&&current.registry.owner_identity()==registry.owner_identity())else{return Ok(false)};if cx.should_yield()||cx.is_cancelled()||!cx.retained_work_can_enter(current.infos.alias_demands()){return Ok(false)}
  let grant=cx.retained_grant();let Some((turn,_))=cx.with_original_effect(grant,|_,effect|current.infos.retain_original(effect))?else{return Ok(false)};match turn.result{Ok(Some((infos,p)))=>{*output=Some(infos);cx.consume_retained(p)?;Ok(true)},Ok(None)=>Ok(false),Err(error)=>Err(OriginalTurnFailure::debit_original(error,cx))}
 }
 /// ♻️ Pays one admitted close step from the caller's turn; denied turns charge nothing and keep every cursor.
 pub fn close(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{let d=self.close_demands(cx.retained_grant().maximum_copy_bytes)?;Self::closed(cx,d,|body,effect|self.close_step(body,effect))}
 pub fn begin_close(&mut self){self.closing=true;if let Some(build)=self.build.as_mut(){build.begin_close()}}
 pub fn close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if let Some(build)=self.build.as_ref(){return if build.terminal_is_empty(){Ok(fixed(size_of::<usize>()))}else{build.next_close_demands(copy)}}
  if let Some(owner)=self.retirement.as_ref(){return owner.selected_demand(copy)}
  if let Some(owner)=self.close.as_ref(){return owner.selected_demand(copy)}
  Ok(fixed(size_of::<ControlledRetirement<Fields>>()))
 }
 pub fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  if !self.closing{return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0})}
  if let Some(build)=self.build.as_mut(){if !build.terminal_is_empty(){return build.close_step(body,effect)}if !effect.performed(1).unwrap_or(false){return done(RetainedCloneStep::Progress(Default::default()))}self.build=None;return done(RetainedCloneStep::Progress(unit(size_of::<usize>())))}
  if let Some(owner)=self.retirement.as_mut(){let poll=owner.step(body,effect);if owner.terminal_is_empty(){self.retirement=None}return poll}
  if let Some(owner)=self.close.as_mut(){let poll=owner.step(body,effect);if owner.terminal_is_empty(){self.close=None}return poll}
  if let Some(fields)=self.fields.take(){self.close=Some(ControlledRetirement::new(fields).unwrap_or_else(|_|unreachable!("catalogue owner fields declare controlled retirement")));return done(RetainedCloneStep::Progress(unit(0)))}
  done(RetainedCloneStep::Complete(Default::default()))
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.build.is_none()&&self.fields.is_none()&&self.retirement.is_none()&&self.close.is_none()}
}
impl Drop for FlowOperatorCatalogueOwner{fn drop(&mut self){assert!(self.terminal_is_empty(),"instance catalogue must close actual derivation and every published generation");unsafe{ManuallyDrop::drop(&mut self.fields)}}}
