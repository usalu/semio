//! 🔭️ Derived history previews and report replays borrow the Store frontier without moving its cursor.
use super::*;
use super::replacement::{Replaced,ReplacementTurn,close_replacement,select_replacement};

macro_rules! replaced{($p:expr)=>{Replaced{wire:&mut $p.replacement.wire,receiving:&mut $p.replacement.receiving,foreign:&mut $p.replacement.foreign,operation:&mut $p.replacement.operation,phase:&mut $p.replacement.phase,input:&mut $p.history_input,source:$p.history_source.as_ref(),drafts:&mut $p.drafts,supersessions:$p.supersessions.as_mut().expect("live preview retains its original supersession index")}}}

impl<'a, P, Mutation, Log> StoreDerived<'a, P, Mutation, Log>
where
    P: semio_os_spr::StoreState,
    Mutation: semio_os_spr::StoreMutationFor<P>,
    Log: std::ops::Deref<Target = StoreLog<P, Mutation>>,
{
    pub fn admit_derived_source(&self, registry: &SnapshotReadRegistryHandle, generation: u64, revision: [u8; 32]) -> Result<(), VcsError> {
        self.cursor.admit_derived_source(&self.owners, &self.wire, registry, generation, revision)
    }

    pub fn seal_derived_snapshot(&self, owner: Arc<P>) -> ArtifactDerivedSnapshot<P> {
        self.cursor.seal_derived_snapshot(&self.owners, &self.wire, owner)
    }

    pub fn applied_revision(&self) -> &CursorRevisionAccumulator {
        self.cursor.applied_revision(&self.wire)
    }

    pub fn applied_edit(&self, position: usize) -> Result<&Edit<Mutation>, VcsError> {
        self.cursor.applied_edit(&self.log, &self.wire, position)
    }

    pub fn step_history_read_plan(&self, plan: &mut HistoryReadPlan,order:Option<&semio_os_vcs::HistoryPageStack<semio_os_vcs::ArtifactHistoryKey>>,grant:RetainedCloneGrant,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>, deadline: &mut dyn FnMut() -> bool) -> Result<(bool,RetainedCloneProgress), VcsError> {
        self.live.step_history_read_plan(&self.log, &self.wire, &self.cursor, plan,order,grant,control,deadline)
    }

    pub fn live_base(&self, length: usize) -> (usize, Arc<P>) {
        self.live.live_base(&self.log, &self.cursor, length)
    }

    pub fn convergence_checkpoints(&self, horizon: usize, equal: fn(&P, &P) -> bool) -> Result<ReplayConvergence<P>, VcsError> {
        self.live.convergence_checkpoints(&self.log, &self.cursor, horizon, equal)
    }
    pub fn generation(&self) -> u64 { self.cursor.generation(&self.wire) }
    pub fn content_revision(&self) -> [u8; 32] { self.cursor.content_revision(&self.wire) }
    pub fn generation_now(&self) -> u64 { self.cursor.generation_now(&self.wire) }
    pub fn content_revision_now(&self) -> [u8; 32] { self.cursor.content_revision_now(&self.wire) }
    pub fn merge_policy(&self) -> semio_os_spr::MergePolicy { self.log.merge_policy() }
    pub fn owned_mutation_retirement_factory(&self) -> Result<&Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>, ValueError> { self.owners.owned_mutation_retirement_factory() }
    pub fn retire_snapshot_alias(&self, alias: &mut Option<Arc<P>>, effect: &mut semio_framework_value::RetainedAllocationEffect<'_>) -> Result<Option<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress)>, ValueError> where P: Sync { self.owners.retire_snapshot_alias(alias, effect) }

}

impl<'a, P, Mutation, Log> StoreDerived<'a, P, Mutation, Log>
where
    P: semio_os_spr::StoreState,
    Mutation: semio_os_spr::StoreMutationFor<P>,
    Log: std::ops::Deref<Target = StoreLog<P, Mutation>>,
{
    pub fn finished_history_preview_read_demands(&self, source: &ArtifactDerivedHistoryPreview<P, Mutation>) -> Result<semio_framework_value::RetirementDemand, ValueError> where P: Sync {
        if !source.finished || source.state.is_none() || !source.registry.ptr_eq(&self.owners.snapshot_read_leases) || source.generation != self.generation() || source.revision != self.content_revision() || !source.registry.authority_matches(source.generation, source.revision) { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "finished checkpoint read requires its unchanged original Store frontier")); }
        Ok(semio_framework_value::RetirementDemand { copy_bytes: SnapshotRead::<P>::capture_copy_byte_demand(), depth: 1, ..Default::default() })
    }

    pub fn admit_finished_history_preview_read(&self, source: &ArtifactDerivedHistoryPreview<P, Mutation>, output: &mut Option<SnapshotRead<P>>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> where P: Sync {
        if output.is_some() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "finished checkpoint read output is already occupied")); }
        let demand = self.finished_history_preview_read_demands(source)?;
        if grant.maximum_items == 0 || !semio_framework_value::retained_clone::retained_clone_grant_admits(grant, demand) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let owner = Arc::clone(source.state.as_ref().unwrap());
        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() };
        let lease = match source.registry.try_issue_original(Arc::clone(&owner)) { Ok(lease) => lease, Err((error, alias)) => { drop(alias); return Err(error.into_value_error().with_retained_progress(progress)); } };
        *output = Some(SnapshotRead::new(owner, lease));
        Ok(RetainedCloneStep::Complete(progress))
    }

    pub fn retained_derived_read_demands(&self, source: &ArtifactDerivedSnapshot<P>) -> Result<semio_framework_value::RetirementDemand, ValueError>
    where P: Sync {
        if !source.registry.ptr_eq(&self.owners.snapshot_read_leases) { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "projection read requires its original Store issuer")); }
        Ok(semio_framework_value::RetirementDemand { copy_bytes: SnapshotRead::<P>::capture_copy_byte_demand() + size_of::<ErasedSnapshotRead>() + size_of::<Option<ErasedSnapshotRead>>(), depth: 2, ..Default::default() })
    }

    pub fn admit_retained_derived_read(&self, source: &ArtifactDerivedSnapshot<P>, output: &mut Option<ErasedSnapshotRead>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>
    where P: Sync {
        if output.is_some() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "original projection read output is already occupied")); }
        let demand = self.retained_derived_read_demands(source)?;
        if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant, demand) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let owner = Arc::clone(&source.owner);
        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() };
        let lease = match source.registry.try_issue(Arc::clone(&owner)) { Ok(lease) => lease, Err(alias) => { drop(alias); return Ok(RetainedCloneStep::Progress(progress)); } };
        *output = Some(ErasedSnapshotRead::new(owner, lease));
        Ok(RetainedCloneStep::Complete(progress))
    }

    pub fn derive_reference_provisional_snapshot(&self, source: &ArtifactDerivedSnapshot<P>, op: &Mutation) -> Result<Option<ArtifactDerivedSnapshot<P>>, VcsError> where P: Clone, Mutation: ColdMutation<P> {
        self.admit_derived_source(&source.registry, source.generation, source.revision)?;
        let outcome = <Mutation as ColdMutation<P>>::diff(op, source.snapshot());
        let applicable = outcome.is_applicable(protocol::MergePolicy::default());
        let (diff, _) = outcome.into_parts();
        let next = applicable.then(|| semio_os_spr::apply_diff_cold(&diff, source.snapshot()).ok()).flatten();
        <<Mutation as ColdMutation<P>>::ReferenceDiff as MutationDiff<P>>::retire_cold(diff);
        Ok(next.map(|owner| self.seal_derived_snapshot(Arc::new(owner))))
    }

    pub fn derive_reference_history_snapshot(&self, source: &ArtifactDerivedSnapshot<P>, op: &Mutation) -> Result<(Option<ArtifactDerivedSnapshot<P>>, Vec<protocol::MutationMessage>), VcsError> where P: Clone, Mutation: ColdMutation<P> {
        self.admit_derived_source(&source.registry, source.generation, source.revision)?;
        let (diff, mut messages) = <Mutation as ColdMutation<P>>::diff(op, source.snapshot()).into_parts();
        let applied = semio_os_spr::apply_diff_cold(&diff, source.snapshot());
        <<Mutation as ColdMutation<P>>::ReferenceDiff as MutationDiff<P>>::retire_cold(diff);
        let next = match applied {
            Ok(owner) => Some(self.seal_derived_snapshot(Arc::new(owner))),
            Err(error) => {
                messages.push(protocol::MutationMessage::fatal(error.code, error.message).at(error.target));
                None
            }
        };
        Ok((next, messages))
    }

    pub fn begin_derived_history_preview(&self, target: MutationId, drafts: protocol::HistoryInputDrafts) -> Result<ArtifactDerivedHistoryPreview<P, Mutation>, VcsError> {
        self.wire.ensure_durable_group_idle()?;
        Ok(ArtifactDerivedHistoryPreview { replacement:Default::default(),supersessions:Some(Default::default()),checkpoint:None,
            derived_operation:None,history_source:None,history_input:Default::default(),history_issuer:None,history_issuer_close:None,displaced_operation:None,operation_close:None,operation_identity:None,operation_adopted:false,preparation_receipt:Default::default(),preparation_work:None,plan: Some(HistoryReadPlan::new(Some(target), drafts, self.log.applied_edit_ids.len(), false)), plan_retirement:None, drafts: protocol::HistoryInputDrafts::new(), target: None, cursor: 0, operation: 0, state: None,
            progress: ReplayProgress::default(), registry: self.owners.snapshot_read_leases.clone(), generation: self.generation(), revision: self.content_revision(), marker: PhantomData, finished: false,
        })
    }

    pub fn begin_derived_report_replay(&self, drafts: protocol::HistoryInputDrafts, from: Option<MutationId>) -> Result<ArtifactDerivedReplay<P, Mutation>, VcsError> {
        self.wire.ensure_durable_group_idle()?;
        Ok(ArtifactDerivedReplay {capacity_ledger:Default::default(),replacement_wire:Default::default(),replacement_foreign:false,original_decode:None,original_selection:None,original_order:None,original_history_issuance:self.log.envelope.vcs.edits.retained_issuance(),target:Default::default(),target_operation:None,replacement:Default::default(),replacement_operation:None,replacement_phase:0,history_source:None,history_input:Default::default(),history_issuer:None,history_issuer_close:None,history_operation:None,failed_operation:None,preparation_receipt:Default::default(),preparation_work:None,replay: None, plan: Some(HistoryReadPlan::new(from, drafts, self.log.applied_edit_ids.len(), true)), plan_retirement:None, registry: self.owners.snapshot_read_leases.clone(), generation: self.generation(), revision: self.content_revision() })
    }

    pub fn begin_checkpoint_report_replay(&self,order:&semio_os_vcs::HistoryPageStack<semio_os_vcs::ArtifactHistoryKey>,supersessions:&mut semio_os_store_replay::EffectiveSupersessions)->Result<ArtifactDerivedReplay<P,Mutation>,VcsError>{let mut replay=self.begin_derived_report_replay(Default::default(),None)?;let identity=Some((order as*const _ as usize,order.len()));replay.original_order=identity;let plan=replay.plan.as_mut().unwrap();plan.original_order=identity;plan.supersessions=std::mem::take(supersessions);plan.original_supersessions=true;plan.first_input=0;Ok(replay)}

    pub fn begin_selected_report_replay(&self,selection:semio_os_vcs::OriginalHistoryEditSelection)->Result<ArtifactDerivedReplay<P,Mutation>,VcsError>{let selected=selection.selected_len(self.log.applied_edit_ids.len()).ok_or(VcsError::HistoryReplaying)?;if self.log.envelope.vcs.edits.get_key(selection.key).is_none(){return Err(VcsError::HistoryReplaying)}if !selection.restoring&&self.cursor.revision_accumulator.applied.get(selection.position).and_then(|record|record.ledger_key)!=Some(selection.key){return Err(VcsError::HistoryReplaying)}let mut replay=self.begin_derived_report_replay(Default::default(),None)?;replay.original_selection=Some(selection);let plan=replay.plan.as_mut().unwrap();plan.original_selection=Some(selection);plan.first_input=selection.position.min(selected);Ok(replay)}

    /// 📏️ Quotes the same original Report child selected by controlled cancellation.
    pub fn derived_report_replay_retirement_demands(&self,replay:&ArtifactDerivedReplay<P,Mutation>,body:usize)->Result<semio_framework_value::RetirementDemand,ValueError>where P:Sync{
        let header=|copy_bytes|semio_framework_value::RetirementDemand{copy_bytes,depth:1,..Default::default()};
        if !replay.replacement_wire.terminal_is_empty(){return replay.replacement_wire.close_demands(body)}
        if let Some(failed)=replay.failed_operation.as_ref(){return failed.next_demand(body)}
        if !replay.target.terminal_is_empty(){return replay.target.close_demands(body)}
        if replay.target_operation.is_some(){return Ok(header(size_of::<Option<(usize,usize,u8)>>()));}
        if replay.replacement_operation.is_some()&&replay.replay.as_ref().is_some_and(|raw|raw.original_preparation_ready()){
            if let Some(input)=replay.history_input.as_ref(){return if input.terminal_is_empty(){Ok(header(size_of::<Option<semio_framework_value::RetainedOwnedProjection<Mutation>>>()))}else{input.close_demand(body)}}
            if !replay.replacement.terminal_is_empty(){return replay.replacement.close_demands(body)}
            return Ok(Self::original_report_replacement_reset_demand());
        }
        if let Some(raw)=replay.replay.as_ref(){return raw.retirement_demands(body)}
        if let Some(input)=replay.history_input.as_ref(){return if input.terminal_is_empty(){Ok(header(size_of::<Option<semio_framework_value::RetainedOwnedProjection<Mutation>>>()))}else{input.close_demand(body)}}
        if let Some(source)=replay.history_source.as_ref(){return source.close_demands(&self.log.envelope.vcs.edits,body)}
        if let Some(issuer)=replay.history_issuer_close.as_ref(){return issuer.demands(body)}
        if replay.history_issuer.is_some(){return Ok(header(size_of::<Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<Mutation>>>>()))}
        Ok(semio_framework_value::RetirementDemand{capacity_bytes:semio_os_store_read::history_read_admission::history_read_retirement_birth_bytes::<P,Mutation>(),depth:1,..Default::default()})
    }

    pub fn history_edit_read_issuer_demands(&self)->semio_framework_value::RetirementDemand {edit_retirement::history_edit_read_issuer_demands::<Mutation>()}

    pub fn admit_history_edit_read_issuer(&self,slot:&mut Option<Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<Mutation>>>>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        let original=self.owners.mutation_retirement_factory.as_ref().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"history read requires the installed original mutation issuer"))?;
        edit_retirement::admit_history_edit_read_issuer(original,slot,grant)
    }

}

