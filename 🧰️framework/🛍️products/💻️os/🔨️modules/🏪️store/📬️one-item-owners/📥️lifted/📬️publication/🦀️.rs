//! 📥️ Items of `one_item_publication` that belong to `os-store-one-item-batch` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/🎮️one-item/📬️publication/🦀️.rs`.
use semio_framework_os_store_one_item_publication::batch_digest::{ArtifactStoreBatchDigest,ArtifactStoreBatchMessage};
use crate::lifted_member_group as member_group;
use semio_framework_os_store_ledger::edit_retirement::ArtifactStoreDecodedEditRetirement;
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementFactory};
use semio_framework_value::retained_clone::{RetainedCloneBirthDemand, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::ValueError;
use semio_framework_os_spr::HybridLogicalTimestamp;
use semio_framework_os_spr::{Edit, Mutation, MutationMeta};
use std::sync::Arc;
use semio_framework_value_snapshot_read::{SnapshotRead, artifact_retirement_box_close_step, artifact_retirement_box_demands};
use semio_framework_os_store_ledger::lifted_snapshot_read::ArtifactStoreDisplacedOwnerReservation;
use semio_framework_value_snapshot_read::{SnapshotReadRegistryAliasRetirement,snapshot_registry_alias_demands,snapshot_registry_alias_close_step};
use semio_framework_value::retirement::frame::admit_retirement_frame as admit_artifact_retirement;
use semio_framework_os_store_ledger::ArtifactStoreHistoryCommitReservation;
use semio_framework_os_store_one_item_publication::prepared_retirement::ArtifactStorePreparedRetirement;
use semio_framework_os_store_one_item_publication::lifted_prepared_operation_wire::ArtifactPreparedOperationSource;
use semio_framework_os_store_one_item::{ARTIFACT_STORE_ONE_ITEM_MAXIMUM_RETRIES, ArtifactStoreOneItemCheckpoint, ArtifactStoreOneItemFootprint, ArtifactStoreOneItemGrant};
use semio_framework_os_store_command::HistoryLane;
use semio_framework_os_store_one_item::LaneItemReceipt;
/// 🔄 Persistent store-side operation phases. Publication and ACK are distinct turns; close
/// remains observable until the domain owner proves exact terminal emptiness.
use semio_framework_os_store_one_item::ArtifactStoreOneItemPublicationPhase;
use semio_framework_os_store_one_item_publication::ArtifactStoreOneItemLiveAuthority;
use semio_framework_os_store_one_item_publication::ArtifactStoreOneItemPrepared;
use semio_framework_os_store_one_item_publication::ArtifactStoreOneItemPreparationRequest;
use semio_framework_os_store_one_item_publication::ArtifactStoreOneItemPreparation;
use semio_framework_os_store_one_item_publication::ArtifactStoreBatchSource;
pub use semio_framework_os_store_one_item_publication::lifted_publication::{MemberStoreOneItemWire, MemberStoreOneItemWirePreparationFactory, MemberStoreOneItemWireRequest};






/// 🪪 Erased retained publication; only its exact member may advance the typed candidate.
pub trait ErasedMemberStoreOneItemPublication: Send {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn phase(&self) -> ArtifactStoreOneItemPublicationPhase;
    fn progress(&self) -> ArtifactStoreOneItemCheckpoint;
    fn prepared_edit_id(&self) -> Option<&str>;
    fn admit_original_origins(&mut self, origins:&mut Option<std::collections::VecDeque<protocol::MutationOrigin>>, grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
    fn prepared_operation_count(&self, inverse: bool) -> Option<usize>;
    fn prepared_operation(&self, inverse: bool, index: usize) -> Option<&dyn std::any::Any>;
    fn prepared_operation_metadata(&self, index: usize) -> Option<&MutationMeta>;
    fn prepared_operation_schema_parts(&self, index: usize) -> Option<(&str, &str)>;
    fn prepared_operation_wire_source(&self, inverse: bool, index: usize) -> Option<ArtifactPreparedOperationSource<'_>>;
    fn next_group_byte_demand(&self) -> usize;
    fn retirement_demands(&self, maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError>;
    fn fault(&self) -> Option<&str>;
    fn preparation_refusal(&self) -> Option<&ValueError>;
    fn retry(&mut self) -> bool;
    fn acknowledge(&mut self) -> bool;
    fn begin_close(&mut self);
    fn close_original_step(&mut self,original:&mut semio_framework_value::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
    fn terminal_is_empty(&self) -> bool;
}

pub struct MemberStoreOneItemPublication<P, Mutation> {
    pub member: Option<SnapshotReadRegistryAliasRetirement>,
    pub publication: ArtifactStoreBatchPublication<P, Mutation>,
    pub group_history: Option<ArtifactStoreHistoryCommitReservation>,
    pub group_displaced: Option<ArtifactStoreDisplacedOwnerReservation>,
}

impl<P: Send + Sync + 'static, Mutation: Send + 'static> ErasedMemberStoreOneItemPublication for MemberStoreOneItemPublication<P, Mutation> {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn phase(&self) -> ArtifactStoreOneItemPublicationPhase {
        self.publication.phase()
    }

    fn progress(&self) -> ArtifactStoreOneItemCheckpoint {
        self.publication.progress()
    }

    fn admit_original_origins(&mut self, origins:&mut Option<std::collections::VecDeque<protocol::MutationOrigin>>, grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.publication.admit_original_origins(origins,grant)}
    fn prepared_edit_id(&self) -> Option<&str> { self.publication.stage.as_ref()?.edit.as_ref().map(|edit| edit.id.as_str()) }
    fn prepared_operation_count(&self, inverse: bool) -> Option<usize> { self.publication.stage.as_ref()?.edit.as_ref().map(|edit| if inverse { edit.inverse.len() } else { edit.forwards.len() }) }
    fn prepared_operation(&self, inverse: bool, index: usize) -> Option<&dyn std::any::Any> {
        let stage = self.publication.stage.as_ref()?;
        if inverse { stage.edit.as_ref()?.inverse.get(index).map(|mutation| mutation as &dyn std::any::Any) } else { stage.edit.as_ref()?.forwards.get(index).map(|mutation| mutation as &dyn std::any::Any) }
    }
    fn prepared_operation_metadata(&self, index: usize) -> Option<&MutationMeta> { self.publication.stage.as_ref()?.edit.as_ref()?.mutation_meta.get(index) }
    fn prepared_operation_schema_parts(&self, index: usize) -> Option<(&str, &str)> {
        if self.publication.close_started || self.publication.published || self.publication.fault.is_some() { return None; }
        let mutation = self.publication.stage.as_ref()?.edit.as_ref()?.forwards.get(index)?;
        self.publication.source.as_ref()?.operation_schema_parts(mutation)
    }
    fn prepared_operation_wire_source(&self, inverse: bool, index: usize) -> Option<ArtifactPreparedOperationSource<'_>> {
        if self.publication.close_started || self.publication.published || self.publication.fault.is_some() { return None; }
        let stage = self.publication.stage.as_ref()?;
        let mutation = if inverse { stage.edit.as_ref()?.inverse.get(index)? } else { stage.edit.as_ref()?.forwards.get(index)? };
        self.publication.source.as_ref()?.operation_wire_source(mutation)
    }

    fn next_group_byte_demand(&self) -> usize { self.publication.next_group_byte_demand() }
    fn retirement_demands(&self, maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if self.group_history.is_some() || self.group_displaced.is_some() { return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() }); }
        if !self.publication.terminal_is_empty() { let mut demand = self.publication.retirement_demands(maximum_copy_bytes)?; demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "member publication retirement depth overflow"))?; return Ok(demand); }
        snapshot_registry_alias_demands(&self.member)
    }

    fn fault(&self) -> Option<&str> { self.publication.fault() }
    fn preparation_refusal(&self) -> Option<&ValueError> { self.publication.preparation_refusal() }

    fn retry(&mut self) -> bool {
        self.publication.retry()
    }

    fn acknowledge(&mut self) -> bool {
        self.publication.acknowledge()
    }

    fn begin_close(&mut self) {
        self.publication.begin_close();
    }

    fn close_original_step(&mut self,original:&mut semio_framework_value::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if self.group_history.is_some()||self.group_displaced.is_some()||grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()))}if !self.publication.terminal_is_empty(){let d=self.retirement_demands(grant.maximum_copy_bytes)?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};return self.publication.close_original_step(original,child).map(|s|RetainedCloneStep::Progress(s.progress()))}let d=snapshot_registry_alias_demands(&self.member)?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}original.charge(d.capacity_bytes)?;snapshot_registry_alias_close_step(&mut self.member,grant)}
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep};
        if self.group_history.is_some() || self.group_displaced.is_some() || grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.publication.terminal_is_empty() {
            if grant.maximum_depth < self.retirement_demands(grant.maximum_copy_bytes)?.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "member publication exceeds admitted retirement depth")); }
            let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
            return self.publication.close_step(child).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        snapshot_registry_alias_close_step(&mut self.member, grant)
    }

    fn terminal_is_empty(&self) -> bool {
        self.member.is_none() && self.publication.terminal_is_empty() && self.group_history.is_none() && self.group_displaced.is_none()
    }
}

