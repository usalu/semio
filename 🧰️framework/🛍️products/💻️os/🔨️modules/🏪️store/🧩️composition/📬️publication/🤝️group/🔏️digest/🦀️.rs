//! 🔏️ Original aggregate edits retain their native typed canonical seal and independent frame custody.
use super::*;
use crate::os_spr::command::ArtifactCanonicalEditSealCursor;
use semio_framework_value::{RetirementDemand,ValueRefusalKind,retirement::RetireOwned};

/// 🧵️ The exact original typed sealer crosses only this bounded owning interface.
pub trait ArtifactStoreBatchDigest<M>:ErasedSnapshotRetirement{
 fn next_digest_demand(&self)->Result<RetirementDemand,ValueError>;
 fn advance_digest(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
 fn ready(&self)->bool;
 fn take_edit(&mut self,grant:RetainedCloneGrant)->Result<Option<(Box<Edit<M>>,[u8;32],RetainedCloneProgress)>,ValueError>;
 fn begin_close(&mut self);
}
struct NativeBatchDigest<M:RetireOwned+ArtifactCanonicalJsonTree>{original:ArtifactCanonicalEditSealCursor<M>}
fn nested(mut demand:RetirementDemand)->Result<RetirementDemand,ValueError>{demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"batch digest original depth overflow"))?;Ok(demand)}
fn child(mut grant:RetainedCloneGrant)->RetainedCloneGrant{grant.maximum_items=1;grant.maximum_depth-=1;grant}
fn admitted(d:RetirementDemand,g:RetainedCloneGrant)->bool{g.maximum_items>0&&g.maximum_copy_bytes>=d.copy_bytes&&g.maximum_capacity_bytes>=d.capacity_bytes&&g.maximum_release_bytes>=d.release_bytes&&g.maximum_depth>=d.depth}

/// 📏️ Quotes the actual native source and separate original transport frame before admission.
pub fn artifact_batch_digest_birth_demand<M:RetireOwned+ArtifactCanonicalJsonTree>()->Result<RetirementDemand,ValueError>{let mut demand=nested(ArtifactCanonicalEditSealCursor::<M>::constructor_demand())?;demand.capacity_bytes=demand.capacity_bytes.checked_add(std::mem::size_of::<NativeBatchDigest<M>>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"batch digest frame extent overflow"))?;Ok(demand)}

/// 🎟️ Quotes the original native source admission and its separately retained concrete Box frame.
pub fn admit_artifact_batch_digest<M:RetireOwned+ArtifactCanonicalJsonTree+Send+Sync+'static>(edit:&mut Option<Box<Edit<M>>>,grant:RetainedCloneGrant)->Result<Option<(Box<dyn ArtifactStoreBatchDigest<M>>,RetainedCloneProgress)>,ValueError>{
 let demand=artifact_batch_digest_birth_demand::<M>()?;
 if !admitted(demand,grant){return Ok(None);}
 let Some((original,receipt))=ArtifactCanonicalEditSealCursor::admit_original(edit,child(grant))? else{return Ok(None);};
 Ok(Some((Box::new(NativeBatchDigest{original}),RetainedCloneProgress{retained_capacity_bytes:receipt.retained_capacity_bytes+std::mem::size_of::<NativeBatchDigest<M>>(),..receipt})))
}
impl<M:RetireOwned+ArtifactCanonicalJsonTree+Send+Sync+'static> ArtifactStoreBatchDigest<M> for NativeBatchDigest<M>{
 fn next_digest_demand(&self)->Result<RetirementDemand,ValueError>{nested(if self.original.is_ready(){self.original.next_take_demand()}else{self.original.next_demand()?})}
 fn advance_digest(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if !admitted(self.next_digest_demand()?,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}self.original.advance(child(grant))}
 fn ready(&self)->bool{self.original.is_ready()}
 fn take_edit(&mut self,grant:RetainedCloneGrant)->Result<Option<(Box<Edit<M>>,[u8;32],RetainedCloneProgress)>,ValueError>{if !admitted(self.next_digest_demand()?,grant){return Ok(None);}self.original.take_edit(child(grant))}
 fn begin_close(&mut self){self.original.begin_close();}
}
impl<M:RetireOwned+ArtifactCanonicalJsonTree+Send+Sync+'static> ErasedSnapshotRetirement for NativeBatchDigest<M>{
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.original.begin_close();if self.original.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}if !admitted(nested(self.original.next_demand()?)?,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}self.original.advance(child(grant))}
 fn terminal_is_empty(&self)->bool{self.original.terminal_is_empty()}
 fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(nested(self.original.next_demand()?)?.copy_bytes)}
 fn next_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(nested(self.original.next_demand()?)?.capacity_bytes)}
 fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(nested(self.original.next_demand()?)?.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(nested(self.original.next_demand()?)?.depth)}
}
