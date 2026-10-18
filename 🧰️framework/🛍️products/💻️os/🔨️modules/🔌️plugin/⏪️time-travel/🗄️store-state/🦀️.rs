//! 🗄️ Store state of the time-travel session: the store-free owners trait, the per-store history-edit state, the replay, preview and commit steps and the typed store commands, outputs and queries. See [`super`].

use super::*;

//#region 🔖️StoreState
/// 🧹️ The store-free half of one store's history-edit owners: whether a replay runs, whether owners still retire, the
/// terminal witness and one bounded retirement unit — callable on a composed member's erased owners without its store.
#[doc(hidden)]
pub trait TimeTravelOwners: Send {
    fn replaying(&self) -> bool;
    fn preview_progress(&self) -> Option<store::os_store::replay::ReplayProgress>;
    fn processed(&self) -> Option<u32>;
    fn has_pending_work(&self) -> bool;
    fn terminal_is_empty(&self) -> bool;
    fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError>;
    fn command_pending(&self)->bool;
    fn cancel_prepared_command(&mut self);
    fn retire_step(&mut self, grant: RetainedCloneGrant) -> Result<Option<RetainedCloneStep>, Fault>;
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

/// 🗃️ The typed owners of a history edit on ONE store — the document's own or one composed member's (design §12): the
/// edited operation's kind (staged by an open until the session adopts it), the draft preview, the running Report replay,
/// its finished result and replayed head, the retirements of the snapshots they displaced (taken from the store at once,
/// so they retire without it) and the operations a draft discarded.
#[doc(hidden)]
pub struct TimeTravelStoreState<P, Mu: ::protocol::Mutation<P>> {
    pub(super) discard_effects: Option<Box<discard_decision::TimeTravelDiscardStoreEffects<P,Mu>>>,
    pub label_of: fn(&Mu) -> LocalizedLabel,
    pub(super) staged: Option<Mu>,
    pub(super) kind: Option<Mu>,
    pub(super) preview: Option<store::os_store::snapshot_read::ArtifactDerivedSnapshot<P>>,
    pub(super) preview_job: Option<store::os_store::snapshot_read::ArtifactDerivedHistoryPreview<P, Mu>>,
    pub(super) preview_input: Option<protocol::InputReplacement>,
    pub(super) replay: Option<store::os_store::snapshot_read::ArtifactDerivedReplay<P, Mu>>,
    pub(super) finished: Option<store::os_store::replay::EditReplayResult<P, Mu>>,
    pub(super) head: Option<store::os_store::snapshot_read::ArtifactDerivedSnapshot<P>>,
    pub(super) snapshot_aliases: ManuallyDrop<[Option<store::os_store::snapshot_read::ArtifactDerivedSnapshot<P>>; 8]>,
    pub(super) snapshot_issuers: ManuallyDrop<Option<(Arc<dyn semio_framework_value::ArtifactOwnedValueRetirementFactory<P>>,Arc<dyn semio_framework_value::ArtifactOwnedValueRetirementFactory<Mu>>)>>,
    pub(super) snapshot_close: ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    pub(super) snapshot_issuer_close: [Option<semio_framework_value::FactoryAuthority>; 2],
    pub(crate) retirements: VecDeque<Box<dyn store::ErasedSnapshotRetirement>>,
    pub(crate) discarded: Vec<Mu>,
    pub(crate) discarded_pending: Option<Mu>,
    pub(super) discarded_active: Option<Box<dyn store::ErasedSnapshotRetirement>>,
    pub(crate) discarded_factory: Option<Arc<dyn semio_framework_value::ArtifactOwnedValueRetirementFactory<Mu>>>,
    pub(super) discarded_factory_close: Option<semio_framework_value::FactoryAuthority>,
}

impl<P,Mu:Mutation<P>> Drop for TimeTravelStoreState<P,Mu>{
    fn drop(&mut self){
        let empty=self.snapshot_aliases.iter().all(Option::is_none)&&self.snapshot_close.is_none()&&self.snapshot_issuers.is_none()&&self.snapshot_issuer_close.iter().all(Option::is_none);
        assert!(std::thread::panicking()||empty,"original derived snapshot custody requires granted retirement");
        if empty{unsafe{ManuallyDrop::drop(&mut self.snapshot_aliases);ManuallyDrop::drop(&mut self.snapshot_close);ManuallyDrop::drop(&mut self.snapshot_issuers)}}
    }
}

impl<P,Mu:Mutation<P>> TimeTravelStoreState<P,Mu>{
    /// 🎟️ Whether discarded operations wait for an issuer the store has not yet lent.
    pub fn discard_issuer_missing(&self)->bool{self.discarded_factory.is_none()&&(!self.discarded.is_empty()||self.discarded_pending.is_some())}
    /// 🎟️ Adopts the store's operation retirement issuer for the discarded operations.
    pub fn adopt_discard_issuer(&mut self,factory:Arc<dyn semio_framework_value::ArtifactOwnedValueRetirementFactory<Mu>>){self.discarded_factory=Some(factory);}
    /// ♻️ Queues the retirement of a displaced snapshot so it retires without the store.
    pub fn queue_retirement(&mut self,owner:Box<dyn store::ErasedSnapshotRetirement>){self.retirements.push_back(owner);}
}

/// ⏭️ What one replay slice did: progress, a completed report (the head is swapped in), or a fault.
#[doc(hidden)]
pub enum TimeTravelReplayStep {
    Pending { done: u32, total: u32 },
    Completed(protocol::ReplayReport),
    Faulted,
}

/// 🖼️ One preview slice leaves progress or publishes a complete selected-mutation projection.
#[doc(hidden)]
pub enum TimeTravelPreviewStep {
    Pending,
    Completed(Vec<protocol::MutationMessage>),
}

/// 🌿️ How a finalize ended on its store; a finalize carries the encoded envelopes of every transition it authored (an
/// overwrite's `Supersede`; an alternative's pending-edit commit, `Branch` and scoped `Supersede`), in authoring order.
#[doc(hidden)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TimeTravelCommit {
    Finalized { authored: Vec<u8> },
    Stale,
    Blocked,
    Failed,
}

