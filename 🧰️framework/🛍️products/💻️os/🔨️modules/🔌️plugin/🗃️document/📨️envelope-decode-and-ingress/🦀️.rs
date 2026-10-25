//! 📨️ Envelope decode and ingress of the document ingress: the decode operation handle and poll, the active decode state with its worker and rejection lifecycles, the active decode and the active ingress with their drop guards. See [`super`].

use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtifactEnvelopeDecodeOperationHandle {
    pub operation: semio_framework_job::OperationId,
    pub generation: semio_framework_job::Generation,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtifactEnvelopeDecodeOperationPoll {
    Pending,
    Progress,
    Ready,
    Cancelled,
    Fault,
}

#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActiveArtifactEnvelopeDecodeState {
    Active,
    Ready,
    ClosingCancelled,
    ClosingFault,
    Complete,
}

/// 🎟️ Keeps original worker receipts and child terminal evidence through one admitted lifecycle turn.
pub(crate) fn original_worker_lifecycle(cx:&mut semio_framework_job::StepContext<'_>,demand:RetirementDemand,grant:RetainedCloneGrant,reason:&'static str,owner:impl FnOnce(&mut semio_framework_job::StepContext<'_>)->Result<(semio_framework_job_worker_authority::WorkerJobCloseStep,bool),semio_framework_job::OriginalTurnFailure>)->Result<PluginLifecycleStep,semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
    semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|cx,grant|{
        let(step,terminal)=cx.with_retained_loan(grant,owner)?;
        let step=match step{
            semio_framework_job_worker_authority::WorkerJobCloseStep::Pending{progress}if progress.fits(grant)=>PluginLifecycleStep::Progress(progress),
            semio_framework_job_worker_authority::WorkerJobCloseStep::Complete{progress}if progress.fits(grant)&&terminal=>PluginLifecycleStep::Progress(progress),
            semio_framework_job_worker_authority::WorkerJobCloseStep::Blocked=>PluginLifecycleStep::AwaitingInput{reason},
            _=>return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original worker retirement violated its grant or terminal witness").with_retained_progress(step.progress()).into())
        };Ok((step,false))
    })
}

/// 🎟️ Retains the whole original rejected source through its actual context and paid terminal witness.
#[doc(hidden)]
pub fn original_rejection_lifecycle(cx:&mut semio_framework_job::StepContext<'_>,demand:RetirementDemand,grant:RetainedCloneGrant,reason:&'static str,owner:impl FnOnce(&mut semio_framework_job::StepContext<'_>)->Result<(semio_framework_job::InteractiveJobCloseStep,bool),semio_framework_job::OriginalTurnFailure>)->Result<PluginLifecycleStep,semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
    semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|cx,grant|{
        let(step,terminal)=cx.with_retained_loan(grant,owner)?;let step=match step.admit(grant,terminal)?{
            semio_framework_job::InteractiveJobCloseStep::Pending{progress}|semio_framework_job::InteractiveJobCloseStep::Complete{progress}=>PluginLifecycleStep::Progress(progress),
            semio_framework_job::InteractiveJobCloseStep::Blocked=>PluginLifecycleStep::AwaitingInput{reason},
        };Ok((step,false))
    })
}

#[doc(hidden)]
pub struct ActiveArtifactEnvelopeDecode<P: Send + 'static, Mutation: Send + 'static> {
    pub operation: semio_framework_job::OperationId,
    pub generation: semio_framework_job::Generation,
    pub cancel: semio_framework_job::CancelToken,
    pub session: ManuallyDrop<Option<semio_framework_job_worker::MountedWorkerJobSession<store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeAuthority<P, Mutation>>>>,
    pub session_rejected: ManuallyDrop<Option<semio_framework_plugin_ownership::reserved_job_session::RefusedWorkerSource<store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeAuthority<P, Mutation>>>>,
    pub outcome_pending: bool,
    terminal_state: Option<ActiveArtifactEnvelopeDecodeState>,
    pub resume_pending: bool,
    pub rejected: ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    pub completion: std::sync::Arc<store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeCompletion>,
    completed_close_requested: bool,
    pub state: ActiveArtifactEnvelopeDecodeState,
}

