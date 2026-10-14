//! 🗃️ Document ingress: envelope decode and ingress, store replacement, composition pins and document archive loads.
use protocol::{OpBinary, OpText};
use semio_framework::{kernel::ActorId, Fault, FaultCode, FaultOrigin};
/// 🚪️ `io`'s `ArtifactRef`/`ArtifactKindId` vocabulary is not glob-re-exported at the
/// `semio-framework-os-kernel` crate root (deliberate — see that crate's own glue.rs comment on
/// the `io` mount), so it is named through the `store::io::` path everywhere in this file,
/// exactly like the sibling `🎞️gif` migration leaf (`semio_framework_artifact_reference::ArtifactDialect`) already does.
use semio_framework_artifact_reference::ArtifactRef;
use pack::snapshot::ArtifactPack;
use store::{os_store::store_schema::ArtifactEnvelope, ArtifactStore, os_store::composition_coordinator::CompositionCoordinator, MemberFactory, Mutation, SpaceMember};
/// 🏅️🪆️🎯️ Standards/subsets dialect vocabulary (ticket 26/08/10/STDIO-ARTIFACTS-AND-IO phase
/// 2). Defined in `semio_framework` so plugins and the OS product
/// share one definition without an inverted dependency; re-exported here verbatim.
use semio_framework_artifact_reference::ArtifactDialect;
use semio_framework_value::{RetirementDemand, ValueError, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
use std::mem::ManuallyDrop;
use crate::child_content::advance_original_child_root_retirement;
use crate::child_content::original_child_root_retirement_demands;
use crate::app_contract::ArtifactApp;
use crate::app_contract::DOCUMENT_LOAD_FAILED_CODE;
use crate::app_contract::document_load_fault;
use crate::bounded_ownership::ARTIFACT_LIVE_OUTPUT_SLOTS;
use crate::bounded_ownership::ArtifactFixedRegistry;
use crate::bounded_ownership::PluginLifecycleStep;
use crate::bounded_ownership::plugin_bytes_close;
use crate::bounded_ownership::plugin_bytes_retirement_demand;
use crate::bounded_ownership::plugin_turn_admitted;
use crate::child_content::ChildContentBorrowedOwners;
use crate::child_view::ChildContentEntry;
use crate::child_content::ChildContentRetirement;
use crate::child_view::ChildContentView;
use crate::child_content::ChildMemberEntry;
use crate::child_content::ChildMemberRegistry;
use crate::child_content::ChildMemberRetirement;
use crate::child_content::DocumentClosureSourceView;
use crate::child_view::MemberKeyRef;
use crate::child_content::OWNED_DOCUMENT_MEMBER_PREPARATION_BYTES;
use crate::child_content::OwnedDocumentMemberIngress;
use crate::child_content::OwnedDocumentMemberIngressRegistry;
use crate::history_view::HistoryCommandFilter;
use crate::history_view::HistoryView;
use crate::sdk_fault::plugin_sdk_fault;
use crate::store_lifecycle::ArtifactDisposal;
use crate::store_lifecycle::ArtifactStoreInitializationJob;



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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ActiveArtifactEnvelopeDecodeState {
    Active,
    Ready,
    ClosingCancelled,
    ClosingFault,
    Complete,
}

/// 🎟️ Keeps original worker receipts and child terminal evidence through one admitted lifecycle turn.
pub(crate) fn original_worker_lifecycle(cx:&mut semio_framework_job::StepContext<'_>,demand:RetirementDemand,grant:RetainedCloneGrant,reason:&'static str,owner:impl FnOnce(&mut semio_framework_job::StepContext<'_>)->Result<(semio_framework_job::WorkerJobCloseStep,bool),semio_framework_job::OriginalTurnFailure>)->Result<PluginLifecycleStep,crate::PluginLifecycleFailure>{
    crate::bounded_ownership::original_lifecycle_step(cx,grant,demand,|cx,grant|{
        let(step,terminal)=cx.with_retained_loan(grant,owner)?;
        let step=match step{
            semio_framework_job::WorkerJobCloseStep::Pending{progress}if progress.fits(grant)=>PluginLifecycleStep::Progress(progress),
            semio_framework_job::WorkerJobCloseStep::Complete{progress}if progress.fits(grant)&&terminal=>PluginLifecycleStep::Progress(progress),
            semio_framework_job::WorkerJobCloseStep::Blocked=>PluginLifecycleStep::AwaitingInput{reason},
            _=>return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original worker retirement violated its grant or terminal witness").with_retained_progress(step.progress()).into())
        };Ok((step,false))
    })
}

/// 🎟️ Retains the whole original rejected source through its actual context and paid terminal witness.
pub(crate) fn original_rejection_lifecycle(cx:&mut semio_framework_job::StepContext<'_>,demand:RetirementDemand,grant:RetainedCloneGrant,reason:&'static str,owner:impl FnOnce(&mut semio_framework_job::StepContext<'_>)->Result<(semio_framework_job::InteractiveJobCloseStep,bool),semio_framework_job::OriginalTurnFailure>)->Result<PluginLifecycleStep,crate::PluginLifecycleFailure>{
    crate::bounded_ownership::original_lifecycle_step(cx,grant,demand,|cx,grant|{
        let(step,terminal)=cx.with_retained_loan(grant,owner)?;let step=match step.admit(grant,terminal)?{
            semio_framework_job::InteractiveJobCloseStep::Pending{progress}|semio_framework_job::InteractiveJobCloseStep::Complete{progress}=>PluginLifecycleStep::Progress(progress),
            semio_framework_job::InteractiveJobCloseStep::Blocked=>PluginLifecycleStep::AwaitingInput{reason},
        };Ok((step,false))
    })
}

pub(crate) struct ActiveArtifactEnvelopeDecode<P: Send + 'static, Mutation: Send + 'static> {
    pub(crate) operation: semio_framework_job::OperationId,
    pub(crate) generation: semio_framework_job::Generation,
    pub(crate) cancel: semio_framework_job::CancelToken,
    session: ManuallyDrop<Option<semio_framework_job::MountedWorkerJobSession<store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeAuthority<P, Mutation>>>>,
    session_rejected: ManuallyDrop<Option<crate::reserved_job_session::RefusedWorkerSource<store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeAuthority<P, Mutation>>>>,
    pub(crate) outcome_pending: bool,
    terminal_state: Option<ActiveArtifactEnvelopeDecodeState>,
    pub(crate) resume_pending: bool,
    rejected: ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    pub(crate) completion: std::sync::Arc<store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeCompletion>,
    completed_close_requested: bool,
    pub(crate) state: ActiveArtifactEnvelopeDecodeState,
}

impl<P, Mutation> ActiveArtifactEnvelopeDecode<P, Mutation>
where
    P: Send + 'static,
    Mutation: Send + 'static,
{
    pub(crate) fn admitted(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, decode: store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeAuthority<P, Mutation>, completion: std::sync::Arc<store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeCompletion>, source_identity:(semio_framework_job::OperationId,semio_framework_job::Generation)) -> Self {
        let cancel = semio_framework_job::CancelToken::root_now();
        let session=None;
        let session_rejected=Some(crate::reserved_job_session::RefusedWorkerSource::pending(decode,source_identity));

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

    pub(crate) fn rejected(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, rejected: Box<dyn store::ErasedSnapshotRetirement>, completion: std::sync::Arc<store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeCompletion>) -> Self {
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
    pub(crate) fn drive_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
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
    pub(crate) fn drive(
        &mut self,
        cx: &mut semio_framework_job::StepContext<'_>,
        live_generation: semio_framework_job::Generation,
        completed: &store::os_store::owned_schema_decode::ArtifactEnvelopeCompletedRecordRegistry<P, Mutation>,
        grant: RetainedCloneGrant,
    ) -> Result<PluginLifecycleStep, crate::PluginLifecycleFailure> {
        if grant.maximum_items==0||grant.maximum_depth==0{return Ok(PluginLifecycleStep::Progress(Default::default()));}
        if let Some(rejected)=self.session_rejected.as_mut(){
            if rejected.admission_pending()&&!cx.is_cancelled()&&!self.cancel.is_cancelled_now(){
                let config=semio_framework_job::BatchDriveConfig{retained:grant,site:"artifact_envelope_decode",stage:semio_framework_job::InteractiveStage::InteractiveStep,fuel_per_step:cx.fuel_remaining(),step_budget_us:0};
                let before=cx.retained_progress();
                if let Some(session)=rejected.admit_pending(config,cx)?{*self.session=Some(session);drop(self.session_rejected.take());}
                return Ok(PluginLifecycleStep::Progress(crate::bounded_ownership::original_lifecycle_delta(before,cx.retained_progress())?));
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
            let demand=match session.retirement_demands(grant.maximum_copy_bytes){Ok(demand)=>demand,Err(semio_framework_job::WorkerJobDemandError::Contention(_))=>return Ok(PluginLifecycleStep::AwaitingInput{reason:"envelope original worker quote awaits its handback"}),Err(semio_framework_job::WorkerJobDemandError::Refused(error))=>return Err(error.into())};
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
            Ok(semio_framework_job::WorkerJobPoll::Outcome|semio_framework_job::WorkerJobPoll::Terminal)=>{
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
            Ok(semio_framework_job::WorkerJobPoll::Submitted|semio_framework_job::WorkerJobPoll::Idle)=>Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),
            Ok(_)=>Ok(PluginLifecycleStep::AwaitingInput{reason:"envelope worker awaits its original mounted wake"}),
            Err(failure)=>Err(failure.into())
        }
    }

    pub(crate) fn decode_worker_owners_are_terminal(&self) -> bool {
        self.session.is_none() && !self.outcome_pending && self.session_rejected.is_none() && self.rejected.is_none() && self.terminal_state.is_none() && !self.resume_pending
    }

    /// 🧹️ Closes the parked worker session/outcome ladder after [`Self::state`] reached
    /// [`ActiveArtifactEnvelopeDecodeState::Ready`] so store replacement can retire the decode owner.
    /// 🏃️ Whether one more `drive` can advance this decode. A `Ready` decode still runs until its
    /// worker session, retained outcome, and terminal close target are empty — only then does it
    /// park awaiting exact consumer publication.
    pub(crate) fn has_runnable_work(&self) -> bool {
        if self.state == ActiveArtifactEnvelopeDecodeState::Ready {
            return self.terminal_state.is_some() || self.outcome_pending || self.session.as_ref().is_some_and(|session| !session.terminal_is_empty());
        }
        self.state != ActiveArtifactEnvelopeDecodeState::Ready
    }

    /// 📊️ A decode cancelled after its worker finished is Cancelled, never Ready: maintenance closes
    /// its completed record, so no consumer may admit it in between.
    pub(crate) fn poll(&self) -> ArtifactEnvelopeDecodeOperationPoll {
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

    pub(crate) fn terminal_is_empty(&self, completed: &store::os_store::owned_schema_decode::ArtifactEnvelopeCompletedRecordRegistry<P, Mutation>) -> bool {
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

pub(crate) struct ActiveArtifactEnvelopeIngress {
    pub(crate) pages: ManuallyDrop<Option<store::os_store::owned_schema_decode::OwnedSchemaDecodePages>>,
    pub(crate) closing: bool,
}

impl ActiveArtifactEnvelopeIngress {
    pub(crate) fn new(pages: store::os_store::owned_schema_decode::OwnedSchemaDecodePages) -> Self {
        Self { pages: ManuallyDrop::new(Some(pages)), closing: false }
    }

    pub(crate) fn take_pages(&mut self) -> Option<store::os_store::owned_schema_decode::OwnedSchemaDecodePages> {
        self.pages.take()
    }

    /// 📏️ The next page's payload, then the slot backing's whole physical release.
    pub(crate) fn retirement_demands(&self) -> RetirementDemand {
        match self.pages.as_ref() {
            None => RetirementDemand::default(),
            Some(pages) => match pages.close_page_bytes() {
                Some(copy_bytes) => RetirementDemand { copy_bytes, depth: 1, ..Default::default() },
                None => RetirementDemand { release_bytes: pages.allocation_byte_demand(), depth: 1, ..Default::default() },
            },
        }
    }

    pub(crate) fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, Fault> {
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
        let released_bytes = pages.allocation_byte_demand();
        let pages = self.pages.take().expect("terminal ingress pages remain retained");
        assert!(pages.terminal_is_empty());
        drop(pages);
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes, ..empty }))
    }
    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.pages.is_none()
    }
}

impl Drop for ActiveArtifactEnvelopeIngress {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "artifact envelope ingress reached Drop before every admitted page was transferred or bounded-closed");
    }
}

/// 🔐️ Performs the sole document-store swap only inside its fresh authority decision.
#[expect(clippy::result_large_err, reason = "A rejected publication returns the unchanged candidate store, preserving its existing bounded retirement authority.")]
pub fn publish_document_store_candidate_if_authoritative<P, Mutation>(
    live: &mut ArtifactStore<P, Mutation>,
    candidate: ArtifactStore<P, Mutation>,
    validate: impl FnOnce() -> Result<(), Fault>,
) -> Result<ArtifactStore<P, Mutation>, (Fault, ArtifactStore<P, Mutation>)>
where
    P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + Send + Sync + 'static,
    Mutation: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + store::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    if let Err(fault) = validate() {
        return Err((fault, candidate));
    }
    Ok(std::mem::replace(live, candidate))
}

/// 🔄️ Publishes the replacement owner's heap candidate without moving either complete Store
/// through the maintenance stack; the returned box owns the displaced live Store in place.
pub(crate) fn publish_boxed_document_store_candidate_if_authoritative<P, Mutation>(
    live: &mut ArtifactStore<P, Mutation>,
    mut candidate: Box<ArtifactStore<P, Mutation>>,
    validate: impl FnOnce() -> Result<(), Fault>,
) -> Result<Box<ArtifactStore<P, Mutation>>, (Fault, Box<ArtifactStore<P, Mutation>>)>
where
    P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + Send + Sync + 'static,
    Mutation: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + store::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    if let Err(fault) = validate() {
        return Err((fault, candidate));
    }
    std::mem::swap(live, candidate.as_mut());
    Ok(candidate)
}

pub(crate) struct CompositionPinsRetirement {
    pub(crate) pins: ManuallyDrop<Option<Vec<vcs::CompositionPin>>>,
    active: ManuallyDrop<Option<vcs::CompositionPin>>,
    pub(crate) field: u8,
}

impl CompositionPinsRetirement {
    pub(crate) fn new(pins: Vec<vcs::CompositionPin>) -> Self {
        Self { pins: ManuallyDrop::new(Some(pins)), active: ManuallyDrop::new(None), field: 0 }
    }

    fn active_string_mut(&mut self) -> Option<&mut String> {
        let pin = self.active.as_mut()?;
        match self.field {
            0 => Some(&mut pin.child_ref.artifact_id),
            1 => Some(&mut pin.child_ref.dialect.artifact_kind),
            2 => Some(&mut pin.child_ref.dialect.standard),
            3 => Some(&mut pin.child_ref.dialect.subset),
            4 => Some(&mut pin.checkpoint_id),
            _ => None,
        }
    }

    /// 📐️ Quotes one original pin transfer, native field, or separate vector backing.
    pub(crate) fn retirement_demands(&self) -> Result<RetirementDemand, ValueError> {
        if let Some(pin) = self.active.as_ref() {
            let value = match self.field { 0 => Some(&pin.child_ref.artifact_id), 1 => Some(&pin.child_ref.dialect.artifact_kind), 2 => Some(&pin.child_ref.dialect.standard), 3 => Some(&pin.child_ref.dialect.subset), 4 => Some(&pin.checkpoint_id), _ => None };
            return Ok(value.map_or(RetirementDemand { copy_bytes: size_of_val(&*self.active), depth: 1, ..Default::default() }, |value| RetirementDemand { copy_bytes: size_of::<String>() + size_of::<u8>(), release_bytes: value.capacity(), depth: 1, ..Default::default() }));
        }
        let Some(pins) = self.pins.as_ref() else { return Ok(RetirementDemand::default()); };
        if !pins.is_empty() { return Ok(RetirementDemand { copy_bytes: size_of::<vcs::CompositionPin>() + size_of_val(&*self.active) + size_of::<usize>() + size_of::<u8>(), depth: 1, ..Default::default() }); }
        let release_bytes = pins.capacity().checked_mul(size_of::<vcs::CompositionPin>()).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "original composition-pin backing extent overflowed"))?;
        Ok(RetirementDemand { copy_bytes: size_of_val(&*self.pins), release_bytes, depth: 1, ..Default::default() })
    }

    /// 🧹️ Retains every original until all five caller currencies admit its physical unit.
    pub(crate) fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, Fault> {
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        let demand = self.retirement_demands().map_err(|error| plugin_sdk_fault(error.into_message()))?;
        if grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes || grant.maximum_depth < demand.depth { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if self.active.is_some() {
            if let Some(value) = self.active_string_mut() { drop(std::mem::take(value)); self.field += 1; } else { drop(self.active.take()); }
        } else if self.pins.as_ref().is_some_and(|pins| !pins.is_empty()) {
            *self.active = self.pins.as_mut().and_then(Vec::pop);
            self.field = 0;
        } else { drop(self.pins.take()); }
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes: demand.release_bytes, ..Default::default() }))
    }
    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.pins.is_none() && self.active.is_none()
    }
}

