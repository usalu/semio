//! 📭️ Staged slice of the `instance-root` band of the mounted artifact instance: its `instance-docs` members (WP6 step S0 of the instance family).
//!
//! 🧵️ The mounted artifact instance `VcsArtifactApp`: its state, its turns and its `PluginApp` implementation.
#[cfg(any(test, feature = "artifact-app-testing"))]
use std::sync::Arc;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework::kernel::ArtifactHandle;
use semio_framework_plugin_decl_views::media_contract::{ArtifactMediaExportCompletion, ArtifactMediaExportCredit, ArtifactMediaExportHandle, ArtifactOutputChunks};
use semio_framework_plugin_decl_app::media_contract::ArtifactMediaExportJobRequest;
use semio_framework_plugin_ownership::store_lifecycle::{ArtifactOwnedDisposer, ArtifactSnapshotCloseLease, ArtifactSnapshotCloseRetention};
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_decl_app::tool_contract::ArtifactReservedToolJob;
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_plugin_decl_emission::{Emit, TransactionPhase};
use semio_framework::{Fault, FaultCode, FaultFrom, FaultOrigin};
use std::future::Future;
use std::mem::ManuallyDrop;
use store::{MemberFactory, Mutation, SpaceMember};
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_value::{RetirementDemand, ValueError};
use semio_framework_plugin_operation::{ToolCancellationLease, ToolOperationKey};
use semio_framework_ui_contract::*;
use crate::docs_maintenance_ladder::item;

#[doc(hidden)]
pub struct MediaExportPreparation<A:ArtifactApp>{
 pub checkpoint:Option<super::native_checkpoint_export::NativeCheckpointExportSource<A>>,pub checkpoint_identity:Option<(usize,usize)>,
 pub canonical_revision:[u8;32],pub metadata:ManuallyDrop<Option<[Vec<u8>;6]>>,pub(crate) metadata_stage:usize,pub metadata_close:ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<[Vec<u8>;6]>>>,
 pub request_close:ManuallyDrop<Option<semio_framework_plugin_decl_app::media_contract::ArtifactMediaExportRequestRetirement<A>>>,pub parts:ManuallyDrop<Option<semio_framework_plugin_document_media::media_export::MediaExportOwnedParts>>,pub(crate) parts_close:ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<semio_framework_plugin_document_media::media_export::MediaExportOwnedParts>>>,pub handle:ManuallyDrop<Option<ArtifactMediaExportHandle>>,pub handle_close:ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<ArtifactMediaExportHandle>>>,pub operation:Option<semio_framework_job::Operation>,pub(crate) body_ready:bool,pub(crate) operation_bound:bool,pub assembled:bool,pub published:bool,
 pub snapshot_source:semio_framework_plugin_ownership::store_lifecycle::ArtifactMediaSnapshotSource<A::Snapshot>,
 pub original_read:ManuallyDrop<Option<store::os_store::snapshot_read::SnapshotRead<A::Snapshot>>>,pub original_read_close:ManuallyDrop<Option<ControlledRetirement<store::os_store::snapshot_read::SnapshotRead<A::Snapshot>>>>,pub original_children:ManuallyDrop<Option<semio_framework_plugin_decl_views::ChildContentView>>,pub original_children_close:ManuallyDrop<Option<ControlledRetirement<semio_framework_plugin_decl_views::ChildContentView>>>,pub child_root_identity:Option<usize>,
 pub snapshot:ManuallyDrop<Option<std::sync::Arc<A::Snapshot>>>,pub(crate) snapshot_lease:ManuallyDrop<Option<ArtifactSnapshotCloseLease<A::Snapshot>>>,pub(crate) snapshot_retention:ManuallyDrop<Option<ArtifactSnapshotCloseRetention>>,
 pub request:ManuallyDrop<Option<ArtifactMediaExportJobRequest<A>>>,pub body:ManuallyDrop<Option<Box<dyn semio_framework_plugin_decl_app::tool_contract::ArtifactReservedJob+Send>>>,pub producer:ManuallyDrop<Option<ArtifactReservedToolJob>>,
 pub source_identity:(semio_framework_job::OperationId,semio_framework_job::Generation,usize),pub(crate) port_identity:(usize,usize),pub key:ToolOperationKey,
 pub lease:Option<ToolCancellationLease>,pub retirement:Option<Box<semio_framework_plugin_operation::ToolCancellationLeaseRetirement>>,
 pub completion:Option<ArtifactMediaExportCompletion>,pub completion_close:Option<semio_framework_value::retirement::controlled::ControlledRetirement<ArtifactMediaExportCompletion>>,
 pub credit:Option<ArtifactMediaExportCredit>,pub(crate) credit_close:Option<semio_framework_value::retirement::controlled::ControlledRetirement<ArtifactMediaExportCredit>>,
 pub chunks:Option<ArtifactOutputChunks>,pub(crate) chunks_close:Option<semio_framework_value::retirement::controlled::ControlledRetirement<ArtifactOutputChunks>>,
}

