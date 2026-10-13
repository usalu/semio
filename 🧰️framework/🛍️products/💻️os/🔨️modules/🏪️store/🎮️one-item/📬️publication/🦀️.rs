//! 📬️ The retained one-item publication protocol: the preparation trait and its factory, the live authority the Store
//! mints, the prepared owner and its seal, batch sources and the member one-item wire. Domain preparation owners and
//! the Store engine both speak it; it names only the one-item vocabulary, the ledger and snapshot-read owners below it.
//!
//! Layering: `📓️2026-10-13-wave-e1-os-kernel.md` (ticket BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT).
use crate::ArtifactStoreBatchDigest;
use super::member_owned_batch;
use super::member_group;
use super::canonical_edit;
use super::ArtifactStoreDecodedEditRetirement;
use crate::os_vcs::io::binary::genesis::ArtifactGenesisCodec;
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementFactory};
use semio_framework_value::retained_clone::{RetainedCloneBirthDemand, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use crate::os_spr::command::ArtifactMessageSource;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;
use crate::os_spr::{ActorId, HybridLogicalTimestamp, MutationId, UndoPolicy};
use crate::os_spr::{DiffRegions, Edit, Mutation, MutationDiff, MutationMeta, OpBinary, OpText};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::sync::Arc;
use super::snapshot_read::{SnapshotRead, artifact_retirement_box_close_step, artifact_retirement_box_demands};
use super::snapshot_read::ArtifactStoreDisplacedOwnerReservation;
use super::snapshot_read_registry_retirement::{SnapshotReadRegistryAliasRetirement,snapshot_registry_alias_demands,snapshot_registry_alias_close_step};
use semio_framework_value::retirement::frame::admit_retirement_frame as admit_artifact_retirement;
use crate::os_vcs::{
    Identified, Patchable,
};
use crate::os_pack::pack_rt;
use super::ledger::{ArtifactStoreHistoryCommitReservation, CursorRevisionAccumulator};
use super::prepared_retirement::ArtifactStorePreparedRetirement;
use super::canonical_edit::{
    ArtifactCanonicalJson, ArtifactCanonicalJsonTree, ArtifactStoreOneItemSealer,
};
use super::prepared_operation_wire::{ArtifactOperationWire, ArtifactOperationText, ArtifactPreparedOperationSource};
use super::one_item::{ARTIFACT_STORE_ONE_ITEM_ID_BYTES, ARTIFACT_STORE_ONE_ITEM_MAXIMUM_RETRIES, ArtifactStoreOneItemCheckpoint, ArtifactStoreOneItemFootprint, ArtifactStoreOneItemGrant, ArtifactStoreOneItemPreparationStep};
use super::{HistoryLane, LaneItemReceipt};

#[path = "🪪️authority/🦀️.rs"]
mod publication_authority_retirement;

/// 🪪 Exact live publication metadata minted by the Store and shared immutably with the
/// domain preparation owner. Apps cannot invent cursor position, sequence, clock, or actor.
pub struct ArtifactStoreOneItemLiveAuthority {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    base_revision: [u8; 32],
    base_applied_edit_count: usize,
    next_sequence_number: i32,
    next_clock: HybridLogicalTimestamp,
    actor: semio_framework_value::SharedUtf8,
    line: Option<String>,
    group_id: Option<String>,
    stamped_edit_id: Option<String>,
}

impl ArtifactStoreOneItemLiveAuthority {
    /// 🪙️ Mints the live authority of one Store publication. Every field is the Store's own operation, cursor, sequence,
    /// clock and actor state, so only the Store engine mints; a domain receives the minted authority, never its fields.
    pub(crate) fn mint(
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        base_revision: [u8; 32],
        base_applied_edit_count: usize,
        next_sequence_number: i32,
        next_clock: HybridLogicalTimestamp,
        actor: semio_framework_value::SharedUtf8,
        line: Option<String>,
        group_id: Option<String>,
        stamped_edit_id: Option<String>,
    ) -> Self {
        Self { operation, generation, base_revision, base_applied_edit_count, next_sequence_number, next_clock, actor, line, group_id, stamped_edit_id }
    }

    /// 🔗️ The same publication authority rebound under one durable group decision.
    pub(crate) fn rebind_group(&self, group_id: String) -> Self {
        Self { actor: self.actor.clone(), line: self.line.clone(), group_id: Some(group_id), stamped_edit_id: self.stamped_edit_id.clone(), ..*self }
    }

    /// 🎭️ The Store-minted actor text every edit of this publication carries.
    pub(crate) fn actor_utf8(&self) -> &semio_framework_value::SharedUtf8 {
        &self.actor
    }

    pub fn operation(&self) -> semio_framework_job::OperationId {
        self.operation
    }

    pub fn generation(&self) -> semio_framework_job::Generation {
        self.generation
    }

    pub fn base_revision(&self) -> [u8; 32] {
        self.base_revision
    }

    pub fn base_applied_edit_count(&self) -> usize {
        self.base_applied_edit_count
    }

    pub fn next_sequence_number(&self) -> i32 {
        self.next_sequence_number
    }

    pub fn next_clock(&self) -> HybridLogicalTimestamp {
        self.next_clock
    }

    pub fn actor(&self) -> &str {
        &self.actor
    }
    pub fn admit_actor_lease(&self,grant:RetainedCloneGrant)->Result<(semio_framework_value::SharedUtf8,RetainedCloneProgress),ValueError>{self.actor.admit_clone(grant)}

    /// 🪪️ The globally unique identity of the edit this authority publishes — content-addressed
    /// over the actor, the sequence and the HLC tick the Store minted, so two replicas' edits never
    /// share an identity the way a replica-local counter would. The Store stamps it on the staged edit.
    ///
    /// 🔏 A server-stamped publication instead publishes under the identity its durable
    /// decision was hashed over: the approval's own mutation id, itself content-addressed over the
    /// job and the proposal, so the uniqueness argument above still holds and the committed edit
    /// carries the identity every downstream verifier (WAL, ledger, actor frontier) binds against.
    pub fn edit_id(&self) -> String {
        if let Some(stamped) = self.stamped_edit_id.as_ref() {
            return stamped.clone();
        }
        let mut material = Vec::with_capacity(self.actor.len() + 48);
        material.extend_from_slice(&(self.actor.len() as u64).to_le_bytes());
        material.extend_from_slice(self.actor.as_bytes());
        material.extend_from_slice(&self.next_sequence_number.to_le_bytes());
        for part in [self.next_clock.actor, self.next_clock.physical_ms, self.next_clock.logical] {
            material.extend_from_slice(&part.to_le_bytes());
        }
        let digest = semio_framework_hash::hash(&material);
        let mut id = String::with_capacity(21);
        id.push_str("edit-");
        for byte in &digest.as_bytes()[..8] {
            id.push_str(&format!("{byte:02x}"));
        }
        id
    }

    /// 🌿️ The immutable authored branch of this Store publication.
    pub fn line_id(&self) -> Option<&str> {
        self.line.as_deref()
    }

    pub fn group_id(&self) -> Option<&str> {
        self.group_id.as_deref()
    }

    /// 🔏 The server-stamped identity this publication was minted under, or `None` for every
    /// local gesture. The fold reads it to keep the stamped mutation id on the first folded item
    /// instead of rewriting it to the store's `<edit-id>#<position>` form.
    pub fn stamped_edit_id(&self) -> Option<&str> {
        self.stamped_edit_id.as_deref()
    }

    /// 🧾️ The ONE semantic edit a one-item preparation publishes: `forward` with its `inverse` rows, minted entirely from
    /// this authority (identity, actor, line, group, sequence, clock, base version) so no domain hand-builds an edit or
    /// invents a field the Store validates. It carries no hand-written description (design §20.6): the fold stamps semantic
    /// kind, label and transaction, and history labels the row from the leaf.
    pub fn next_edit<Mutation>(&self, forward: Mutation, inverse: Vec<Mutation>) -> Edit<Mutation> {
        let id = self.edit_id();
        Edit {
            line: self.line.clone(),
            mutation_meta: vec![MutationMeta {
                mutation_id: Some(MutationId(format!("{id}#0"))),
                dependencies: Vec::new(),
                base_version: self.base_applied_edit_count as u64,
                author_id: Some(ActorId(self.actor.clone())),
                timestamp: self.next_clock,
                undo_policy: UndoPolicy::ExactBaseOnly,
                payload_hash: None,
                semantic_kind: None,
                label: None,
                group_id: self.group_id.clone(),
                origin: Default::default(),
                transaction: None,
            }],
            id,
            actor: Some(self.actor.clone()),
            forwards: vec![forward],
            inverse: inverse.into(),
            verb: None,
            sequence_number: self.next_sequence_number,
            started_at: String::new(),
            finished_at: None,
        }
    }

    /// 📐️ Measures the original publication authority's exact retained cursor frame.
    pub fn retirement_birth_demand(&self) -> RetainedCloneBirthDemand {
        RetainedCloneBirthDemand { capacity_bytes: size_of::<canonical_edit::ArtifactStoreOneItemAuthorityRetirement>(), depth: 1 }
    }

    /// 🧹️ Admits the original authority cursor before moving its Arc or allocating its frame.
    pub fn retire(self: Arc<Self>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, Arc<Self>)> {
        let receipt = match self.retirement_birth_demand().admit(grant) { Ok(receipt) => receipt, Err(error) => return Err((error, self)) };
        Ok((Box::new(canonical_edit::ArtifactStoreOneItemAuthorityRetirement::new(self)), receipt))
    }

    pub(crate) fn validate_semantic_edit<Mutation>(&self, edit: &Edit<Mutation>) -> Result<(), ValueError> {
        let meta = edit.mutation_meta.first().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"one-item edit lacks its exact authority metadata"))?;
        if edit.id.is_empty()
            || edit.id.len() > ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            || edit.sequence_number != self.next_sequence_number
            || edit.forwards.len() != 1
            || edit.mutation_meta.len() != 1
            || edit.actor.as_deref() != Some(self.actor.as_str())
            || edit.line != self.line
            || meta.author_id.as_ref().map(|actor| actor.0.as_str()) != Some(self.actor.as_str())
            || meta.timestamp != self.next_clock
            || meta.group_id.as_deref() != self.group_id.as_deref()
        {
            return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"one-item semantic edit disagrees with its immutable Store authority"));
        }
        Ok(())
    }

    /// 🔏 Canonical oracle for explicitly bounded edits; retained large edits use the byte sealer.
    pub fn prepared_edit_digest<Mutation: ToValue>(&self, edit: &Edit<Mutation>) -> Result<[u8; 32], ValueError> {
        self.validate_semantic_edit(edit)?;
        Ok(CursorRevisionAccumulator::edit_digest(edit))
    }

    /// 📦 Seals an explicitly bounded semantic edit into exact Store-owned publication authority.
    pub fn prepare_one_item<P, Mutation: ToValue>(self: &Arc<Self>, edit: Edit<Mutation>, post_snapshot: Arc<P>) -> Result<ArtifactStoreOneItemPrepared<P, Mutation>, (ValueError, Edit<Mutation>, Arc<P>)> {
        let edit_digest = match self.prepared_edit_digest(&edit) { Ok(digest) => digest, Err(error) => return Err((error, edit, post_snapshot)) };
        Ok(self.seal_prepared(Box::new(edit), post_snapshot, edit_digest))
    }

    /// 🧵️ Moves the exact edit and post root into retained canonical preparation without encoding.
    pub fn begin_one_item_seal<P, Mutation>(
        self: Arc<Self>,
        edit: Edit<Mutation>,
        post_snapshot: Arc<P>,
        mutation_retirement: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
        snapshot_retirement: Arc<dyn SnapshotRetirementFactory<P>>,
        grant: RetainedCloneGrant,
    ) -> Result<(ArtifactStoreOneItemSealer<P, Mutation>, RetainedCloneProgress), (ValueError, Arc<Self>, Edit<Mutation>, Arc<P>, Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>, Arc<dyn SnapshotRetirementFactory<P>>)> where Mutation: semio_framework_value::retirement::RetireOwned+Sync+ArtifactCanonicalJsonTree {
        ArtifactStoreOneItemSealer::admit(self, edit, post_snapshot, mutation_retirement, snapshot_retirement, grant)
    }

    fn seal_prepared<P, Mutation>(self: &Arc<Self>, edit: Box<Edit<Mutation>>, post_snapshot: Arc<P>, edit_digest: [u8; 32]) -> ArtifactStoreOneItemPrepared<P, Mutation> {
        let edit_id = edit.id.clone();
        let identities = [self.actor.as_str().into(), edit_id.as_str().into(), edit_id.into()];
        self.seal_prepared_owned(edit, post_snapshot, edit_digest, identities)
    }

    pub(crate) fn seal_prepared_owned<P, Mutation>(self: &Arc<Self>, edit: Box<Edit<Mutation>>, post_snapshot: Arc<P>, edit_digest: [u8; 32], identities: [semio_framework_value::paged::PagedUtf8<{usize::MAX}>; 3]) -> ArtifactStoreOneItemPrepared<P, Mutation> {
        let [actor, applied_edit_id, tail_edit_id] = identities;
        let seal = ArtifactStoreOneItemSeal { authority: Arc::clone(self), edit_address: edit.as_ref() as *const Edit<Mutation> as usize, post_address: Arc::as_ptr(&post_snapshot) as usize, digest: edit_digest };
        ArtifactStoreOneItemPrepared { foreign_step_presence: None, edit, post_snapshot, next_clock: self.next_clock, edit_digest, local_actor: Some(actor), applied_edit_id, tail_edit_id, seal }
    }

    pub(crate) fn validate_prepared<P, Mutation>(self: &Arc<Self>, prepared: &ArtifactStoreOneItemPrepared<P, Mutation>) -> Result<(), ValueError> {
        self.validate_semantic_edit(&prepared.edit)?;
        if !Arc::ptr_eq(self, &prepared.seal.authority)
            || prepared.seal.edit_address != prepared.edit.as_ref() as *const Edit<Mutation> as usize
            || prepared.seal.post_address != Arc::as_ptr(&prepared.post_snapshot) as usize
            || prepared.seal.digest != prepared.edit_digest
        {
            return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"one-item private Store seal disagrees with exact authority or moved owners"));
        }
        Ok(())
    }
}

