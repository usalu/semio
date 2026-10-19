//! 🏁️ Job close and interactive of the board fill: the granted close, the interactive job impl, the job drop and the send assertions. See [`super`].

use super::*;

// #region board_host
//#region 🪣️Fill
impl BoardFillJob {
    fn close_granted(&mut self,grant:semio_framework_value::RetainedCloneGrant)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_value::ValueError>{
        let maximum_items=grant.maximum_items;
        let maximum_bytes=grant.maximum_release_bytes;
        if grant.maximum_depth==0 {return Ok(semio_framework_job::InteractiveJobCloseStep::Pending{progress:Default::default()});}
        self.closing = true;
        if maximum_items == 0 {
            return Ok(semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() });
        }
        if self.state.is_none() {
            if let Some(mut checkpoint) = self.checkpoint.take() {
                self.state = checkpoint.state.take();
                return Ok(semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,..Default::default()} });
            }
        }
        if self.commit_encoder.take().is_some() {
            return Ok(semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,..Default::default()} });
        }
        if let Some(writer) = self.commit_writer.as_mut() {
            writer.begin_close();
            let step = writer.close_step(grant);
            if writer.terminal_is_empty() {self.commit_writer = None;}
            return Ok(match step {
                Ok(step)=>semio_framework_job::InteractiveJobCloseStep::Pending{progress:step.progress()},
                Err(error)=>return Err(error),
            });
        }
        for slot in [&mut self.lent, &mut self.commit_output] {
            if let Some(payload) = slot.as_mut() {
                if payload.terminal_is_empty() {
                    *slot = None;
                    return Ok(semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,..Default::default()} });
                }
                return Ok(match payload.close_step(grant) {
                    Ok(step) => semio_framework_job::InteractiveJobCloseStep::Pending { progress: step.progress() },
                    Err(error)=>return Err(error),
                });
            }
        }
        if self.preview.take().is_some() || self.fault.take().is_some() {
            return Ok(semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,..Default::default()} });
        }
        let Some(state) = self.state.as_mut() else { return Ok(semio_framework_job::InteractiveJobCloseStep::Complete {progress:Default::default()}) };
        if let Some(placement) = state.pending_placement.as_mut() {
            if placement.handles.pop().is_some() {
                return Ok(semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,..Default::default()} });
            }
            if let Some(bytes) = placement.handles.retained_page_bytes() {
                if bytes > maximum_bytes {
                    return Ok(semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() });
                }
                let Some(released_bytes) = placement.handles.retire_empty_page() else {
                    return Ok(semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() });
                };
                return Ok(semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,released_bytes,..Default::default()} });
            }
            if let Some(step)=placement.handles.retire_descriptor(grant){return Ok(step);}
            state.pending_placement = None;
            return Ok(semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,..Default::default()} });
        }
        if state.accept_handle_template.take().is_some()
            || state.accept_handle_id.take().is_some()
            || state.accept_handle_pending_virtual.take().is_some()
            || state.accept_handle_pending_placement.take().is_some()
            || state.accept_pending_virtual_node.take().is_some()
            || state.source_capture.take().is_some()
            || state.compatibility_candidate.take().is_some()
            || state.current_preview.take().is_some()
            || state.candidate_event.take().is_some()
            || state.rejection.take().is_some()
        {
            return Ok(semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,..Default::default()} });
        }
        if let Some(kind) = state.snapshot.kinds.len.checked_sub(1).and_then(|index| state.snapshot.kinds.get_mut(index)) {
            if kind.handles.pop().is_some() {
                return Ok(semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,..Default::default()} });
            }
            if let Some(bytes) = kind.handles.retained_page_bytes() {
                if bytes > maximum_bytes {
                    return Ok(semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() });
                }
                let Some(released_bytes) = kind.handles.retire_empty_page() else {
                    return Ok(semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() });
                };
                return Ok(semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,released_bytes,..Default::default()} });
            }
        }
        if let Some(kind)=state.snapshot.kinds.len.checked_sub(1).and_then(|index|state.snapshot.kinds.get_mut(index)){if let Some(step)=kind.handles.retire_descriptor(grant){return Ok(step);}}
        if state.snapshot.kinds.pop().is_some()
            || state.virtual_handles.pop().is_some()
            || state.virtual_nodes.pop().is_some()
            || state.candidates.pop().is_some()
            || state.sources.pop().is_some()
            || state.snapshot.rules.pop().is_some()
            || state.snapshot.handles.pop().is_some()
            || state.snapshot.nodes.pop().is_some()
        {
            return Ok(semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,..Default::default()} });
        }
        macro_rules! retire_page {
            ($owners:expr) => {
                if let Some(bytes) = $owners.retained_page_bytes() {
                    if bytes > maximum_bytes {
                        return Ok(semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() });
                    }
                    let Some(released_bytes) = $owners.retire_empty_page() else {
                        return Ok(semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() });
                    };
                    return Ok(semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,released_bytes,..Default::default()} });
                }
                if let Some(step)=$owners.retire_descriptor(grant){return Ok(step);}
            };
        }
        retire_page!(state.virtual_handles);
        retire_page!(state.virtual_nodes);
        retire_page!(state.candidates);
        retire_page!(state.sources);
        retire_page!(state.snapshot.rules);
        retire_page!(state.snapshot.handles);
        retire_page!(state.snapshot.nodes);
        retire_page!(state.snapshot.kinds);
        self.state = None;
        Ok(semio_framework_job::InteractiveJobCloseStep::Complete {progress:semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()}})
    
    }
}

