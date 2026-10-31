//! 🔏️ Original aggregate edits retain their native typed canonical seal and independent frame custody.
use semio_framework_pack_json::ArtifactCanonicalJsonTree;
use semio_framework_os_spr::Edit;
use semio_framework_value::{ErasedSnapshotRetirement, ValueError};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneSource, RetainedCloneSourceAdmission, RetainedCloneStep};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use std::mem::size_of;
use protocol::io::text::canonical::ArtifactCanonicalEditSealCursor;
use semio_framework_value::{RetirementDemand,ValueRefusalKind,retirement::RetireOwned};

/// 🧵️ The exact original typed sealer crosses only this bounded owning interface.
pub trait ArtifactStoreBatchDigest<M>:ErasedSnapshotRetirement{
 fn next_digest_demand(&self)->Result<RetirementDemand,ValueError>;
 fn advance_digest(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
 fn ready(&self)->bool;
 fn take_edit(&mut self,grant:RetainedCloneGrant)->Result<Option<(Box<Edit<M>>,[u8;32],RetainedCloneProgress)>,ValueError>;
 fn begin_close(&mut self);
}
struct NativeBatchDigest<M:RetireOwned+ArtifactCanonicalJsonTree>{edit:Option<Box<Edit<M>>>,admission:Option<RetainedCloneSourceAdmission<Box<Edit<M>>,Box<Edit<M>>>>,source:Option<RetainedCloneSource<Box<Edit<M>>>>,original:Option<ArtifactCanonicalEditSealCursor<M>>,edit_close:Option<ControlledRetirement<Box<Edit<M>>>>}
fn nested(mut demand:RetirementDemand)->Result<RetirementDemand,ValueError>{demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"batch digest original depth overflow"))?;Ok(demand)}
fn child(mut grant:RetainedCloneGrant)->RetainedCloneGrant{grant.maximum_items=1;grant.maximum_depth-=1;grant}
fn admitted(d:RetirementDemand,g:RetainedCloneGrant)->bool{g.maximum_items>0&&g.maximum_copy_bytes>=d.copy_bytes&&g.maximum_capacity_bytes>=d.capacity_bytes&&g.maximum_release_bytes>=d.release_bytes&&g.maximum_depth>=d.depth}
fn header<M>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Box<Edit<M>>>(),depth:1,..Default::default()}}

/// 📏️ Quotes the separate original transport frame that retains the edit before any source admission.
pub fn artifact_batch_digest_birth_demand<M:RetireOwned+ArtifactCanonicalJsonTree>()->Result<RetirementDemand,ValueError>{let mut demand=nested(header::<M>())?;demand.capacity_bytes=size_of::<NativeBatchDigest<M>>();Ok(demand)}