impl Drop for CompositionPinsRetirement {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "composition-pin retirement reached Drop before every displaced identity owner was bounded-closed");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ActiveArtifactStoreReplacementState {
    Initializing,
    AwaitingMembers,
    OpeningMembers,
    ClosingRejectedMember,
    ValidatingClosure,
    PreparingCandidateViews,
    CandidateReady,
    RetiringRejectedMembers,
    RetiringCommittedMembers,
    RetiringCommittedStore,
    RetiringRejectedCandidate,
    Complete,
}

/// 🧭️ Which leg of a document-archive replacement refused the candidate, and with what
/// evidence. An atomic replacement validates three of them in order — the owned-member set
/// (`members`), the recursive ownership `closure`, and the `retained-publication` guard plus the
/// app's own publication `authority` — and every one of them used to collapse into a single
/// `faulted: bool` whose only explanation was a `#[cfg(test)]` `eprintln!`. A headless host
/// therefore saw one opaque sentence for eleven distinct causes. This carries the cause to the
/// wire instead: {@link ArtifactStoreReplacementRefusal::into_fault} mints the leg-specific
/// {@link FaultCode} and detail that `Effect::LoadDocument` reports to the shell.
/// 🔖️ A bounded inline copy of a job fault's first detail page — the initializer's own
/// refusal slug (e.g. `bounded-store.initializer-envelope-invalid`), which the replacement used
/// to drop on the floor. Fixed capacity, no allocation, `Copy`: it rides the refusal record.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ArtifactStoreRefusalCode {
    bytes: [u8; 96],
    len: u8,
}