pub(crate) struct ArtifactStoreOneItemSeal {
    authority: Arc<ArtifactStoreOneItemLiveAuthority>,
    edit_address: usize,
    post_address: usize,
    digest: [u8; 32],
}

/// 📦 Fully prepared one-item candidate. The domain must build every potentially unbounded
/// mutation inverse/diff/apply/encoding result inside its retained cursor before exposing this
/// value. The store publication turn only validates fixed authorities and moves these owners.
pub struct ArtifactStoreOneItemPrepared<P, Mutation> {
    edit: Box<Edit<Mutation>>,
    post_snapshot: Arc<P>,
    next_clock: HybridLogicalTimestamp,
    edit_digest: [u8; 32],
    foreign_step_presence: Option<bool>,
    local_actor: Option<semio_framework_value::paged::PagedUtf8<{usize::MAX}>>,
    pub(crate) applied_edit_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
    pub(crate) tail_edit_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
    pub(crate) seal: ArtifactStoreOneItemSeal,
}

impl<P, Mutation> ArtifactStoreOneItemPrepared<P, Mutation> {
    /// 🌐️ Captures the original domain producer's measured foreign-step decision.
    pub fn record_foreign_step_presence(&mut self, presence: bool) { self.foreign_step_presence = Some(presence); }

    pub fn edit_digest(&self) -> [u8; 32] {
        self.edit_digest
    }