impl<P, Mutation> Drop for MemberStoreOneItemPublication<P, Mutation> {
    fn drop(&mut self) {
        assert!(
            std::thread::panicking() || (self.member.is_none() && self.publication.terminal_is_empty() && self.group_history.is_none() && self.group_displaced.is_none()),
            "member publication reached Drop before exact member, reservation, and candidate retirement"
        );
    }
}

/// 🚫 Exact owners returned when the app has no retained factory, preflight rejects, or the
/// store's live base authority no longer matches. The whole admitted gesture comes back in the
/// order it was offered, so the caller can retry or retire it without reconstructing anything.
#[derive(Debug)]
pub struct ArtifactStoreBatchAdmissionRejected<Mutation> {
    pub reason: String,
    pub mutations: Vec<Mutation>,
}

impl<Mutation> ArtifactStoreBatchAdmissionRejected<Mutation> {
    pub fn into_owners(self) -> (String, Vec<Mutation>) {
        (self.reason, self.mutations)
    }
}

/// 🧾️ One gesture's staged edit. Every admitted mutation is folded into these exact owners against
/// the running post root, so the whole gesture commits as ONE `Edit` in ONE history ledger slot and
/// is ONE undo step. It carries its own retirement authority: a cancelled batch retires the staged
/// forwards, inverses, metadata and post root one owner per bounded turn.
pub struct ArtifactStoreBatchStage<P, Mutation> {
    pub diagnostics: semio_framework_os_store_one_item_publication::diagnostics::ArtifactStoreDiagnosticStage,
    pub diagnostic_row: usize,
    pub handoff_ids: [Option<semio_framework_value::paged::PagedUtf8<{usize::MAX}>>; 2],
    pub handoff_cursors: [Option<semio_framework_value::paged::PagedUtf8AppendCursor>; 2],
    pub unit_flags: Vec<(usize, bool)>,
    pub folding: Option<Box<ArtifactStorePreparedRetirement<P, Mutation>>>,
    pub fold_phase: u8,
    pub digest_owner: Option<Box<dyn ArtifactStoreBatchDigest<Mutation>>>,
    pub digest_complete: bool,
    pub commit_metadata:Option<member_group::BatchCommitMetadata>,
    pub displaced_report:Option<semio_framework_os_store_ledger::ArtifactStorePendingReportRetirement>,
    pub message_owner: Option<Box<dyn ArtifactStoreBatchMessage<Mutation>>>,
    pub message: Option<semio_framework_os_store_one_item_publication::batch_digest::OriginalCausalMessageBytes>,
    pub message_complete: bool,
    pub observed_position: Option<usize>,
    pub observed_source: Option<semio_framework_os_vcs::ArtifactHistoryKey>,
    pub observed_complete: bool,
    pub seed_owner: Option<protocol::causal::original_seeds::OriginalMutationDagSeeds>,
    pub seed_pending: Option<protocol::MutationId>,
    pub seed_retiring: Option<semio_framework_value::retirement::controlled::ControlledRetirement<String>>,
    pub seed_index: usize,
    pub edit: Option<Box<Edit<Mutation>>>,
    pub pending_edit: Option<Edit<Mutation>>,
    pub post: Option<Arc<P>>,
    pub next_clock: HybridLogicalTimestamp,
    pub local_actor: Option<semio_framework_value::paged::PagedUtf8<{usize::MAX}>>,
    pub applied_edit_id: String,
    pub tail_edit_id: String,
    pub digest: [u8; 32],
    pub folded: usize,
    pub completed_items: u32,
    pub completed_bytes: u64,
    pub mutation_retirement: Option<Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>>,
    pub snapshot_retirement: Option<Arc<dyn SnapshotRetirementFactory<P>>>,
    pub retiring: Option<Box<dyn ErasedSnapshotRetirement>>,
    pub factory_close: [Option<semio_framework_value::FactoryAuthority>; 2],
}

