//! 🌀️ Shard loop registry and pump: the registered actors, the pump entry points, the terminal frame and completion takers, the frame epoch and the primed pump. See [`super`].

use super::*;

impl ShardLoop {
    /// 📌️ Adds an already-instantiated actor to this shard's live set — called once per
    /// `Kernel::activate` that lands on this shard. `actor.0` (the bit-packed `u64`) is the map key
    /// throughout this type: `Envelope.to`/`ShardOutcome`'s tag both carry the SAME raw id, so no
    /// `RuntimeActorId` round-trip is needed at the boundary.
    pub async fn is_registered(&self, actor: ActorId) -> bool {
        self.instances.contains_key(&actor.0)
    }

    /// ✂️ Releases an actor's instance (generation change on restart, or a real unload) — calls
    /// [`super::GuestRuntime::drop_instance`] so the pooling allocator reclaims its slab.
    pub async fn unregister(&mut self, actor: ActorId) {
        self.retire_actor_replay_owners(actor.0);
        self.allocations.remove(&actor.0);
        self.granted_budgets.remove(&actor.0);
        if let Some(instance) = self.instances.remove(&actor.0) {
            self.runtime.drop_instance(instance).await;
        }
        self.running_jobs.retain(|&(job_actor, _)| job_actor != actor.0);
        self.job_turns.retain(|&(job_actor, _), _| job_actor != actor.0);
        self.job_authorities.retain(|&(job_actor, _), _| job_actor != actor.0);
        self.job_placement.retain(|&(job_actor, _), _| job_actor != actor.0);
        self.actor_lanes.remove(&actor.0);
    }

    pub async fn actor_count(&self) -> usize {
        self.instances.len()
    }

    /// 🌀️ Admits at most one [`ShardFrame`] currently buffered on the transport and grants
    /// exactly one deferred authority. Returns `1` when an authority or replay opportunity was
    /// handled and `0` for frame-only admission or idle. Equivalent to `self.pump_primed(None)`.
    pub async fn pump(&mut self) -> Result<usize, PluginHostError> {
        self.pump_primed(None).await
    }

    /// 🤝️ Admits at most one transport frame and grants exactly one actor turn or one job-step
    /// opportunity. All other decoded authorities remain owned by this shard for a later grant.
    pub async fn drive_one(&mut self) -> ShardDrive {
        self.drive_one_primed(None).await
    }

    pub async fn drive_one_primed(&mut self, primed: Option<Vec<u8>>) -> ShardDrive {
        self.last_drive_consumed_epoch = None;
        match self.pump_primed(primed).await {
            Ok(_) if self.has_pending_work() => ShardDrive::MoreWork { consumed_epoch: self.last_drive_consumed_epoch },
            Ok(_) => ShardDrive::Idle { consumed_epoch: self.last_drive_consumed_epoch },
            Err(error) => {
                ShardDrive::Fault { error, consumed_epoch: self.last_drive_consumed_epoch, work_remains: self.has_pending_work(), terminal_frame: !self.terminal_frames.is_empty(), terminal_overflow: !self.terminal_frame_overflow.is_empty() }
            }
        }
    }

    pub fn can_accept_primed_frame(&self) -> bool {
        !self.has_pending_work() || self.has_lifecycle_retry() || self.has_unissued_authority()
    }

    pub fn has_pending_work(&self) -> bool {
        !self.pending_interactive.is_empty()
            || !self.pending_background.is_empty()
            || self.rejected_frame.is_some()
            || self.replay_seed_refusals.iter().any(Option::is_some)
            || self.replay_seeds.iter().flatten().any(|seed| !matches!(seed.phase, ReplaySeedPhase::Retained) || !self.instances.contains_key(&seed.actor))
    }

    pub fn take_terminal_frame(&mut self) -> Option<Vec<u8>> {
        self.terminal_frames.pop_front().map(|(_, bytes)| bytes)
    }

    pub fn take_terminal_frame_and_rearm(&mut self) -> (Option<Vec<u8>>, Option<u64>) {
        let frame = self.terminal_frames.pop_front().map(|(_, bytes)| bytes);
        if frame.is_none() {
            return (None, None);
        }
        let Some(overflow) = self.terminal_frame_overflow.front() else {
            return (frame, None);
        };
        if self.terminal_frames.can_admit(1, overflow.bytes.len()).is_err() {
            return (frame, None);
        }
        let (_, overflow) = self.terminal_frame_overflow.pop_front().expect("ShardLoop: terminal overflow front");
        let epoch = overflow.epoch;
        let byte_len = overflow.bytes.len();
        let _ = self.terminal_frames.try_push(overflow.bytes, byte_len).expect("ShardLoop: freed terminal capacity owns one overflow frame");
        (frame, Some(epoch))
    }