impl semio_framework_job::InteractiveJob for BoardFillJob {
    fn step<'a>(&'a mut self, context: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
                self.validate_original_source(context)?;
if self.admitted.get() || self.offer.get().is_none() {
            self.admitted.set(false);
            let offer = self.next_offer(context);
            self.lent = match offer {
                BoardFillOffer::Preview => Some(semio_framework_job_payload::RetainedJobPayload::empty(semio_framework_job_payload::JobPayloadStream::Preview)),
                BoardFillOffer::Checkpoint(_) => Some(semio_framework_job_payload::RetainedJobPayload::empty(semio_framework_job_payload::JobPayloadStream::CheckpointState)),
                BoardFillOffer::Fault => Some(semio_framework_job_payload::RetainedJobPayload::empty(semio_framework_job_payload::JobPayloadStream::Fault)),
                BoardFillOffer::Yield | BoardFillOffer::Complete | BoardFillOffer::Cancelled => None,
            };
            self.offer.set(Some(offer));
        }
        self.present(context).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,context))
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        use semio_framework_job::JobOutcomeKind as Kind;
        let lent = || self.lent.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "fill outcome payload is absent"));
        match descriptor.kind() {
            Kind::Yield => descriptor.yielded(),
            Kind::Cancelled => descriptor.cancelled(),
            Kind::PreviewReady => descriptor.preview(lent()?),
            Kind::CheckpointReady { .. } => descriptor.checkpoint(lent()?),
            Kind::Fault => descriptor.fault(lent()?),
            Kind::Complete => descriptor.complete(None, self.commit_output.as_ref()),
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
                 self.validate_original_source(cx)?;
if self.terminal_is_empty(){return Ok(semio_framework_job::InteractiveJobCloseStep::Complete{progress:Default::default()})}
        let demand=(||->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{Ok(semio_framework_value::RetirementDemand{copy_bytes:self.next_close_copy_byte_demand()?,capacity_bytes:self.next_close_capacity_byte_demand(cx.retained_grant().maximum_copy_bytes)?,release_bytes:self.next_close_release_byte_demand()?,depth:self.next_close_depth_demand()?})})().map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))?;
        let mut blocked=false;
        let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|self.close_granted(grant).map(|step|match step{semio_framework_job::InteractiveJobCloseStep::Pending{progress}=>semio_framework_value::RetainedCloneStep::Progress(progress),semio_framework_job::InteractiveJobCloseStep::Complete{progress}=>semio_framework_value::RetainedCloneStep::Complete(progress),semio_framework_job::InteractiveJobCloseStep::Blocked=>{blocked=true;semio_framework_value::RetainedCloneStep::Progress(Default::default())}}))?;
        if blocked{return Ok(semio_framework_job::InteractiveJobCloseStep::Blocked)}
        if self.terminal_is_empty(){Ok(semio_framework_job::InteractiveJobCloseStep::Complete{progress:step.progress()})}else{Ok(step)}
    }

    fn next_close_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{
        if self.state.is_none()&&self.checkpoint.is_some()||self.commit_encoder.is_some(){return Ok(0);}
        if let Some(payload)=self.lent.as_ref().or(self.commit_output.as_ref()).filter(|_|self.commit_writer.is_none()){return payload.retirement_demands().map(|demand|demand.copy_bytes);}
        self.commit_writer.as_ref().map_or(Ok(0),|writer|writer.retirement_demands().map(|demand|demand.copy_bytes))
    }

    fn next_close_capacity_byte_demand(&self,_:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}

    fn next_close_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{
        if self.terminal_is_empty(){return Ok(0);}
        if self.state.is_none()&&self.checkpoint.is_some()||self.commit_encoder.is_some(){return Ok(1);}
        if let Some(payload)=self.lent.as_ref().or(self.commit_output.as_ref()).filter(|_|self.commit_writer.is_none()){return payload.retirement_demands().map(|demand|demand.depth);}
        self.commit_writer.as_ref().map_or(Ok(1),|writer|writer.retirement_demands().map(|demand|demand.depth))
    }

    fn next_close_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{
        if self.state.is_none()&&self.checkpoint.is_some()||self.commit_encoder.is_some(){return Ok(0);}
        if let Some(writer)=self.commit_writer.as_ref(){return writer.retirement_demands().map(|demand|demand.release_bytes);}
        if let Some(payload)=self.lent.as_ref().or(self.commit_output.as_ref()){return payload.retirement_demands().map(|demand|demand.release_bytes);}
        if self.preview.is_some()||self.fault.is_some(){return Ok(0);}
        let Some(state)=self.state.as_ref()else{return Ok(0)};
        if let Some(placement)=state.pending_placement.as_ref(){return Ok(placement.handles.next_release_bytes());}
        if state.accept_handle_template.is_some()||state.accept_handle_id.is_some()||state.accept_handle_pending_virtual.is_some()||state.accept_handle_pending_placement.is_some()||state.accept_pending_virtual_node.is_some()||state.source_capture.is_some()||state.compatibility_candidate.is_some()||state.current_preview.is_some()||state.candidate_event.is_some()||state.rejection.is_some(){return Ok(0);}
        if let Some(kind)=state.snapshot.kinds.len.checked_sub(1).and_then(|index|state.snapshot.kinds.get(index)){if !kind.handles.terminal_is_empty(){return Ok(kind.handles.next_release_bytes());}}
        if !state.snapshot.kinds.is_empty()||!state.virtual_handles.is_empty()||!state.virtual_nodes.is_empty()||!state.candidates.is_empty()||!state.sources.is_empty()||!state.snapshot.rules.is_empty()||!state.snapshot.handles.is_empty()||!state.snapshot.nodes.is_empty(){return Ok(0);}
        for bytes in [state.virtual_handles.next_release_bytes(),state.virtual_nodes.next_release_bytes(),state.candidates.next_release_bytes(),state.sources.next_release_bytes(),state.snapshot.rules.next_release_bytes(),state.snapshot.handles.next_release_bytes(),state.snapshot.nodes.next_release_bytes(),state.snapshot.kinds.next_release_bytes()]{if bytes!=0{return Ok(bytes);}}
        Ok(0)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.state.is_none() && self.checkpoint.is_none() && self.preview.is_none() && self.commit_encoder.is_none() && self.commit_writer.is_none() && self.lent.is_none() && self.commit_output.is_none() && self.fault.is_none()
    }
}

impl Drop for BoardFillJob {
    fn drop(&mut self) {
        assert!(semio_framework_job::InteractiveJob::terminal_is_empty(self) || std::thread::panicking(), "Board fill job must reach exact terminal-empty before Drop");
    }
}

pub const _: fn() = || {
    fn assert_send<T: Send>() {}
    assert_send::<BoardFillSnapshot>();
    assert_send::<BoardFillSnapshotCapture>();
    assert_send::<BoardFillSnapshotIngress>();
    assert_send::<BoardFillCheckpoint>();
    assert_send::<BoardFillPlacement>();
    assert_send::<BoardFillJob>();
};
//#endregion 🧵️FillJob
// #endregion board_host
