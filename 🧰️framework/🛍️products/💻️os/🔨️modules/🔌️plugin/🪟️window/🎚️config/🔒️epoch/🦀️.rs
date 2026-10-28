//! 🔒️ A single original Window preview epoch prepares every declared partition before shared publication.
use super::*;
use semio_framework_job::{StepContext,OriginalTurnFailure,OriginalSourceBinding,OriginalConstructionDemand};
use semio_framework_value_native_owned::native_decoding::{NativeDecodeControl,NativeConstructionStepFailure};
/// 🧳️ The same original caller and decoder own traversal, visibility and every displaced preview read.
pub struct OriginalWindowPreviewEpoch{binding:Option<OriginalSourceBinding>,decoder:(u64,u64),lifetime:u64,epoch:u64,previous:u64,body:usize,owner:usize,partition:usize,partition_ready:bool,ready:bool,published:bool,closing:bool}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::OwnershipLimit,message)}
fn header<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}
fn progress(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}}
fn construction(d:RetirementDemand)->OriginalConstructionDemand{OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth}}
impl OriginalWindowPreviewEpoch{
 /// 📏️ Quotes the actual retained epoch header and registry reservation.
 pub fn admission_demands()->RetirementDemand{header::<(Self,Self,Option<u64>)>()}
 /// 📥️ Reserves one private epoch under its genuine caller, without changing visible previews.
 pub fn admit(registry:&mut WindowConfigOwnerRegistry,body:usize,cx:&mut StepContext<'_>,output:&mut Option<Self>)->Result<bool,OriginalTurnFailure>{
  if output.is_some()||registry.preview_active_epoch.is_some()||body==0{return Err(invalid("original Window preview requires an empty epoch recipient and an unreserved registry").into())}
  let epoch=registry.preview_visible_epoch.checked_add(1).filter(|epoch|*epoch!=0).ok_or_else(||invalid("original Window preview epoch exhausted"))?;
  let binding=OriginalSourceBinding::borrow(cx);let d=Self::admission_demands();semio_framework_job::admit_original_construction(cx,output,construction(d),|control|{let decoder=(control.ledger_identity(),control.retirement_recipient_identity().ok_or_else(||invalid("original Window preview requires its actual decoder"))?);registry.preview_active_epoch=Some(epoch);Ok((Self{binding:Some(binding),decoder,lifetime:registry.lifetime,epoch,previous:registry.preview_visible_epoch,body,owner:0,partition:0,partition_ready:false,ready:false,published:false,closing:false},progress(d)))})
 }
 /// 🔗️ Validates original authority before an enclosing common publication event.
 pub fn validate_context(&self,registry:&WindowConfigOwnerRegistry,cx:&StepContext<'_>)->Result<(),ValueError>{if !self.binding.is_some_and(|binding|binding.matches(cx))||cx.native_receiving_identity()!=Some(self.decoder)||registry.lifetime!=self.lifetime||registry.preview_active_epoch!=Some(self.epoch)||registry.preview_visible_epoch!=if self.published{self.epoch}else{self.previous}{return Err(invalid("original Window preview changed its caller, decoder or registry epoch"))}Ok(())}
 /// 🪪️ Every original Window leaf uses this reserved epoch.
 pub fn epoch(&self)->u64{self.epoch}
 /// 📏️ Quotes one partition preparation or bounded ordinal advance.
 pub fn demands(&self,registry:&WindowConfigOwnerRegistry)->Result<RetirementDemand,ValueError>{
  if self.closing{return self.close_demands(registry)}if self.ready{return Ok(Default::default())}
  let Some(owner)=registry.owners.get_index(self.owner)else{return Ok(header::<bool>())};
  if self.partition_ready||self.partition>=owner.preview_partition_count(){return Ok(header::<(usize,usize,bool)>())}
  let mut d=owner.preview_partition_demands(self.partition,self.epoch,false,self.body)?;d.copy_bytes+=size_of::<bool>();Ok(d)
 }
 /// 🪜️ Prepares exactly one original partition fragment; earlier previews remain visible.
 pub fn advance(&mut self,registry:&mut WindowConfigOwnerRegistry,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate_context(registry,cx)?;if self.closing{return Err(invalid("original Window preview is closing").into())}if self.ready{return Ok(true)}let d=self.demands(registry)?;
  let Some(owner)=registry.owners.get_index(self.owner)else{paid(cx,d,|_|{self.ready=true;Ok(progress(d))})?;return Ok(self.ready)};
  if self.partition_ready||self.partition>=owner.preview_partition_count(){paid(cx,d,|_|{if self.partition_ready{self.partition+=1;self.partition_ready=false}else{self.owner+=1;self.partition=0}Ok(progress(d))})?;return Ok(false)}
  receive(cx,d,|native,g|{let child=RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-size_of::<bool>(),..g};let(done,mut p)=registry.owners.get_index_mut(self.owner).unwrap().preview_partition_step(self.partition,self.epoch,false,self.body,child,native)?;self.partition_ready=done;p.copied_items=p.copied_items.max(1);p.copied_bytes+=size_of::<bool>();Ok(RetainedCloneStep::Progress(p))})?;Ok(false)
 }
 /// ✅️ All declared partitions now hold their complete private candidate or explicit cleared preview.
 pub fn ready(&self)->bool{self.ready&&!self.closing}
 /// 📏️ The common publication only changes the visibility epoch and retained decision bit.
 pub fn publication_demands(&self)->RetirementDemand{header::<(u64,bool)>()}
 /// 📣️ An enclosing original event publishes this epoch together with document and configuration aliases.
 pub fn publish_granted(&mut self,registry:&mut WindowConfigOwnerRegistry,g:RetainedCloneGrant,native:&mut NativeDecodeControl<'_>)->Result<Option<RetainedCloneProgress>,ValueError>{
  if !self.ready()||self.published||registry.lifetime!=self.lifetime||registry.preview_active_epoch!=Some(self.epoch)||registry.preview_visible_epoch!=self.previous||(native.ledger_identity(),native.retirement_recipient_identity().ok_or_else(||invalid("original Window publication requires its actual decoder"))?)!=self.decoder{return Err(invalid("original Window publication changed its prepared source"))}let d=self.publication_demands();if !registry::fits(g,d){return Ok(None)}registry.preview_visible_epoch=self.epoch;self.published=true;Ok(Some(progress(d)))
 }
 /// 📏️ Closure either adopts the published original reads or discards private candidates.
 pub fn close_demands(&self,registry:&WindowConfigOwnerRegistry)->Result<RetirementDemand,ValueError>{
  if self.binding.is_none(){return Ok(Default::default())}if !self.closing{return Ok(header::<(usize,usize,bool,bool)>())}
  let Some(owner)=registry.owners.get_index(self.owner)else{return Ok(header::<(Option<OriginalSourceBinding>,Option<u64>)>())};
  if self.partition_ready||self.partition>=owner.preview_partition_count(){return Ok(header::<(usize,usize,bool)>())}
  let mut d=owner.preview_epoch_close_demands(self.partition,self.epoch,self.published,self.body)?;d.copy_bytes+=size_of::<bool>();Ok(d)
 }
 /// ♻️ Cancelled callers keep their original native closure authority until every private read returns.
 pub fn close_step(&mut self,registry:&mut WindowConfigOwnerRegistry,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.terminal_is_empty(){return Ok(true)}self.validate_context(registry,cx)?;let d=self.close_demands(registry)?;
  if !self.closing{close_paid(cx,d,|_|{self.closing=true;self.owner=0;self.partition=0;self.partition_ready=false;Ok(progress(d))})?;return Ok(false)}
  let Some(owner)=registry.owners.get_index(self.owner)else{close_paid(cx,d,|_|{registry.preview_active_epoch=None;self.binding=None;Ok(progress(d))})?;return Ok(self.terminal_is_empty())};
  if self.partition_ready||self.partition>=owner.preview_partition_count(){close_paid(cx,d,|_|{if self.partition_ready{self.partition+=1;self.partition_ready=false}else{self.owner+=1;self.partition=0}Ok(progress(d))})?;return Ok(false)}
  if cx.retirement_should_yield()||!registry::fits(cx.retained_grant(),d){return Ok(false)}let grant=cx.retained_grant();let result=cx.with_native_retirement(|native|{let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-size_of::<bool>(),..grant};let(done,mut p)=registry.owners.get_index_mut(self.owner).unwrap().preview_epoch_close_step(self.partition,self.epoch,self.published,self.body,child,native)?;self.partition_ready=done;p.copied_items=p.copied_items.max(1);p.copied_bytes+=size_of::<bool>();Ok::<_,ValueError>(RetainedCloneStep::Progress(p))});post(cx,result)?;Ok(false)
 }
 /// 🧺️ The original registry reservation and every retained private source have closed.
 pub fn terminal_is_empty(&self)->bool{self.binding.is_none()}
}
fn paid(cx:&mut StepContext<'_>,d:RetirementDemand,run:impl FnOnce(RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>)->Result<(),OriginalTurnFailure>{let grant=cx.retained_grant();let mut output=None;semio_framework_job::admit_original_construction(cx,&mut output,construction(d),|_|run(grant).map(|p|((),p)))?;Ok(())}
fn close_paid(cx:&mut StepContext<'_>,d:RetirementDemand,run:impl FnOnce(RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>)->Result<(),OriginalTurnFailure>{semio_framework_job::close_original_retained_step(cx,d,|g|run(g).map(RetainedCloneStep::Progress))?;Ok(())}
fn receive(cx:&mut StepContext<'_>,d:RetirementDemand,run:impl FnOnce(&mut NativeDecodeControl<'_>,RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>)->Result<(),OriginalTurnFailure>{if cx.should_yield()||!registry::fits(cx.retained_grant(),d){return Ok(())}let result=cx.with_native_receiving::<_,NativeConstructionStepFailure<ValueError>>(|native|native.receive_construction_step(d,run));let p=result.as_ref().map(|step|step.progress()).unwrap_or_else(|error|error.retained_progress());let receipt=cx.receiving_issued_grant().map(|grant|cx.consume_receiving_retained(grant,p)).transpose();cx.consume_fuel(p.copied_items as u64);combine(result.map(|_|()),receipt.map(|_|()))}
fn post(cx:&mut StepContext<'_>,result:Result<RetainedCloneStep,ValueError>)->Result<(),OriginalTurnFailure>{let p=result.as_ref().map(|step|step.progress()).unwrap_or_else(|error|error.retained_progress());let receipt=cx.consume_retained(p);cx.consume_fuel(p.copied_items as u64);combine(result.map(|_|()),receipt)}
fn combine<E:Into<OriginalTurnFailure>>(result:Result<(),E>,receipt:Result<(),ValueError>)->Result<(),OriginalTurnFailure>{match(result,receipt){(Ok(()),Ok(()))=>Ok(()),(Err(error),Ok(()))=>Err(error.into()),(Ok(()),Err(error))=>Err(error.into()),(Err(error),Err(refusal))=>{let mut error=error.into();error.receipt_refusal=Some(refusal);Err(error)}}}
impl Drop for OriginalWindowPreviewEpoch{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original Window preview epoch requires paid source closure")}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