impl ArtifactStoreRefusalCode {
    const EMPTY: Self = Self { bytes: [0; 96], len: 0 };

    fn from_page(page: &[u8]) -> Self {
        let mut bytes = [0u8; 96];
        let len = page.len().min(bytes.len());
        bytes[..len].copy_from_slice(&page[..len]);
        Self { bytes, len: len as u8 }
    }

    fn text(&self) -> &str {
        std::str::from_utf8(&self.bytes[..self.len as usize]).unwrap_or("<non-utf8 refusal page>")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ArtifactStoreReplacementRefusal {
    Invariant { leg: ArtifactStoreReplacementLeg, detail: &'static str },
    InitializerAdmissionRejected,
    InitializerFailed(ArtifactStoreRefusalCode),
    Cancelled,
    MemberOpenRejected { ordinal: usize, diagnostic: store::MemberOpenDiagnostic },
    MemberIdentityMismatch { ordinal: usize, expected: usize },
    MemberFactoryRefused { ordinal: usize },
    ClosureRejected { diagnostic: store::OwnedDocumentClosureDiagnostic },
    ClosureMemberCount { members: usize, children: usize },
    PublicationGuard(ArtifactStoreReplacementPublicationGuard),
    PublicationAuthorityRejected,
}

/// 🦵️ The named leg a refusal belongs to — the same three the host message lists.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ArtifactStoreReplacementLeg {
    Initializer,
    Members,
    Closure,
    CandidateViews,
    RetainedPublication,
}

impl ArtifactStoreReplacementLeg {
    fn slug(self) -> &'static str {
        match self {
            Self::Initializer => "initializer",
            Self::Members => "members",
            Self::Closure => "closure",
            Self::CandidateViews => "candidate-views",
            Self::RetainedPublication => "retained-publication",
        }
    }
}

/// 🚦️ Every input the `CandidateReady` publication guard reads, as it read them.
/// A bare "guard rejected" is unactionable; which of the seven conditions tripped is the answer.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct ArtifactStoreReplacementPublicationGuard {
    pub(crate) closing: bool,
    pub(crate) cancelled: bool,
    pub(crate) parent_generation: u64,
    pub(crate) live_generation: u64,
    pub(crate) base_child_generation: u64,
    pub(crate) child_generation: u64,
    pub(crate) complete_candidate: bool,
    pub(crate) retirement_admitted: bool,
    pub(crate) next_generation: bool,
}

impl ArtifactStoreReplacementRefusal {
    fn leg(self) -> ArtifactStoreReplacementLeg {
        match self {
            Self::Invariant { leg, .. } => leg,
            Self::InitializerAdmissionRejected | Self::InitializerFailed(_) => ArtifactStoreReplacementLeg::Initializer,
            Self::Cancelled | Self::MemberOpenRejected { .. } | Self::MemberIdentityMismatch { .. } | Self::MemberFactoryRefused { .. } => ArtifactStoreReplacementLeg::Members,
            Self::ClosureRejected { .. } | Self::ClosureMemberCount { .. } => ArtifactStoreReplacementLeg::Closure,
            Self::PublicationGuard(_) | Self::PublicationAuthorityRejected => ArtifactStoreReplacementLeg::RetainedPublication,
        }
    }

    fn slug(self) -> &'static str {
        match self {
            Self::Invariant { .. } => "invariant",
            Self::InitializerAdmissionRejected => "initializer-admission-rejected",
            Self::InitializerFailed(_) => "initializer-failed",
            Self::Cancelled => "cancelled",
            Self::MemberOpenRejected { .. } => "member-open-rejected",
            Self::MemberIdentityMismatch { .. } => "member-identity-mismatch",
            Self::MemberFactoryRefused { .. } => "member-factory-refused",
            Self::ClosureRejected { .. } => "closure-rejected",
            Self::ClosureMemberCount { .. } => "closure-member-count",
            Self::PublicationGuard(_) => "publication-guard",
            Self::PublicationAuthorityRejected => "publication-authority-rejected",
        }
    }

    fn detail(self) -> String {
        match self {
            Self::Invariant { detail, .. } => detail.to_string(),
            Self::InitializerAdmissionRejected => "the app's document-store initialization job was refused admission to the interactive worker lane".to_string(),
            Self::InitializerFailed(code) => format!("the app's document-store initialization job faulted or was cancelled before it produced a candidate, reporting `{}`; its `build_document_store_initialization_job` refused this envelope, or the envelope's schema/id/history failed that job's own validation", code.text()),
            Self::Cancelled => "the replacement was cancelled before its member set reached candidate readiness".to_string(),
            Self::MemberOpenRejected { ordinal, diagnostic } => format!("owned member {ordinal} was rejected by its typed open ({diagnostic:?})"),
            Self::MemberIdentityMismatch { ordinal, expected } => format!("owned member open returned ordinal {ordinal} while the sealed set expected {expected}, or its artifact/owner reference changed across the open"),
            Self::MemberFactoryRefused { ordinal } => format!("the app's member factory refused the admitted open request for ordinal {ordinal}"),
            Self::ClosureRejected { diagnostic } => format!("recursive ownership closure validation rejected the candidate ({diagnostic:?})"),
            Self::ClosureMemberCount { members, children } => format!("closure validation reached {members} members while the candidate registry owns {children}; the archive's member roster is not the parent's exact owned closure"),
            Self::PublicationGuard(guard) => format!(
                "the retained publication guard refused the ready candidate: closing={} cancelled={} parent-generation={} live-generation={} base-child-generation={} child-generation={} complete-candidate={} retirement-admitted={} next-generation={}",
                guard.closing,
                guard.cancelled,
                guard.parent_generation,
                guard.live_generation,
                guard.base_child_generation,
                guard.child_generation,
                guard.complete_candidate,
                guard.retirement_admitted,
                guard.next_generation,
            ),
            Self::PublicationAuthorityRejected => "the app's `validate_document_store_publication` refused the exact candidate".to_string(),
        }
    }

    /// 🧯️ The typed fault the host reports instead of the one-sentence collapse.
    pub(crate) fn into_fault(self) -> Fault {
        let leg = self.leg();
        Fault::new(FaultOrigin::Plugin, FaultCode::new(format!("plugin.internal.document-archive-replacement.{}", self.slug())), format!("document archive replacement failed its {} leg: {}", leg.slug(), self.detail()))
    }
}

