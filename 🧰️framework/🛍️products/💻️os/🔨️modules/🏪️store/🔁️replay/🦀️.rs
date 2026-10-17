//! 🔁️ Replay of the edit history: effective forwards and supersessions, the bounded edit replay with its prefix snapshot
//! ring, and pair merges. The engine, history reads and time travel replay through it; it never names the engine.
//!
//! Layering: `📓️2026-10-13-wave-e1-os-kernel.md` (ticket BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT).
#[path = "📥️lifted/🏪️store/🦀️.rs"]
pub mod lifted_store;
#[path = "🎮️operation/🦀️.rs"]
pub mod replay_preparation;
#[path = "🛑️cancel/🦀️.rs"]
pub mod raw_replay_retirement;
#[path = "📥️lifted/🔗️read/🦀️.rs"]
pub mod lifted_snapshot_read;
#[cfg(any(test, feature = "hot-path-census"))] use semio_framework_os_store_ledger::hot_path_census;

use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementFactory};
use semio_framework_value::retained_clone::RetainedCloneGrant;
use crate::replay_preparation::{ArtifactReplayPreparation, ArtifactReplayPreparationFactory, ArtifactReplayPreparationProgress, ArtifactReplayPreparationRetirement, ArtifactReplayPreparationTask};
use semio_framework_os_spr::command::ArtifactReplayPrepared;
use semio_framework_os_spr::command::ArtifactMessageSource;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_os_spr::{HybridLogicalTimestamp, MutationId};
use semio_framework_os_spr::{DiffRegions, Edit, Mutation, MutationDiff, OpBinary, OpText};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::marker::PhantomData;
use std::sync::Arc;
use semio_framework_os_spr::command::MutationMessageLedgerRetirement as ArtifactStoreMessageLedgerRetirement;
use semio_framework_os_vcs::VcsError;
use semio_framework_os_vcs::{ArtifactHistoryLedger, Identified, Patchable};
use semio_framework_os_pack::pack_rt;
use semio_framework_os_store_message_ledger::ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES;
use semio_framework_os_store_ledger::CursorRevisionAccumulator;
use semio_framework_os_store_projection::retire_replayed_projection;
use semio_framework_os_store_command::retire_scratch_operations;
use semio_framework_pack_json::ArtifactCanonicalJsonTree;

/// 🫱️ What merging a persisted pair into a live store found ([`ArtifactStore::merge_persisted_history`]): `merged` events the
/// store took from the pair, and `ahead` events the store holds that the pair lacks — what a writer persists again for.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PairMerge {
    pub merged: usize,
    pub ahead: usize,
}



/// 🎭️ One forward operation as every fold site reads it: the recorded original, its decoded replacement, or nothing
/// when withdrawn or when its supersession breaks the supersede law ([`admit_replacement`]). A decoded replacement
/// retires itself through [`Mutation::retire_cold`].
pub struct EffectiveOperation<'a, P, Mutation>
where
    Mutation: self::Mutation<P>,
{
    pub op_index: u32,
    pub mutation_id: MutationId,
    pub original: &'a Mutation,
    pub replacement: Option<Mutation>,
    pub withdrawn: bool,
    pub fault: Option<semio_framework_os_spr::MutationMessage>,
    pub marker: PhantomData<fn() -> P>,
}

impl<P, Mutation> EffectiveOperation<'_, P, Mutation>
where
    Mutation: self::Mutation<P>,
{
    /// ▶️ The operation a fold applies, `None` when withdrawn or faulted.
    pub fn operation(&self) -> Option<&Mutation> {
        if self.withdrawn || self.fault.is_some() {
            return None;
        }
        Some(self.replacement.as_ref().unwrap_or(self.original))
    }

    /// 🔀️ Whether a supersession replaced or withdrew the recorded forward (validly or not).
    pub fn superseded(&self) -> bool {
        self.withdrawn || self.replacement.is_some() || self.fault.is_some()
    }

    /// 🚫️ Whether a valid supersession withdrew the operation.
    pub fn withdrawn(&self) -> bool {
        self.withdrawn
    }

    /// 🧯️ The `Fatal` message of a supersession breaking the supersede law; the operation then folds as a no-op.
    pub fn fault(&self) -> Option<&semio_framework_os_spr::MutationMessage> {
        self.fault.as_ref()
    }
}

impl<P, Mutation> Drop for EffectiveOperation<'_, P, Mutation>
where
    Mutation: self::Mutation<P>,
{
    fn drop(&mut self) {
        if let Some(replacement) = self.replacement.take() {
            self::Mutation::<P>::retire_cold(replacement);
        }
    }
}

/// 🔍️ The effective input of `operation` — operation `op_index` of its edit, identified as `mutation_id` — under
/// `supersessions` in a document of `schema`. Total: a supersession breaking the supersede law ([`admit_replacement`])
/// folds the operation as a no-op carrying one `Fatal` `mutation.invariant` message that targets the operation —
/// identically at every fold site, so a garbage or foreign replacement never aborts a projection.
pub fn effective_operation<'a, P, Mutation>(operation: &'a Mutation, op_index: u32, mutation_id: MutationId, schema: &str, supersessions: &EffectiveSupersessions) -> EffectiveOperation<'a, P, Mutation>
where
    Mutation: self::Mutation<P> + OpBinary,
{
    let supersession = supersessions.get(&mutation_id);
    effective_operation_with_supersession::<P, Mutation>(operation, op_index, mutation_id, schema, supersession)
}

pub fn effective_operation_with_supersession<'a, P, Mutation>(operation: &'a Mutation, op_index: u32, mutation_id: MutationId, schema: &str, supersession: Option<&protocol::EffectiveSupersession>) -> EffectiveOperation<'a, P, Mutation>
where
    Mutation: self::Mutation<P> + OpBinary,
{
    let mut effective = EffectiveOperation { op_index, mutation_id, original: operation, replacement: None, withdrawn: false, fault: None, marker: PhantomData };
    let Some(supersession) = supersession else { return effective };
    match admit_replacement::<P, Mutation>(operation, &supersession.replacement, schema) {
        Ok(Some(replacement)) => effective.replacement = Some(replacement),
        Ok(None) => effective.withdrawn = true,
        Err(reason) => {
            let mut fault =
                semio_framework_os_spr::MutationMessage::fatal("mutation.invariant", format!("supersession {} of {} {reason}; the operation folds as a no-op", supersession.transition_id, effective.mutation_id.0)).at([effective.mutation_id.0.clone()]);
            fault.op_index = Some(op_index);
            effective.fault = Some(fault);
        }
    }
    effective
}

/// 🛂️ The supersede law every input obeys — authored, ingested, loaded or folded. A withdrawal is admitted for every
/// operation. A replacement is an operation of `schema` in its canonical encoding that plans no foreign steps; an operation
/// that plans foreign steps takes no replacement but its own recorded input — what the undo of its withdrawal authors —
/// because any other input would plan steps the other documents never received. Answers the decoded replacement (`None` for
/// a withdrawal) or why the input breaks the law.
pub fn admit_replacement<P, Mutation>(original: &Mutation, replacement: &protocol::InputReplacement, schema: &str) -> Result<Option<Mutation>, String>
where
    Mutation: self::Mutation<P> + OpBinary,
{
    let protocol::InputReplacement::Input { schema: named, payload } = replacement else { return Ok(None) };
    if named != schema {
        return Err(format!("names schema {named}, not {schema}"));
    }
    let decoded = Mutation::decode_op(payload).map_err(|error| format!("does not decode: {error}"))?;
    let refusal = match decoded.encode_op() {
        Err(error) => Some(format!("does not re-encode: {error}")),
        Ok(recoded) if recoded != *payload => Some("is not canonically encoded".to_string()),
        Ok(recoded) if original.may_emit_foreign_steps() => (!original.encode_op().is_ok_and(|recorded| recorded == recoded)).then(|| "replaces the input of an operation that plans foreign steps".to_string()),
        Ok(_) if decoded.may_emit_foreign_steps() => Some("plans foreign steps".to_string()),
        Ok(_) => None,
    };
    match refusal {
        None => Ok(Some(decoded)),
        Some(reason) => {
            self::Mutation::<P>::retire_cold(decoded);
            Err(reason)
        }
    }
}