    /// 🧾️ The one semantic edit sealed under this candidate's live authority.
    pub fn edit(&self) -> &Edit<Mutation> { &self.edit }

    /// 🌅️ The post snapshot the sealed edit produces from its authority's base.
    pub fn post_snapshot(&self) -> &Arc<P> { &self.post_snapshot }

    /// 🕰️ The clock tick the candidate was prepared at; validation requires it to equal its authority's tick.
    pub fn next_clock(&self) -> HybridLogicalTimestamp { self.next_clock }

    /// 🌐️ The domain producer's measured foreign-step decision, once recorded.
    pub fn foreign_step_presence(&self) -> Option<bool> { self.foreign_step_presence }

    /// 🎭️ The paged local actor the candidate retained for its publication, when the domain bound one.
    pub fn local_actor(&self) -> Option<&semio_framework_value::paged::PagedUtf8<{usize::MAX}>> { self.local_actor.as_ref() }

    /// 🏷️ The paged applied-edit identity the publication appends to the cursor.
    pub fn applied_edit_id(&self) -> &semio_framework_value::paged::PagedUtf8<{usize::MAX}> { &self.applied_edit_id }

    /// 🏷️ The paged tail-edit identity the publication retains for undo.
    pub fn tail_edit_id(&self) -> &semio_framework_value::paged::PagedUtf8<{usize::MAX}> { &self.tail_edit_id }

    /// 🔏️ The live authority this candidate is sealed under, shared immutably with the Store that minted it.
    pub fn authority(&self) -> &Arc<ArtifactStoreOneItemLiveAuthority> { &self.seal.authority }

    /// 🧳️ Hands every owner of the sealed candidate to the Store that stages or retires it; the parts cannot be
    /// sealed again, so they never stand in for a prepared candidate.
    pub fn into_parts(self) -> ArtifactStoreOneItemPreparedParts<P, Mutation> {
        let Self { edit, post_snapshot, next_clock, edit_digest, foreign_step_presence, local_actor, applied_edit_id, tail_edit_id, seal } = self;
        ArtifactStoreOneItemPreparedParts { edit, post_snapshot, next_clock, edit_digest, foreign_step_presence, local_actor, applied_edit_id, tail_edit_id, authority: seal.authority }
    }
}

#[cfg(test)]
impl ArtifactStoreOneItemLiveAuthority {
    /// 🧪️ Rebinds the identity strings a law varies, on an authority no publication shares yet.
    pub(crate) fn rebind_identity_for_law(&mut self, actor: semio_framework_value::SharedUtf8, group_id: String, stamped_edit_id: String, line: String) {
        self.actor = actor;
        self.group_id = Some(group_id);
        self.stamped_edit_id = Some(stamped_edit_id);
        self.line = Some(line);
    }
}

#[cfg(test)]
impl<P, Mutation> ArtifactStoreOneItemPrepared<P, Mutation> {
    /// 🧪️ Tamper seam for seal laws: the sealed edit, its digest and the post snapshot the seal covers.
    pub(crate) fn sealed_parts_mut(&mut self) -> (&mut Box<Edit<Mutation>>, &mut [u8; 32], &mut Arc<P>) {
        (&mut self.edit, &mut self.edit_digest, &mut self.post_snapshot)
    }
}

/// 🧳️ The owners of a consumed sealed candidate, taken apart by the Store that stages or retires them.
pub struct ArtifactStoreOneItemPreparedParts<P, Mutation> {
    pub edit: Box<Edit<Mutation>>,
    pub post_snapshot: Arc<P>,
    pub next_clock: HybridLogicalTimestamp,
    pub edit_digest: [u8; 32],
    pub foreign_step_presence: Option<bool>,
    pub local_actor: Option<semio_framework_value::paged::PagedUtf8<{usize::MAX}>>,
    pub applied_edit_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
    pub tail_edit_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
    pub authority: Arc<ArtifactStoreOneItemLiveAuthority>,
}