impl<A:ArtifactApp> MediaExportPreparation<A>{
 pub fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  use semio_framework_job::InteractiveJob;
  let one=RetirementDemand{depth:1,..Default::default()};
  if let Some(job)=self.producer.as_ref(){return if job.terminal_is_empty(){Ok(one)}else{Ok(RetirementDemand{copy_bytes:job.next_close_copy_byte_demand()?,capacity_bytes:job.next_close_capacity_byte_demand(body)?,release_bytes:job.next_close_release_byte_demand()?,depth:job.next_close_depth_demand()?.max(1)})}}
  if let Some(job)=self.body.as_ref(){return if job.terminal_is_empty(){Ok(RetirementDemand{release_bytes:size_of_val(&**job),..one})}else{Ok(RetirementDemand{copy_bytes:job.next_close_copy_byte_demand()?,capacity_bytes:job.next_close_capacity_byte_demand(body)?,release_bytes:job.next_close_release_byte_demand()?,depth:job.next_close_depth_demand()?.max(1)})}}
  if let Some(owner)=self.request_close.as_ref(){return if owner.terminal_is_empty(){Ok(one)}else{owner.retirement_demands(body)}}
  if self.request.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<semio_framework_plugin_decl_app::media_contract::ArtifactMediaExportRequestRetirement<A>>(),..one})}
  if self.original_read.is_some()||self.original_read_close.is_some(){return semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_demand(&self.original_read,&self.original_read_close,body)}
  if self.original_children.is_some()||self.original_children_close.is_some(){return semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_demand(&self.original_children,&self.original_children_close,body)}
  if self.parts.is_some()||self.parts_close.is_some(){return semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_demand(&self.parts,&self.parts_close,body)}
  if self.handle.is_some()||self.handle_close.is_some(){return semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_demand(&self.handle,&self.handle_close,body)}
  if self.metadata.is_some()||self.metadata_close.is_some(){return semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_demand(&self.metadata,&self.metadata_close,body)}
  if self.snapshot.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<std::sync::Arc<A::Snapshot>>(),..one})}
  if let Some(lease)=self.snapshot_lease.as_ref(){return Ok(if lease.terminal_is_empty(){one}else{lease.retirement_demands()})}
  if self.completion.is_some()||self.completion_close.is_some(){return semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_demand(&self.completion,&self.completion_close,body)}
  if self.credit.is_some()||self.credit_close.is_some(){return semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_demand(&self.credit,&self.credit_close,body)}
  if self.chunks.is_some()||self.chunks_close.is_some(){return semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_demand(&self.chunks,&self.chunks_close,body)}
  if self.operation.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<semio_framework_job::Operation>(),..one})}
  Ok(one)
 }
 pub(crate) fn close_material_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,semio_framework_job::OriginalTurnFailure>{
  use semio_framework_job::InteractiveJob;
  let item=RetainedCloneProgress{copied_items:1,..Default::default()};
  if let Some(job)=self.producer.as_mut(){if job.terminal_is_empty(){self.producer.take();return Ok(Some(item))}job.begin_close();return job.close_step(cx).map(|step|Some(step.progress()))}
  if let Some(job)=self.body.as_mut(){if job.terminal_is_empty(){let bytes=size_of_val(&**job);self.body.take();return Ok(Some(RetainedCloneProgress{released_bytes:bytes,..item}))}job.begin_close();return job.close_step(cx).map(|step|Some(step.progress()))}
  if let Some(owner)=self.request_close.as_mut(){if owner.terminal_is_empty(){self.request_close.take();return Ok(Some(item))}return owner.close_step(grant).map(|step|Some(step.progress())).map_err(Into::into)}
  if let Some(request)=self.request.take(){*self.request_close=Some(semio_framework_plugin_decl_app::media_contract::ArtifactMediaExportRequestRetirement::new(request));return Ok(Some(RetainedCloneProgress{copied_bytes:size_of::<semio_framework_plugin_decl_app::media_contract::ArtifactMediaExportRequestRetirement<A>>(),..item}))}
  if self.original_read.is_some()||self.original_read_close.is_some(){return semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_step(&mut self.original_read,&mut self.original_read_close,grant).map(|step|Some(step.progress())).map_err(Into::into)}
  if self.original_children.is_some()||self.original_children_close.is_some(){return semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_step(&mut self.original_children,&mut self.original_children_close,grant).map(|step|Some(step.progress())).map_err(Into::into)}
  if self.parts.is_some()||self.parts_close.is_some(){return semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_step(&mut self.parts,&mut self.parts_close,grant).map(|step|Some(step.progress())).map_err(Into::into)}
  if self.handle.is_some()||self.handle_close.is_some(){return semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_step(&mut self.handle,&mut self.handle_close,grant).map(|step|Some(step.progress())).map_err(Into::into)}
  if self.metadata.is_some()||self.metadata_close.is_some(){return semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_step(&mut self.metadata,&mut self.metadata_close,grant).map(|step|Some(step.progress())).map_err(Into::into)}
  if let Some(snapshot)=self.snapshot.as_ref(){if !self.snapshot_lease.as_ref().is_some_and(|lease|lease.can_release(snapshot)){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"pending media snapshot lost its paired supplier").into())}self.snapshot.take();return Ok(Some(RetainedCloneProgress{copied_bytes:size_of::<std::sync::Arc<A::Snapshot>>(),..item}))}
  if let Some(lease)=self.snapshot_lease.as_mut(){if lease.terminal_is_empty(){self.snapshot_lease.take();return Ok(Some(item))}return lease.close_step(grant).map(|step|Some(step.progress())).map_err(Into::into)}
  if self.completion.is_some()||self.completion_close.is_some(){return semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_step(&mut self.completion,&mut self.completion_close,grant).map(|step|Some(step.progress())).map_err(Into::into)}
  if self.credit.is_some()||self.credit_close.is_some(){return semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_step(&mut self.credit,&mut self.credit_close,grant).map(|step|Some(step.progress())).map_err(Into::into)}
  if self.chunks.is_some()||self.chunks_close.is_some(){return semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_step(&mut self.chunks,&mut self.chunks_close,grant).map(|step|Some(step.progress())).map_err(Into::into)}
  if self.operation.is_some(){self.operation.take();return Ok(Some(RetainedCloneProgress{copied_bytes:size_of::<semio_framework_job::Operation>(),..item}))}
  Ok(None)
 }
}