impl<P, Mutation> ActiveArtifactEnvelopeDecode<P, Mutation>
where
    P: Send + 'static,
    Mutation: Send + 'static,
{
    pub fn admitted(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, decode: store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeAuthority<P, Mutation>, completion: std::sync::Arc<store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeCompletion>, source_identity:(semio_framework_job::OperationId,semio_framework_job::Generation)) -> Self {
        let cancel = semio_framework_job::CancelToken::root_now();
        let session=None;
        let session_rejected=Some(semio_framework_plugin_ownership::reserved_job_session::RefusedWorkerSource::pending(decode,source_identity));

        Self {
            operation,
            generation,
            cancel,
            session: ManuallyDrop::new(session),
            session_rejected: ManuallyDrop::new(session_rejected),
            outcome_pending: false,
            terminal_state: None,
            resume_pending: false,
            rejected: ManuallyDrop::new(None),
            completion,
            completed_close_requested: false,
            state: ActiveArtifactEnvelopeDecodeState::Active,
        }
    }

    pub fn rejected(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, rejected: Box<dyn store::ErasedSnapshotRetirement>, completion: std::sync::Arc<store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeCompletion>) -> Self {
        Self {
            operation,
            generation,
            cancel: semio_framework_job::CancelToken::root_now(),
            session: ManuallyDrop::new(None),
            session_rejected: ManuallyDrop::new(None),
            outcome_pending: false,
            terminal_state: None,
            resume_pending: false,
            rejected: ManuallyDrop::new(Some(rejected)),
            completion,
            completed_close_requested: false,
            state: ActiveArtifactEnvelopeDecodeState::ClosingFault,
        }
    }

    fn request_completed_close(&mut self, completed: &store::os_store::owned_schema_decode::ArtifactEnvelopeCompletedRecordRegistry<P, Mutation>) -> Result<bool, Fault> {
        let Some(ticket) = self.completion.ticket() else { return Ok(true) };
        if !self.completed_close_requested {
            match completed.try_request_close(ticket) {
                Ok(()) => self.completed_close_requested = true,
                Err(store::os_store::owned_schema_decode::ArtifactEnvelopeCompletedRecordFault::Contended) => return Ok(false),
                Err(store::os_store::owned_schema_decode::ArtifactEnvelopeCompletedRecordFault::Stale) if completed.ticket_reclaimed(ticket) => return Ok(true),
                Err(_) => return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.completed-close"), "completed envelope result lost its exact close authority")),
            }
        }
        Ok(completed.ticket_reclaimed(ticket))
    }

    /// 📏️ The minimal grant on every independent axis the next [`Self::drive`] turn needs.
    pub fn drive_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        let item = RetirementDemand { depth: 1, ..Default::default() };
        if let Some(rejected) = self.session_rejected.as_ref() {
            return if rejected.admission_pending()&&!self.cancel.is_cancelled_now(){rejected.admission_demands()}else{rejected.retirement_demands(body)};
        }
        if let Some(rejected) = self.rejected.as_ref() {
            return store::os_store::snapshot_read::artifact_retirement_box_demands(rejected, body);
        }
        let state = if self.state == ActiveArtifactEnvelopeDecodeState::Ready && self.cancel.is_cancelled_now() { ActiveArtifactEnvelopeDecodeState::ClosingCancelled } else { self.state };
        if matches!(state, ActiveArtifactEnvelopeDecodeState::ClosingCancelled | ActiveArtifactEnvelopeDecodeState::ClosingFault | ActiveArtifactEnvelopeDecodeState::Complete) {
            return Ok(item);
        }
        if state == ActiveArtifactEnvelopeDecodeState::Ready && self.session.as_ref().is_some_and(|session| session.terminal_is_empty()) {
            return Ok(item);
        }
        if self.outcome_pending || self.terminal_state.is_some() {
            if let Some(session) = self.session.as_ref() {
                return Ok(session.retirement_demands(body).unwrap_or(item));
            }
        }
        Ok(item)
    }

    /// 🔎 The detail page is the only record of WHY a live decode failed; it retires
    /// with the outcome, so it is reported here before the close loop releases it.
    pub fn drive(
        &mut self,
        cx: &mut semio_framework_job::StepContext<'_>,
        live_generation: semio_framework_job::Generation,
        completed: &store::os_store::owned_schema_decode::ArtifactEnvelopeCompletedRecordRegistry<P, Mutation>,
        grant: RetainedCloneGrant,
    ) -> Result<PluginLifecycleStep, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        if grant.maximum_items==0||grant.maximum_depth==0{return Ok(PluginLifecycleStep::Progress(Default::default()));}
        if let Some(rejected)=self.session_rejected.as_mut(){
            if rejected.admission_pending()&&!cx.is_cancelled()&&!self.cancel.is_cancelled_now(){
                let config=semio_framework_job_worker_authority::BatchDriveConfig{retained:grant,site:"artifact_envelope_decode",stage:semio_framework_job::InteractiveStage::InteractiveStep,fuel_per_step:cx.fuel_remaining(),step_budget_us:0};
                let before=cx.retained_progress();
                if let Some(session)=rejected.admit_pending(config,cx)?{*self.session=Some(session);drop(self.session_rejected.take());}
                return Ok(PluginLifecycleStep::Progress(semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_delta(before,cx.retained_progress())?));
            }

            if rejected.terminal_is_empty(){
                let bytes=size_of_val(rejected);if grant.maximum_copy_bytes<bytes{return Ok(PluginLifecycleStep::Progress(Default::default()));}
                drop(self.session_rejected.take());self.state=if cx.is_cancelled()||self.cancel.is_cancelled_now(){ActiveArtifactEnvelopeDecodeState::ClosingCancelled}else{ActiveArtifactEnvelopeDecodeState::ClosingFault};
                return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}));
            }
            let demand=rejected.retirement_demands(grant.maximum_copy_bytes)?;
            return original_rejection_lifecycle(cx,demand,grant,"envelope rejection awaits its original worker",|cx|{let step=rejected.close_step(cx)?;Ok((step,rejected.terminal_is_empty()))});
        }
        if let Some(rejected)=self.rejected.as_mut(){
            let demand=if rejected.terminal_is_empty(){RetirementDemand{copy_bytes:size_of::<Option<Box<dyn store::ErasedSnapshotRetirement>>>()+size_of::<ActiveArtifactEnvelopeDecodeState>(),release_bytes:size_of_val(&**rejected),depth:1,..Default::default()}}else{RetirementDemand{copy_bytes:rejected.next_copy_byte_demand()?,capacity_bytes:rejected.next_capacity_byte_demand(grant.maximum_copy_bytes)?,release_bytes:rejected.next_release_byte_demand()?,depth:rejected.next_depth_demand()?.checked_add(1).ok_or_else(||plugin_sdk_fault("envelope original rejected frame depth overflow"))?}};
            if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(PluginLifecycleStep::Progress(Default::default()));}
            if rejected.terminal_is_empty(){drop(self.rejected.take());self.state=ActiveArtifactEnvelopeDecodeState::Complete;return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}));}
            let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};
            let step=rejected.close_step(child)?;
            let step=semio_framework_value::retained_clone::admit_retained_clone_close(child,step,rejected.terminal_is_empty(),"envelope original rejected frame")?;
            return Ok(PluginLifecycleStep::retained(step,false));
        }
        if self.outcome_pending{
            let session=self.session.as_mut().ok_or_else(||plugin_sdk_fault("envelope original outcome acknowledgement lost its session"))?;
            let step=session.acknowledge_checked_out_outcome(grant);
            if matches!(step,RetainedCloneStep::Complete(_)){self.outcome_pending=false;self.resume_pending=true;}
            return Ok(PluginLifecycleStep::retained(step,false));
        }
        if self.resume_pending{
            if grant.maximum_copy_bytes<size_of::<bool>(){return Ok(PluginLifecycleStep::Progress(Default::default()));}
            let session=self.session.as_mut().ok_or_else(||plugin_sdk_fault("envelope original outcome handback lost its session"))?;
            if self.terminal_state.is_some(){session.begin_close();}else if session.resume().is_err(){return Ok(PluginLifecycleStep::AwaitingInput{reason:"envelope original outcome resume remains contended"});}
            self.resume_pending=false;return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<bool>(),..Default::default()}));
        }
        if let Some(target)=self.terminal_state{
            let session=self.session.as_mut().ok_or_else(||plugin_sdk_fault("envelope original terminal target lost its session"))?;
            if session.terminal_is_empty(){
                let bytes=size_of_val(session);if grant.maximum_copy_bytes<bytes{return Ok(PluginLifecycleStep::Progress(Default::default()));}
                drop(self.session.take());self.terminal_state=None;if self.state!=ActiveArtifactEnvelopeDecodeState::Ready{self.state=target;}
                return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}));
            }
            let demand=match session.retirement_demands(grant.maximum_copy_bytes){Ok(demand)=>demand,Err(semio_framework_job_worker::WorkerJobDemandError::Contention(_))=>return Ok(PluginLifecycleStep::AwaitingInput{reason:"envelope original worker quote awaits its handback"}),Err(semio_framework_job_worker::WorkerJobDemandError::Refused(error))=>return Err(error.into())};
            return original_worker_lifecycle(cx,demand,grant,"envelope original terminal worker remains contended",|cx|{let step=session.close_step(cx)?;Ok((step,session.terminal_is_empty()))});
        }
        if grant.maximum_copy_bytes<size_of::<ActiveArtifactEnvelopeDecodeState>(){return Ok(PluginLifecycleStep::Progress(Default::default()));}
        if live_generation!=self.generation&&self.state==ActiveArtifactEnvelopeDecodeState::Active{self.cancel.cancel_now();}
        if self.state==ActiveArtifactEnvelopeDecodeState::Ready&&self.cancel.is_cancelled_now(){self.state=ActiveArtifactEnvelopeDecodeState::ClosingCancelled;}
        if matches!(self.state,ActiveArtifactEnvelopeDecodeState::ClosingCancelled|ActiveArtifactEnvelopeDecodeState::ClosingFault){
            if self.request_completed_close(completed)?{self.state=ActiveArtifactEnvelopeDecodeState::Complete;return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<ActiveArtifactEnvelopeDecodeState>(),..Default::default()}));}
            return Ok(PluginLifecycleStep::AwaitingInput{reason:"envelope original completed record awaits its granted return pump"});
        }
        if self.state==ActiveArtifactEnvelopeDecodeState::Complete{return Ok(PluginLifecycleStep::AwaitingInput{reason:"envelope cancellation and completion aliases retain their original close authority"});}
        if self.state==ActiveArtifactEnvelopeDecodeState::Ready&&self.session.is_none(){return Ok(PluginLifecycleStep::AwaitingInput{reason:"decoded envelope awaits exact consumer publication"});}
        let session=self.session.as_mut().ok_or_else(||plugin_sdk_fault("envelope live decode lost its original worker session"))?;
        match session.pump_one_for_interactive_turn(cx){
            Ok(semio_framework_job_worker::WorkerJobPoll::Outcome|semio_framework_job_worker::WorkerJobPoll::Terminal)=>{
                let Some((issued,progress))=session.take_checked_out_retained_step_receipt()else{return Ok(PluginLifecycleStep::Blocked{reason:"envelope worker original receipt awaits its sole recipient"});};
                if !progress.fits(issued)||!progress.fits(grant){return Err(plugin_sdk_fault("envelope worker original receipt exceeded its received turn").with_retained_progress(progress).into());}
                self.terminal_state=match session.checked_out_outcome(){
                    Ok(Some(semio_framework_job::JobOutcomeView::Complete{..}))if self.completion.ticket().is_some()=>{self.state=ActiveArtifactEnvelopeDecodeState::Ready;Some(ActiveArtifactEnvelopeDecodeState::Ready)},
                    Ok(Some(semio_framework_job::JobOutcomeView::Cancelled{..}))=>Some(ActiveArtifactEnvelopeDecodeState::ClosingCancelled),
                    Ok(Some(semio_framework_job::JobOutcomeView::Fault{..}|semio_framework_job::JobOutcomeView::Complete{..}))=>Some(ActiveArtifactEnvelopeDecodeState::ClosingFault),
                    Ok(_)=>None,
                    Err(_)=>return Ok(PluginLifecycleStep::Blocked{reason:"envelope worker original outcome cannot be borrowed"}),
                };
                self.outcome_pending=true;
                Ok(PluginLifecycleStep::Progress(progress))
            }
            Ok(semio_framework_job_worker::WorkerJobPoll::Submitted|semio_framework_job_worker::WorkerJobPoll::Idle)=>Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),
            Ok(_)=>Ok(PluginLifecycleStep::AwaitingInput{reason:"envelope worker awaits its original mounted wake"}),
            Err(failure)=>Err(failure.into())
        }
    }

    pub fn decode_worker_owners_are_terminal(&self) -> bool {
        self.session.is_none() && !self.outcome_pending && self.session_rejected.is_none() && self.rejected.is_none() && self.terminal_state.is_none() && !self.resume_pending
    }

    /// 🧹️ Closes the parked worker session/outcome ladder after [`Self::state`] reached
    /// [`ActiveArtifactEnvelopeDecodeState::Ready`] so store replacement can retire the decode owner.
    /// 🏃️ Whether one more `drive` can advance this decode. A `Ready` decode still runs until its
    /// worker session, retained outcome, and terminal close target are empty — only then does it
    /// park awaiting exact consumer publication.
    pub fn has_runnable_work(&self) -> bool {
        if self.state == ActiveArtifactEnvelopeDecodeState::Ready {
            return self.terminal_state.is_some() || self.outcome_pending || self.session.as_ref().is_some_and(|session| !session.terminal_is_empty());
        }
        self.state != ActiveArtifactEnvelopeDecodeState::Ready
    }

    /// 📊️ A decode cancelled after its worker finished is Cancelled, never Ready: maintenance closes
    /// its completed record, so no consumer may admit it in between.
    pub fn poll(&self) -> ArtifactEnvelopeDecodeOperationPoll {
        match self.state {
            ActiveArtifactEnvelopeDecodeState::Active => ArtifactEnvelopeDecodeOperationPoll::Pending,
            ActiveArtifactEnvelopeDecodeState::Ready if self.cancel.is_cancelled_now() => ArtifactEnvelopeDecodeOperationPoll::Cancelled,
            ActiveArtifactEnvelopeDecodeState::Ready => ArtifactEnvelopeDecodeOperationPoll::Ready,
            ActiveArtifactEnvelopeDecodeState::ClosingCancelled => ArtifactEnvelopeDecodeOperationPoll::Cancelled,
            ActiveArtifactEnvelopeDecodeState::ClosingFault => ArtifactEnvelopeDecodeOperationPoll::Fault,
            ActiveArtifactEnvelopeDecodeState::Complete if self.cancel.is_cancelled_now() => ArtifactEnvelopeDecodeOperationPoll::Cancelled,
            ActiveArtifactEnvelopeDecodeState::Complete => ArtifactEnvelopeDecodeOperationPoll::Fault,
        }
    }

    pub fn terminal_is_empty(&self, completed: &store::os_store::owned_schema_decode::ArtifactEnvelopeCompletedRecordRegistry<P, Mutation>) -> bool {
        let output_terminal = self.completion.ticket().is_none_or(|ticket| completed.ticket_reclaimed(ticket));
        self.state == ActiveArtifactEnvelopeDecodeState::Complete && self.session.is_none() && self.session_rejected.is_none() && !self.outcome_pending && self.rejected.is_none() && output_terminal
    }
}

