//! 📸️ Historical checkpoint reads borrow exact ledger positions without moving the live Store cursor.
use super::*;
use semio_framework_os_store_read::{ArtifactCheckpointSelection,ArtifactCheckpointReadSource};
impl<P,Mutation> ArtifactStore<P,Mutation>
where P:ToValue+Send+Sync+'static,Mutation:ToValue+FromValue+protocol::Mutation<P>+OpBinary+OpText+Send+'static{
 pub fn begin_derived_checkpoint_preview(&self,id:&str,grant:RetainedCloneGrant)->Result<Option<(ArtifactDerivedHistoryPreview<P,Mutation>,RetainedCloneProgress)>,ValueError>{
  if id.is_empty()||id.chars().count()>512{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"checkpoint selection exceeds its authored identity bound"))}let bytes=size_of::<ArtifactDerivedHistoryPreview<P,Mutation>>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(None)}self.wire.ensure_durable_group_idle().map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"checkpoint read awaits the current durable group"))?;
  Ok(Some((ArtifactDerivedHistoryPreview{checkpoint:Some(ArtifactCheckpointSelection::new(id)),derived_operation:None,history_source:None,history_input:None,history_issuer:None,history_issuer_close:None,displaced_operation:None,operation_close:None,operation_identity:None,operation_adopted:false,preparation_receipt:Default::default(),plan:None,plan_retirement:None,drafts:protocol::HistoryInputDrafts::new(),target:None,cursor:0,operation:0,state:None,progress:ReplayProgress::default(),registry:self.owners.snapshot_read_leases.clone(),generation:self.generation_now(),revision:self.content_revision_now(),marker:PhantomData,finished:false},RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})))
 }
 pub fn derived_history_preview_retirement_demands(&self,preview:&ArtifactDerivedHistoryPreview<P,Mutation>,body:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{
  let header=|copy_bytes|semio_framework_value::RetirementDemand{copy_bytes,depth:1,..Default::default()};
  if let Some(operation)=preview.derived_operation.as_ref(){if !operation.is_closing(){return Ok(header(2*size_of::<bool>()))}return operation.next_demand(body)}
  if let Some(input)=preview.history_input.as_ref(){return if input.terminal_is_empty(){Ok(header(size_of::<Option<semio_framework_value::retained_clone::RetainedOwnedProjection<Mutation>>>()))}else{input.close_demand(body)}}
  if let Some(source)=preview.history_source.as_ref(){return source.close_demands(&self.log.envelope.vcs.edits,body)}if let Some(issuer)=preview.history_issuer_close.as_ref(){return issuer.demands(body)}if preview.history_issuer.is_some(){return Ok(header(size_of::<Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<Mutation>>>>()))}
  Ok(semio_framework_value::RetirementDemand{capacity_bytes:semio_framework_os_store_read::history_read_admission::history_read_retirement_birth_bytes::<P,Mutation>(),depth:1,..Default::default()})
 }
 pub(super) fn history_preview_edit_slot(&self,preview:&ArtifactDerivedHistoryPreview<P,Mutation>)->Result<semio_framework_os_vcs::ArtifactHistoryKey,VcsError>{if let Some(selection)=preview.checkpoint.as_ref(){return selection.current_edit.and_then(|position|self.log.envelope.vcs.edits.key_at(position)).ok_or(VcsError::HistoryReplaying)}self.applied_revision().applied.get(preview.cursor).and_then(|record|record.ledger_key).ok_or(VcsError::HistoryReplaying)}
 pub(super) fn history_preview_edit(&self,preview:&ArtifactDerivedHistoryPreview<P,Mutation>)->Result<&Edit<Mutation>,VcsError>{if preview.checkpoint.is_some(){return self.log.envelope.vcs.edits.get_key(self.history_preview_edit_slot(preview)?).ok_or(VcsError::HistoryReplaying)}self.applied_edit(preview.cursor)}
 pub(super) fn step_checkpoint_selection(&self,preview:&mut ArtifactDerivedHistoryPreview<P,Mutation>,id:&str,grant:RetainedCloneGrant,deadline:&mut dyn FnMut()->bool)->Result<bool,VcsError>{
  let selection=preview.checkpoint.as_ref().unwrap();if selection.complete||selection.current_edit.is_some(){return Ok(true)}if deadline(){return Ok(false)}
  if preview.state.is_none(){let copy=size_of::<Option<Arc<P>>>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<copy{return Ok(false)}preview.state=Some(self.log.envelope.vcs.genesis.facts().share_snapshot());preview.preparation_receipt=RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()};return Ok(false)}
  let source=CheckpointSource{envelope:&self.log.envelope};preview.preparation_receipt=preview.checkpoint.as_mut().unwrap().step(&source,id,grant).map_err(VcsError::InverseRefused)?;Ok(false)
 }
}

struct CheckpointSource<'a,P,M>{envelope:&'a semio_framework_os_store_space_schema::ArtifactEnvelopeOwners<P,M>}
impl<P,M> ArtifactCheckpointReadSource for CheckpointSource<'_,P,M>{
 fn checkpoint_id(&self,position:usize)->Option<&str>{self.envelope.vcs.checkpoints.get(position).map(|row|row.id.as_str())}
 fn checkpoint_change_id(&self,checkpoint:usize,change:usize)->Option<&str>{self.envelope.vcs.checkpoints.get(checkpoint)?.change_ids.get(change).map(String::as_str)}
 fn change_id(&self,position:usize)->Option<&str>{self.envelope.vcs.changes.get(position).map(|row|row.id.as_str())}
 fn change_edit_id(&self,change:usize,edit:usize)->Option<&str>{self.envelope.vcs.changes.get(change)?.edit_ids.get(edit).map(String::as_str)}
 fn edit_id(&self,position:usize)->Option<&str>{self.envelope.vcs.edits.get(position).map(|row|row.id.as_str())}
}

#[path="order.rs"]
pub mod order;
pub use order::OriginalCheckpointEditOrder;