/// 📜️ `edit`'s effective forwards in operation order ([`effective_operation`]). The single accessor every fold site
/// reads an edit through.
pub fn effective_forwards<'a, P, Mutation>(edit: &'a Edit<Mutation>, schema: &'a str, supersessions: &'a EffectiveSupersessions) -> impl Iterator<Item = EffectiveOperation<'a, P, Mutation>> + 'a
where
    P: 'a,
    Mutation: self::Mutation<P> + OpBinary + 'a,
{
    let ids = semio_framework_os_spr::mutation_ids_for_edit::<P, Mutation>(edit);
    edit.forwards.iter().zip(ids).enumerate().map(move |(index, (operation, mutation_id))| effective_operation::<P, Mutation>(operation, index as u32, mutation_id, schema, supersessions))
}



/// 🔂️ Folds the first `take` effective forwards of `edit` into `projection` with keep-and-record semantics
/// ([`fold_operation`]) — the one loop every messageless fold site shares.
pub fn fold_effective_edit<P, Mutation>(projection: &mut impl FoldProjection<P, Mutation>, edit: &Edit<Mutation>, schema: &str, supersessions: &EffectiveSupersessions, take: usize)
where
    Mutation: self::Mutation<P> + OpBinary,
{
    if supersessions.is_empty() {
        for (index, operation) in edit.forwards.iter().enumerate().take(take) {
            if let (Some(next), _) = fold_operation::<P, Mutation>(projection.projection(), operation, index as u32) {
                projection.advance(next);
            }
        }
        return;
    }
    for effective in effective_forwards::<P, Mutation>(edit, schema, supersessions).take(take) {
        if let Some(operation) = effective.operation() {
            if let (Some(next), _) = fold_operation::<P, Mutation>(projection.projection(), operation, effective.op_index) {
                projection.advance(next);
            }
        }
    }
}

pub use protocol::apply_operation;

/// 🛡️ Applies an already computed outcome through [`protocol::apply_diff`]: the next projection with the outcome intact, or —
/// on an apply refusal — the unchanged `base`, an empty diff and one `Fatal` message. The builder-side twin of the fatal
/// message production dispatch records.
pub fn apply_outcome<P, D>(base: &P, outcome: semio_framework_os_spr::MutationOutcome<D>) -> (P, semio_framework_os_spr::MutationOutcome<D>)
where
    P: Clone,
    D: MutationDiff<P>,
{
    let (diff, mut messages) = outcome.into_parts();
    match semio_framework_os_spr::apply_diff(&diff, base) {
        Ok(next) => (next, semio_framework_os_spr::MutationOutcome::new(diff).absorb_messages(messages)),
        Err(error) => {
            messages.push(semio_framework_os_spr::MutationMessage::fatal(error.code, error.message).at(error.target));
            D::retire_cold(diff);
            (base.clone(), semio_framework_os_spr::MutationOutcome::new(D::default()).absorb_messages(messages))
        }
    }
}

/// 🎬️ One operation that never drops a projection: the next projection when its diff applied, and its op-stamped
/// messages — an apply refusal becomes one `Fatal` message and folds as a no-op. The keep-and-record step every fold
/// site, the retained initializers included, takes.
pub fn fold_operation<P, Mutation>(state: &P, operation: &Mutation, op_index: u32) -> (Option<P>, Vec<semio_framework_os_spr::MutationMessage>)
where
    Mutation: self::Mutation<P>,
{
    let (applied, mut messages) = apply_operation::<P, Mutation>(state, operation, op_index);
    match applied {
        Ok(next) => (Some(next), messages),
        Err(error) => {
            let mut fatal = semio_framework_os_spr::MutationMessage::fatal(error.code, error.message).at(error.target);
            fatal.op_index = Some(op_index);
            messages.push(fatal);
            (None, messages)
        }
    }
}

/// 📼️ Folds `applied[from..to]` onto `base` under `supersessions` with keep-and-record semantics, handing out the
/// projection after every prefix length below `to` that is a multiple of `stride` (aliases of the running state, no
/// copies). A failure retires the running state and everything recorded.
pub fn fold_recorded<P, Mutation>(
    base: Arc<P>,
    applied: &[String],
    from: usize,
    to: usize,
    edits: &ArtifactHistoryLedger<Edit<Mutation>>,
    schema: &str,
    supersessions: &EffectiveSupersessions,
    stride: usize,
) -> Result<(Arc<P>, Vec<(usize, Arc<P>)>), VcsError>
where
    Mutation: self::Mutation<P> + OpBinary,
{
    let mut running = base;
    let mut recorded: Vec<(usize, Arc<P>)> = Vec::new();
    let mut failure = None;
    for (offset, edit_id) in applied[from..to].iter().enumerate() {
        let Some(edit) = edits.find_near(|edit| edit.id == *edit_id) else {
            failure = Some(VcsError::UnknownEdit(edit_id.clone()));
            break;
        };
        fold_effective_edit(&mut running, edit, schema, supersessions, usize::MAX);
        let length = from + offset + 1;
        if length < to && length % stride == 0 {
            recorded.push((length, Arc::clone(&running)));
        }
    }
    if let Some(error) = failure {
        retire_shared_projection::<P, Mutation>(running);
        for (_, snapshot) in recorded {
            retire_shared_projection::<P, Mutation>(snapshot);
        }
        return Err(error);
    }
    Ok((running, recorded))
}

pub use semio_framework_os_store_projection::retire_shared_projection;





/// 🎏️ Lists `ledger`'s facts in the order `order` names them — the fold's order, a pure function of the log, whatever order
/// their events reached this replica in — by exchanging entries in place: slots, keys and reservations stay where they are.
/// One comparison pass when the order already holds.
pub fn order_ledger_as<'order, T>(ledger: &mut ArtifactHistoryLedger<T>, id: impl Fn(&T) -> &str, order: impl Iterator<Item = &'order str> + Clone) {
    if ledger.iter().map(&id).eq(order.clone()) {
        return;
    }
    let mut entries: Vec<&mut T> = ledger.iter_mut().collect();
    for (position, expected) in order.enumerate() {
        if let Some(found) = (position..entries.len()).find(|candidate| id(&*entries[*candidate]) == expected).filter(|found| *found != position) {
            let (head, tail) = entries.split_at_mut(found);
            std::mem::swap(&mut *head[position], &mut *tail[0]);
        }
    }
}

/// 🤝️ How many leading ids `live` and `next` share.
pub fn shared_prefix_len(live: &semio_framework_os_vcs::HistoryPageStack<String>, next: &[String]) -> usize {
    live.iter().zip(next).take_while(|(live, next)| live == next).count()
}

/// 🪞️ Rewrites `mirror` from position `dirty` on to `live`'s suffix ([`StackMirrorDirt`]); answers the ids it displaced.
pub fn mirror_id_suffix(mirror: &mut semio_framework_os_vcs::HistoryPageStack<String>, live: &semio_framework_os_vcs::HistoryPageStack<String>, dirty: usize) -> Option<Vec<String>> {
    let keep = dirty.min(live.len()).min(mirror.len());
    let mut displaced = Vec::new();
    while mirror.len() > keep {
        displaced.push(mirror.pop().expect("a mirrored id remains present"));
    }
    #[cfg(any(test, feature = "hot-path-census"))]
    hot_path_census::count(|census| census.mirrored += live.len() - keep);
    mirror.extend(live.iter_from(keep).cloned());
    (!displaced.is_empty()).then_some(displaced)
}