/// 🧳 Exact owner bundle transferred to a domain preparation factory only after preflight.
pub struct ArtifactStoreOneItemPreparationRequest<P, Input, Mutation> {
    pub operation: semio_framework_job::OperationId,
    pub generation: semio_framework_job::Generation,
    pub base_revision: [u8; 32],
    pub lane: HistoryLane,
    pub authority: Arc<ArtifactStoreOneItemLiveAuthority>,
    pub base: SnapshotRead<P>,
    pub mutation: Input,
    pub mutation_retirement: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
    pub snapshot_retirement: Arc<dyn SnapshotRetirementFactory<P>>,
}

/// 🧩 Domain-owned retained mutation preparation. Generic `Mutation` is intentionally not
/// invoked by the store: a domain that cannot split its mutation semantics into these turns is
/// rejected at admission instead of receiving false resumability credit.
pub trait ArtifactStoreOneItemPreparation<P, Mutation>: Send {
    fn advance(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, ValueError>;
    fn checkpoint(&self) -> ArtifactStoreOneItemCheckpoint;
    fn prepared(&self) -> Option<&ArtifactStoreOneItemPrepared<P, Mutation>>;
    fn take_prepared(&mut self) -> Option<ArtifactStoreOneItemPrepared<P, Mutation>>;
    fn cancel(&mut self);
    fn begin_close(&mut self);
    fn close_step(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<RetainedCloneStep, ValueError>;
    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError>;
    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, ValueError>;
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError>;
    fn next_close_depth_demand(&self) -> Result<usize, ValueError>;
    fn terminal_is_empty(&self) -> bool;
}

/// 🏭️ Exact app-owned factory. `preflight` must not allocate, copy the mutation, or retain
/// the supplied references; `begin` receives ownership only after the declared footprint passes.
pub trait ArtifactStoreOneItemPreparationFactory<P, Mutation>: semio_framework_value::FactoryRetirement + Send + Sync {
    /// 🔏️ Admits the original typed aggregate edit's canonical digest owner without replacing its payload.
    fn begin_batch_digest(&self, edit: &mut Option<Box<Edit<Mutation>>>, grant: RetainedCloneGrant) -> Result<Option<(Box<dyn ArtifactStoreBatchDigest<Mutation>>, RetainedCloneProgress)>, ValueError>;
    /// 📦️ Borrows the domain codec wire of this factory's exact original prepared mutation. `None` is the answer of every factory without a domain codec: the
    /// mutation then authors through its store's installed canonical [`ArtifactOperationWire`], so no factory is ever the only way a lane authors.
    fn operation_wire_source<'a>(&self, _mutation: &'a Mutation) -> Option<ArtifactPreparedOperationSource<'a>> { None }
    /// 🏷️ Borrows the `(entity, kind)` schema pair of a domain-codec mutation; `None` defers to the store's canonical [`ArtifactOperationWire`].
    fn operation_schema_parts<'a>(&'a self, _mutation: &'a Mutation) -> Option<(&'a str, &'a str)> { None }
    fn preflight(&self, mutation: &Mutation, lane: HistoryLane) -> Result<ArtifactStoreOneItemFootprint, String>;
    fn begin_demand(&self, mutation: &Mutation, lane: HistoryLane) -> Result<RetainedCloneBirthDemand, ValueError>;
    fn begin(&self, request: ArtifactStoreOneItemPreparationRequest<P, Mutation, Mutation>, grant: ArtifactStoreOneItemGrant) -> Result<(Box<dyn ArtifactStoreOneItemPreparation<P, Mutation>>, RetainedCloneProgress), (ValueError, ArtifactStoreOneItemPreparationRequest<P, Mutation, Mutation>)>;

    /// 🕰️ Declares a server-derived publication clock the factory will stamp on its prepared edit,
    /// so the Store mints the authority at that exact tick instead of its own wall-clock one. A
    /// factory that returns `None` keeps the Store's clock, which is what every local gesture wants.
    fn stamped_clock(&self) -> Option<HybridLogicalTimestamp> {
        None
    }

    /// 🔏 Declares the server-derived mutation identity the factory will stamp on its prepared edit.
    /// A stamped publication is hashed into a durable decision under this exact identity before the
    /// Store ever sees it, so the Store must publish under it rather than mint its own — otherwise
    /// the committed edit carries an identity no verifier can bind to the approval it executed.
    /// `None` keeps the Store's content-addressed mint, which is what every local gesture wants.
    fn stamped_mutation_id(&self) -> Option<&MutationId> {
        None
    }
}

/// 🏭️ The per-item app authority one batched gesture draws on, erased over whichever input its
/// lane admits: a typed mutation for the app's own document lane, an owned wire item for a member.
/// The batch machine calls it exactly once per staged mutation and never inspects the input.
pub(crate) trait ArtifactStoreBatchItemAuthority<P, Mutation>: Send + Sync {
    type Input: Send + 'static;
    fn operation_wire_source<'a>(&self, _mutation: &'a Mutation) -> Option<ArtifactPreparedOperationSource<'a>> { None }
    fn operation_schema_parts<'a>(&'a self, _mutation: &'a Mutation) -> Option<(&'a str, &'a str)> { None }

    fn preflight(&self, input: &Self::Input, lane: HistoryLane) -> Result<ArtifactStoreOneItemFootprint, String>;
    fn begin_demand(&self, input: &Self::Input, lane: HistoryLane) -> Result<RetainedCloneBirthDemand, ValueError>;
    fn begin(&self, request: ArtifactStoreOneItemPreparationRequest<P, Self::Input, Mutation>, grant: ArtifactStoreOneItemGrant) -> Result<(Box<dyn ArtifactStoreOneItemPreparation<P, Mutation>>, RetainedCloneProgress), (ValueError, ArtifactStoreOneItemPreparationRequest<P, Self::Input, Mutation>)>;

    fn input_retirement_birth_bytes(&self, input: &Self::Input) -> usize;
    fn retire_input(&self, input: Self::Input, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, Self::Input)>;
    fn into_retirement_authorities(self) -> [Option<semio_framework_value::FactoryAuthority>; 2];
    fn stamped_clock(&self) -> Option<HybridLogicalTimestamp> {
        None
    }

    fn stamped_mutation_id(&self) -> Option<&MutationId> {
        None
    }
}

pub(crate) struct ArtifactTypedBatchItemAuthority<P, Mutation> {
    pub(crate) preparation: Arc<dyn ArtifactStoreOneItemPreparationFactory<P, Mutation>>,
    pub(crate) retirement: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
}

