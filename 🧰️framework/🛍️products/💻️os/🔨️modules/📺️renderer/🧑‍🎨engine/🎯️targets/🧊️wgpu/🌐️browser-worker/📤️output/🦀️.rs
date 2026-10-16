//! 📤️ The original browser tick body remains in paid canonical encoding and wire custody until acknowledgement.
#[derive(semio_framework_value::RetireOwned,semio_framework_value::CanonicalJsonTree)]
#[value(rename_all="camelCase")]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct BrowserTickOutput {
    #[canonical_json(decimal_string)]
    pub presentation_generation:u64,
    pub media_slots:BrowserMediaOutput,
    pub cursor: &'static str,
    pub fullscreen: Option<bool>,
    pub request_frame: bool,
    pub next_deadline_delay_ms: Option<f64>,
    pub continue_frame: bool,
    pub progress: f32,
    pub quarantined: bool,
    pub fault_code: Option<&'static str>,
    pub fault_detail: Option<String>,
}

#[derive(semio_framework_value::RetireOwned)]
pub enum BrowserMediaOutput{Empty,Snapshot(crate::render_snapshot::RenderSnapshotOutputLoan)}
impl semio_framework_pack_json::ArtifactCanonicalJsonTree for BrowserMediaOutput{
 fn canonical_tree_node(&self)->Result<semio_framework_pack_json::ArtifactCanonicalJsonNode<'_>,semio_framework_value::ValueError>{use semio_framework_pack_json::ArtifactCanonicalJsonTree;match self{Self::Empty=>Ok(semio_framework_pack_json::ArtifactCanonicalJsonNode::Array(0)),Self::Snapshot(original)=>original.snapshot().media_slots.canonical_tree_node()}}
 fn canonical_tree_child(&self,index:usize)->Result<&dyn semio_framework_pack_json::ArtifactCanonicalJsonTree,semio_framework_value::ValueError>{use semio_framework_pack_json::ArtifactCanonicalJsonTree;match self{Self::Snapshot(original)=>original.snapshot().media_slots.canonical_tree_child(index),Self::Empty=>Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"original empty media output has no child"))}}
}

use semio_framework_pack_json::ArtifactCanonicalJsonTreeWireCursor;
use semio_framework_value::{retirement::controlled::ControlledRetirement,RetainedCloneProgress,RetirementDemand,RetainedCloneGrant,ValueError};
use semio_framework_job::{StepContext,OriginalTurnFailure};
use std::mem::ManuallyDrop;
pub struct OriginalBrowserOutput{encoding:ManuallyDrop<Option<ArtifactCanonicalJsonTreeWireCursor<BrowserTickOutput>>>,wire:ManuallyDrop<Option<ControlledRetirement<Vec<u8>>>>,closing:bool}
impl OriginalBrowserOutput{
 pub fn new(original:BrowserTickOutput)->Self{Self{encoding:ManuallyDrop::new(Some(ArtifactCanonicalJsonTreeWireCursor::new(original))),wire:ManuallyDrop::new(None),closing:false}}
 pub fn bytes(&self)->Option<&[u8]>{self.wire.as_ref().and_then(ControlledRetirement::original).map(|bytes|bytes.as_slice())}
 pub fn terminal_is_empty(&self)->bool{self.encoding.is_none()&&self.wire.is_none()}
 pub fn advance_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if self.wire.is_some(){return Ok(Default::default())}self.encoding.as_ref().map_or(Ok(Default::default()),|owner|owner.next_demand(body))}
 fn permits(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{grant.maximum_items>0&&grant.maximum_depth>=demand.depth&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes}
 pub fn advance(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.closing{return Ok(false)}if self.wire.is_some(){return Ok(true)}if cx.should_yield(){return Ok(false)}let grant=cx.retained_grant();
  let Some(encoding)=self.encoding.as_mut()else{return Ok(false)};let demand=encoding.next_demand(grant.maximum_copy_bytes)?;
  if grant.maximum_items==0||grant.maximum_depth<demand.depth||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(false)}
  cx.consume_fuel(1);let step=encoding.advance(grant).map_err(|error|OriginalTurnFailure::debit_original(error,cx))?;
  if let Some(original)=step.output{*self.wire=Some(ControlledRetirement::new(original).unwrap_or_else(|_|unreachable!("original wire Vec has typed retirement")))}
  cx.consume_retained(step.progress)?;Ok(self.wire.is_some())
 }
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}
  if !self.closing{cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;self.closing=true;if let Some(owner)=self.encoding.as_mut(){owner.begin_close()}return Ok(false)}
  if let Some(owner)=self.wire.as_mut(){if !owner.terminal_is_empty(){if !Self::permits(grant,owner.selected_demand(grant.maximum_copy_bytes)?){return Ok(false)}let demand=owner.selected_demand(grant.maximum_copy_bytes)?;let _=semio_framework_job::close_original_retained_step(cx,demand,|grant|owner.step(grant))?;return Ok(false)}cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;*self.wire=None;return Ok(false)}
  if let Some(owner)=self.encoding.as_mut(){if !owner.terminal_is_empty(){let demand=owner.next_demand(grant.maximum_copy_bytes)?;let _=semio_framework_job::close_original_retained_step(cx,demand,|grant|owner.close_step(grant).map(semio_framework_value::RetainedCloneStep::Progress))?;return Ok(false)}cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;*self.encoding=None;return Ok(false)}
  Ok(true)
 }
}
impl Drop for OriginalBrowserOutput{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original browser output requires full paid acknowledgement and closure")}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]mod tests;