/// 🧭️ Forwards-only effective prefix digests of `applied`: entry `i` names the projection after the first `i` edits by
/// their ids, forward counts and effective supersessions — never the inverses, metadata or messages a replay rewrites.
pub fn forward_prefix_digests<P, Mutation, I, S>(identity: [u8; 32], applied: I, edits: &ArtifactHistoryLedger<Edit<Mutation>>, supersessions: &EffectiveSupersessions) -> Result<Vec<[u8; 32]>, VcsError>
where
    Mutation: self::Mutation<P>,
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut digest = forward_prefix_genesis(identity);
    let mut digests = Vec::new();
    digests.push(digest);
    for edit_id in applied {
        let edit_id = edit_id.as_ref();
        let edit = edits.find_near(|edit| edit.id == edit_id).ok_or_else(|| VcsError::UnknownEdit(edit_id.to_string()))?;
        digest = forward_prefix_step::<P, Mutation>(&digest, edit, supersessions);
        digests.push(digest);
    }
    Ok(digests)
}

/// 🧭️ The forwards-only digest of the empty prefix of a document of `identity` ([`forward_prefix_digests`]).
pub fn forward_prefix_genesis(identity: [u8; 32]) -> [u8; 32] {
    CursorRevisionAccumulator::hash_record(b"forward-prefix", &[&identity])
}

/// 🧭️ The forwards-only prefix digest after `edit`, given the digest of the prefix before it ([`forward_prefix_digests`]).
pub fn forward_prefix_step<P, Mutation>(previous: &[u8; 32], edit: &Edit<Mutation>, supersessions: &EffectiveSupersessions) -> [u8; 32]
where
    Mutation: self::Mutation<P>,
{
    #[cfg(any(test, feature = "hot-path-census"))]
    hot_path_census::count(|census| census.digested += 1);
    let superseded = edit_supersession_digest::<P, Mutation>(edit, supersessions).unwrap_or([0; 32]);
    CursorRevisionAccumulator::hash_record(b"forward-prefix", &[previous, edit.id.as_bytes(), &(edit.forwards.len() as u64).to_be_bytes(), &superseded])
}

/// 🧮️ The applied positions of every edit owning an operation whose effective supersession differs between `previous`
/// and `next`, ascending.
pub fn superseded_positions<P, Mutation>(applied: &[String], edits: &ArtifactHistoryLedger<Edit<Mutation>>, previous: &EffectiveSupersessions, next: &EffectiveSupersessions) -> Result<Vec<usize>, VcsError>
where
    Mutation: self::Mutation<P>,
{
    let changed: HashSet<&MutationId> = previous
        .iter()
        .filter(|(target, supersession)| next.get(*target) != Some(*supersession))
        .map(|(target, _)| target)
        .chain(next.iter().filter(|(target, supersession)| previous.get(*target) != Some(*supersession)).map(|(target, _)| target))
        .collect();
    if changed.is_empty() {
        return Ok(Vec::new());
    }
    let mut positions = Vec::new();
    for (position, edit_id) in applied.iter().enumerate() {
        let edit = edits.find_near(|edit| edit.id == *edit_id).ok_or_else(|| VcsError::UnknownEdit(edit_id.clone()))?;
        if semio_framework_os_spr::mutation_ids_for_edit::<P, Mutation>(edit).iter().any(|mutation_id| changed.contains(mutation_id)) {
            positions.push(position);
        }
    }
    Ok(positions)
}

/// 📝️ `supersessions` with every draft replacement laid over it — the effective inputs a time-travel session previews.
pub fn draft_supersessions(supersessions: &EffectiveSupersessions, drafts: &protocol::HistoryInputDrafts) -> EffectiveSupersessions {
    let mut effective = supersessions.clone();
    for (target, replacement) in drafts {
        effective.insert(target.clone(), protocol::EffectiveSupersession { transition_id: DRAFT_SUPERSESSION_ID.to_string(), actor: semio_framework_value::SharedUtf8::default(), timestamp: HybridLogicalTimestamp::new(0, 0), scope: None, replacement: replacement.clone() });
    }
    effective
}

/// 🏷️ The transition id a not-yet-authored draft supersession carries inside a preview.
pub const DRAFT_SUPERSESSION_ID: &str = "draft";

/// ⚖️ How an [`EditReplay`] settles each edit. `Merge(policy)` is the policy-local ingest verdict: an edit whose worst
/// message the policy rejects is quarantined and an apply refusal aborts the replay. `Report` keeps every edit, records
/// every outcome and never aborts, so its result is policy-independent and identical on every replica.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReplayMode {
    Merge(semio_framework_os_spr::MergePolicy),
    Report,
}

/// 📚️ Where a replay reads the edits its order names.
pub trait ReplayEdits<Mutation> {
    fn replay_edit(&self, edit_id: &str) -> Option<&Edit<Mutation>>;
}

pub struct IndexedAppliedReplayEdits<'a, Mutation> {
    pub edits: &'a ArtifactHistoryLedger<Edit<Mutation>>,
    pub revision: &'a CursorRevisionAccumulator,
    pub ids: &'a semio_framework_os_vcs::HistoryPageStack<String>,
}

impl<Mutation> ReplayEdits<Mutation> for IndexedAppliedReplayEdits<'_, Mutation> {
    fn replay_edit(&self, edit_id: &str) -> Option<&Edit<Mutation>> {
        let digest = CursorRevisionAccumulator::hash_record(b"edit-id", &[edit_id.as_bytes()]);
        let position = self.revision.indexed_edits.get(&digest)?.0;
        if self.ids.get(position).is_none_or(|id| id != edit_id) { return None; }
        let key = self.revision.applied.get(position)?.ledger_key?;
        self.edits.get_key(key).filter(|edit| edit.id == edit_id)
    }
}

/// 📶️ Completed replay work: folded operations, planned source rows and sealed suffix edits.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ReplayProgress {
    pub done: u32,
    pub total: u32,
}

/// ⏯️ Where one [`EditReplay::step`] left its replay.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReplayStep {
    Pending(ReplayProgress),
    Finished(ReplayProgress),
}

/// 🧲️ The previous projection a Report replay may converge onto. From prefix length `horizon` on every changed input
/// has been replayed, so a prefix whose state equals the previous state after the same prefix leaves the whole tail
/// unchanged: the previous head is spliced in and the replay stops. `checkpoints` ascend by prefix length and end with
/// the previous head.
pub struct ReplayConvergence<P> {
    pub horizon: usize,
    pub checkpoints: Vec<(usize, Arc<P>)>,
    pub equal: fn(&P, &P) -> bool,
}

/// ⏪️ Resumable replay of `order[from..]` over the projection after `order[..from]`, one operation per inner step:
/// each operation's effective input is re-diffed and re-inverted against the running state and its messages are kept
/// per mutation with their operation index. [`ArtifactStore::replay_suffix_partitioned`] drives a `Merge` replay to
/// completion; history editing steps a `Report` replay under a deadline with progress. Cancellation retains
/// running projections, staged inverses and checkpoint aliases until their independently admitted retirement.
/// 🧳️ Retains the actual native replay state until every original owner has an admitted close.
pub struct EditReplay<P, Mutation: self::Mutation<P>> {
    pub original: std::mem::ManuallyDrop<ReplayOwnedState<P, Mutation>>,
    pub(crate) raw_retirement: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    pub(crate) raw_factory: std::mem::ManuallyDrop<Option<semio_framework_value::FactoryAuthority>>,
    pub retiring: bool,
}

impl<P, Mutation: self::Mutation<P>> std::ops::Deref for EditReplay<P, Mutation> {
    type Target = ReplayOwnedState<P, Mutation>;
    fn deref(&self) -> &Self::Target { &self.original }
}

impl<P, Mutation: self::Mutation<P>> std::ops::DerefMut for EditReplay<P, Mutation> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.original }
}

