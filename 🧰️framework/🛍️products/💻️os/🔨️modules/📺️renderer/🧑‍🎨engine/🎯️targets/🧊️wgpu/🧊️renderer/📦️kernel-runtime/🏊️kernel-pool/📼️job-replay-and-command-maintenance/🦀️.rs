//! 📼️ Job replay and command maintenance: the job replay request and capture, the replay and command maintenance steps, the fault close and the job progress acknowledgement. See [`super`].

use super::*;

//#region 🎠️KernelRuntime
//#region 🔖️KernelPoolState
impl KernelPoolState {
    pub(super) async fn request_job_replay(&mut self, actor: ActorId, job: u64, worker_count: u16, worker_slot: u16, restore_start_ordinal: Option<u64>) -> Result<(), String> {
        if !matches!(worker_count, 1 | 2 | 4) && worker_count != self.worker_count {
            return Err("kernel: replay worker profile is not admitted by the mounted shared process route".to_string());
        }
        let process_slot = self.runtime.kernel().actor_record(actor).await.map_or(u16::MAX, |record| record.shard.0);
        if mounted_product_replay_worker_slot(process_slot, worker_count) != Some(worker_slot) {
            return Err("kernel: replay worker slot does not match the qualified mounted process profile".to_string());
        }
        let entry = self.job_replays.iter_mut().flatten().find(|entry| entry.actor == actor && entry.authority.job == job).ok_or_else(|| "kernel: replay log is not mounted".to_string())?;
        if entry.accepted_replay_sequence != restore_start_ordinal {
            return Err("kernel: replay restore/start ordinal does not match the qualified previous profile".to_string());
        }
        if entry.replay_requested {
            if !entry.pending_replay_start_is_exact(worker_count, worker_slot) {
                return Err("kernel: replay begin does not match the retained pending restore/start identity".to_string());
            }
            return Ok(());
        }
        if !entry.terminal_seen || entry.log.has_pending_work() || entry.captured.is_some() || entry.policy.is_some() {
            return Err("kernel: replay log is not sealed and quiescent".to_string());
        }
        entry.log.begin_replay(entry.authority.operation.generation).map_err(|fault| format!("kernel: replay begin rejected: {fault:?}"))?;
        entry.replay_requested = true;
        entry.replay_started = false;
        entry.replay_submit_sequence = None;
        entry.accepted_replay_sequence = None;
        entry.replay_worker_count = worker_count;
        entry.replay_worker_slot = worker_slot;
        Ok(())
    }