impl<P, Mutation> ArtifactStoreBatchStage<P, Mutation> {
    pub fn checkpoint(&self) -> ArtifactStoreOneItemCheckpoint {
        ArtifactStoreOneItemCheckpoint { cursor: self.folded as u32, completed_items: self.completed_items, completed_bytes: self.completed_bytes, digest: self.digest }
    }

    pub fn retirement_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError>
    where P: Send + Sync + 'static, Mutation: Send + 'static,
    {
        use semio_framework_value::RetirementDemand;
        let nested = |mut demand: RetirementDemand| -> Result<RetirementDemand, ValueError> { demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "batch stage depth overflow"))?; Ok(demand) };
        if self.commit_metadata.is_some()||self.seed_owner.is_some()||self.seed_pending.is_some()||self.seed_retiring.is_some(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original batch metadata and causal custody require the original Store batch abort port"));}
        if !self.diagnostics.terminal_is_empty() { return nested(self.diagnostics.close_demands(body)?); }
        if let Some(owner)=self.displaced_report.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of_val(&self.displaced_report),depth:1,..Default::default()})}else{nested(owner.next_demand(body)?)};}
        if let Some(owner) = self.retiring.as_ref() { return nested(artifact_retirement_box_demands(owner, body)?); }
        if let Some(owner) = self.handoff_cursors.iter().find_map(Option::as_ref) { return nested(owner.next_demand(body)?); }
        if self.handoff_ids.iter().any(Option::is_some) { return Ok(RetirementDemand { copy_bytes: size_of_val(&self.handoff_ids), capacity_bytes: semio_framework_value::retirement::owned_retirement_birth_bytes::<(Option<semio_framework_value::paged::PagedUtf8<{usize::MAX}>>, Option<semio_framework_value::paged::PagedUtf8<{usize::MAX}>>)>(), depth: 2, ..Default::default() }); }
        if let Some(owner) = self.retiring.as_ref() { return nested(artifact_retirement_box_demands(owner, body)?); }
        if self.message_owner.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Box<dyn ArtifactStoreBatchMessage<Mutation>>>(),depth:1,..Default::default()});}
        if self.message.is_some(){return Ok(RetirementDemand{capacity_bytes:semio_framework_value::retirement::owned_retirement_birth_bytes::<semio_framework_os_store_one_item_publication::batch_digest::OriginalCausalMessageBytes>(),depth:2,..Default::default()});}
        if self.digest_owner.is_some() { return Ok(RetirementDemand { copy_bytes: size_of::<Box<dyn ArtifactStoreBatchDigest<Mutation>>>(), depth: 1, ..Default::default() }); }
        if self.folding.is_some() { return Ok(RetirementDemand { copy_bytes: size_of::<Box<ArtifactStorePreparedRetirement<P, Mutation>>>(), depth: 1, ..Default::default() }); }
        if self.edit.is_some() { return Ok(RetirementDemand { copy_bytes:size_of::<Edit<Mutation>>(), release_bytes: size_of::<Edit<Mutation>>(), depth: 1, ..Default::default() }); }
        if self.pending_edit.is_some() { return Ok(RetirementDemand { capacity_bytes: size_of::<ArtifactStoreDecodedEditRetirement<Mutation>>(), depth: 2, ..Default::default() }); }
        if self.unit_flags.capacity() != 0 { return Ok(RetirementDemand { release_bytes: self.unit_flags.capacity().checked_mul(size_of::<(usize, bool)>()).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "batch flags layout overflow"))?, depth: 1, ..Default::default() }); }
        if let Some(post) = self.post.as_ref() { return Ok(RetirementDemand { capacity_bytes: self.snapshot_retirement.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "batch stage lacks original snapshot factory"))?.retirement_birth_bytes(post), depth: 2, ..Default::default() }); }
        if let Some(value) = [&self.applied_edit_id, &self.tail_edit_id].into_iter().find(|value| value.capacity() != 0) { return Ok(RetirementDemand { release_bytes: value.capacity(), depth: 1, ..Default::default() }); }
        if self.local_actor.is_some() { return Ok(RetirementDemand { capacity_bytes: semio_framework_value::retirement::owned_retirement_birth_bytes::<semio_framework_value::paged::PagedUtf8<{usize::MAX}>>(), depth: 2, ..Default::default() }); }
        if self.mutation_retirement.is_some() || self.snapshot_retirement.is_some() { return Ok(RetirementDemand { copy_bytes: size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(), depth: 1, ..Default::default() }); }
        if let Some(factory) = self.factory_close.iter().find_map(Option::as_ref) { return nested(factory.demands(body)?); }
        Ok(Default::default())
    }
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>
    where P: Send + Sync + 'static, Mutation: Send + 'static,
    {
        use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep, RetainedCloneProgress};
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let demand = self.retirement_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "batch stage exceeds admitted depth")); }
        if demand.copy_bytes > grant.maximum_copy_bytes || demand.capacity_bytes > grant.maximum_capacity_bytes || demand.release_bytes > grant.maximum_release_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if !self.diagnostics.terminal_is_empty() { self.diagnostics.begin_close(); return self.diagnostics.close_step(child).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if let Some(owner)=self.displaced_report.as_mut(){if !owner.terminal_is_empty(){return owner.close_step(child).map(|step|RetainedCloneStep::Progress(step.progress()))}self.displaced_report=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}
        if self.retiring.is_some() { return artifact_retirement_box_close_step(&mut self.retiring, child).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if let Some(slot) = self.handoff_cursors.iter_mut().find(|slot| slot.is_some()) { let owner=slot.as_mut().unwrap(); owner.begin_close(); let step=owner.close_step(child)?; if owner.terminal_is_empty() { *slot=None; } return Ok(RetainedCloneStep::Progress(step.progress())); }
        if self.handoff_ids.iter().any(Option::is_some) { let [a,b]=std::mem::take(&mut self.handoff_ids); return match semio_framework_value::retirement::admit_owned_retirement((a,b),child) { Ok((owner,receipt))=>{self.retiring=Some(owner);Ok(RetainedCloneStep::Progress(receipt))}, Err((error,(a,b)))=>{self.handoff_ids=[a,b];Err(error)} }; }
        if self.retiring.is_some() { return artifact_retirement_box_close_step(&mut self.retiring, child).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if let Some(mut original)=self.message_owner.take(){original.begin_close();self.retiring=Some(original);return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}
        if let Some(original)=self.message.take(){return match semio_framework_value::retirement::admit_owned_retirement(original,child){Ok((owner,p))=>{self.retiring=Some(owner);Ok(RetainedCloneStep::Progress(p))},Err((error,original))=>{self.message=Some(original);Err(error)}};}
        if let Some(mut original)=self.digest_owner.take(){original.begin_close();self.retiring=Some(original);return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}
        if let Some(original) = self.folding.take() { self.retiring = Some(original); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })); }
        if self.edit.is_some() {
            let original = self.edit.take().expect("observed original staged edit");
            self.pending_edit = Some(*original);
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes:demand.copy_bytes,released_bytes: demand.release_bytes, ..Default::default() }));
        }
        if let Some(edit) = self.pending_edit.take() {
            let factory = self.mutation_retirement.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "batch stage lacks original mutation factory"));
            let factory = match factory { Ok(factory) => factory, Err(error) => { self.pending_edit = Some(edit); return Err(error); } };
            return match admit_artifact_retirement(edit, child, |edit| ArtifactStoreDecodedEditRetirement::new(edit, factory.clone())) {
                Ok((owner, progress)) => { self.retiring = Some(owner); Ok(RetainedCloneStep::Progress(progress)) },
                Err((error, original)) => { self.pending_edit = Some(original); Err(error) },
            };
        }
        if self.unit_flags.capacity() != 0 { self.unit_flags = Vec::new(); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..Default::default() })); }
        if let Some(post) = self.post.take() {
            return match self.snapshot_retirement.as_ref().expect("observed original snapshot factory").retire(post, child) {
                Ok((owner, progress)) => { self.retiring = Some(owner); if !progress.fits(child) || progress.retained_capacity_bytes != demand.capacity_bytes { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "batch stage constructor changed its receipt")); } Ok(RetainedCloneStep::Progress(progress)) },
                Err((error, original)) => { self.post = Some(original); Err(error) },
            };
        }
        if let Some(value) = [&mut self.applied_edit_id, &mut self.tail_edit_id].into_iter().find(|value| value.capacity() != 0) { *value = String::new(); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..Default::default() })); }
        if let Some(original)=self.local_actor.take(){return match semio_framework_value::retirement::admit_owned_retirement(original,child){Ok((owner,receipt))=>{self.retiring=Some(owner);Ok(RetainedCloneStep::Progress(receipt))},Err((error,original))=>{self.local_actor=Some(original);Err(error)}};}
        if let Some(factory) = self.mutation_retirement.take() { let factory: Arc<dyn semio_framework_value::FactoryRetirement> = factory; self.factory_close[0] = Some(semio_framework_value::FactoryAuthority::new(factory)); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })); }
        if let Some(factory) = self.snapshot_retirement.take() { let factory: Arc<dyn semio_framework_value::FactoryRetirement> = factory; self.factory_close[1] = Some(semio_framework_value::FactoryAuthority::new(factory)); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })); }
        if let Some(slot) = self.factory_close.iter_mut().find(|slot| slot.is_some()) { let factory = slot.as_mut().unwrap(); let step = factory.step(child)?; let step = semio_framework_value::retained_clone::admit_retained_clone_close(child, step, factory.terminal_is_empty(), "batch stage factory")?; if factory.terminal_is_empty() { *slot = None; } return Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) }); }
        Ok(RetainedCloneStep::Complete(Default::default()))
    }
    pub fn terminal_is_empty(&self) -> bool {
        self.diagnostics.terminal_is_empty() && self.handoff_ids.iter().all(Option::is_none) && self.handoff_cursors.iter().all(Option::is_none) && self.digest_owner.is_none() && self.message_owner.is_none() && self.message.is_none() && self.commit_metadata.is_none() && self.displaced_report.is_none() && self.seed_owner.is_none() && self.seed_pending.is_none() && self.seed_retiring.is_none() && self.folding.is_none() && self.edit.is_none() && self.pending_edit.is_none() && self.retiring.is_none() && self.post.is_none() && self.unit_flags.capacity() == 0 && self.applied_edit_id.capacity() == 0 && self.tail_edit_id.capacity() == 0 && self.local_actor.is_none() && self.mutation_retirement.is_none() && self.snapshot_retirement.is_none() && self.factory_close.iter().all(Option::is_none)
    }
}

