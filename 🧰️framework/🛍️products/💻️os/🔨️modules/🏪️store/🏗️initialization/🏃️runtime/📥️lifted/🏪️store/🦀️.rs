//! 📥️ Items of `store` that belong to `os-store-initialization-runtime` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/🦀️.rs`.
use semio_framework_os_store_text_io::ArtifactStoreSnapshot as _;
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;
use semio_framework_os_spr::{ActorId, ArtifactId, HybridLogicalTimestamp, MutationId};
use semio_framework_os_spr::{Edit, Mutation, OpBinary};
use std::collections::HashMap;
use std::sync::Arc;
use semio_framework_os_store_read::{artifact_retirement_box_close_step, artifact_retirement_box_demands};
use semio_framework_os_store_read::{ArtifactStoreMutationDagRetirement, ArtifactStoreRevisionAccumulatorRetirement, ArtifactStoreStringRetirement, ArtifactStoreStringVectorRetirement, ArtifactStoreSupersessionRetirement};
use semio_framework_value::retirement::frame::admit_retirement_frame as admit_artifact_retirement;
use semio_framework_value::factory::FactorySharedRetirement as ReturnedSnapshotReadRetirement;
use semio_framework_os_vcs::{ArtifactHistoryKey, ArtifactHistoryLedger};
use semio_framework_os_store_space_schema::{ArtifactCursor, ArtifactEnvelopeOwners};
//#region 🔖️EphemeralLanes

use semio_framework_os_store_ephemeral::*;
use pack::snapshot::ArtifactPack;
use semio_framework_os_store_text_io::ArtifactStoreSnapshot;
use crate::lifted_document_materialize::fresh_replica_clock;
use semio_framework_os_store_ledger::{ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES, ARTIFACT_EDIT_MESSAGE_OWNER_BYTES, ARTIFACT_EDIT_MESSAGE_TARGET_BYTES, CursorRevisionAccumulator, CursorRevisionRecord};
use semio_framework_os_store_replay::{EffectiveOperation, EffectiveSupersessions, effective_operation, fold_operation};
//#region 🔖️Authority
//#region 🔖️EphemeralLanes
//#region 🔖️Text
//#region 🔖️Pack
//#region 🔖️CodecRegistry
//#region 🔖️TextFormat
//#region 🔖️History
//#region 🔖️ArtifactStore
//#region 🔖️ArtifactStore
/// 🌱️ The identity digest of a document's genesis every load path names its content revision from — a whole load, a retained
/// initializer of any app — so one history reloaded either way names one revision (design §3.7).
pub fn artifact_initial_digest<P: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot>(initial: &P) -> [u8; 32] {
    artifact_initial_digest_of_pack(&initial.encode_pack())
}

/// 🌱️ [`artifact_initial_digest`] of a genesis already encoded as its pack.
pub fn artifact_initial_digest_of_pack(pack: &[u8]) -> [u8; 32] {
    *semio_framework_hash::hash(pack).as_bytes()
}

/// 🧮️ Domain-neutral incremental digest for retained store initialization. A
/// caller observes at most one already-admitted field/page per job step and never materializes a
/// serialized document or edit merely to derive revision identity.
pub struct ArtifactStoreInitializationDigest {
    pub lanes: [u64; 4],
    pub bytes: u64,
}

impl ArtifactStoreInitializationDigest {
    pub fn new(domain: &[u8]) -> Self {
        let mut digest = Self { lanes: [0xcbf2_9ce4_8422_2325, 0x9e37_79b9_7f4a_7c15, 0x6a09_e667_f3bc_c909, 0xbb67_ae85_84ca_a73b], bytes: 0 };
        digest.observe(domain);
        digest
    }

    pub fn observe(&mut self, bytes: &[u8]) {
        for (offset, byte) in bytes.iter().copied().enumerate() {
            let lane = (self.bytes as usize + offset) & 3;
            self.lanes[lane] ^= u64::from(byte);
            self.lanes[lane] = self.lanes[lane].wrapping_mul(0x0000_0100_0000_01b3).rotate_left((lane as u32 + 1) * 7);
        }
        self.bytes = self.bytes.checked_add(bytes.len() as u64).expect("pre-admitted initialization byte credits cannot overflow u64");
    }

    pub fn finish(self) -> [u8; 32] {
        let mut output = [0; 32];
        for (index, lane) in self.lanes.into_iter().enumerate() {
            output[index * 8..(index + 1) * 8].copy_from_slice(&lane.wrapping_add(self.bytes.rotate_left(index as u32 * 11)).to_be_bytes());
        }
        output
    }
}

/// 🗂️ The edit ids of an envelope a retained initializer validates, admitted one edit per job step ([`Self::admit`]) and
/// found in O(1) ([`Self::position`]): the one mechanism that keeps every store initializer's duplicate check and its
/// applied and redo lookups linear in the history.
#[derive(Debug, Default)]
pub struct ArtifactStoreInitializationEditIndex {
    positions: HashMap<String, usize>,
}

/// 🪜️ What admitting one edit to an [`ArtifactStoreInitializationEditIndex`] found.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtifactStoreInitializationEditAdmission {
    Admitted,
    Complete,
    Oversized,
    Duplicate,
}