    pub(super) fn begin_job_replay_capture(&mut self, actor: ActorId, authority: JobTurn, request: JobReplayRequest, placement: JobPlacement, worker_count: u16, worker_slot: u16, publication: JobPublication) {
        let replay_index = self.job_replays.iter().position(|entry| entry.as_ref().is_some_and(|entry| entry.actor == actor && entry.authority.job == authority.job)).or_else(|| self.job_replays.iter().position(Option::is_none));
        let Some(replay_index) = replay_index else {
            self.retain_job_progress_rejection(JobProgressRejected::new(semio_framework_actor::JobProgressFault::Capacity, publication));
            return;
        };
        if self.job_replays[replay_index].is_none() {
            let Some(seed) = self.replay_routes.iter().flatten().find(|seed| seed.actor == actor).copied() else {
                self.retain_job_progress_rejection(JobProgressRejected::new(semio_framework_actor::JobProgressFault::Missing, publication));
                return;
            };
            let Some(recovery) = mounted_replay_recovery_registry().lock().expect("replay recovery lock").reserve() else {
                self.retain_job_progress_rejection(JobProgressRejected::new(semio_framework_actor::JobProgressFault::Capacity, publication));
                return;
            };
            let Ok(log) = JobReplayLog::new(seed.compose(request), authority.operation.generation) else {
                assert!(mounted_replay_recovery_registry().lock().expect("replay recovery lock").release(recovery));
                self.retain_job_progress_rejection(JobProgressRejected::new(semio_framework_actor::JobProgressFault::Stale, publication));
                return;
            };
            self.job_replays[replay_index] = Some(MountedJobReplay {
                actor,
                authority,
                request,
                placement,
                log: ManuallyDrop::new(log),
                captured: None,
                policy: None,
                terminal_seen: false,
                replay_requested: false,
                replay_started: false,
                replay_submit_sequence: None,
                accepted_replay_sequence: None,
                replay_worker_count: 0,
                replay_worker_slot: u16::MAX,
                recovery: Some(recovery),
            });
        }
        let entry = self.job_replays[replay_index].as_mut().expect("mounted replay slot");
        if entry.authority.operation.operation != authority.operation.operation
            || entry.authority.operation.base_revision != authority.operation.base_revision
            || entry.authority.operation.generation != authority.operation.generation
            || entry.authority.operation.seed != authority.operation.seed
            || entry.placement != placement
        {
            self.retain_job_progress_rejection(JobProgressRejected::new(semio_framework_actor::JobProgressFault::Stale, publication));
            return;
        }
        let terminal = matches!(&publication.outcome, semio_framework_actor::JobStepOutcome::Complete { .. } | semio_framework_actor::JobStepOutcome::Cancelled | semio_framework_actor::JobStepOutcome::Fault { .. });
        let now = semio_framework_job::default_now_us();
        let mut preview_sequence = publication.turn.operation.preview_sequence;
        let mut retained_progress = semio_framework_job::RetainedCloneProgress::default();
        let mut context = semio_framework_job::StepContext::new(
            semio_framework_job::OperationId(publication.turn.operation.operation),
            semio_framework_job::Generation(publication.turn.operation.generation),
            now.and_then(|now| semio_framework_job::StepBudget::from_duration(1, now, semio_framework_job::INTERACTIVE_LANE_WALL_US, JOB_REPLAY_STEP_GRANT)).unwrap_or(semio_framework_job::StepBudget::new(0, 0, JOB_REPLAY_STEP_GRANT)),
            semio_framework_job::root_cancel_token(),
            semio_framework_job::default_now_us,
            &mut preview_sequence,
            &mut retained_progress,
        );
        match entry.log.begin_capture(&mut context, actor, semio_framework_actor::JobReplaySchedule { worker_count, worker_slot, granted_fuel: TURN_BUDGET.fuel, deadline_class_ms: TURN_BUDGET.deadline_ms }, publication) {
            Ok(()) => entry.terminal_seen |= terminal,
            Err(rejected) => {
                let fault = match rejected.fault {
                    JobReplayFault::Stale | JobReplayFault::Sequence | JobReplayFault::Mismatch | JobReplayFault::Exhausted => semio_framework_actor::JobProgressFault::Stale,
                    JobReplayFault::Cancelled => semio_framework_actor::JobProgressFault::Cancelled,
                    JobReplayFault::Deadline => semio_framework_actor::JobProgressFault::Budget,
                    JobReplayFault::Busy => semio_framework_actor::JobProgressFault::Busy,
                    JobReplayFault::Capacity | JobReplayFault::PageBytes | JobReplayFault::ProcessPages => semio_framework_actor::JobProgressFault::Capacity,
                };
                self.retain_job_progress_rejection(JobProgressRejected::new(fault, rejected.into_publication()));
            }
        }
    }

    fn job_replay_maintenance_step(&mut self) -> bool {
        let Some(index) = self.job_replays.iter().position(|entry| entry.as_ref().is_some_and(|entry| entry.log.has_pending_work() || entry.captured.is_some() || entry.policy.is_some())) else {
            return false;
        };
        if let Some(policy) = self.job_replays[index].as_mut().expect("mounted replay").policy.take() {
            let entry = self.job_replays[index].as_mut().expect("mounted replay");
            let operation = entry.authority.operation;
            let now = semio_framework_job::default_now_us();
            let mut preview_sequence = operation.preview_sequence;
            let mut retained_progress = semio_framework_job::RetainedCloneProgress::default();
            let mut context = semio_framework_job::StepContext::new(
                semio_framework_job::OperationId(operation.operation),
                semio_framework_job::Generation(operation.generation),
                now.and_then(|now| semio_framework_job::StepBudget::from_duration(1, now, semio_framework_job::MAINTENANCE_LANE_WALL_US, JOB_REPLAY_STEP_GRANT)).unwrap_or(semio_framework_job::StepBudget::new(0, 0, JOB_REPLAY_STEP_GRANT)),
                semio_framework_job::root_cancel_token(),
                semio_framework_job::default_now_us,
                &mut preview_sequence,
                &mut retained_progress,
            );
            if entry.log.acknowledge_publication(&mut context, policy).is_err() {
                entry.log.begin_close();
            }
            return true;
        }
        if let Some(publication) = self.job_replays[index].as_mut().expect("mounted replay").captured.take() {
            let entry = self.job_replays[index].as_ref().expect("mounted replay");
            let (actor, authority) = (entry.actor, entry.authority);
            let policy = self.publish_captured_job_progress(actor, authority, publication);
            self.job_replays[index].as_mut().expect("mounted replay").policy = Some(policy);
            return true;
        }
        if self.job_replays[index].as_ref().expect("mounted replay").log.publication_is_ready() {
            let publication = self.job_replays[index].as_mut().expect("mounted replay").log.take_captured_publication();
            self.job_replays[index].as_mut().expect("mounted replay").captured = publication;
            return true;
        }
        let operation = self.job_replays[index].as_ref().expect("mounted replay").authority.operation;
        let now = semio_framework_job::default_now_us();
        let mut preview_sequence = operation.preview_sequence;
        let mut retained_progress = semio_framework_job::RetainedCloneProgress::default();
        let mut context = semio_framework_job::StepContext::new(
            semio_framework_job::OperationId(operation.operation),
            semio_framework_job::Generation(operation.generation),
            now.and_then(|now| semio_framework_job::StepBudget::from_duration(1, now, semio_framework_job::INTERACTIVE_LANE_WALL_US, JOB_REPLAY_STEP_GRANT)).unwrap_or(semio_framework_job::StepBudget::new(0, 0, JOB_REPLAY_STEP_GRANT)),
            semio_framework_job::root_cancel_token(),
            semio_framework_job::default_now_us,
            &mut preview_sequence,
            &mut retained_progress,
        );
        if self.job_replays[index].as_ref().expect("mounted replay").log.close_started() {
            let _ = self.job_replays[index].as_mut().expect("mounted replay").log.close_step(&mut context);
            if self.job_replays[index].as_ref().expect("mounted replay").log.terminal_is_empty() {
                let terminal = self.job_replays[index].take().expect("terminal mounted replay");
                drop(terminal);
            }
            return true;
        }
        if matches!(self.job_replays[index].as_mut().expect("mounted replay").log.maintenance_step(&mut context), JobReplayStep::Refused(_)) {
            self.job_replays[index].as_mut().expect("mounted replay").log.begin_close();
        }
        true
    }

