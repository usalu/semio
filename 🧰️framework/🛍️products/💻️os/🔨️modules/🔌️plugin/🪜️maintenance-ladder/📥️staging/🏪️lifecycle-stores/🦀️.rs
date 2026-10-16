//! 🪯️ Staged slice of the `lifecycle-stores` band of the mounted artifact instance: its `instance-ladders` members (WP6 step S0 of the instance family).
use super::*;

/// 🏪️ Original store disposal keeps body, concrete frame and stage advance on separate grants.
pub(crate) const ORIGINAL_STORE_LANES:[&str;8]=["document","config","draft","presence","transient","interaction","windowConfig","windowTransient"];

fn original_disposal_demands<T>(slot:&ArtifactDisposal<T>,owner:&T,body:usize)->Result<RetirementDemand,ValueError>{
 let disposer=slot.as_ref().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"store close has no original disposal authority"))?;
 if disposer.terminal_is_empty(owner){return Ok(RetirementDemand{release_bytes:{let bytes=disposer.terminal_frame_release_bytes().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"terminal disposer has no original physical frame authority"))?;if bytes!=size_of_val(&**disposer){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"terminal disposer changed its declared original Box extent"));}bytes},depth:1,..Default::default()});}
 disposer.retirement_demands(owner,body)
}

fn original_disposal_step<T>(slot:&mut ArtifactDisposal<T>,owner:&mut T,grant:RetainedCloneGrant)->Result<(PluginLifecycleStep,bool),Fault>{
 let demand=original_disposal_demands(slot,owner,grant.maximum_copy_bytes).map_err(|error|Fault::from(error.into_message()))?;
 if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok((PluginLifecycleStep::Progress(Default::default()),false));}
 let disposer=slot.as_mut().expect("quoted original disposer");
 if disposer.terminal_is_empty(owner){let bytes=demand.release_bytes;slot.take();return Ok((PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()}),true));}
 let step=disposer.close_step(owner,grant)?;
 if matches!(step,PluginLifecycleStep::Complete(_))&&!disposer.terminal_is_empty(owner){return Err(Fault::from("store disposer claimed a false original terminal witness"));}
 Ok((match step{PluginLifecycleStep::Complete(progress)=>PluginLifecycleStep::Progress(progress),other=>other},false))
}

impl<A:ArtifactApp,M:SpaceMember+MemberFactory+Send+'static> VcsArtifactApp<A,M>{
 pub(crate) fn close_owned_stores_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.close.close_owned_advance_ready{return Ok(RetirementDemand{depth:1,..Default::default()});}
  match self.close.close_owned_stage{
   0=>original_disposal_demands(&self.close.close_document_disposer,&self.stores.store,body),
   1=>original_disposal_demands(&self.close.close_config_disposer,&self.stores.config_store,body),
   2=>original_disposal_demands(&self.close.close_draft_disposer,&self.stores.draft_store,body),
   3=>original_disposal_demands(&self.close.close_presence_disposer,&self.stores.presence_store,body),
   4=>original_disposal_demands(&self.close.close_transient_disposer,&self.stores.transient_store,body),
   5=>original_disposal_demands(&self.close.close_interaction_disposer,&self.stores.interaction_store,body),
   6=>if self.stores.window_config_store.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{self.stores.window_config_store.retirement_demands(body)},
   7=>if self.stores.window_transient_store.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{self.stores.window_transient_store.retirement_demands(body)},
   _=>Ok(Default::default()),
  }
 }

 pub(crate) fn close_owned_stores_step(&mut self,grant:RetainedCloneGrant)->Result<PluginLifecycleStep,Fault>{
  let demand=self.close_owned_stores_demands(grant.maximum_copy_bytes).map_err(|error|Fault::from(error.into_message()))?;
  if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(PluginLifecycleStep::Progress(Default::default()));}
  if self.close.close_owned_advance_ready{self.close.close_owned_stage += 1;self.close.close_owned_advance_ready=false;return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}));}
  let (step,ready)=match self.close.close_owned_stage{
   0=>{original_disposal_step(&mut self.close.close_document_disposer,&mut self.stores.store,grant)?},
   1=>{original_disposal_step(&mut self.close.close_config_disposer,&mut self.stores.config_store,grant)?},
   2=>{original_disposal_step(&mut self.close.close_draft_disposer,&mut self.stores.draft_store,grant)?},
   3=>{original_disposal_step(&mut self.close.close_presence_disposer,&mut self.stores.presence_store,grant)?},
   4=>{original_disposal_step(&mut self.close.close_transient_disposer,&mut self.stores.transient_store,grant)?},
   5=>{original_disposal_step(&mut self.close.close_interaction_disposer,&mut self.stores.interaction_store,grant)?},
   6=>{if self.stores.window_config_store.terminal_is_empty(){(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}),true)}else{(self.stores.window_config_store.close_step(grant)?,false)}},
   7=>{if self.stores.window_transient_store.terminal_is_empty(){(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}),true)}else{(self.stores.window_transient_store.close_step(grant)?,false)}},
   _=>return Ok(PluginLifecycleStep::Complete(Default::default())),
  };
  self.close.close_owned_advance_ready=ready;Ok(match step{PluginLifecycleStep::Complete(progress)=>PluginLifecycleStep::Progress(progress),other=>other})
 }
}

#[cfg(test)]
#[path="../../../♻️lifecycle/🏪️stores/🧪️tests/🦀️.rs"]
mod original_store_stage_tests;