/// 📬️ Retained batched publication authority: ONE user gesture, N admitted mutations, ONE staged
/// `Edit`, ONE history ledger slot, ONE undo step. Each turn folds at most one mutation — the
/// single-mutation case is simply the N = 1 batch. It never calls `apply_one`, `apply_command`,
/// `replay_mutations`, `pump`, or `flush_outbound`.
pub struct ArtifactStoreBatchPublication<P, Mutation> {
    pub original_registry: std::mem::ManuallyDrop<Option<semio_framework_value_snapshot_read::SnapshotReadRegistryHandle>>,
    pub registry_retirement: Option<Box<dyn ErasedSnapshotRetirement>>,
    pub caller_generation: Option<semio_framework_job::Generation>,
    pub original_origins: std::mem::ManuallyDrop<Option<std::collections::VecDeque<protocol::MutationOrigin>>>,
    pub origins_retirement:semio_framework_value::retained_clone::RetainedCloneClose,
    pub operation: semio_framework_job::OperationId,
    pub expected_generation: u64,
    pub expected_revision: [u8; 32],
    pub lane: HistoryLane,
    pub footprint: ArtifactStoreOneItemFootprint,
    pub source_census_completed: u32,
    pub source_census_finished: bool,
    pub admitted_items: usize,
    pub authority: Option<Arc<ArtifactStoreOneItemLiveAuthority>>,
    pub authority_retirement: Option<Box<dyn ErasedSnapshotRetirement>>,
    pub source: Option<Box<dyn ArtifactStoreBatchSource<P, Mutation>>>,
    pub preparation: Option<Box<dyn ArtifactStoreOneItemPreparation<P, Mutation>>>,
    pub item_closing: bool,
    pub stage: Option<Box<ArtifactStoreBatchStage<P, Mutation>>>,
    pub group_preparation: Option<member_group::MemberGroupPreparation>,
    pub retained_checkpoint: ArtifactStoreOneItemCheckpoint,
    pub receipt: Option<LaneItemReceipt>,
    pub attempts: u8,
    pub published: bool,
    pub cancel_requested: bool,
    pub close_started: bool,
    pub fault: Option<ValueError>,
    pub refused_base: Option<SnapshotRead<P>>,
    pub refused_authority: Option<Arc<ArtifactStoreOneItemLiveAuthority>>,
    pub refused_issuers: [Option<semio_framework_value::FactoryAuthority>; 2],
    pub phase: ArtifactStoreOneItemPublicationPhase,
    pub verb: Option<String>,
    pub transaction: std::mem::ManuallyDrop<Option<protocol::TransactionRef>>,
    pub transaction_close: Option<semio_framework_value::retirement::controlled::ControlledRetirement<protocol::TransactionRef>>,
    pub lane_insert: Option<protocol::HistoryFoldIndexInsert<String, HistoryLane>>,
    pub lane_close: Option<semio_framework_value::retirement::controlled::ControlledRetirement<String>>,
    pub lane_ready: bool,
    pub outbound: bool,
    pub announce_from: usize,
    pub announce_items: usize,
    pub transaction_open: bool,
}