impl ArtifactStoreInitializationEditIndex {
    /// ➕️ Admits the edit at `position` of `edits`: `Complete` once no edit is left, `Oversized` when its id is longer than
    /// `max_id_bytes`, `Duplicate` when it repeats an admitted edit's id.
    pub fn admit<Mutation>(&mut self, edits: &ArtifactHistoryLedger<Edit<Mutation>>, position: usize, max_id_bytes: usize) -> ArtifactStoreInitializationEditAdmission {
        match edits.get(position) {
            None => ArtifactStoreInitializationEditAdmission::Complete,
            Some(edit) if edit.id.len() > max_id_bytes => ArtifactStoreInitializationEditAdmission::Oversized,
            Some(edit) if self.positions.insert(edit.id.clone(), position).is_some() => ArtifactStoreInitializationEditAdmission::Duplicate,
            Some(_) => ArtifactStoreInitializationEditAdmission::Admitted,
        }
    }

    /// 🔎️ The ledger position of the admitted edit `edit_id`.
    pub fn position(&self, edit_id: &str) -> Option<usize> {
        self.positions.get(edit_id).copied()
    }

    /// 🧹️ Forgets every admitted id once the applied and redo lookups are done.
    pub fn clear(&mut self) {
        self.positions = HashMap::new();
    }

    /// 🧹️ Forgets at most `maximum` admitted ids once the applied and redo lookups are done — one bounded slice per job
    /// step, never the whole index at once — and answers whether none is left (its table released).
    pub fn retire_step(&mut self, maximum: usize) -> bool {
        self.positions.extract_if(|_, _| true).take(maximum).for_each(drop);
        if !self.positions.is_empty() {
            return false;
        }
        self.positions = HashMap::new();
        true
    }
}

/// ▶️ What folding one forward of an applied edit left ([`ArtifactStoreInitializationRuntime::fold_forward`]).
pub enum ArtifactStoreInitializationForward<P> {
    /// 🏁️ The edit has no forward at that index.
    Exhausted,
    /// ⏭️ The forward's effective input folded with keep-and-record semantics: `displaced` is the projection it replaced
    /// (`None` when it folded as a no-op — withdrawn, faulted or refused), which the caller retires through its own factory;
    /// `fuel` is the operation's encoded size (1 without an operation).
    Folded { displaced: Option<P>, fuel: usize },
}
//#endregion 🔖️ArtifactStore
//#endregion 🔖️ArtifactStore
//#endregion 🔖️History
//#endregion 🔖️TextFormat
//#endregion 🔖️CodecRegistry
//#endregion 🔖️Pack
//#endregion 🔖️Text
//#endregion 🔖️EphemeralLanes
//#endregion 🔖️Authority

//#region 🔖️Authority
//#region 🔖️EphemeralLanes
//#region 🔖️Text
//#region 🔖️Pack
//#region 🔖️CodecRegistry
//#region 🔖️TextFormat
//#region 🔖️History
//#region 🔖️ArtifactStore
//#region 🔖️ArtifactStore
/// 🏗️ Exact runtime owners assembled by a domain's retained store initializer.
/// Every mutating method advances one already-admitted history or reference owner; final store
/// construction only moves these prepared authorities into their from-birth terminal shells.
pub struct ArtifactStoreInitializationOwnerCatalog {
    pub applied_edit_ids: semio_framework_os_vcs::HistoryPageStack<String>,
    pub redo_edit_ids: semio_framework_os_vcs::HistoryPageStack<String>,
    pub cursor_applied_edit_ids: semio_framework_os_vcs::HistoryPageStack<String>,
    pub cursor_redo_edit_ids: semio_framework_os_vcs::HistoryPageStack<String>,
    pub applied_revision: semio_framework_os_vcs::HistoryPageStack<CursorRevisionRecord>,
    pub redo_revision: semio_framework_os_vcs::HistoryPageStack<CursorRevisionRecord>,
}

impl ArtifactStoreInitializationOwnerCatalog {
    pub fn retain_id_capacity(ids: Vec<String>) -> semio_framework_os_vcs::HistoryPageStack<String> {
        ids.into()
    }

