//! 📤️ Original history envelopes stream directly into the canonical paged backbone message.
use super::ArtifactPreparedOperationOutput;
use semio_framework_os_spr::MutationEnvelope;
use semio_framework_replication_causal::binary::native_encoding::CausalEnvelopeWireCursor;
use semio_framework_value::ErasedSnapshotRetirement;
use semio_framework_value::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,ValueError,ValueRefusalKind};
use std::mem::size_of;
/// 🪡️ The enclosing original publisher retains and authenticates its exact immutable transition vector and epoch.
pub struct OriginalHistoryBatchMessage{cursor:CausalEnvelopeWireCursor,output:ArtifactPreparedOperationOutput,source:Option<(usize,usize,usize,u64)>,pending:[u8;64],length:usize,offset:usize,taken:bool,closing:bool}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original history message changed its envelope source or epoch")}
fn fixed()->RetirementDemand{RetirementDemand{copy_bytes:128,depth:1,..Default::default()}}
impl OriginalHistoryBatchMessage{
 /// 🌱️ Empty output owns no heap allocation and keeps the real causal batch wire bound.
 pub fn new(maximum_bytes:usize,maximum_capacity:usize)->Result<Self,ValueError>{Ok(Self{cursor:Default::default(),output:ArtifactPreparedOperationOutput::empty(maximum_bytes,maximum_capacity)?,source:None,pending:[0;64],length:0,offset:0,taken:false,closing:false})}
 /// 📏️ Cursor reads and actual page reservations each quote their own bounded action.
 pub fn demands(&self,_body:usize)->Result<RetirementDemand,ValueError>{if self.closing{return self.close_demands()}let mut d=if self.offset<self.length{self.output.append_demands(self.length-self.offset)?}else{self.cursor.demand(64)};d.copy_bytes=d.copy_bytes.checked_add(128).ok_or_else(invalid)?;Ok(d)}
 /// ✍️ Reads canonical metadata or at most64 original payload bytes without gathering pages or cloning an envelope.
 pub fn advance(&mut self,envelopes:&Vec<MutationEnvelope>,epoch:u64,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{let source=(envelopes as*const _ as usize,envelopes.as_ptr()as usize,envelopes.len(),epoch);if self.closing||self.taken||self.source.is_some_and(|original|original!=source){return Err(invalid())}if self.ready(){return Ok(Default::default())}let d=self.demands(grant.maximum_copy_bytes)?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(Default::default())}self.source=Some(source);let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-128,..grant};let p=if self.offset<self.length{let p=self.output.append(&self.pending[self.offset..self.length],child)?;self.offset=self.offset.checked_add(p.copied_bytes).ok_or_else(invalid)?;p}else{let step=self.cursor.advance(envelopes,&mut self.pending,child)?;self.length=step.written_bytes;self.offset=0;step.progress};p.checked_add(RetainedCloneProgress{copied_items:usize::from(p.copied_items==0),copied_bytes:128,..Default::default()})}
 /// 🏁️ Every field fragment has reached its actual output page before the message can be donated.
 pub fn ready(&self)->bool{!self.closing&&!self.taken&&self.cursor.is_complete()&&self.offset==self.length}
 pub fn take_demands()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<semio_framework_value::paged::PagedBytes<{isize::MAX as usize}>>()+128,depth:1,..Default::default()}}
 /// 🤝️ Donates the same canonical page chain to the original native outbox, without a Vec intermediary.
 pub fn take(&mut self,grant:RetainedCloneGrant)->Result<Option<(semio_framework_value::paged::PagedBytes<{isize::MAX as usize}>,RetainedCloneProgress)>,ValueError>{if !self.ready(){return Err(invalid())}let d=Self::take_demands();if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(None)}let Some((bytes,_))=self.output.take_bytes(grant)?else{return Ok(None)};self.taken=true;Ok(Some((bytes,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))}
 /// 📏️ Exact original page disposal precedes the final borrowed-coordinate return.
 pub fn close_demands(&self)->Result<RetirementDemand,ValueError>{if !self.output.terminal_is_empty(){return self.output.retirement_demands()}Ok(fixed())}
 /// ♻️ The original recipient retains donated pages; cancellation physically returns only this owner's admitted pages.
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.closing=true;if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}let d=self.close_demands()?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}if !self.output.terminal_is_empty(){return self.output.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()))}self.source=None;Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
 pub fn terminal_is_empty(&self)->bool{self.source.is_none()&&self.output.terminal_is_empty()}
}
impl Drop for OriginalHistoryBatchMessage{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"original history message requires paid transfer or closure");}}

#[cfg(test)]
#[path="tests.rs"]
mod tests;
