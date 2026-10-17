//! 📬️ The retained one-item publication protocol: the preparation trait and its factory, the live authority the Store
//! mints, the prepared owner and its seal, batch sources and the member one-item wire. Domain preparation owners and
//! the Store engine both speak it; it names only the one-item vocabulary, the ledger and snapshot-read owners below it.
//!
//! Layering: `📓️2026-10-13-wave-e1-os-kernel.md` (ticket BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT).
#[path = "📥️lifted/🏪️store/🦀️.rs"]
pub mod lifted_store;
#[path = "📥️lifted/📬️publication/🦀️.rs"]
pub mod lifted_publication;
#[path = "../../📦️prepared/♻️retirement/🦀️.rs"]
pub mod prepared_retirement;
#[path = "../../🧩️composition/📬️publication/🤝️group/🔏️digest/🦀️.rs"]
pub mod batch_digest;
#[path = "../../🧵️canonical-edit/🧳️source/🦀️.rs"]
pub mod native_source;
#[path = "📥️lifted/📝️text/🦀️.rs"]
pub mod lifted_text;
#[path = "📥️lifted/📦️owned/🦀️.rs"]
pub mod lifted_member_owned_batch;
#[path = "📥️lifted/🧵️canonical-edit/🦀️.rs"]
pub mod lifted_canonical_edit;
#[path = "📥️lifted/🧵️operation-wire/🦀️.rs"]
pub mod lifted_prepared_operation_wire;
use crate::batch_digest::ArtifactStoreBatchDigest;
use crate::lifted_member_owned_batch as member_owned_batch;

use crate::lifted_canonical_edit as canonical_edit;