    pub fn try_new() -> Result<Self, &'static str> {
        Ok(Self {
            applied_edit_ids: semio_framework_os_vcs::HistoryPageStack::try_new().map_err(|_| "artifact store applied owner catalog allocation failed")?,
            redo_edit_ids: semio_framework_os_vcs::HistoryPageStack::try_new().map_err(|_| "artifact store redo owner catalog allocation failed")?,
            cursor_applied_edit_ids: semio_framework_os_vcs::HistoryPageStack::try_new().map_err(|_| "artifact store cursor-applied owner catalog allocation failed")?,
            cursor_redo_edit_ids: semio_framework_os_vcs::HistoryPageStack::try_new().map_err(|_| "artifact store cursor-redo owner catalog allocation failed")?,
            applied_revision: semio_framework_os_vcs::HistoryPageStack::try_new().map_err(|_| "artifact store applied revision catalog allocation failed")?,
            redo_revision: semio_framework_os_vcs::HistoryPageStack::try_new().map_err(|_| "artifact store redo revision catalog allocation failed")?,
        })
    }

    pub fn admitted_items(&self) -> usize {
        self.applied_edit_ids.capacity() + self.redo_edit_ids.capacity() + self.cursor_applied_edit_ids.capacity() + self.cursor_redo_edit_ids.capacity() + self.applied_revision.capacity() + self.redo_revision.capacity()
    }

    pub fn admitted_bytes(&self) -> Option<usize> {
        self.applied_edit_ids
            .capacity()
            .checked_mul(size_of::<String>())?
            .checked_add(self.redo_edit_ids.capacity().checked_mul(size_of::<String>())?)?
            .checked_add(self.cursor_applied_edit_ids.capacity().checked_mul(size_of::<String>())?)?
            .checked_add(self.cursor_redo_edit_ids.capacity().checked_mul(size_of::<String>())?)?
            .checked_add(self.applied_revision.capacity().checked_mul(size_of::<CursorRevisionRecord>())?)?
            .checked_add(self.redo_revision.capacity().checked_mul(size_of::<CursorRevisionRecord>())?)
    }
}

pub struct ArtifactStoreInitializationRuntime<P> {
    pub current: std::mem::ManuallyDrop<Option<Arc<P>>>,
    pub applied_edit_ids: std::mem::ManuallyDrop<semio_framework_os_vcs::HistoryPageStack<String>>,
    pub redo_edit_ids: std::mem::ManuallyDrop<semio_framework_os_vcs::HistoryPageStack<String>>,
    pub cursor_applied_edit_ids: std::mem::ManuallyDrop<semio_framework_os_vcs::HistoryPageStack<String>>,
    pub cursor_redo_edit_ids: std::mem::ManuallyDrop<semio_framework_os_vcs::HistoryPageStack<String>>,
    pub current_checkpoint_id: std::mem::ManuallyDrop<Option<String>>,
    pub local_actor_id: std::mem::ManuallyDrop<ActorId>,
    pub dag: std::mem::ManuallyDrop<Option<semio_framework_os_spr::MutationDag>>,
    pub edit_sequence: i32,
    pub clock: HybridLogicalTimestamp,
    pub initial_digest: [u8; 32],
    pub revision: std::mem::ManuallyDrop<CursorRevisionAccumulator>,
    pub close_active: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    pub supersessions: std::mem::ManuallyDrop<EffectiveSupersessions>,
    pub(crate) close_phase: u8,
    taken: bool,
}

impl<P> ArtifactStoreInitializationRuntime<P> {
    pub fn new(artifact_id: &str, schema: &str, current: Arc<P>, initial_digest: [u8; 32], actor: ActorId) -> Self {
        let catalog = ArtifactStoreInitializationOwnerCatalog::try_new().expect("artifact store initialization owner catalog must be pre-admitted");
        Self::new_with_owner_catalog(artifact_id, schema, current, initial_digest, actor, catalog)
    }

    pub fn new_with_owner_catalog(artifact_id: &str, schema: &str, current: Arc<P>, initial_digest: [u8; 32], actor: ActorId, catalog: ArtifactStoreInitializationOwnerCatalog) -> Self {
        let identity_digest = CursorRevisionAccumulator::hash_record(b"initial", &[artifact_id.as_bytes(), schema.as_bytes(), &initial_digest]);
        let ArtifactStoreInitializationOwnerCatalog { applied_edit_ids, redo_edit_ids, cursor_applied_edit_ids, cursor_redo_edit_ids, applied_revision, redo_revision } = catalog;
        Self {
            current: std::mem::ManuallyDrop::new(Some(current)),
            applied_edit_ids: std::mem::ManuallyDrop::new(applied_edit_ids),
            redo_edit_ids: std::mem::ManuallyDrop::new(redo_edit_ids),
            cursor_applied_edit_ids: std::mem::ManuallyDrop::new(cursor_applied_edit_ids),
            cursor_redo_edit_ids: std::mem::ManuallyDrop::new(cursor_redo_edit_ids),
            current_checkpoint_id: std::mem::ManuallyDrop::new(None),
            local_actor_id: std::mem::ManuallyDrop::new(actor),
            dag: std::mem::ManuallyDrop::new(Some(semio_framework_os_spr::MutationDag::new())),
            edit_sequence: 0,
            clock: fresh_replica_clock(),
            initial_digest,
            revision: std::mem::ManuallyDrop::new(CursorRevisionAccumulator { identity_digest, applied: applied_revision, redo: redo_revision, applied_tail_chains: None, mutation_positions: protocol::HistoryFoldIndex::new(), indexed_edits: protocol::HistoryFoldIndex::new(), unit_flags: protocol::HistoryFoldIndex::new() }),
            close_active: std::mem::ManuallyDrop::new(None),
            supersessions: std::mem::ManuallyDrop::new(EffectiveSupersessions::new()),
            close_phase: 0,
            taken: false,
        }
    }

    pub fn current_mut(&mut self) -> Option<&mut P> {
        self.current.as_mut().and_then(Arc::get_mut)
    }

    pub fn current_ref(&self) -> &P { self.current.as_deref().expect("initialization retains its current projection") }