impl<P, Mutation: self::Mutation<P> + OpBinary> EditReplay<P, Mutation> {
    pub fn retain(original: ReplayOwnedState<P, Mutation>) -> Self { Self { original: std::mem::ManuallyDrop::new(original), raw_retirement: std::mem::ManuallyDrop::new(None), raw_factory: std::mem::ManuallyDrop::new(None), retiring: false } }
    pub fn new(mode: ReplayMode, base: Arc<P>, order: Vec<String>, from: usize, schema: &str, supersessions: EffectiveSupersessions, edits: &impl ReplayEdits<Mutation>) -> Result<Self, VcsError> { ReplayOwnedState::new(mode, base, order, from, schema, supersessions, edits).map(Self::retain) }
    pub fn new_counted(mode: ReplayMode, base: Arc<P>, order: semio_framework_os_vcs::HistoryPageStack<String>, from: usize, schema: &str, supersessions: EffectiveSupersessions, total: u32) -> Result<Self, VcsError> { ReplayOwnedState::new_counted(mode, base, order, from, schema, supersessions, total).map(Self::retain) }
    pub fn new_after_prefix(mode: ReplayMode, base_length: usize, base: Arc<P>, order: Vec<String>, from: usize, schema: &str, supersessions: EffectiveSupersessions, edits: &impl ReplayEdits<Mutation>) -> Result<Self, VcsError> { ReplayOwnedState::new_after_prefix(mode, base_length, base, order, from, schema, supersessions, edits).map(Self::retain) }
    pub fn with_convergence(mut self, convergence: Option<ReplayConvergence<P>>) -> Self { self.original.replace_convergence(convergence); self }
    pub fn recording(mut self, stride: usize, prefixes: Vec<(usize, Arc<P>)>) -> Self { self.original.record_stride = stride.max(1); self.original.recorded.extend(prefixes); self.original.record(); self }
    pub fn with_base(mut self, generation: u64, revision: [u8; 32]) -> Self { self.original.generation = generation; self.original.revision = revision; self }
    pub fn with_drafts(mut self, drafts: protocol::HistoryInputDrafts) -> Self { self.original.drafts = drafts; self }
    pub fn with_retirement_factories(mut self, snapshots: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>, mutations: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>) -> Self where P: Send + Sync + 'static, Mutation: Send + 'static { self.original.replay_retirement_factory = Some(semio_framework_os_spr::command::registered_replay_retirement_factory(snapshots, mutations)); self }
    pub fn with_operation_preparation(mut self, factory: Arc<dyn ArtifactReplayPreparationFactory<P, Mutation>>) -> Self { self.original.operation_preparation_factory = Some(factory); self }
    pub fn step(&mut self, edits: &impl ReplayEdits<Mutation>, deadline: &mut dyn FnMut() -> bool) -> Result<ReplayStep, VcsError> {
        if self.retiring { return Err(VcsError::HistoryReplaying); }
        self.original.step(edits, deadline)
    }
    pub fn step_operations(&mut self, edits: &impl ReplayEdits<Mutation>, maximum_operations: usize, deadline: &mut dyn FnMut() -> bool) -> Result<ReplayStep, VcsError> {
        if self.retiring { return Err(VcsError::HistoryReplaying); }
        self.original.step_operations(edits, maximum_operations, deadline)
    }
    /// 🏃️ Steps the replay to its result without a deadline; a refusal retires the raw replay unscheduled before it surfaces.
    pub fn run(mut self, edits: &impl ReplayEdits<Mutation>) -> Result<EditReplayResult<P, Mutation>, VcsError> where P: 'static, Mutation: 'static {
        let outcome = loop {
            match self.step(edits, &mut || false) {
                Ok(ReplayStep::Pending(_)) => {}
                Ok(ReplayStep::Finished(_)) => break self.finish(),
                Err(error) => break Err(error),
            }
        };
        if outcome.is_err() {
            let _ = self.retire_unscheduled();
        }
        outcome
    }

    /// 🏁️ The finished replay's result; refused while operations remain.
    pub fn finish(&mut self) -> Result<EditReplayResult<P, Mutation>, VcsError> {
        if !self.finished || self.retiring {
            return Err(VcsError::HistoryReplaying);
        }
        let outcomes = std::mem::take(&mut self.outcomes);
        let worst = self.report_worst;
        let result = ReplayResultOwnedState {
            order: std::mem::replace(&mut self.order, semio_framework_os_vcs::HistoryPageStack::empty()),
            from: self.from,
            supersessions: std::mem::take(&mut self.supersessions),
            drafts: std::mem::take(&mut self.drafts),
            generation: self.generation,
            revision: self.revision,
            state: self.state.take(),
            converged_at: self.converged_at,
            committed: std::mem::take(&mut self.committed),
            quarantined: std::mem::take(&mut self.quarantined),
            rebased_inverse: std::mem::take(&mut self.rebased_inverse),
            replayed: std::mem::take(&mut self.replayed),
            recorded: std::mem::take(&mut self.recorded),
            unit_flags: std::mem::take(&mut self.unit_flags),
            report: protocol::ReplayReport { from_position: u32::try_from(self.from).unwrap_or(u32::MAX), outcomes, worst },
        };
        let mut residual = Self::retain(std::mem::replace(&mut *self.original, ReplayOwnedState::terminal()));
        *residual.raw_retirement = self.raw_retirement.take();
        *residual.raw_factory = self.raw_factory.take();
        Ok(EditReplayResult { original: std::mem::ManuallyDrop::new(result), residual: std::mem::ManuallyDrop::new(Some(residual)) })
    }

}

/// 🧳️ Finished publication data retains its original residual replay alongside any untaken outputs.
pub struct EditReplayResult<P, Mutation: self::Mutation<P>> {
    pub original: std::mem::ManuallyDrop<ReplayResultOwnedState<P, Mutation>>,
    pub residual: std::mem::ManuallyDrop<Option<EditReplay<P, Mutation>>>,
}

impl<P, Mutation: self::Mutation<P>> std::ops::Deref for EditReplayResult<P, Mutation> {
    type Target = ReplayResultOwnedState<P, Mutation>;
    fn deref(&self) -> &Self::Target { &self.original }
}

impl<P, Mutation: self::Mutation<P>> std::ops::DerefMut for EditReplayResult<P, Mutation> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.original }
}

pub struct ReplayOwnedState<P, Mutation>
where
    Mutation: self::Mutation<P>,
{
    pub mode: ReplayMode,
    pub order: semio_framework_os_vcs::HistoryPageStack<String>,
    pub from: usize,
    prefix_until: usize,
    pub prefix_work_progress: bool,
    operation_progress: bool,
    pub cursor: usize,
    pub operation: usize,
    pub schema: String,
    pub supersessions: EffectiveSupersessions,
    pub state: Option<Arc<P>>,
    pub candidate: Option<Arc<P>>,
    pub edit_messages: semio_framework_os_spr::command::ReplayMessageAccumulator,
    pub edit_inverse: semio_framework_value::list::PagedList<Mutation, {usize::MAX}>,
    pub message_settlement: Option<semio_framework_os_store_ledger::edit_message_clamp::settlement::EditMessageSettlement>,
    closed_outcomes: usize,
    pub committed: Vec<String>,
    pub(crate) quarantined: Vec<String>,
    pub rebased_inverse: Vec<(String, semio_framework_value::list::PagedList<Mutation, {usize::MAX}>)>,
    pub replayed: Vec<semio_framework_os_spr::EditMessages>,
    pub outcomes: Vec<protocol::MutationReplayOutcome>,
    pub(crate) report_worst: Option<semio_framework_diagnostic::Severity>,
    pub unit_flags: protocol::HistoryFoldIndex<([u8; 32], usize), bool>,
    pub operation_preparation_factory: Option<Arc<dyn ArtifactReplayPreparationFactory<P, Mutation>>>,
    pub(crate) operation_preparation: Option<ArtifactReplayPreparationTask<P, Mutation>>,
    pub prepared_operation: Option<ArtifactReplayPrepared<P, Mutation>>,
    pub(crate) operation_retirement: Option<ArtifactReplayPreparationRetirement<P, Mutation>>,
    pub(crate) operation_message_settlement: Option<replay_preparation::OperationMessageSettlement>,
    pub(crate) preparation_completed: ArtifactReplayPreparationProgress,
    pub(crate) replay_retirement_factory: Option<Arc<dyn replay_preparation::ArtifactReplayRetirementFactory<P, Mutation>>>,
    pub replay_retirement: Option<replay_preparation::ReplayRetirement<P, Mutation>>,
    pub(crate) prefix_message_retirement: Option<ArtifactStoreMessageLedgerRetirement>,
    pub(crate) convergence: Option<ReplayConvergence<P>>,
    pub convergence_search: Option<usize>,
    convergence_pending_finish: bool,
    pub converged_at: Option<usize>,
    record_stride: usize,
    pub recorded: Vec<(usize, Arc<P>)>,
    pub drafts: protocol::HistoryInputDrafts,
    pub generation: u64,
    pub revision: [u8; 32],
    pub progress: ReplayProgress,
    pub finished: bool,
}

