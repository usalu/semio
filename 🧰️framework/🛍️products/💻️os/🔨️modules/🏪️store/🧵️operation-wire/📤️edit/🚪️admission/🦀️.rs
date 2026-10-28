//! 🚪️ The typed Store vocabulary admits one actual original message owner behind its defining interface.
use super::*;
use semio_framework_value::retirement::RetireOwned;
use semio_framework_os_store_one_item_publication::batch_digest::ArtifactStoreBatchMessage;
struct NativeMessage<M:RetireOwned+Send+Sync+'static>{original:ArtifactOriginalEditMessage<M>}
fn birth<M:RetireOwned+Send+Sync+'static>()->RetirementDemand{let mut d=ArtifactOriginalEditMessage::<M>::birth_demands();d.capacity_bytes+=size_of::<NativeMessage<M>>();d.copy_bytes+=size_of::<Box<dyn ArtifactStoreBatchMessage<M>>>();d}
fn admit<M:RetireOwned+Send+Sync+'static>(edit:&mut Option<Box<Edit<M>>>,wire:ArtifactOperationWire<M>,grant:RetainedCloneGrant)->Result<Option<(Box<dyn ArtifactStoreBatchMessage<M>>,RetainedCloneProgress)>,ValueError>{let d=birth::<M>();if !retained_clone_grant_admits(grant,d){return Ok(None)}let Some((original,mut p))=ArtifactOriginalEditMessage::admit(edit,wire,grant)?else{return Ok(None)};p.retained_capacity_bytes+=size_of::<NativeMessage<M>>();p.copied_bytes+=size_of::<Box<dyn ArtifactStoreBatchMessage<M>>>();Ok(Some((Box::new(NativeMessage{original}),p)))}
/// 🧵️ Installs original precommit custody for a vocabulary with actual typed retirement.
pub fn with_original_edit_message<M:RetireOwned+Send+Sync+'static>(wire:ArtifactOperationWire<M>)->ArtifactOperationWire<M>{wire.with_original_message(birth::<M>,admit::<M>)}
impl<M:RetireOwned+Send+Sync+'static> ArtifactStoreBatchMessage<M> for NativeMessage<M>{
 fn next_message_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.original.demands(body)}
 fn advance_message(&mut self,document:&str,schema:&str,observed:Option<&str>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.original.advance(OriginalEditWireHeaders{document,schema,observed},grant)}
 fn ready(&self)->bool{self.original.ready()}
 fn take_edit_message(&mut self,grant:RetainedCloneGrant)->Result<Option<(Box<Edit<M>>,PagedBytes<{isize::MAX as usize}>,RetainedCloneProgress)>,ValueError>{self.original.take(grant)}
 fn begin_close(&mut self){self.original.begin_close()}
}
impl<M:RetireOwned+Send+Sync+'static> ErasedSnapshotRetirement for NativeMessage<M>{
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.original.close_demands(body)}
 fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.original.close_demands(4096)?.copy_bytes)}
 fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.original.close_demands(body)?.capacity_bytes)}
 fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.original.close_demands(4096)?.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.original.close_demands(4096)?.depth)}
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.original.begin_close();self.original.close_step(grant)}
 fn terminal_is_empty(&self)->bool{self.original.terminal_is_empty()}
}