pub(crate) struct ActiveArtifactStoreReplacement<P, Mutation, M>
where
    P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,
    Mutation: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + store::Mutation<P> + OpBinary + OpText + Send + 'static,
    M: SpaceMember + MemberFactory,
{
    pub(crate) operation: semio_framework_job::OperationId,
    pub(crate) generation: semio_framework_job::Generation,
    pub(crate) base_child_content_generation: u64,
    cancel: ManuallyDrop<Option<semio_framework_job::CancelToken>>,
    cancel_signal: ManuallyDrop<Option<std::sync::Arc<std::sync::atomic::AtomicBool>>>,
    cancel_retirement: ManuallyDrop<Option<semio_framework_async::CancelTokenRetirement>>,
    session: ManuallyDrop<Option<semio_framework_job::MountedWorkerJobSession<ArtifactStoreInitializationJob<P, Mutation>>>>,
    session_rejected: ManuallyDrop<Option<crate::reserved_job_session::RefusedWorkerSource<ArtifactStoreInitializationJob<P, Mutation>>>>,
    pub(crate) outcome_pending: bool,
    outcome_terminal: Option<(u8, ArtifactStoreRefusalCode)>,
    terminal_target: Option<ActiveArtifactStoreReplacementState>,
    pub(crate) resume_pending: bool,
    pub(crate) retained_store: ManuallyDrop<Option<Box<ArtifactStore<P, Mutation>>>>,
    pub(crate) retained_disposer: ManuallyDrop<ArtifactDisposal<ArtifactStore<P, Mutation>>>,
    pub(crate) member_ingress: ManuallyDrop<Option<OwnedDocumentMemberIngressRegistry>>,
    active_member_ingress: ManuallyDrop<Option<OwnedDocumentMemberIngress>>,
    pub(crate) active_member_open: ManuallyDrop<Option<M::Open>>,
    member_open_admission: ManuallyDrop<Option<store::MemberOpenSourceAdmission>>,
    opened_member: ManuallyDrop<Option<M>>,
    pub(crate) candidate_children: ManuallyDrop<Option<ChildMemberRegistry<M>>>,
    pub(crate) candidate_content: ManuallyDrop<Option<ChildContentView>>,
    candidate_content_retirement: ManuallyDrop<Option<ChildContentRetirement>>,
    pub(crate) displaced_content_retirements: ManuallyDrop<Option<ArtifactFixedRegistry<ChildContentRetirement>>>,
    displaced_content_retirement_cursor: usize,
    pub(crate) candidate_composition: ManuallyDrop<Option<CompositionCoordinator>>,
    pub(crate) displaced_pins: ManuallyDrop<Option<CompositionPinsRetirement>>,
    retiring_child: ManuallyDrop<Option<ChildMemberRetirement<M>>>,
    retiring_child_cursor: usize,
    next_member_ordinal: usize,
    pub(crate) view_member_ordinal: usize,
    pub(crate) candidate_source_generation: u64,
    member_step_sequence: u64,
    expires_at_us: u64,
    closure: Option<store::OwnedDocumentClosure>,
    pub(crate) committed: bool,
    pub(crate) refusal: Option<ArtifactStoreReplacementRefusal>,
    /// 📈️ The initializer's last reported `(operations folded, operations to fold)` (gap N17), kept after its handoff.
    pub(crate) fold_progress: (u64, u64),
    pub(crate) state: ActiveArtifactStoreReplacementState,
}