impl<P, M: Mutation<P>> ReplayOwnedState<P, M> {
    pub fn terminal() -> Self {
        Self {
            mode: ReplayMode::Report, order: semio_framework_os_vcs::HistoryPageStack::empty(), from: 0, prefix_until: 0,
            prefix_work_progress: false, operation_progress: false, cursor: 0, operation: 0, schema: String::new(),
            supersessions: Default::default(), state: None, candidate: None, edit_messages: Default::default(),
            edit_inverse: Default::default(), message_settlement: None, closed_outcomes: 0, committed: Vec::new(),
            quarantined: Vec::new(), rebased_inverse: Vec::new(), replayed: Vec::new(), outcomes: Vec::new(),
            report_worst: None, unit_flags: protocol::HistoryFoldIndex::new(), operation_preparation_factory: None,
            operation_preparation: None, prepared_operation: None, operation_retirement: None,
            operation_message_settlement: None, preparation_completed: Default::default(), replay_retirement_factory: None,
            replay_retirement: None, prefix_message_retirement: None, convergence: None, convergence_search: None,
            convergence_pending_finish: false, converged_at: None, record_stride: 1, recorded: Vec::new(),
            drafts: protocol::HistoryInputDrafts::new(), generation: 0, revision: [0; 32],
            progress: ReplayProgress { done: 0, total: 0 }, finished: true,
        }
    }
}