    pub fn share_current(&self) -> Arc<P> { Arc::clone(self.current.as_ref().expect("initialization retains its current projection")) }

    /// 📏️ Borrows the exact current shared frame or original whole catalog handoff.
    pub fn initialization_retirement_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError>
    where P: Send + Sync + 'static,
    {
        if self.taken { return Ok(Default::default()); }
        if let Some(active) = self.close_active.as_ref() { return artifact_retirement_box_demands(active, body); }
        let capacity_bytes = match self.close_phase {
            0 if self.current.is_some() => ReturnedSnapshotReadRetirement::<P>::constructor_capacity_bytes(),
            1 if self.applied_edit_ids.capacity() != 0 => size_of::<ArtifactStoreStringVectorRetirement>(),
            2 if self.redo_edit_ids.capacity() != 0 => size_of::<ArtifactStoreStringVectorRetirement>(),
            3 if self.cursor_applied_edit_ids.capacity() != 0 => size_of::<ArtifactStoreStringVectorRetirement>(),
            4 if self.cursor_redo_edit_ids.capacity() != 0 => size_of::<ArtifactStoreStringVectorRetirement>(),
            5 if self.current_checkpoint_id.is_some() => size_of::<ArtifactStoreStringRetirement>(),
            6 if self.local_actor_id.0.has_owner() => size_of::<semio_framework_value::retirement::controlled::ControlledRetirement<semio_framework_value::SharedUtf8>>(),
            7 if self.dag.is_some() => size_of::<ArtifactStoreMutationDagRetirement>(),
            8 => size_of::<ArtifactStoreSupersessionRetirement>(),
            9 => size_of::<ArtifactStoreRevisionAccumulatorRetirement>(),
            _ => 0,
        };
        Ok(semio_framework_value::RetirementDemand { capacity_bytes, depth: if capacity_bytes == 0 { 1 } else { 2 }, ..Default::default() })
    }
    pub fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> where P: Send + Sync + 'static { Ok(self.initialization_retirement_demands(0)?.copy_bytes) }
    pub fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> where P: Send + Sync + 'static { Ok(self.initialization_retirement_demands(body)?.capacity_bytes) }
    pub fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> where P: Send + Sync + 'static { Ok(self.initialization_retirement_demands(0)?.release_bytes) }
    pub fn next_close_depth_demand(&self) -> Result<usize, ValueError> where P: Send + Sync + 'static { Ok(self.initialization_retirement_demands(0)?.depth) }

    pub fn replace_current_alias(&mut self, current: Arc<P>) -> Arc<P> { self.current.replace(current).expect("initialization retains its current projection") }

    pub fn adopt_current_owned(&mut self, current: P, factory: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>) -> Result<(), P>
    where P: Send + Sync + 'static,
    {
        if self.close_phase != 0 || self.close_active.is_some() || self.current.is_none() { return Err(current); }
        let previous = self.current.replace(Arc::new(current)).expect("initialization retains its current projection");
        *self.close_active = Some(Box::new(ReturnedSnapshotReadRetirement::new(previous, factory)));
        Ok(())
    }

