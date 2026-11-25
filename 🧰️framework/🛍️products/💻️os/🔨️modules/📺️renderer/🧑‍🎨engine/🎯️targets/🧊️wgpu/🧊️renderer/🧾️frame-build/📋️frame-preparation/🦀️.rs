//! 📋️ Frame preparation: the preparation cursor and its presentation counterpart, beside the build cursors that own them so a type and every inherent impl of it share a crate; the app-runtime crate of the renderer partition (plan section 5.2).
use super::*;

//#region 📮️RuntimeMailbox
/// 🧾️ A worker turn receipt excludes the return span its outcome reserved on the caller, so the caller's already posted total is the receipt plus that reserved delta.
pub fn posted_receipt(before: semio_framework_job::RetainedCloneProgress, receipt: semio_framework_job::RetainedCloneProgress, held_before: semio_framework_job::RetainedCloneProgress, cx: &semio_framework_job::StepContext<'_>) -> bool {
    let held_after = cx.reserved_return_progress();
    let delta = |after: usize, before: usize| after.checked_sub(before);
    let (Some(copied_items), Some(copied_bytes), Some(retained_capacity_bytes), Some(released_bytes)) = (
        delta(held_after.copied_items, held_before.copied_items),
        delta(held_after.copied_bytes, held_before.copied_bytes),
        delta(held_after.retained_capacity_bytes, held_before.retained_capacity_bytes),
        delta(held_after.released_bytes, held_before.released_bytes),
    ) else { return false };
    let reserved = semio_framework_job::RetainedCloneProgress { copied_items, copied_bytes, retained_capacity_bytes, released_bytes };
    before.checked_add(receipt).and_then(|total| total.checked_add(reserved)).is_ok_and(|total| total == cx.retained_progress())
}

#[derive(Clone,Copy,PartialEq,Eq)]
pub enum FramePreparationStep { Pending, Complete, Cancelled, Fault }

pub struct AppFramePreparation {
    pub retained: semio_framework_job::RetainedCloneGrant,
    pub params: Option<semio_framework_job_worker_authority::BatchJobParams>,
    pub params_cancel_retirement: Option<semio_framework_async::CancelTokenRetirement>,
    pub worker_semantic_pending: bool,
    pub outcome_pending: Option<FramePreparationStep>,
    pub outcome_acknowledged: bool,
    pub refused_receipt: Option<(RetainedCloneGrant, RetainedCloneProgress)>,
    pub job: Option<semio_ui_prepared::PreparedRenderJob>,
    pub job_rejected: Option<semio_ui_prepared::PreparedRenderJobRejected>,
    pub input_candidate: Option<semio_renderer_wgpu_shell_input::PresentedInputCandidateWitness>,
    pub session: Option<semio_framework_job_worker::BatchJobSession<semio_ui_prepared::PreparedRenderJob>>,
    pub admission: semio_framework_job_worker::WorkerJobSessionAdmission<semio_ui_prepared::PreparedRenderJob,semio_framework_job_worker::BatchJobSession<semio_ui_prepared::PreparedRenderJob>>,
    pub engine_packets: Option<FrameEnginePackets>,
    pub generation: semio_framework_trace::Generation,
    pub cursor: SemioCursor,
    pub theme_dark: bool,
    pub fullscreen: Option<bool>,
    pub cursor_wake: Option<infinite_world::world::WorldCursorWakeToken>,
    #[cfg(not(target_arch = "wasm32"))]
    pub job_progress: Option<kernel_runtime::JobProgressPresentationLease>,
    pub terminal: bool,
    pub fault: Option<&'static str>,
}

/// 🪙️ Compares actual child demand with the five unchanged incoming frame axes.
fn frame_retirement_permits(grant: semio_framework_job::RetainedCloneGrant, demand: semio_framework_value::RetirementDemand) -> bool {
    grant.maximum_items != 0 && demand.copy_bytes <= grant.maximum_copy_bytes && demand.capacity_bytes <= grant.maximum_capacity_bytes && demand.release_bytes <= grant.maximum_release_bytes && demand.depth <= grant.maximum_depth
}