impl<P: Send + Sync, Mutation: Send + 'static> ArtifactStoreBatchItemAuthority<P, Mutation> for ArtifactTypedBatchItemAuthority<P, Mutation> {
    type Input = Mutation;
    fn operation_wire_source<'a>(&self, mutation: &'a Mutation) -> Option<ArtifactPreparedOperationSource<'a>> { self.preparation.as_ref().operation_wire_source(mutation) }
    fn operation_schema_parts<'a>(&'a self, mutation: &'a Mutation) -> Option<(&'a str, &'a str)> { self.preparation.as_ref().operation_schema_parts(mutation) }

    fn preflight(&self, input: &Mutation, lane: HistoryLane) -> Result<ArtifactStoreOneItemFootprint, String> {
        self.preparation.as_ref().preflight(input, lane)
    }

    fn begin_demand(&self, input: &Mutation, lane: HistoryLane) -> Result<RetainedCloneBirthDemand, ValueError> { self.preparation.as_ref().begin_demand(input, lane) }
    fn begin(&self, request: ArtifactStoreOneItemPreparationRequest<P, Mutation, Mutation>, grant: ArtifactStoreOneItemGrant) -> Result<(Box<dyn ArtifactStoreOneItemPreparation<P, Mutation>>, RetainedCloneProgress), (ValueError, ArtifactStoreOneItemPreparationRequest<P, Mutation, Mutation>)> {
        self.preparation.as_ref().begin(request, grant)
    }

    fn input_retirement_birth_bytes(&self, input: &Mutation) -> usize { self.retirement.retirement_birth_bytes(input) }
    fn retire_input(&self, input: Mutation, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, Mutation)> { self.retirement.retire_owned(input, grant) }
    fn into_retirement_authorities(self) -> [Option<semio_framework_value::FactoryAuthority>; 2] {
        let preparation: Arc<dyn semio_framework_value::FactoryRetirement> = self.preparation;
        let retirement: Arc<dyn semio_framework_value::FactoryRetirement> = self.retirement;
        [Some(semio_framework_value::FactoryAuthority::new(preparation)), Some(semio_framework_value::FactoryAuthority::new(retirement))]
    }
    fn stamped_clock(&self) -> Option<HybridLogicalTimestamp> {
        self.preparation.as_ref().stamped_clock()
    }

    fn stamped_mutation_id(&self) -> Option<&MutationId> {
        self.preparation.as_ref().stamped_mutation_id()
    }
}

pub(crate) struct ArtifactStoreWireRetirement { pub(crate) schema: Option<String>, pub(crate) bytes: Option<Vec<u8>> }

impl ErasedSnapshotRetirement for ArtifactStoreWireRetirement {
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }
    fn next_capacity_byte_demand(&self, _body: usize) -> Result<usize, ValueError> { Ok(0) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.schema.as_ref().map(|value| value.capacity()).or_else(|| self.bytes.as_ref().map(|value| value.capacity())).unwrap_or(0)) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(usize::from(!self.terminal_is_empty())) }
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneStep, RetainedCloneProgress};
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if grant.maximum_depth == 0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "wire retirement requires admitted depth")); }
        let release = self.next_release_byte_demand()?;
        if release > grant.maximum_release_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if self.schema.is_some() { self.schema = None; } else { self.bytes = None; }
        let receipt = RetainedCloneProgress { copied_items: 1, released_bytes: release, ..Default::default() };
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(receipt) } else { RetainedCloneStep::Progress(receipt) })
    }
    fn terminal_is_empty(&self) -> bool { self.schema.is_none() && self.bytes.is_none() }
}

/// 🧳 One item's exact owner bundle, minted by the store from the batch's single shared live
/// authority and the running post root the item before it produced.
pub(crate) struct ArtifactStoreBatchItemRequest<P, Mutation> {
    pub(crate) operation: semio_framework_job::OperationId,
    pub(crate) generation: semio_framework_job::Generation,
    pub(crate) base_revision: [u8; 32],
    pub(crate) lane: HistoryLane,
    pub(crate) authority: Arc<ArtifactStoreOneItemLiveAuthority>,
    pub(crate) base: SnapshotRead<P>,
    pub(crate) mutation_retirement: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
    pub(crate) snapshot_retirement: Arc<dyn SnapshotRetirementFactory<P>>,
}

/// 🧺️ The still-unstaged inputs of one admitted gesture. `remaining` is the only census the batch
/// machine reads; every owner leaves through `begin_next` or one bounded `close_step`.
pub(crate) trait ArtifactStoreBatchSource<P, Mutation>: Send {
    fn operation_wire_source<'a>(&self, mutation: &'a Mutation) -> Option<ArtifactPreparedOperationSource<'a>>;
    fn operation_schema_parts<'a>(&'a self, mutation: &'a Mutation) -> Option<(&'a str, &'a str)>;
    fn remaining(&self) -> usize;
    fn census_one(&mut self, lane: HistoryLane) -> Result<Option<ArtifactStoreOneItemFootprint>, String>;
    fn next_begin_demand(&self, lane: HistoryLane) -> Result<RetainedCloneBirthDemand, ValueError>;
    fn begin_next(&mut self, request: ArtifactStoreBatchItemRequest<P, Mutation>, grant: ArtifactStoreOneItemGrant) -> Result<(Box<dyn ArtifactStoreOneItemPreparation<P, Mutation>>, RetainedCloneProgress), (ValueError, ArtifactStoreBatchItemRequest<P, Mutation>)>;
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
    fn retirement_demands(&self, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError>;
    fn terminal_is_empty(&self) -> bool;
}

pub(crate) struct ArtifactStoreBatchSourceOf<P, Mutation, A: ArtifactStoreBatchItemAuthority<P, Mutation>> {
    pub(crate) authority: Option<A>,
    pub(crate) inputs: VecDeque<A::Input>,
    census: usize,
    footprint: ArtifactStoreOneItemFootprint,
    issuer: Option<member_owned_batch::SourceRetirementIssuer<A::Input>>,
    retirement: Option<Box<dyn member_owned_batch::BatchRetirement>>,
    active_input: Option<Box<dyn ErasedSnapshotRetirement>>,
    authority_close: [Option<semio_framework_value::FactoryAuthority>; 2],
    wire: Option<ArtifactOperationWire<Mutation>>,
    marker: PhantomData<fn() -> (P, Mutation)>,
}

