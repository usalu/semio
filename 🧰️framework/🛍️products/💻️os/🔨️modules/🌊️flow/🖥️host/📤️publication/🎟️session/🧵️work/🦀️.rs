//! 🎯️ The original session implements the source controller's first-party publication target.
use super::*;
use semio_os_flow_host_source::publication::{FlowPublicationTarget,FlowPublicationPayload};
pub type FlowWorkPublication=semio_os_flow_host_source::publication::FlowWorkPublication<FlowEvalSession>;
#[derive(semio_framework_value::RetireOwned)]
pub struct FlowSessionPublicationDisplaced{live:Option<SharedOwner<String>>,painted:String,converged:String,baseline:FlowBaselineLeases}
impl FlowPublicationTarget for FlowEvalSession{
 type Displaced=FlowSessionPublicationDisplaced;
 fn publication_identity(&self)->([usize;8],u64){([self.state.neural_cache.as_ref().map_or(0,|v|v.identity()),self.state.eval_json.as_ref().map_or(0,|v|v.identity()),self.state.painted_eval_json.as_ptr()as usize,self.state.painted_eval_json.len(),self.state.converged_eval_json.as_ptr()as usize,self.state.converged_eval_json.len(),self.state.previous_snapshot.as_ref().map_or(0,|v|v.identity()),self.state.previous_channels.as_ref().map_or(0,|v|v.identity())],self.state.flow_extension_generation)}
 fn converged_json(&self)->&str{&self.state.converged_eval_json}
 fn publish_original(&mut self,payload:FlowPublicationPayload,grant:RetainedCloneGrant)->Result<(Self::Displaced,RetainedCloneProgress),(ValueError,FlowPublicationPayload)>{
  if self.closing||grant.maximum_items==0||grant.maximum_depth==0||payload.generation<self.state.flow_extension_generation{return Err((ValueError::literal(ValueRefusalKind::WorkLimit,"original publication target requires its current funded ownership event"),payload))}
  let converged=payload.snapshot.is_some();let original=FlowSessionPublicationDisplaced{live:self.state.eval_json.take(),painted:std::mem::replace(&mut self.state.painted_eval_json,payload.painted),converged:payload.converged.map_or_else(String::new,|text|std::mem::replace(&mut self.state.converged_eval_json,text)),baseline:FlowBaselineLeases{snapshot:if converged{self.state.previous_snapshot.take()}else{None},channels:if converged{self.state.previous_channels.take()}else{None},current:None}};self.state.eval_json=Some(payload.live);if converged{self.state.previous_snapshot=payload.snapshot;self.state.previous_channels=payload.channels;self.state.flow_extension_generation=self.state.flow_extension_generation.max(payload.generation);self.state.baseline_geometry_pending=self.state.previous_channels.is_some();}Ok((original,RetainedCloneProgress{copied_items:1,..Default::default()}))
 }
}
impl FlowEvalSession{
 /// 🧵️ Leases the actual preborn evaluation authorities without constructing mutable host state.
 pub fn evaluation_work(&self,infos:SharedOwner<HistoryFoldIndex<String, OperatorInfo>>,revision:u64)->Result<FlowEvaluationWork,ValueError>{
  let registry=self.state.operator_registry.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original session requires its preborn registry authority"))?;
  Ok(FlowEvaluationWork::new(infos,registry.clone(),self.state.previous_channels.clone(),revision,self.state.flow_extension_generation).with_cache(self.neural_cache()))
 }
}

impl FlowEvalSession{
 /// 🧵️ Captures the supplied current registry without granting an older baseline a newer generation.
 pub fn evaluation_work_with_registry(&self,infos:SharedOwner<HistoryFoldIndex<String, OperatorInfo>>,registry:neural::SharedRegistry,revision:u64,generation:u64)->FlowEvaluationWork{FlowEvaluationWork::new(infos,registry,if self.state.flow_extension_generation==generation{self.state.previous_channels.clone()}else{None},revision,generation).with_cache(self.neural_cache())}
 /// 📥️ Moves genuine preborn baseline leases through the session's original retirement boundary.
 pub fn receive_eval_baseline_sources(&mut self,snapshot:&mut Option<SharedOwner<TreeSnapshot>>,channels:&mut Option<SharedOwner<EvalChannels>>,generation:u64,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<Option<RetainedCloneProgress>,ValueError>{
  let grant=effect.grant();
  if self.closing||self.state.baseline_geometry_pending||generation<self.state.flow_extension_generation{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original session baseline is closing or awaiting geometry"))}
  if grant.maximum_items==0||grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original baseline leases require an admitted event"))}
  let displaced=self.state.previous_snapshot.is_some()||self.state.previous_channels.is_some();let progress=if displaced{
   self.state.retirement.preflight(grant)?;
   let mut original=Some(SessionCollectionOwner::Baseline(FlowBaselineLeases{snapshot:self.state.previous_snapshot.take(),channels:self.state.previous_channels.take(),current:None}));
   let turn=self.state.retirement.admit(&mut original,effect);
   match turn.result{Ok(Some(p))=>p,result=>{let Some(SessionCollectionOwner::Baseline(original))=original else{unreachable!("original baseline lease family")};self.state.previous_snapshot=original.snapshot;self.state.previous_channels=original.channels;return result.map(|_|None)}}
  }else{RetainedCloneProgress{copied_items:1,..Default::default()}};
  self.state.previous_snapshot=snapshot.take();self.state.previous_channels=channels.take();self.state.flow_extension_generation=generation;self.state.baseline_geometry_pending=self.state.previous_channels.is_some();Ok(Some(progress))
 }
}
#[cfg(any(test,feature="testkit"))]
impl FlowEvalSession{
 /// 🔬️ Identifies the original pending roster without lending mutable private custody.
 pub fn test_preview_retention_roster_pointer(&self)->Option<*const String>{Some(self.state.preview_retention.as_ref()?.roster.as_ref()?.as_ptr())}
 /// 🔬️ Reports the three original tick-intake custody states without changing their issuer.
 pub fn test_tick_intake_diagnostic(&self)->(bool,bool,Option<(u64,u64)>){(self.state.pending_tick_snapshot.is_some(),self.state.pending_tick_snapshot_retirement.is_some(),self.state.pending_tick_snapshot_origin)}
 /// 🔬️ Counts genuine parked origins without exporting the private ledger entry type.
 pub fn test_invocation_origin_count(&self)->usize{self.state.invocation_origins.len()}
}

impl FlowEvalSession{
 /// 🔗️ Borrows the current original baseline leases without copying semantic data.
 pub fn eval_baseline_sources(&self)->(Option<SharedOwner<TreeSnapshot>>,Option<SharedOwner<EvalChannels>>){(self.state.previous_snapshot.clone(),self.state.previous_channels.clone())}
}