impl AppFramePreparation {
    /// 🩺️ Why this preparation refused, for the ONE caller that must not swallow it.
    ///
    /// 🩸️ Every refusal here became an empty `StepOutcome::Fault`/`Cancelled`, and `ActiveFrameBuild`
    /// answered it by cancelling its own token — no fault recorded, no log, no quarantine. A single
    /// unbound raster producer therefore ended every frame build after the reference image decoded and
    /// the shell presented ONE frame per boot with a completely silent console (ticket
    /// 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w7b-presenter-one-frame-per-boot.md`).
    pub fn fault(&self) -> Option<&'static str> {
        self.fault
    }

    pub fn callback_verdict(&self) -> Option<&semio_framework_trace::CallbackVerdict> {
        self.session.as_ref().and_then(semio_framework_job_worker::BatchJobSession::callback_verdict)
    }

    pub fn drive_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<FramePreparationStep, semio_framework_job::OriginalTurnFailure> {
        use semio_framework_job::JobOutcomeView;
        use semio_framework_job_worker::WorkerJobPoll;
        use semio_framework_value::{ValueError,ValueRefusalKind};
        let refuse=|message|ValueError::literal(ValueRefusalKind::InvariantViolated,message);
        if self.refused_receipt.is_some() { return Err(refuse("original prepared worker receipt remains refused").into()); }
        if self.job_rejected.is_some() { return Err(refuse("original prepared render admission remains rejected").into()); }
        if self.job.is_some() && self.params.is_none() {
            let mut cancel = None;
            let admitted = cx.admit_original_cancel_alias(&mut cancel);
            if let Some(cancel) = cancel {
                if let Some(job)=self.job.as_mut(){job.bind_business_generation(cx.generation().0);}
                self.params = Some(semio_framework_job_worker_authority::BatchJobParams { operation:cx.operation(),generation:cx.generation(),cancel,config:semio_framework_job_worker_authority::BatchDriveConfig { retained:self.retained,site:"os_renderer.prepare.worker",stage:semio_framework_job::InteractiveStage::BackgroundStep,fuel_per_step:1,step_budget_us:1000 },now_us:cx.clock_source() });
            }
            admitted?;
            return Ok(FramePreparationStep::Pending);
        }
        if self.job.is_some() || !self.admission.terminal_is_empty() {
            self.admission.advance(&mut self.job, &mut self.params, &mut self.session, cx)?;
            return Ok(FramePreparationStep::Pending);
        }
        let session=self.session.as_mut().ok_or_else(||refuse("original prepared render session is absent"))?;
        if let Some(outcome)=self.outcome_pending {
            if !self.outcome_acknowledged {
                self.outcome_acknowledged=session.acknowledge_outcome(cx)?;
                return Ok(FramePreparationStep::Pending);
            }
            if outcome==FramePreparationStep::Pending {
                session.resume().map_err(|_|refuse("original prepared worker resume was refused"))?;
                self.worker_semantic_pending=false;self.outcome_pending=None;self.outcome_acknowledged=false;
                return Ok(FramePreparationStep::Pending);
            }
            self.terminal=true;
            return Ok(outcome);
        }
        if !self.worker_semantic_pending {
            let before=cx.retained_progress();let held_before=cx.reserved_return_progress();let expected=cx.retained_grant();
            if !matches!(session.step_on_original_caller(cx)?,WorkerJobPoll::Outcome|WorkerJobPoll::Terminal) || !session.checkout_outcome() { return Ok(FramePreparationStep::Pending); }
            if let Some((issued,progress))=session.take_checked_out_retained_step_receipt() {
                self.worker_semantic_pending=true;
                let posted=posted_receipt(before,progress,held_before,cx);
                if issued!=expected || !progress.fits(issued) || !posted { self.refused_receipt=Some((issued,progress));return Err(refuse("original prepared worker tuple differs from its already posted original caller receipt").into()); }
                return Ok(FramePreparationStep::Pending);
            }
            return Err(refuse("original prepared worker omitted its checked-out physical receipt").into());
        }
        let fault_detail=session.checked_out_job().and_then(|job|job.fault_detail());
        self.outcome_pending=Some(match session.checked_out_outcome()? {
            None|Some(JobOutcomeView::Yield { .. })=>FramePreparationStep::Pending,
            Some(JobOutcomeView::Complete { .. })=>FramePreparationStep::Complete,
            Some(JobOutcomeView::Cancelled { .. })=>{self.fault=Some("prepared render job was cancelled");FramePreparationStep::Cancelled},
            Some(JobOutcomeView::Fault { .. }|JobOutcomeView::PreviewReady { .. }|JobOutcomeView::CheckpointReady { .. })=>{self.fault=Some(fault_detail.unwrap_or("prepared render job refused its terminal candidate"));FramePreparationStep::Fault},
        });
        Ok(FramePreparationStep::Pending)
    }

    pub fn take_presentation(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<AppFramePresentation>,semio_framework_value::ValueError> {
        if !self.terminal || !self.outcome_acknowledged {
            return Ok(None);
        }
        let grant=cx.retained_grant();
        if grant.maximum_items==0 || grant.maximum_depth==0 { return Ok(None); }
        if self.engine_packets.is_none() { return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original frame engine packets are absent")); }
        let Some(packet)=self.session.as_mut().and_then(|session|session.checked_out_job_mut()).and_then(|job|job.take_packet())else{return Ok(None)};
        cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;
        if let Some(session)=self.session.as_mut(){session.begin_close();}
        Ok(Some(AppFramePresentation {
            retained: self.retained,
            packet: Some(packet),
            input_candidate: self.input_candidate.take(),
            engine_packets: self.engine_packets.take().expect("original frame engine packets validated before publication"),
            generation: self.generation,
            cursor: self.cursor,
            theme_dark: self.theme_dark,
            fullscreen: self.fullscreen,
            cursor_wake: self.cursor_wake.take(),
            #[cfg(not(target_arch = "wasm32"))]
            job_progress: self.job_progress.take(),
        }))
    }

    pub fn discard_presented_input_candidate(&mut self, runtime: &AppMailbox) -> bool {
        discard_frame_input_candidate(runtime, &mut self.input_candidate)
    }

    pub fn close_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>, witness: Option<&semio_framework_job::CancelToken>) -> Result<InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure> {
        let grant=cx.retained_grant();
        if cx.should_yield(){return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
        if self.input_candidate.is_some() { return Ok(InteractiveJobCloseStep::Blocked); }
        if self.job_rejected.is_some() { return frame_receive_close(frame_unpriced_child(),cx); }
        if !self.admission.terminal_is_empty() { return frame_received_child(self.admission.close_step(cx), grant, self.admission.terminal_is_empty()); }
        if self.params.is_some() {
            if let Some(step)=frame_inline_gate(grant,0).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))?{return Ok(step)}
            let params=self.params.take().expect("original prepared params selected before paid transfer");
            self.params_cancel_retirement=Some(semio_framework_async::CancelTokenRetirement::from_token(params.cancel));
            return frame_receive_close(frame_inline_progress(0),cx);
        }
        if let Some(original)=self.params_cancel_retirement.as_mut() {
            if !original.terminal_is_empty() {
                let Some(witness)=witness else{return Ok(InteractiveJobCloseStep::Blocked)};
                let turn=cx.with_original_effect(grant,|_,effect|original.return_alias_step(witness,effect))?.map_or(semio_framework_value::RetainedWorkOutcome{result:Ok(semio_framework_value::RetainedCloneStep::Progress(Default::default())),work:0},|(turn,_)|turn);
                let blocked = matches!(&turn.result, Err(semio_framework_async::CancelTokenRetirementError::Blocked(_)));
                let result = match turn.result {
                    Ok(step) => Ok(step),
                    Err(semio_framework_async::CancelTokenRetirementError::Blocked(_)) => Ok(semio_framework_value::RetainedCloneStep::Progress(Default::default())),
                    Err(semio_framework_async::CancelTokenRetirementError::Refused(error)) => Err(error),
                };
                let step = semio_framework_job::receive_original_retained_work(cx, semio_framework_value::RetainedWorkOutcome { result, work: turn.work })?;
                return Ok(if blocked { InteractiveJobCloseStep::Blocked } else { InteractiveJobCloseStep::Pending { progress: step.progress() } });
            }
            if let Some(step)=frame_inline_gate(grant,0).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))?{return Ok(step)}
            self.params_cancel_retirement=None;
            return frame_receive_close(frame_inline_progress(0),cx);
        }
        if let Some(job) = self.job.as_mut() {
            use semio_framework_job::InteractiveJob;
            job.begin_close();
            if !job.terminal_is_empty() {
                return frame_received_child(job.close_step(cx), grant, job.terminal_is_empty());
            }
            let bytes = std::mem::size_of::<Option<semio_ui_prepared::PreparedRenderJob>>();
            if let Some(step) = frame_inline_gate(grant, bytes).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))? { return Ok(step); }
            self.job = None;
            return frame_receive_close(frame_inline_progress(bytes),cx);
        }
        if let Some(session) = self.session.as_mut() {
            if !matches!(session.poll(), semio_framework_job_worker::WorkerJobPoll::Closing | semio_framework_job_worker::WorkerJobPoll::TerminalEmpty) {
                session.begin_close();
                return Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() });
            }
            if let Some(witness)=witness {
                match session.has_original_cancel_alias_witness(witness) {
                    Ok(true) => {
                        let turn = cx.with_original_effect(grant, |cx, effect| session.return_original_cancel_alias_step(cx.original_cancel_token(), effect))?
                            .map_or(semio_framework_value::RetainedWorkOutcome { result: Ok(None), work: 0 }, |(turn, _)| turn);
                        let blocked = matches!(&turn.result, Ok(None) | Err(semio_framework_job_worker::WorkerJobDemandError::Contention(_)));
                        let result = match turn.result {
                            Ok(Some(step)) => Ok(step),
                            Ok(None) | Err(semio_framework_job_worker::WorkerJobDemandError::Contention(_)) => Ok(semio_framework_value::RetainedCloneStep::Progress(Default::default())),
                            Err(semio_framework_job_worker::WorkerJobDemandError::Refused(error)) => Err(error),
                        };
                        let step = semio_framework_job::receive_original_retained_work(cx, semio_framework_value::RetainedWorkOutcome { result, work: turn.work })?;
                        return Ok(if blocked { InteractiveJobCloseStep::Blocked } else { InteractiveJobCloseStep::Pending { progress: step.progress() } });
                    }
                    Ok(false)=>{},
                    Err(semio_framework_job_worker::WorkerJobDemandError::Contention(_))=>return Ok(InteractiveJobCloseStep::Blocked),
                    Err(semio_framework_job_worker::WorkerJobDemandError::Refused(error))=>return Err(semio_framework_job::OriginalTurnFailure::debit_original(error,cx)),
                }
            }
            if !session.terminal_is_empty() { let step=session.close_step(cx)?;return frame_worker_child_step(step, grant, session.terminal_is_empty()).map_err(semio_framework_job::OriginalTurnFailure::from); }
            let bytes = std::mem::size_of::<Option<semio_framework_job_worker::BatchJobSession<semio_ui_prepared::PreparedRenderJob>>>();
            if let Some(step) = frame_inline_gate(grant, bytes).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))? { return Ok(step); }
            self.session = None;
            return frame_receive_close(frame_inline_progress(bytes),cx);
        }
        if self.cursor_wake.is_some() {
            let bytes = std::mem::size_of::<Option<infinite_world::world::WorldCursorWakeToken>>();
            if let Some(step) = frame_inline_gate(grant, bytes).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))? { return Ok(step); }
            self.cursor_wake = None;
            return frame_receive_close(frame_inline_progress(bytes),cx);
        }
        #[cfg(not(target_arch = "wasm32"))]
        if self.job_progress.is_some() { return frame_receive_close(frame_unpriced_child(),cx); }
        if let Some(packets) = self.engine_packets.as_mut() {
            if !packets.terminal_is_empty() {
                if !packets.is_empty() { return frame_receive_close(frame_unpriced_child(),cx); }
                return frame_receive_close(frame_child_step(packets.close_backing_step(grant), grant, packets.terminal_is_empty()),cx);
            }
            let bytes = std::mem::size_of::<Option<FrameEnginePackets>>();
            if let Some(step) = frame_inline_gate(grant, bytes).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))? { return Ok(step); }
            self.engine_packets = None;
            return frame_receive_close(frame_inline_progress(bytes),cx);
        }
        if self.worker_semantic_pending || self.outcome_pending.is_some() || self.outcome_acknowledged || self.refused_receipt.is_some() {
            if let Some(step)=frame_inline_gate(grant,0).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))?{return Ok(step)}
            self.worker_semantic_pending=false;self.outcome_pending=None;self.outcome_acknowledged=false;self.refused_receipt=None;
            return frame_receive_close(frame_inline_progress(0),cx);
        }
        Ok(InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() })
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.params.is_none() && self.admission.terminal_is_empty() && self.params_cancel_retirement.is_none() && !self.worker_semantic_pending && self.outcome_pending.is_none() && !self.outcome_acknowledged && self.refused_receipt.is_none() && self.job.is_none() && self.job_rejected.is_none() && self.session.is_none() && self.engine_packets.is_none() && self.input_candidate.is_none() && self.cursor_wake.is_none() && {
            #[cfg(not(target_arch = "wasm32"))]
            {
                self.job_progress.is_none()
            }
            #[cfg(target_arch = "wasm32")]
            {
                true
            }
        }
    }
}