impl<P, Mutation, A: ArtifactStoreBatchItemAuthority<P, Mutation>> ArtifactStoreBatchSourceOf<P, Mutation, A> {
    pub(crate) fn new(authority: A, inputs: Vec<A::Input>, issuer: Option<member_owned_batch::SourceRetirementIssuer<A::Input>>, wire: Option<ArtifactOperationWire<Mutation>>) -> Self {
        Self { authority: Some(authority), inputs: inputs.into(), census: 0, footprint: ArtifactStoreOneItemFootprint { work_items: 0, retained_bytes: 0 }, issuer, retirement: None, active_input: None, authority_close: Default::default(), wire, marker: PhantomData }
    }
}

impl<P: Send, Mutation, A: ArtifactStoreBatchItemAuthority<P, Mutation>> ArtifactStoreBatchSource<P, Mutation> for ArtifactStoreBatchSourceOf<P, Mutation, A> {
    fn operation_wire_source<'a>(&self, mutation: &'a Mutation) -> Option<ArtifactPreparedOperationSource<'a>> { self.authority.as_ref()?.operation_wire_source(mutation).or_else(|| self.wire.and_then(|wire| wire.source(mutation))) }
    fn operation_schema_parts<'a>(&'a self, mutation: &'a Mutation) -> Option<(&'a str, &'a str)> { self.authority.as_ref()?.operation_schema_parts(mutation).or_else(|| self.wire.and_then(|wire| wire.schema(mutation))) }
    fn remaining(&self) -> usize { self.inputs.len() }
    fn census_one(&mut self, lane: HistoryLane) -> Result<Option<ArtifactStoreOneItemFootprint>, String> {
        let Some(input) = self.inputs.get(self.census) else { return Ok(Some(self.footprint)); };
        let item = self.authority.as_ref().ok_or_else(|| "batch source lost its exact app-owned item authority".to_string())?.preflight(input, lane)?;
        let footprint = self.footprint.merged(item);
        if !item.is_admissible() || !footprint.is_admissible() { return Err("one-item preparation footprint exceeds its fixed item or byte capacity".into()); }
        self.footprint = footprint;
        self.census += 1;
        Ok(None)
    }
    fn next_begin_demand(&self, lane: HistoryLane) -> Result<RetainedCloneBirthDemand, ValueError> {
        let authority=self.authority.as_ref().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"batch source lost its original item authority"))?;
        let input=self.inputs.front().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"batch source has no original input before preparation birth"))?;
        authority.begin_demand(input,lane)
    }
    fn begin_next(&mut self, request: ArtifactStoreBatchItemRequest<P, Mutation>, grant: ArtifactStoreOneItemGrant) -> Result<(Box<dyn ArtifactStoreOneItemPreparation<P, Mutation>>, RetainedCloneProgress), (ValueError, ArtifactStoreBatchItemRequest<P, Mutation>)> {
        let demand=match self.next_begin_demand(request.lane){Ok(demand)=>demand,Err(error)=>return Err((error,request))};
        if let Err(error)=demand.admit(grant.retained_grant()){return Err((error,request));}
        let authority=self.authority.as_ref().expect("observed original batch item authority");
        let mutation=self.inputs.pop_front().expect("observed original batch item input");
        let ArtifactStoreBatchItemRequest{operation,generation,base_revision,lane,authority:live,base,mutation_retirement,snapshot_retirement}=request;
        let item=ArtifactStoreOneItemPreparationRequest{operation,generation,base_revision,lane,authority:live,base,mutation,mutation_retirement,snapshot_retirement};
        match authority.begin(item,grant){
            Ok(preparation)=>Ok(preparation),
            Err((error,item))=>{
                let ArtifactStoreOneItemPreparationRequest{operation,generation,base_revision,lane,authority,base,mutation,mutation_retirement,snapshot_retirement}=item;
                self.inputs.push_front(mutation);
                Err((error,ArtifactStoreBatchItemRequest{operation,generation,base_revision,lane,authority,base,mutation_retirement,snapshot_retirement}))
            }
        }
    }
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneStep, RetainedCloneProgress};
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let demand = self.retirement_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "batch source exceeds admitted depth")); }
        if demand.copy_bytes > grant.maximum_copy_bytes || demand.capacity_bytes > grant.maximum_capacity_bytes || demand.release_bytes > grant.maximum_release_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if let Some(retirement) = self.retirement.as_mut() {
            if !retirement.terminal_is_empty() { let step = retirement.step(child)?; let step = semio_framework_value::retained_clone::admit_retained_clone_close(child, step, retirement.terminal_is_empty(), "batch source typed retirement")?; return Ok(RetainedCloneStep::Progress(step.progress())); }
            self.retirement = None;
            self.issuer = None;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..Default::default() }));
        }
        if let Some(issuer) = self.issuer.as_ref() {
            return match (issuer.begin)(std::mem::take(&mut self.inputs), child) {
                Ok((owner, progress)) => { self.retirement = Some(owner); Ok(RetainedCloneStep::Progress(progress)) },
                Err((error, original)) => { self.inputs = original; Err(error) },
            };
        }
        if self.active_input.is_some() { return artifact_retirement_box_close_step(&mut self.active_input, child).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if let Some(input) = self.inputs.pop_front() {
            return match self.authority.as_ref().expect("original batch input authority").retire_input(input, child) {
                Ok((owner, progress)) => { self.active_input = Some(owner); if !progress.fits(child) || progress.retained_capacity_bytes != demand.capacity_bytes { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "batch input constructor changed its receipt")); } Ok(RetainedCloneStep::Progress(progress)) },
                Err((error, original)) => { self.inputs.push_front(original); Err(error) },
            };
        }
        if self.inputs.capacity() != 0 {
            self.inputs = VecDeque::new();
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..Default::default() }));
        }
        if let Some(authority) = self.authority.take() {
            self.authority_close = authority.into_retirement_authorities();
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }));
        }
        if let Some(slot) = self.authority_close.iter_mut().find(|slot| slot.is_some()) {
            let authority = slot.as_mut().unwrap();
            let step = authority.step(child)?;
            let step = semio_framework_value::retained_clone::admit_retained_clone_close(child, step, authority.terminal_is_empty(), "batch source authority")?;
            if authority.terminal_is_empty() { *slot = None; }
            return Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) });
        }
        Ok(RetainedCloneStep::Complete(Default::default()))
    }
    fn retirement_demands(&self, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        use semio_framework_value::RetirementDemand;
        let nested = |mut demand: RetirementDemand| -> Result<RetirementDemand, ValueError> { demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "batch source depth overflow"))?; Ok(demand) };
        if let Some(retirement) = self.retirement.as_ref() { return if retirement.terminal_is_empty() { Ok(RetirementDemand { release_bytes: size_of_val(retirement.as_ref()), depth: 1, ..Default::default() }) } else { nested(retirement.demands(maximum_body_bytes)?) }; }
        if let Some(issuer) = self.issuer.as_ref() { return Ok(RetirementDemand { capacity_bytes: issuer.birth_bytes, depth: 2, ..Default::default() }); }
        if let Some(active) = self.active_input.as_ref() { return nested(artifact_retirement_box_demands(active, maximum_body_bytes)?); }
        if let Some(input) = self.inputs.front() { return Ok(RetirementDemand { capacity_bytes: self.authority.as_ref().expect("original batch authority").input_retirement_birth_bytes(input), depth: 2, ..Default::default() }); }
        if self.inputs.capacity() != 0 { return Ok(RetirementDemand { release_bytes: self.inputs.capacity().checked_mul(size_of::<A::Input>()).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "batch source backing layout overflow"))?, depth: 1, ..Default::default() }); }
        if self.authority.is_some() { return Ok(RetirementDemand { copy_bytes: size_of::<A>(), depth: 1, ..Default::default() }); }
        if let Some(authority) = self.authority_close.iter().find_map(Option::as_ref) { return nested(authority.demands(maximum_body_bytes)?); }
        Ok(Default::default())
    }
    fn terminal_is_empty(&self) -> bool { self.inputs.capacity() == 0 && self.inputs.is_empty() && self.authority.is_none() && self.retirement.is_none() && self.issuer.is_none() && self.active_input.is_none() && self.authority_close.iter().all(Option::is_none) }
}