/// 🎟️ Moves the original edit into its retained frame; the native source and sealer are admitted by later granted turns.
pub fn admit_artifact_batch_digest<M:RetireOwned+ArtifactCanonicalJsonTree+Send+Sync+'static>(edit:&mut Option<Box<Edit<M>>>,grant:RetainedCloneGrant)->Result<Option<(Box<dyn ArtifactStoreBatchDigest<M>>,RetainedCloneProgress)>,ValueError>{
 let demand=artifact_batch_digest_birth_demand::<M>()?;
 if !admitted(demand,grant){return Ok(None);}
 let Some(original)=edit.take() else{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"batch digest requires its original edit"))};
 Ok(Some((Box::new(NativeBatchDigest{edit:Some(original),admission:None,source:None,original:None,edit_close:None}),RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,..Default::default()})))
}
impl<M:RetireOwned+ArtifactCanonicalJsonTree+Send+Sync+'static> NativeBatchDigest<M>{
 fn close_demand(&self)->Result<RetirementDemand,ValueError>{
  if let Some(original)=self.original.as_ref(){return nested(original.next_demand()?);}
  if let Some(admission)=self.admission.as_ref(){return nested(admission.next_return_demand()?);}
  if let Some(source)=self.source.as_ref(){let body=source.next_close_copy_byte_demand()?;return nested(RetirementDemand{copy_bytes:body,capacity_bytes:source.next_close_capacity_byte_demand(body)?,release_bytes:source.next_close_release_byte_demand()?,depth:source.next_close_depth_demand()?,..Default::default()});}
  if let Some(owner)=self.edit_close.as_ref(){let body=owner.next_copy_byte_demand()?;return nested(RetirementDemand{copy_bytes:body,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?,..Default::default()});}
  nested(header::<M>())
 }
}
impl<M:RetireOwned+ArtifactCanonicalJsonTree+Send+Sync+'static> ArtifactStoreBatchDigest<M> for NativeBatchDigest<M>{
 fn next_digest_demand(&self)->Result<RetirementDemand,ValueError>{
  if let Some(original)=self.original.as_ref(){return nested(if original.is_ready(){original.next_take_demand()}else{original.next_demand()?});}
  if let Some(admission)=self.admission.as_ref(){return if admission.phase()=="ready"{nested(header::<M>())}else{nested(admission.next_demand()?)};}
  if self.source.is_some(){return nested(ArtifactCanonicalEditSealCursor::<M>::source_constructor_demand());}
  nested(header::<M>())
 }
 fn advance_digest(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if !admitted(self.next_digest_demand()?,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if let Some(original)=self.original.as_mut(){return original.advance(child(grant));}
  if let Some(admission)=self.admission.as_mut(){
   if admission.phase()=="ready"{let step=admission.take_output(&mut self.source,child(grant)).result?;if admission.terminal_is_empty(){self.admission=None;}return Ok(RetainedCloneStep::Progress(step.progress()));}
   return admission.advance(child(grant)).result.map(|step|RetainedCloneStep::Progress(step.progress()));
  }
  if self.source.is_some(){return Ok(match ArtifactCanonicalEditSealCursor::admit_source(&mut self.source,child(grant))?{Some((cursor,receipt))=>{self.original=Some(cursor);RetainedCloneStep::Progress(receipt)},None=>RetainedCloneStep::Progress(Default::default())});}
  let edit=self.edit.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"batch digest lost its original edit"))?;
  self.admission=Some(RetainedCloneSource::<Box<Edit<M>>>::admission_owned(edit,()));
  Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Box<Edit<M>>>(),..Default::default()}))
 }
 fn ready(&self)->bool{self.original.as_ref().is_some_and(ArtifactCanonicalEditSealCursor::is_ready)}
 fn take_edit(&mut self,grant:RetainedCloneGrant)->Result<Option<(Box<Edit<M>>,[u8;32],RetainedCloneProgress)>,ValueError>{if !admitted(self.next_digest_demand()?,grant){return Ok(None);}match self.original.as_mut(){Some(original)=>original.take_edit(child(grant)),None=>Ok(None)}}
 fn begin_close(&mut self){if let Some(original)=self.original.as_mut(){original.begin_close();}}
}
impl<M:RetireOwned+ArtifactCanonicalJsonTree+Send+Sync+'static> ErasedSnapshotRetirement for NativeBatchDigest<M>{
 fn next_demand(&self,_:usize)->Result<RetirementDemand,ValueError>{self.close_demand()}
 fn close_step(&mut self, grant: RetainedCloneGrant) -> semio_framework_value::RetainedWorkOutcome {
  let result=(||->Result<RetainedCloneStep,ValueError>{
   if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
   if !admitted(self.close_demand()?,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
   let progress=if let Some(original)=self.original.as_mut(){original.begin_close();if original.terminal_is_empty(){self.original=None;RetainedCloneProgress{copied_items:1,..Default::default()}}else{original.close_step(child(grant))?.progress()}}
    else if let Some(admission)=self.admission.as_mut(){let mut returned=None;let turn=admission.return_inputs(&mut returned,child(grant));if let Some((edit,()))=returned{self.edit=Some(edit);}if admission.terminal_is_empty(){self.admission=None;}turn.result?.progress()}
    else if let Some(source)=self.source.as_mut(){if source.terminal_is_empty(){self.source=None;RetainedCloneProgress{copied_items:1,..Default::default()}}else{source.close_step(child(grant)).result?.progress()}}
    else if let Some(edit)=self.edit.take(){self.edit_close=Some(ControlledRetirement::retain(edit));RetainedCloneProgress{copied_items:1,..Default::default()}}
    else if let Some(owner)=self.edit_close.as_mut(){if owner.terminal_is_empty(){self.edit_close=None;RetainedCloneProgress{copied_items:1,..Default::default()}}else{owner.step(child(grant))?.progress()}}
    else{Default::default()};
   Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
  })();
  let work=u64::from(result.as_ref().is_ok_and(|step|step.progress()!=RetainedCloneProgress::default()));semio_framework_value::RetainedWorkOutcome{result,work}
 }
 fn terminal_is_empty(&self)->bool{self.edit.is_none()&&self.admission.is_none()&&self.source.is_none()&&self.original.is_none()&&self.edit_close.is_none()}
 fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand()?.copy_bytes)}
 fn next_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(self.close_demand()?.capacity_bytes)}
 fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand()?.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand()?.depth)}
}

/// 📤️ Original precommit message custody returns the unchanged edit and its prepared causal bytes.
pub type OriginalCausalMessageBytes=semio_framework_value::paged::PagedBytes<{isize::MAX as usize}>;

/// 📤️ Actual original paged causal bytes stay with their retained edit issuer until handoff.
pub trait ArtifactStoreBatchMessage<M>:ErasedSnapshotRetirement{
 fn next_message_demand(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn advance_message(&mut self,document:&str,schema:&str,observed:Option<&str>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
 fn ready(&self)->bool;
 fn take_edit_message(&mut self,grant:RetainedCloneGrant)->Result<Option<(Box<Edit<M>>,OriginalCausalMessageBytes,RetainedCloneProgress)>,ValueError>;
 fn begin_close(&mut self);
}