impl<P, Mutation> ReplayOwnedState<P, Mutation>
where
    Mutation: self::Mutation<P> + OpBinary,
{
    /// 🐣️ A replay of `order[from..]` starting from `base`, the projection after `order[..from]`, reading every
    /// operation of a document of `schema` through `supersessions`. `edits` must hold every edit the suffix names.
    fn new(mode: ReplayMode, base: Arc<P>, order: Vec<String>, from: usize, schema: &str, supersessions: EffectiveSupersessions, edits: &impl ReplayEdits<Mutation>) -> Result<Self, VcsError> {
        if from > order.len() { return Err(VcsError::ValidationFailed(format!("edit replay starts at {from} past its {}-edit order", order.len()))); }
        let mut total = 0u32;
        for edit_id in &order[from..] {
            let edit = edits.replay_edit(edit_id).ok_or_else(|| VcsError::UnknownEdit(edit_id.clone()))?;
            total = total.saturating_add(u32::try_from(edit.forwards.len()).unwrap_or(u32::MAX));
        }
        let mut catalog = semio_framework_os_vcs::HistoryPageStack::new();
        catalog.extend(order);
        Self::new_counted(mode, base, catalog, from, schema, supersessions, total)
    }

    fn new_counted(mode: ReplayMode, base: Arc<P>, order: semio_framework_os_vcs::HistoryPageStack<String>, from: usize, schema: &str, supersessions: EffectiveSupersessions, total: u32) -> Result<Self, VcsError> {
        let total = total.saturating_add(u32::try_from(order.len().saturating_sub(from)).unwrap_or(u32::MAX));
        let replay = Self {
            mode,
            order,
            from,
            prefix_until: from,
            prefix_work_progress: false,
            operation_progress: false,
            cursor: from,
            operation: 0,
            schema: schema.to_string(),
            supersessions,
            state: Some(base),
            candidate: None,
            edit_messages: semio_framework_os_spr::command::ReplayMessageAccumulator::default(),
            edit_inverse: Default::default(),
            message_settlement: None,
            closed_outcomes: 0,
            committed: Vec::new(),
            quarantined: Vec::new(),
            rebased_inverse: Vec::new(),
            replayed: Vec::new(),
            outcomes: Vec::new(),
            report_worst: None,
            unit_flags: protocol::HistoryFoldIndex::new(),
            operation_preparation_factory: None,
            operation_preparation: None,
            prepared_operation: None,
            operation_retirement: None,
            operation_message_settlement: None,
            preparation_completed: ArtifactReplayPreparationProgress::default(),
            replay_retirement_factory: None,
            replay_retirement: None,
            prefix_message_retirement: None,
            convergence: None,
            convergence_search: None,
            convergence_pending_finish: false,
            converged_at: None,
            record_stride: usize::MAX,
            recorded: Vec::new(),
            drafts: protocol::HistoryInputDrafts::new(),
            generation: 0,
            revision: [0; 32],
            progress: ReplayProgress { done: 0, total },
            finished: false,
        };
        if replay.from > replay.order.len() {
            return Err(VcsError::ValidationFailed(format!("edit replay starts at {} past its {}-edit order", replay.from, replay.order.len())));
        }
        Ok(replay)
    }

    /// 🐣️ [`Self::new`] starting from `base`, the projection after `order[..base_length]` for a `base_length <= from`: the
    /// replay first folds `order[base_length..from]` plainly — stepped under the same deadline as its suffix and recording the
    /// prefix ring on the way — and then replays `order[from..]` as [`Self::new`] does, so no synchronous fold precedes it.
    #[allow(clippy::too_many_arguments)]
    fn new_after_prefix(mode: ReplayMode, base_length: usize, base: Arc<P>, order: Vec<String>, from: usize, schema: &str, supersessions: EffectiveSupersessions, edits: &impl ReplayEdits<Mutation>) -> Result<Self, VcsError> {
        if base_length > from {
            return Err(VcsError::ValidationFailed(format!("edit replay base after {base_length} edits lies past its start {from}")));
        }
        let mut replay = Self::new(mode, base, order, from, schema, supersessions, edits)?;
        replay.cursor = base_length;
        Ok(replay)
    }

    pub fn with_convergence(mut self, convergence: Option<ReplayConvergence<P>>) -> Self {
        self.replace_convergence(convergence);
        self
    }

    fn replace_convergence(&mut self, convergence: Option<ReplayConvergence<P>>) {
        if let Some(previous) = std::mem::replace(&mut self.convergence, convergence) {
            for (_, checkpoint) in previous.checkpoints {
                retire_shared_projection::<P, Mutation>(checkpoint);
            }
        }
    }

    /// 🧵️ Takes `appended` operations a tick appended to `edit_id` into the total when that edit is the unreplayed tail of
    /// this replay's order (the grown edit is read by id when the replay gets there), and forgets the convergence probe,
    /// whose previous head predates the tick.
    pub fn absorb_tail_growth(&mut self, edit_id: &str, appended: usize) {
        if self.cursor < self.order.len() && self.order.last().is_some_and(|tail| tail == edit_id) {
            self.progress.total = self.progress.total.saturating_add(u32::try_from(appended).unwrap_or(u32::MAX));
        }
        self.replace_convergence(None);
    }

    /// 🎙️ Keeps the projection after every replayed prefix whose length is a multiple of `stride`, for the prefix ring —
    /// seeded with `prefixes`, the retained projections the base was folded through, and with the base itself when its
    /// length is such a multiple.
    pub fn recording(mut self, stride: usize, prefixes: Vec<(usize, Arc<P>)>) -> Self {
        self.record_stride = stride.max(1);
        self.recorded.extend(prefixes);
        self.record();
        self
    }

    /// 🔢️ Stamps the store generation and content revision this replay was started against — the freshness a finished
    /// replay is committed under ([`ArtifactStore::commit_finished_replay`]).
    pub fn with_base(mut self, generation: u64, revision: [u8; 32]) -> Self {
        self.generation = generation;
        self.revision = revision;
        self
    }

    /// 🖊️ Records the draft inputs this replay previews, which a finalization authors as the supersession.
    pub fn with_drafts(mut self, drafts: protocol::HistoryInputDrafts) -> Self {
        self.drafts = drafts;
        self
    }

    /// 🆔️ The store generation this replay was started against.
    pub fn generation(&self) -> u64 {
        self.generation
    }

    /// 🗒️ The draft inputs this replay previews.
    pub fn drafts(&self) -> &protocol::HistoryInputDrafts {
        &self.drafts
    }

    /// 📈️ Replay work completed, including an edit's final diagnostic settlement.
    pub fn progress(&self) -> ReplayProgress {
        self.progress
    }

    /// ⌛️ Whether every operation of the suffix was replayed (or the replay converged).
    pub fn is_finished(&self) -> bool {
        self.finished
    }

    /// ⏭️ Replays operations until the replay finishes or `deadline`, asked after every operation, answers `true`.
    pub fn step(&mut self, edits: &impl ReplayEdits<Mutation>, deadline: &mut dyn FnMut() -> bool) -> Result<ReplayStep, VcsError> {
        self.step_with_operation_cap(edits, usize::MAX, deadline)
    }

    /// 🔢️ Caps folded mutations while retaining every structural deadline and owner check.
    pub fn step_operations(&mut self, edits: &impl ReplayEdits<Mutation>, maximum_operations: usize, deadline: &mut dyn FnMut() -> bool) -> Result<ReplayStep, VcsError> {
        if !self.operation_progress {
            if self.progress.done != 0 { return Err(VcsError::ValidationFailed("operation-counted replay authority must be selected before work begins".into())); }
            self.progress.total = self.progress.total.saturating_sub(u32::try_from(self.order.len().saturating_sub(self.from)).unwrap_or(u32::MAX));
            self.operation_progress = true;
        }
        self.step_with_operation_cap(edits, maximum_operations, deadline)
    }

    fn step_with_operation_cap(&mut self, edits: &impl ReplayEdits<Mutation>, maximum_operations: usize, deadline: &mut dyn FnMut() -> bool) -> Result<ReplayStep, VcsError> {
        if maximum_operations == 0 && !self.finished { return Ok(ReplayStep::Pending(self.progress)); }
        let mut completed_operations = 0usize;
        while !self.finished {
            if let Some(retirement) = self.replay_retirement.as_mut() {
                let body = retirement.next_copy_byte_demand().map_err(VcsError::InverseRefused)?;
                let capacity = retirement.next_capacity_byte_demand(body).map_err(VcsError::InverseRefused)?;
                if body.saturating_add(capacity) > 4096 { return Err(VcsError::ValidationFailed("replay cleanup native frame exceeds one body/capacity grant".into())); }
                let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: body, maximum_capacity_bytes: capacity, maximum_release_bytes: retirement.next_release_byte_demand().map_err(VcsError::InverseRefused)?, maximum_depth: retirement.next_depth_demand().map_err(VcsError::InverseRefused)? };
                let step = retirement.close_step(grant).map_err(VcsError::InverseRefused)?;
                if !step.progress().fits(grant) { return Err(VcsError::ValidationFailed("replay cleanup exceeded its admitted grant".into())); }
                if retirement.terminal_is_empty() {
                    self.replay_retirement = None;
                }
                if deadline() || step.progress().copied_items == 0 { return Ok(ReplayStep::Pending(self.progress)); }
                continue;
            }
            if let Some(owner) = self.prefix_message_retirement.as_mut() {
                let body = owner.next_copy_byte_demand();
                let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: body, maximum_capacity_bytes: 0, maximum_release_bytes: owner.next_release_byte_demand().map_err(VcsError::InverseRefused)?, maximum_depth: owner.next_depth_demand().map_err(VcsError::InverseRefused)? };
                let step = owner.close_step(grant).map_err(VcsError::InverseRefused)?;
                if !step.progress().fits(grant) { return Err(VcsError::ValidationFailed("prefix diagnostic cleanup exceeded its actual grant".into())); }
                if owner.terminal_is_empty() { self.prefix_message_retirement = None; }
                if deadline() || step.progress().copied_items == 0 { return Ok(ReplayStep::Pending(self.progress)); }
                continue;
            }
            if self.convergence_pending_finish {
                self.convergence_pending_finish = false;
                self.finished = true;
                self.progress.done = self.progress.total;
                break;
            }
            if self.convergence_search.is_some() {
                self.step_convergence();
                if deadline() { return Ok(ReplayStep::Pending(self.progress)); }
                continue;
            }
            let Some(edit_id) = self.order.get(self.cursor) else {
                self.finished = true;
                break;
            };
            let edit = edits.replay_edit(edit_id).ok_or_else(|| VcsError::UnknownEdit(edit_id.clone()))?;
            if self.cursor < self.prefix_until {
                if self.operation < edit.forwards.len() {
                    if self.operation_preparation_factory.is_some() {
                        if !self.step_prepared_operation(edit, true, deadline)? { deadline(); return Ok(ReplayStep::Pending(self.progress)); }
                        self.operation += 1;
                    completed_operations = completed_operations.saturating_add(1);
                        if self.prefix_work_progress { self.progress.done = self.progress.done.saturating_add(1); }
                        if deadline() || completed_operations >= maximum_operations { return Ok(ReplayStep::Pending(self.progress)); }
                        continue;
                    }
                    let index = self.operation;
                    let id = semio_framework_os_spr::mutation_id_for_edit_operation::<P, Mutation>(edit, index).expect("recorded operation identity");
                    if edit.forwards[index].may_emit_foreign_steps() {
                        let digest = CursorRevisionAccumulator::hash_record(b"edit-id", &[edit.id.as_bytes()]);
                        self.unit_flags.insert((digest, index), edit.forwards[index].foreign_step_source(self.state.as_ref().expect("replay prefix state"), 0).map_err(VcsError::from)?.is_some());
                    }
                    let effective = effective_operation::<P, Mutation>(&edit.forwards[index], index as u32, id, &self.schema, &self.supersessions);
                    if let Some(operation) = effective.operation() {
                        let state = self.state.as_mut().ok_or_else(|| VcsError::ValidationFailed("edit replay lost its running projection".into()))?;
                        if let (Some(next), _) = fold_operation::<P, Mutation>(state, operation, index as u32) { <Arc<P> as FoldProjection<P, Mutation>>::advance(state, next); }
                    }
                    self.operation += 1;
                    completed_operations = completed_operations.saturating_add(1);
                    if self.prefix_work_progress { self.progress.done = self.progress.done.saturating_add(1); }
                    if deadline() || completed_operations >= maximum_operations { return Ok(ReplayStep::Pending(self.progress)); }
                    continue;
                }
                self.cursor += 1;
                self.operation = 0;
                self.record();
                continue;
            }
            if self.operation < edit.forwards.len() {
                if self.operation_preparation_factory.is_some() {
                    if !self.step_prepared_operation(edit, false, deadline)? { deadline(); return Ok(ReplayStep::Pending(self.progress)); }
                    self.operation += 1;
                    completed_operations = completed_operations.saturating_add(1);
                    self.progress.done = self.progress.done.saturating_add(1);
                    if deadline() || completed_operations >= maximum_operations { return Ok(ReplayStep::Pending(self.progress)); }
                    continue;
                }
                self.replay_operation(edit)?;
                self.operation += 1;
                    completed_operations = completed_operations.saturating_add(1);
                self.progress.done = self.progress.done.saturating_add(1);
                if deadline() || completed_operations >= maximum_operations { return Ok(ReplayStep::Pending(self.progress)); }
                continue;
            }
            if !self.step_close_edit(&edit.id)? {
                if deadline() { return Ok(ReplayStep::Pending(self.progress)); }
                continue;
            }
            if !self.operation_progress { self.progress.done = self.progress.done.saturating_add(1); }
            self.cursor += 1;
            self.operation = 0;
            self.record();
            self.converge();
            if self.cursor == self.order.len() { self.finished = true; }
            if !self.finished && deadline() { return Ok(ReplayStep::Pending(self.progress)); }
        }
        Ok(ReplayStep::Finished(self.progress))
    }

    /// 🫀️ Replays operation `self.operation` of `edit`: its effective input is re-diffed and re-inverted against the running
    /// state and its outcome recorded. A `Report` replay never aborts on a leaf: an inverse the leaf refuses on the replayed
    /// state is one `Fatal` `mutation.inverse-refused` message on that mutation, the operation folds forward exactly as every
    /// messageless fold site folds it, and its edit stages no inverse rows for it — the report blocks finalizing until that
    /// mutation is edited or withdrawn. A `Merge` replay refuses with [`VcsError::InverseRefused`].
    fn replay_operation(&mut self, edit: &Edit<Mutation>) -> Result<(), VcsError> {
        let index = self.operation;
        let mutation_id = semio_framework_os_spr::mutation_id_for_edit_operation::<P, Mutation>(edit, index).expect("recorded operation identity");
        if edit.forwards[index].may_emit_foreign_steps() {
            let digest = CursorRevisionAccumulator::hash_record(b"edit-id", &[edit.id.as_bytes()]);
            let base = self.candidate.as_ref().or(self.state.as_ref()).expect("replay operation state");
            self.unit_flags.insert((digest, index), edit.forwards[index].foreign_step_source(base, 0).map_err(VcsError::from)?.is_some());
        }
        let effective = effective_operation::<P, Mutation>(&edit.forwards[index], index as u32, mutation_id, &self.schema, &self.supersessions);
        let mut messages: Vec<semio_framework_os_spr::MutationMessage> = effective.fault().cloned().into_iter().collect();
        if let Some(operation) = effective.operation() {
            let base = match self.mode {
                ReplayMode::Merge(_) => self.candidate.as_ref().or(self.state.as_ref()),
                ReplayMode::Report => self.state.as_ref(),
            }
            .ok_or_else(|| VcsError::ValidationFailed("edit replay lost its running projection".into()))?;
            let (back, refused) = match operation.inverse(base) {
                Ok(mut back) => {
                    back.reverse();
                    (back, None)
                }
                Err(cause) if self.mode == ReplayMode::Report => (Vec::new(), Some(cause)),
                Err(cause) => return Err(VcsError::InverseRefused(cause)),
            };
            let next = match self.mode {
                ReplayMode::Report => {
                    let (next, operation_messages) = fold_operation::<P, Mutation>(base, operation, index as u32);
                    messages = operation_messages;
                    next
                }
                ReplayMode::Merge(_) => {
                    let (applied, operation_messages) = apply_operation::<P, Mutation>(base, operation, index as u32);
                    messages = operation_messages;
                    match applied {
                        Ok(next) => Some(next),
                        Err(error) => {
                            retire_scratch_operations::<P, Mutation>(back);
                            return Err(error.into());
                        }
                    }
                }
            };
            if let Some(cause) = refused {
                let mut fatal = semio_framework_os_spr::MutationMessage::fatal("mutation.inverse-refused", format!("the operation has no inverse on the replayed state: {cause}")).at([effective.mutation_id.0.clone()]);
                fatal.op_index = Some(index as u32);
                messages.push(fatal);
            }
            match next {
                Some(next) => {
                    self.edit_inverse.extend(back);
                    let next = Arc::new(next);
                    let displaced = match self.mode {
                        ReplayMode::Merge(_) => self.candidate.replace(next),
                        ReplayMode::Report => self.state.replace(next),
                    };
                    if let Some(displaced) = displaced {
                        retire_shared_projection::<P, Mutation>(displaced);
                    }
                }
                None => retire_scratch_operations::<P, Mutation>(back),
            }
        }
        let worst = semio_framework_os_spr::worst_level(&messages);
        self.report_worst = self.report_worst.max(worst);
        let mut drain = semio_framework_os_spr::command::OperationMessageDrain::new(protocol::MutationReplayOutcome { mutation_id: effective.mutation_id.clone(), edit_id: edit.id.clone(), op_index: index as u32, worst, messages, superseded: effective.superseded(), withdrawn: effective.withdrawn() });
        while !drain.is_finished() {
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: drain.next_copy_byte_demand(&self.edit_messages), maximum_capacity_bytes: drain.next_capacity_byte_demand(&self.edit_messages).map_err(VcsError::InverseRefused)?, maximum_release_bytes: drain.next_release_byte_demand(), maximum_depth: 1 };
            drain.step(&mut self.edit_messages, grant).map_err(VcsError::InverseRefused)?;
        }
        self.outcomes.push(drain.take().expect("cold original operation messages transferred"));
        Ok(())
    }

    /// 📨️ Settles one bounded diagnostic unit before sealing an edit's staged owners.
    fn step_close_edit(&mut self, edit_id: &str) -> Result<bool, VcsError> {
        let Some(settlement) = self.message_settlement.as_mut() else {
            self.message_settlement = Some(semio_framework_os_store_ledger::edit_message_clamp::settlement::EditMessageSettlement::new(edit_id, &self.edit_messages, self.closed_outcomes, self.outcomes.len()));
            return Ok(false);
        };
        if !settlement.is_finished() {
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: settlement.next_copy_byte_demand(&self.edit_messages).map_err(VcsError::InverseRefused)?, maximum_capacity_bytes: settlement.next_capacity_byte_demand().map_err(VcsError::InverseRefused)?, maximum_release_bytes: settlement.next_release_byte_demand(&self.edit_messages).map_err(VcsError::InverseRefused)?, maximum_depth: 64 };
            if grant.maximum_copy_bytes.saturating_add(grant.maximum_capacity_bytes) > ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES { return Err(VcsError::ValidationFailed("edit message settlement exceeds its body/capacity grant".into())); }
            settlement.step(&mut self.edit_messages, &mut self.outcomes, grant).map_err(VcsError::InverseRefused)?;
            return Ok(false);
        }
        let rejects = match self.mode {
            ReplayMode::Merge(policy) => semio_framework_os_spr::worst_level(settlement.messages()).is_some_and(|level| policy.rejects(level)),
            ReplayMode::Report => false,
        };
        if rejects {
            if let Some(candidate) = self.candidate.take() { self.begin_replay_snapshot_retirement(candidate); return Ok(false); }
            if !self.edit_inverse.terminal_is_empty() {
                let inverse = std::mem::take(&mut self.edit_inverse);
                self.begin_replay_mutations_retirement(inverse);
                return Ok(false);
            }
        } else if let Some(candidate) = self.candidate.take() {
            if let Some(previous) = self.state.replace(candidate) { self.begin_replay_snapshot_retirement(previous); return Ok(false); }
        }
        let messages = self.message_settlement.as_mut().expect("completed edit message selection").take_messages().expect("final bounded diagnostic owner");
        drop(self.message_settlement.take());
        self.closed_outcomes = self.outcomes.len();
        let inverse = std::mem::take(&mut self.edit_inverse);
        self.replayed.push(semio_framework_os_spr::EditMessages { edit_id: edit_id.to_string(), messages });
        if rejects {
            self.quarantined.push(edit_id.to_string());
        } else {
            self.rebased_inverse.push((edit_id.to_string(), inverse));
            self.committed.push(edit_id.to_string());
        }
        Ok(true)
    }

    pub fn record(&mut self) {
        if self.mode != ReplayMode::Report || self.cursor % self.record_stride != 0 || self.cursor >= self.order.len() {
            return;
        }
        if let Some(state) = self.state.as_ref() {
            self.recorded.push((self.cursor, Arc::clone(state)));
        }
    }

    fn converge(&mut self) {
        if self.mode != ReplayMode::Report { return; }
        let Some(convergence) = self.convergence.as_ref() else { return };
        if self.cursor < convergence.horizon || self.cursor >= self.order.len() { return; }
        self.convergence_search = Some(0);
    }

    fn step_convergence(&mut self) {
        let Some(index) = self.convergence_search else { return };
        let Some(convergence) = self.convergence.as_ref() else { self.convergence_search = None; return };
        let Some((length, previous)) = convergence.checkpoints.get(index) else { self.convergence_search = None; return };
        if *length != self.cursor {
            self.convergence_search = Some(index + 1);
            return;
        }
        self.convergence_search = None;
        let Some(state) = self.state.as_ref() else { return };
        let pointer_equal = Arc::ptr_eq(state, previous);
        let cold_authority = self.operation_preparation_factory.is_none() && self.replay_retirement_factory.is_none();
        if !pointer_equal && !(cold_authority && (convergence.equal)(state, previous)) { return; }
        let Some(head) = convergence.checkpoints.last().filter(|(length, _)| *length == self.order.len()).map(|(_, head)| Arc::clone(head)) else { return };
        self.converged_at = Some(self.cursor);
        self.convergence_pending_finish = true;
        if let Some(previous) = self.state.replace(head) { self.begin_replay_snapshot_retirement(previous); }
    }

}