impl<P, Mutation, M> ActiveArtifactStoreReplacement<P, Mutation, M>
where
    P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,
    Mutation: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + store::Mutation<P> + OpBinary + OpText + Send + 'static,
    M: SpaceMember + MemberFactory,
{
    pub(crate) fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, base_child_content_generation:u64,job:ArtifactStoreInitializationJob<P,Mutation>,source_identity:(semio_framework_job::OperationId,semio_framework_job::Generation))->Self {
        let cancel = semio_framework_job::CancelToken::root_now();
        let cancel_signal = job.cancel_signal();
        let session=None;
        let session_rejected=Some(crate::reserved_job_session::RefusedWorkerSource::pending(job,source_identity));

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
    #[cfg(test)]
    pub(crate) fn refusal_fault(&self) -> Option<Fault> {
        self.refusal.map(ArtifactStoreReplacementRefusal::into_fault)
    }

    /// 🧭️ Records the FIRST leg that refused and moves the replacement onto its retirement path.
    /// The first one is kept because every later state transition is a consequence of it.
    pub(crate) fn refuse(&mut self, refusal: ArtifactStoreReplacementRefusal) {
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

    pub(crate) fn request_cancel(&self) {
        if let Some(cancel) = self.cancel.as_ref() { cancel.cancel_now(); }
        if let Some(signal) = self.cancel_signal.as_ref() { signal.store(true, std::sync::atomic::Ordering::Release); }
    }

    /// 🧪️ Borrows the terminal retained store frame through its exact replacement authority.
    #[cfg(test)]
    pub(crate) fn terminal_store_frame_bytes(&self) -> Option<usize> {
        if !matches!(self.state, ActiveArtifactStoreReplacementState::RetiringRejectedCandidate | ActiveArtifactStoreReplacementState::RetiringCommittedStore) { return None; }
        self.retained_store.as_ref().filter(|owner| owner.close_owned_store_terminal_is_empty()).map(|owner| std::mem::size_of_val(owner.as_ref()))
    }

    pub(crate) fn begin_members(&mut self, expected: usize, expires_at_us: u64) -> Result<bool, Fault> {
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
    pub(crate) fn admit_member(&mut self, ingress: OwnedDocumentMemberIngress) -> Result<(), (Fault, OwnedDocumentMemberIngress)> {
        if self.state != ActiveArtifactStoreReplacementState::AwaitingMembers || ingress.operation() != self.operation || ingress.generation() != self.generation {
            return Err((plugin_sdk_fault("owned document member ingress does not match the active replacement authority"), ingress));
        }
        let Some(registry) = self.member_ingress.as_mut() else {
            return Err((plugin_sdk_fault("owned document member ingress registry has not been initialized"), ingress));
        };
        registry.admit(ingress)
    }

    pub(crate) fn seal_members(&mut self) -> Result<(), Fault> {
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
    pub(crate) fn candidate_awaiting_members(&self) -> Option<&ArtifactStore<P, Mutation>> {
        (self.state == ActiveArtifactStoreReplacementState::AwaitingMembers).then(|| self.retained_store.as_deref()).flatten()
    }

    pub(crate) fn cancelled(&self) -> bool {
        self.cancel.as_ref().is_some_and(semio_framework_job::CancelToken::is_cancelled_now)
    }

    /// 📏️ Quotes the exact next [`Self::drive_initializer`] turn.
    pub(crate) fn initializer_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        let item = RetirementDemand { depth: 1, ..Default::default() };
        let session_demand = |session: &semio_framework_job::MountedWorkerJobSession<ArtifactStoreInitializationJob<P, Mutation>>| match session.retirement_demands(body) {
            Ok(demand) => Ok(demand),
            Err(semio_framework_job::WorkerJobDemandError::Contention(_)) => Ok(item),
            Err(semio_framework_job::WorkerJobDemandError::Refused(error)) => Err(error),
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
    pub(crate) fn member_open_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
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

    pub(crate) fn closure_demands(&self) -> RetirementDemand {
        RetirementDemand { copy_bytes: store::MEMBER_OPEN_IDENTITY_BYTES, depth: 1, ..Default::default() }
    }

    pub(crate) fn candidate_views_demands(&self) -> RetirementDemand {
        RetirementDemand { copy_bytes: OWNED_DOCUMENT_MEMBER_PREPARATION_BYTES, depth: 1, ..Default::default() }
    }

    pub(crate) fn drive_member_open(&mut self,cx:&mut semio_framework_job::StepContext<'_>, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, crate::PluginLifecycleFailure> {
        let idle = RetainedCloneProgress::default();
        let item = RetainedCloneProgress { copied_items: 1, ..idle };
        if grant.maximum_items == 0 { return Ok(PluginLifecycleStep::Progress(idle)); }
        if self.opened_member.is_some(){
            if let Some(open)=self.active_member_open.as_mut(){
                if store::MemberOpenOperation::terminal_is_empty(open){drop(self.active_member_open.take());return Ok(PluginLifecycleStep::Progress(item))}
                let parent=cx.original_source_parent()?;let before=cx.retained_progress();cx.with_retained_loan(grant,|same|store::MemberOpenOperation::close_step(open,parent,same))?;
                return Ok(PluginLifecycleStep::Progress(crate::bounded_ownership::original_lifecycle_delta(before,cx.retained_progress())?));
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
        Ok(PluginLifecycleStep::Progress(crate::bounded_ownership::original_lifecycle_delta(before,cx.retained_progress())?))
    }

    fn publish_opened_member(&mut self,member:M)->Result<PluginLifecycleStep,crate::PluginLifecycleFailure>{
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

    pub(crate) fn drive_closure(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
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

    pub(crate) fn drive_candidate_views(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
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
    fn replacement_member_retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
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

    /// 🧹️ Keeps original custody until every child and separate parent transfer is admitted.
    pub(crate) fn drive_rejected_members(&mut self, current_content: &ChildContentView,cx:&mut semio_framework_job::StepContext<'_>, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, crate::PluginLifecycleFailure> {
        let demand = self.replacement_member_retirement_demands(grant.maximum_copy_bytes).map_err(crate::PluginLifecycleFailure::from)?;
        if grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes || grant.maximum_depth < demand.depth { return Ok(PluginLifecycleStep::Progress(Default::default())); }
        let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
        let paid = || PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes: demand.release_bytes, ..Default::default() });
        let parent = |step: RetainedCloneStep, terminal: bool| -> Result<PluginLifecycleStep, crate::PluginLifecycleFailure> {
            let admitted = semio_framework_value::retained_clone::admit_retained_clone_close(child,step,terminal,"replacement original member child").map_err(crate::PluginLifecycleFailure::from)?;
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
            let step = ingress.close_step(child).map_err(crate::PluginLifecycleFailure::from)?; return parent(step,ingress.terminal_is_empty());
        }
        if let Some(registry) = self.member_ingress.as_mut() {
            if registry.terminal_is_empty() { drop(self.member_ingress.take()); return Ok(paid()); }
            let step = registry.close_step(child)?; return parent(step,registry.terminal_is_empty());
        }
        if let Some(registry) = self.displaced_content_retirements.as_mut() {
            let children = self.candidate_children.as_mut().ok_or_else(|| plugin_sdk_fault("replacement original displaced roots lost their member registry"))?;
            let current = self.candidate_content.as_ref().ok_or_else(|| plugin_sdk_fault("replacement original displaced roots lost their current root"))?;
            if original_child_root_retirement_demands::<M>(registry,children,None,self.displaced_content_retirement_cursor,child.maximum_copy_bytes,true).map_err(crate::PluginLifecycleFailure::from)?.is_none() { drop(self.displaced_content_retirements.take()); return Ok(paid()); }
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
            let step = semio_framework_value::retained_clone::admit_retained_clone_close(inner,step,retirement.terminal_is_empty(),"replacement original candidate root").map_err(crate::PluginLifecycleFailure::from)?;
            let receipt = step.progress();
            return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_bytes: receipt.copied_bytes + reserved, ..receipt }));
        }
        if self.candidate_content.is_some() { let content = self.candidate_content.take().expect("original candidate root remains retained"); *self.candidate_content_retirement = Some(ChildContentRetirement::new(content,true)); return Ok(paid()); }
        if let Some(composition) = self.candidate_composition.as_mut() {
            if composition.terminal_is_empty() { drop(self.candidate_composition.take()); return Ok(paid()); }
            let step = composition.close_step(child).map_err(crate::PluginLifecycleFailure::from)?; return parent(step,composition.terminal_is_empty());
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
                Err(semio_framework_async::CancelTokenRetirementError::Refused(error)) => Err(crate::PluginLifecycleFailure::from(error)),
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

    pub(crate) fn drive_initializer(&mut self,cx:&mut semio_framework_job::StepContext<'_>, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, crate::PluginLifecycleFailure> {
        if self.state!=ActiveArtifactStoreReplacementState::Initializing{return Ok(PluginLifecycleStep::AwaitingInput{reason:"store initializer awaits its actual replacement stage"});}
        if grant.maximum_items==0||grant.maximum_depth==0{return Ok(PluginLifecycleStep::Progress(Default::default()));}
        let business_cancelled=self.cancelled();
        if let Some(rejected)=self.session_rejected.as_mut(){
            if rejected.admission_pending()&&!cx.is_cancelled()&&!business_cancelled{
                let config=semio_framework_job::BatchDriveConfig{retained:grant,site:"artifact_store_replacement",stage:semio_framework_job::InteractiveStage::InteractiveStep,fuel_per_step:cx.fuel_remaining(),step_budget_us:0};
                let before=cx.retained_progress();
                if let Some(session)=rejected.admit_pending(config,cx)?{*self.session=Some(session);drop(self.session_rejected.take());}
                return Ok(PluginLifecycleStep::Progress(crate::bounded_ownership::original_lifecycle_delta(before,cx.retained_progress())?));
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
            let demand=match session.retirement_demands(grant.maximum_copy_bytes){Ok(demand)=>demand,Err(semio_framework_job::WorkerJobDemandError::Contention(_))=>return Ok(PluginLifecycleStep::AwaitingInput{reason:"store initializer original quote awaits worker handback"}),Err(semio_framework_job::WorkerJobDemandError::Refused(error))=>return Err(error.into())};
            return original_worker_lifecycle(cx,demand,grant,"store initializer original terminal worker remains contended",|cx|{let step=session.close_step(cx)?;Ok((step,session.terminal_is_empty()))});
        }
        let session = self.session.as_mut().ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-store.initializer-session-missing"), "store initializer lost its retained session before terminal recovery"))?;
        match session.pump_one_for_interactive_turn(cx) {
            Ok(semio_framework_job::WorkerJobPoll::Outcome | semio_framework_job::WorkerJobPoll::Terminal) => {
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
            Ok(semio_framework_job::WorkerJobPoll::Submitted | semio_framework_job::WorkerJobPoll::Idle) => Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),
            Ok(_) => Ok(PluginLifecycleStep::AwaitingInput { reason: "store initializer awaits its mounted wake" }),
            Err(failure) => Err(failure.into()),
        }
    }

    /// 📏️ Quotes the exact next retirement turn of the current retiring state: one member owner, or the retained Store and its disposer.
    pub(crate) fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
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

    pub(crate) fn drive_retained_store(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
        if grant.maximum_items==0||grant.maximum_depth==0{return Ok(PluginLifecycleStep::Progress(Default::default()));}
        let Some(store)=self.retained_store.as_mut()else{return Ok(PluginLifecycleStep::AwaitingInput{reason:"replacement original cancellation and closure owners retain their physical authority"});};
        let Some(disposer)=self.retained_disposer.as_mut()else{
            if !store.close_owned_store_terminal_is_empty(){return Err(plugin_sdk_fault("retained replacement Store lost its exact granted disposer"));}
            let demand=RetirementDemand{copy_bytes:size_of_val(store)+size_of::<ActiveArtifactStoreReplacementState>(),release_bytes:size_of_val(&**store),depth:1,..Default::default()};
            if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(PluginLifecycleStep::Progress(Default::default()));}
            return crate::bounded_ownership::runtime_lifecycle_step(demand,grant,|_|{
                drop(self.retained_store.take());self.state=if self.member_owners_terminal_is_empty(){ActiveArtifactStoreReplacementState::Complete}else{ActiveArtifactStoreReplacementState::RetiringRejectedMembers};
                Ok((PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}),false))
            });
        };
        if disposer.terminal_is_empty(store){
            let demand=RetirementDemand{copy_bytes:size_of_val(disposer),release_bytes:size_of_val(&**disposer),depth:1,..Default::default()};
            if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(PluginLifecycleStep::Progress(Default::default()));}
            return crate::bounded_ownership::runtime_lifecycle_step(demand,grant,|_|{
                drop(self.retained_disposer.take());
                Ok((PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}),false))
            });
        }
        let demand=disposer.retirement_demands(store,grant.maximum_copy_bytes).map_err(|error|plugin_sdk_fault(error.to_string()))?;
        crate::bounded_ownership::runtime_lifecycle_step(demand,grant,|grant|{
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

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.state == ActiveArtifactStoreReplacementState::Complete && self.member_owners_terminal_is_empty() && self.session.is_none() && self.session_rejected.is_none() && !self.outcome_pending && self.retained_store.is_none() && self.retained_disposer.is_none()
    }
}

impl<P, Mutation, M> Drop for ActiveArtifactStoreReplacement<P, Mutation, M>
where
    P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,
    Mutation: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + store::Mutation<P> + OpBinary + OpText + Send + 'static,
    M: SpaceMember + MemberFactory,
{
    fn drop(&mut self) {
        assert!(
            std::thread::panicking() || self.terminal_is_empty(),
            "artifact store replacement reached Drop before initializer/candidate/displaced-store ownership was terminal empty"
        );
    }
}

pub(crate) struct ArtifactStoreReplacementAdmissionTarget<'a, A: ArtifactApp, M: SpaceMember + MemberFactory> {
    pub(crate) jobs: &'a mut ArtifactFixedRegistry<ActiveArtifactStoreReplacement<A::Snapshot, A::Mutation, M>>,
    pub(crate) operation: semio_framework_job::OperationId,
    pub(crate) generation: semio_framework_job::Generation,
    pub(crate) child_content_generation: u64,
    pub(crate) source_identity:(semio_framework_job::OperationId,semio_framework_job::Generation),
    pub(crate) actor: ActorId,
    pub(crate) marker: std::marker::PhantomData<fn() -> A>,
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
pub(crate) type AppProjectionCache<A> = ((u64, u64, u64, HistoryCommandFilter), std::sync::Arc<<A as ArtifactApp>::Snapshot>, std::sync::Arc<<A as ArtifactApp>::Config>, std::sync::Arc<HistoryView>);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ActiveDocumentArchiveLoadState {
    Pending,
    Running,
    Ready,
    Cancelled,
    Fault,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ActiveDocumentArchiveLoadPhase {
    DecodeParent,
    RetireParentHistoryAuxiliary,
    MergeParent,
    HydrateParent,
    AwaitingMembers,
    BeginMember,
    FillMember,
    RetireMemberSource,
    AdmitMember,
    SealMembers,
    AwaitReplacement,
    Terminal,
}

pub(crate) struct PendingDocumentArchiveMember {
    pub(crate) entry: Option<protocol::OwnedDocumentMemberPackEntry>,
    entry_retirement: Option<Box<dyn store::ErasedSnapshotRetirement>>,
    pages: Option<store::os_store::owned_schema_decode::OwnedSchemaDecodePages>,
    pub(crate) copied: usize,
    identity_field: u8,
}

impl PendingDocumentArchiveMember {
    fn empty_ingress() -> OwnedDocumentMemberIngress {
        OwnedDocumentMemberIngress { identity: ManuallyDrop::new(None), request: ManuallyDrop::new(None), identity_field: 0 }
    }

    pub(crate) fn new(entry: protocol::OwnedDocumentMemberPackEntry) -> Result<Self, (Fault, protocol::OwnedDocumentMemberPackEntry)> {
        let page_count = entry.envelope_pack.len().div_ceil(store::os_store::owned_schema_decode::OWNED_SCHEMA_DECODE_PAGE_BYTES);
        let pages = match store::os_store::owned_schema_decode::OwnedSchemaDecodePages::try_with_credits(store::os_store::owned_schema_decode::OwnedSchemaDecodeCredits { maximum_pages: page_count, maximum_bytes: entry.envelope_pack.len() }) {
            Ok(pages) => pages,
            Err(fault) => return Err((document_load_fault(DOCUMENT_LOAD_FAILED_CODE, format!("document archive member page authority was rejected: {fault:?}")), entry)),
        };
        Ok(Self { entry: Some(entry), entry_retirement: None, pages: Some(pages), copied: 0, identity_field: 0 })
    }

    pub(crate) fn fill_one_page(&mut self, maximum_bytes: usize) -> Result<bool, Fault> {
        let entry = self.entry.as_ref().ok_or_else(|| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "document archive member source owner is absent"))?;
        if self.copied == entry.envelope_pack.len() {
            return Ok(true);
        }
        let end = self.copied.saturating_add(store::os_store::owned_schema_decode::OWNED_SCHEMA_DECODE_PAGE_BYTES).min(entry.envelope_pack.len());
        let length = end - self.copied;
        if length > maximum_bytes {
            return Ok(false);
        }
        let page = store::os_store::owned_schema_decode::OwnedSchemaDecodePage::try_from_slice(&entry.envelope_pack[self.copied..end]).map_err(|_| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "document archive member page exceeded its fixed extent"))?;
        self.pages.as_mut().ok_or_else(|| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "document archive member page owner is absent"))?.admit_page(page).map_err(|_| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "document archive member page was rejected by its exact authority"))?;
        self.copied = end;
        Ok(self.copied == entry.envelope_pack.len())
    }

    /// 📏️ The source bytes still owned: payload truncation, then their whole physical backing.
    pub(crate) fn source_retirement_demands(&self) -> RetirementDemand {
        self.entry.as_ref().map_or_else(RetirementDemand::default, |entry| plugin_bytes_retirement_demand(&entry.envelope_pack))
    }

    pub(crate) fn retire_source_step(&mut self, grant: RetainedCloneGrant) -> RetainedCloneStep {
        let empty = RetainedCloneProgress::default();
        let Some(entry) = self.entry.as_mut() else { return RetainedCloneStep::Complete(empty) };
        match plugin_bytes_close(&mut entry.envelope_pack, grant) {
            Some(progress) => RetainedCloneStep::Progress(progress),
            None => RetainedCloneStep::Complete(empty),
        }
    }
    pub(crate) fn take_ingress(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle, actor: ActorId) -> Result<OwnedDocumentMemberIngress, (Fault, OwnedDocumentMemberIngress)> {
        let mut entry = self.entry.take().ok_or_else(|| (document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "document archive member identity owner is absent"), Self::empty_ingress()))?;
        if !entry.envelope_pack.is_empty() {
            self.entry = Some(entry);
            return Err((document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "document archive member source bytes remain live before identity transfer"), Self::empty_ingress()));
        }
        let mut pages = self.pages.take().ok_or_else(|| (document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "document archive member page owner is absent"), Self::empty_ingress()))?;
        if let Err(fault) = pages.seal() {
            self.entry = Some(entry);
            self.pages = Some(pages);
            return Err((document_load_fault(DOCUMENT_LOAD_FAILED_CODE, format!("document archive member pages could not seal: {fault:?}")), Self::empty_ingress()));
        }
        let reference = ArtifactRef {
            artifact_id: std::mem::take(&mut entry.reference.artifact_id),
            dialect: ArtifactDialect { artifact_kind: std::mem::take(&mut entry.reference.artifact_kind), standard: std::mem::take(&mut entry.reference.standard), subset: std::mem::take(&mut entry.reference.subset) },
        };
        let owner = store::OwnerRef {
            parent: ArtifactRef {
                artifact_id: std::mem::take(&mut entry.owner.parent.artifact_id),
                dialect: ArtifactDialect { artifact_kind: std::mem::take(&mut entry.owner.parent.artifact_kind), standard: std::mem::take(&mut entry.owner.parent.standard), subset: std::mem::take(&mut entry.owner.parent.subset) },
            },
            slot: std::mem::take(&mut entry.owner.slot),
            child_id: std::mem::take(&mut entry.owner.child_id),
        };
        let ordinal = entry.ordinal as usize;
        drop(entry);
        let request = match store::MemberOpenRequest::new(handle.operation, handle.generation, u64::MAX, reference.clone(), Some(owner.clone()), pages, actor).admit(1) {
            Ok(request) => request,
            Err(rejected) => {
                let ingress = OwnedDocumentMemberIngress { identity: ManuallyDrop::new(Some((ordinal, reference, owner))), request: ManuallyDrop::new(Some(rejected.request)), identity_field: 0 };
                return Err((document_load_fault(DOCUMENT_LOAD_FAILED_CODE, format!("document archive member request was rejected: {:?}", rejected.diagnostic)), ingress));
            }
        };
        OwnedDocumentMemberIngress::try_new(ordinal, reference, owner, request)
    }

    /// 📏️ Borrows original member and whole page-directory demands before any transfer.
    pub(crate) fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(pages)=self.pages.as_ref() {
            return Ok(if let Some(bytes)=pages.close_page_bytes() { RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()} }
                else { RetirementDemand{release_bytes:pages.allocation_byte_demand(),depth:1,..Default::default()} });
        }
        if let Some(owner)=self.entry_retirement.as_ref() {
            let mut demand=store::os_store::snapshot_read::artifact_retirement_box_demands(owner,body)?;
            demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"archive member retirement parent depth overflow"))?;
            return Ok(demand);
        }
        Ok(if self.entry.is_some() { RetirementDemand{capacity_bytes:semio_framework_value::retirement::owned_retirement_birth_bytes::<protocol::OwnedDocumentMemberPackEntry>(),depth:2,..Default::default()} } else { Default::default() })
    }

    /// 🧹️ Pays inline payload work and releases each original physical allocation separately.
    pub(crate) fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty=RetainedCloneProgress::default();
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty));}
        let demand=self.retirement_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(empty));}
        if let Some(pages)=self.pages.as_mut() {
            if let Some(page)=pages.close_take_page(){let bytes=page.len();drop(page);return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..empty}));}
            if !pages.terminal_is_empty(){return pages.close_backing_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
            drop(self.pages.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}));
        }
        let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};
        if self.entry_retirement.is_some(){return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.entry_retirement,child).map(|step|RetainedCloneStep::Progress(step.progress()));}
        let original=self.entry.take().expect("quoted original archive member");
        match semio_framework_value::retirement::admit_owned_retirement(original,child) {
            Ok((owner,progress))=>{self.entry_retirement=Some(owner);semio_framework_value::retained_clone::admit_retained_clone_progress(child,progress,"original archive member birth")?;Ok(RetainedCloneStep::Progress(progress))}
            Err((error,original))=>{self.entry=Some(original);Err(error)}
        }
    }
    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.entry.is_none() && self.entry_retirement.is_none() && self.pages.is_none()
    }
}