/// 🆔️ Deterministic session-local `ArtifactHandle` for a CHILD's real (string) artifact id.
/// `result_from_last_edit`'s own `ArtifactHandle(meta.instance_id as u128)` only identifies the
/// ONE wasm-hosted document this plugin instance runs (the parent) — a child pulled in through
/// `dispatch_emit_group` is a different document with its own string `artifact_id` and no
/// instance id of its own, so `KernelMutation.document` needs a handle derived from THAT id
/// instead, so history correctly attributes a child's edit to the child, not the parent. FNV-1a
/// (128-bit, two interleaved 64-bit lanes) rather than `std::collections::hash_map::DefaultHasher`
/// — the latter's hash is only guaranteed stable for one process/compiler invocation, not the
/// byte-identical value the SAME child id must always map to.
#[doc(hidden)]
pub async fn artifact_handle_of(artifact_id: &str) -> ArtifactHandle {
    const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const PRIME: u64 = 0x0000_0100_0000_01b3;
    let mut lane0 = OFFSET;
    let mut lane1 = OFFSET ^ 0x9e37_79b9_7f4a_7c15;
    for byte in artifact_id.as_bytes() {
        lane0 = (lane0 ^ *byte as u64).wrapping_mul(PRIME);
        lane1 = (lane1 ^ (*byte as u64).rotate_left(7)).wrapping_mul(PRIME);
    }
    ArtifactHandle(((lane0 as u128) << 64) | lane1 as u128)
}

