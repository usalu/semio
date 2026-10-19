//! 🚀️ Turn dispatch and pool runner: the single turn run, the turn dispatch and result application, the ui patch, the retained surface steps, the maintenance yield, the pool runner and the pool futures. See [`super`].

use super::*;

//#region 🎠️KernelRuntime
//#region 🔖️KernelPoolState
impl KernelPoolState {
    /// 🎠️ terra-kernel-loop: the real loop the packet brief's item 1 asks for — `Kernel::submit`
    /// (honouring `Backpressure`; a non-`Accept` result is logged rather than silently ignored,
    /// but does not abort the turn since `Coalesced`/`Dropped` both still leave AT LEAST one
    /// envelope queued and `Rejected` on a freshly-activated actor's own generous Interactive-lane
    /// mailbox should not occur in practice) → `Kernel::tick` → dispatch to the actor's OWN pinned
    /// logical shard executor on the shared pool → wait for that shard's `ShardOutcome` →
    /// `Kernel::complete` (closing the bridging
    /// gap this method's OWN doc comment used to flag as unreached) → hand the result to
    /// `apply_turn_result`. Loops `tick_and_dispatch` until nothing is left to grant — normally
    /// one iteration (this host submits for exactly one actor per call), but `Kernel::tick`'s DRR
    /// scheduler is global, so this stays correct if that ever changes.
    ///
    /// 🕳️ Honest gap: `Kernel::commit_frame`/`apply_scene_patch` are NOT called here —
    /// `KernelPoolState::activate` (via `ParallelRuntime::activate`) still passes `window: None`
    /// for every actor, so `Kernel`'s own `SceneStore` would stay permanently empty regardless;
    /// this host's UI pipeline already has its own frame-boundary mechanism (`retained`/
    /// `apply_ui_patch`, "item 4" of the original H3 packet). Wiring per-window `Kernel::
    /// commit_frame` for real would mean migrating THIS host's whole UI-patch pipeline onto
    /// `Kernel`'s `SceneStore`, a substantially larger, separate refactor out of this packet's
    /// scope (see `📓️terra-kernel-loop-report.md`'s own gaps section).
    ///
    /// 🎯️ One granted turn, answering what the guest still owes: `None` once it settled, else the
    /// events of the continuation turn (see [`Self::run_turn`]). A granted actor the shard reports
    /// [`ShardOutcome::Preempted`] — ours or one `Kernel::tick` granted beside it — is resumed right
    /// here with an `Event::Wake` envelope (the shard's "resume with no events"), so the tick loop
    /// keeps granting it until its turn returns — after each of OUR actor's slices the request at the
    /// head of the queue may run first ([`Self::yield_slice`]), and only an explicit cancel ends the turn
    /// ([`Self::turn_cancelled`]).
    pub(super) async fn run_turn_once(&mut self, actor: ActorId, instance: u32, events: Vec<Event>) -> Result<(ExchangeOutcome, Option<Vec<Event>>), ProgramFault> {
        let mut envelopes = Vec::with_capacity(events.len().max(1));
        let mut replay_start_index = None;
        if events.is_empty() {
            let replay_index = self.job_replays.iter().position(|entry| entry.as_ref().is_some_and(|entry| entry.actor == actor && entry.replay_requested));
            let (sequence, payload) = if let Some(index) = replay_index {
                let entry = self.job_replays[index].as_mut().expect("requested replay remains mounted");
                let Some(turn) = entry.log.expected_turn() else {
                    entry.replay_requested = false;
                    entry.replay_submit_sequence = None;
                    return Ok((ExchangeOutcome { frames: Vec::new(), surfaces: UiFixedList::default(), effects: Vec::new(), command_ingress: semio_framework::kernel::CommandIngressStatus::Idle, typed_results: Vec::new() }, None));
                };
                if entry.replay_started {
                    (next_seq()?, Payload::JobStep { turn })
                } else {
                    let sequence = entry.replay_submission_sequence()?;
                    replay_start_index = Some(index);
                    (sequence, Payload::JobReplay { turn, request: entry.request, worker_count: entry.replay_worker_count, worker_slot: entry.replay_worker_slot })
                }
            } else {
                (next_seq()?, Payload::Event { bytes: serde_json::to_vec(&Event::Wake).map_err(|error| error.to_string())? })
            };
            envelopes.push(Envelope { to: actor, from: Origin::Kernel, lane: Lane::Interactive, seq: sequence, deadline_ms: None, coalesce: None, cancel_of: None, payload });
        } else {
            for event in &events {
                envelopes.push(Envelope {
                    to: actor,
                    from: Origin::Kernel,
                    lane: Lane::Interactive,
                    seq: next_seq()?,
                    deadline_ms: None,
                    coalesce: None,
                    cancel_of: None,
                    payload: Payload::Event { bytes: serde_json::to_vec(event).map_err(|error| error.to_string())? },
                });
            }
        }
        self.dispatch_turn(actor, instance, envelopes, replay_start_index).await
    }