/// 📨 Owned member wire, transferred without decoding or cloning at the composition boundary.
#[derive(Debug)]
pub struct MemberStoreOneItemWire {
    pub schema: String,
    pub bytes: Vec<u8>,
}

/// 🧾 Exact member revision and grouped-history metadata accompanying one owned wire item.
pub struct MemberStoreOneItemWireRequest {
    pub operation: semio_framework_job::OperationId,
    pub expected_generation: u64,
    pub expected_revision: [u8; 32],
    pub actor: semio_framework_value::SharedUtf8,
    pub group_id: Option<String>,
    pub wire: MemberStoreOneItemWire,
}

/// 🏭 A member's retained decoder and semantic preparation, never a generic `decode_op` fallback.
pub trait MemberStoreOneItemWirePreparationFactory<P, Mutation>: semio_framework_value::FactoryRetirement + Send + Sync {
    fn preflight(&self, wire: &MemberStoreOneItemWire, lane: HistoryLane) -> Result<ArtifactStoreOneItemFootprint, String>;
    fn begin_demand(&self, wire: &MemberStoreOneItemWire, lane: HistoryLane) -> Result<RetainedCloneBirthDemand, ValueError>;
    fn begin(&self, request: ArtifactStoreOneItemPreparationRequest<P, MemberStoreOneItemWire, Mutation>, grant: ArtifactStoreOneItemGrant) -> Result<(Box<dyn ArtifactStoreOneItemPreparation<P, Mutation>>, RetainedCloneProgress), (ValueError, ArtifactStoreOneItemPreparationRequest<P, MemberStoreOneItemWire, Mutation>)>;
}

/// 🪪 Erased retained publication; only its exact member may advance the typed candidate.
pub trait ErasedMemberStoreOneItemPublication: Send {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn phase(&self) -> ArtifactStoreOneItemPublicationPhase;
    fn progress(&self) -> ArtifactStoreOneItemCheckpoint;
    fn prepared_edit_id(&self) -> Option<&str>;
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
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
    fn terminal_is_empty(&self) -> bool;
}

pub(crate) struct MemberStoreOneItemPublication<P, Mutation> {
    pub(crate) member: Option<SnapshotReadRegistryAliasRetirement>,
    pub(crate) publication: ArtifactStoreBatchPublication<P, Mutation>,
    pub(crate) group_history: Option<ArtifactStoreHistoryCommitReservation>,
    pub(crate) group_displaced: Option<ArtifactStoreDisplacedOwnerReservation>,
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

/// 🔄 Persistent store-side operation phases. Publication and ACK are distinct turns; close
/// remains observable until the domain owner proves exact terminal emptiness.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub enum ArtifactStoreOneItemPublicationPhase {
    Preparing,
    PreparingCursor,
    PreflightingCommit,
    Publishing,
    AwaitingAck,
    Closing,
    Complete,
    Fault,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtifactStoreOneItemAdvance {
    Progress(ArtifactStoreOneItemCheckpoint),
    PreparationProgress(ArtifactStoreOneItemCheckpoint, RetainedCloneProgress),
    PreparationRefused,
    Published(LaneItemReceipt),
    AwaitingAck(LaneItemReceipt),
    Complete,
    Blocked,
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
pub(crate) struct ArtifactStoreBatchStage<P, Mutation> {
    pub(crate) unit_flags: Vec<(usize, bool)>,
    pub(crate) folding: Option<Box<ArtifactStorePreparedRetirement<P, Mutation>>>,
    pub(crate) fold_phase: u8,
    pub(crate) digest_owner: Option<Box<dyn ArtifactStoreBatchDigest<Mutation>>>,
    pub(crate) digest_complete: bool,
    pub(crate) edit: Option<Box<Edit<Mutation>>>,
    pub(crate) pending_edit: Option<Edit<Mutation>>,
    pub(crate) post: Option<Arc<P>>,
    pub(crate) next_clock: HybridLogicalTimestamp,
    pub(crate) local_actor: Option<semio_framework_value::paged::PagedUtf8<{usize::MAX}>>,
    pub(crate) applied_edit_id: String,
    pub(crate) tail_edit_id: String,
    pub(crate) digest: [u8; 32],
    pub(crate) folded: usize,
    pub(crate) completed_items: u32,
    pub(crate) completed_bytes: u64,
    pub(crate) mutation_retirement: Option<Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>>,
    pub(crate) snapshot_retirement: Option<Arc<dyn SnapshotRetirementFactory<P>>>,
    pub(crate) retiring: Option<Box<dyn ErasedSnapshotRetirement>>,
    pub(crate) factory_close: [Option<semio_framework_value::FactoryAuthority>; 2],
}

impl<P, Mutation> ArtifactStoreBatchStage<P, Mutation> {
    fn checkpoint(&self) -> ArtifactStoreOneItemCheckpoint {
        ArtifactStoreOneItemCheckpoint { cursor: self.folded as u32, completed_items: self.completed_items, completed_bytes: self.completed_bytes, digest: self.digest }
    }

    fn retirement_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError>
    where P: Send + Sync + 'static, Mutation: Send + 'static,
    {
        use semio_framework_value::RetirementDemand;
        let nested = |mut demand: RetirementDemand| -> Result<RetirementDemand, ValueError> { demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "batch stage depth overflow"))?; Ok(demand) };
        if let Some(owner) = self.retiring.as_ref() { return nested(artifact_retirement_box_demands(owner, body)?); }
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
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>
    where P: Send + Sync + 'static, Mutation: Send + 'static,
    {
        use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep, RetainedCloneProgress};
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let demand = self.retirement_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "batch stage exceeds admitted depth")); }
        if demand.copy_bytes > grant.maximum_copy_bytes || demand.capacity_bytes > grant.maximum_capacity_bytes || demand.release_bytes > grant.maximum_release_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if self.retiring.is_some() { return artifact_retirement_box_close_step(&mut self.retiring, child).map(|step| RetainedCloneStep::Progress(step.progress())); }
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
    fn terminal_is_empty(&self) -> bool {
        self.digest_owner.is_none() && self.folding.is_none() && self.edit.is_none() && self.pending_edit.is_none() && self.retiring.is_none() && self.post.is_none() && self.unit_flags.capacity() == 0 && self.applied_edit_id.capacity() == 0 && self.tail_edit_id.capacity() == 0 && self.local_actor.is_none() && self.mutation_retirement.is_none() && self.snapshot_retirement.is_none() && self.factory_close.iter().all(Option::is_none)
    }
}