use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementFactory};
use semio_framework_value::retained_clone::{RetainedCloneBirthDemand, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_os_spr::command::ArtifactMessageSource;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;
use semio_framework_os_spr::{ActorId, HybridLogicalTimestamp, MutationId, UndoPolicy};
use semio_framework_os_spr::{DiffRegions, Edit, Mutation, MutationDiff, MutationMeta, OpBinary, OpText};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::sync::Arc;
use semio_framework_value_snapshot_read::{SnapshotRead, artifact_retirement_box_close_step, artifact_retirement_box_demands};

use semio_framework_value_snapshot_read::{SnapshotReadRegistryAliasRetirement,snapshot_registry_alias_demands,snapshot_registry_alias_close_step};
use semio_framework_value::retirement::frame::admit_retirement_frame as admit_artifact_retirement;
use semio_framework_os_vcs::{Identified, Patchable};
use semio_framework_os_pack::pack_rt;
use semio_framework_os_store_ledger::CursorRevisionAccumulator;

use semio_framework_os_store_canonical_json::ArtifactCanonicalJson;
use semio_framework_pack_json::ArtifactCanonicalJsonTree;
use crate::lifted_canonical_edit::ArtifactStoreOneItemSealer;
use crate::lifted_prepared_operation_wire::{ArtifactOperationWire, ArtifactPreparedOperationSource};
use crate::lifted_text::ArtifactOperationText;
use semio_framework_os_store_one_item::{ARTIFACT_STORE_ONE_ITEM_ID_BYTES, ARTIFACT_STORE_ONE_ITEM_MAXIMUM_RETRIES, ArtifactStoreOneItemCheckpoint, ArtifactStoreOneItemFootprint, ArtifactStoreOneItemGrant, ArtifactStoreOneItemPreparationStep};
use semio_framework_os_store_command::HistoryLane;
use semio_framework_os_store_one_item::LaneItemReceipt;

#[path = "🪪️authority/🦀️.rs"]
mod publication_authority_retirement;

/// 🪪 Exact live publication metadata minted by the Store and shared immutably with the
/// domain preparation owner. Apps cannot invent cursor position, sequence, clock, or actor.
pub struct ArtifactStoreOneItemLiveAuthority {
    pub operation: semio_framework_job::OperationId,
    pub generation: semio_framework_job::Generation,
    pub base_revision: [u8; 32],
    pub base_applied_edit_count: usize,
    pub next_sequence_number: i32,
    pub next_clock: HybridLogicalTimestamp,
    pub actor: semio_framework_value::SharedUtf8,
    pub line: Option<String>,
    pub group_id: Option<String>,
    pub stamped_edit_id: Option<String>,
}

/// 🪪️ The capability a store engine presents to mint a live authority. Only a live store log hands an implementor out (`StoreLog::mint_witness`); a type that implements it elsewhere fabricates a publication authority and is a review finding.
pub trait StoreMintWitness {}

impl ArtifactStoreOneItemLiveAuthority {
    /// 🪙️ Mints the live authority of one Store publication. Every field is the Store's own operation, cursor, sequence,
    /// clock and actor state, so only the Store engine mints; a domain receives the minted authority, never its fields.
    pub fn mint(
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

    /// 🪙️ The cross-crate entry of [`Self::mint`]: the store engine presents the witness a live store log hands out, so the authority is never minted without one.
    pub fn mint_for_store(
        _witness: &dyn StoreMintWitness,
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
        Self::mint(operation, generation, base_revision, base_applied_edit_count, next_sequence_number, next_clock, actor, line, group_id, stamped_edit_id)
    }

    /// 🔗️ The same publication authority rebound under one durable group decision.
    pub fn rebind_group(&self, group_id: String) -> Self {
        Self { actor: self.actor.clone(), line: self.line.clone(), group_id: Some(group_id), stamped_edit_id: self.stamped_edit_id.clone(), ..*self }
    }

    /// 🎭️ The Store-minted actor text every edit of this publication carries.
    pub fn actor_utf8(&self) -> &semio_framework_value::SharedUtf8 {
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

    pub fn validate_semantic_edit<Mutation>(&self, edit: &Edit<Mutation>) -> Result<(), ValueError> {
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

    pub fn seal_prepared_owned<P, Mutation>(self: &Arc<Self>, edit: Box<Edit<Mutation>>, post_snapshot: Arc<P>, edit_digest: [u8; 32], identities: [semio_framework_value::paged::PagedUtf8<{usize::MAX}>; 3]) -> ArtifactStoreOneItemPrepared<P, Mutation> {
        let [actor, applied_edit_id, tail_edit_id] = identities;
        let seal = ArtifactStoreOneItemSeal { authority: Arc::clone(self), edit_address: edit.as_ref() as *const Edit<Mutation> as usize, post_address: Arc::as_ptr(&post_snapshot) as usize, digest: edit_digest };
        ArtifactStoreOneItemPrepared { foreign_step_presence: None, edit, post_snapshot, next_clock: self.next_clock, edit_digest, local_actor: Some(actor), applied_edit_id, tail_edit_id, seal }
    }

    pub fn validate_prepared<P, Mutation>(self: &Arc<Self>, prepared: &ArtifactStoreOneItemPrepared<P, Mutation>) -> Result<(), ValueError> {
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

pub struct ArtifactStoreOneItemSeal {
    pub authority: Arc<ArtifactStoreOneItemLiveAuthority>,
    edit_address: usize,
    post_address: usize,
    pub digest: [u8; 32],
}

/// 📦 Fully prepared one-item candidate. The domain must build every potentially unbounded
/// mutation inverse/diff/apply/encoding result inside its retained cursor before exposing this
/// value. The store publication turn only validates fixed authorities and moves these owners.
pub struct ArtifactStoreOneItemPrepared<P, Mutation> {
    pub edit: Box<Edit<Mutation>>,
    pub post_snapshot: Arc<P>,
    pub next_clock: HybridLogicalTimestamp,
    pub edit_digest: [u8; 32],
    pub foreign_step_presence: Option<bool>,
    pub local_actor: Option<semio_framework_value::paged::PagedUtf8<{usize::MAX}>>,
    pub applied_edit_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
    pub tail_edit_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
    pub seal: ArtifactStoreOneItemSeal,
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

#[cfg(any(test, feature = "testkit"))]
impl ArtifactStoreOneItemLiveAuthority {
    /// 🧪️ Rebinds the identity strings a law varies, on an authority no publication shares yet.
    pub fn rebind_identity_for_law(&mut self, actor: semio_framework_value::SharedUtf8, group_id: String, stamped_edit_id: String, line: String) {
        self.actor = actor;
        self.group_id = Some(group_id);
        self.stamped_edit_id = Some(stamped_edit_id);
        self.line = Some(line);
    }
}

#[cfg(any(test, feature = "testkit"))]
impl<P, Mutation> ArtifactStoreOneItemPrepared<P, Mutation> {
    /// 🧪️ Tamper seam for seal laws: the sealed edit, its digest and the post snapshot the seal covers.
    pub fn sealed_parts_mut(&mut self) -> (&mut Box<Edit<Mutation>>, &mut [u8; 32], &mut Arc<P>) {
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
pub trait ArtifactStoreBatchItemAuthority<P, Mutation>: Send + Sync {
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

pub struct ArtifactTypedBatchItemAuthority<P, Mutation> {
    pub preparation: Arc<dyn ArtifactStoreOneItemPreparationFactory<P, Mutation>>,
    pub retirement: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
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

pub struct ArtifactStoreWireRetirement { pub schema: Option<String>, pub bytes: Option<Vec<u8>> }

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
pub struct ArtifactStoreBatchItemRequest<P, Mutation> {
    pub operation: semio_framework_job::OperationId,
    pub generation: semio_framework_job::Generation,
    pub base_revision: [u8; 32],
    pub lane: HistoryLane,
    pub authority: Arc<ArtifactStoreOneItemLiveAuthority>,
    pub base: SnapshotRead<P>,
    pub mutation_retirement: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
    pub snapshot_retirement: Arc<dyn SnapshotRetirementFactory<P>>,
}

/// 🧺️ The still-unstaged inputs of one admitted gesture. `remaining` is the only census the batch
/// machine reads; every owner leaves through `begin_next` or one bounded `close_step`.
pub trait ArtifactStoreBatchSource<P, Mutation>: Send {
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

pub struct ArtifactStoreBatchSourceOf<P, Mutation, A: ArtifactStoreBatchItemAuthority<P, Mutation>> {
    pub authority: Option<A>,
    pub inputs: VecDeque<A::Input>,
    census: usize,
    pub footprint: ArtifactStoreOneItemFootprint,
    issuer: Option<member_owned_batch::SourceRetirementIssuer<A::Input>>,
    pub retirement: Option<Box<dyn member_owned_batch::BatchRetirement>>,
    active_input: Option<Box<dyn ErasedSnapshotRetirement>>,
    pub authority_close: [Option<semio_framework_value::FactoryAuthority>; 2],
    pub wire: Option<ArtifactOperationWire<Mutation>>,
    pub marker: PhantomData<fn() -> (P, Mutation)>,
}

impl<P, Mutation, A: ArtifactStoreBatchItemAuthority<P, Mutation>> ArtifactStoreBatchSourceOf<P, Mutation, A> {
    pub fn new(authority: A, inputs: Vec<A::Input>, issuer: Option<member_owned_batch::SourceRetirementIssuer<A::Input>>, wire: Option<ArtifactOperationWire<Mutation>>) -> Self {
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















/// 🔄 Persistent store-side operation phases. Publication and ACK are distinct turns; close
/// remains observable until the domain owner proves exact terminal emptiness.
pub use semio_framework_os_store_one_item::ArtifactStoreOneItemPublicationPhase;

pub use semio_framework_os_store_one_item::ArtifactStoreOneItemAdvance;
