impl AppFramePresentation {
    pub fn discard_presented_input_candidate(&mut self, runtime: &AppMailbox) -> bool {
        discard_frame_input_candidate(runtime, &mut self.input_candidate)
    }

    /// 📏️ Quotes the next child in the presentation's retained close sequence.
    pub fn next_close_demands(&self, copy: usize) -> Result<RetainedCloneGrant, semio_framework_value::ValueError> {
        if self.input_candidate.is_some() {
            return Ok(RetainedCloneGrant::default());
        }
        if let Some(packet) = self.packet.as_ref() {
            return if packet.retirement_is_empty() {
                semio_ui_prepared::retirement::prepared_owner_demands(false, Ok(0))
            } else {
                packet.next_close_demands(copy)
            };
        }
        if !self.engine_packets.terminal_is_empty() {
            if !self.engine_packets.is_empty() {
                return frame_unpriced_child().map(|_| RetainedCloneGrant::default());
            }
            let demand = self.engine_packets.backing_retirement_demands();
            return Ok(RetainedCloneGrant {
                maximum_items: 1,
                maximum_copy_bytes: demand.copy_bytes,
                maximum_capacity_bytes: demand.capacity_bytes,
                maximum_release_bytes: demand.release_bytes,
                maximum_depth: demand.depth,
            });
        }
        semio_ui_prepared::retirement::prepared_owner_demands(self.cursor_wake.is_none(), Ok(0))
    }