impl Drop for PendingDocumentArchiveMember {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "recursive document archive member reached Drop before its source and page owners were terminal empty");
    }
}

fn document_archive_member_identity(entry: &protocol::OwnedDocumentMemberPackEntry, field: u8) -> Option<&String> {
    match field {
        0 => Some(&entry.reference.artifact_id),
        1 => Some(&entry.reference.artifact_kind),
        2 => Some(&entry.reference.standard),
        3 => Some(&entry.reference.subset),
        4 => Some(&entry.owner.parent.artifact_id),
        5 => Some(&entry.owner.parent.artifact_kind),
        6 => Some(&entry.owner.parent.standard),
        7 => Some(&entry.owner.parent.subset),
        8 => Some(&entry.owner.slot),
        9 => Some(&entry.owner.child_id),
        _ => None,
    }
}

fn document_archive_member_identity_mut(entry: &mut protocol::OwnedDocumentMemberPackEntry, field: u8) -> Option<&mut String> {
    match field {
        0 => Some(&mut entry.reference.artifact_id),
        1 => Some(&mut entry.reference.artifact_kind),
        2 => Some(&mut entry.reference.standard),
        3 => Some(&mut entry.reference.subset),
        4 => Some(&mut entry.owner.parent.artifact_id),
        5 => Some(&mut entry.owner.parent.artifact_kind),
        6 => Some(&mut entry.owner.parent.standard),
        7 => Some(&mut entry.owner.parent.subset),
        8 => Some(&mut entry.owner.slot),
        9 => Some(&mut entry.owner.child_id),
        _ => None,
    }
}

