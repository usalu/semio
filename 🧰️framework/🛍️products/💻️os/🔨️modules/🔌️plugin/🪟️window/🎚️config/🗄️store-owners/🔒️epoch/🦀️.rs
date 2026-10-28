//! 🔒️ A complete original Window epoch retains its private read until the common visibility decision.
use super::*;
impl<O:WindowConfigOwner> WindowConfigPartition<O>{
 fn epoch_private_pending(&self)->bool{self.prepared_epoch!=0||self.prepared_preview.is_some()||self.semantic_preview.is_some()||self.finish_preview_base.is_some()||self.finish_preview_read.is_some()||self.preview_retirement_pending()}
 fn epoch_displaced_pending(&self)->bool{self.pending_preview.is_some()||self.pending_preview_reads.is_some()||self.pending_preview_projection.is_some()||self.pending_preview_projection_read.is_some()||self.preview_retirement.is_some()}
 pub(super) fn epoch_close_demands(&self,epoch:u64,published:bool,body:usize)->Result<RetirementDemand,ValueError>{
  if published&&self.prepared_epoch==epoch{
   if self.epoch_displaced_pending(){return self.preview_retirement_demand(body)}
   return Ok(RetirementDemand{copy_bytes:if self.published_preview.is_some(){size_of::<Option<WindowConfigSnapshot>>()+size_of::<u64>()}else{size_of::<Option<WindowConfigSnapshot>>()+2*size_of::<u64>()},depth:1,..Default::default()})
  }
  if !self.epoch_private_pending(){return Ok(Default::default())}
  self.preview_cancel_demand(body,false)
 }
 pub(super) fn epoch_close_step(&mut self,epoch:u64,published:bool,body:usize,grant:RetainedCloneGrant,control:&mut dyn semio_framework_value::OriginalRetirementControl)->Result<(bool,RetainedCloneProgress),ValueError>{
  let demand=self.epoch_close_demands(epoch,published,body)?;if !registry::fits(grant,demand){return Ok((false,Default::default()))}
  if !self.epoch_private_pending(){return Ok((true,Default::default()))}
  let p=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
  if published&&self.prepared_epoch==epoch{
   if self.epoch_displaced_pending(){return self.preview_retirement_step_original(body,grant,Some(control)).map(|step|(false,step.progress()))}
   if self.published_preview.is_some(){self.pending_preview=self.published_preview.take();self.published_epoch=0;return Ok((false,p))}
   self.published_preview=self.prepared_preview.take();self.published_epoch=epoch;self.prepared_epoch=0;return Ok((false,p))
  }
  if self.semantic_preview.as_ref().is_some_and(WindowSemanticPreview::needs_cancellation){self.semantic_preview.as_mut().unwrap().cancel();return Ok((false,p))}
  if self.epoch_displaced_pending(){return self.preview_retirement_step_original(body,grant,Some(control)).map(|step|(false,step.progress()))}
  if self.prepared_epoch!=0||self.prepared_preview.is_some(){self.pending_preview=self.prepared_preview.take();self.prepared_epoch=0;return Ok((false,p))}
  if self.finish_preview_read.is_some(){self.pending_preview_projection_read=self.finish_preview_read.take();return Ok((false,p))}
  if self.finish_preview_base.is_some(){self.pending_preview_projection=self.finish_preview_base.take();return Ok((false,p))}
  let step=self.preview_retirement_step_original(body,grant,Some(control))?;Ok((!self.epoch_private_pending(),step.progress()))
 }
}
