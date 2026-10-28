//! 🧰️ Store replacement initializer and retirement: the rejected member driver, the initializer driver, the retirement demands, the retained store driver, the drop guard, the admission target and the app projection cache. See [`super`].

use super::*;

impl<P, Mutation, M> ActiveArtifactStoreReplacement<P, Mutation, M>
where
    P: semio_framework_value::ToValue + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,
    Mutation: semio_framework_value::ToValue + semio_framework_value::FromValue + store::Mutation<P> + OpBinary + OpText + Send + 'static,
    M: SpaceMember + MemberFactory,
{
    /// 🧹️ Keeps original custody until every child and separate parent transfer is admitted.
    pub fn drive_rejected_members(&mut self, current_content: &ChildContentView,cx:&mut semio_framework_job::StepContext<'_>, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        let demand = self.replacement_member_retirement_demands(grant.maximum_copy_bytes).map_err(semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure::from)?;
        if grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes || grant.maximum_depth < demand.depth { return Ok(PluginLifecycleStep::Progress(Default::default())); }
        let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
        let paid = || PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes: demand.release_bytes, ..Default::default() });
        let parent = |step: RetainedCloneStep, terminal: bool| -> Result<PluginLifecycleStep, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
            let admitted = semio_framework_value::retained_clone::admit_retained_clone_close(child,step,terminal,"replacement original member child").map_err(semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure::from)?;
            Ok(PluginLifecycleStep::Progress(admitted.progress()))
        };
        if self.closure.is_some() { drop(self.closure.take()); return Ok(paid()); }
        if let Some(admission)=self.member_open_admission.as_mut(){
            if admission.terminal_is_empty(){drop(self.member_open_admission.take());return Ok(paid())}
            let original=cx.original_source_parent()?;let step=cx.with_retained_loan(child,|same|admission.close_step(original,same))?;return parent(step,admission.terminal_is_empty());
        }
        if let Some(open) = self.active_member_open.as_mut() {
            if store::MemberOpenOperation::terminal_is_empty(open) { drop(self.active_member_open.take()); return Ok(paid()); }
            let step=cx.with_retained_loan(child,|same|store::MemberOpenOperation::close_step(open,same.original_source_parent()?,same))?;
            return parent(step,store::MemberOpenOperation::terminal_is_empty(open));
        }
        if self.opened_member.is_some(){
            let (_,reference,owner)=self.active_member_ingress.as_mut().and_then(OwnedDocumentMemberIngress::take_identity).ok_or_else(||plugin_sdk_fault("ready rejected member lost its exact ingress identity"))?;
            let member=self.opened_member.take().expect("original ready member remains retained until its identity is admitted");
            *self.retiring_child=Some(ChildMemberRetirement::new(ChildMemberEntry{reference,owner,member}));return Ok(paid());
        }
        if let Some(ingress) = self.active_member_ingress.as_mut() {
            if ingress.terminal_is_empty() { drop(self.active_member_ingress.take()); return Ok(paid()); }
            let step = ingress.close_step(child).map_err(semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure::from)?; return parent(step,ingress.terminal_is_empty());
        }
        if let Some(registry) = self.member_ingress.as_mut() {
            if registry.terminal_is_empty() { drop(self.member_ingress.take()); return Ok(paid()); }
            let step = registry.close_step(child)?; return parent(step,registry.terminal_is_empty());
        }
        if let Some(registry) = self.displaced_content_retirements.as_mut() {
            let children = self.candidate_children.as_mut().ok_or_else(|| plugin_sdk_fault("replacement original displaced roots lost their member registry"))?;
            let current = self.candidate_content.as_ref().ok_or_else(|| plugin_sdk_fault("replacement original displaced roots lost their current root"))?;
            if original_child_root_retirement_demands::<M>(registry,children,None,self.displaced_content_retirement_cursor,child.maximum_copy_bytes,true).map_err(semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure::from)?.is_none() { drop(self.displaced_content_retirements.take()); return Ok(paid()); }
            let step = advance_original_child_root_retirement(registry,children,None,current,None,None,&mut self.displaced_content_retirement_cursor,child,true)?;
            return parent(step,registry.is_empty() && registry.slots.is_empty());
        }
        if let Some(retirement) = self.candidate_content_retirement.as_mut() {
            if retirement.terminal_is_empty() { drop(self.candidate_content_retirement.take()); return Ok(paid()); }
            let children = self.candidate_children.as_mut().ok_or_else(|| plugin_sdk_fault("replacement original candidate root lost its member registry"))?;
            let reserved = ChildContentBorrowedOwners::copy_reservation();
            let inner = RetainedCloneGrant { maximum_copy_bytes: child.maximum_copy_bytes - reserved, ..child };
            let owners = ChildContentBorrowedOwners::none();
            let step = retirement.close_step(children,None,current_content,&owners,inner)?;
            let step = semio_framework_value::retained_clone::admit_retained_clone_close(inner,step,retirement.terminal_is_empty(),"replacement original candidate root").map_err(semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure::from)?;
            let receipt = step.progress();
            return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_bytes: receipt.copied_bytes + reserved, ..receipt }));
        }
        if self.candidate_content.is_some() { let content = self.candidate_content.take().expect("original candidate root remains retained"); *self.candidate_content_retirement = Some(ChildContentRetirement::new(content,true)); return Ok(paid()); }
        if let Some(composition) = self.candidate_composition.as_mut() {
            if composition.terminal_is_empty() { drop(self.candidate_composition.take()); return Ok(paid()); }
            let step = composition.close_step(child).map_err(semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure::from)?; return parent(step,composition.terminal_is_empty());
        }
        if let Some(pins) = self.displaced_pins.as_mut() {
            if pins.terminal_is_empty() { drop(self.displaced_pins.take()); return Ok(paid()); }
            let step = pins.close_step(child)?; return parent(step,pins.terminal_is_empty());
        }
        if let Some(retirement) = self.retiring_child.as_mut() {
            if retirement.terminal_is_empty() { drop(self.retiring_child.take()); return Ok(paid()); }
            let step = retirement.close_step(child)?; return parent(step,retirement.terminal_is_empty());
        }
        if let Some(children) = self.candidate_children.as_mut() {
            if children.has_admission_in_flight() { return Ok(PluginLifecycleStep::AwaitingInput { reason: "replacement original member reservation awaits its admitted owner" }); }
            if children.len() != 0 {
                let index = children.dense_slots[children.len()-1];
                let entry = children.take_at(index).expect("original dense replacement member remains occupied");
                *self.retiring_child = Some(ChildMemberRetirement::new(entry));
                self.retiring_child_cursor = index;
                return Ok(paid());
            }
            if !children.slots.is_empty() { drop(std::mem::take(&mut children.slots)); children.allocation_admitted = false; return Ok(paid()); }
            drop(self.candidate_children.take()); return Ok(paid());
        }
        if self.cancel.is_some() { let original = self.cancel.take().expect("original replacement cancellation remains retained"); *self.cancel_retirement = Some(semio_framework_async::CancelTokenRetirement::from_token(original)); return Ok(paid()); }
        if let Some(cancel) = self.cancel_retirement.as_mut() {
            if cancel.terminal_is_empty() { drop(self.cancel_retirement.take()); return Ok(paid()); }
            return match cancel.close_step(child) {
                Ok(step) => parent(step,cancel.terminal_is_empty()),
                Err(semio_framework_async::CancelTokenRetirementError::Blocked(_)) => Ok(PluginLifecycleStep::AwaitingInput { reason: "replacement original cancellation retains its waiter or alias" }),
                Err(semio_framework_async::CancelTokenRetirementError::Refused(error)) => Err(semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure::from(error)),
            };
        }
        if let Some(original) = self.cancel_signal.as_ref() {
            if std::sync::Arc::strong_count(original) != 1 || std::sync::Arc::weak_count(original) != 0 { return Ok(PluginLifecycleStep::AwaitingInput { reason: "replacement original cancellation signal retains an external owner" }); }
            return match std::sync::Arc::try_unwrap(self.cancel_signal.take().unwrap()) {
                Ok(original) => { drop(original); Ok(paid()) },
                Err(original) => { *self.cancel_signal = Some(original); Ok(PluginLifecycleStep::AwaitingInput { reason: "replacement original cancellation signal acquired an alias" }) },
            };
        }
        self.state = if self.retained_store.is_some() { if self.committed { ActiveArtifactStoreReplacementState::RetiringCommittedStore } else { ActiveArtifactStoreReplacementState::RetiringRejectedCandidate } } else { ActiveArtifactStoreReplacementState::Complete };
        Ok(paid())
    }

    pub fn drive_initializer(&mut self,cx:&mut semio_framework_job::StepContext<'_>, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        if self.state!=ActiveArtifactStoreReplacementState::Initializing{return Ok(PluginLifecycleStep::AwaitingInput{reason:"store initializer awaits its actual replacement stage"});}
        if grant.maximum_items==0||grant.maximum_depth==0{return Ok(PluginLifecycleStep::Progress(Default::default()));}
        let business_cancelled=self.cancelled();
        if let Some(rejected)=self.session_rejected.as_mut(){
            if rejected.admission_pending()&&!cx.is_cancelled()&&!business_cancelled{
                let config=semio_framework_job_worker_authority::BatchDriveConfig{retained:grant,site:"artifact_store_replacement",stage:semio_framework_job::InteractiveStage::InteractiveStep,fuel_per_step:cx.fuel_remaining(),step_budget_us:0};
                let before=cx.retained_progress();
                if let Some(session)=rejected.admit_pending(config,cx)?{*self.session=Some(session);drop(self.session_rejected.take());}
                return Ok(PluginLifecycleStep::Progress(semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_delta(before,cx.retained_progress())?));
            }

            if rejected.terminal_is_empty(){
                let bytes=size_of_val(rejected);if grant.maximum_copy_bytes<bytes{return Ok(PluginLifecycleStep::Progress(Default::default()));}
                drop(self.session_rejected.take());self.refuse(if cx.is_cancelled()||business_cancelled{ArtifactStoreReplacementRefusal::Cancelled}else{ArtifactStoreReplacementRefusal::InitializerAdmissionRejected});
                return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}));
            }
            let demand=rejected.retirement_demands(grant.maximum_copy_bytes)?;
            return original_rejection_lifecycle(cx,demand,grant,"store initializer rejection awaits its original worker",|cx|{let step=rejected.close_step(cx)?;Ok((step,rejected.terminal_is_empty()))});
        }
        if self.outcome_pending{
            let session=self.session.as_mut().ok_or_else(||plugin_sdk_fault("store initializer original outcome acknowledgement lost its session"))?;
            let step=session.acknowledge_checked_out_outcome(grant);
            if matches!(step,RetainedCloneStep::Complete(_)){
                self.outcome_pending=false;
                if let Some(progress) = session.checked_out_job_mut().map(|job| job.progress()).filter(|(_, total)| *total > 0) {
                    self.fold_progress = progress;
                }
                match self.outcome_terminal.take() {
                    Some((1, _)) => {
                        let candidate = session
                            .checked_out_job_mut()
                            .and_then(ArtifactStoreInitializationJob::take_candidate)
                            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-store.initializer-candidate-missing"), "completed store initializer lost its exact candidate"))?;
                        *self.retained_store = Some(candidate);
                        self.terminal_target = Some(ActiveArtifactStoreReplacementState::AwaitingMembers);
                    }
                    Some((2, initializer_code)) => {
                        let accepted = session.checked_out_job_mut().is_some_and(ArtifactStoreInitializationJob::accept_terminal_failure);
                        if !accepted {
                            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-store.initializer-failure-retirement"), "failed store initializer did not reach terminal-empty authority").into());
                        }
                        self.refusal.get_or_insert(ArtifactStoreReplacementRefusal::InitializerFailed(initializer_code));
                        self.terminal_target = Some(ActiveArtifactStoreReplacementState::RetiringRejectedMembers);
                    }
                    _ => {}
                }
                self.resume_pending=true;
            }
            return Ok(PluginLifecycleStep::retained(step,false));
        }
        if self.resume_pending{
            if grant.maximum_copy_bytes<size_of::<bool>(){return Ok(PluginLifecycleStep::Progress(Default::default()));}
            let session=self.session.as_mut().ok_or_else(||plugin_sdk_fault("store initializer original outcome handback lost its session"))?;
            if self.terminal_target.is_some(){session.begin_close();}else if session.resume().is_err(){return Ok(PluginLifecycleStep::AwaitingInput{reason:"store initializer original outcome resume remains contended"});}
            self.resume_pending=false;
            return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<bool>(),..Default::default()}));
        }
        if let Some(target)=self.terminal_target{
            let session=self.session.as_mut().ok_or_else(||plugin_sdk_fault("store initializer terminal target lost its original session"))?;
            if session.terminal_is_empty(){
                let bytes=size_of_val(session);if grant.maximum_copy_bytes<bytes{return Ok(PluginLifecycleStep::Progress(Default::default()));}
                drop(self.session.take());self.terminal_target=None;self.state=target;
                return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}));
            }
            let demand=match session.retirement_demands(grant.maximum_copy_bytes){Ok(demand)=>demand,Err(semio_framework_job_worker::WorkerJobDemandError::Contention(_))=>return Ok(PluginLifecycleStep::AwaitingInput{reason:"store initializer original quote awaits worker handback"}),Err(semio_framework_job_worker::WorkerJobDemandError::Refused(error))=>return Err(error.into())};
            return original_worker_lifecycle(cx,demand,grant,"store initializer original terminal worker remains contended",|cx|{let step=session.close_step(cx)?;Ok((step,session.terminal_is_empty()))});
        }
        let session = self.session.as_mut().ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-store.initializer-session-missing"), "store initializer lost its retained session before terminal recovery"))?;
        match session.pump_one_for_interactive_turn(cx) {
            Ok(semio_framework_job_worker::WorkerJobPoll::Outcome | semio_framework_job_worker::WorkerJobPoll::Terminal) => {
                let Some((issued, progress)) = session.take_checked_out_retained_step_receipt() else { return Ok(PluginLifecycleStep::Blocked { reason: "store initializer original receipt awaits its sole recipient" }) };
                if !progress.fits(issued) || !progress.fits(grant) {
                    return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-store.initializer-receipt"), "store initializer original receipt exceeded its received turn").with_retained_progress(progress).into());
                }
                self.outcome_terminal = match session.checked_out_outcome() {
                    Ok(Some(semio_framework_job::JobOutcomeView::Complete { .. })) => Some((1, ArtifactStoreRefusalCode::EMPTY)),
                    Ok(Some(semio_framework_job::JobOutcomeView::Cancelled { .. })) => Some((2, ArtifactStoreRefusalCode::from_page(b"initializer-cancelled"))),
                    Ok(Some(semio_framework_job::JobOutcomeView::Fault { detail, .. })) => Some((2, ArtifactStoreRefusalCode::from_page(detail.page(0).unwrap_or(b"initializer-faulted-without-detail")))),
                    Ok(Some(_)) | Ok(None) => None,
                    Err(_) => return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-store.initializer-outcome-missing"), "store initializer checkout lost its exact outcome").into()),
                };
                self.outcome_pending = true;
                Ok(PluginLifecycleStep::Progress(progress))
            }
            Ok(semio_framework_job_worker::WorkerJobPoll::Submitted | semio_framework_job_worker::WorkerJobPoll::Idle) => Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),
            Ok(_) => Ok(PluginLifecycleStep::AwaitingInput { reason: "store initializer awaits its mounted wake" }),
            Err(failure) => Err(failure.into()),
        }
    }

    /// 📏️ Quotes the exact next retirement turn of the current retiring state: one member owner, or the retained Store and its disposer.
    pub fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        let item = RetirementDemand { depth: 1, ..Default::default() };
        if matches!(self.state, ActiveArtifactStoreReplacementState::RetiringRejectedMembers | ActiveArtifactStoreReplacementState::RetiringCommittedMembers) {
            return self.replacement_member_retirement_demands(body);
        }
        let Some(store) = self.retained_store.as_ref() else { return Ok(item) };
        let Some(disposer) = self.retained_disposer.as_ref() else {
            return Ok(RetirementDemand { copy_bytes: size_of_val(store) + size_of::<ActiveArtifactStoreReplacementState>(), release_bytes: size_of_val(&**store), ..item });
        };
        if disposer.terminal_is_empty(store) { return Ok(RetirementDemand { copy_bytes: size_of_val(disposer), release_bytes: size_of_val(&**disposer), ..item }); }
        disposer.retirement_demands(store, body)
    }

    pub fn drive_retained_store(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
        if grant.maximum_items==0||grant.maximum_depth==0{return Ok(PluginLifecycleStep::Progress(Default::default()));}
        let Some(store)=self.retained_store.as_mut()else{return Ok(PluginLifecycleStep::AwaitingInput{reason:"replacement original cancellation and closure owners retain their physical authority"});};
        let Some(disposer)=self.retained_disposer.as_mut()else{
            if !store.close_owned_store_terminal_is_empty(){return Err(plugin_sdk_fault("retained replacement Store lost its exact granted disposer"));}
            let demand=RetirementDemand{copy_bytes:size_of_val(store)+size_of::<ActiveArtifactStoreReplacementState>(),release_bytes:size_of_val(&**store),depth:1,..Default::default()};
            if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(PluginLifecycleStep::Progress(Default::default()));}
            return semio_framework_plugin_ownership::bounded_ownership::runtime_lifecycle_step(demand,grant,|_|{
                drop(self.retained_store.take());self.state=if self.member_owners_terminal_is_empty(){ActiveArtifactStoreReplacementState::Complete}else{ActiveArtifactStoreReplacementState::RetiringRejectedMembers};
                Ok((PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}),false))
            });
        };
        if disposer.terminal_is_empty(store){
            let demand=RetirementDemand{copy_bytes:size_of_val(disposer),release_bytes:size_of_val(&**disposer),depth:1,..Default::default()};
            if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(PluginLifecycleStep::Progress(Default::default()));}
            return semio_framework_plugin_ownership::bounded_ownership::runtime_lifecycle_step(demand,grant,|_|{
                drop(self.retained_disposer.take());
                Ok((PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}),false))
            });
        }
        let demand=disposer.retirement_demands(store,grant.maximum_copy_bytes).map_err(|error|plugin_sdk_fault(error.to_string()))?;
        semio_framework_plugin_ownership::bounded_ownership::runtime_lifecycle_step(demand,grant,|grant|{
            let step=disposer.close_step(store,grant)?;
            if matches!(step,PluginLifecycleStep::Complete(_))&&!disposer.terminal_is_empty(store){return Err(plugin_sdk_fault("replacement original disposer completed with retained Store owners"));}
            Ok((match step{PluginLifecycleStep::Complete(progress)=>PluginLifecycleStep::Progress(progress),step=>step},false))
        })
    }

    fn member_owners_terminal_is_empty(&self) -> bool {
        self.closure.is_none()
            && self.cancel.is_none()
            && self.cancel_signal.is_none()
            && self.cancel_retirement.is_none()
            && self.member_ingress.is_none()
            && self.active_member_ingress.is_none()
            && self.active_member_open.is_none()
            && self.member_open_admission.is_none()
            && self.opened_member.is_none()
            && self.candidate_children.is_none()
            && self.candidate_content.is_none()
            && self.candidate_content_retirement.is_none()
            && self.displaced_content_retirements.is_none()
            && self.candidate_composition.is_none()
            && self.displaced_pins.is_none()
            && self.retiring_child.is_none()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.state == ActiveArtifactStoreReplacementState::Complete && self.member_owners_terminal_is_empty() && self.session.is_none() && self.session_rejected.is_none() && !self.outcome_pending && self.retained_store.is_none() && self.retained_disposer.is_none()
    }
}

