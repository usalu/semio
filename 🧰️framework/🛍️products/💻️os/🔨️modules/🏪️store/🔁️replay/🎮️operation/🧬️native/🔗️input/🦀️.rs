//! 🔗️ Decoded and pressed native operations keep the original owner and captured retirement issuer.
use super::*;
use semio_framework_value_factory::owned::FactoryOwnedSourceAuthority;
/// 🧳️ Retains the same operation through source birth, projection handoff, and exact issuer closure.
pub struct ArtifactMutationSource<M:Send+Sync+'static>{
 authority:ManuallyDrop<Option<FactoryOwnedSourceAuthority<M>>>,
 admission:ManuallyDrop<Option<RetainedCloneSourceAdmission<M,FactoryOwnedSourceAuthority<M>>>>,
 source:ManuallyDrop<Option<RetainedCloneSource<M,FactoryOwnedSourceAuthority<M>>>>,
 projection:ManuallyDrop<Option<RetainedOwnedProjection<M>>>,
 retirement:RetainedCloneClose,
 closing:bool,
}
fn project_authority<M:Send+Sync+'static>(authority:&FactoryOwnedSourceAuthority<M>)->&M{authority.get()}
type Effect<'a,'b>=&'a mut semio_framework_value::RetainedAllocationEffect<'b>;
fn one(effect:Effect<'_,'_>)->Result<(),ValueError>{if effect.performed(1)?{Ok(())}else{Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original mutation source action exceeds its work ceiling"))}}
fn settled(poll:semio_framework_value::OriginalRetirementPoll)->Result<RetainedCloneStep,ValueError>{match poll{semio_framework_value::RetainedWorkPoll::Pending{..}=>Ok(RetainedCloneStep::Progress(Default::default())),semio_framework_value::RetainedWorkPoll::Complete(turn)=>turn.result.map_err(semio_framework_value::OriginalRetirementFailure::into_value_error)}}
fn charged<T>(effect:Effect<'_,'_>,turn:semio_framework_value::RetainedWorkOutcome<T>)->Result<T,ValueError>{let owed=turn.work.saturating_sub(effect.performed_work());if owed>0&&!effect.performed(u64::try_from(owed).unwrap_or(u64::MAX))?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original mutation source work exceeds its ceiling"))}turn.result}
impl<M:Send+Sync+'static> ArtifactMutationSource<M>{
 pub fn constructor_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<M>()+size_of::<Arc<dyn ArtifactOwnedValueRetirementFactory<M>>>()+size_of::<Self>(),depth:1,..Default::default()}}
 pub fn admit(original:&mut Option<M>,factory:&Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,grant:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{
  let d=Self::constructor_demand();if !retained_clone_grant_admits(grant,d){return Ok(None);}let original=original.take().ok_or_else(||invalid("original mutation source owner absent"))?;
  Ok(Some((Self{authority:ManuallyDrop::new(None),admission:ManuallyDrop::new(Some(RetainedCloneSource::admission_borrowed(FactoryOwnedSourceAuthority::new(original,Arc::clone(factory)),project_authority::<M>))),source:ManuallyDrop::new(None),projection:ManuallyDrop::new(None),retirement:Default::default(),closing:false},RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))
 }
 pub fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing{return self.close_demand(body);}if let Some(admission)=self.admission.as_ref(){return admission.next_demand();}
  if self.projection.is_none(){return Ok(RetainedOwnedProjection::<M>::constructor_demand());}Ok(header::<Option<RetainedOwnedProjection<M>>>())
 }
 pub fn advance(&mut self,effect:Effect<'_,'_>)->Result<RetainedCloneStep,ValueError>{
  let grant=effect.grant();let d=self.demands(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()));}if self.closing{return self.close_step(effect);}
  if self.admission.is_some(){return self.admit_source(effect).map(RetainedCloneStep::Progress);}
  if self.projection.is_none(){let(original,p)=self.source.as_ref().ok_or_else(||invalid("original mutation source absent"))?.retain_original(effect).result?;*self.projection=Some(original);return Ok(RetainedCloneStep::Complete(p));}
  Ok(RetainedCloneStep::Complete(Default::default()))
 }
 fn admit_source(&mut self,effect:Effect<'_,'_>)->Result<RetainedCloneProgress,ValueError>{let grant=effect.grant();let admission=self.admission.as_mut().ok_or_else(||invalid("original mutation source admission absent"))?;if admission.phase()!="ready"{return Ok(admission.advance(effect).result?.progress());}let mut output=None;let step=charged(effect,admission.take_output(&mut output,grant))?;if let Some(source)=output{*self.source=Some(source);self.admission.take();}Ok(step.progress())}
 fn close_admission(&mut self,effect:Effect<'_,'_>)->Result<RetainedCloneProgress,ValueError>{if self.admission.as_ref().is_some_and(|admission|admission.phase()=="ready"){return self.admit_source(effect);}let admission=self.admission.as_mut().ok_or_else(||invalid("original mutation source admission absent"))?;let mut inputs=None;let step=admission.return_inputs(&mut inputs,effect).result?;if let Some((authority,()))=inputs{*self.authority=Some(authority);self.admission.take();}Ok(step.progress())}
 pub fn take(&mut self,output:&mut Option<RetainedOwnedProjection<M>>,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{if output.is_some(){return Err(invalid("original mutation projection output already occupied"));}if self.closing||self.projection.is_none(){return Ok(None);}let d=header::<Option<RetainedOwnedProjection<M>>>();if !retained_clone_grant_admits(grant,d){return Ok(None);}*output=self.projection.take();Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
 pub fn ready(&self)->bool{!self.closing&&self.projection.is_some()}
 pub fn begin_close(&mut self){self.closing=true;}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.retirement.is_empty(){return Ok(RetirementDemand{copy_bytes:self.retirement.next_copy_byte_demand()?,capacity_bytes:self.retirement.next_capacity_byte_demand(body)?,release_bytes:self.retirement.next_release_byte_demand()?,depth:self.retirement.next_depth_demand()?});}
  if let Some(admission)=self.admission.as_ref(){return if admission.phase()=="ready"{admission.next_demand()}else{admission.next_return_demand()};}
  if let Some(projection)=self.projection.as_ref(){return if projection.terminal_is_empty(){Ok(header::<Option<RetainedOwnedProjection<M>>>())}else{projection.close_demand(body)};}
  if let Some(source)=self.source.as_ref(){return if source.terminal_is_empty(){Ok(header::<Option<RetainedCloneSource<M,FactoryOwnedSourceAuthority<M>>>>())}else{source.next_close_demand(body)};}
  self.retirement.next_owner_demands::<FactoryOwnedSourceAuthority<M>>(self.authority.is_some(),&None,body)
 }
 pub fn close_step(&mut self,effect:Effect<'_,'_>)->Result<RetainedCloneStep,ValueError>{
  let grant=effect.grant();if !self.closing{return Ok(RetainedCloneStep::Progress(Default::default()));}let d=self.close_demand(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if !self.retirement.is_empty(){return settled(self.retirement.step_granted(effect));}
  if self.admission.is_some(){return self.close_admission(effect).map(RetainedCloneStep::Progress);}
  if let Some(projection)=self.projection.as_mut(){if !projection.terminal_is_empty(){return settled(projection.close_step(effect));}self.projection.take();one(effect)?;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}));}
  if let Some(source)=self.source.as_mut(){if !source.terminal_is_empty(){return settled(source.close_step(effect));}self.source.take();one(effect)?;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}));}
  if let Some(step)=self.retirement.begin_granted(&mut self.authority,effect).result?{return Ok(step);}Ok(RetainedCloneStep::Complete(Default::default()))
 }
}
impl<M:Send+Sync+'static> ArtifactMutationSource<M>{pub fn terminal_is_empty(&self)->bool{self.closing&&self.authority.is_none()&&self.admission.is_none()&&self.source.is_none()&&self.projection.is_none()&&self.retirement.is_empty()}}
impl<M:Send+Sync+'static> Drop for ArtifactMutationSource<M>{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"original mutation source requires funded closure");}}