pub(crate) struct ActiveDocumentArchiveLoad<P, Mutation>
where
    P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    Mutation: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + store::Mutation<P>,
{
    pub(crate) operation: u64,
    pub(crate) archive: Option<Box<protocol::DocumentArchivePack>>,
    archive_retirement: Option<Box<dyn store::ErasedSnapshotRetirement>>,
    pub(crate) history: Option<protocol::RetainedHistoryDecode>,
    history_custody: Option<(Option<protocol::HistoryLog>, Option<protocol::HistoryLog>, Option<(Vec<String>, Vec<String>)>)>,
    pub(crate) decoded_history: Option<protocol::HistoryLog>,
    pub(crate) member: Option<PendingDocumentArchiveMember>,
    pub(crate) rejected_ingress: Option<OwnedDocumentMemberIngress>,
    pub(crate) retained: Option<Box<dyn store::ErasedSnapshotRetirement>>,
    pub(crate) retained_pending: Option<(Option<protocol::HistoryLog>, Option<(Vec<String>, Vec<String>)>)>,
    pub(crate) hydration: Option<store::RetainedPersistedDocumentHydration<P, Mutation>>,
    pub(crate) hydration_sequence: u64,
    pub(crate) replacement: Option<ArtifactEnvelopeDecodeOperationHandle>,
    pub(crate) genesis_complete: bool,
    pub(crate) phase: ActiveDocumentArchiveLoadPhase,
    pub(crate) terminal_target: Option<ActiveDocumentArchiveLoadState>,
    close_identity_field: u8,
    pub(crate) state: ActiveDocumentArchiveLoadState,
    pub(crate) completed: u64,
    pub(crate) total: u64,
    pub(crate) fold: (u64, u64),
    pub(crate) decoded: u64,
    pub(crate) merge: bool,
    pub(crate) ahead: u64,
    pub(crate) fault: Vec<u8>,
}

impl<P, Mutation> ActiveDocumentArchiveLoad<P, Mutation>
where
    P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + Send + Sync + 'static,
    Mutation: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + store::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    pub(crate) fn new(operation: u64, archive: protocol::DocumentArchivePack, history: protocol::RetainedHistoryDecode) -> Self {
        let total = archive.members.len() as u64 + 1;
        Self {
            operation,
            archive: Some(Box::new(archive)),
            archive_retirement: None,
            history: Some(history),
            history_custody: None,
            decoded_history: None,
            member: None,
            rejected_ingress: None,
            retained: None,
            retained_pending: None,
            hydration: None,
            hydration_sequence: 0,
            replacement: None,
            genesis_complete: false,
            phase: ActiveDocumentArchiveLoadPhase::DecodeParent,
            terminal_target: None,
            close_identity_field: 0,
            state: ActiveDocumentArchiveLoadState::Pending,
            completed: 0,
            total,
            fold: (0, 0),
            decoded: 0,
            merge: false,
            ahead: 0,
            fault: Vec::new(),
        }
    }

    pub(crate) fn status(&self) -> protocol::DocumentArchiveLoadStatus {
        let state = match self.state {
            ActiveDocumentArchiveLoadState::Pending => protocol::DocumentArchiveLoadState::Pending,
            ActiveDocumentArchiveLoadState::Running => protocol::DocumentArchiveLoadState::Running,
            ActiveDocumentArchiveLoadState::Ready => protocol::DocumentArchiveLoadState::Ready,
            ActiveDocumentArchiveLoadState::Cancelled => protocol::DocumentArchiveLoadState::Cancelled,
            ActiveDocumentArchiveLoadState::Fault => protocol::DocumentArchiveLoadState::Fault,
        };
        protocol::DocumentArchiveLoadStatus { operation: self.operation, state, completed: self.completed.saturating_add(self.fold.0), total: self.total.saturating_add(self.fold.1), ahead: self.ahead, fault: self.fault.clone() }
    }

    pub(crate) fn terminal(&self) -> bool {
        matches!(self.state, ActiveDocumentArchiveLoadState::Ready | ActiveDocumentArchiveLoadState::Cancelled | ActiveDocumentArchiveLoadState::Fault)
    }

    /// ⏳️ Whether the document is still being loaded: not once the load ended, not while a cancelled, faulted or
    /// committed load only retires what it held — the document is then already the one it will stay — and never for a
    /// read-back merge, which replaces nothing.
    pub(crate) fn loading(&self) -> bool {
        !self.merge && !self.terminal() && self.terminal_target.is_none()
    }

    /// 🫙️ Every original input owner is returned; only the encoded fault diagnostic the terminal Fault status carries may remain.
    pub(crate) fn inputs_are_empty(&self) -> bool {
        self.archive.is_none() && self.archive_retirement.is_none() && self.history.is_none() && self.history_custody.is_none() && self.decoded_history.is_none() && self.member.is_none() && self.rejected_ingress.is_none() && self.retained.is_none() && self.hydration.is_none() && self.replacement.is_none()
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.inputs_are_empty() && self.fault.capacity() == 0
    }

    pub(crate) fn request_terminal(&mut self, target: ActiveDocumentArchiveLoadState) {
        if self.terminal_target.is_none() {
            self.terminal_target = Some(target);
        }
    }

    /// 🔀️ Whether a read-back merge decoded its log and waits for the host's poll turn to merge it.
    pub(crate) fn awaits_merge(&self) -> bool {
        self.phase == ActiveDocumentArchiveLoadPhase::MergeParent && self.terminal_target.is_none()
    }

    pub(crate) fn request_fault(&mut self, fault: &Fault) {
        if self.terminal_target.is_none() {
            self.fault = semio_framework_diagnostic::encode_fault_bytes(fault);
            self.terminal_target = Some(ActiveDocumentArchiveLoadState::Fault);
        }
    }
    /// 📏️ Quotes complete original child currency before history or archive ownership moves.
    pub(crate) fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        let parent=|mut demand:RetirementDemand|->Result<RetirementDemand,ValueError>{demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"archive retirement parent depth overflow"))?;Ok(demand)};
        if self.replacement.is_some(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"archive replacement awaits its original operation recipient"));}
        if let Some(owner)=self.retained.as_ref(){return parent(store::os_store::snapshot_read::artifact_retirement_box_demands(owner,body)?);}
        if let Some(owner)=self.archive_retirement.as_ref(){return parent(store::os_store::snapshot_read::artifact_retirement_box_demands(owner,body)?);}
        if let Some(hydration)=self.hydration.as_ref(){
            if store::ErasedSnapshotRetirement::terminal_is_empty(hydration){return Ok(RetirementDemand{depth:1,..Default::default()});}
            return parent(RetirementDemand{copy_bytes:store::ErasedSnapshotRetirement::next_copy_byte_demand(hydration)?,capacity_bytes:store::ErasedSnapshotRetirement::next_capacity_byte_demand(hydration,body)?,release_bytes:store::ErasedSnapshotRetirement::next_release_byte_demand(hydration)?,depth:store::ErasedSnapshotRetirement::next_depth_demand(hydration)?});
        }
        if let Some(ingress)=self.rejected_ingress.as_ref(){return if ingress.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{parent(ingress.retirement_demands(body)?)};}
        if let Some(member)=self.member.as_ref(){return if member.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{parent(member.retirement_demands(body)?)};}
        if self.history_custody.is_some()||self.history.is_some()||self.decoded_history.is_some(){return Ok(RetirementDemand{capacity_bytes:semio_framework_value::retirement::owned_retirement_birth_bytes::<(Option<protocol::HistoryLog>,Option<protocol::HistoryLog>,Option<(Vec<String>,Vec<String>)>)>(),depth:2,..Default::default()});}
        if self.archive.is_some(){return Ok(RetirementDemand{capacity_bytes:semio_framework_value::retirement::owned_retirement_birth_bytes::<Box<protocol::DocumentArchivePack>>(),depth:2,..Default::default()});}
        Ok(Default::default())
    }

    /// 🎟️ Retains all original history and physical archive backing until full admission succeeds.
    pub(crate) fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty=RetainedCloneProgress::default();
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty));}
        let demand=self.retirement_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(empty));}
        let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};
        if self.retained.is_some(){return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retained,child).map(|step|RetainedCloneStep::Progress(step.progress()));}
        if self.archive_retirement.is_some(){return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.archive_retirement,child).map(|step|RetainedCloneStep::Progress(step.progress()));}
        if let Some(hydration)=self.hydration.as_mut(){
            if store::ErasedSnapshotRetirement::terminal_is_empty(hydration){drop(self.hydration.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}));}
            let step=store::ErasedSnapshotRetirement::close_step(hydration,child)?;
            semio_framework_value::retained_clone::admit_retained_clone_close(child,step,store::ErasedSnapshotRetirement::terminal_is_empty(hydration),"original archive hydration")?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(ingress)=self.rejected_ingress.as_mut(){
            if ingress.terminal_is_empty(){drop(self.rejected_ingress.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}));}
            let step=ingress.close_step(child)?;semio_framework_value::retained_clone::admit_retained_clone_close(child,step,ingress.terminal_is_empty(),"original archive ingress")?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(member)=self.member.as_mut(){
            if member.terminal_is_empty(){drop(self.member.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}));}
            let step=member.close_step(child)?;semio_framework_value::retained_clone::admit_retained_clone_close(child,step,member.terminal_is_empty(),"original archive pending member")?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.history_custody.is_none()&&(self.history.is_some()||self.decoded_history.is_some()){
            let partial=self.history.as_mut().and_then(protocol::RetainedHistoryDecode::take_partial);
            let auxiliary=self.history.as_mut().map(protocol::RetainedHistoryDecode::take_auxiliary_owners);
            self.history_custody=Some((self.decoded_history.take(),partial,auxiliary));
            if self.history.as_ref().is_some_and(|decoder|!decoder.terminal_is_empty()){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"archive decoder retained an untransferred original owner"));}
            drop(self.history.take());
        }
        if let Some(original)=self.history_custody.take(){
            match semio_framework_value::retirement::admit_owned_retirement(original,child){
                Ok((owner,progress))=>{self.retained=Some(owner);semio_framework_value::retained_clone::admit_retained_clone_progress(child,progress,"original archive history birth")?;return Ok(RetainedCloneStep::Progress(progress));}
                Err((error,original))=>{self.history_custody=Some(original);return Err(error);}
            }
        }
        if let Some(original)=self.archive.take(){
            return match semio_framework_value::retirement::admit_owned_retirement(original,child){
                Ok((owner,progress))=>{self.archive_retirement=Some(owner);semio_framework_value::retained_clone::admit_retained_clone_progress(child,progress,"original physical archive birth")?;Ok(RetainedCloneStep::Progress(progress))}
                Err((error,original))=>{self.archive=Some(original);Err(error)}
            };
        }
        Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"archive close lost its quoted original frontier"))
    }
}