    /// 🎟️ Closes one discarded presentation child under its original caller's effect.
    pub fn close_step_on_original(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<InteractiveJobCloseStep, semio_framework_job::OriginalTurnFailure> {
        let demand = self.next_close_demands(cx.retained_grant().maximum_copy_bytes)?;
        frame_prepared_close(cx, demand, |effect| self.close_step(effect))
    }

    pub fn close_step(&mut self, effect: &mut semio_framework_value::RetainedAllocationEffect<'_>) -> Result<InteractiveJobCloseStep,semio_framework_value::RetirementPollExit> {
        let grant = effect.grant();
        if self.input_candidate.is_some() { return Ok(InteractiveJobCloseStep::Blocked); }
        if let Some(packet) = self.packet.as_mut() {
            if !packet.retirement_is_empty() { return semio_ui_prepared::retirement::prepared_close_child(packet.close_step(effect)); }
            let bytes = std::mem::size_of::<Option<semio_ui_prepared::PreparedRenderPacket>>();
            if let Some(step) = frame_inline_gate(grant, bytes)? { return Ok(step); }
            self.packet = None;
            return Ok(frame_inline_progress(bytes)?);
        }
        if !self.engine_packets.terminal_is_empty() {
            if !self.engine_packets.is_empty() { return Ok(frame_unpriced_child()?); }
            return Ok(frame_child_step(self.engine_packets.close_backing_step(grant), grant, self.engine_packets.terminal_is_empty())?);
        }
        if self.cursor_wake.is_some() {
            let bytes = std::mem::size_of::<Option<infinite_world::world::WorldCursorWakeToken>>();
            if let Some(step) = frame_inline_gate(grant, bytes)? { return Ok(step); }
            self.cursor_wake = None;
            return Ok(frame_inline_progress(bytes)?);
        }
        #[cfg(not(target_arch = "wasm32"))]
        if self.job_progress.is_some() { return Ok(frame_unpriced_child()?); }
        Ok(InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() })
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.packet.is_none() && self.input_candidate.is_none() && self.engine_packets.terminal_is_empty() && self.cursor_wake.is_none() && {
            #[cfg(not(target_arch = "wasm32"))]
            {
                self.job_progress.is_none()
            }
            #[cfg(target_arch = "wasm32")]
            {
                true
            }
        }
    }
}
//#endregion 📮️RuntimeMailbox