/// 📬️ Retained batched publication authority: ONE user gesture, N admitted mutations, ONE staged
/// `Edit`, ONE history ledger slot, ONE undo step. Each turn folds at most one mutation — the
/// single-mutation case is simply the N = 1 batch. It never calls `apply_one`, `apply_command`,
/// `replay_mutations`, `pump`, or `flush_outbound`.
pub struct ArtifactStoreBatchPublication<P, Mutation> {
    pub(crate) operation: semio_framework_job::OperationId,
    pub(crate) expected_generation: u64,
    pub(crate) expected_revision: [u8; 32],
    pub(crate) lane: HistoryLane,
    pub(crate) footprint: ArtifactStoreOneItemFootprint,
    pub(crate) source_census_completed: u32,
    pub(crate) source_census_finished: bool,
    pub(crate) admitted_items: usize,
    pub(crate) authority: Option<Arc<ArtifactStoreOneItemLiveAuthority>>,
    pub(crate) authority_retirement: Option<Box<dyn ErasedSnapshotRetirement>>,
    pub(crate) source: Option<Box<dyn ArtifactStoreBatchSource<P, Mutation>>>,
    pub(crate) preparation: Option<Box<dyn ArtifactStoreOneItemPreparation<P, Mutation>>>,
    pub(crate) item_closing: bool,
    pub(crate) stage: Option<Box<ArtifactStoreBatchStage<P, Mutation>>>,
    pub(crate) group_preparation: Option<member_group::MemberGroupPreparation>,
    pub(crate) retained_checkpoint: ArtifactStoreOneItemCheckpoint,
    pub(crate) receipt: Option<LaneItemReceipt>,
    pub(crate) attempts: u8,
    pub(crate) published: bool,
    pub(crate) cancel_requested: bool,
    pub(crate) close_started: bool,
    pub(crate) fault: Option<ValueError>,
    pub(crate) refused_base: Option<SnapshotRead<P>>,
    pub(crate) refused_authority: Option<Arc<ArtifactStoreOneItemLiveAuthority>>,
    pub(crate) refused_issuers: [Option<semio_framework_value::FactoryAuthority>; 2],
    pub(crate) phase: ArtifactStoreOneItemPublicationPhase,
    pub(crate) verb: Option<String>,
    pub(crate) transaction: Option<protocol::TransactionRef>,
    pub(crate) outbound: bool,
    pub(crate) announce_from: usize,
    pub(crate) announce_items: usize,
    pub(crate) transaction_open: bool,
}

impl<P, Mutation> ArtifactStoreBatchPublication<P, Mutation> {
    /// 🎟️ Current private group copy or allocation demand, never accumulated credit.
    pub fn next_group_byte_demand(&self) -> usize { self.group_preparation.as_ref().map_or(1, |group| group.next_byte_demand()) }

    /// 📏️ Names every physical and logical close axis before transferring the next owner.
    pub fn retirement_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError>
    where P: Send + Sync + 'static, Mutation: Send + 'static,
    {
        use semio_framework_value::RetirementDemand;
        let nested = |mut demand: RetirementDemand| -> Result<RetirementDemand, ValueError> { demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "publication depth overflow"))?; Ok(demand) };
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
        Ok(RetirementDemand { depth: usize::from(!self.terminal_is_empty()), ..Default::default() })
    }

    pub fn operation(&self) -> semio_framework_job::OperationId {
        self.operation
    }

    pub fn phase(&self) -> ArtifactStoreOneItemPublicationPhase {
        self.phase
    }

    /// 🏷️ The id of the action or command this gesture publishes for, stamped on the gesture's new edit (an amend keeps
    /// the verb of the edit it extends). See [`crate::os_spr::command::Edit::verb`].
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
    pub(crate) fn take_staged_prepared(&mut self) -> Option<ArtifactStoreOneItemPrepared<P, Mutation>> {
        if self.phase != ArtifactStoreOneItemPublicationPhase::Publishing || self.stage.as_ref().is_none_or(|stage| stage.post.is_none()) {
            return None;
        }
        let authority = Arc::clone(self.authority.as_ref()?);
        let stage = self.take_stage()?;
        let ArtifactStoreBatchStage { edit, post, local_actor: _, applied_edit_id, tail_edit_id, digest, .. } = *stage;
        let edit = edit.expect("live staged edit owner");
        let post = post.expect("validated staged batch post root");
        Some(authority.seal_prepared_owned(edit, post, digest, [authority.actor.as_str().into(), applied_edit_id.into(), tail_edit_id.into()]))
    }

    pub(crate) fn take_stage(&mut self) -> Option<Box<ArtifactStoreBatchStage<P, Mutation>>> {
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
    }
}

impl<P: Send + Sync + 'static, Mutation: Send + 'static> ErasedSnapshotRetirement for ArtifactStoreBatchPublication<P, Mutation> {
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.retirement_demands(body)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.depth) }
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { ArtifactStoreBatchPublication::close_step(self, grant) }
    fn terminal_is_empty(&self) -> bool { ArtifactStoreBatchPublication::terminal_is_empty(self) }
}