/// 📨️ One typed history-edit operation against the store a session edits.
#[doc(hidden)]
pub enum TimeTravelStoreCommand {
    Open { target: MutationId, current: Option<protocol::InputReplacement>, withdraw: bool },
    Adopt(bool),
    Rebuild(DslValue),
    Preview { target: MutationId, replacement: protocol::InputReplacement, drafts: protocol::HistoryInputDrafts, deadline_us: u64, clock: fn() -> Option<u64> },
    StepPreview { deadline_us: u64, clock: fn() -> Option<u64> },
    StartReplay { drafts: protocol::HistoryInputDrafts, from: MutationId },
    CancelReplay,
    StepReplay { deadline_us: u64, clock: fn() -> Option<u64> },
    Read(TimeTravelStage),
}

/// 📬️ What a [`TimeTravelStoreCommand`] answered.
#[doc(hidden)]
pub enum TimeTravelStoreOutput {
    Opened(Result<(TimeTravelEditor, protocol::InputReplacement), TimeTravelActionRefusal>),
    Rebuilt(Result<protocol::InputReplacement, String>),
    Previewed(TimeTravelPreviewStep),
    ReplayStarted(bool),
    Stepped(TimeTravelReplayStep),
    Read(Option<(store::os_store::snapshot_read::ErasedSnapshotRead, [u8; 32])>),
    Done,
}

/// 🔎️ One store-only read a session takes synchronously.
#[doc(hidden)]
pub enum TimeTravelStoreQuery {
    Base,
    Positions(Vec<MutationId>),
    ShownValue(TimeTravelStage),
}