impl<'a, P, Mutation, Log> StoreDerived<'a, P, Mutation, Log>
where
    P: semio_os_spr::StoreState,
    Mutation: semio_os_spr::StoreMutationFor<P>,
    Log: std::ops::DerefMut<Target = StoreLog<P, Mutation>>,
{
    pub fn retire_derived_history_preview(&mut self,owner:&mut Option<ArtifactDerivedHistoryPreview<P,Mutation>>,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError>where P:Sync{self.retire_derived_history_preview_with_control(owner,effect,None)}
    pub fn retire_derived_history_preview_controlled(&mut self,owner:&mut Option<ArtifactDerivedHistoryPreview<P,Mutation>>,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,control:&mut dyn semio_framework_value::OriginalRetirementControl)->Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError>where P:Sync{self.retire_derived_history_preview_with_control(owner,effect,Some(control))}
    pub fn retire_derived_history_preview_with_control(&mut self,owner:&mut Option<ArtifactDerivedHistoryPreview<P,Mutation>>,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,mut control:Option<&mut dyn semio_framework_value::OriginalRetirementControl>)->Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError>where P:Sync{
        use semio_framework_value::{RetainedWorkPoll,OriginalRetirementPoll};
        let settled=|poll:OriginalRetirementPoll|->Result<RetainedCloneProgress,ValueError>{match poll{RetainedWorkPoll::Pending{..}=>Ok(Default::default()),RetainedWorkPoll::Complete(turn)=>turn.result.map(|step|step.progress()).map_err(semio_framework_value::OriginalRetirementFailure::into_value_error)}};
        let grant=effect.grant();
        let Some(preview) = owner.as_mut() else { return Ok(None) };preview.preparation_work=None;
        if !preview.registry.ptr_eq(&self.owners.snapshot_read_leases) { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"foreign history preview cannot retire through this Store")); }
        if let Some(operation)=preview.derived_operation.as_mut(){if !operation.is_closing(){let copy=2*size_of::<bool>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<copy{return Ok(None)}operation.cancel();preview.preparation_receipt=RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()};return Ok(None)}let poll=match control.as_mut(){Some(control)=>ErasedSnapshotRetirement::close_step_controlled(operation,effect.grant().maximum_copy_bytes,effect,*control),None=>ErasedSnapshotRetirement::close_step(operation,effect.grant().maximum_copy_bytes,effect)};preview.preparation_receipt=settled(poll)?;if operation.terminal_is_empty(){preview.derived_operation=None;}return Ok(None);}
        if preview.replacement.operation.is_some(){let control=control.as_mut().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original replacement closure requires its original decoder custody"))?;preview.preparation_receipt=close_replacement::<P,Mutation>(&mut replaced!(preview),effect,*control)?;return Ok(None);}
        if let Some(input)=preview.history_input.as_mut(){if input.terminal_is_empty(){let copy=size_of::<Option<semio_framework_value::retained_clone::RetainedOwnedProjection<Mutation>>>();if grant.maximum_items==0||grant.maximum_copy_bytes<copy||grant.maximum_depth==0{return Ok(None);}preview.history_input.vacate();preview.preparation_receipt=RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()};}else{preview.preparation_receipt=settled(input.close_step(effect))?;}return Ok(None);}
        if let Some(source)=preview.history_source.as_mut(){source.begin_close();let turn=source.close_step(&mut self.log.envelope.vcs.edits,effect);preview.preparation_work=Some(u64::try_from(effect.performed_work()).unwrap_or(u64::MAX));preview.preparation_receipt=turn.result?.progress();if source.terminal_is_empty(){preview.history_source=None;}return Ok(None);}
        if let Some(issuer)=preview.history_issuer_close.as_mut(){preview.preparation_receipt=settled(issuer.step(effect.grant().maximum_copy_bytes,effect))?;if issuer.terminal_is_empty(){preview.history_issuer_close=None;}return Ok(None);}
        if preview.history_issuer.is_some(){let d=semio_framework_value::RetirementDemand{copy_bytes:size_of::<Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<Mutation>>>>(),depth:1,..Default::default()};if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(None);}preview.history_issuer_close=Some(semio_framework_value::FactoryAuthority::new(preview.history_issuer.take().unwrap()));preview.preparation_receipt=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};return Ok(None);}
        self.owners.admit_history_owner_retirement(owner,grant,|retirement,preview|*retirement.preview=Some(preview))
    }

    pub fn retire_derived_report_replay(&mut self,owner:&mut Option<ArtifactDerivedReplay<P,Mutation>>,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError>where P:Sync{self.retire_derived_report_replay_with_control(owner,effect,None)}
    pub fn retire_derived_report_replay_controlled(&mut self,owner:&mut Option<ArtifactDerivedReplay<P,Mutation>>,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,control:&mut dyn semio_framework_value::OriginalRetirementControl)->Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError>where P:Sync{self.retire_derived_report_replay_with_control(owner,effect,Some(control))}
    pub fn retire_derived_report_replay_with_control(&mut self,owner:&mut Option<ArtifactDerivedReplay<P,Mutation>>,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,mut control:Option<&mut dyn semio_framework_value::OriginalRetirementControl>)->Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError>where P:Sync{
        use semio_framework_value::{RetainedWorkPoll,OriginalRetirementPoll};
        let settled=|poll:OriginalRetirementPoll|->Result<RetainedCloneProgress,ValueError>{match poll{RetainedWorkPoll::Pending{..}=>Ok(Default::default()),RetainedWorkPoll::Complete(turn)=>turn.result.map(|step|step.progress()).map_err(semio_framework_value::OriginalRetirementFailure::into_value_error)}};
        let grant=effect.grant();
        let Some(replay) = owner.as_mut() else { return Ok(None) };replay.preparation_work=None;
        if !replay.registry.ptr_eq(&self.owners.snapshot_read_leases) { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"foreign history replay cannot retire through this Store")); }
        if !replay.replacement_wire.terminal_is_empty(){let d=replay.replacement_wire.close_demands(grant.maximum_copy_bytes)?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(None)}let control=control.as_mut().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original canonical replacement closure requires its original decoder"))?;control.charge(d.capacity_bytes)?;replay.preparation_receipt=replay.replacement_wire.close_step(effect)?.progress();return Ok(None);}
        if let Some(failed)=replay.failed_operation.as_mut(){let poll=match control.as_mut(){Some(control)=>ErasedSnapshotRetirement::close_step_controlled(failed,effect.grant().maximum_copy_bytes,effect,*control),None=>ErasedSnapshotRetirement::close_step(failed,effect.grant().maximum_copy_bytes,effect)};replay.preparation_receipt=settled(poll)?;if failed.terminal_is_empty(){replay.failed_operation=None;}return Ok(None);}
        if !replay.target.terminal_is_empty(){replay.preparation_receipt=replay.target.close_step(effect)?.progress();return Ok(None);}
        if replay.target_operation.is_some(){let d=semio_framework_value::RetirementDemand{copy_bytes:size_of::<Option<(usize,usize,u8)>>(),depth:1,..Default::default()};if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(None)}replay.target_operation=None;replay.preparation_receipt=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};return Ok(None);}
        if replay.replacement_operation.is_some()&&replay.replay.as_ref().is_some_and(|raw|raw.original_preparation_ready()){let control=control.as_mut().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original replacement closure requires its original decoder custody"))?;replay.preparation_receipt=self.close_original_report_replacement(replay,effect,*control)?;return Ok(None);}
        if let Some(raw)=replay.replay.as_mut(){raw.cancel();let poll=match control.as_mut(){Some(control)=>raw.close_original_step_controlled(effect,*control),None=>raw.close_original_step(effect)};replay.preparation_receipt=settled(poll)?;if raw.terminal_is_empty(){replay.replay=None;}return Ok(None);}
        if let Some(input)=replay.history_input.as_mut(){if input.terminal_is_empty(){let d=semio_framework_value::RetirementDemand{copy_bytes:size_of::<Option<semio_framework_value::retained_clone::RetainedOwnedProjection<Mutation>>>(),depth:1,..Default::default()};if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(None);}replay.history_input.vacate();replay.preparation_receipt=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};}else{replay.preparation_receipt=settled(input.close_step(effect))?;}return Ok(None);}
        if let Some(source)=replay.history_source.as_mut(){source.begin_close();let turn=source.close_step(&mut self.log.envelope.vcs.edits,effect);replay.preparation_work=Some(u64::try_from(effect.performed_work()).unwrap_or(u64::MAX));replay.preparation_receipt=turn.result?.progress();if source.terminal_is_empty(){replay.history_source=None;replay.history_operation=None;}return Ok(None);}
        if let Some(issuer)=replay.history_issuer_close.as_mut(){replay.preparation_receipt=settled(issuer.step(effect.grant().maximum_copy_bytes,effect))?;if issuer.terminal_is_empty(){replay.history_issuer_close=None;}return Ok(None);}
        if replay.history_issuer.is_some(){let d=semio_framework_value::RetirementDemand{copy_bytes:size_of::<Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<Mutation>>>>(),depth:1,..Default::default()};if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(None);}replay.history_issuer_close=Some(semio_framework_value::FactoryAuthority::new(replay.history_issuer.take().unwrap()));replay.preparation_receipt=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};return Ok(None);}
        self.owners.admit_history_owner_retirement(owner,grant,|retirement,replay|retirement.replay.install_original(replay))
    }

    pub fn step_derived_history_preview(&mut self,preview:&mut ArtifactDerivedHistoryPreview<P,Mutation>,grant:RetainedCloneGrant,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,maximum_work:u64,deadline:&mut dyn FnMut()->bool)->Result<ReplayStep,VcsError>{self.step_derived_history_preview_inner(preview,None,grant,control,maximum_work,deadline)}

    pub fn step_derived_checkpoint_preview(&mut self,preview:&mut ArtifactDerivedHistoryPreview<P,Mutation>,checkpoint:&str,grant:RetainedCloneGrant,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,deadline:&mut dyn FnMut()->bool)->Result<ReplayStep,VcsError>{self.step_derived_history_preview_inner(preview,Some(checkpoint),grant,control,0,deadline)}

    pub fn step_derived_history_preview_inner(&mut self, preview: &mut ArtifactDerivedHistoryPreview<P, Mutation>,checkpoint:Option<&str>, retirement_grant:RetainedCloneGrant, control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>, maximum_work:u64, deadline: &mut dyn FnMut() -> bool) -> Result<ReplayStep, VcsError> {
        preview.preparation_work=None;
        self.admit_derived_source(&preview.registry, preview.generation, preview.revision)?;
        if preview.checkpoint.is_some(){let checkpoint=checkpoint.ok_or_else(||VcsError::InverseRefused(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"checkpoint preview requires its explicit original selection")))?;preview.checkpoint.as_ref().unwrap().validate(checkpoint).map_err(VcsError::InverseRefused)?;if !self.step_checkpoint_selection(preview,checkpoint,retirement_grant,deadline)?{return Ok(ReplayStep::Pending(preview.progress));}}

        if preview.plan_retirement.is_some() {
            preview.preparation_receipt=funded_plan_retirement(control,retirement_grant,|effect|preview.retire_completed_plan(effect))?;
            return Ok(ReplayStep::Pending(preview.progress));
        }
        if let Some(plan) = preview.plan.as_mut() {
            let(ready,p)=self.step_history_read_plan(plan,None,retirement_grant,control,deadline)?;preview.preparation_receipt=p;if !ready||p!=RetainedCloneProgress::default(){return Ok(ReplayStep::Pending(plan.progress));}
            let live_until=preview.retain_completed_plan();
            let (start, base) = self.live_base(live_until);
            preview.cursor = start;
            preview.state = Some(base);
            return Ok(ReplayStep::Pending(preview.progress));
        }
        if deadline()||retirement_grant.maximum_items==0{return Ok(ReplayStep::Pending(preview.progress));}
        preview.preparation_receipt=RetainedCloneProgress::default();
        if preview.operation_close.is_some(){preview.preparation_receipt=funded_plan_retirement(control,retirement_grant,|effect|semio_framework_value::close_factory_ticket(&mut preview.operation_close,effect.grant().maximum_copy_bytes,effect))?;return Ok(ReplayStep::Pending(preview.progress));}
        if let Some(owner)=preview.derived_operation.as_mut(){
            if owner.is_closing(){preview.preparation_receipt=funded_plan_retirement(control,retirement_grant,|effect|owner.close_step(effect.grant().maximum_copy_bytes,effect))?;if owner.terminal_is_empty(){drop(preview.derived_operation.take());}return Ok(ReplayStep::Pending(preview.progress));}
        }
        if preview.displaced_operation.is_some(){let Some(done)=control.with_effect(retirement_grant,ARTIFACT_STORE_PLAN_RETIREMENT_WORK,|_,effect|self.retire_snapshot_alias(&mut preview.displaced_operation,effect)).map_err(VcsError::InverseRefused)?else{return Ok(ReplayStep::Pending(preview.progress))};if let Some((owner,p))=done.map_err(VcsError::InverseRefused)?{preview.operation_close=Some(owner);preview.preparation_receipt=p;}return Ok(ReplayStep::Pending(preview.progress));}
        if preview.operation_adopted{
            if preview.replacement.operation.is_some(){preview.preparation_receipt=control.with_effect(retirement_grant,maximum_work.saturating_sub(semio_framework_value::ORIGINAL_INVOCATION_WORK),|ctl,effect|ctl.with_retirement(retirement_grant,|rctl|close_replacement::<P,Mutation>(&mut replaced!(preview),effect,rctl))).map_err(VcsError::InverseRefused)?.transpose().map_err(VcsError::InverseRefused)?.unwrap_or_default();return Ok(ReplayStep::Pending(preview.progress));}
            let return_source=preview.operation+1==self.history_preview_edit(preview)?.forwards.len()||preview.target==Some((preview.cursor,preview.operation+1));if return_source{if let Some(source)=preview.history_source.as_mut(){source.begin_close();let ledger=&mut self.log.envelope.vcs.edits;let mut work=None;let receipt=funded_plan_retirement(control,retirement_grant,|effect|{let turn=source.close_step(ledger,effect);work=Some(u64::try_from(effect.performed_work()).unwrap_or(u64::MAX));semio_framework_value::RetainedWorkPoll::Complete(turn).map_err(Into::into)})?;preview.preparation_work=work;preview.preparation_receipt=receipt;if source.terminal_is_empty(){preview.history_source=None;}return Ok(ReplayStep::Pending(preview.progress));}}
            let release=preview.operation_identity.as_ref().map_or(0,|id|id.0.capacity());let copy=size_of::<Option<MutationId>>()+size_of::<usize>();if retirement_grant.maximum_copy_bytes<copy||retirement_grant.maximum_release_bytes<release{return Ok(ReplayStep::Pending(preview.progress));}drop(preview.operation_identity.take());preview.operation+=1;preview.operation_adopted=false;preview.progress.done=preview.progress.done.saturating_add(1);preview.preparation_receipt=RetainedCloneProgress{copied_items:1,copied_bytes:copy,released_bytes:release,..Default::default()};return Ok(ReplayStep::Pending(preview.progress));
        }
        let at_end=if let Some(selection)=preview.checkpoint.as_ref(){selection.complete}else{let(position,op_index)=preview.target.ok_or_else(||VcsError::ValidationFailed("history preview lost its target".into()))?;preview.cursor==position&&preview.operation==op_index};
        if at_end{
            if let Some(factory)=preview.history_issuer_close.as_mut(){preview.preparation_receipt=funded_plan_retirement(control,retirement_grant,|effect|factory.step(effect.grant().maximum_copy_bytes,effect))?;if factory.terminal_is_empty(){preview.history_issuer_close=None;}return Ok(ReplayStep::Pending(preview.progress));}
            if preview.history_issuer.is_some(){let copy=size_of::<Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<Mutation>>>>();if retirement_grant.maximum_items==0||retirement_grant.maximum_copy_bytes<copy||retirement_grant.maximum_depth==0{return Ok(ReplayStep::Pending(preview.progress));}preview.history_issuer_close=Some(semio_framework_value::FactoryAuthority::new(preview.history_issuer.take().unwrap()));preview.preparation_receipt=RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()};return Ok(ReplayStep::Pending(preview.progress));}
            preview.finished=true;return Ok(ReplayStep::Finished(preview.progress));
        }
        if preview.operation==self.history_preview_edit(preview)?.forwards.len(){if let Some(source)=preview.history_source.as_mut(){source.begin_close();let ledger=&mut self.log.envelope.vcs.edits;let mut work=None;let receipt=funded_plan_retirement(control,retirement_grant,|effect|{let turn=source.close_step(ledger,effect);work=Some(u64::try_from(effect.performed_work()).unwrap_or(u64::MAX));semio_framework_value::RetainedWorkPoll::Complete(turn).map_err(Into::into)})?;preview.preparation_work=work;preview.preparation_receipt=receipt;if source.terminal_is_empty(){preview.history_source=None;}return Ok(ReplayStep::Pending(preview.progress));}if retirement_grant.maximum_items==0||retirement_grant.maximum_copy_bytes<2*size_of::<usize>()||retirement_grant.maximum_depth==0{return Ok(ReplayStep::Pending(preview.progress));}preview.cursor+=1;preview.operation=0;if let Some(selection)=preview.checkpoint.as_mut(){selection.advance_edit();}preview.preparation_receipt=RetainedCloneProgress{copied_items:1,copied_bytes:2*size_of::<usize>(),..Default::default()};return Ok(ReplayStep::Pending(preview.progress));}
        if preview.history_issuer.is_none(){preview.preparation_receipt=self.admit_history_edit_read_issuer(&mut preview.history_issuer,retirement_grant).map_err(VcsError::InverseRefused)?.progress();return Ok(ReplayStep::Pending(preview.progress));}
        if preview.history_source.is_none(){let slot=self.history_preview_edit_slot(preview)?;let key=self.log.envelope.vcs.edits.retained_read_key(slot).ok_or(VcsError::HistoryReplaying)?;if let Some((source,p))=semio_os_store_replay_operation::native::history::OriginalHistorySource::admit(key,self.log.envelope.vcs.edits.retained_read_issuer(key).map_err(VcsError::InverseRefused)?.unwrap_or(preview.history_issuer.as_ref().unwrap()),retirement_grant).map_err(VcsError::InverseRefused)?{preview.history_source=Some(source);preview.preparation_receipt=p;}return Ok(ReplayStep::Pending(preview.progress));}
        if !preview.history_input.is_some()&&preview.derived_operation.is_none()&&preview.replacement.operation.is_none(){let index=preview.operation;if self.history_preview_edit(preview)?.forwards.len()>index{let ledger=&mut self.log.envelope.vcs.edits;let source=preview.history_source.as_mut().unwrap();let input=&mut preview.history_input;let mut work=None;let receipt=funded_plan_retirement(control,retirement_grant,|effect|{let turn=source.advance(ledger,input,index+1,|edit|&edit.forwards[index],effect);work=Some(u64::try_from(effect.performed_work()).unwrap_or(u64::MAX));semio_framework_value::RetainedWorkPoll::Complete(turn).map_err(Into::into)})?;preview.preparation_work=work;preview.preparation_receipt=receipt;return Ok(ReplayStep::Pending(preview.progress));}}
        let edit=self.history_preview_edit(preview)?;
        if preview.operation==edit.forwards.len(){if let Some(source)=preview.history_source.as_mut(){source.begin_close();let ledger=&mut self.log.envelope.vcs.edits;let mut work=None;let receipt=funded_plan_retirement(control,retirement_grant,|effect|{let turn=source.close_step(ledger,effect);work=Some(u64::try_from(effect.performed_work()).unwrap_or(u64::MAX));semio_framework_value::RetainedWorkPoll::Complete(turn).map_err(Into::into)})?;preview.preparation_work=work;preview.preparation_receipt=receipt;if source.terminal_is_empty(){preview.history_source=None;}return Ok(ReplayStep::Pending(preview.progress));}if retirement_grant.maximum_copy_bytes<2*size_of::<usize>(){return Ok(ReplayStep::Pending(preview.progress));}preview.cursor+=1;preview.operation=0;if let Some(selection)=preview.checkpoint.as_mut(){selection.advance_edit();}preview.preparation_receipt=RetainedCloneProgress{copied_items:1,copied_bytes:2*size_of::<usize>(),..Default::default()};return Ok(ReplayStep::Pending(preview.progress));}
        let index=preview.operation;
        let id=if let Some(id)=edit.mutation_meta.get(index).and_then(|meta|meta.mutation_id.as_ref()){id}else{
            if preview.operation_identity.is_none(){let digits=if index==0{1}else{index.ilog10() as usize+1};let bytes=edit.id.len().saturating_add(1).saturating_add(digits);if bytes.saturating_add(size_of::<MutationId>())>4096||retirement_grant.maximum_capacity_bytes<bytes||retirement_grant.maximum_copy_bytes<bytes{return Ok(ReplayStep::Pending(preview.progress));}let mut id=String::new();id.try_reserve_exact(bytes).map_err(|_|VcsError::ValidationFailed("history operation identity reservation refused".into()))?;id.push_str(&edit.id);id.push('#');use std::fmt::Write;write!(&mut id,"{index}").unwrap();let capacity=id.capacity();preview.operation_identity=Some(MutationId(id));preview.preparation_receipt=RetainedCloneProgress{copied_items:1,copied_bytes:bytes,retained_capacity_bytes:capacity,..Default::default()};return Ok(ReplayStep::Pending(preview.progress));}preview.operation_identity.as_ref().unwrap()
        };
        if preview.derived_operation.is_none(){
            if preview.checkpoint.is_some(){if self.live.supersessions.get(id).is_some(){return Err(VcsError::InverseRefused(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original checkpoint replacement requires its retained semantic input issuer")));}}
            else{
                if preview.replacement.operation.is_none(){if let Some(selected)=select_replacement::<P,Mutation>(preview.history_source.as_ref().ok_or(VcsError::HistoryReplaying)?,&preview.drafts,preview.supersessions.as_ref().ok_or(VcsError::HistoryReplaying)?,index).map_err(VcsError::InverseRefused)?{let coordinate=(preview.cursor,index);if let Some(p)=self.begin_replacement(&mut replaced!(preview),coordinate,selected,retirement_grant)?{preview.preparation_receipt=p;}return Ok(ReplayStep::Pending(preview.progress));}}
                let turn=self.advance_replacement(&mut replaced!(preview),0,retirement_grant,control,maximum_work)?;
                match turn{
                    Some(ReplacementTurn::Pending(p))=>{preview.preparation_receipt=p;return Ok(ReplayStep::Pending(preview.progress));}
                    Some(ReplacementTurn::Withdrawn(p))=>{preview.operation_adopted=true;preview.preparation_receipt=p;return Ok(ReplayStep::Pending(preview.progress));}
                    None=>{}
                }
            }
            let source=ArtifactDerivedSnapshot{owner:Arc::clone(preview.state.as_ref().ok_or(VcsError::HistoryReplaying)?),registry:preview.registry.clone(),generation:preview.generation,revision:preview.revision};if let Some(p)=self.begin_derived_operation(&mut preview.derived_operation,&source,preview.history_input.slot(),index as u32,ArtifactDerivedOperationMode::HistoryPrefix,retirement_grant).map_err(VcsError::InverseRefused)?{preview.preparation_receipt=p;}return Ok(ReplayStep::Pending(preview.progress));}
        let owner=preview.derived_operation.as_mut().unwrap();
        if owner.is_finished(){if let Some(((next,_messages),p))=self.take_derived_operation(owner,retirement_grant).map_err(VcsError::InverseRefused)?{if let Some(next)=next{preview.displaced_operation=preview.state.replace(next.into_snapshot_owner());}preview.operation_adopted=true;preview.preparation_receipt=p;}return Ok(ReplayStep::Pending(preview.progress));}
        match self.step_derived_operation(owner,retirement_grant.maximum_copy_bytes,retirement_grant,control,deadline){Ok(step)=>{preview.preparation_receipt=step.progress();Ok(ReplayStep::Pending(preview.progress))},Err(error)=>{preview.preparation_receipt=error.retained_progress();Err(VcsError::InverseRefused(error.into_value_error()))}}
    }

    pub fn step_derived_report_replay(&mut self, owner: &mut ArtifactDerivedReplay<P, Mutation>, retirement_grant:RetainedCloneGrant, control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>, maximum_work:u64, deadline: &mut dyn FnMut() -> bool) -> Result<ReplayStep, VcsError> {
        self.step_derived_report_replay_in_order(owner,None,retirement_grant,control,maximum_work,deadline)
    }

    pub fn step_derived_report_replay_in_order(&mut self, owner: &mut ArtifactDerivedReplay<P, Mutation>, order:Option<&semio_os_vcs::HistoryPageStack<semio_os_vcs::ArtifactHistoryKey>>,retirement_grant:RetainedCloneGrant, control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>, maximum_work:u64, deadline: &mut dyn FnMut() -> bool) -> Result<ReplayStep, VcsError> {
        if owner.original_order!=order.map(|o|(o as*const _ as usize,o.len())){return Err(VcsError::InverseRefused(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"Report changed its original selected order source")))}
        self.admit_derived_source(&owner.registry, owner.generation, owner.revision)?;
        let recipient=control.retirement_recipient_identity().ok_or_else(||VcsError::InverseRefused(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"native Report requires its actual original decoder recipient")))?;let identity=(control.ledger_identity(),recipient);if owner.original_decode.is_some_and(|original|original!=identity){return Err(VcsError::InverseRefused(ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"native Report changed its original decoder continuation")))}if owner.original_decode.is_none(){let d=semio_framework_value::RetirementDemand{copy_bytes:size_of::<Option<(semio_framework_value::retained_clone::NativeAllocationIdentity,u64)>>(),depth:1,..Default::default()};if semio_framework_value::retained_clone::retained_clone_grant_admits(retirement_grant,d){owner.original_decode=Some(identity);owner.preparation_receipt=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};}return Ok(ReplayStep::Pending(owner.replay.as_ref().map_or_else(ReplayProgress::default,|replay|replay.progress())))}
        if owner.plan_retirement.is_some() {
            owner.preparation_receipt=funded_plan_retirement(control,retirement_grant,|effect|owner.retire_completed_plan(effect))?;
            return Ok(ReplayStep::Pending(owner.replay.as_ref().map_or_else(ReplayProgress::default,|replay|replay.progress())));
        }
        if let Some(plan) = owner.plan.as_mut() {
            let(ready,p)=self.step_history_read_plan(plan,order,retirement_grant,control,deadline)?;owner.preparation_receipt=p;if !ready||p!=RetainedCloneProgress::default(){return Ok(ReplayStep::Pending(plan.progress));}
            if self.owners.replay_preparation_factory.is_none()||self.owners.initial_snapshot_retirement_factory.is_none()||self.owners.mutation_retirement_factory.is_none(){return Err(VcsError::InverseRefused(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original report requires its installed semantic and retirement issuers")))}
            let capacity=semio_os_spr::command::registered_replay_retirement_factory_allocation_bytes::<P,Mutation>();let copy=size_of::<EditReplay<P,Mutation>>()+4*size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>();let d=semio_framework_value::RetirementDemand{copy_bytes:copy,capacity_bytes:capacity,depth:1,..Default::default()};if !semio_framework_value::retained_clone::retained_clone_grant_admits(retirement_grant,d){return Ok(ReplayStep::Pending(plan.progress))}control.admit_turn_capacity(retirement_grant.maximum_capacity_bytes).map_err(VcsError::InverseRefused)?;control.charge(capacity).map_err(VcsError::InverseRefused)?;
            let generation=owner.generation;let revision=owner.revision;
            let plan=owner.retain_completed_plan();
            let from = plan.first_input;
            let (start, base) = self.live_base(from);
            let total = plan.operations.saturating_sub(plan.counts[from]);
            let mut replay = EditReplay::new_counted(ReplayMode::Report, base, std::mem::replace(&mut plan.order,semio_os_vcs::HistoryPageStack::empty()), from, std::mem::take(&mut plan.native_schema), std::mem::take(&mut plan.supersessions), total)?;
            replay.cursor = start;
            replay.prefix_work_progress = true;
            replay.progress.total = replay.progress.total.saturating_add(plan.counts[from].saturating_sub(plan.counts[start]));
            replay.progress.done = plan.progress.done;
            replay.progress.total = replay.progress.total.saturating_add(plan.progress.done);
            replay = replay.with_base(generation, revision).with_drafts(std::mem::take(&mut plan.drafts));
            replay.operation_preparation_factory = (*self.owners.replay_preparation_factory).clone();
            if let (Some(snapshots), Some(mutations)) = ((*self.owners.initial_snapshot_retirement_factory).clone(), (*self.owners.mutation_retirement_factory).clone()) {
                replay = replay.with_retirement_factories(snapshots, mutations);
            } else if replay.operation_preparation_factory.is_some() {
                let _ = replay.retire_unscheduled();
                return Err(VcsError::ValidationFailed("interactive operation preparation requires exact replay retirement factories".into()));
            }
            owner.replay = Some(replay);owner.preparation_receipt=RetainedCloneProgress{copied_items:1,copied_bytes:copy,retained_capacity_bytes:capacity,..Default::default()};
            return Ok(ReplayStep::Pending(owner.replay.as_ref().expect("planned replay").progress()));
        }
        self.step_original_report_replay(owner,order,retirement_grant,control,maximum_work,deadline)
    }

}