    pub fn take_terminal_completion(&mut self) -> Option<(u64, Event)> {
        if !matches!(self.terminal_authorities.front(), Some(DeferredAuthority::Event { .. })) {
            return None;
        }
        let (_, DeferredAuthority::Event { actor, event }) = self.terminal_authorities.pop_front().expect("ShardLoop: terminal completion front") else { unreachable!("ShardLoop: terminal completion kind changed") };
        Some((actor, event?))
    }

    pub fn take_terminal_authority(&mut self) -> Option<DeferredAuthority> {
        self.terminal_authorities.pop_front().map(|(_, authority)| authority)
    }

    fn claim_frame_epoch(&mut self) -> u64 {
        let epoch = self.next_frame_epoch;
        self.next_frame_epoch = self.next_frame_epoch.checked_add(1).expect("ShardLoop: ingress epoch exhausted");
        epoch
    }

    /// 🅿️ Same as [`Self::pump`], but takes one frame's bytes that were ALREADY read off the
    /// transport (e.g. by `ShardExecutor`'s blocking park on `ThreadTransport::recv_deadline`)
    /// before the normal non-blocking drain loop continues — lets a blocking wait and this
    /// non-blocking drain share the exact same transport without losing whatever woke the wait.
    /// `primed: None` (what [`Self::pump`] passes) behaves identically to the pre-`ShardFrame`
    /// `pump()`.
    ///
    /// 🔀️ Same E0502 reason as the turn-execution loop above — computed before `get_mut`.
    ///
    /// 🐕️ P1c: same watchdog treatment as `Self::execute_turn_for` — see that call site's doc.
    pub async fn pump_primed(&mut self, primed: Option<Vec<u8>>) -> Result<usize, PluginHostError> {
        let has_deferred = !self.pending_interactive.is_empty()
            || !self.pending_background.is_empty()
            || self.replay_seed_refusals.iter().any(Option::is_some)
            || self.replay_seeds.iter().flatten().any(|seed| !matches!(seed.phase, ReplaySeedPhase::Retained) || !self.instances.contains_key(&seed.actor));
        let mut frame = self.rejected_frame.take();
        if frame.is_none() {
            if let Some(bytes) = primed {
                frame = Some((self.claim_frame_epoch(), bytes));
            }
        }
        if frame.is_none() && (!has_deferred || self.has_lifecycle_retry() || self.has_unissued_authority()) {
            if let Some(bytes) = self.transport.recv().await {
                frame = Some((self.claim_frame_epoch(), bytes));
            }
        }
        if let Some((epoch, bytes)) = frame {
            match self.consume_frame(bytes).await {
                Ok(FrameAdmission::Answered) => {
                    self.last_drive_consumed_epoch = Some(epoch);
                    return Ok(1);
                }
                Ok(FrameAdmission::Deferred) => self.last_drive_consumed_epoch = Some(epoch),
                Err(rejected) => match rejected {
                    FrameAdmissionError::Full { bytes } => self.rejected_frame = Some((epoch, bytes)),
                    FrameAdmissionError::TerminalCapacity { bytes, error } => {
                        let byte_len = bytes.len();
                        let _ = self.terminal_frame_overflow.try_push(TerminalFrameOverflow { epoch, bytes }, byte_len).expect("ShardLoop: one terminal overflow owner while drive is parked");
                        return Err(error);
                    }
                    FrameAdmissionError::Fault(error) => {
                        self.last_drive_consumed_epoch = Some(epoch);
                        return Err(error);
                    }
                },
            }
        }

        self.drive_unissued_authority();
        if self.drive_replay_refusal().await? {
            return Ok(1);
        }
        let mut selected_step = None;
        if let Some(mut owner) = self.select_pending_authority() {
            if matches!(owner.authority, DeferredAuthority::Register { .. }) {
                return Ok(1);
            }
            if !self.allocation_is_current(owner.allocation) {
                if owner.allocation.is_none() && !matches!(owner.authority, DeferredAuthority::Unregister { .. }) {
                    self.send_outcome(&ShardOutcome::Fault { actor: owner.authority.actor(), message: "deferred authority has no registered actor allocation".into() }).await?;
                }
                return Ok(1);
            }
            let budget = owner.budget.take().expect("selected authority owns original issued grant");
            let lane = owner.lane;
            if let DeferredAuthority::Event { actor, event } = &mut owner.authority {
                let actor=*actor;
                let issued=*self.granted_budgets.get(&actor).ok_or_else(||PluginHostError::NativeIo(IssuedShardTurn::refusal("original Event lacks Kernel-issued input")))?.original_input();
                if owner.original_event.is_none(){let source_event=event.take().ok_or_else(||PluginHostError::NativeIo(IssuedShardTurn::refusal("original Event source is absent")))?;owner.original_event=Some(match OriginalShardEvent::new(source_event,issued){Ok(source)=>source,Err((error,source))=>{*event=Some(source);return Err(PluginHostError::NativeIo(error))}})}
                let mut original=owner.original_event.take().expect("original Event remains owned");
                let result=self.execute_turn_for(actor,&mut original,budget,lane).await;
                owner.original_event=Some(original);
                match result{Ok(true)=>self.retain_lifecycle_retry(owner)?,Ok(false)=>{},Err(error)=>{self.retain_lifecycle_retry(owner)?;return Err(error)}}
                return Ok(1);
            }
            match owner.authority {
                DeferredAuthority::Register { .. } | DeferredAuthority::Event { .. } => unreachable!("selected non-event authority"),
                DeferredAuthority::Unregister { actor } => {
                    self.unregister(actor).await;
                    return Ok(1);
                }
                DeferredAuthority::JobStep { actor, turn } => {
                    while self.replay_seeds.iter().flatten().any(|seed| seed.actor == actor && seed.job == turn.job && !matches!(seed.phase, ReplaySeedPhase::Retained)) {
                        if !self.drive_replay_seed().await? {
                            break;
                        }
                    }
                    self.accept_job_turn(actor, turn)?;
                    selected_step = Some((actor, turn, budget));
                }
                DeferredAuthority::JobReplay { actor, turn, request, worker_count, worker_slot } => {
                    self.begin_replay_seed(actor, turn, request, worker_count, worker_slot)?;
                    return Ok(1);
                }
                DeferredAuthority::Cancel(cursor) => {
                    self.cancel_one(cursor, lane).await?;
                    return Ok(1);
                }
                DeferredAuthority::Suspend { actor, operation, applied_progress } => {
                    self.suspend_one(actor, operation, applied_progress).await?;
                    return Ok(1);
                }
                DeferredAuthority::Resume { actor, operation, checkpoint } => {
                    self.resume_one(actor, operation, checkpoint).await?;
                    return Ok(1);
                }
            }
        }
        if selected_step.is_none() && self.drive_replay_seed().await? {
            return Ok(1);
        }
        let selected_step = selected_step;
        if let Some((actor_id, turn, budget)) = selected_step {
            let job = turn.job;
            let Some(authority) = self.job_authorities.get(&(actor_id, job)).copied() else {
                let message = format!("ShardLoop::pump: job {job} has no independently admitted operation authority");
                if !self.close_replay_job(actor_id, job, ReplaySeedCloseReason::Fault { stage: "step-authority", detail: message.clone() }) {
                    self.running_jobs.remove(&(actor_id, job));
                    self.job_turns.remove(&(actor_id, job));
                    self.job_placement.remove(&(actor_id, job));
                }
                self.send_outcome(&ShardOutcome::Fault { actor: actor_id, message }).await?;
                return Ok(1);
            };
            let Some(placement) = self.job_placement.get(&(actor_id, job)).copied() else {
                let message = format!("ShardLoop::pump: job {job} has no retained placement authority");
                if !self.close_replay_job(actor_id, job, ReplaySeedCloseReason::Fault { stage: "step-placement", detail: message.clone() }) {
                    self.running_jobs.remove(&(actor_id, job));
                    self.job_turns.remove(&(actor_id, job));
                    self.job_authorities.remove(&(actor_id, job));
                }
                self.send_outcome(&ShardOutcome::Fault { actor: actor_id, message }).await?;
                return Ok(1);
            };
            if turn.step_sequence == u64::MAX || turn.operation.preview_sequence == u64::MAX {
                let message = format!("ShardLoop::pump: job {job} exhausted its checked replay sequence");
                if !self.close_replay_job(actor_id, job, ReplaySeedCloseReason::Fault { stage: "step-sequence", detail: message.clone() }) {
                    self.running_jobs.remove(&(actor_id, job));
                    self.job_turns.remove(&(actor_id, job));
                    self.job_authorities.remove(&(actor_id, job));
                    self.job_placement.remove(&(actor_id, job));
                }
                self.send_outcome(&ShardOutcome::Fault { actor: actor_id, message }).await?;
                return Ok(1);
            }
            let job_budget = job_budget_from_grant(&budget);
            let actor_lane = self.actor_lane(actor_id);
            let watchdog_stage = interactive_stage_for(actor_lane);
            let replayable = self.replay_seeds.iter().flatten().any(|seed| seed.actor == actor_id && seed.job == job && seed.replayable);
            let Some(instance) = self.instances.get_mut(&actor_id) else {
                self.close_replay_job(actor_id, job, ReplaySeedCloseReason::ActorLost);
                self.send_outcome(&ShardOutcome::Fault { actor: actor_id, message: format!("ShardLoop::pump: actor {actor_id} is not registered on this shard") }).await?;
                return Ok(1);
            };
            let job_outcome = {
                let _watchdog = Watchdog::start("plugin-host.shard.step_job", OperationId(actor_id), Generation(job), watchdog_stage);
                self.runtime.step_job(instance, job, job_budget.await).await
            };
            let outcome = match job_outcome {
                Ok(step) => {
                    let step_outcome = match step {
                        JobStep::Running { progress: Some(preview) } => JobStepOutcome::PreviewReady { preview },
                        JobStep::Running { progress: None } => JobStepOutcome::Yield,
                        JobStep::Done { output } => match if replayable { self.identities.get_mut(self.allocations.get(&actor_id).expect("registered actor allocation")).expect("registered actor owns original identity").checkpoint(&self.runtime,instance).await } else { Ok(Vec::new()) } {
                            Ok(state) => {
                                self.close_replay_job(actor_id, job, ReplaySeedCloseReason::Completed);
                                defer_completion(
                                    &mut self.pending_interactive,
                                    &mut self.pending_background,
                                    &mut self.terminal_authorities,
                                    actor_lane,
                                    self.allocations.get(&actor_id).copied(),
                                    self.granted_budgets.get(&actor_id).map(|turn| turn.budget),
                                    actor_id,
                                    Event::JobCompleted { job, result: RequestOutcome::Ok(output.clone()) },
                                )?;
                                JobStepOutcome::Complete { candidate: JobCommitCandidate { state, output } }
                            }
                            Err(fault) => {
                                self.close_replay_job(actor_id, job, ReplaySeedCloseReason::Fault { stage: "terminal-checkpoint", detail: fault.to_string() });
                                let detail = start_job_fault_bytes(&TurnFault::from(fault));
                                defer_completion(
                                    &mut self.pending_interactive,
                                    &mut self.pending_background,
                                    &mut self.terminal_authorities,
                                    actor_lane,
                                    self.allocations.get(&actor_id).copied(),
                                    self.granted_budgets.get(&actor_id).map(|turn| turn.budget),
                                    actor_id,
                                    Event::JobCompleted { job, result: RequestOutcome::Err(detail.clone()) },
                                )?;
                                JobStepOutcome::Fault { detail }
                            }
                        },
                        JobStep::Failed { error } => {
                            self.close_replay_job(actor_id, job, ReplaySeedCloseReason::Fault { stage: "guest-failed", detail: String::from_utf8_lossy(&error).into_owned() });
                            defer_completion(
                                &mut self.pending_interactive,
                                &mut self.pending_background,
                                &mut self.terminal_authorities,
                                actor_lane,
                                self.allocations.get(&actor_id).copied(),
                                    self.granted_budgets.get(&actor_id).map(|turn| turn.budget),
                                actor_id,
                                Event::JobCompleted { job, result: RequestOutcome::Err(error.clone()) },
                            )?;
                            JobStepOutcome::Fault { detail: error }
                        }
                    };
                    let mut published_turn = turn;
                    if matches!(step_outcome, JobStepOutcome::PreviewReady { .. }) {
                        published_turn.operation.preview_sequence += 1;
                    }
                    if !matches!(step_outcome, JobStepOutcome::Complete { .. } | JobStepOutcome::Cancelled | JobStepOutcome::Fault { .. }) {
                        self.job_turns.insert((actor_id, job), JobTurn { step_sequence: turn.step_sequence + 1, ..published_turn });
                    }
                    ShardOutcome::Job { actor: actor_id, authority: authority.turn, request: authority.request, placement, publication: JobPublication { turn: published_turn, outcome: step_outcome } }
                }
                Err(fault) => {
                    self.close_replay_job(actor_id, job, ReplaySeedCloseReason::Fault { stage: "step-job", detail: turn_fault_message(&fault) });
                    defer_completion(
                        &mut self.pending_interactive,
                        &mut self.pending_background,
                        &mut self.terminal_authorities,
                        actor_lane,
                        self.allocations.get(&actor_id).copied(),
                                    self.granted_budgets.get(&actor_id).map(|turn| turn.budget),
                        actor_id,
                        Event::JobCompleted { job, result: RequestOutcome::Err(start_job_fault_bytes(&fault)) },
                    )?;
                    ShardOutcome::Fault { actor: actor_id, message: turn_fault_message(&fault) }
                }
            };
            self.send_outcome(&outcome).await?;
            return Ok(1);
        }

        Ok(0)
    }

}