impl<P: Send + 'static, Mutation: Send + 'static> Drop for ActiveArtifactEnvelopeDecode<P, Mutation> {
    fn drop(&mut self) {
        assert!(
            std::thread::panicking() || self.session.is_none() && self.session_rejected.is_none() && !self.outcome_pending && self.rejected.is_none() && self.state == ActiveArtifactEnvelopeDecodeState::Complete,
            "active envelope decode reached Drop before its worker and rejected owner were terminal empty"
        );
    }
}

#[doc(hidden)]
pub struct ActiveArtifactEnvelopeIngress {
    pub pages: ManuallyDrop<Option<store::os_store::owned_schema_decode::OwnedSchemaDecodePages>>,
    pub closing: bool,
}

impl ActiveArtifactEnvelopeIngress {
    pub fn new(pages: store::os_store::owned_schema_decode::OwnedSchemaDecodePages) -> Self {
        Self { pages: ManuallyDrop::new(Some(pages)), closing: false }
    }

    pub fn take_pages(&mut self) -> Option<store::os_store::owned_schema_decode::OwnedSchemaDecodePages> {
        self.pages.take()
    }

    /// 📏️ The next page's payload, then the slot backing's whole physical release.
    pub fn retirement_demands(&self) -> RetirementDemand {
        match self.pages.as_ref() {
            None => RetirementDemand::default(),
            Some(pages) => match pages.close_page_bytes() {
                Some(copy_bytes) => RetirementDemand { copy_bytes, depth: 1, ..Default::default() },
                None => RetirementDemand { release_bytes: pages.allocation_byte_demand(), depth: 1, ..Default::default() },
            },
        }
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, Fault> {
        let empty = RetainedCloneProgress::default();
        if self.pages.is_none() {
            return Ok(RetainedCloneStep::Complete(empty));
        }
        if !plugin_turn_admitted(self.retirement_demands(), grant)? {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let pages = self.pages.as_mut().expect("ingress pages were observed");
        if let Some(page) = pages.close_take_page() {
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: page.len(), ..empty }));
        }
        let progress = pages.close_backing_step(grant).map_err(|_| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.pages-close"), "ingress decode directory refused its admitted release"))?.progress();
        assert!(pages.terminal_is_empty());
        drop(self.pages.take());
        Ok(RetainedCloneStep::Progress(progress))
    }
    pub fn terminal_is_empty(&self) -> bool {
        self.pages.is_none()
    }
}

impl Drop for ActiveArtifactEnvelopeIngress {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "artifact envelope ingress reached Drop before every admitted page was transferred or bounded-closed");
    }
}