    pub fn settle_current_retirement_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        artifact_retirement_box_close_step(&mut self.close_active, grant)
    }

    /// ✏️ Installs the effective supersessions the initializer folds the applied edits with; the adopted store keeps
    /// them as the map its projection was made under.
    pub fn set_supersessions(&mut self, supersessions: EffectiveSupersessions) -> Result<(), String> {
        if !self.supersessions.is_empty() {
            return Err("artifact store initialization supersessions are installed exactly once".into());
        }
        *self.supersessions = supersessions;
        Ok(())
    }

    /// ✏️ The effective supersessions the initializer folds the applied edits with.
    pub fn supersessions(&self) -> &EffectiveSupersessions {
        &self.supersessions
    }

    /// 🪞️ Folds transition `transition` of `envelope`'s log — one per job step, in the log's `(hlc, id)` order, a later
    /// supersession of an operation replacing an earlier one — into the effective supersessions: every input of a
    /// `Supersede` scoped to the document or to this replica's line ([`crate::os_spr::fold_history_for`]'s rule). A target no
    /// operation carries matches nothing here; the store's first full fold refuses that history as every fold site does.
    /// Answers whether another transition follows.
    pub fn fold_supersession_step<Mutation>(&mut self, envelope: &ArtifactEnvelopeOwners<P, Mutation>, transition: usize) -> Result<bool, String> {
        let Some(event) = envelope.transitions.get(transition) else { return Ok(false) };
        if let Some(previous) = transition.checked_sub(1).and_then(|previous| envelope.transitions.get(previous)) {
            if (previous.timestamp.cmp_key(), previous.mutation_id.0.as_str()) >= (event.timestamp.cmp_key(), event.mutation_id.0.as_str()) {
                return Err("artifact store history transitions are not in (hlc, id) order".into());
            }
        }
        let Some(semio_framework_os_spr::HistoryTransition::Supersede(supersede)) = semio_framework_os_spr::history_transition_from_envelope(event).map_err(|error| error.to_string())? else {
            return Ok(true);
        };
        let trunk = semio_framework_os_spr::trunk_alternative_id(&ArtifactId(envelope.id.clone()));
        let line = envelope.active_alternative_id.as_deref().filter(|line| *line != trunk.as_str()).unwrap_or(trunk.as_str());
        if supersede.scope.as_deref().is_some_and(|scope| scope != line) {
            return Ok(true);
        }
        for input in supersede.inputs {
            self.supersessions.insert(input.target, protocol::EffectiveSupersession { transition_id: event.mutation_id.0.clone(), actor: event.actor.0.clone(), timestamp: event.timestamp, scope: supersede.scope.clone(), replacement: input.replacement });
        }
        Ok(true)
    }

    /// 🎭️ The effective input of forward `index` of the applied `edit` under the folded supersessions ([`effective_operation`],
    /// the same operation id every seeding step names), for an initializer that applies operations itself: `operation()` is
    /// `None` for a withdrawn or faulted input; `None` when the edit has no forward at `index`.
    pub fn effective_forward<'a, Mutation>(&self, edit: &'a Edit<Mutation>, index: usize, schema: &str) -> Option<EffectiveOperation<'a, P, Mutation>>
    where
        Mutation: self::Mutation<P> + OpBinary,
    {
        let original = edit.forwards.get(index)?;
        let mutation_id = edit.mutation_meta.get(index).and_then(|meta| meta.mutation_id.clone()).or_else(|| original.mutation_id()).unwrap_or_else(|| MutationId(format!("{}#{index}", edit.id)));
        Some(effective_operation::<P, Mutation>(original, index as u32, mutation_id, schema, &self.supersessions))
    }

    /// ▶️ Folds forward `index` of the applied `edit` onto the current projection through its effective input
    /// ([`effective_operation`] under the folded supersessions, keep-and-record [`fold_operation`] — a withdrawn, faulted
    /// or refused operation folds as a no-op exactly as every other fold site folds it). Refuses an operation whose encoding
    /// fails or exceeds `maximum_bytes`.
    pub fn fold_forward<Mutation>(&mut self, edit: &Edit<Mutation>, index: usize, schema: &str, maximum_bytes: usize) -> Result<ArtifactStoreInitializationForward<P>, &'static str>
    where
        Mutation: self::Mutation<P> + OpBinary,
    {
        if let Some(original) = edit.forwards.get(index).filter(|op| op.may_emit_foreign_steps()) {
            let digest = CursorRevisionAccumulator::hash_record(b"edit-id", &[edit.id.as_bytes()]);
            self.revision.unit_flags.insert((digest, index), original.foreign_step_source(self.current.as_deref().ok_or("artifact store initialization lost its current projection")?, 0).map_err(|_| "artifact store original foreign source refused")?.is_some());
        }
        let Some(effective) = self.effective_forward(edit, index, schema) else { return Ok(ArtifactStoreInitializationForward::Exhausted) };
        let Some(operation) = effective.operation() else { return Ok(ArtifactStoreInitializationForward::Folded { displaced: None, fuel: 1 }) };
        let fuel = match operation.encode_op() {
            Ok(encoded) if encoded.len() <= maximum_bytes => encoded.len().max(1),
            _ => return Err("artifact store initialization forward encoding exceeds its page"),
        };
        let current = self.current.as_mut().and_then(Arc::get_mut).ok_or("artifact store initialization requires its bounded owned workspace")?;
        let (next, _) = fold_operation::<P, Mutation>(current, operation, index as u32);
        Ok(ArtifactStoreInitializationForward::Folded { displaced: next.map(|next| std::mem::replace(current, next)), fuel })
    }

    fn push_revision_record(records: &mut semio_framework_os_vcs::HistoryPageStack<CursorRevisionRecord>, identity_digest: [u8; 32], domain: &[u8], id: &str, edit_digest: [u8; 32], ledger_key: ArtifactHistoryKey) -> Result<(), String> {
        if !records.admits_one() {
            return Err("artifact store revision ledger is saturated".into());
        }
        let previous = records.last().map_or(identity_digest, |record| record.prefix_digest);
        let prefix_digest = CursorRevisionAccumulator::hash_record(domain, &[&previous, &edit_digest]);
        records.push(CursorRevisionRecord::new(Some(ledger_key), CursorRevisionAccumulator::hash_record(b"edit-id", &[id.as_bytes()]), edit_digest, prefix_digest));
        Ok(())
    }

    fn push_applied(&mut self, id: String, edit_digest: [u8; 32], ledger_key: ArtifactHistoryKey) -> Result<(), String> {
        if !self.applied_edit_ids.admits_one() || !self.cursor_applied_edit_ids.admits_one() || !self.revision.applied.admits_one() {
            return Err("artifact store applied ledger is saturated".into());
        }
        let identity_digest = self.revision.identity_digest;
        Self::push_revision_record(&mut self.revision.applied, identity_digest, b"applied", &id, edit_digest, ledger_key)?;
        self.cursor_applied_edit_ids.push(id.clone());
        self.applied_edit_ids.push(id);
        Ok(())
    }

    /// 🔏️ Pushes the applied `edit` with the store's canonical revision record — its digest extended by the folded
    /// supersessions of its operations (design §3.7) — so a retained reload names the revision a whole load names.
    pub fn push_applied_edit<Mutation: ToValue + self::Mutation<P>>(&mut self, edit: &Edit<Mutation>, ledger_key: ArtifactHistoryKey) -> Result<(), String> {
        self.push_applied_admitted(edit.id.clone(), edit, ledger_key)
    }

    /// 🔏️ [`Self::push_applied_edit`] under `id`, `edit`'s id the caller already copied under its own grant.
    pub fn push_applied_admitted<Mutation: ToValue + self::Mutation<P>>(&mut self, id: String, edit: &Edit<Mutation>, ledger_key: ArtifactHistoryKey) -> Result<(), String> {
        let digest = CursorRevisionAccumulator::effective_edit_digest::<P, Mutation>(edit, CursorRevisionAccumulator::edit_digest(edit), &self.supersessions);
        self.revision.index_applied_edit::<P, Mutation>(self.applied_edit_ids.len(), edit);
        self.push_applied(id, digest, ledger_key)
    }

    fn push_redo(&mut self, id: String, edit_digest: [u8; 32], ledger_key: ArtifactHistoryKey) -> Result<(), String> {
        if !self.redo_edit_ids.admits_one() || !self.cursor_redo_edit_ids.admits_one() || !self.revision.redo.admits_one() {
            return Err("artifact store redo ledger is saturated".into());
        }
        let identity_digest = self.revision.identity_digest;
        Self::push_revision_record(&mut self.revision.redo, identity_digest, b"redo", &id, edit_digest, ledger_key)?;
        self.cursor_redo_edit_ids.push(id.clone());
        self.redo_edit_ids.push(id);
        Ok(())
    }

    /// 🔏️ Pushes the redo `edit` with the store's canonical revision record ([`Self::push_applied_edit`]).
    pub fn push_redo_edit<Mutation: ToValue + self::Mutation<P>>(&mut self, edit: &Edit<Mutation>, ledger_key: ArtifactHistoryKey) -> Result<(), String> {
        self.push_redo_admitted(edit.id.clone(), edit, ledger_key)
    }

    /// 🔏️ [`Self::push_redo_edit`] under `id`, `edit`'s id the caller already copied under its own grant.
    pub fn push_redo_admitted<Mutation: ToValue + self::Mutation<P>>(&mut self, id: String, edit: &Edit<Mutation>, ledger_key: ArtifactHistoryKey) -> Result<(), String> {
        let digest = CursorRevisionAccumulator::effective_edit_digest::<P, Mutation>(edit, CursorRevisionAccumulator::edit_digest(edit), &self.supersessions);
        self.push_redo(id, digest, ledger_key)
    }

    /// 🌱️ Seeds operation `id` of history entry `entry_id` into the causal owner. An edit a replica folded from a peer is
    /// named after its only operation (`edit_from_operation_envelope`), so an operation id equal to its own entry id is the node
    /// the entry already seeded — one node, never two; a repeat across entries still refuses as a duplicate.
    pub fn seed_edit_operation(&mut self, entry_id: &str, id: MutationId) -> Result<(), String> {
        if id.0 == entry_id {
            return Ok(());
        }
        self.seed_mutation(id)
    }

    pub fn seed_mutation(&mut self, id: MutationId) -> Result<(), String> {
        self.dag.as_mut().ok_or_else(|| "artifact store initialization causal owner was already detached".to_string())?.seed_applied(id).map_err(|error| error.to_string())
    }

    pub fn observe_sequence(&mut self, sequence: i32) {
        self.edit_sequence = self.edit_sequence.max(sequence);
    }

    pub fn observe_timestamp(&mut self, timestamp: HybridLogicalTimestamp) {
        self.clock.merge(&timestamp);
    }

    pub fn set_current_checkpoint_id(&mut self, value: Option<String>) {
        *self.current_checkpoint_id = value;
    }

    /// 🧹️ Admits original whole storage before transfer and retains refused values in their original fields.
    pub fn close_step(&mut self, current_factory: &Arc<dyn ArtifactOwnedValueRetirementFactory<P>>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>
    where P: Send + Sync + 'static,
    {
        let empty = RetainedCloneProgress::default();
        if self.taken { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.initialization_retirement_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "initialization retirement exceeds admitted depth")); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        if self.close_active.is_some() { return self.settle_current_retirement_step(grant); }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        match self.close_phase {
            0 => {
                if let Some(original) = self.current.take() {
                    match ReturnedSnapshotReadRetirement::admit(original, Arc::clone(current_factory), child) {
                        Ok((owner, receipt)) => { *self.close_active = Some(owner); self.close_phase = 1; if !receipt.fits(child) || receipt.retained_capacity_bytes != demand.capacity_bytes { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "initialization shared constructor changed its admitted receipt")); } return Ok(RetainedCloneStep::Progress(receipt)); }
                        Err((error, original, alias)) => { *self.current = Some(original); drop(alias); return Err(error); }
                    }
                }
            }
            1 => {
                if self.applied_edit_ids.capacity() != 0 {
                    let original = std::mem::replace(&mut *self.applied_edit_ids, semio_framework_os_vcs::HistoryPageStack::empty());
                    match admit_artifact_retirement(original, child, ArtifactStoreStringVectorRetirement::new) {
                        Ok((owner, receipt)) => { *self.close_active = Some(owner); self.close_phase = 2; return Ok(RetainedCloneStep::Progress(receipt)); }
                        Err((error, original)) => { *self.applied_edit_ids = original; return Err(error); }
                    }
                }
            }
            2 => {
                if self.redo_edit_ids.capacity() != 0 {
                    let original = std::mem::replace(&mut *self.redo_edit_ids, semio_framework_os_vcs::HistoryPageStack::empty());
                    match admit_artifact_retirement(original, child, ArtifactStoreStringVectorRetirement::new) {
                        Ok((owner, receipt)) => { *self.close_active = Some(owner); self.close_phase = 3; return Ok(RetainedCloneStep::Progress(receipt)); }
                        Err((error, original)) => { *self.redo_edit_ids = original; return Err(error); }
                    }
                }
            }
            3 => {
                if self.cursor_applied_edit_ids.capacity() != 0 {
                    let original = std::mem::replace(&mut *self.cursor_applied_edit_ids, semio_framework_os_vcs::HistoryPageStack::empty());
                    match admit_artifact_retirement(original, child, ArtifactStoreStringVectorRetirement::new) {
                        Ok((owner, receipt)) => { *self.close_active = Some(owner); self.close_phase = 4; return Ok(RetainedCloneStep::Progress(receipt)); }
                        Err((error, original)) => { *self.cursor_applied_edit_ids = original; return Err(error); }
                    }
                }
            }
            4 => {
                if self.cursor_redo_edit_ids.capacity() != 0 {
                    let original = std::mem::replace(&mut *self.cursor_redo_edit_ids, semio_framework_os_vcs::HistoryPageStack::empty());
                    match admit_artifact_retirement(original, child, ArtifactStoreStringVectorRetirement::new) {
                        Ok((owner, receipt)) => { *self.close_active = Some(owner); self.close_phase = 5; return Ok(RetainedCloneStep::Progress(receipt)); }
                        Err((error, original)) => { *self.cursor_redo_edit_ids = original; return Err(error); }
                    }
                }
            }
            5 => {
                if let Some(original) = self.current_checkpoint_id.take() {
                    match admit_artifact_retirement(original, child, ArtifactStoreStringRetirement::new) {
                        Ok((owner, receipt)) => { *self.close_active = Some(owner); self.close_phase = 6; return Ok(RetainedCloneStep::Progress(receipt)); }
                        Err((error, original)) => { *self.current_checkpoint_id = Some(original); return Err(error); }
                    }
                }
            }
            6 => {
                if self.local_actor_id.0.has_owner() {
                    let original = std::mem::take(&mut self.local_actor_id.0);
                    match admit_artifact_retirement(original, child, |original|semio_framework_value::retirement::controlled::ControlledRetirement::new(original).unwrap_or_else(|_|unreachable!("shared UTF8 declares controlled retirement"))) {
                        Ok((owner, receipt)) => { *self.close_active = Some(owner); self.close_phase = 7; return Ok(RetainedCloneStep::Progress(receipt)); }
                        Err((error, original)) => { self.local_actor_id.0 = original; return Err(error); }
                    }
                }
            }
            7 => {
                if let Some(original) = self.dag.take() {
                    match admit_artifact_retirement(original, child, ArtifactStoreMutationDagRetirement::new) {
                        Ok((owner, receipt)) => { *self.close_active = Some(owner); self.close_phase = 8; return Ok(RetainedCloneStep::Progress(receipt)); }
                        Err((error, original)) => { *self.dag = Some(original); return Err(error); }
                    }
                }
            }
            8 => {
                let original = std::mem::take(&mut *self.supersessions);
                match admit_artifact_retirement(original, child, ArtifactStoreSupersessionRetirement::new) {
                    Ok((owner, receipt)) => { *self.close_active = Some(owner); self.close_phase = 9; return Ok(RetainedCloneStep::Progress(receipt)); }
                    Err((error, original)) => { *self.supersessions = original; return Err(error); }
                }
            }
            9 => {
                let vacant = CursorRevisionAccumulator { identity_digest: [0; 32], applied: semio_framework_os_vcs::HistoryPageStack::empty(), redo: semio_framework_os_vcs::HistoryPageStack::empty(), applied_tail_chains: None, mutation_positions: protocol::HistoryFoldIndex::new(), indexed_edits: protocol::HistoryFoldIndex::new(), unit_flags: protocol::HistoryFoldIndex::new() };
                let original = std::mem::replace(&mut *self.revision, vacant);
                match admit_artifact_retirement(original, child, ArtifactStoreRevisionAccumulatorRetirement::new) {
                    Ok((owner, receipt)) => { *self.close_active = Some(owner); self.close_phase = 10; return Ok(RetainedCloneStep::Progress(receipt)); }
                    Err((error, original)) => { *self.revision = original; return Err(error); }
                }
            }
            _ => { self.taken = true; return Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..empty })); }
        }
        self.close_phase += 1;
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }))
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.taken && self.current.is_none() && self.dag.is_none() && self.close_active.is_none()
    }

    #[allow(clippy::type_complexity)]
    pub fn into_parts(
        mut self,
    ) -> (Arc<P>, semio_framework_os_vcs::HistoryPageStack<String>, semio_framework_os_vcs::HistoryPageStack<String>, ArtifactCursor, ActorId, semio_framework_os_spr::MutationDag, i32, HybridLogicalTimestamp, [u8; 32], CursorRevisionAccumulator, EffectiveSupersessions) {
        assert!(self.close_phase == 0 && self.close_active.is_none(), "a closing initialization runtime cannot be adopted");
        let current = self.current.take().expect("validated initialization owns its current snapshot");
        let applied = unsafe { std::mem::ManuallyDrop::take(&mut self.applied_edit_ids) };
        let redo = unsafe { std::mem::ManuallyDrop::take(&mut self.redo_edit_ids) };
        let checkpoint = self.current_checkpoint_id.take();
        let cursor = ArtifactCursor::new(unsafe { std::mem::ManuallyDrop::take(&mut self.cursor_applied_edit_ids) }, unsafe { std::mem::ManuallyDrop::take(&mut self.cursor_redo_edit_ids) }, checkpoint);
        let actor = ActorId(std::mem::take(&mut self.local_actor_id.0));
        let dag = self.dag.take().expect("validated initialization owns its causal graph");
        let revision = unsafe { std::mem::ManuallyDrop::take(&mut self.revision) };
        let supersessions = unsafe { std::mem::ManuallyDrop::take(&mut self.supersessions) };
        self.taken = true;
        (current, applied, redo, cursor, actor, dag, self.edit_sequence, self.clock, self.initial_digest, revision, supersessions)
    }
}