impl<P, Mutation> ArtifactStoreBatchPublication<P, Mutation> {
    /// 🧷️ The same original decoder cleanup loan reaches wire preparation before every outer owner retires.
    pub fn close_original_step(&mut self,original:&mut semio_framework_value::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>where P:Send+Sync+'static,Mutation:Send+'static{
     self.begin_close();if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}if self.group_preparation.is_some()||grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()))}let d=self.retirement_demands(grant.maximum_copy_bytes)?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}
     if self.transaction_close.is_none()&&self.transaction.is_none()&&self.lane_close.is_none()&&self.lane_insert.is_none(){if let Some(owner)=self.preparation.as_mut(){if !owner.terminal_is_empty(){let child=ArtifactStoreOneItemGrant{maximum_items:1,maximum_copy_bytes:grant.maximum_copy_bytes,maximum_capacity_bytes:grant.maximum_capacity_bytes,maximum_release_bytes:grant.maximum_release_bytes,maximum_depth:grant.maximum_depth-1};let s=owner.close_original_step(original,child)?;let s=semio_framework_value::retained_clone::admit_retained_clone_close(child.retained_grant(),s,owner.terminal_is_empty(),"original publication preparation")?;return Ok(RetainedCloneStep::Progress(s.progress()))}}}
     original.charge(d.capacity_bytes)?;self.close_step(grant)
    }
    /// 🧬️ Captures one original provenance row per mutation before any semantic candidate is admitted.
    pub fn admit_original_origins(&mut self,origins:&mut Option<std::collections::VecDeque<protocol::MutationOrigin>>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        let invalid=||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original publication provenance must precede semantic preparation and match every admitted mutation");
        if self.original_origins.is_some()||!self.origins_retirement.is_empty()||self.preparation.is_some()||self.stage.is_some()||self.close_started||self.published||origins.as_ref().is_none_or(|rows|rows.len()!=self.admitted_items){return Err(invalid())}
        let demand=semio_framework_value::RetirementDemand{copy_bytes:2*size_of::<Option<std::collections::VecDeque<protocol::MutationOrigin>>>(),depth:1,..Default::default()};
        if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()))}
        *self.original_origins=origins.take();Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))
    }

    /// 🎟️ Current private group copy or allocation demand, never accumulated credit.
    pub fn next_group_byte_demand(&self) -> usize { self.group_preparation.as_ref().map_or(1, |group| group.next_byte_demand()) }

    /// 📏️ Names every physical and logical close axis before transferring the next owner.
    pub fn retirement_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError>
    where P: Send + Sync + 'static, Mutation: Send + 'static,
    {
        use semio_framework_value::RetirementDemand;
        let nested = |mut demand: RetirementDemand| -> Result<RetirementDemand, ValueError> { demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "publication depth overflow"))?; Ok(demand) };
        if self.original_origins.is_some()||!self.origins_retirement.is_empty(){return self.origins_retirement.next_owner_demands::<std::collections::VecDeque<protocol::MutationOrigin>>(self.original_origins.is_some(),&None,body)}
        if let Some(owner)=self.transaction_close.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<semio_framework_value::retirement::controlled::ControlledRetirement<protocol::TransactionRef>>>(),depth:1,..Default::default()})}else{owner.next_demand(body)};}
        if self.transaction.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<protocol::TransactionRef>>()+size_of::<semio_framework_value::retirement::controlled::ControlledRetirement<protocol::TransactionRef>>(),depth:1,..Default::default()});}
        if let Some(owner)=self.lane_close.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<semio_framework_value::retirement::controlled::ControlledRetirement<String>>>(),depth:1,..Default::default()})}else{owner.next_demand(body)};}
        if self.lane_insert.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<protocol::HistoryFoldIndexInsert<String,HistoryLane>>()+size_of::<Option<protocol::HistoryFoldIndexInsert<String,HistoryLane>>>()+size_of::<semio_framework_value::retirement::controlled::ControlledRetirement<String>>(),depth:1,..Default::default()});}
        if let Some(group) = self.group_preparation.as_ref() { return group.retirement_demands(); }
        if let Some(owner) = self.preparation.as_ref() {
            return if owner.terminal_is_empty() { Ok(RetirementDemand { release_bytes: size_of_val(owner.as_ref()), depth: 1, ..Default::default() }) } else { nested(RetirementDemand { copy_bytes: owner.next_close_copy_byte_demand()?, capacity_bytes: owner.next_close_capacity_byte_demand(body)?, release_bytes: owner.next_close_release_byte_demand()?, depth: owner.next_close_depth_demand()? }) };
        }
        if let Some(source) = self.source.as_ref() { return if source.terminal_is_empty() { Ok(RetirementDemand { release_bytes: size_of_val(source.as_ref()), depth: 1, ..Default::default() }) } else { nested(source.retirement_demands(body)?) }; }
        if let Some(stage) = self.stage.as_ref() { return if stage.terminal_is_empty() { Ok(RetirementDemand { release_bytes: size_of::<ArtifactStoreBatchStage<P, Mutation>>(), depth: 1, ..Default::default() }) } else { nested(stage.retirement_demands(body)?) }; }
        if let Some(verb) = self.verb.as_ref() { return Ok(RetirementDemand { release_bytes: verb.capacity(), depth: 1, ..Default::default() }); }
        if self.receipt.is_some() { return Ok(RetirementDemand { depth: 1, ..Default::default() }); }
        if let Some(owner) = self.authority_retirement.as_ref() { return nested(artifact_retirement_box_demands(owner, body)?); }
        if self.refused_base.is_some() { return Ok(RetirementDemand { depth: 1, ..Default::default() }); }
        if let Some(authority) = self.refused_authority.as_ref() { let birth = authority.retirement_birth_demand(); return nested(RetirementDemand { capacity_bytes: birth.capacity_bytes, depth: birth.depth, ..Default::default() }); }
        if let Some(issuer) = self.refused_issuers.iter().find_map(Option::as_ref) { return nested(issuer.demands(body)?); }
        if let Some(authority) = self.authority.as_ref() { let birth = authority.retirement_birth_demand(); return nested(RetirementDemand { capacity_bytes: birth.capacity_bytes, depth: birth.depth, ..Default::default() }); }
        if let Some(fault) = self.fault.as_ref() { let release_bytes=match &fault.message{std::borrow::Cow::Borrowed(_)=>0,std::borrow::Cow::Owned(text)=>text.capacity()};return Ok(RetirementDemand { release_bytes, depth: 1, ..Default::default() }); }
        if let Some(owner)=self.registry_retirement.as_ref(){return artifact_retirement_box_demands(owner,body);}
        if self.original_registry.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<SnapshotReadRegistryAliasRetirement>()+size_of::<Option<semio_framework_value_snapshot_read::snapshot_read_registry_handle::SnapshotReadRegistryHandle>>()+size_of::<Option<Box<dyn ErasedSnapshotRetirement>>>(),capacity_bytes:size_of::<SnapshotReadRegistryAliasRetirement>(),depth:1,..Default::default()});}
        Ok(RetirementDemand { depth: usize::from(!self.terminal_is_empty()), ..Default::default() })
    }

    pub fn operation(&self) -> semio_framework_job::OperationId {
        self.operation
    }

    pub fn phase(&self) -> ArtifactStoreOneItemPublicationPhase {
        self.phase
    }

    /// 🏷️ The id of the action or command this gesture publishes for, stamped on the gesture's new edit (an amend keeps
    /// the verb of the edit it extends). See [`semio_framework_os_spr::command::Edit::verb`].
    pub fn set_verb(&mut self, verb: Option<String>) {
        self.verb = verb.filter(|value| !value.is_empty());
    }

    /// 🧾️ The committed tool transaction this gesture publishes for, stamped on every staged operation.
    pub fn transaction(&self) -> Option<&protocol::TransactionRef> {
        self.transaction.as_ref()
    }

    /// 🫙️ Keeps this gesture's tool transaction open after its operations land: they join the transaction's open edit (the
    /// first streamed gesture opens it) and nothing is announced until a closing gesture or `CommitTransaction` commits
    /// it. A closing gesture (the default) that carries the open transaction's ref appends to its edit and commits it.
    pub fn set_transaction_open(&mut self, open: bool) {
        self.transaction_open = open;
    }

    /// 🔢️ How many mutations this gesture admitted, and how many of them are already staged.
    pub fn admitted_items(&self) -> usize {
        self.admitted_items
    }

    pub fn staged_items(&self) -> usize {
        self.stage.as_ref().map_or(0, |stage| stage.folded)
    }

    /// 🎁️ Hands the staged gesture edit back as ONE sealed candidate so a group-atomic decision can
    /// linearize several stores' candidates before any of them becomes visible. Only the commit turn
    /// may call it, and the publication then owns nothing but its receipt lane.
    pub fn take_staged_prepared(&mut self, grant: RetainedCloneGrant, receiving: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<(Option<ArtifactStoreOneItemPrepared<P, Mutation>>,RetainedCloneProgress),ValueError> {
        let blocked=||Ok((None,RetainedCloneProgress::default()));
        if self.phase != ArtifactStoreOneItemPublicationPhase::Publishing || self.stage.as_ref().is_none_or(|stage| stage.post.is_none()) { return blocked(); }
        let stage=self.stage.as_mut().unwrap();
        for index in 0..2 {
            if stage.handoff_ids[index].is_none() {
                let copy=size_of::<semio_framework_value::paged::PagedUtf8<{usize::MAX}>>()+size_of::<semio_framework_value::paged::PagedUtf8AppendCursor>();
                if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<copy{return blocked();}
                stage.handoff_ids[index]=Some(Default::default());stage.handoff_cursors[index]=Some(Default::default());
                return Ok((None,RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}));
            }
            if !stage.handoff_cursors[index].as_ref().is_some_and(|cursor|cursor.complete) {
                let source=if index==0{stage.applied_edit_id.as_str()}else{stage.tail_edit_id.as_str()};
                let cursor=stage.handoff_cursors[index].as_mut().unwrap();let output=stage.handoff_ids[index].as_mut().unwrap();
                let demand=cursor.next_advance_demand(source,output)?;
                if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return blocked();}
                receiving.charge(demand.capacity_bytes)?;
                let progress=cursor.advance(source,output,grant)?.progress();
                return Ok((None,progress));
            }
        }
        let copy=size_of::<ArtifactStoreOneItemPrepared<P,Mutation>>()+size_of::<Arc<ArtifactStoreOneItemLiveAuthority>>();
        let message=if stage.diagnostics.row_count()!=0{size_of::<semio_framework_os_spr::command::MutationMessagePublication>()}else{0};
        if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<copy+message||stage.local_actor.is_none()||self.authority.is_none()||stage.diagnostics.row_count()!=0&&!stage.diagnostics.joined_transfer_ready(){return blocked();}
        let messages=if stage.diagnostics.row_count()!=0{Some(stage.diagnostics.take_joined(RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-copy,..grant}).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"joined original diagnostics refused admitted handoff"))?)}else{None};
        let actor=stage.local_actor.take().unwrap();
        let [applied,tail]=std::mem::take(&mut stage.handoff_ids);
        let mut prepared=self.authority.as_ref().unwrap().seal_prepared_owned(stage.edit.take().unwrap(),stage.post.take().unwrap(),stage.digest,[actor,applied.unwrap(),tail.unwrap()]);
        let mut progress=RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()};
        if let Some((owner,receipt))=messages{prepared.record_outcome_messages(owner);progress.copied_bytes+=receipt.copied_bytes;progress.retained_capacity_bytes+=receipt.retained_capacity_bytes;progress.released_bytes+=receipt.released_bytes;}
        self.retained_checkpoint=stage.checkpoint();
        Ok((Some(prepared),progress))
    }

    pub fn take_stage(&mut self) -> Option<Box<ArtifactStoreBatchStage<P, Mutation>>> {
        let stage = self.stage.take()?;
        self.retained_checkpoint = stage.checkpoint();
        Some(stage)
    }

    /// 📍️ Gesture-wide progress survives folding, publication, acknowledgement, and owner retirement.
    pub fn progress(&self) -> ArtifactStoreOneItemCheckpoint {
        if self.close_started {
            return self.retained_checkpoint;
        }
        let staged = self.stage.as_ref().map_or(self.retained_checkpoint, |stage| stage.checkpoint());
        let item = if self.item_closing { ArtifactStoreOneItemCheckpoint::default() } else { self.preparation.as_ref().map_or_else(ArtifactStoreOneItemCheckpoint::default, |owner| owner.checkpoint()) };
        ArtifactStoreOneItemCheckpoint {
            cursor: staged.cursor,
            completed_items: staged.completed_items.saturating_add(item.completed_items).saturating_add(self.source_census_completed).saturating_add(self.group_preparation.as_ref().map_or(0, |group| group.completed_items())),
            completed_bytes: staged.completed_bytes.saturating_add(item.completed_bytes),
            digest: if item.digest == [0; 32] { staged.digest } else { item.digest },
        }
    }

    pub fn fault(&self) -> Option<&str> { self.fault.as_ref().map(|fault|fault.message.as_ref()) }

    /// 🚫️ Borrows the one original refusal and its actual consumed physical currencies.
    pub fn preparation_refusal(&self) -> Option<&ValueError> { self.fault.as_ref() }

    pub fn retry(&mut self) -> bool {
        if self.phase != ArtifactStoreOneItemPublicationPhase::AwaitingAck || self.attempts >= ARTIFACT_STORE_ONE_ITEM_MAXIMUM_RETRIES {
            return false;
        }
        self.attempts += 1;
        true
    }

    /// 🧾️ Prices only the original checkpoint and close-state coordination after retained intent exists.
    pub fn acknowledgement_demands(&self)->semio_framework_value::RetirementDemand{semio_framework_value::RetirementDemand{copy_bytes:std::mem::size_of::<ArtifactStoreOneItemCheckpoint>()+2*std::mem::size_of::<ArtifactStoreOneItemPublicationPhase>()+3*std::mem::size_of::<bool>(),depth:1,..Default::default()}}

    /// 🎟️ The caller posts this exact receipt before relinquishing the original publication source.
    pub fn acknowledge_granted(&mut self,grant:RetainedCloneGrant)->Option<RetainedCloneProgress>{let d=self.acknowledgement_demands();if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d)||!self.acknowledge(){return None}Some(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})}

    pub fn acknowledge(&mut self) -> bool {
        if self.phase != ArtifactStoreOneItemPublicationPhase::AwaitingAck || self.outbound {
            return false;
        }
        self.phase = ArtifactStoreOneItemPublicationPhase::Closing;
        self.begin_close();
        true
    }

    pub fn begin_close(&mut self) {
        if self.close_started {
            return;
        }
        self.retained_checkpoint = self.progress();
        self.close_started = true;
        self.phase = if self.phase == ArtifactStoreOneItemPublicationPhase::Complete { ArtifactStoreOneItemPublicationPhase::Complete } else { ArtifactStoreOneItemPublicationPhase::Closing };
        if let Some(owner) = self.preparation.as_mut() {
            if !self.published {
                self.cancel_requested = true;
                owner.cancel();
            }
            owner.begin_close();
        }
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>
    where P: Send + Sync + 'static, Mutation: Send + 'static,
    {
        use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep, RetainedCloneProgress};
        self.begin_close();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if self.group_preparation.is_some() || grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let demand = self.retirement_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "publication exceeds admitted depth")); }
        if demand.copy_bytes > grant.maximum_copy_bytes || demand.capacity_bytes > grant.maximum_capacity_bytes || demand.release_bytes > grant.maximum_release_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if self.original_origins.is_some()||!self.origins_retirement.is_empty(){let step=if !self.origins_retirement.is_empty(){self.origins_retirement.select(grant.maximum_copy_bytes)?.step(grant)?}else{self.origins_retirement.begin_granted(&mut self.original_origins,grant)?.unwrap_or(RetainedCloneStep::Complete(Default::default()))};return Ok(RetainedCloneStep::Progress(step.progress()))}
        if let Some(owner)=self.transaction_close.as_mut(){if !owner.terminal_is_empty(){return owner.step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}self.transaction_close=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}
        if let Some(original)=self.transaction.take(){self.transaction_close=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(original).map_err(|(error,_)|error)?);return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}
        if let Some(owner)=self.lane_close.as_mut(){if !owner.terminal_is_empty(){return owner.step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}self.lane_close=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}
        if let Some(owner)=self.lane_insert.as_mut(){let Some(((key,_lane),_))=owner.return_original_controlled(grant)else{return Ok(RetainedCloneStep::Progress(Default::default()));};self.lane_close=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(key).map_err(|(error,_)|error)?);self.lane_insert=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if let Some(owner) = self.preparation.as_mut() {
            if !owner.terminal_is_empty() {
                let preparation_grant = ArtifactStoreOneItemGrant { maximum_items: child.maximum_items, maximum_copy_bytes: child.maximum_copy_bytes, maximum_capacity_bytes: child.maximum_capacity_bytes, maximum_release_bytes: child.maximum_release_bytes, maximum_depth: child.maximum_depth };
                let step = owner.close_step(preparation_grant)?;
                let step = semio_framework_value::retained_clone::admit_retained_clone_close(child, step, owner.terminal_is_empty(), "publication preparation")?;
                return Ok(RetainedCloneStep::Progress(step.progress()));
            }
            self.preparation = None;
            self.item_closing = false;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..Default::default() }));
        }
        if let Some(source) = self.source.as_mut() {
            if !source.terminal_is_empty() { let step = source.close_step(child)?; let step = semio_framework_value::retained_clone::admit_retained_clone_close(child, step, source.terminal_is_empty(), "publication source")?; return Ok(RetainedCloneStep::Progress(step.progress())); }
            self.source = None;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..Default::default() }));
        }
        if let Some(stage) = self.stage.as_mut() {
            if !stage.terminal_is_empty() { let step = stage.close_step(child)?; let step = semio_framework_value::retained_clone::admit_retained_clone_close(child, step, stage.terminal_is_empty(), "publication stage")?; return Ok(RetainedCloneStep::Progress(step.progress())); }
            self.stage = None;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..Default::default() }));
        }
        if self.verb.take().is_some() { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..Default::default() })); }
        if self.receipt.take().is_some() { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
        if self.authority_retirement.is_some() { return artifact_retirement_box_close_step(&mut self.authority_retirement, child).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if let Some(original) = self.refused_base.take() {
            return match original.try_return_to_registry_witness() { Ok(witness) => { drop(witness); Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })) }, Err(original) => { self.refused_base = Some(original); Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "refused original preparation read registry is busy")) } };
        }
        if let Some(authority) = self.refused_authority.take() {
            return match authority.retire(child) { Ok((owner, progress)) => { self.authority_retirement = Some(owner); Ok(RetainedCloneStep::Progress(progress)) }, Err((error, original)) => { self.refused_authority = Some(original); Err(error) } };
        }
        if let Some(slot) = self.refused_issuers.iter_mut().find(|slot| slot.is_some()) {
            let issuer = slot.as_mut().expect("refused request retains its installed issuer");
            let step = issuer.step(child)?;
            let step = semio_framework_value::retained_clone::admit_retained_clone_close(child, step, issuer.terminal_is_empty(), "refused original preparation issuer")?;
            if issuer.terminal_is_empty() { *slot = None; }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(authority) = self.authority.take() {
            return match authority.retire(child) { Ok((owner, progress)) => { self.authority_retirement = Some(owner); Ok(RetainedCloneStep::Progress(progress)) }, Err((error, original)) => { self.authority = Some(original); Err(error) } };
        }
        if self.fault.take().is_some() { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..Default::default() })); }
        if self.registry_retirement.is_some(){return artifact_retirement_box_close_step(&mut self.registry_retirement,grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
        if let Some(original)=self.original_registry.take(){return match admit_artifact_retirement(original,grant,SnapshotReadRegistryAliasRetirement::new){Ok((owner,mut progress))=>{progress.copied_bytes=demand.copy_bytes;self.registry_retirement=Some(owner);Ok(RetainedCloneStep::Progress(progress))},Err((error,original))=>{*self.original_registry=Some(original);Err(error)}};}

        self.caller_generation = None;
        self.phase = ArtifactStoreOneItemPublicationPhase::Complete;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.phase == ArtifactStoreOneItemPublicationPhase::Complete
            && self.preparation.is_none()
            && self.source.is_none()
            && self.stage.is_none()
            && self.group_preparation.is_none()
            && self.authority.is_none()
            && self.authority_retirement.is_none()
            && self.receipt.is_none()
            && self.refused_base.is_none()
            && self.refused_authority.is_none()
            && self.refused_issuers.iter().all(Option::is_none)
            && self.fault.is_none()
            && self.verb.is_none()
            && self.transaction.is_none()
            && self.transaction_close.is_none()
            && self.original_origins.is_none()
            && self.origins_retirement.is_empty()
            && self.lane_insert.is_none()
            && self.lane_close.is_none()
            && self.original_registry.is_none()
            && self.registry_retirement.is_none()
    }

    /// 🔬️ The first owner `close_step` would try to retire, in its exact drain order, plus whether
    /// `begin_close` has run. `close_step` answers only `Pending`/`Blocked`/`Complete`, so a
    /// publication that will not reach terminal emptiness under repeated one-item grants cannot say
    /// WHICH of its seven owners refuses. This names it.
    pub fn closing_owner_witness(&self) -> &'static str {
        if !self.close_started {
            return "close-not-started";
        }
        if self.preparation.is_some() {
            "preparation"
        } else if self.source.is_some() {
            "source"
        } else if self.stage.is_some() {
            "stage"
        } else if self.verb.is_some() {
            "verb"
        } else if self.receipt.is_some() {
            "receipt"
        } else if self.authority_retirement.is_some() {
            "authority-retirement"
        } else if self.refused_base.is_some() {
            "refused-base"
        } else if self.refused_authority.is_some() {
            "refused-authority"
        } else if self.refused_issuers.iter().any(Option::is_some) {
            "refused-issuer"
        } else if self.authority.is_some() {
            "authority"
        } else if self.fault.is_some() {
            "fault"
        } else {
            "none"
        }
    }
}

impl<P, Mutation> Drop for ArtifactStoreBatchPublication<P, Mutation> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.terminal_is_empty()), "batched artifact-store publication reached Drop without its exact terminal-empty witness");
        if self.terminal_is_empty(){unsafe{std::mem::ManuallyDrop::drop(&mut self.original_registry);std::mem::ManuallyDrop::drop(&mut self.transaction);}}
    }
}

impl<P: Send + Sync + 'static, Mutation: Send + 'static> ErasedSnapshotRetirement for ArtifactStoreBatchPublication<P, Mutation> {
    fn close_original_step(&mut self,original:&mut semio_framework_value::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{ArtifactStoreBatchPublication::close_original_step(self,original,grant)}
    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> { self.retirement_demands(body) }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.retirement_demands(body)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.depth) }
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { ArtifactStoreBatchPublication::close_step(self, grant) }
    fn terminal_is_empty(&self) -> bool { ArtifactStoreBatchPublication::terminal_is_empty(self) }
}