/// 🎁️ What a finished [`EditReplay`] produced: the projection it reached, the edits it kept and quarantined, their
/// rebased inverses and messages, and its per-mutation [`protocol::ReplayReport`]. Untaken owners remain attached.
pub struct ReplayResultOwnedState<P, Mutation>
where
    Mutation: self::Mutation<P>,
{
    pub unit_flags: protocol::HistoryFoldIndex<([u8; 32], usize), bool>,
    pub order: semio_framework_os_vcs::HistoryPageStack<String>,
    pub from: usize,
    pub supersessions: EffectiveSupersessions,
    pub drafts: protocol::HistoryInputDrafts,
    pub generation: u64,
    pub revision: [u8; 32],
    pub state: Option<Arc<P>>,
    pub converged_at: Option<usize>,
    pub committed: Vec<String>,
    pub(crate) quarantined: Vec<String>,
    pub rebased_inverse: Vec<(String, semio_framework_value::list::PagedList<Mutation, {usize::MAX}>)>,
    pub replayed: Vec<semio_framework_os_spr::EditMessages>,
    pub recorded: Vec<(usize, Arc<P>)>,
    pub report: protocol::ReplayReport,
}

impl<P, Mutation> ReplayResultOwnedState<P, Mutation>
where
    Mutation: self::Mutation<P>,
{
    /// ⏲️ The store generation the replay was started against.
    pub fn generation(&self) -> u64 {
        self.generation
    }

    /// 📃️ The draft inputs the replay previewed.
    pub fn drafts(&self) -> &protocol::HistoryInputDrafts {
        &self.drafts
    }

    /// 🖼️ The projection the replay reached (the previous head when it converged).
    pub fn state(&self) -> Option<&Arc<P>> {
        self.state.as_ref()
    }

    /// 🎴️ Takes the reached projection; its retirement moves to the caller.
    pub fn take_state(&mut self) -> Option<Arc<P>> {
        self.state.take()
    }

    /// 🤝️ The prefix length at which the replay met the previous projection, if it did.
    pub fn converged_at(&self) -> Option<usize> {
        self.converged_at
    }

    /// ✅️ The edits the replay kept, in order.
    pub fn committed(&self) -> &[String] {
        &self.committed
    }

    /// 🚧️ The edits a `Merge` policy quarantined, in order.
    pub fn quarantined(&self) -> &[String] {
        &self.quarantined
    }

    /// 📨️ Every replayed edit's messages, in order.
    pub fn replayed(&self) -> &[semio_framework_os_spr::EditMessages] {
        &self.replayed
    }

    /// 📊️ The per-mutation outcomes.
    pub fn report(&self) -> &protocol::ReplayReport {
        &self.report
    }

    /// 🪝️ Takes the per-mutation outcomes.
    pub fn take_report(&mut self) -> protocol::ReplayReport {
        std::mem::take(&mut self.report)
    }

    /// ☑️ Takes the kept edit ids.
    pub fn take_committed(&mut self) -> Vec<String> {
        std::mem::take(&mut self.committed)
    }

    /// 🪤️ Takes the quarantined edit ids.
    pub fn take_quarantined(&mut self) -> Vec<String> {
        std::mem::take(&mut self.quarantined)
    }

    /// 📬️ Takes the replayed edits' messages.
    pub fn take_replayed(&mut self) -> Vec<semio_framework_os_spr::EditMessages> {
        std::mem::take(&mut self.replayed)
    }

    /// 🧳️ Takes the recorded prefix projections `(length, projection)`; their retirement moves to the caller.
    pub fn take_recorded(&mut self) -> Vec<(usize, Arc<P>)> {
        std::mem::take(&mut self.recorded)
    }

    /// 🔙️ Takes every kept edit's rebased inverse; their retirement moves to the caller.
    pub fn take_rebased_undo_rows(&mut self) -> Vec<(String, semio_framework_value::list::PagedList<Mutation, {usize::MAX}>)> {
        std::mem::take(&mut self.rebased_inverse)
    }
}