    pub(super) fn command_maintenance_step(&mut self) -> bool {
        if mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").has_close_work() {
            let _ = mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").close_one();
            return !self.command_maintenance_pending();
        }
        if mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").has_work() {
            let step = mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").step_one(true);
            if let Some(owner) = step.request {
                if let Err(owner) = self.mount_product_replay(owner) {
                    owner.retire();
                }
            }
            let _ = step.progressed;
            return !self.command_maintenance_pending();
        }
        if mounted_replay_recovery_registry().lock().expect("replay recovery lock").has_close_work() {
            let _ = mounted_replay_recovery_registry().lock().expect("replay recovery lock").close_one();
            return !self.command_maintenance_pending();
        }
        if self.job_replay_maintenance_step() {
            return !self.command_maintenance_pending();
        }
        if let Some(close_index) = self.fault_closing_actors.iter().position(Option::is_some) {
            let actor = self.fault_closing_actors[close_index].expect("fault close actor");
            if let Some(pending_index) = self.pending_job_progress_presentations.iter().position(|pending| pending.as_ref().is_some_and(|pending| pending.receipt.identity().actor == actor)) {
                let token = self.pending_job_progress_presentations[pending_index].as_ref().expect("fault pending presentation").token;
                if !job_progress_presentation_bridge().lock().expect("job progress presentation bridge lock").cancel(token) {
                    return false;
                }
                let pending = self.pending_job_progress_presentations[pending_index].take().expect("cancelled fault presentation");
                if self.job_progress.abort(pending.receipt).is_err() {
                    let _ = self.job_progress.begin_close_actor(actor);
                }
                return false;
            }
            if self.job_progress.begin_close_actor(actor).is_ok() {
                self.fault_closing_actors[close_index] = None;
            }
            return false;
        }
        if self.job_progress.has_close_work() {
            let now = semio_framework_job::default_now_us();
            let mut preview_sequence = 0;
            let mut retained_progress = semio_framework_job::RetainedCloneProgress::default();
            let mut context = semio_framework_job::StepContext::new(
                semio_framework_job::OperationId(0),
                semio_framework_job::Generation(0),
                now.and_then(|now| semio_framework_job::StepBudget::from_duration(1, now, semio_framework_job::MAINTENANCE_LANE_WALL_US, JOB_REPLAY_STEP_GRANT)).unwrap_or(semio_framework_job::StepBudget::new(0, 0, JOB_REPLAY_STEP_GRANT)),
                semio_framework_job::root_cancel_token(),
                semio_framework_job::default_now_us,
                &mut preview_sequence,
                &mut retained_progress,
            );
            let _ = self.job_progress.close_step(&mut context);
            return !self.command_maintenance_pending();
        }
        if let Some(index) = self.rejected_job_progress.iter().position(Option::is_some) {
            let rejected = self.rejected_job_progress[index].take().expect("fixed rejected job-progress slot is occupied");
            if let Err(rejected) = self.job_progress.retain_rejected(rejected) {
                self.rejected_job_progress[index] = Some(rejected);
            }
            return !self.command_maintenance_pending();
        }
        if self.command_document_closes.has_close_work() {
            let _ = self.command_document_closes.close_one();
            return !self.command_maintenance_pending();
        }
        if self.retained_command_closes.has_close_work() {
            let _ = self.retained_command_closes.close_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES);
            return !self.retained_command_closes.has_close_work() && !self.queued_command_closes.has_close_work() && self.rejected_command_builds.terminal_is_empty();
        }
        if self.queued_command_closes.has_close_work() {
            let _ = self.queued_command_closes.close_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES);
            return !self.queued_command_closes.has_close_work() && self.rejected_command_builds.terminal_is_empty();
        }
        if !self.rejected_command_builds.terminal_is_empty() {
            return self.rejected_command_builds.close_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES).0;
        }
        if let Some(owner) = self.rejected_events.as_mut() {
            let (terminal, _) = owner.close_step();
            if terminal {
                let terminal = self.rejected_events.take().expect("terminal rejected event owner is present");
                assert!(terminal.terminal_is_empty(), "rejected event terminal witness changed before removal");
            }
            return self.rejected_events.is_none();
        }
        true
    }

    pub(super) fn command_maintenance_pending(&self) -> bool {
        mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").has_close_work()
            || mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").has_work()
            || mounted_replay_recovery_registry().lock().expect("replay recovery lock").has_close_work()
            || self.fault_closing_actors.iter().flatten().any(|actor| {
                self.pending_job_progress_presentations
                    .iter()
                    .find(|pending| pending.as_ref().is_some_and(|pending| pending.receipt.identity().actor == *actor))
                    .is_none_or(|pending| job_progress_presentation_bridge().lock().expect("job progress presentation bridge lock").can_cancel(pending.as_ref().expect("matching pending presentation").token))
            })
            || self.job_progress.has_close_work()
            || self.job_replays.iter().flatten().any(|entry| entry.log.has_pending_work() || entry.captured.is_some() || entry.policy.is_some())
            || self.rejected_job_progress.iter().any(Option::is_some)
            || self.retained_command_closes.has_close_work()
            || self.queued_command_closes.has_close_work()
            || self.command_document_closes.has_close_work()
            || !self.rejected_command_builds.terminal_is_empty()
            || self.rejected_events.is_some()
    }

    pub(super) fn begin_fault_close(&mut self, actor: ActorId) {
        if self.fault_closing_actors.iter().flatten().any(|closing| *closing == actor) {
            return;
        }
        if let Some(slot) = self.fault_closing_actors.iter_mut().find(|slot| slot.is_none()) {
            *slot = Some(actor);
        } else {
            self.job_progress.begin_close_all();
        }
    }

    pub(super) fn acknowledge_job_progress(&mut self, token: JobProgressPresentationToken) {
        if !job_progress_presentation_bridge().lock().expect("job progress presentation bridge lock").presented(token) {
            return;
        }
        let Some(pending) = self.pending_job_progress_presentations[token.index].take() else {
            let _ = job_progress_presentation_bridge().lock().expect("job progress presentation bridge lock").release_presented(token);
            return;
        };
        if pending.token != token {
            self.pending_job_progress_presentations[token.index] = Some(pending);
            return;
        }
        let receipt = pending.receipt;
        if let Err((fault, receipt)) = self.job_progress.acknowledge(receipt) {
            let actor = receipt.identity().actor;
            if self.job_progress.abort(receipt).is_err() {
                let _ = self.job_progress.begin_close_actor(actor);
            }
            crate::log_debug(&format!("kernel: presented job-progress ACK failed closed: {fault}"));
        }
        let _ = job_progress_presentation_bridge().lock().expect("job progress presentation bridge lock").release_presented(token);
    }

    pub(super) fn begin_job_progress(&mut self, actor: ActorId, turn: &JobTurn) -> Result<bool, String> {
        let live = JobProgressLiveAuthority::new(turn.operation.operation, turn.operation.base_revision, turn.operation.generation);
        if self.job_progress.live_authority(actor, turn.job) == Some(live) {
            return Ok(false);
        }
        self.job_progress.begin_operation(actor, turn.job, live).map_err(|fault| fault.to_string())?;
        Ok(true)
    }

}
//#endregion
//#endregion 🎠️KernelRuntime