impl<P, Mutation, M> Drop for ActiveArtifactStoreReplacement<P, Mutation, M>
where
    P: semio_framework_value::ToValue + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,
    Mutation: semio_framework_value::ToValue + semio_framework_value::FromValue + store::Mutation<P> + OpBinary + OpText + Send + 'static,
    M: SpaceMember + MemberFactory,
{
    fn drop(&mut self) {
        assert!(
            std::thread::panicking() || self.terminal_is_empty(),
            "artifact store replacement reached Drop before initializer/candidate/displaced-store ownership was terminal empty"
        );
    }
}

#[doc(hidden)]
pub struct ArtifactStoreReplacementAdmissionTarget<'a, A: ArtifactApp, M: SpaceMember + MemberFactory> {
    pub jobs: &'a mut ArtifactFixedRegistry<ActiveArtifactStoreReplacement<A::Snapshot, A::Mutation, M>>,
    pub operation: semio_framework_job::OperationId,
    pub generation: semio_framework_job::Generation,
    pub child_content_generation: u64,
    pub source_identity:(semio_framework_job::OperationId,semio_framework_job::Generation),
    pub actor: ActorId,
    pub marker: std::marker::PhantomData<fn() -> A>,
}

impl<A: ArtifactApp, M: SpaceMember + MemberFactory> store::os_store::owned_schema_decode::ArtifactEnvelopeCompletedRecordTarget<A::Snapshot, A::Mutation> for ArtifactStoreReplacementAdmissionTarget<'_, A, M> {
    fn try_adopt_completed(&mut self, mut envelope: ArtifactEnvelope<A::Snapshot, A::Mutation>) -> Result<(), ArtifactEnvelope<A::Snapshot, A::Mutation>> {
        if envelope.dialect.is_none() {
            envelope.dialect = Some(A::DIALECT.into());
        }
        if envelope.schema != A::DOCUMENT_SCHEMA || envelope.dialect != Some(A::DIALECT.into()) || !self.jobs.can_insert(self.operation.0) {
            return Err(envelope);
        }
        let job = A::build_document_store_initialization_job(envelope, self.operation, self.generation, self.actor.clone())?;
        self.jobs.insert_admitted(self.operation.0, ActiveArtifactStoreReplacement::new(self.operation,self.generation,self.child_content_generation,job,self.source_identity));
        Ok(())
    }
}

/// 🗂️ Generation-keyed immutable document, configuration, and history projections.
#[doc(hidden)]
pub type AppProjectionCache<A> = ((u64, u64, u64, HistoryCommandFilter), std::sync::Arc<<A as ArtifactApp>::Snapshot>, std::sync::Arc<<A as ArtifactApp>::Config>, std::sync::Arc<HistoryView>);