/// 🟰️ Whether two supersession maps install the same effective inputs — which transition installed them aside.
pub fn same_effective_inputs(left: &EffectiveSupersessions, right: &EffectiveSupersessions) -> bool {
    left.len() == right.len() && left.iter().zip(right.iter()).all(|((left_target, left), (right_target, right))| left_target == right_target && left.replacement == right.replacement)
}

/// 📸️ A retained projection after the first `length` applied edits, valid while `digest` names the live effective
/// prefix ([`forward_prefix_digests`]).
pub struct PrefixSnapshot<P> {
    pub length: usize,
    pub digest: [u8; 32],
    pub snapshot: Arc<P>,
}

/// 💍️ The most prefix projections a store retains, whatever its history length; the stride adapts so they spread over the
/// whole applied history ([`ArtifactStore::prefix_ring_capacity`]).
pub const PREFIX_SNAPSHOT_RING_CAPACITY: usize = 16;

/// 🪙️ The fewest prefix projections a store retains once its history is long enough to stride.
pub const PREFIX_SNAPSHOT_RING_FLOOR: usize = 8;

/// 🎟️ Displaced owners one reprojection displaces besides its message entries and prefix projections: the tail
/// snapshot and its id, `current`, the applied, redo and checkpoint owners, the supersessions, the revision bump and a
/// full prefix-ring prune.
pub const REPROJECTION_OWNER_SLOTS: usize = 10 + PREFIX_SNAPSHOT_RING_CAPACITY;

#[path="⏳pending/🦀️.rs"]
pub(crate) mod pending;
pub use pending::{PendingLocalStep,PendingReprojection,ReprojectionAdvance,TailStep};

#[path="⏱️budget/🦀️.rs"]
mod budget;
pub use budget::ReplayTurnBudget;
pub use semio_framework_os_store_ledger::lifted_replay::{EffectiveSupersessions, FoldProjection, edit_supersession_digest, supersession_digest};