    /// 🧰️ One turn of `actor`'s oldest live [`ReservedToolJob`]: its next `Payload::JobStep`, or, once
    /// the job ended, no envelope at all and only the wait for the deferred `Event::JobCompleted` turn
    /// [`Self::dispatch_turn`] counts as owed.
    pub(super) async fn run_reserved_job_once(&mut self, actor: ActorId, instance: u32) -> Result<(ExchangeOutcome, Option<Vec<Event>>), ProgramFault> {
        let Some(oldest) = self.reserved_jobs.iter().flatten().filter(|job| job.actor == actor).min_by_key(|job| job.job).copied() else {
            return Ok((ExchangeOutcome { frames: Vec::new(), surfaces: UiFixedList::default(), effects: Vec::new(), command_ingress: semio_framework::kernel::CommandIngressStatus::Idle, typed_results: Vec::new() }, None));
        };
        let envelopes = match oldest.step {
            Some(turn) => vec![Envelope { to: actor, from: Origin::Kernel, lane: Lane::Interactive, seq: next_seq()?, deadline_ms: None, coalesce: None, cancel_of: None, payload: Payload::JobStep { turn } }],
            None => Vec::new(),
        };
        self.dispatch_turn(actor, instance, envelopes, None).await
    }

    /// 🚚️ Submits `envelopes` to `actor`, then grants and collects until nothing is left to grant and
    /// no deferred reserved-job completion of `actor` is owed. An outcome no grant of this call asked
    /// for is such a completion: the shard runs a job's `Event::JobCompleted` turn by itself.
    ///
    /// 🎠️ terra-kernel-loop: a trap must ALSO reach `Kernel::complete` — otherwise
    /// the failure ladder (`FailureState::on_signal`) never sees it, staying just as
    /// inert for the trap path as `Kernel::complete` being uncalled at all used to
    /// leave it. `ShardOutcome::Fault` carries no `TurnResult` (no `fuel_used`, no
    /// `Effect`s — the turn never returned one), so a minimal `Faulted` `TurnResult`
    /// is synthesized from its `message` — the same shape `apply_turn_result`'s
    /// caller already treats a fault as `TurnStatus::Faulted` for retry purposes.
    ///
    /// 🚧️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (K1, landed mid-session):
    /// `ShardOutcome` also carries checkpoint/resume/cancel control responses.
    /// This kernel pool state machine does not consume those control responses;
    /// a `Job` publication is handled by the explicit owner-moving arm above.
    /// Any remaining control response reaching here — for `actor` OR any other
    /// actor `Kernel::tick` happened to grant in the SAME call — is silently
    /// ignored rather than aborting an otherwise-successful turn; unlike the
    /// ORIGINAL `Fault`/`Job` handling this replaces, this loop may observe outcomes
    /// for actors OTHER than `actor` (DRR is global), so those must not error out.
    async fn dispatch_turn(&mut self, actor: ActorId, instance: u32, envelopes: Vec<Envelope>, replay_start_index: Option<usize>) -> Result<(ExchangeOutcome, Option<Vec<Event>>), ProgramFault> {
        let stepped_reserved_job = envelopes.iter().find_map(|envelope| match &envelope.payload {
            Payload::JobStep { turn } => self.reserved_jobs.iter().flatten().any(|job| job.actor == actor && job.job == turn.job).then_some(turn.job),
            _ => None,
        });
        for envelope in &envelopes {
            let began_job = match &envelope.payload {
                Payload::JobStep { turn } if stepped_reserved_job != Some(turn.job) => self.begin_job_progress(actor, turn)?,
                Payload::Cancel { .. } => {
                    self.job_progress.begin_close_actor(actor).map_err(|fault| fault.to_string())?;
                    false
                }
                _ => false,
            };
            let pressure = self.runtime.submit(envelope).await;
            if matches!(pressure, Backpressure::Accept) {
                if let Some(index) = replay_start_index {
                    let entry = self.job_replays[index].as_mut().expect("accepted replay start remains mounted");
                    entry.accept_replay_submission(envelope.seq)?;
                }
            } else {
                if began_job {
                    let _ = self.job_progress.begin_close_actor(actor);
                }
                crate::log_debug(&format!("kernel: run_turn submit for actor {} was not Accept-ed (mailbox pressure)", actor.0));
                if replay_start_index.is_some() {
                    return Ok((ExchangeOutcome { frames: Vec::new(), surfaces: UiFixedList::default(), effects: Vec::new(), command_ingress: semio_framework::kernel::CommandIngressStatus::Idle, typed_results: Vec::new() }, None));
                }
            }
        }
        let mut turn_results: Vec<TurnResult> = Vec::new();
        let mut fault: Option<String> = None;
        let mut replay_capture_started = false;
        let mut reserved_step_published = false;
        loop {
            self.now_ms += 1;
            let decision = self.runtime.tick_and_dispatch(self.now_ms, |_actor, original| crate::actor_budget_from_turn_budget(TURN_BUDGET, original)).await?;
            let mut granted: Vec<u64> = decision.run.iter().map(|grant| grant.actor.0).collect();
            let expected = granted.len() + self.reserved_jobs.iter().flatten().filter(|job| job.actor == actor && job.step.is_none()).count();
            if expected == 0 {
                break;
            }
            let outcomes = self.runtime.wait_for_outcomes(expected, RUN_TURN_OUTCOME_TIMEOUT);
            if outcomes.len() < granted.len() {
                return Err(("kernel: shard produced no outcome for this turn".to_string()).into());
            }
            if outcomes.len() < expected {
                return Err((format!("kernel: actor {}'s deferred reserved-job completion turn never arrived", actor.0)).into());
            }
            for outcome in outcomes {
                let reported = outcome.actor();
                if let Some(index) = granted.iter().position(|granted| *granted == reported) {
                    granted.swap_remove(index);
                } else if let Some(slot) = self.reserved_jobs.iter_mut().find(|slot| slot.as_ref().is_some_and(|job| job.actor.0 == reported && job.step.is_none())) {
                    *slot = None;
                }
                match outcome {
                    ShardOutcome::OriginalCall{actor:reported,returned,continuation,call}=>{
                        self.runtime.kernel_mut().receive_original_call(ActorId(reported),returned,continuation).map_err(ProgramFault::from)?;
                        if let Some(call)=call{if matches!(call.kind,semio_framework_plugin_host::original_actor_context::OriginalActorFixedKind::Failure{..}){return Err(call.into())}}
                    }
                    ShardOutcome::Turn { actor: reported, result, jobs } => {
                        self.runtime.complete_actor(ActorId(reported), &result, self.now_ms).await.map_err(ProgramFault::from)?;
                        if reported == actor.0 {
                            let decoded = decode_actor_turn_result(&result, reported)?;
                            self.admit_reserved_jobs(actor, &decoded.effects, &jobs)?;
                            turn_results.push(decoded);
                        } else {
                            let _ = semio_framework::kernel::close_ui_turn_patch_transport_session_one(reported);
                            let _ = semio_framework::kernel::close_ui_turn_patch_transport_one();
                        }
                    }
                    ShardOutcome::Job { actor: reported, authority, publication, .. } if self.reserved_jobs.iter().flatten().any(|job| job.actor.0 == reported && job.job == authority.job) => {
                        let next = match publication.outcome {
                            semio_framework_actor::JobStepOutcome::Complete { .. } | semio_framework_actor::JobStepOutcome::Cancelled | semio_framework_actor::JobStepOutcome::Fault { .. } => None,
                            semio_framework_actor::JobStepOutcome::Yield | semio_framework_actor::JobStepOutcome::PreviewReady { .. } | semio_framework_actor::JobStepOutcome::CheckpointReady { .. } => {
                                Some(JobTurn { step_sequence: publication.turn.step_sequence.saturating_add(1), ..publication.turn })
                            }
                        };
                        if let Some(job) = self.reserved_jobs.iter_mut().flatten().find(|job| job.actor.0 == reported && job.job == authority.job) {
                            job.step = next;
                            reserved_step_published = true;
                        }
                    }
                    ShardOutcome::Job { actor: reported, authority, request, placement, publication } => {
                        let worker_slot = self.runtime.kernel().actor_record(ActorId(reported)).await.map_or(u16::MAX, |record| record.shard.0);
                        self.begin_job_replay_capture(ActorId(reported), authority, request, placement, self.worker_count, worker_slot, publication);
                        replay_capture_started = true;
                    }
                    ShardOutcome::Preempted { actor: reported } => {
                        if reported == actor.0 {
                            if self.turn_cancelled() {
                                return Err((self.retire_cancelled_turn(actor).await).into());
                            }
                            self.yield_slice(actor, instance).await;
                        }
                        let resume = Envelope {
                            to: ActorId(reported),
                            from: Origin::Kernel,
                            lane: Lane::Interactive,
                            seq: next_seq()?,
                            deadline_ms: None,
                            coalesce: None,
                            cancel_of: None,
                            payload: Payload::Event { bytes: serde_json::to_vec(&Event::Wake).map_err(|error| error.to_string())? },
                        };
                        if !matches!(self.runtime.submit(&resume).await, Backpressure::Accept) {
                            return Err((format!("kernel: preempted actor {reported} refused its resume envelope")).into());
                        }
                    }
                    ShardOutcome::Resumed { actor: reported, operation } if reported == actor.0 => {
                        let valid = self.job_replays.iter().flatten().any(|entry| {
                            entry.actor == actor
                                && entry.replay_started
                                && entry.authority.operation.operation == operation.operation
                                && entry.authority.operation.base_revision == operation.base_revision
                                && entry.authority.operation.generation == operation.generation
                                && entry.authority.operation.seed == operation.seed
                        });
                        if !valid {
                            return Err(("kernel: replay restart returned stale operation identity".to_string()).into());
                        }
                        replay_capture_started = true;
                    }
                    ShardOutcome::Fault { actor: reported, message } => {
                        self.begin_fault_close(ActorId(reported));
                        let faulted = semio_framework_actor::TurnResult {
                            cold_pair_ingress: Default::default(),
                            ui_patches: Vec::new(),
                            effects: Vec::new(),
                            command_ingress: Vec::new(),
                            lifecycle_receipt: None,
                            ui_patch_receipt: None,
                            next_wake: None,
                            status: semio_framework_actor::TurnStatus::Faulted { detail: message.as_bytes().to_vec() },
                            usage: semio_framework_actor::Usage::default(),
                        };
                        let _ = self.runtime.complete_actor(ActorId(reported), &faulted, self.now_ms).await;
                        if reported == actor.0 {
                            if let Some(slot) = self.reserved_jobs.iter_mut().find(|slot| slot.as_ref().is_some_and(|job| job.actor == actor && Some(job.job) == stepped_reserved_job)) {
                                *slot = None;
                            }
                            fault = Some(match self.runtime.take_shard_failure() {
                                Some(cause) => format!("{message} (retained shard failure: {cause})"),
                                None => message,
                            });
                        }
                    }
                    _ => {}
                }
            }
            if replay_capture_started {
                break;
            }
        }
        if let Some(message) = fault {
            return Err((message).into());
        }
        if turn_results.is_empty() {
            return if replay_capture_started || reserved_step_published {
                Ok((ExchangeOutcome { frames: Vec::new(), surfaces: UiFixedList::default(), effects: Vec::new(), command_ingress: semio_framework::kernel::CommandIngressStatus::Idle, typed_results: Vec::new() }, None))
            } else {
                Err(ProgramFault::from("kernel: shard produced no outcome for this turn"))
            };
        }
        let mut settled: Option<ExchangeOutcome> = None;
        let mut owed = Vec::new();
        let mut more_work = false;
        for result in turn_results {
            owed.extend(result.lifecycle_receipt.map(|receipt| Event::InstanceLifecycleAck(semio_framework::kernel::ActorInstanceLifecycleAck { receipt })));
            more_work = matches!(result.status, semio_framework::kernel::TurnStatus::MoreWork)
                && matches!(result.command_ingress, semio_framework::kernel::CommandIngressStatus::Idle)
                && matches!(result.cold_pair_ingress, semio_framework::kernel::ColdPairIngressStatus::Idle);
            let outcome = self.apply_turn_result(actor, instance, result).await?;
            owed.extend(
                outcome.typed_results.iter().map(|page| Event::Message { source: MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId(instance.to_string()) }, payload: TypedOperationResultPage::encode_ack(page.token) }),
            );
            match settled.as_mut() {
                Some(earlier) => earlier.absorb(outcome)?,
                None => settled = Some(outcome),
            }
        }
        let outcome = settled.ok_or_else(|| "kernel: settled turn results produced no outcome".to_string())?;
        let continuation = if !owed.is_empty() { Some(owed) } else { more_work.then(Vec::new) };
        Ok((outcome, continuation))
    }

    /// 🎠️ terra-kernel-loop: `Kernel::complete` (the bridge this doc comment used to flag as
    /// unreached — "bridging the two needs a real pack-encode step this packet didn't reach")
    /// is now genuinely called, from `run_turn`, for EVERY `ShardOutcome::Turn` a tick grants
    /// (including `actor`'s own, before this method is even invoked) — so `Kernel`'s
    /// failure-ladder/metrics bookkeeping is live for this host now, not skipped.
    async fn apply_turn_result(&mut self, actor: ActorId, instance: u32, mut result: TurnResult) -> Result<ExchangeOutcome, String> {
        let _ = actor;
        let mut frames = Vec::new();
        let mut effects = Vec::new();
        let mut typed_results = Vec::new();
        for effect in result.effects {
            if matches!(&effect, Effect::SpawnJob { kind, .. } if kind == semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND) {
                continue;
            }
            if let Effect::SendMessage { target: MessageEndpoint::Shell { instance: target_instance }, payload } = &effect {
                if target_instance.0 == instance.to_string() {
                    if let Some(page) = TypedOperationResultPage::decode_guest_message(payload).filter(|page| page.token.receiver == instance) {
                        typed_results.push(page);
                        continue;
                    }
                    if let Ok(frame) = protocol::decode_app_frame(payload).await {
                        frames.push(frame);
                        continue;
                    }
                }
            }
            effects.push(effect);
        }
        let mut surfaces = UiFixedList::default();
        let patch_receipt = result.ui_patch_receipt;
        loop {
            match result.ui_patches.try_transfer_one(|patch| self.apply_ui_patch(instance, patch, patch_receipt)) {
                semio_framework::kernel::UiTurnPatchTransfer::Empty => break,
                semio_framework::kernel::UiTurnPatchTransfer::Transferred(()) => {}
                semio_framework::kernel::UiTurnPatchTransfer::Refused => return Err("renderer retained patch admission refused without consuming the exact turn owner".to_string()),
            }
        }
        let _ = self.advance_retained_one(instance, None, &mut surfaces);
        Ok(ExchangeOutcome { frames, surfaces, effects, command_ingress: result.command_ingress, typed_results })
    }

    fn apply_ui_patch(&mut self, instance: u32, patch: KernelUiPatch, receipt: Option<semio_framework::kernel::ActorUiPatchReceipt>) -> Result<(), KernelUiPatch> {
        let surface = patch.surface.clone();
        let index = match self.retained.try_admit(instance, surface.clone()) {
            Ok(index) => index,
            Err(surface) => {
                let Ok(reservation) = reserve_seq() else {
                    return Err(patch);
                };
                let generation = reservation.generation();
                let rejection =
                    PendingSurfaceRejection { generation, instance, surface, revision: UiRevision::default(), reason: ui_contract::UiText::try_from_str("retained surface capacity exhausted").unwrap_or_default(), patch: Some(patch), receipt };
                match self.pending_rejections.try_admit(rejection) {
                    Ok(()) => {
                        let _ = reservation.commit();
                    }
                    Err(mut rejection) => {
                        return rejection.patch.take().map_or(Ok(()), Err);
                    }
                }
                return Ok(());
            }
        };
        let Some(slot) = self.retained.slots[index].as_mut() else { return Err(patch) };
        slot.owner.patch_receipt = receipt;
        let rejected = slot.owner.admit_patch(patch).err().and_then(|patch| slot.owner.rejected_patches.try_push(patch).err());
        if let Some(patch) = rejected {
            let Ok(reservation) = reserve_seq() else {
                return Err(patch);
            };
            let generation = reservation.generation();
            let rejection = PendingSurfaceRejection { generation, instance, surface, revision: UiRevision::default(), reason: ui_contract::UiText::try_from_str("retained patch queues exhausted").unwrap_or_default(), receipt, patch: Some(patch) };
            match self.pending_rejections.try_admit(rejection) {
                Ok(()) => {
                    let _ = reservation.commit();
                }
                Err(mut rejection) => {
                    return rejection.patch.take().map_or(Ok(()), Err);
                }
            }
        }
        Ok(())
    }

    fn advance_retained_surface_one(&mut self, instance: u32, surface: SurfaceId) -> Result<ExchangeOutcome, ProgramFault> {
        let mut surfaces = UiFixedList::default();
        self.advance_retained_one(instance, Some(&surface), &mut surfaces)?;
        Ok(ExchangeOutcome { frames: Vec::new(), surfaces, effects: Vec::new(), command_ingress: semio_framework::kernel::CommandIngressStatus::Idle, typed_results: Vec::new() })
    }

    fn advance_retained_one(&mut self, instance: u32, requested: Option<&SurfaceId>, out: &mut UiFixedList<ExchangeSurfaceDocument, UI_DOCUMENT_LEASE_SLOTS>) -> Result<(), String> {
        let index = requested.and_then(|surface| self.retained.index(instance, surface)).or_else(|| {
            self.retained.slots.iter().position(|slot| {
                slot.as_ref().is_some_and(|slot| {
                    slot.instance == instance
                        && (slot.owner.patch.is_some()
                            || !slot.owner.queued_patches.is_empty()
                            || !slot.owner.rejected_patches.is_empty()
                            || slot.owner.build.is_some()
                            || slot.owner.build_pending
                            || slot.owner.closing.is_some()
                            || slot.owner.exchange_closing.is_some())
                })
            })
        });
        let Some(index) = index else {
            if let Some(surface) = requested {
                return Err(format!("retained surface '{}' is not admitted", surface.as_ref()));
            }
            if self.semantic_close_document_lane {
                let _ = ui_contract::close_ui_document_page_one();
            } else {
                let _ = semio_framework_ui_contract_validation::close_ui_patch_owner_one();
            }
            self.semantic_close_document_lane = !self.semantic_close_document_lane;
            return Ok(());
        };
        let Some(slot) = self.retained.slots[index].as_mut() else { return Err("retained surface slot is vacant".to_string()) };
        let generation = slot.generation;
        let surface = slot.surface.clone();
        let retained = &mut slot.owner;
        let active =
            retained.patch.is_some() || !retained.queued_patches.is_empty() || !retained.rejected_patches.is_empty() || retained.build.is_some() || retained.build_pending || retained.closing.is_some() || retained.exchange_closing.is_some();
        if !active {
            if let Some(rejection) = self.pending_rejections.get(instance, &surface).filter(|rejection| rejection.generation == generation) {
                if !retained.has_published() {
                    return Err(rejection.reason.to_string());
                }
            }
        }
        if active {
            if retained.patch.is_some() || !retained.queued_patches.is_empty() {
                if let Some(rejection) = retained.advance_patch_one() {
                    let reason = rejection.1.to_string();
                    let owner = PendingSurfaceRejection { generation, instance, surface: surface.clone(), revision: rejection.0, reason: rejection.1, patch: None, receipt: retained.patch_receipt };
                    let _ = self.pending_rejections.try_upsert(owner);
                    if requested.is_some() && !retained.has_published() {
                        return Err(reason);
                    }
                } else if retained.patch.is_none() {
                    let _ = self.pending_rejections.remove(instance, &surface);
                }
            } else {
                retained.advance_document_one();
            }
        }
        if retained.exchange_closing.is_none() {
            if let Some(document) = retained.try_alias() {
                if let Err(rejected) = out.try_push(ExchangeSurfaceDocument { surface: surface.clone(), document }) {
                    retained.exchange_closing = Some(rejected.document);
                    if requested.is_some() {
                        return Err("retained surface exchange capacity exhausted".to_string());
                    }
                }
            }
        }
        Ok(())
    }
}

