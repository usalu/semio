//! 🔁️ Store replacement admission and member drive: construction, refusal, member admission and sealing, the exact demands, and the member open, closure and candidate view drivers of the active store replacement. See [`super`].

use super::*;

impl<P, Mutation, M> ActiveArtifactStoreReplacement<P, Mutation, M>
where
    P: store::ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_schema_composition::ArtifactCompositionFields + ::semio_framework_os_kernel::StoreState,
    Mutation: ::semio_framework_os_kernel::StoreMutationFor<P>,
    M: SpaceMember + MemberFactory,
{
    pub fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, base_child_content_generation:u64,job:ArtifactStoreInitializationJob<P,Mutation>,source_identity:(semio_framework_job::OperationId,semio_framework_job::Generation))->Self {
        let cancel = semio_framework_job::CancelToken::root_now();
        let cancel_signal = job.cancel_signal();
        let session=None;
        let session_rejected=Some(semio_framework_plugin_ownership::reserved_job_session::RefusedWorkerSource::pending(job,source_identity));

        Self {
            operation,
            generation,
            base_child_content_generation,
            cancel: ManuallyDrop::new(Some(cancel)),
            cancel_signal: ManuallyDrop::new(Some(cancel_signal)),
            cancel_retirement: ManuallyDrop::new(None),
            session: ManuallyDrop::new(session),
            session_rejected: ManuallyDrop::new(session_rejected),
            outcome_pending: false,
            outcome_terminal: None,
            terminal_target: None,
            resume_pending: false,
            retained_store: ManuallyDrop::new(None),
            retained_disposer: ManuallyDrop::new(None),
            member_ingress: ManuallyDrop::new(None),
            active_member_ingress: ManuallyDrop::new(None),
            active_member_open: ManuallyDrop::new(None),
            member_open_admission: ManuallyDrop::new(None),
            opened_member: ManuallyDrop::new(None),
            candidate_children: ManuallyDrop::new(Some(ChildMemberRegistry::new())),
            candidate_content: ManuallyDrop::new(Some(ChildContentView::EMPTY)),
            candidate_content_retirement: ManuallyDrop::new(None),
            displaced_content_retirements: ManuallyDrop::new(None),
            displaced_content_retirement_cursor: 0,
            candidate_composition: ManuallyDrop::new(None),
            displaced_pins: ManuallyDrop::new(None),
            retiring_child: ManuallyDrop::new(None),
            retiring_child_cursor: 0,
            next_member_ordinal: 0,
            view_member_ordinal: 0,
            candidate_source_generation: 0,
            member_step_sequence: 0,
            expires_at_us: u64::MAX,
            closure: None,
            committed: false,
            refusal: None,
            fold_progress: (0, 0),
            state: ActiveArtifactStoreReplacementState::Initializing,
        }
    }

    /// 🔎️ The first leg's refusal as its typed fault, for a caller that must name why a stalled
    /// replacement never reached its target state.
    #[cfg(any(test, feature = "artifact-app-testing"))]
    pub fn refusal_fault(&self) -> Option<Fault> {
        self.refusal.map(ArtifactStoreReplacementRefusal::into_fault)
    }

    /// 🧭️ Records the FIRST leg that refused and moves the replacement onto its retirement path.
    /// The first one is kept because every later state transition is a consequence of it.
    pub fn refuse(&mut self, refusal: ArtifactStoreReplacementRefusal) {
        self.refusal.get_or_insert(refusal);
        self.state = ActiveArtifactStoreReplacementState::RetiringRejectedMembers;
    }

    /// 🧯️ Records a broken internal invariant and hands back that site's own unchanged fault, so
    /// the direct `Err` channel keeps its exact message while the archive's terminal report gains
    /// the leg that produced it.
    fn refuse_invariant(&mut self, leg: ArtifactStoreReplacementLeg, detail: &'static str) -> Fault {
        self.refuse(ArtifactStoreReplacementRefusal::Invariant { leg, detail });
        plugin_sdk_fault(detail)
    }

    pub fn request_cancel(&self) {
        if let Some(cancel) = self.cancel.as_ref() { cancel.cancel_now(); }
        if let Some(signal) = self.cancel_signal.as_ref() { signal.store(true, std::sync::atomic::Ordering::Release); }
    }

    /// 🧪️ Borrows the terminal retained store frame through its exact replacement authority.
    #[cfg(any(test, feature = "artifact-app-testing"))]
    pub fn terminal_store_frame_bytes(&self) -> Option<usize> {
        if !matches!(self.state, ActiveArtifactStoreReplacementState::RetiringRejectedCandidate | ActiveArtifactStoreReplacementState::RetiringCommittedStore) { return None; }
        self.retained_store.as_ref().filter(|owner| owner.close_owned_store_terminal_is_empty()).map(|owner| std::mem::size_of_val(owner.as_ref()))
    }

    pub fn begin_members(&mut self, expected: usize, expires_at_us: u64) -> Result<bool, Fault> {
        if self.state != ActiveArtifactStoreReplacementState::AwaitingMembers {
            return Ok(false);
        }
        if self.member_ingress.is_some() {
            return Err(plugin_sdk_fault("owned document member ingress was already initialized"));
        }
        *self.member_ingress = Some(OwnedDocumentMemberIngressRegistry::try_new(expected)?);
        self.expires_at_us = expires_at_us;
        Ok(true)
    }

    #[expect(clippy::result_large_err, reason = "Rejected admission returns the exact retained member request and identity owner unchanged.")]
    pub fn admit_member(&mut self, ingress: OwnedDocumentMemberIngress) -> Result<(), (Fault, OwnedDocumentMemberIngress)> {
        if self.state != ActiveArtifactStoreReplacementState::AwaitingMembers || ingress.operation() != self.operation || ingress.generation() != self.generation {
            return Err((plugin_sdk_fault("owned document member ingress does not match the active replacement authority"), ingress));
        }
        let Some(registry) = self.member_ingress.as_mut() else {
            return Err((plugin_sdk_fault("owned document member ingress registry has not been initialized"), ingress));
        };
        registry.admit(ingress)
    }

    pub fn seal_members(&mut self) -> Result<(), Fault> {
        if self.state != ActiveArtifactStoreReplacementState::AwaitingMembers {
            return Err(plugin_sdk_fault("owned document member ingress cannot seal outside its exact awaiting state"));
        }
        let registry = self.member_ingress.as_mut().ok_or_else(|| plugin_sdk_fault("owned document member ingress registry has not been initialized"))?;
        if let Err(fault) = registry.seal() {
            self.refuse(ArtifactStoreReplacementRefusal::Invariant { leg: ArtifactStoreReplacementLeg::Members, detail: "an owned-member or candidate-view authority returned its own exact refusal" });
            return Err(fault);
        }
        self.state = ActiveArtifactStoreReplacementState::OpeningMembers;
        Ok(())
    }

    fn member_open_birth_demand(&self) -> Result<store::RetainedCloneBirthDemand, store::MemberOpenDiagnostic> {
        if self.active_member_open.is_some() { return Ok(store::RetainedCloneBirthDemand { capacity_bytes: 0, depth: 0 }); }
        let ingress = if let Some(ingress) = self.active_member_ingress.as_ref() { Some(ingress) } else {
            self.member_ingress.as_ref().and_then(|registry| registry.entry(self.next_member_ordinal))
        };
        let Some(request) = self.member_open_admission.as_ref().and_then(store::MemberOpenSourceAdmission::request).or_else(||ingress.and_then(|ingress| ingress.request.as_ref())) else { return Ok(store::RetainedCloneBirthDemand { capacity_bytes: 0, depth: 0 }); };
        let demand = M::open_birth_demand(request)?;
        if demand.capacity_bytes > store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES || demand.depth > 64 { return Err(store::MemberOpenDiagnostic::Capacity); }
        Ok(demand)
    }

    fn member_ingress_release_bytes(&self) -> Result<usize, store::MemberOpenDiagnostic> {
        if self.active_member_open.is_some() || self.active_member_ingress.is_some() { return Ok(0); }
        let Some(registry) = self.member_ingress.as_ref() else { return Ok(0); };
        if self.next_member_ordinal != registry.expected { return Ok(0); }
        let bytes = registry.retirement_demands(0).map_err(|_|store::MemberOpenDiagnostic::Capacity)?.release_bytes;
        if bytes > store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES { return Err(store::MemberOpenDiagnostic::Capacity); }
        Ok(bytes)
    }

    /// 🌱️ The hydrated candidate parent while its member roster is still open — the only window in
    /// which a derivable child may still be minted onto the roster (`complete_document_archive_genesis`).
    pub fn candidate_awaiting_members(&self) -> Option<&ArtifactStore<P, Mutation>> {
        (self.state == ActiveArtifactStoreReplacementState::AwaitingMembers).then(|| self.retained_store.as_deref()).flatten()
    }

    pub fn cancelled(&self) -> bool {
        self.cancel.as_ref().is_some_and(semio_framework_job::CancelToken::is_cancelled_now)
    }

    /// 📏️ Quotes the exact next [`Self::drive_initializer`] turn.
    pub fn initializer_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        let item = RetirementDemand { depth: 1, ..Default::default() };
        let session_demand = |session: &semio_framework_job_worker::MountedWorkerJobSession<ArtifactStoreInitializationJob<P, Mutation>>| match session.retirement_demands(body) {
            Ok(demand) => Ok(demand),
            Err(semio_framework_job_worker::WorkerJobDemandError::Contention(_)) => Ok(item),
            Err(semio_framework_job_worker::WorkerJobDemandError::Refused(error)) => Err(error),
        };
        if let Some(rejected) = self.session_rejected.as_ref() {
            if rejected.terminal_is_empty() { return Ok(RetirementDemand { copy_bytes: size_of_val(rejected), ..item }); }
            return if rejected.admission_pending()&&!self.cancelled(){rejected.admission_demands()}else{rejected.retirement_demands(body)};
        }
        let Some(session) = self.session.as_ref() else { return Ok(item) };
        if self.outcome_pending { return session_demand(session); }
        if self.resume_pending { return Ok(RetirementDemand { copy_bytes: size_of::<bool>(), ..item }); }
        if self.terminal_target.is_some() {
            if session.terminal_is_empty() { return Ok(RetirementDemand { copy_bytes: size_of_val(session), ..item }); }
            return session_demand(session);
        }
        Ok(item)
    }

    /// 📏️ Quotes the exact next [`Self::drive_member_open`] turn.
    pub fn member_open_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        let item = RetirementDemand { depth: 1, ..Default::default() };
        let refused = |_| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "member open birth demand exceeds its fixed authority");
        if let Some(open)=self.active_member_open.as_ref(){return Ok(RetirementDemand{copy_bytes:store::MemberOpenOperation::next_copy_byte_demand(open)?,capacity_bytes:store::MemberOpenOperation::next_capacity_byte_demand(open,body)?,release_bytes:store::MemberOpenOperation::next_release_byte_demand(open)?,depth:store::MemberOpenOperation::next_depth_demand(open)?.max(1)})}
        if self.opened_member.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<M>(),depth:1,..Default::default()})}
        if self.active_member_ingress.is_none() {
            if let Some(registry) = self.member_ingress.as_ref() {
                if self.next_member_ordinal == registry.expected { return registry.retirement_demands(body); }
            }
        }
        let birth = self.member_open_birth_demand().map_err(refused)?;
        Ok(RetirementDemand { capacity_bytes: birth.capacity_bytes, depth: birth.depth.max(1), ..item })
    }

    pub fn closure_demands(&self) -> RetirementDemand {
        RetirementDemand { copy_bytes: store::MEMBER_OPEN_IDENTITY_BYTES, depth: 1, ..Default::default() }
    }

    pub fn candidate_views_demands(&self) -> RetirementDemand {
        RetirementDemand { copy_bytes: OWNED_DOCUMENT_MEMBER_PREPARATION_BYTES, depth: 1, ..Default::default() }
    }

}