/// 🧭️ Selects the original archive operation eligible for one close turn.
fn original_document_archive_close_slot<P,Mutation>(document_archive_loads:&ArtifactFixedRegistry<ActiveDocumentArchiveLoad<P,Mutation>>,closing:bool)->Option<u64>
where P:Clone+semio_framework_value::ToValue+semio_framework_value::FromValue,Mutation:Clone+semio_framework_value::ToValue+semio_framework_value::FromValue+store::Mutation<P>{
    (0..ARTIFACT_LIVE_OUTPUT_SLOTS).find_map(|index|document_archive_loads.entry(index).filter(|(_,active)|closing||active.terminal_target.is_some()&&!matches!(active.state,ActiveDocumentArchiveLoadState::Ready|ActiveDocumentArchiveLoadState::Cancelled|ActiveDocumentArchiveLoadState::Fault)).map(|(operation,_)|*operation))
}

/// 📏️ Borrows the exact archive frontier and separate original registry backing.
pub(crate) fn original_document_archive_close_demands<P,Mutation>(document_archive_loads:&ArtifactFixedRegistry<ActiveDocumentArchiveLoad<P,Mutation>>,body:usize,closing:bool)->Result<Option<RetirementDemand>,ValueError>
where P:Clone+semio_framework_value::ToValue+semio_framework_value::FromValue+ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot+Send+Sync+'static,Mutation:Clone+semio_framework_value::ToValue+semio_framework_value::FromValue+store::Mutation<P>+OpBinary+OpText+Send+'static{
    if let Some(operation)=original_document_archive_close_slot(document_archive_loads,closing){
        let active=document_archive_loads.get(operation).expect("original archive close slot");
        return if active.terminal()||active.terminal_target.is_none()||active.inputs_are_empty(){Ok(Some(RetirementDemand{release_bytes:if active.terminal(){active.fault.capacity()}else{0},depth:1,..Default::default()}))}else{active.retirement_demands(body).map(Some)};
    }
    Ok(if closing{document_archive_loads.empty_backing_byte_demand().filter(|bytes|*bytes!=0).map(|release_bytes|RetirementDemand{release_bytes,depth:1,..Default::default()})}else{None})
}

/// 🎟️ Yields actual child progress before terminal status, slot removal, or backing release.
pub(crate) fn advance_original_document_archive_close<P,Mutation>(document_archive_loads:&mut ArtifactFixedRegistry<ActiveDocumentArchiveLoad<P,Mutation>>,grant:RetainedCloneGrant,closing:bool)->Result<RetainedCloneStep,ValueError>
where P:Clone+semio_framework_value::ToValue+semio_framework_value::FromValue+ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot+Send+Sync+'static,Mutation:Clone+semio_framework_value::ToValue+semio_framework_value::FromValue+store::Mutation<P>+OpBinary+OpText+Send+'static{
    let idle=RetainedCloneProgress::default();
    let Some(demand)=original_document_archive_close_demands(document_archive_loads,grant.maximum_copy_bytes,closing)?else{return Ok(RetainedCloneStep::Complete(idle));};
    if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(idle));}
    let Some(operation)=original_document_archive_close_slot(document_archive_loads,closing)else{return document_archive_loads.close_empty_backing_granted(grant);};
    let active=document_archive_loads.get_mut(operation).expect("original archive close slot");
    if active.terminal(){
        if !active.inputs_are_empty(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"terminal archive retained an original physical owner"));}
        let released_bytes=active.fault.capacity();
        if released_bytes!=0{drop(std::mem::take(&mut active.fault));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes,..idle}));}
        drop(document_archive_loads.remove(operation));
        return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..idle}));
    }
    if active.terminal_target.is_none(){active.request_terminal(ActiveDocumentArchiveLoadState::Cancelled);return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..idle}));}
    if active.inputs_are_empty(){
        let target=active.terminal_target.expect("original archive terminal target");active.state=target;active.phase=ActiveDocumentArchiveLoadPhase::Terminal;
        return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..idle}));
    }
    let step=active.close_step(grant)?;
    semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,active.terminal_is_empty(),"original scheduled archive close")?;
    Ok(RetainedCloneStep::Progress(step.progress()))
}

impl<P, Mutation> Drop for ActiveDocumentArchiveLoad<P, Mutation>
where
    P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    Mutation: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + store::Mutation<P>,
{
    fn drop(&mut self) {
        assert!(
            std::thread::panicking()
                || (self.archive.is_none()
                    && self.archive_retirement.is_none()
                    && self.history.is_none()
                    && self.history_custody.is_none()
                    && self.decoded_history.is_none()
                    && self.member.is_none()
                    && self.rejected_ingress.is_none()
                    && self.retained.is_none()
                    && self.retained_pending.is_none()
                    && self.hydration.is_none()
                    && self.replacement.is_none()
                    && self.fault.capacity() == 0),
            "recursive document archive operation reached Drop before its retained input owner was terminal empty",
        );
    }
}

#[path="../🌱️initialization/🏪️catalog/🦀️.rs"]
mod original_catalog_constructor;

pub use original_catalog_constructor::{ArtifactStoreConstructorKind,ArtifactStoreConstructorPhase};

pub(crate) use original_catalog_constructor::OriginalCatalogConstructor;
