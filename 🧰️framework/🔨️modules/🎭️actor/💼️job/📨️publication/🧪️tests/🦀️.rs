use super::*;
use semio_framework_actor_retained_turn::{RetainedTurnInput,OriginalActorTurnWork};
use std::collections::VecDeque;

    fn bridge_operation() -> semio_framework_job::Operation {
        semio_framework_job::Operation::new(semio_framework_job::OperationId(91), semio_framework_job::RevisionId(7), semio_framework_job::Generation(3), 0x5eed)
    }

    fn bridge_turn(step_sequence: u64, preview_sequence: u64) -> JobTurn {
        let mut operation = bridge_operation();
        operation.preview_sequence = preview_sequence;
        JobTurn { job: 44, operation: JobOperation::from_job(operation), step_sequence }
    }

    fn declared_bridge_grant(field: &str) -> semio_framework_job::RetainedCloneGrant {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/📄️checkpoint/🔣️.json")).unwrap();
        let grant = &fixture[field];
        semio_framework_job::RetainedCloneGrant { maximum_items: grant["maximumItems"].as_u64().unwrap() as usize, maximum_copy_bytes: grant["maximumCopyBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: grant["maximumCapacityBytes"].as_u64().unwrap() as usize, maximum_release_bytes: grant["maximumReleaseBytes"].as_u64().unwrap() as usize, maximum_depth: grant["maximumDepth"].as_u64().unwrap() as usize }
    }

    fn original_bridge_grant() -> semio_framework_job::RetainedCloneGrant { declared_bridge_grant("originalGrant") }

    fn close_original_publication(publication: JobPublication) {
        let original = declared_bridge_grant("destinationCloseGrant");
        let mut receipt = semio_framework_job::RetainedCloneProgress::default();
        let mut owner = JobPublicationOwner::try_new(publication).expect("fixture original publication shape");
        while !owner.terminal_is_empty() {
            let release = match (&owner.publication.outcome, owner.phase) {
                (JobStepOutcome::PreviewReady { preview }, 0) => preview.capacity(),
                (JobStepOutcome::CheckpointReady { checkpoint }, 0) => checkpoint.state.capacity(),
                (JobStepOutcome::Complete { candidate }, 0) => candidate.state.capacity(),
                (JobStepOutcome::Complete { candidate }, 1) => candidate.output.capacity(),
                (JobStepOutcome::Fault { detail }, 0) => detail.capacity(),
                _ => 0,
            };
            assert!(receipt.copied_items < original.maximum_items);
            assert!(release <= original.maximum_release_bytes - receipt.released_bytes);
            assert!(original.maximum_depth > 0);
            let (_, items, released_bytes) = owner.close_one();
            assert_eq!(release, released_bytes);
            receipt = receipt.checked_add(semio_framework_job::RetainedCloneProgress { copied_items: items, released_bytes, ..Default::default() }).unwrap();
        }
        assert!(receipt.fits(original));
    }

    fn close_original_bridge_and_script(bridge: &mut JobTurnBridge, job: &mut ScriptJob) {
        use semio_framework_job::InteractiveJob;
        let original = declared_bridge_grant("producerCloseGrant");
        let mut receipt = semio_framework_job::RetainedCloneProgress::default();
        while !bridge.terminal_is_empty() {
            let remaining = semio_framework_job::RetainedCloneGrant { maximum_items: original.maximum_items - receipt.copied_items, maximum_copy_bytes: original.maximum_copy_bytes - receipt.copied_bytes, maximum_capacity_bytes: original.maximum_capacity_bytes - receipt.retained_capacity_bytes, maximum_release_bytes: original.maximum_release_bytes - receipt.released_bytes, ..original };
            let step = bridge.close_step(remaining).unwrap();
            assert!(step.progress().fits(remaining));
            receipt = receipt.checked_add(step.progress()).unwrap();
        }
        job.begin_close();
        let source=semio_framework_job::RetainedCloneGrant{maximum_items:original.maximum_items-receipt.copied_items,maximum_copy_bytes:original.maximum_copy_bytes-receipt.copied_bytes,maximum_capacity_bytes:original.maximum_capacity_bytes-receipt.retained_capacity_bytes,maximum_release_bytes:original.maximum_release_bytes-receipt.released_bytes,..original};
        let mut caller=semio_framework_job::OriginalJobFixtureCaller::admit(bridge.operation.operation,bridge.operation.generation,source,BRIDGE_NOW_US,1000000,1000000);
        while !job.terminal_is_empty() {
            let remaining = semio_framework_job::RetainedCloneGrant { maximum_items: original.maximum_items - receipt.copied_items, maximum_copy_bytes: original.maximum_copy_bytes - receipt.copied_bytes, maximum_capacity_bytes: original.maximum_capacity_bytes - receipt.retained_capacity_bytes, maximum_release_bytes: original.maximum_release_bytes - receipt.released_bytes, ..original };
            let step = caller.turn(remaining,|cx|job.close_step(cx)).unwrap();
            assert!(!matches!(step, semio_framework_job::InteractiveJobCloseStep::Blocked));
            assert!(step.progress().fits(remaining));
            receipt = receipt.checked_add(step.progress()).unwrap();
        }
        assert!(receipt.fits(original));
        caller.retire(source);
    }

    const BRIDGE_NOW_US: fn() -> Option<u64> = || Some(10);

    #[derive(Default)]
    struct ScriptJob {
        outcomes: VecDeque<JobStepOutcome>,
        calls: usize,
        pending_state: Option<semio_framework_job::RetainedJobPayload>,
        pending_complete: Option<JobCommitCandidate>,
        pending_output: Option<semio_framework_job::RetainedJobPayload>,
        original_current: Option<JobStepOutcome>,
        current_copied: bool,
        published_kind: Option<semio_framework_job::JobOutcomeKind>,
        closing: bool,
    }

    impl semio_framework_job::InteractiveJob for ScriptJob {
        fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
            self.calls += 1;
            if self.published_kind.is_some() {
                for payload in [&mut self.pending_state, &mut self.pending_output] {
                    if let Some(original) = payload.as_mut() {
                        let step = original.close_step(cx.retained_grant())?;
                        cx.consume_retained(step.progress())?;
                        if !original.terminal_is_empty() { return Ok(None); }
                        *payload = None;
                    }
                }
                self.published_kind = None;
            }
            if let Some(original) = self.original_current.as_mut().filter(|_| self.current_copied) {
                if let Some(bytes) = script_original_bytes(original) {
                    let release = bytes.capacity();
                    let grant = cx.retained_grant();
                    if grant.maximum_items == 0 || grant.maximum_release_bytes < release || grant.maximum_depth == 0 { return Ok(None); }
                    drop(std::mem::take(bytes));
                    cx.consume_retained(semio_framework_job::RetainedCloneProgress { copied_items: 1, released_bytes: release, ..Default::default() })?;
                    return Ok(None);
                }
                self.original_current = None;
            }
            if self.pending_complete.is_some() && self.pending_state.is_none() {
                let candidate = self.pending_complete.as_mut().unwrap();
                let bytes = if candidate.state.capacity() != 0 { &mut candidate.state } else { &mut candidate.output };
                let release = bytes.capacity();
                let grant = cx.retained_grant();
                if grant.maximum_items == 0 || grant.maximum_release_bytes < release || grant.maximum_depth == 0 { return Ok(None); }
                drop(std::mem::take(bytes));
                cx.consume_retained(semio_framework_job::RetainedCloneProgress { copied_items: 1, released_bytes: release, ..Default::default() })?;
                if candidate.state.capacity() == 0 && candidate.output.capacity() == 0 { self.pending_complete = None; }
                return Ok(None);
            }
            if let Some(candidate) = self.pending_complete.as_ref() {
                let bytes = &candidate.output;
                let grant = cx.retained_grant();
                if grant.maximum_items < 2 || grant.maximum_copy_bytes < bytes.len() || grant.maximum_capacity_bytes < semio_framework_job::JOB_PAYLOAD_PAGE_BYTES || grant.maximum_depth == 0 { return Ok(None); }
                self.pending_output = Some(cx.payload_from_bytes(semio_framework_job::JobPayloadStream::CommitOutput, bytes).expect("fixture original page admission"));
                cx.consume_retained(semio_framework_job::RetainedCloneProgress { copied_items: 1, copied_bytes: bytes.len(), retained_capacity_bytes: semio_framework_job::JOB_PAYLOAD_PAGE_BYTES, ..Default::default() })?;
                self.published_kind = Some(semio_framework_job::JobOutcomeKind::Complete);
                return semio_framework_job::JobOutcomeBorrow::admit_complete(cx, self.pending_state.as_ref(), self.pending_output.as_ref()).map_err(Into::into);
            }
            if self.original_current.is_none() { self.original_current = self.outcomes.pop_front(); self.current_copied = false; }
            let original = self.original_current.as_ref().expect("scripted original bridge outcome");
            let (kind, stream, bytes) = match original {
                JobStepOutcome::Yield => { let outcome = semio_framework_job::JobOutcomeBorrow::admit_yield(cx)?; self.current_copied = outcome.is_some(); return Ok(outcome); },
                JobStepOutcome::Cancelled => { let outcome = semio_framework_job::JobOutcomeBorrow::admit_cancelled(cx)?; self.current_copied = outcome.is_some(); return Ok(outcome); },
                JobStepOutcome::PreviewReady { preview } => (semio_framework_job::JobOutcomeKind::PreviewReady, semio_framework_job::JobPayloadStream::Preview, preview.as_slice()),
                JobStepOutcome::CheckpointReady { checkpoint } => (semio_framework_job::JobOutcomeKind::CheckpointReady { applied_progress: checkpoint.applied_progress }, semio_framework_job::JobPayloadStream::CheckpointState, checkpoint.state.as_slice()),
                JobStepOutcome::Complete { candidate } => (semio_framework_job::JobOutcomeKind::Complete, semio_framework_job::JobPayloadStream::CommitState, candidate.state.as_slice()),
                JobStepOutcome::Fault { detail } => (semio_framework_job::JobOutcomeKind::Fault, semio_framework_job::JobPayloadStream::Fault, detail.as_slice()),
            };
            let grant = cx.retained_grant();
            if grant.maximum_items < 2 || grant.maximum_copy_bytes < bytes.len() || grant.maximum_capacity_bytes < semio_framework_job::JOB_PAYLOAD_PAGE_BYTES || grant.maximum_depth == 0 { return Ok(None); }
            self.pending_state = Some(cx.payload_from_bytes(stream, bytes).expect("fixture original page admission"));
            cx.consume_retained(semio_framework_job::RetainedCloneProgress { copied_items: 1, copied_bytes: bytes.len(), retained_capacity_bytes: semio_framework_job::JOB_PAYLOAD_PAGE_BYTES, ..Default::default() })?;
            self.current_copied = true;
            if kind == semio_framework_job::JobOutcomeKind::Complete {
                let Some(JobStepOutcome::Complete { candidate }) = self.original_current.take() else { unreachable!() };
                self.pending_complete = Some(candidate);
                return semio_framework_job::JobOutcomeBorrow::admit_yield(cx).map_err(Into::into);
            }
            self.published_kind = Some(kind);
            match kind {
                semio_framework_job::JobOutcomeKind::PreviewReady => {
                    cx.next_preview_sequence().expect("fixture original preview sequence");
                    semio_framework_job::JobOutcomeBorrow::admit_preview(cx, self.pending_state.as_ref().unwrap())
                }
                semio_framework_job::JobOutcomeKind::CheckpointReady { applied_progress } => semio_framework_job::JobOutcomeBorrow::admit_checkpoint(cx, self.pending_state.as_ref().unwrap(), applied_progress),
                semio_framework_job::JobOutcomeKind::Fault => semio_framework_job::JobOutcomeBorrow::admit_fault(cx, self.pending_state.as_ref().unwrap()),
                _ => unreachable!(),
            }.map_err(Into::into)
        }

        fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
            match descriptor.kind() {
                semio_framework_job::JobOutcomeKind::Yield => descriptor.yielded(),
                semio_framework_job::JobOutcomeKind::Cancelled => descriptor.cancelled(),
                semio_framework_job::JobOutcomeKind::PreviewReady => descriptor.preview(self.pending_state.as_ref().expect("original preview")),
                semio_framework_job::JobOutcomeKind::CheckpointReady { .. } => descriptor.checkpoint(self.pending_state.as_ref().expect("original checkpoint")),
                semio_framework_job::JobOutcomeKind::Complete => descriptor.complete(self.pending_state.as_ref(), self.pending_output.as_ref()),
                semio_framework_job::JobOutcomeKind::Fault => descriptor.fault(self.pending_state.as_ref().expect("original fault")),
            }
        }

        fn begin_close(&mut self) {
            self.closing = true;
        }

        fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
            self.begin_close();if self.terminal_is_empty(){return Ok(semio_framework_job::InteractiveJobCloseStep::Complete{progress:Default::default()})}
            let demand=semio_framework_value::RetirementDemand{copy_bytes:self.next_close_copy_byte_demand()?,capacity_bytes:self.next_close_capacity_byte_demand(cx.retained_grant().maximum_copy_bytes)?,release_bytes:self.next_close_release_byte_demand()?,depth:self.next_close_depth_demand()?};
            semio_framework_job::close_original_retained_step(cx,demand,|grant|{
                if let Some(state)=self.pending_state.as_mut(){let step=state.close_step(semio_framework_job::RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant})?;if state.terminal_is_empty(){self.pending_state=None;}return Ok(step)}
                if let Some(output)=self.pending_output.as_mut(){let step=output.close_step(semio_framework_job::RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant})?;if output.terminal_is_empty(){self.pending_output=None;}return Ok(step)}
                if let Some(original)=self.original_current.as_mut(){if let Some(bytes)=script_original_bytes(original){drop(std::mem::take(bytes));}else{self.original_current=None;}}
                else if let Some(candidate)=self.pending_complete.as_mut(){if candidate.state.capacity()!=0{drop(std::mem::take(&mut candidate.state));}else if candidate.output.capacity()!=0{drop(std::mem::take(&mut candidate.output));}else{self.pending_complete=None;}}
                else if let Some(outcome)=self.outcomes.front_mut(){if let Some(bytes)=script_original_bytes(outcome){drop(std::mem::take(bytes));}else{self.outcomes.pop_front();}}
                else{drop(std::mem::take(&mut self.outcomes));}
                let progress=semio_framework_job::RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()};Ok(if self.terminal_is_empty(){semio_framework_value::RetainedCloneStep::Complete(progress)}else{semio_framework_value::RetainedCloneStep::Progress(progress)})
            })
        }

        fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.pending_state.as_ref().or(self.pending_output.as_ref()).map(|owner| owner.retirement_demands()).transpose()?.map_or(0, |demand| demand.copy_bytes)) }
        fn next_close_capacity_byte_demand(&self, _: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.pending_state.as_ref().or(self.pending_output.as_ref()).map(|owner| owner.retirement_demands()).transpose()?.map_or(0, |demand| demand.capacity_bytes)) }
        fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
            if let Some(state) = self.pending_state.as_ref() { return Ok(state.retirement_demands()?.release_bytes); }
            if let Some(output) = self.pending_output.as_ref() { return Ok(output.retirement_demands()?.release_bytes); }
            if let Some(original) = self.original_current.as_ref() { return Ok(script_original_capacity(original)); }
            if let Some(candidate) = self.pending_complete.as_ref() { return Ok(if candidate.state.capacity() != 0 { candidate.state.capacity() } else { candidate.output.capacity() }); }
            if let Some(outcome) = self.outcomes.front() { return Ok(script_original_capacity(outcome)); }
            self.outcomes.capacity().checked_mul(std::mem::size_of::<JobStepOutcome>()).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "original script queue extent overflow"))
        }
        fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(if let Some(state) = self.pending_state.as_ref().or(self.pending_output.as_ref()) { state.retirement_demands()?.depth + 1 } else { usize::from(!self.terminal_is_empty()) }) }

        fn terminal_is_empty(&self) -> bool {
            self.closing && self.outcomes.capacity() == 0 && self.pending_state.is_none() && self.pending_output.is_none() && self.original_current.is_none() && self.pending_complete.is_none()
        }
    }

    fn script_original_bytes(outcome: &mut JobStepOutcome) -> Option<&mut Vec<u8>> {
        match outcome {
            JobStepOutcome::PreviewReady { preview } if preview.capacity() != 0 => Some(preview),
            JobStepOutcome::CheckpointReady { checkpoint } if checkpoint.state.capacity() != 0 => Some(&mut checkpoint.state),
            JobStepOutcome::Complete { candidate } => if candidate.state.capacity() != 0 { Some(&mut candidate.state) } else if candidate.output.capacity() != 0 { Some(&mut candidate.output) } else { None },
            JobStepOutcome::Fault { detail } if detail.capacity() != 0 => Some(detail),
            _ => None,
        }
    }

    fn script_original_capacity(outcome: &JobStepOutcome) -> usize {
        match outcome {
            JobStepOutcome::PreviewReady { preview } => preview.capacity(),
            JobStepOutcome::CheckpointReady { checkpoint } => checkpoint.state.capacity(),
            JobStepOutcome::Complete { candidate } => if candidate.state.capacity() != 0 { candidate.state.capacity() } else { candidate.output.capacity() },
            JobStepOutcome::Fault { detail } => detail.capacity(),
            _ => 0,
        }
    }

    struct UnsequencedPreviewJob { original: semio_framework_job::RetainedJobPayload }

    impl semio_framework_job::InteractiveJob for UnsequencedPreviewJob {
        fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> { semio_framework_job::JobOutcomeBorrow::admit_preview(cx, &self.original).map_err(Into::into) }
        fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> { descriptor.preview(&self.original) }

        fn begin_close(&mut self) {}

        fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{let demand=self.original.retirement_demands()?;semio_framework_job::close_original_retained_step(cx,demand,|grant|self.original.close_step(grant))}
        fn next_close_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.original.retirement_demands()?.copy_bytes)}
        fn next_close_capacity_byte_demand(&self,_:usize)->Result<usize,semio_framework_value::ValueError>{Ok(self.original.retirement_demands()?.capacity_bytes)}
        fn next_close_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.original.retirement_demands()?.release_bytes)}
        fn next_close_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.original.retirement_demands()?.depth)}
        fn terminal_is_empty(&self)->bool{self.original.terminal_is_empty()}
    }
    //#endregion 🔖️Helpers

    //#region 🪪️JobBridge
    #[test]
    fn job_bridge_invokes_exactly_one_step_per_turn() {
        let operation = bridge_operation();
        let mut bridge = JobTurnBridge::new(operation, original_bridge_grant()).unwrap();
        let mut job = ScriptJob { outcomes: VecDeque::from([JobStepOutcome::Yield, JobStepOutcome::Yield]), calls: 0, ..Default::default() };
        let publication = bridge
            .step(&mut job, bridge_turn(0, 0), operation.operation, operation.base_revision, operation.generation, "actor.job.one-step", semio_framework_job::InteractiveStage::InteractiveStep, semio_framework_job::StepBudget::new(100, 20, original_bridge_grant()), semio_framework_job::root_cancel_token(), BRIDGE_NOW_US)
            .expect("first job turn");
        assert_eq!(job.calls, 1);
        assert!(matches!(publication.outcome, JobStepOutcome::Yield));
        close_original_bridge_and_script(&mut bridge, &mut job);
    }

    #[test]
    fn original_actor_bridge_preserves_checkpoint_state_and_applied_progress() {
        let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧪️tests/📄️checkpoint/🔣️.json")).unwrap();
        let operation = bridge_operation();
        let mut bridge = JobTurnBridge::new(operation, original_bridge_grant()).unwrap();
        let checkpoint = JobCheckpoint { state: serde_json::from_value(fixture["state"].clone()).unwrap(), applied_progress: fixture["appliedProgress"].as_u64().unwrap() };
        let mut job = ScriptJob { outcomes: VecDeque::from([JobStepOutcome::CheckpointReady { checkpoint: checkpoint.clone() }]), calls: 0, ..Default::default() };
        let mut publication=None;
        for (sequence,expected) in fixture["publications"].as_array().unwrap().iter().enumerate(){
            let current=bridge.step(&mut job,bridge_turn(sequence as u64,0),operation.operation,operation.base_revision,operation.generation,"actor.job.checkpoint",semio_framework_job::InteractiveStage::BackgroundStep,semio_framework_job::StepBudget::new(100, 20, original_bridge_grant()),semio_framework_job::root_cancel_token(),BRIDGE_NOW_US).expect("checkpoint projection publication");
            assert_eq!(job.calls,fixture["jobCalls"].as_u64().unwrap()as usize);
            if expected=="yield"{assert_eq!(current.outcome,JobStepOutcome::Yield);}else{assert_eq!(current.outcome,JobStepOutcome::CheckpointReady{checkpoint:checkpoint.clone()});}
            publication=Some(current);
        }
        let publication = publication.unwrap();
        assert!(matches!(&publication.outcome, JobStepOutcome::CheckpointReady { checkpoint: returned } if returned == &checkpoint));
        let allocation: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/📄️checkpoint/allocation.json")).unwrap();
        if let JobStepOutcome::CheckpointReady { checkpoint } = &publication.outcome { assert_eq!(checkpoint.state.capacity(), allocation["declaredCapacityBytes"].as_u64().unwrap() as usize); }
        close_original_publication(publication);
        close_original_bridge_and_script(&mut bridge, &mut job);
    }

    #[semio_framework_async_macros::async_test]
    async fn job_bridge_cancellation_is_terminal_and_skips_the_job() {
        let operation = bridge_operation();
        let mut bridge = JobTurnBridge::new(operation, original_bridge_grant()).unwrap();
        let cancel = semio_framework_job::root_cancel_token();
        cancel.cancel().await;
        let mut job = ScriptJob { outcomes: VecDeque::from([JobStepOutcome::Yield]), calls: 0, ..Default::default() };
        let publication = bridge
            .step(&mut job, bridge_turn(0, 0), operation.operation, operation.base_revision, operation.generation, "actor.job.cancel", semio_framework_job::InteractiveStage::InteractiveStep, semio_framework_job::StepBudget::new(100, 20, original_bridge_grant()), cancel, BRIDGE_NOW_US)
            .expect("cancel publication");
        assert_eq!(job.calls, 0);
        assert!(matches!(publication.outcome, JobStepOutcome::Cancelled));
        assert!(matches!(
            bridge.step(
                &mut job,
                bridge_turn(1, 0),
                operation.operation,
                operation.base_revision,
                operation.generation,
                "actor.job.cancel",
                semio_framework_job::InteractiveStage::InteractiveStep,
                semio_framework_job::StepBudget::new(100, 20, original_bridge_grant()),
                semio_framework_job::root_cancel_token(),
                BRIDGE_NOW_US
            ),
            Err(JobPublicationError::Terminal)
        ));
        close_original_bridge_and_script(&mut bridge, &mut job);
    }

    #[test]
    fn job_bridge_rejects_stale_commit_before_work_or_publication() {
        let operation = bridge_operation();
        let mut bridge = JobTurnBridge::new(operation, original_bridge_grant()).unwrap();
        let mut job = ScriptJob { outcomes: VecDeque::from([JobStepOutcome::Complete { candidate: JobCommitCandidate { state: vec![1], output: vec![2] } }]), calls: 0, ..Default::default() };
        let result =
            bridge.step(&mut job, bridge_turn(0, 0), operation.operation, semio_framework_job::RevisionId(8), operation.generation, "actor.job.stale", semio_framework_job::InteractiveStage::InteractiveStep, semio_framework_job::StepBudget::new(100, 20, original_bridge_grant()), semio_framework_job::root_cancel_token(), BRIDGE_NOW_US);
        assert!(matches!(result, Err(JobPublicationError::Stale { live_revision: 8, live_generation: 3 })));
        assert_eq!(job.calls, 0);
        close_original_bridge_and_script(&mut bridge, &mut job);
    }

    #[test]
    fn job_bridge_rejects_replayed_preview_identity_before_work() {
        let operation = bridge_operation();
        let mut bridge = JobTurnBridge::new(operation, original_bridge_grant()).unwrap();
        let mut job = ScriptJob { outcomes: VecDeque::from([JobStepOutcome::Yield]), calls: 0, ..Default::default() };
        assert!(matches!(
            bridge.step(
                &mut job,
                bridge_turn(0, 1),
                operation.operation,
                operation.base_revision,
                operation.generation,
                "actor.job.replayed-preview",
                semio_framework_job::InteractiveStage::InteractiveStep,
                semio_framework_job::StepBudget::new(100, 20, original_bridge_grant()),
                semio_framework_job::root_cancel_token(),
                BRIDGE_NOW_US,
            ),
            Err(JobPublicationError::Stale { .. })
        ));
        assert_eq!(job.calls, 0);
        close_original_bridge_and_script(&mut bridge, &mut job);
    }

    #[test]
    fn job_bridge_rejects_a_preview_without_exactly_one_sequence_advance() {
        let operation = bridge_operation();
        let mut bridge = JobTurnBridge::new(operation, original_bridge_grant()).unwrap();
        assert!(matches!(
            bridge.step(
                &mut UnsequencedPreviewJob { original: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Preview) },
                bridge_turn(0, 0),
                operation.operation,
                operation.base_revision,
                operation.generation,
                "actor.job.preview-order",
                semio_framework_job::InteractiveStage::InteractiveStep,
                semio_framework_job::StepBudget::new(100, 20, original_bridge_grant()),
                semio_framework_job::root_cancel_token(),
                BRIDGE_NOW_US,
            ),
            Err(JobPublicationError::PreviewSequence { before: 0, after: 0 })
        ));
        let original = declared_bridge_grant("producerCloseGrant");
        let mut receipt = semio_framework_job::RetainedCloneProgress::default();
        while !bridge.terminal_is_empty() {
            let remaining = semio_framework_job::RetainedCloneGrant { maximum_items: original.maximum_items - receipt.copied_items, maximum_copy_bytes: original.maximum_copy_bytes - receipt.copied_bytes, maximum_capacity_bytes: original.maximum_capacity_bytes - receipt.retained_capacity_bytes, maximum_release_bytes: original.maximum_release_bytes - receipt.released_bytes, ..original };
            let step = bridge.close_step(remaining).unwrap();
            receipt = receipt.checked_add(step.progress()).unwrap();
        }

    }

    #[test]
    fn mounted_fixed_replay_capture_is_deterministic_and_returns_the_exact_live_owner() {
        fn run(worker_count: u16, worker_slot: u16) -> [u64; 4] {
            let mut original_recipient = semio_framework_job::RetainedCloneProgress::default();
            let operation = bridge_operation();
            let mut bridge = JobTurnBridge::new(operation, original_bridge_grant()).unwrap();
            let mut job = ScriptJob {
                outcomes: VecDeque::from([
                    JobStepOutcome::Yield,
                    JobStepOutcome::PreviewReady { preview: vec![9, 8] },
                    JobStepOutcome::CheckpointReady { checkpoint: JobCheckpoint { state: vec![7, 6], applied_progress: 5 } },
                    JobStepOutcome::Complete { candidate: JobCommitCandidate { state: vec![3], output: vec![2, 1] } },
                ]),
                calls: 0,
                ..Default::default()
            };
            let route = JobReplayRoute { plugin: [1; 32], package: [2; 32], controller: [3; 32], tool: [4; 32], window: 5, artifact: [6; 32], request_schema: [7; 32], request_version: 1, request_digest: [8; 32] };
            let mut log = JobReplayLog::new(route, operation.generation.0).expect("fixed replay authority");
            let mut turn = bridge_turn(0, 0);
            loop {
                let publication = bridge
                    .step(&mut job, turn, operation.operation, operation.base_revision, operation.generation, "actor.job.replay", semio_framework_job::InteractiveStage::InteractiveStep, semio_framework_job::StepBudget::new(100, 20, original_bridge_grant()), semio_framework_job::root_cancel_token(), BRIDGE_NOW_US)
                    .expect("replay publication");
                let terminal = matches!(&publication.outcome, JobStepOutcome::Complete { .. });
                turn = JobTurn { step_sequence: publication.turn.step_sequence + 1, ..publication.turn };
                let payload_identity = match &publication.outcome {
                    JobStepOutcome::PreviewReady { preview } => preview.as_ptr(),
                    JobStepOutcome::CheckpointReady { checkpoint } => checkpoint.state.as_ptr(),
                    JobStepOutcome::Complete { candidate } => candidate.output.as_ptr(),
                    _ => std::ptr::null(),
                };
                let mut preview_sequence = publication.turn.operation.preview_sequence;
                let mut context = semio_framework_job::StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, 20, original_bridge_grant()), semio_framework_job::root_cancel_token(), BRIDGE_NOW_US, &mut preview_sequence, &mut original_recipient);
                log.begin_capture(&mut context, ActorId(9), JobReplaySchedule { worker_count, worker_slot, granted_fuel: 1, deadline_class_ms: 4 }, publication).expect("capture admission");
                drop(context);
                loop {
                    let mut preview_sequence = turn.operation.preview_sequence;
                    let mut context = semio_framework_job::StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, 20, original_bridge_grant()), semio_framework_job::root_cancel_token(), BRIDGE_NOW_US, &mut preview_sequence, &mut original_recipient);
                    if log.capture_step(&mut context) == JobReplayStep::PublicationReady {
                        break;
                    }
                }
                let publication = log.take_captured_publication().expect("same live publication");
                let returned_identity = match &publication.outcome {
                    JobStepOutcome::PreviewReady { preview } => preview.as_ptr(),
                    JobStepOutcome::CheckpointReady { checkpoint } => checkpoint.state.as_ptr(),
                    JobStepOutcome::Complete { candidate } => candidate.output.as_ptr(),
                    _ => std::ptr::null(),
                };
                assert_eq!(returned_identity, payload_identity);
                drop(publication);
                let now = BRIDGE_NOW_US().expect("original replay clock");
                let mut sequence = 0;
                let mut context = semio_framework_job::StepContext::new(
                    semio_framework_job::OperationId(operation.operation.0),
                    semio_framework_job::Generation(operation.generation.0),
                    semio_framework_job::StepBudget::from_duration(1, now, 4_000, original_bridge_grant()).expect("test deadline"),
                    semio_framework_job::root_cancel_token(),
                    BRIDGE_NOW_US,
                    &mut sequence, &mut original_recipient);
                log.acknowledge_publication(&mut context, JobReplayPublicationPolicy::Accepted).expect("P2d ACK");
                drop(context);
                if terminal {
                    break;
                }
            }
            close_original_bridge_and_script(&mut bridge, &mut job);
            log.begin_replay(operation.generation.0).expect("sealed generation replay");
            let mut replay_bridge = JobTurnBridge::new(operation, original_bridge_grant()).unwrap();
            let mut replay_job = ScriptJob {
                outcomes: VecDeque::from([
                    JobStepOutcome::Yield,
                    JobStepOutcome::PreviewReady { preview: vec![9, 8] },
                    JobStepOutcome::CheckpointReady { checkpoint: JobCheckpoint { state: vec![7, 6], applied_progress: 5 } },
                    JobStepOutcome::Complete { candidate: JobCommitCandidate { state: vec![3], output: vec![2, 1] } },
                ]),
                calls: 0,
                ..Default::default()
            };
            let mut replay_turn = bridge_turn(0, 0);
            loop {
                let publication = replay_bridge
                    .step(
                        &mut replay_job,
                        replay_turn,
                        operation.operation,
                        operation.base_revision,
                        operation.generation,
                        "actor.job.replay-live",
                        semio_framework_job::InteractiveStage::InteractiveStep,
                        semio_framework_job::StepBudget::new(100, 20, original_bridge_grant()),
                        semio_framework_job::root_cancel_token(),
                        BRIDGE_NOW_US,
                    )
                    .expect("replayed live publication");
                assert_eq!(log.expected_turn(), Some(publication.turn));
                let terminal = matches!(&publication.outcome, JobStepOutcome::Complete { .. });
                replay_turn = JobTurn { step_sequence: publication.turn.step_sequence + 1, ..publication.turn };
                let mut preview_sequence = publication.turn.operation.preview_sequence;
                let mut context = semio_framework_job::StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, 20, original_bridge_grant()), semio_framework_job::root_cancel_token(), BRIDGE_NOW_US, &mut preview_sequence, &mut original_recipient);
                log.begin_capture(&mut context, ActorId(9), JobReplaySchedule { worker_count, worker_slot, granted_fuel: 1, deadline_class_ms: 4 }, publication).expect("replay capture admission");
                drop(context);
                loop {
                    let mut preview_sequence = replay_turn.operation.preview_sequence;
                    let mut context = semio_framework_job::StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, 20, original_bridge_grant()), semio_framework_job::root_cancel_token(), BRIDGE_NOW_US, &mut preview_sequence, &mut original_recipient);
                    if log.capture_step(&mut context) == JobReplayStep::PublicationReady {
                        break;
                    }
                }
                drop(log.take_captured_publication().expect("matched replay publication"));
                let mut preview_sequence = replay_turn.operation.preview_sequence;
                let mut context = semio_framework_job::StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, 20, original_bridge_grant()), semio_framework_job::root_cancel_token(), BRIDGE_NOW_US, &mut preview_sequence, &mut original_recipient);
                log.acknowledge_publication(&mut context, JobReplayPublicationPolicy::Accepted).expect("matched replay policy");
                drop(context);
                while log.has_pending_work() {
                    let mut preview_sequence = replay_turn.operation.preview_sequence;
                    let mut context = semio_framework_job::StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, 20, original_bridge_grant()), semio_framework_job::root_cancel_token(), BRIDGE_NOW_US, &mut preview_sequence, &mut original_recipient);
                    let _ = log.maintenance_step(&mut context);
                }
                if terminal {
                    break;
                }
            }
            close_original_bridge_and_script(&mut replay_bridge, &mut replay_job);
            assert!(log.expected_turn().is_none());
            let digests = std::array::from_fn(|index| log.record_header(index).expect("sealed record").prefix_digest);
            log.begin_close();
            while !log.terminal_is_empty() {
                let mut preview_sequence = 0;
                let mut context = semio_framework_job::StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, 20, original_bridge_grant()), semio_framework_job::root_cancel_token(), BRIDGE_NOW_US, &mut preview_sequence, &mut original_recipient);
                let _ = log.close_step(&mut context);
            }
            digests
        }

        let host_default = u16::try_from(std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get)).unwrap_or(u16::MAX);
        for worker_count in [1, 2, 4, host_default] {
            assert_eq!(run(worker_count, 0), run(worker_count, 0), "1/2/4/default worker replay preserves the exact ordered publication identity");
        }
    }

    #[test]
    fn mounted_replay_records_and_replays_the_exact_cancelled_terminal_classification() {
        let mut original_recipient = semio_framework_job::RetainedCloneProgress::default();
        let operation = bridge_operation();
        let route = JobReplayRoute { plugin: [11; 32], package: [13; 32], controller: [17; 32], tool: [19; 32], window: 23, artifact: [29; 32], request_schema: [31; 32], request_version: 1, request_digest: [37; 32] };
        let mut log = JobReplayLog::new(route, operation.generation.0).expect("fixed replay authority");
        let turn = bridge_turn(0, 0);
        for replaying in [false, true] {
            let publication = JobPublication { turn, outcome: JobStepOutcome::Cancelled };
            let mut sequence = 0;
            let mut context = semio_framework_job::StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, 20, original_bridge_grant()), semio_framework_job::root_cancel_token(), BRIDGE_NOW_US, &mut sequence, &mut original_recipient);
            log.begin_capture(&mut context, ActorId(9), JobReplaySchedule { worker_count: 1, worker_slot: 0, granted_fuel: 1, deadline_class_ms: 4 }, publication).expect("cancel capture admission");
            drop(context);
            loop {
                let mut sequence = 0;
                let mut context = semio_framework_job::StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, 20, original_bridge_grant()), semio_framework_job::root_cancel_token(), BRIDGE_NOW_US, &mut sequence, &mut original_recipient);
                if log.capture_step(&mut context) == JobReplayStep::PublicationReady {
                    break;
                }
            }
            assert!(log.record_header(0).expect("cancel record").cancellation_observed);
            assert!(matches!(log.take_captured_publication().expect("exact cancelled owner").outcome, JobStepOutcome::Cancelled));
            let mut sequence = 0;
            let mut context = semio_framework_job::StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, 20, original_bridge_grant()), semio_framework_job::root_cancel_token(), BRIDGE_NOW_US, &mut sequence, &mut original_recipient);
            log.acknowledge_publication(&mut context, JobReplayPublicationPolicy::Accepted).expect("cancel ACK");
            drop(context);
            if !replaying {
                log.begin_replay(operation.generation.0).expect("cancel replay generation");
            }
        }
        assert!(log.expected_turn().is_none());
        log.begin_close();
        while !log.terminal_is_empty() {
            let mut sequence = 0;
            let mut context = semio_framework_job::StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, 20, original_bridge_grant()), semio_framework_job::root_cancel_token(), BRIDGE_NOW_US, &mut sequence, &mut original_recipient);
            let _ = log.close_step(&mut context);
        }
    }

    #[test]
    fn mounted_replay_cancel_deadline_and_stale_refuse_the_exact_publication_owner_unchanged() {
        let mut original_recipient = semio_framework_job::RetainedCloneProgress::default();
        let operation = bridge_operation();
        let route = JobReplayRoute { plugin: [41; 32], package: [43; 32], controller: [47; 32], tool: [53; 32], window: 59, artifact: [61; 32], request_schema: [67; 32], request_version: 1, request_digest: [71; 32] };
        let mut log = JobReplayLog::new(route, operation.generation.0).expect("fixed replay authority");
        let publication = |generation: u64| JobPublication { turn: JobTurn { operation: JobOperation { generation, ..bridge_turn(0, 0).operation }, ..bridge_turn(0, 0) }, outcome: JobStepOutcome::PreviewReady { preview: vec![73, 79] } };

        let cancelled = publication(operation.generation.0);
        let cancelled_identity = match &cancelled.outcome {
            JobStepOutcome::PreviewReady { preview } => preview.as_ptr(),
            _ => unreachable!(),
        };
        let cancel = semio_framework_job::root_cancel_token();
        cancel.cancel_now();
        let mut sequence = 0;
        let mut context = semio_framework_job::StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, 20, original_bridge_grant()), cancel, BRIDGE_NOW_US, &mut sequence, &mut original_recipient);
        let cancelled = log.begin_capture(&mut context, ActorId(9), JobReplaySchedule { worker_count: 1, worker_slot: 0, granted_fuel: 1, deadline_class_ms: 4 }, cancelled).expect_err("cancelled capture refuses before transfer").into_publication();
        drop(context);
        assert!(matches!(&cancelled.outcome, JobStepOutcome::PreviewReady { preview } if preview.as_ptr() == cancelled_identity));

        let expired = publication(operation.generation.0);
        let expired_identity = match &expired.outcome {
            JobStepOutcome::PreviewReady { preview } => preview.as_ptr(),
            _ => unreachable!(),
        };
        let mut sequence = 0;
        let mut context = semio_framework_job::StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, 10, original_bridge_grant()), semio_framework_job::root_cancel_token(), BRIDGE_NOW_US, &mut sequence, &mut original_recipient);
        let expired = log.begin_capture(&mut context, ActorId(9), JobReplaySchedule { worker_count: 1, worker_slot: 0, granted_fuel: 1, deadline_class_ms: 4 }, expired).expect_err("expired capture refuses before transfer").into_publication();
        drop(context);
        assert!(matches!(&expired.outcome, JobStepOutcome::PreviewReady { preview } if preview.as_ptr() == expired_identity));

        let stale = publication(operation.generation.0 + 1);
        let stale_identity = match &stale.outcome {
            JobStepOutcome::PreviewReady { preview } => preview.as_ptr(),
            _ => unreachable!(),
        };
        let mut sequence = 0;
        let mut context = semio_framework_job::StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, 20, original_bridge_grant()), semio_framework_job::root_cancel_token(), BRIDGE_NOW_US, &mut sequence, &mut original_recipient);
        let stale = log.begin_capture(&mut context, ActorId(9), JobReplaySchedule { worker_count: 1, worker_slot: 0, granted_fuel: 1, deadline_class_ms: 4 }, stale).expect_err("stale capture refuses before transfer").into_publication();
        drop(context);
        assert!(matches!(&stale.outcome, JobStepOutcome::PreviewReady { preview } if preview.as_ptr() == stale_identity));
        assert_eq!(log.sealed_records(), 0);
        assert!(!log.has_pending_work());
        log.begin_close();
        assert!(log.terminal_is_empty());
    }

    #[test]
    fn mounted_replay_preserves_the_exact_fault_payload_and_prefix_across_replay() {
        let mut original_recipient = semio_framework_job::RetainedCloneProgress::default();
        let operation = bridge_operation();
        let route = JobReplayRoute { plugin: [73; 32], package: [79; 32], controller: [83; 32], tool: [89; 32], window: 97, artifact: [101; 32], request_schema: [103; 32], request_version: 1, request_digest: [107; 32] };
        let mut log = JobReplayLog::new(route, operation.generation.0).expect("fixed replay authority");
        let turn = bridge_turn(0, 0);
        for replaying in [false, true] {
            let publication = JobPublication { turn, outcome: JobStepOutcome::Fault { detail: vec![109, 113, 127] } };
            let payload_identity = match &publication.outcome {
                JobStepOutcome::Fault { detail } => detail.as_ptr(),
                _ => unreachable!(),
            };
            let mut sequence = 0;
            let mut context = semio_framework_job::StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, 20, original_bridge_grant()), semio_framework_job::root_cancel_token(), BRIDGE_NOW_US, &mut sequence, &mut original_recipient);
            log.begin_capture(&mut context, ActorId(9), JobReplaySchedule { worker_count: 1, worker_slot: 0, granted_fuel: 1, deadline_class_ms: 4 }, publication).expect("fault capture admission");
            drop(context);
            loop {
                let mut sequence = 0;
                let mut context = semio_framework_job::StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, 20, original_bridge_grant()), semio_framework_job::root_cancel_token(), BRIDGE_NOW_US, &mut sequence, &mut original_recipient);
                if log.capture_step(&mut context) == JobReplayStep::PublicationReady {
                    break;
                }
            }
            let publication = log.take_captured_publication().expect("exact fault owner");
            assert!(matches!(&publication.outcome, JobStepOutcome::Fault { detail } if detail.as_ptr() == payload_identity && detail.as_slice() == [109, 113, 127]));
            let mut sequence = 0;
            let mut context = semio_framework_job::StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, 20, original_bridge_grant()), semio_framework_job::root_cancel_token(), BRIDGE_NOW_US, &mut sequence, &mut original_recipient);
            log.acknowledge_publication(&mut context, JobReplayPublicationPolicy::Accepted).expect("fault ACK");
            drop(context);
            if !replaying {
                log.begin_replay(operation.generation.0).expect("fault replay generation");
            }
        }
        assert!(log.expected_turn().is_none());
        log.begin_close();
        while !log.terminal_is_empty() {
            let mut sequence = 0;
            let mut context = semio_framework_job::StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, 20, original_bridge_grant()), semio_framework_job::root_cancel_token(), BRIDGE_NOW_US, &mut sequence, &mut original_recipient);
            let _ = log.close_step(&mut context);
        }
    }
    //#endregion 🪪️JobBridge

    //#region 🔖️JobProgressOverlay
    const PROGRESS_NOW_US: fn() -> Option<u64> = || Some(0);

    fn with_progress_context<T>(operation: u64, generation: u64, cancel: semio_framework_job::CancelToken, run: impl FnOnce(&mut semio_framework_job::StepContext<'_>) -> T) -> T {
        let mut original_recipient = semio_framework_job::RetainedCloneProgress::default();
        let mut preview_sequence = 0;
        let mut context = semio_framework_job::StepContext::new(semio_framework_job::OperationId(operation), semio_framework_job::Generation(generation), semio_framework_job::StepBudget::new(8, 10, original_bridge_grant()), cancel, PROGRESS_NOW_US, &mut preview_sequence, &mut original_recipient);
        run(&mut context)
    }

    fn progress_publication(job: u64, operation: u64, base_revision: u64, generation: u64, step_sequence: u64, preview_sequence: u64, outcome: JobStepOutcome) -> JobPublication {
        JobPublication { turn: JobTurn { job, operation: JobOperation { operation, base_revision, generation, preview_sequence, seed: 13 }, step_sequence }, outcome }
    }

    fn admit_progress(store: &mut JobProgressOverlayStore, actor: ActorId, publication: JobPublication) -> JobProgressReceipt {
        let live = store.live_authority(actor, publication.turn.job).expect("job operation was admitted before its publication");
        with_progress_context(publication.turn.operation.operation, publication.turn.operation.generation, semio_framework_job::root_cancel_token(), |context| {
            let admission = store.preflight(context, actor, &publication, live).expect("publication preflight");
            store.publish_reserved(context, admission, publication, live).expect("reserved publication")
        })
    }

    fn drive_progress_close(store: &mut JobProgressOverlayStore) {
        for _ in 0..(JOB_PROGRESS_TOTAL_MAXIMUM_ITEMS + JOB_PROGRESS_ACTIVE_CAPACITY * 3 + 8) {
            if store.terminal_is_empty() {
                return;
            }
            let step = with_progress_context(0, 0, semio_framework_job::root_cancel_token(), |context| store.close_step(context));
            assert!(matches!(step, JobProgressCloseStep::Pending { released_items: 0 | 1, .. } | JobProgressCloseStep::Complete));
        }
        panic!("job progress overlay did not reach its terminal witness");
    }

    #[test]
    fn job_progress_preview_is_distinct_owned_and_checked_out_drop_hands_back_exactly() {
        let actor = ActorId(7);
        let live = JobProgressLiveAuthority::new(11, 17, 23);
        let mut store = JobProgressOverlayStore::new();
        store.begin_operation(actor, 29, live).unwrap();
        let preview = Vec::from([1, 2, 3, 4]);
        let pointer = preview.as_ptr();
        let receipt = admit_progress(&mut store, actor, progress_publication(29, 11, 17, 23, 0, 1, JobStepOutcome::PreviewReady { preview }));
        assert_eq!(receipt.kind(), JobProgressKind::Preview);
        store.acknowledge(receipt).unwrap();
        {
            let checkout = store.take(actor, 29).unwrap();
            assert_eq!(checkout.preview().as_ptr(), pointer);
            assert_eq!(checkout.preview(), &[1, 2, 3, 4]);
        }
        assert_eq!(store.take(actor, 29).unwrap().preview().as_ptr(), pointer, "Drop must return the exact preview owner to its fixed slot");
        let replacement = admit_progress(&mut store, actor, progress_publication(29, 11, 17, 23, 1, 2, JobStepOutcome::PreviewReady { preview: vec![8, 9] }));
        store.abort(replacement).unwrap();
        assert_eq!(store.take(actor, 29).unwrap().preview().as_ptr(), pointer, "abort must restore the last valid preview owner rather than exposing the staged replacement");
        let checkpoint = admit_progress(&mut store, actor, progress_publication(29, 11, 17, 23, 1, 1, JobStepOutcome::CheckpointReady { checkpoint: JobCheckpoint { state: vec![10], applied_progress: 1 } }));
        store.abort(checkpoint).unwrap();
        assert_eq!(store.take(actor, 29).unwrap().preview().as_ptr(), pointer, "aborting scalar progress must retain the prior preview without an unnecessary owner move");
        store.begin_close_actor(actor).unwrap();
        drive_progress_close(&mut store);
    }

    #[test]
    fn job_progress_fixed_capacity_and_aba_admission_fail_closed() {
        let mut store = JobProgressOverlayStore::new();
        for ordinal in 0..JOB_PROGRESS_ACTIVE_CAPACITY {
            store.begin_operation(ActorId(ordinal as u64 + 1), 5, JobProgressLiveAuthority::new(ordinal as u64 + 10, 0, 1)).unwrap();
        }
        assert_eq!(store.begin_operation(ActorId(1000), 5, JobProgressLiveAuthority::new(1000, 0, 1)), Err(JobProgressFault::Capacity));
        assert_eq!(store.begin_operation(ActorId(1), 5, JobProgressLiveAuthority::new(10, 0, 1)), Err(JobProgressFault::Busy));
        store.begin_close_all();
        drive_progress_close(&mut store);
        store.begin_operation(ActorId(1), 5, JobProgressLiveAuthority::new(99, 0, 2)).unwrap();
        assert_eq!(store.live_authority(ActorId(1), 5), Some(JobProgressLiveAuthority::new(99, 0, 2)));
        store.begin_close_all();
        drive_progress_close(&mut store);
    }

    #[test]
    fn job_progress_mounted_aggregate_item_and_byte_caps_reject_plus_one_exactly() {
        let mut item_store = JobProgressOverlayStore::new();
        for ordinal in 0..JOB_PROGRESS_ACTIVE_CAPACITY {
            let actor = ActorId(ordinal as u64 + 1);
            let live = JobProgressLiveAuthority::new(ordinal as u64 + 10, 0, 1);
            item_store.begin_operation(actor, 5, live).unwrap();
            let receipt = admit_progress(&mut item_store, actor, progress_publication(5, live.operation, 0, 1, 0, 1, JobStepOutcome::PreviewReady { preview: Vec::new() }));
            item_store.acknowledge(receipt).unwrap();
        }
        for ordinal in 0..JOB_PROGRESS_RETIREMENT_CAPACITY {
            item_store
                .retain_rejected(JobProgressRejected::new(
                    JobProgressFault::Stale,
                    progress_publication(1_000 + ordinal as u64, 2_000 + ordinal as u64, 0, 1, 0, 0, JobStepOutcome::Complete { candidate: JobCommitCandidate { state: Vec::new(), output: Vec::new() } }),
                ))
                .unwrap();
        }
        assert_eq!(item_store.owned_items, JOB_PROGRESS_TOTAL_MAXIMUM_ITEMS);
        let rejected = JobProgressRejected::new(JobProgressFault::Stale, progress_publication(9_999, 8_888, 0, 1, 0, 0, JobStepOutcome::Yield));
        let returned = item_store.retain_rejected(rejected).unwrap_err();
        assert_eq!(returned.publication().turn.job, 9_999);
        item_store.begin_close_all();
        drive_progress_close(&mut item_store);

        let mut byte_store = JobProgressOverlayStore::new();
        for ordinal in 0..JOB_PROGRESS_ACTIVE_CAPACITY {
            let actor = ActorId(ordinal as u64 + 101);
            let live = JobProgressLiveAuthority::new(ordinal as u64 + 301, 0, 1);
            byte_store.begin_operation(actor, 7, live).unwrap();
            let receipt = admit_progress(&mut byte_store, actor, progress_publication(7, live.operation, 0, 1, 0, 1, JobStepOutcome::PreviewReady { preview: vec![3; JOB_PROGRESS_PAGE_MAXIMUM_BYTES] }));
            byte_store.acknowledge(receipt).unwrap();
        }
        for ordinal in 0..96 {
            byte_store
                .retain_rejected(JobProgressRejected::new(
                    JobProgressFault::Stale,
                    progress_publication(3_000 + ordinal, 4_000 + ordinal, 0, 1, 0, 0, JobStepOutcome::Complete { candidate: JobCommitCandidate { state: vec![4; JOB_PROGRESS_PAGE_MAXIMUM_BYTES], output: vec![5; JOB_PROGRESS_PAGE_MAXIMUM_BYTES] } }),
                ))
                .unwrap();
        }
        assert_eq!(byte_store.owned_bytes, JOB_PROGRESS_TOTAL_MAXIMUM_BYTES);
        let plus_one = vec![9];
        let pointer = plus_one.as_ptr();
        let returned = byte_store.retain_rejected(JobProgressRejected::new(JobProgressFault::Stale, progress_publication(7_777, 8_888, 0, 1, 0, 1, JobStepOutcome::PreviewReady { preview: plus_one }))).unwrap_err();
        assert!(matches!(&returned.publication().outcome, JobStepOutcome::PreviewReady { preview } if preview.as_ptr() == pointer));
        byte_store.begin_close_all();
        drive_progress_close(&mut byte_store);
    }

    #[test]
    fn job_progress_page_boundary_plus_one_stale_order_and_cancel_preserve_owner() {
        let actor = ActorId(31);
        let live = JobProgressLiveAuthority::new(37, 41, 43);
        let mut store = JobProgressOverlayStore::new();
        store.begin_operation(actor, 47, live).unwrap();
        let exact = vec![5; JOB_PROGRESS_PAGE_MAXIMUM_BYTES];
        let exact_pointer = exact.as_ptr();
        let exact_publication = progress_publication(47, 37, 41, 43, 0, 1, JobStepOutcome::PreviewReady { preview: exact });
        let admission = with_progress_context(37, 43, semio_framework_job::root_cancel_token(), |context| store.preflight(context, actor, &exact_publication, live)).unwrap();
        assert_eq!(
            match &exact_publication.outcome {
                JobStepOutcome::PreviewReady { preview } => preview.as_ptr(),
                _ => unreachable!(),
            },
            exact_pointer
        );
        store.cancel_admission(admission).unwrap();

        let oversized = vec![7; JOB_PROGRESS_PAGE_MAXIMUM_BYTES + 1];
        let oversized_pointer = oversized.as_ptr();
        let oversized_publication = progress_publication(47, 37, 41, 43, 0, 1, JobStepOutcome::PreviewReady { preview: oversized });
        let oversized_fault = with_progress_context(37, 43, semio_framework_job::root_cancel_token(), |context| store.preflight(context, actor, &oversized_publication, live)).unwrap_err();
        assert_eq!(oversized_fault, JobProgressFault::Oversized);
        assert_eq!(
            match &oversized_publication.outcome {
                JobStepOutcome::PreviewReady { preview } => preview.as_ptr(),
                _ => unreachable!(),
            },
            oversized_pointer
        );

        let stale = progress_publication(47, 37, 42, 43, 0, 1, JobStepOutcome::PreviewReady { preview: vec![1] });
        assert_eq!(with_progress_context(37, 43, semio_framework_job::root_cancel_token(), |context| store.preflight(context, actor, &stale, live)).unwrap_err(), JobProgressFault::Stale);
        let out_of_order = progress_publication(47, 37, 41, 43, 1, 1, JobStepOutcome::Yield);
        assert_eq!(with_progress_context(37, 43, semio_framework_job::root_cancel_token(), |context| store.preflight(context, actor, &out_of_order, live)).unwrap_err(), JobProgressFault::StepSequence);

        let cancelled = progress_publication(47, 37, 41, 43, 0, 1, JobStepOutcome::PreviewReady { preview: vec![9] });
        let cancelled_pointer = match &cancelled.outcome {
            JobStepOutcome::PreviewReady { preview } => preview.as_ptr(),
            _ => unreachable!(),
        };
        let cancel = semio_framework_job::root_cancel_token();
        cancel.cancel_now();
        assert_eq!(with_progress_context(37, 43, cancel, |context| store.preflight(context, actor, &cancelled, live)).unwrap_err(), JobProgressFault::Cancelled);
        assert_eq!(
            match &cancelled.outcome {
                JobStepOutcome::PreviewReady { preview } => preview.as_ptr(),
                _ => unreachable!(),
            },
            cancelled_pointer
        );
        store.begin_close_actor(actor).unwrap();
        drive_progress_close(&mut store);
    }

    #[test]
    fn job_progress_commit_validates_live_authority_and_rejected_close_is_incremental() {
        let actor = ActorId(53);
        let live = JobProgressLiveAuthority::new(59, 61, 67);
        let mut store = JobProgressOverlayStore::new();
        store.begin_operation(actor, 71, live).unwrap();
        let stale = progress_publication(71, 59, 61, 68, 0, 0, JobStepOutcome::Complete { candidate: JobCommitCandidate { state: vec![1], output: vec![2] } });
        assert_eq!(with_progress_context(59, 68, semio_framework_job::root_cancel_token(), |context| store.preflight(context, actor, &stale, JobProgressLiveAuthority::new(59, 61, 68))).unwrap_err(), JobProgressFault::Stale);

        let complete = progress_publication(71, 59, 61, 67, 0, 0, JobStepOutcome::Complete { candidate: JobCommitCandidate { state: vec![3], output: vec![4] } });
        let receipt = admit_progress(&mut store, actor, complete);
        assert_eq!(receipt.kind(), JobProgressKind::CommitValidated);
        store.acknowledge(receipt).unwrap();
        assert!(store.has_close_work(), "committed candidates retire outside the preview overlay");
        drive_progress_close(&mut store);

        let mut rejected_store = JobProgressOverlayStore::new();
        let detail = Vec::from([8, 9, 10]);
        let pointer = detail.as_ptr();
        let rejected = JobProgressRejected::new(JobProgressFault::Stale, progress_publication(73, 79, 83, 89, 0, 0, JobStepOutcome::Fault { detail }));
        assert_eq!(
            match &rejected.publication().outcome {
                JobStepOutcome::Fault { detail } => detail.as_ptr(),
                _ => unreachable!(),
            },
            pointer
        );
        rejected_store.retain_rejected(rejected).unwrap();
        let first = with_progress_context(0, 0, semio_framework_job::root_cancel_token(), |context| rejected_store.close_step(context));
        assert!(matches!(first, JobProgressCloseStep::Pending { released_items: 1, released_bytes } if released_bytes >= 3));
        assert!(!rejected_store.terminal_is_empty(), "publication shell retires on a distinct grant");
        drive_progress_close(&mut rejected_store);
    }

    #[test]
    fn job_progress_replay_is_deterministic_for_identical_publications() {
        let actor = ActorId(97);
        let live = JobProgressLiveAuthority::new(101, 103, 107);
        let mut left = JobProgressOverlayStore::new();
        let mut right = JobProgressOverlayStore::new();
        left.begin_operation(actor, 109, live).unwrap();
        right.begin_operation(actor, 109, live).unwrap();
        for (step, preview_sequence, byte) in [(0, 1, 11), (1, 2, 13)] {
            let left_receipt = admit_progress(&mut left, actor, progress_publication(109, 101, 103, 107, step, preview_sequence, JobStepOutcome::PreviewReady { preview: vec![byte] }));
            let right_receipt = admit_progress(&mut right, actor, progress_publication(109, 101, 103, 107, step, preview_sequence, JobStepOutcome::PreviewReady { preview: vec![byte] }));
            assert_eq!(left_receipt.identity(), right_receipt.identity());
            assert_eq!(left_receipt.kind(), right_receipt.kind());
            left.acknowledge(left_receipt).unwrap();
            right.acknowledge(right_receipt).unwrap();
        }
        assert_eq!(left.take(actor, 109).unwrap().preview(), right.take(actor, 109).unwrap().preview());
        left.begin_close_actor(actor).unwrap();
        right.begin_close_actor(actor).unwrap();
        drive_progress_close(&mut left);
        drive_progress_close(&mut right);
    }

#[global_allocator]
static ACTOR_PUBLICATION_TEST_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