impl<P> Drop for ArtifactStoreInitializationRuntime<P> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.taken), "artifact store initialization runtime reached Drop before exact store construction or retained rejection cleanup");
    }
}

/// ✂️ Bounds `messages`, one edit's message ledger entry, to [`ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES`] deterministically, so
/// no replay yields an unstorable entry and every replica stores the same one: messages are kept worst level first (ties
/// in recorded order) while they fit and stay in recorded order, and the dropped rest is summarized by one
/// `mutation.cascade` info at the operation of the first dropped message. A worst message too large on its own keeps
/// its code with its text cut to fit. Answers whether anything was cut.
pub fn bound_edit_messages(edit_id: &str, messages: &mut Vec<semio_framework_os_spr::MutationMessage>) -> bool {
    let size = |message: &semio_framework_os_spr::MutationMessage| message.target.iter().fold(ARTIFACT_EDIT_MESSAGE_OWNER_BYTES.saturating_add(message.code.0.len()).saturating_add(message.message.len()), |bytes, segment| bytes.saturating_add(ARTIFACT_EDIT_MESSAGE_TARGET_BYTES).saturating_add(segment.len()));
    if messages.iter().fold(edit_id.len(), |bytes, message| bytes.saturating_add(size(message))) <= ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES {
        return false;
    }
    let summary = |dropped: usize, op_index: Option<u32>| {
        let mut summary = semio_framework_os_spr::MutationMessage::info("mutation.cascade", format!("{dropped} more messages"));
        summary.op_index = op_index;
        summary
    };
    let budget = ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES.saturating_sub(edit_id.len()).saturating_sub(size(&summary(messages.len(), None)));
    let mut ranked: Vec<usize> = (0..messages.len()).collect();
    ranked.sort_by_key(|index| std::cmp::Reverse(messages[*index].level));
    let mut kept = vec![false; messages.len()];
    let mut used = 0usize;
    for index in ranked.iter().copied() {
        let bytes = size(&messages[index]);
        if used.saturating_add(bytes) > budget {
            break;
        }
        used += bytes;
        kept[index] = true;
    }
    if let Some(worst) = ranked.first().copied().filter(|_| !kept.iter().any(|kept| *kept)) {
        let message = &mut messages[worst];
        message.target.clear();
        let mut cut = budget.saturating_sub(ARTIFACT_EDIT_MESSAGE_OWNER_BYTES).saturating_sub(message.code.0.len()).min(message.message.len());
        while !message.message.is_char_boundary(cut) {
            cut -= 1;
        }
        message.message.truncate(cut);
        kept[worst] = true;
    }
    let first_dropped = kept.iter().position(|kept| !*kept).map(|index| messages[index].op_index);
    let dropped = kept.iter().filter(|kept| !**kept).count();
    let mut flags = kept.into_iter();
    messages.retain(|_| flags.next().unwrap_or(false));
    if let Some(op_index) = first_dropped {
        messages.push(summary(dropped, op_index));
    }
    true
}
//#endregion 🔖️ArtifactStore
//#endregion 🔖️ArtifactStore
//#endregion 🔖️History
//#endregion 🔖️TextFormat
//#endregion 🔖️CodecRegistry
//#endregion 🔖️Pack
//#endregion 🔖️Text
//#endregion 🔖️EphemeralLanes
//#endregion 🔖️Authority

//#endregion 🔖️EphemeralLanes
