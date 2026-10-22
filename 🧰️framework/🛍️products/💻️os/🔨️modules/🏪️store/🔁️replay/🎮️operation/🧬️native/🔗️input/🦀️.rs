//! 🔗️ Decoded and pressed native operations keep the original owner and captured retirement issuer.
use super::*;
use semio_framework_value_factory::owned::FactoryOwnedSourceAuthority;
/// 🧳️ Retains the same operation through source birth, projection handoff, and exact issuer closure.
pub struct ArtifactMutationSource<M:Send+Sync+'static>{
 authority:ManuallyDrop<Option<FactoryOwnedSourceAuthority<M>>>,
 source:ManuallyDrop<Option<RetainedCloneSource<M,FactoryOwnedSourceAuthority<M>>>>,
 projection:ManuallyDrop<Option<RetainedOwnedProjection<M>>>,
 retirement:RetainedCloneClose,
 closing:bool,
}
impl<M:Send+Sync+'static> ArtifactMutationSource<M>{
 pub fn constructor_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<M>()+size_of::<Arc<dyn ArtifactOwnedValueRetirementFactory<M>>>()+size_of::<Self>(),depth:1,..Default::default()}}
 pub fn admit(original:&mut Option<M>,factory:&Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,grant:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{
  let d=Self::constructor_demand();if !retained_clone_grant_admits(grant,d){return Ok(None);}let original=original.take().ok_or_else(||invalid("original mutation source owner absent"))?;
  Ok(Some((Self{authority:ManuallyDrop::new(Some(FactoryOwnedSourceAuthority::new(original,Arc::clone(factory)))),source:ManuallyDrop::new(None),projection:ManuallyDrop::new(None),retirement:Default::default(),closing:false},RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))
 }
 pub fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing{return self.close_demand(body);}if self.authority.is_some(){let birth=RetainedCloneSource::<M,FactoryOwnedSourceAuthority<M>>::borrowed_constructor_demand::<FactoryOwnedSourceAuthority<M>>();return Ok(RetirementDemand{copy_bytes:RetainedCloneSource::<M,FactoryOwnedSourceAuthority<M>>::constructor_copy_bytes(),capacity_bytes:birth.capacity_bytes,depth:birth.depth,..Default::default()});}
  if self.projection.is_none(){return Ok(RetainedOwnedProjection::<M>::constructor_demand());}Ok(header::<Option<RetainedOwnedProjection<M>>>())
 }
 pub fn advance(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let d=self.demands(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()));}if self.closing{return self.close_step(grant);}
  if let Some(authority)=self.authority.take(){return match RetainedCloneSource::<M,FactoryOwnedSourceAuthority<M>>::admit_borrowed(authority,|owner:&FactoryOwnedSourceAuthority<M>|owner.get(),grant){Ok((source,p))=>{*self.source=Some(source);Ok(RetainedCloneStep::Progress(p))},Err((error,authority))=>{*self.authority=Some(authority);Err(error)}};}
  if self.projection.is_none(){let(original,p)=self.source.as_ref().ok_or_else(||invalid("original mutation source absent"))?.retain_original(grant)?;*self.projection=Some(original);return Ok(RetainedCloneStep::Complete(p));}
  Ok(RetainedCloneStep::Complete(Default::default()))
 }
 pub fn take(&mut self,output:&mut Option<RetainedOwnedProjection<M>>,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{if output.is_some(){return Err(invalid("original mutation projection output already occupied"));}if self.closing||self.projection.is_none(){return Ok(None);}let d=header::<Option<RetainedOwnedProjection<M>>>();if !retained_clone_grant_admits(grant,d){return Ok(None);}*output=self.projection.take();Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
 pub fn ready(&self)->bool{!self.closing&&self.projection.is_some()}
 pub fn begin_close(&mut self){self.closing=true;}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.retirement.is_empty(){return Ok(RetirementDemand{copy_bytes:self.retirement.next_copy_byte_demand()?,capacity_bytes:self.retirement.next_capacity_byte_demand(body)?,release_bytes:self.retirement.next_release_byte_demand()?,depth:self.retirement.next_depth_demand()?});}
  if let Some(projection)=self.projection.as_ref(){return if projection.terminal_is_empty(){Ok(header::<Option<RetainedOwnedProjection<M>>>())}else{projection.close_demand(body)};}
  if let Some(source)=self.source.as_ref(){return if source.terminal_is_empty(){Ok(header::<Option<RetainedCloneSource<M,FactoryOwnedSourceAuthority<M>>>>())}else{source.next_close_demand(body)};}
  self.retirement.next_owner_demands::<FactoryOwnedSourceAuthority<M>>(self.authority.is_some(),&None,body)
 }
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if !self.closing{return Ok(RetainedCloneStep::Progress(Default::default()));}let d=self.close_demand(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if !self.retirement.is_empty(){return self.retirement.step_granted(grant);}
  if let Some(projection)=self.projection.as_mut(){if !projection.terminal_is_empty(){return projection.close_step(grant);}self.projection.take();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}));}
  if let Some(source)=self.source.as_mut(){if !source.terminal_is_empty(){return source.close_step(grant);}self.source.take();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}));}
  if let Some(step)=self.retirement.begin_granted(&mut self.authority,grant)?{return Ok(step);}Ok(RetainedCloneStep::Complete(Default::default()))
 }
}
impl<M:Send+Sync+'static> ArtifactMutationSource<M>{pub fn terminal_is_empty(&self)->bool{self.closing&&self.authority.is_none()&&self.source.is_none()&&self.projection.is_none()&&self.retirement.is_empty()}}
impl<M:Send+Sync+'static> Drop for ArtifactMutationSource<M>{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"original mutation source requires funded closure");}}