async fn yield_kernel_maintenance_turn() {
    let mut yielded = false;
    std::future::poll_fn(|cx| {
        if yielded {
            Poll::Ready(())
        } else {
            yielded = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    })
    .await
}

pub async fn run_kernel_pool(queue: Arc<KernelRequestQueue>,original_clock:fn()->Option<u64>) {
    let identity_policy=semio_framework_pack_json::from_json_str(include_str!("../../../../../../../../🔌️plugin/🖥️host/🧵️shard/🪪️identity/⚙️configuration/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).expect("authored native shard identity policy");
    let mut state = KernelPoolState::new(queue.clone(),identity_policy,original_clock).await;
    loop {
        if state.command_maintenance_pending() {
            let _ = state.command_maintenance_step();
            yield_kernel_maintenance_turn().await;
            continue;
        }
        let ready = queue.try_next();
        if ready.is_none() {
            if let Some(instance) = state.product_replay_pending_instance() {
                if let Err(error) = state.advance_product_replay(instance).await {
                    crate::log_debug(&format!("kernel: mounted product replay fault: {error}"));
                    state.abort_product_replay_one(instance);
                }
                yield_kernel_maintenance_turn().await;
                continue;
            }
        }
        let (request, slot) = match ready {
            Some(ready) => ready,
            None => queue.next().await,
        };
        if let Some(outcome) = state.serve(request, &slot).await {
            slot.deliver(outcome);
        }
    }
}

impl KernelPoolState {
    /// 🎛️ Serves one request and answers the outcome its slot receives — `None` for a request that owns its own
    /// completion (a close step, an acknowledgement, a retained rejection drain). The loop and a slice gap
    /// ([`Self::yield_slice`]) both serve through here; the returned future is boxed because a request served
    /// in a gap is itself served by a turn this future drives.
    pub(super) fn serve<'a>(&'a mut self, request: KernelRequest, slot: &Arc<ResponseSlot>) -> Pin<Box<dyn Future<Output = Option<KernelOutcome>> + Send + 'a>> {
        let cancellable = matches!(request, KernelRequest::CreateApp { .. });
        self.serving.push((slot.clone(), cancellable));
        Box::pin(async move {
            let outcome = self.serve_request(request).await;
            self.serving.pop();
            outcome
        })
    }

    async fn serve_request(&mut self, request: KernelRequest) -> Option<KernelOutcome> {
        Some(match request {
            KernelRequest::CreateApp { owner } => {
                KernelOutcome::Created(self.create_app(owner).await)
            }
            KernelRequest::DestroyApp { owner } => {
                if self.destroy_app_step(owner.instance).await {
                    owner.finish(KernelCloseStatus::Complete);
                } else {
                    owner.phase.store(KERNEL_CLOSE_READY, std::sync::atomic::Ordering::Release);
                    let _ = owner.try_schedule();
                }
                return None;
            }
            KernelRequest::CloseRealm { owner } => {
                if self.close_realm_progress_step() {
                    owner.finish(KernelCloseStatus::Complete);
                } else {
                    owner.phase.store(KERNEL_CLOSE_READY, std::sync::atomic::Ordering::Release);
                    let _ = owner.try_schedule();
                }
                return None;
            }
            KernelRequest::AcknowledgeJobProgress { token } => {
                self.acknowledge_job_progress(token);
                return None;
            }
            KernelRequest::Exchange { instance, event } => KernelOutcome::Exchanged(self.exchange(instance, vec![event.into_event()]).await),
            KernelRequest::AdvanceRetained { instance, surface } => KernelOutcome::Exchanged(self.advance_retained_surface_one(instance, surface)),
            KernelRequest::MountProductReplay { owner } => KernelOutcome::ProductReplayMounted(self.mount_product_replay(owner)),
            KernelRequest::RetireProductReplay { owner } => {
                owner.retire();
                KernelOutcome::ProductReplayMounted(Ok(()))
            }
            KernelRequest::RetireProductReplayRefusal { owner } => {
                owner.retire();
                KernelOutcome::ProductReplayMounted(Ok(()))
            }
            KernelRequest::AdvanceProductReplay { instance } => KernelOutcome::Exchanged(self.advance_product_replay(instance).await),
            KernelRequest::ExchangeCommands { instance, driver } => KernelOutcome::Exchanged(self.exchange_commands(instance, driver).await),
            KernelRequest::CloseRejectedCommandBuild { key, owner } => {
                assert!(self.rejected_command_builds.can_insert(key), "worker drains the prior rejected command build before dequeuing another request");
                self.rejected_command_builds.insert_admitted(key, owner);
                return None;
            }
            KernelRequest::CloseRejectedEvents { owner } => {
                assert!(self.rejected_events.is_none(), "worker drains the prior rejected event owner before dequeuing another request");
                self.rejected_events = Some(owner);
                return None;
            }
        })
    }
}
//#endregion
//#endregion 🎠️KernelRuntime
