//! 🔢️ Prefix digests, replay progress and edit replay: the ledger order, the shared prefix digests, the superseded positions, the draft supersessions, the replay mode, steps and convergence, the edit replay with its result and the prefix snapshot ring limits. See [`super`].

use super::*;

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
    Mutation: super::Mutation<P>,
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
    Mutation: super::Mutation<P>,
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
    Mutation: super::Mutation<P>,
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
semio_framework_value::artifact_retire_leaf!(ReplayProgress);

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
pub struct EditReplay<P, Mutation: super::Mutation<P>> {
    pub original: std::mem::ManuallyDrop<ReplayOwnedState<P, Mutation>>,
    pub(crate) raw_retirement: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    pub(crate) raw_factory: std::mem::ManuallyDrop<Option<semio_framework_value::FactoryAuthority>>,
    pub retiring: bool,
}

impl<P, Mutation: super::Mutation<P>> std::ops::Deref for EditReplay<P, Mutation> {
    type Target = ReplayOwnedState<P, Mutation>;
    fn deref(&self) -> &Self::Target { &self.original }
}

impl<P, Mutation: super::Mutation<P>> std::ops::DerefMut for EditReplay<P, Mutation> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.original }
}

impl<P, Mutation: super::Mutation<P> + OpBinary> EditReplay<P, Mutation> {
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
        /// 🎟️ Executes one replay operation frontier under the caller's unchanged original resource allowance.
    pub fn step_granted(&mut self,edits:&impl ReplayEdits<Mutation>,grant:RetainedCloneGrant,control:&mut semio_framework_value::NativeDecodeControl<'_>,deadline:&mut dyn FnMut()->bool)->Result<ReplayStep,VcsError>{if self.retiring{return Err(VcsError::HistoryReplaying);}if deadline()||grant.maximum_items==0{return Ok(ReplayStep::Pending(self.progress));}{self.native_receipt=Default::default();self.original.step_with_operation_cap(edits,1,Some(grant),Some(control),&mut||true)}}
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
pub struct EditReplayResult<P, Mutation: super::Mutation<P>> {
    pub original: std::mem::ManuallyDrop<ReplayResultOwnedState<P, Mutation>>,
    pub residual: std::mem::ManuallyDrop<Option<EditReplay<P, Mutation>>>,
}

impl<P, Mutation: super::Mutation<P>> std::ops::Deref for EditReplayResult<P, Mutation> {
    type Target = ReplayResultOwnedState<P, Mutation>;
    fn deref(&self) -> &Self::Target { &self.original }
}

impl<P, Mutation: super::Mutation<P>> std::ops::DerefMut for EditReplayResult<P, Mutation> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.original }
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