/// 📬️ What a [`TimeTravelStoreQuery`] answered.
#[doc(hidden)]
pub enum TimeTravelQueryOutput {
    Base(TimeTravelBase),
    Positions(Option<Vec<TimeTravelTarget>>),
    Value(Option<DslValue>),
}

impl<P, Mu> TimeTravelStoreState<P, Mu>
where
    P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + Send + Sync + 'static,
    Mu: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ::protocol::Mutation<P> + ::protocol::OpBinary + ::protocol::OpText + Send + 'static,
{
    /// 🏗️ Owners at rest, labelling the store's operations with `label_of`.
    pub fn new(label_of: fn(&Mu) -> LocalizedLabel) -> Self {
        Self { discard_effects:None,label_of, staged: None, kind: None, preview: None, preview_job: None, preview_input: None, replay: None, finished: None, head: None, snapshot_aliases: ManuallyDrop::new(std::array::from_fn(|_|None)), snapshot_issuers: ManuallyDrop::new(None), snapshot_close: ManuallyDrop::new(None), snapshot_issuer_close: std::array::from_fn(|_|None), retirements: VecDeque::new(), discarded: Vec::new(), discarded_pending: None, discarded_active: None, discarded_factory: None, discarded_factory_close: None }
    }

    /// 🪞️ The snapshot the session shows at `stage`: the draft preview while editing; while replaying that preview — or,
    /// for a replay started from a review (Replay again, a restored draft, a moved base), the head reviewed last, so no
    /// window falls back to the committed document in between; the replayed head while reviewing.
    pub fn shown(&self, stage: TimeTravelStage) -> Option<&store::os_store::snapshot_read::ArtifactDerivedSnapshot<P>> {
        match stage {
            TimeTravelStage::Inactive => None,
            TimeTravelStage::Editing => self.preview.as_ref(),
            TimeTravelStage::Replaying => self.preview.as_ref().or(self.head.as_ref()),
            TimeTravelStage::Reviewing | TimeTravelStage::Choosing | TimeTravelStage::Finalizing => self.head.as_ref(),
        }
    }

    fn snapshot_custody_available(&self, count:usize)->Result<(),Fault>{
        if self.snapshot_issuer_close.iter().any(Option::is_some){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original snapshot issuer tickets must close before accepting another source").into_fault())}
        if self.snapshot_aliases.iter().filter(|slot|slot.is_none()).count()<count{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original derived snapshots await their granted retirement turn").into_fault())}
        Ok(())
    }

    pub fn retire(&mut self, store: &ArtifactStore<P, Mu>, snapshot: Option<store::os_store::snapshot_read::ArtifactDerivedSnapshot<P>>) -> Result<(), Fault> {
        if let Some(snapshot)=snapshot {
            let slot=self.snapshot_aliases.iter_mut().find(|slot|slot.is_none()).expect("original snapshot custody was reserved before source publication");
            *slot=Some(snapshot);
            if self.snapshot_issuers.is_none() {
                match (store.owned_snapshot_retirement_factory(),store.owned_mutation_retirement_factory()) {
                    (Ok(snapshots),Ok(mutations))=>*self.snapshot_issuers=Some((Arc::clone(snapshots),Arc::clone(mutations))),
                    _=>return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original derived snapshot retains its missing retirement issuer").into_fault()),
                }
            }
        }
        Ok(())
    }

    fn cancel_preview(&mut self, store: &ArtifactStore<P, Mu>) -> Result<(), Fault> {
        let grant=history_retirement_frame_grant(ArtifactStore::<P,Mu>::history_read_retirement_birth_bytes());
        if let Some((retirement,progress)) = store.retire_derived_history_preview(&mut self.preview_job,grant).map_err(ValueError::into_fault)? { assert!(progress.fits(grant));self.retirements.push_back(retirement); }
        self.preview_input = None;
        Ok(())
    }

    fn cancel_replay(&mut self, store: &ArtifactStore<P, Mu>) -> Result<(), Fault> {
        let grant=history_retirement_frame_grant(ArtifactStore::<P,Mu>::history_read_retirement_birth_bytes());
        if let Some((retirement,progress)) = store.retire_derived_report_replay(&mut self.replay,grant).map_err(ValueError::into_fault)? { assert!(progress.fits(grant));self.retirements.push_back(retirement); }
        Ok(())
    }

    fn cancel_finished(&mut self, store: &ArtifactStore<P, Mu>) -> Result<(), Fault> {
        let grant=history_retirement_frame_grant(ArtifactStore::<P,Mu>::history_read_retirement_birth_bytes());
        if let Some((retirement,progress)) = store.retire_finished_history_replay(&mut self.finished,grant).map_err(ValueError::into_fault)? { assert!(progress.fits(grant));self.retirements.push_back(retirement); }
        Ok(())
    }

    pub fn discard(&mut self, op: Option<Mu>) {
        self.discarded.extend(op);
    }

    /// 🧽️ Drops what `stage` no longer shows: the kind outside `Editing`, the draft preview once the replay finished or
    /// the session left, the replay outside `Replaying`, every replay owner once it is inactive.
    pub fn settle(&mut self, store: &ArtifactStore<P, Mu>, stage: TimeTravelStage) -> Result<(), Fault> {
        self.snapshot_custody_available(2)?;
        if stage != TimeTravelStage::Editing {
            self.cancel_preview(store)?;
            let kind = self.kind.take();
            self.discard(kind);
            let staged = self.staged.take();
            self.discard(staged);
        }
        if !matches!(stage, TimeTravelStage::Editing | TimeTravelStage::Replaying) {
            let preview = self.preview.take();
            self.retire(store, preview)?;
        }
        if stage != TimeTravelStage::Replaying {
            self.cancel_replay(store)?;
        }
        if stage == TimeTravelStage::Inactive {
            self.cancel_finished(store)?;
            let head = self.head.take();
            self.retire(store, head)?;
        }
        Ok(())
    }

    /// 🔎️ A store-only read.
    pub fn query(owners: Option<&Self>, store: &ArtifactStore<P, Mu>, query: TimeTravelStoreQuery) -> TimeTravelQueryOutput {
        match query {
            TimeTravelStoreQuery::Base => TimeTravelQueryOutput::Base(TimeTravelBase { content_revision: store.content_revision() }),
            TimeTravelStoreQuery::Positions(targets) => TimeTravelQueryOutput::Positions(targets.into_iter().map(|target| store.mutation_position(&target).ok().map(|(position, _)| TimeTravelTarget { position: u32::try_from(position).unwrap_or(u32::MAX), mutation: target })).collect()),
            TimeTravelStoreQuery::ShownValue(stage) => TimeTravelQueryOutput::Value(owners.and_then(|owners| owners.shown(stage)).map(|shown| semio_framework_value::ToValue::to_value(shown.snapshot()))),
        }
    }

    /// 📨️ Runs one typed command against `store`.
    pub async fn run(&mut self, store: &mut ArtifactStore<P, Mu>, command: TimeTravelStoreCommand) -> Result<TimeTravelStoreOutput, Fault> {
        Ok(match command {
            TimeTravelStoreCommand::Open { target, current, withdraw } => TimeTravelStoreOutput::Opened(self.open(store, &target, current.as_ref(), withdraw)),
            TimeTravelStoreCommand::Adopt(adopt) => {
                let staged = self.staged.take();
                match adopt {
                    true => {
                        let previous = std::mem::replace(&mut self.kind, staged);
                        self.discard(previous);
                    }
                    false => self.discard(staged),
                }
                TimeTravelStoreOutput::Done
            }
            TimeTravelStoreCommand::Rebuild(candidate) => TimeTravelStoreOutput::Rebuilt(self.rebuild(store, candidate)),
            TimeTravelStoreCommand::Preview { target, replacement, drafts, deadline_us, clock } => {
                self.cancel_preview(store)?;
                self.preview_job = Some(store.begin_derived_history_preview(target, drafts).map_err(|error| error.into_fault())?);
                self.preview_input = Some(replacement);
                TimeTravelStoreOutput::Previewed(self.step_preview(store, deadline_us, clock)?)
            }
            TimeTravelStoreCommand::StepPreview { deadline_us, clock } => TimeTravelStoreOutput::Previewed(self.step_preview(store, deadline_us, clock)?),
            TimeTravelStoreCommand::StartReplay { drafts, from } => {
                self.cancel_replay(store)?;
                match store.begin_derived_report_replay(drafts, Some(from)) {
                    Ok(replay) => {
                        self.replay = Some(replay);
                        TimeTravelStoreOutput::ReplayStarted(true)
                    }
                    Err(_) => TimeTravelStoreOutput::ReplayStarted(false),
                }
            }
            TimeTravelStoreCommand::CancelReplay => {
                self.cancel_replay(store)?;
                TimeTravelStoreOutput::Done
            }
            TimeTravelStoreCommand::StepReplay { deadline_us, clock } => TimeTravelStoreOutput::Stepped(self.step_replay(store, deadline_us, clock)?),
            TimeTravelStoreCommand::Read(stage) => TimeTravelStoreOutput::Read(match self.shown(stage) {
                Some(shown) => Some((store.snapshot_read_derived(shown).map_err(|error| error.into_fault())?, store.content_revision())),
                None => None,
            }),
        })
    }

    /// 🪪️ Commits the original finished replay through the caller's existing identity recipient.
    pub async fn commit_finished(&mut self, store: &mut ArtifactStore<P, Mu>, drafts: protocol::HistoryInputDrafts, finalization: store::HistoryFinalization, actor: Option<&str>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<TimeTravelCommit, Fault> {
        if self.finished.as_ref().is_none_or(|finished| *finished.drafts() != drafts) {
            self.cancel_finished(store)?;
            return Ok(TimeTravelCommit::Stale);
        }
        if actor.is_some_and(|actor| actor != store.local_actor_id().0.as_str()) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("time-travel.actor"), "history edit actor differs from its opened session"));
        }
        let finished = self.finished.take().expect("admitted finished history replay");
        let known: BTreeSet<MutationId> = store.envelope().transitions.iter().map(|transition| transition.mutation_id.clone()).collect();
        Ok(match store.commit_finished_replay(finished, finalization, identity).await {
            Ok(_) => {
                let authored: Vec<_> = store.envelope().transitions.iter().filter(|transition| !known.contains(&transition.mutation_id)).cloned().collect();
                TimeTravelCommit::Finalized { authored: store::os_spr::encode_envelopes(&authored) }
            }
            Err(vcs::VcsError::Stale { .. }) => TimeTravelCommit::Stale,
            Err(vcs::VcsError::Rejected { .. }) => TimeTravelCommit::Blocked,
            Err(error) => return Err(error.into_fault()),
        })
    }

    /// ✏️ The editor of `target` and its effective input before the session (its payload schema, input descriptors,
    /// validator and the value it starts from: `current`, its accepted draft, else its input); its kind is staged until
    /// the session adopts it. Refused for an unknown operation and for one whose inputs cannot be edited: it plans foreign
    /// steps, declares no input schema, or its schema describes no input the editor shows as a row over that value (design
    /// §22.20: an editor with zero rows never opens; a schema that cannot be read still opens, naming why) — unless the
    /// editor opens to `withdraw` it (a history row's Withdraw, design §22.1): then the store's supersede law decides
    /// ([`time_travel_admits_withdrawal`]), and an operation without editable inputs opens on an editor without inputs.
    /// An operation of a cross-document unit takes no supersession in this document alone (`AppliedMutation::unit`, the
    /// store's authoring law): its Withdraw answers the store's own refusal, its inputs are not editable.
    pub fn open(&mut self, store: &ArtifactStore<P, Mu>, target: &MutationId, current: Option<&protocol::InputReplacement>, withdraw: bool) -> Result<(TimeTravelEditor, protocol::InputReplacement), TimeTravelActionRefusal> {
        let refusal = if withdraw { TimeTravelActionRefusal::NotWithdrawable } else { TimeTravelActionRefusal::NotEditable };
        let row = store.applied_mutation(target).map_err(|_| TimeTravelActionRefusal::UnknownMutation)?;
        if row.unit.is_some() {
            return Err(if withdraw { TimeTravelActionRefusal::UnitSpansDocuments } else { TimeTravelActionRefusal::NotEditable });
        }
        let schema_id = store.envelope().schema.clone();
        let original = match row.supersession {
            Some(supersession) => supersession.replacement.clone(),
            None => protocol::InputReplacement::Input { schema: schema_id, payload: <Mu as ::protocol::OpBinary>::encode_op(row.operation).map_err(|_| refusal)? },
        };
        let decode = |replacement: &protocol::InputReplacement| match replacement {
            protocol::InputReplacement::Input { payload, .. } => <Mu as ::protocol::OpBinary>::decode_op(payload).ok(),
            protocol::InputReplacement::Withdrawn => None,
        };
        let kind = decode(&original).unwrap_or_else(|| row.operation.clone());
        let value = match decode(current.unwrap_or(&original)) {
            Some(op) => {
                let value = op.payload_value();
                op.retire_cold();
                value
            }
            None => kind.payload_value(),
        };
        let read = kind.input_schema().filter(|_| !kind.may_emit_foreign_steps()).map(|schema| (schema, mutation_input_defs(schema, &registered_input_schema_document)));
        let read = read.filter(|(_, inputs)| inputs.as_ref().map_or(true, |inputs| !time_travel_input_rows(inputs, &value).is_empty()));
        let admitted = if withdraw { time_travel_admits_withdrawal::<P, Mu>(row.operation) } else { read.is_some() };
        if !admitted {
            kind.retire_cold();
            return Err(refusal);
        }
        let (schema, inputs, inputs_refused, validator) = match read {
            Some((schema, read)) => {
                let (inputs, inputs_refused) = match read {
                    Ok(inputs) => (inputs, None),
                    Err(error) => (Vec::new(), Some(error)),
                };
                let documents = time_travel_schema_documents(schema);
                (Some(schema), inputs, inputs_refused, semio_framework_schema::OwnedJsonSchemaValidator::compile_with_documents(schema, &documents.iter().map(String::as_str).collect::<Vec<_>>()).map_err(|error| error.to_string()))
            }
            None => (None, Vec::new(), None, Err(TimeTravelLabel::RefusalNotEditable.en().to_string())),
        };
        let editor = TimeTravelEditor {
            target: target.clone(),
            position: u32::try_from(row.position).unwrap_or(u32::MAX),
            op_index: row.op_index,
            label: (self.label_of)(&kind),
            schema,
            inputs,
            inputs_refused,
            validator,
            value,
            outcome: Vec::new(),
            refused: None,
        };
        let previous = self.staged.replace(kind);
        self.discard(previous);
        Ok((editor, original))
    }

    /// 🎚️ The validated draft payload `candidate` rebuilt into the edited kind and canonically encoded.
    pub fn rebuild(&mut self, store: &ArtifactStore<P, Mu>, candidate: DslValue) -> Result<protocol::InputReplacement, String> {
        let op = self.kind.as_ref().ok_or("the draft editor lost the operation kind it rebuilds")?.with_payload_value(candidate).map_err(|error| error.to_string())?;
        let encoded = <Mu as ::protocol::OpBinary>::encode_op(&op);
        self.discard(Some(op));
        Ok(protocol::InputReplacement::Input { schema: store.envelope().schema.clone(), payload: encoded.map_err(|error| error.to_string())? })
    }

    /// 🖼️ One bounded prefix slice; only a completed preview displaces the last complete projection.
    fn step_preview(&mut self, store: &mut ArtifactStore<P, Mu>, deadline_us: u64, clock: fn() -> Option<u64>) -> Result<TimeTravelPreviewStep, Fault> {
        self.snapshot_custody_available(2)?;
        let Some(job) = self.preview_job.as_mut() else { return Ok(TimeTravelPreviewStep::Pending) };
        let mut deadline = || clock().is_none_or(|now| now >= deadline_us);
        loop {
            let grant=history_planning_retirement_grant(job.planning_retirement_demands(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES).map_err(ValueError::into_fault)?);
            if matches!(store.step_derived_history_preview(job, grant, &mut deadline).map_err(|error| error.into_fault())?, store::os_store::replay::ReplayStep::Finished(_)) { break; }
            if deadline() { return Ok(TimeTravelPreviewStep::Pending); }
        }
        let base = store.finish_derived_history_preview(&mut self.preview_job).map_err(|error| error.into_fault())?;
        let replacement = self.preview_input.take().ok_or_else(|| Fault::new(FaultOrigin::Plugin, FaultCode::new("timeTravel.preview-input-absent"), "history preview lost its selected input"))?;
        self.cancel_preview(store)?;
        let (preview, outcome) = match replacement {
            protocol::InputReplacement::Withdrawn => (base, Vec::new()),
            protocol::InputReplacement::Input { payload, .. } => {
                let op = <Mu as ::protocol::OpBinary>::decode_op(&payload).map_err(|error| error.into_fault())?;
                let derived = store.derive_history_snapshot(&base, &op);
                self.discard(Some(op));
                let (next, messages) = derived.map_err(|error| error.into_fault())?;
                match next {
                    Some(next) => { self.retire(store, Some(base))?; (next, messages) }
                    None => (base, messages),
                }
            }
        };
        let previous = self.preview.replace(preview);
        self.retire(store, previous)?;
        Ok(TimeTravelPreviewStep::Completed(outcome))
    }

    /// ⏭️ Steps the replay until `deadline_us` on `clock`; completion swaps the replayed head in and keeps the finished result.
    fn step_replay(&mut self, store: &ArtifactStore<P, Mu>, deadline_us: u64, clock: fn() -> Option<u64>) -> Result<TimeTravelReplayStep, Fault> {
        self.snapshot_custody_available(1)?;
        let Some(replay) = self.replay.as_mut() else { return Ok(TimeTravelReplayStep::Faulted) };
        let mut deadline = || clock().is_none_or(|now| now >= deadline_us);
        let grant=history_planning_retirement_grant(replay.planning_retirement_demands(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES).map_err(ValueError::into_fault)?);
        Ok(match store.step_derived_report_replay(replay, grant, &mut deadline) {
            Ok(store::os_store::replay::ReplayStep::Pending(progress)) => TimeTravelReplayStep::Pending { done: progress.done, total: progress.total },
            Ok(store::os_store::replay::ReplayStep::Finished(_)) => {
                match store.finish_derived_report_replay(&mut self.replay).and_then(|(result, head)| store.replay_report(&result).map(|report| (result, head, report))) {
                    Ok((result, head, report)) => {
                        let previous = std::mem::replace(&mut self.head, head);
                        self.retire(store, previous)?;
                        self.cancel_finished(store)?;
                        self.finished = Some(result);
                        TimeTravelReplayStep::Completed(report)
                    }
                    Err(_) => TimeTravelReplayStep::Faulted,
                }
            }
            Err(_) => {
                self.cancel_replay(store)?;
                TimeTravelReplayStep::Faulted
            }
        })
    }
}
//#endregion 🔖️StoreState