impl<P, Mutation, M> ActiveArtifactStoreReplacement<P, Mutation, M>
where
    P: store::ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_schema_composition::ArtifactCompositionFields + ::semio_framework_os_kernel::StoreState,
    Mutation: ::semio_framework_os_kernel::StoreMutationFor<P>,
    M: SpaceMember + MemberFactory,
{
    pub fn drive_member_open(&mut self,cx:&mut semio_framework_job::StepContext<'_>, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        let idle = RetainedCloneProgress::default();
        let item = RetainedCloneProgress { copied_items: 1, ..idle };
        if grant.maximum_items == 0 { return Ok(PluginLifecycleStep::Progress(idle)); }
        if self.opened_member.is_some(){
            if let Some(open)=self.active_member_open.as_mut(){
                if store::MemberOpenOperation::terminal_is_empty(open){drop(self.active_member_open.take());return Ok(PluginLifecycleStep::Progress(item))}
                let parent=cx.original_source_parent()?;let before=cx.retained_progress();cx.with_retained_loan(grant,|same|store::MemberOpenOperation::close_step(open,parent,same))?;
                return Ok(PluginLifecycleStep::Progress(semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_delta(before,cx.retained_progress())?));
            }
            let member=self.opened_member.take().expect("ready member remains retained until its exact source loan returns");
            return self.publish_opened_member(member);
        }
        if self.active_member_open.is_some() {
            if self.active_member_ingress.as_ref().is_none_or(|ingress| !ingress.has_identity()) {
                return Err(self.refuse_invariant(ArtifactStoreReplacementLeg::Members, "active member open lost its exact retained ingress identity").into());
            }
            let before=cx.retained_progress();
            let mut admitted=None;cx.with_retained_loan(grant,|same|{admitted=Some(store::MemberOpenOperation::step(self.active_member_open.as_mut().expect("active member open remains retained"),same,grant)?);Ok(())})?;let step=admitted.expect("original member turn returned its admitted step");
            let after=cx.retained_progress();
            let delta=|end:usize,start:usize|end.checked_sub(start).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original member receipt moved before its initiating recipient"));
            let spent=RetainedCloneProgress{copied_items:delta(after.copied_items,before.copied_items)?,copied_bytes:delta(after.copied_bytes,before.copied_bytes)?,retained_capacity_bytes:delta(after.retained_capacity_bytes,before.retained_capacity_bytes)?,released_bytes:delta(after.released_bytes,before.released_bytes)?};
            return match step {
                store::MemberOpenStep::Pending(_) => Ok(PluginLifecycleStep::Progress(spent)),
                store::MemberOpenStep::Rejected(diagnostic) => {
                    let ordinal = self.next_member_ordinal;
                    self.refuse(if self.cancelled() { ArtifactStoreReplacementRefusal::Cancelled } else { ArtifactStoreReplacementRefusal::MemberOpenRejected { ordinal, diagnostic } });
                    self.state = ActiveArtifactStoreReplacementState::ClosingRejectedMember;
                    Ok(PluginLifecycleStep::Progress(spent))
                }
                store::MemberOpenStep::Ready(member) => {*self.opened_member=Some(member);Ok(PluginLifecycleStep::Progress(spent))}
            };
        }
        if self.active_member_ingress.is_none() {
            let Some(registry) = self.member_ingress.as_mut() else {
                return Err(self.refuse_invariant(ArtifactStoreReplacementLeg::Members, "opening member set lost its exact ingress registry").into());
            };
            if self.next_member_ordinal == registry.expected {
                if !registry.sealed || registry.len != 0 {
                    return Err(self.refuse_invariant(ArtifactStoreReplacementLeg::Members, "opened member set did not consume every sealed ordinal").into());
                }
                let step = registry.close_step(grant)?;
                if !matches!(step, RetainedCloneStep::Complete(_)) { return Ok(PluginLifecycleStep::Progress(step.progress())); }
                if !registry.terminal_is_empty() { return Err(self.refuse_invariant(ArtifactStoreReplacementLeg::Members, "opened member registry returned false terminal").into()); }
                let registry = self.member_ingress.take().expect("terminal member ingress registry remains retained");
                drop(registry);
                self.closure = Some(store::OwnedDocumentClosure::new(self.operation, self.generation, self.candidate_source_generation, self.expires_at_us));
                self.state = ActiveArtifactStoreReplacementState::ValidatingClosure;
                return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: step.progress().copied_items.max(1), ..step.progress() }));
            }
            let Some(ingress) = registry.take(self.next_member_ordinal) else {
                return Err(self.refuse_invariant(ArtifactStoreReplacementLeg::Members, "sealed member set lost an exact ordinal before open").into());
            };
            *self.active_member_ingress = Some(ingress);
        }
        if self.member_open_admission.is_none(){
            let request=self.active_member_ingress.as_mut().and_then(|ingress|ingress.request.take()).ok_or_else(||self.refuse_invariant(ArtifactStoreReplacementLeg::Members,"member ingress lost its exact retained open request"))?;
            *self.member_open_admission=Some(store::MemberOpenSourceAdmission::new(request));
            return Ok(PluginLifecycleStep::Progress(item));
        }
        let parent=cx.original_source_parent()?;
        let before=cx.retained_progress();
        let admission=self.member_open_admission.as_mut().expect("original member source remains retained through constructor admission");
        let _=M::begin_open(admission,&mut self.active_member_open,cx,parent)?;
        if admission.terminal_is_empty(){drop(self.member_open_admission.take());}
        Ok(PluginLifecycleStep::Progress(semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_delta(before,cx.retained_progress())?))
    }

    fn publish_opened_member(&mut self,member:M)->Result<PluginLifecycleStep,semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
                    let (ordinal, reference, owner) = self.active_member_ingress.as_mut().and_then(OwnedDocumentMemberIngress::take_identity).expect("active member identity was verified before its exact open step");
                    if ordinal != self.next_member_ordinal || member.artifact_ref().as_ref() != Some(&reference) || member.owner_ref().as_ref() != Some(&owner) {
                        *self.retiring_child = Some(ChildMemberRetirement::new(ChildMemberEntry { reference, owner, member }));
                        self.refuse(ArtifactStoreReplacementRefusal::MemberIdentityMismatch { ordinal, expected: self.next_member_ordinal });
                        return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}));
                    }
                    let Some(candidate) = self.candidate_children.as_mut() else {
                        *self.retiring_child = Some(ChildMemberRetirement::new(ChildMemberEntry { reference, owner, member }));
                        return Err(self.refuse_invariant(ArtifactStoreReplacementLeg::Members, "member candidate registry was lost before exact admission").into());
                    };
                    let owner_key = if self.retained_store.as_ref().is_some_and(|parent| parent.envelope().id == owner.parent.artifact_id) { "" } else { owner.parent.artifact_id.as_str() };
                    let admission = match candidate.admit_identity(MemberKeyRef { owner: owner_key, slot: &owner.slot, child_id: &owner.child_id }) {
                        Ok(admission) => admission,
                        Err(fault) => {
                            *self.retiring_child = Some(ChildMemberRetirement::new(ChildMemberEntry { reference, owner, member }));
                            self.refuse(ArtifactStoreReplacementRefusal::Invariant { leg: ArtifactStoreReplacementLeg::Members, detail: "an owned-member or candidate-view authority returned its own exact refusal" });
                            return Err(fault.into());
                        }
                    };
                    let Some(candidate_source_generation) = self.candidate_source_generation.checked_add(1) else {
                        let cancelled = candidate.cancel_admission(&admission);
                        debug_assert!(cancelled, "candidate admission remains exclusive until insertion");
                        *self.retiring_child = Some(ChildMemberRetirement::new(ChildMemberEntry { reference, owner, member }));
                        return Err(self.refuse_invariant(ArtifactStoreReplacementLeg::Members, "owned document candidate generation exhausted").into());
                    };
                    candidate.insert_admitted(admission, reference, owner, member);
                    let ingress = self.active_member_ingress.take().expect("ready member ingress remains retained");
                    if !ingress.terminal_is_empty() {
                        return Err(plugin_sdk_fault("ready member ingress retained ownership after exact handoff").into());
                    }
                    drop(ingress);
                    self.next_member_ordinal += 1;
                    self.candidate_source_generation = candidate_source_generation;
                    Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))
                    }

    pub fn drive_closure(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
        if grant.maximum_items == 0 || grant.maximum_copy_bytes < store::MEMBER_OPEN_IDENTITY_BYTES {
            return Ok(PluginLifecycleStep::Progress(Default::default()));
        }
        let Some(candidate) = self.retained_store.as_ref() else {
            return Err(self.refuse_invariant(ArtifactStoreReplacementLeg::Closure, "closure validation lost its retained parent candidate"));
        };
        let Some(dialect) = candidate.envelope().dialect.clone() else {
            return Err(self.refuse_invariant(ArtifactStoreReplacementLeg::Closure, "closure validation requires the candidate parent's exact dialect"));
        };
        let root_reference = ArtifactRef { artifact_id: candidate.envelope().id.clone(), dialect };
        let Some(children) = self.candidate_children.as_ref() else {
            return Err(self.refuse_invariant(ArtifactStoreReplacementLeg::Closure, "closure validation lost its retained member registry"));
        };
        let source = DocumentClosureSourceView::new(self.candidate_source_generation, &root_reference, candidate.snapshot_ref(), children);
        let mut sequence = self.member_step_sequence;
        let mut receipt = RetainedCloneProgress::default();
        let Some(closure) = self.closure.as_mut() else {
            return Err(self.refuse_invariant(ArtifactStoreReplacementLeg::Closure, "closure validation lost its fixed cursor"));
        };
        let step = {
            let mut cx = semio_framework_job::StepContext::new(self.operation, self.generation, semio_framework_job::StepBudget::new(grant.maximum_items as u64, u64::MAX, grant), self.cancel.as_ref().expect("live replacement cancellation root").clone(), semio_framework_job::default_now_us, &mut sequence, &mut receipt);
            closure.step(&source, &mut cx)
        };
        let children_len = children.len();
        self.member_step_sequence = sequence;
        let spent = RetainedCloneProgress { copied_items: receipt.copied_items.max(1), ..receipt };
        match step {
            store::OwnedDocumentClosureStep::Pending(_) => Ok(PluginLifecycleStep::Progress(spent)),
            store::OwnedDocumentClosureStep::Complete { members } if members == children_len => {
                self.closure = None;
                self.state = ActiveArtifactStoreReplacementState::PreparingCandidateViews;
                Ok(PluginLifecycleStep::Progress(spent))
            }
            store::OwnedDocumentClosureStep::Complete { members } => {
                self.closure = None;
                self.refuse(ArtifactStoreReplacementRefusal::ClosureMemberCount { members, children: children_len });
                Ok(PluginLifecycleStep::Progress(spent))
            }
            store::OwnedDocumentClosureStep::Rejected(diagnostic) => {
                self.closure = None;
                self.refuse(ArtifactStoreReplacementRefusal::ClosureRejected { diagnostic });
                Ok(PluginLifecycleStep::Progress(spent))
            }
        }
    }

    pub fn drive_candidate_views(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
        if grant.maximum_items == 0 || grant.maximum_copy_bytes < OWNED_DOCUMENT_MEMBER_PREPARATION_BYTES {
            return Ok(PluginLifecycleStep::Progress(Default::default()));
        }
        let item = PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() });
        let Some(children) = self.candidate_children.as_ref() else {
            return Err(self.refuse_invariant(ArtifactStoreReplacementLeg::CandidateViews, "candidate view preparation lost its exact member registry"));
        };
        if self.candidate_source_generation != children.len() as u64 {
            return Err(self.refuse_invariant(ArtifactStoreReplacementLeg::CandidateViews, "candidate member generation changed after closure validation"));
        }
        if self.candidate_composition.is_none() {
            let composition = match CompositionCoordinator::try_for_owned_closure(children.len()) {
                Ok(composition) => composition,
                Err(error) => {
                    self.refuse(ArtifactStoreReplacementRefusal::Invariant { leg: ArtifactStoreReplacementLeg::CandidateViews, detail: "an app-owned candidate-view owner refused its exact step" });
                    return Err(plugin_sdk_fault(error));
                }
            };
            *self.candidate_composition = Some(composition);
            return Ok(item);
        }
        if self.view_member_ordinal == children.len() {
            self.state = ActiveArtifactStoreReplacementState::CandidateReady;
            return Ok(item);
        }
        let Some((key, entry)) = children.keyed_entry_by_ordinal(self.view_member_ordinal) else {
            return Err(self.refuse_invariant(ArtifactStoreReplacementLeg::CandidateViews, "candidate view preparation lost its exact member ordinal"));
        };
        if entry.member.artifact_ref().as_ref() != Some(&entry.reference) || entry.member.owner_ref().as_ref() != Some(&entry.owner) {
            return Err(self.refuse_invariant(ArtifactStoreReplacementLeg::CandidateViews, "candidate member identity changed after closure validation"));
        }
        let Some(content) = self.candidate_content.as_ref() else {
            return Err(self.refuse_invariant(ArtifactStoreReplacementLeg::CandidateViews, "candidate view preparation lost its immutable content root"));
        };
        let index = match content.admit_member(key) {
            Ok(index) => index,
            Err(fault) => {
                self.refuse(ArtifactStoreReplacementRefusal::Invariant { leg: ArtifactStoreReplacementLeg::CandidateViews, detail: "an owned-member or candidate-view authority returned its own exact refusal" });
                return Err(fault);
            }
        };
        let Some(composition) = self.candidate_composition.as_mut() else {
            return Err(self.refuse_invariant(ArtifactStoreReplacementLeg::CandidateViews, "candidate view preparation lost its coordinator"));
        };
        if let Err(error) = composition.insert_validated_owned(&entry.owner.parent.artifact_id, &entry.owner.slot, &entry.reference.artifact_id) {
            self.refuse(ArtifactStoreReplacementRefusal::Invariant { leg: ArtifactStoreReplacementLeg::CandidateViews, detail: "an app-owned candidate-view owner refused its exact step" });
            return Err(plugin_sdk_fault(error));
        }
        let revision = entry.member.content_revision_now();
        let snapshot = match entry.member.snapshot_read_erased_now() {
            Ok(snapshot) => snapshot,
            Err(error) => {
                self.refuse(ArtifactStoreReplacementRefusal::Invariant { leg: ArtifactStoreReplacementLeg::CandidateViews, detail: "an app-owned candidate-view owner refused its exact step" });
                return Err(plugin_sdk_fault(error));
            }
        };
        let prior = self.candidate_content.take().expect("candidate content owner was verified before snapshot-read issuance");
        *self.candidate_content = Some(prior.insert_entry_admitted(index, ChildContentEntry { owner: key.owner.to_string(), slot: entry.owner.slot.clone(), child_id: entry.owner.child_id.clone(), artifact_id: entry.reference.artifact_id.clone(), dialect: entry.reference.dialect.clone(), revision, snapshot }));
        self.view_member_ordinal += 1;
        Ok(item)
    }

    /// 📐️ Quotes one original child frontier or its separately retained parent metadata.
    pub(super) fn replacement_member_retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        let parent = |mut demand: RetirementDemand| -> Result<RetirementDemand, ValueError> { demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "replacement original parent depth overflowed"))?; Ok(demand) };
        let header = |bytes| RetirementDemand { copy_bytes: bytes, depth: 1, ..Default::default() };
        if self.closure.is_some() { return Ok(header(size_of_val(&self.closure))); }
        if let Some(admission)=self.member_open_admission.as_ref(){return if admission.terminal_is_empty(){Ok(header(size_of_val(&*self.member_open_admission)))}else{parent(admission.retirement_demands(body)?)}}
        if self.opened_member.is_some()&&self.active_member_open.is_none(){return Ok(header(size_of_val(&*self.opened_member)+size_of::<ChildMemberRetirement<M>>()))}
        if let Some(open) = self.active_member_open.as_ref() {
            return if store::MemberOpenOperation::terminal_is_empty(open) { Ok(header(size_of_val(&*self.active_member_open))) } else { parent(RetirementDemand { copy_bytes: store::MemberOpenOperation::next_copy_byte_demand(open)?, capacity_bytes: store::MemberOpenOperation::next_capacity_byte_demand(open,body)?, release_bytes: store::MemberOpenOperation::next_release_byte_demand(open)?, depth: store::MemberOpenOperation::next_depth_demand(open)? }) };
        }
        if let Some(ingress) = self.active_member_ingress.as_ref() { return if ingress.terminal_is_empty() { Ok(header(size_of_val(&*self.active_member_ingress))) } else { parent(ingress.retirement_demands(body)?) }; }
        if let Some(registry) = self.member_ingress.as_ref() { return if registry.terminal_is_empty() { Ok(header(size_of_val(&*self.member_ingress))) } else { parent(registry.retirement_demands(body)?) }; }
        if let Some(registry) = self.displaced_content_retirements.as_ref() {
            let children = self.candidate_children.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "replacement original root registry lost its member owners"))?;
            return match original_child_root_retirement_demands::<M>(registry,children,None,self.displaced_content_retirement_cursor,body,true)? { Some(demand) => parent(demand), None => Ok(header(size_of_val(&*self.displaced_content_retirements))) };
        }
        if let Some(retirement) = self.candidate_content_retirement.as_ref() {
            if retirement.terminal_is_empty() { return Ok(header(size_of_val(&*self.candidate_content_retirement))); }
            let children = self.candidate_children.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "replacement original candidate root lost its member owners"))?;
            let reserved = ChildContentBorrowedOwners::copy_reservation();
            let mut demand = retirement.retirement_demands(children,None,body.saturating_sub(reserved))?;
            demand.copy_bytes = demand.copy_bytes.checked_add(reserved).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "replacement borrowed retainer extent overflowed"))?;
            return parent(demand);
        }
        if self.candidate_content.is_some() { return Ok(header(size_of_val(&*self.candidate_content) + size_of_val(&*self.candidate_content_retirement) + size_of::<ChildContentRetirement>())); }
        if let Some(composition) = self.candidate_composition.as_ref() {
            return if composition.terminal_is_empty() { Ok(header(size_of_val(&*self.candidate_composition))) } else { parent(RetirementDemand { copy_bytes: composition.next_close_copy_byte_demand()?, capacity_bytes: composition.next_close_capacity_byte_demand(body)?, release_bytes: composition.next_close_release_byte_demand()?, depth: composition.next_close_depth_demand()? }) };
        }
        if let Some(pins) = self.displaced_pins.as_ref() { return if pins.terminal_is_empty() { Ok(header(size_of_val(&*self.displaced_pins))) } else { parent(pins.retirement_demands()?) }; }
        if let Some(retirement) = self.retiring_child.as_ref() { return if retirement.terminal_is_empty() { Ok(header(size_of_val(&*self.retiring_child))) } else { parent(retirement.retirement_demands(body)?) }; }
        if let Some(children) = self.candidate_children.as_ref() {
            if children.len() != 0 { return Ok(header(size_of::<ChildMemberEntry<M>>() + size_of_val(&*self.retiring_child) + size_of::<ChildMemberRetirement<M>>() + 2*size_of::<u64>() + 6*size_of::<usize>())); }
            if !children.slots.is_empty() { return Ok(RetirementDemand { copy_bytes: size_of_val(&children.slots) + size_of::<bool>(), release_bytes: size_of_val(children.slots.as_ref()), depth: 1, ..Default::default() }); }
            return Ok(header(size_of_val(&*self.candidate_children)));
        }
        if self.cancel.is_some() { return Ok(header(size_of_val(&*self.cancel) + size_of_val(&*self.cancel_retirement) + size_of::<semio_framework_async::CancelTokenRetirement>())); }
        if let Some(cancel) = self.cancel_retirement.as_ref() { return if cancel.terminal_is_empty() { Ok(header(size_of_val(&*self.cancel_retirement))) } else { parent(cancel.retirement_demands()?) }; }
        if self.cancel_signal.is_some() { return Ok(RetirementDemand { copy_bytes: size_of_val(&*self.cancel_signal) + size_of::<std::sync::atomic::AtomicBool>(), release_bytes: semio_framework_value::shared_retirement_allocation_bytes::<std::sync::atomic::AtomicBool>(), depth: 1, ..Default::default() }); }
        Ok(header(size_of::<ActiveArtifactStoreReplacementState>()))
    }

}