/// 🔒️ Frozen fault (contract §2.3): exact code `viewer.read-only`, `FaultOrigin::Framework` — a
/// mutating verb reached a `Viewer`-role `VcsArtifactApp` instance, either through
/// `dispatch_action`/`dispatch_import_media`'s string-verb guard or `dispatch_emit`'s non-empty
/// `artifact_mutations` backstop (the latter is unreachable through `ViewerApp`'s structurally
/// closed `ViewEmit`, contract §2.2 — this exists so a hand-written runtime `ArtifactApp` impl
/// that ignores `ViewEmit` is safe too, not "redundant").
// 🚫️async: E1 pure error constructor consumed pervasively by sync closures — see R9.
#[doc(hidden)]
pub fn viewer_read_only_fault(verb: &str) -> Fault {
    Fault::new(FaultOrigin::Framework, FaultCode::new("viewer.read-only"), format!("'{verb}' is not permitted on a viewer instance (contract §2.3)"))
}

/// 🛠️ A committed tool transaction is one plain edit per touched member (the parent and every owned child it carries,
/// all stamped with the same ref — design §12); a streamed one is one open edit of the app's own document that its ticks
/// grow (design §15). Refused instead of silently losing a stamp or a trace: a stream or abort naming no transaction, a
/// stream or abort across owned children, a stream carrying operations with foreign steps, and an abort carrying
/// operations.
#[doc(hidden)]
pub fn tool_transaction_shape_fault(verb: &str, transaction: Option<&protocol::TransactionRef>, phase: TransactionPhase, children: bool, carried: bool, foreign: bool) -> Option<Fault> {
    let refuse = |detail: String| Some(Fault::new(FaultOrigin::Framework, FaultCode::new("toolTransaction.shape"), detail));
    let streamed = matches!(phase, TransactionPhase::Stream | TransactionPhase::Abort);
    match transaction {
        None if streamed => refuse(format!("verb {verb:?} streamed or aborted no tool transaction")),
        Some(transaction) if streamed && children => refuse(format!("verb {verb:?} streamed or aborted tool transaction {:?} across owned children; a streamed transaction grows the app's own document", transaction.id)),
        Some(transaction) if phase == TransactionPhase::Stream && foreign => refuse(format!("verb {verb:?} streamed operations with foreign steps into tool transaction {:?}; a streamed transaction grows the app's own document", transaction.id)),
        Some(transaction) if phase == TransactionPhase::Abort && carried => refuse(format!("verb {verb:?} aborted tool transaction {:?} with mutations; an abort carries none", transaction.id)),
        _ => None,
    }
}

#[doc(hidden)]
pub fn typed_operation_document_is_fresh(operation: &semio_framework_job::Operation, canonical_revision: [u8; 32], live_revision: [u8; 32], live_generation: u64) -> bool {
    let live_short = semio_framework_job::RevisionId(u64::from_be_bytes(live_revision[..8].try_into().expect("revision lane width")));
    canonical_revision == live_revision && matches!(semio_framework_job::validate_commit(operation, live_short, semio_framework_job::Generation(live_generation)), semio_framework_job::CommitValidation::Accepted)
}
