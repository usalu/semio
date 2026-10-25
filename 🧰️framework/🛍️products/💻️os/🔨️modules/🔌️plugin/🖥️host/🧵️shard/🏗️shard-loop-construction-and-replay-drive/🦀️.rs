//! 🏗️ Shard loop construction and replay drive: the shard loop constructor, the granted budgets, the authority queues and the replay seed opportunities, refusal, begin, close and drive. See [`super`].

use super::*;

impl ShardLoop {
    pub async fn new(runtime: Arc<GuestRuntimes>, transport: ShardTransports, identity_issuer:OriginalShardIdentityIssuer) -> Self {
        Self {
            identity_issuer,
            identities:HashMap::new(),
            runtime,
            transport,
            instances: HashMap::new(),
            allocations: HashMap::new(),
            next_registration: 1,
            running_jobs: BTreeSet::new(),
            job_turns: HashMap::new(),
            job_authorities: HashMap::new(),
            replay_seeds: std::iter::repeat_with(|| None).take(JOB_REPLAY_SEED_SLOT_CAPACITY).collect(),
            replay_seed_refusals: std::iter::repeat_with(|| None).take(JOB_REPLAY_REFUSAL_SLOT_CAPACITY).collect(),
            replay_seed_cursor: 0,
            next_job_operation: 1,
            job_placement: HashMap::new(),
            terminal_authorities: FixedOwnerRing::new(SHARD_DEFERRED_BYTES.saturating_mul(SHARD_DEFERRED_ITEMS)),
            granted_budgets: HashMap::new(),
            actor_lanes: HashMap::new(),
            pending_interactive: FixedOwnerRing::new(SHARD_DEFERRED_BYTES),
            pending_background: FixedOwnerRing::new(SHARD_DEFERRED_BYTES),
            rejected_frame: None,
            terminal_frames: FixedOwnerRing::new(SHARD_FRAME_MAX_BYTES.saturating_mul(SHARD_DEFERRED_ITEMS)),
            terminal_frame_overflow: FixedOwnerRing::new(usize::MAX),
            next_frame_epoch: 1,
            last_drive_consumed_epoch: None,
        }
    }

    /// 🎟️ Borrows only the actor's original supplied grant.
    pub fn granted_budget(&self, actor: u64) -> Option<&semio_framework_actor::Budget> {
        self.granted_budgets.get(&actor).map(|turn| &turn.budget)
    }

    /// 💼️ Funds the guest job crossings of `actor` from the Kernel-issued retained turn of its last grant.
    pub(super) fn job_funds(&self, actor: u64) -> Result<semio_framework_plugin_host_guest_runtime::JobFunds, TurnFault> {
        let turn = self.granted_budgets.get(&actor).ok_or_else(|| TurnFault::Host(PluginHostError::NativeIo(IssuedShardTurn::refusal("job crossing has no Kernel-issued actor batch"))))?;
        let loan = semio_framework_actor::OriginalActorTurnLoan { retained_policy: turn.retained_policy, work_fuel: turn.work_fuel, deadline_us: turn.deadline_us };
        semio_framework_plugin_host_guest_runtime::JobFunds::issued(*turn.original_input(), loan, turn.budget.fuel).map_err(|error| TurnFault::Host(PluginHostError::NativeIo(error)))
    }

    /// 🚦 terra-shard-lane piece 1: true for the two lanes that must jump a shard's queue ahead of
    /// any Background/Maintenance grant — Interactive (direct user input) and UserVisible
    /// (visible-but-not-actively-touched UI). `pump_primed`'s two queues partition on this.
    pub(super) fn is_high_priority_lane(lane: semio_framework_actor::Lane) -> bool {
        matches!(lane, semio_framework_actor::Lane::Interactive | semio_framework_actor::Lane::UserVisible)
    }

    /// 🚦 `actor`'s last-known scheduling lane — see [`Self::actor_lanes`]'s own doc for why this
    /// is recovered from envelopes already on the wire rather than a new `ShardFrame::Grant` field:
    /// a `Grant`-level `lane` field would have needed to break TWO `ShardFrame::Grant` construction
    /// sites outside this packet's `path_scope` (`💻️os/🖥️host/🎠️activation/🦀️.rs`'s
    /// `NativeKernelRuntime::tick_and_dispatch` and `📺️renderer/🧑‍🎨engine/…/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/
    /// 🎠️runtime.rs`'s equivalent dispatch loop — both live, both construct `ShardFrame::Grant`
    /// directly, neither is `🔌️plugin/🖥️host/**`), and the information a `Grant`-level field would
    /// have carried is ALREADY present per-envelope (`Envelope.lane`, set once per actor by the
    /// same DRR `Scheduler` that would have supplied a `Grant`-level lane). See
    /// `📓️terra-shard-lane-report.md`'s "wire change avoided" note — a `lease-request` is open
    /// against those two files in case a `Grant`-level field is still wanted for a future packet.
    pub fn actor_lane(&self, actor: u64) -> semio_framework_actor::Lane {
        self.actor_lanes.get(&actor).copied().unwrap_or(semio_framework_actor::Lane::Maintenance)
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn pop_next_authority(ring: &mut FixedOwnerRing<DeferredAuthority, SHARD_DEFERRED_ITEMS>, placements: &HashMap<(u64, u64), JobPlacement>) -> Option<(OwnerKey, DeferredAuthority)> {
        let actor = match ring.get(0) {
            Some(DeferredAuthority::JobStep { actor, .. }) => *actor,
            _ => return ring.pop_front(),
        };
        let mut selected = 0;
        for offset in 0..ring.len {
            let Some(DeferredAuthority::JobStep { actor: candidate, turn }) = ring.get(offset) else { break };
            if *candidate != actor {
                break;
            }
            if placements.get(&(actor, turn.job)) == Some(&JobPlacement::Exclusive) {
                selected = offset;
                break;
            }
        }
        ring.pop_at(selected)
    }

    fn consume_replay_opportunity(&mut self, actor: u64) -> bool {
        if !self.actor_generation_is_current(ActorId(actor)) {
            return false;
        }
        let Some(budget) = self.granted_budgets.get_mut(&actor) else { return false };
        if budget.budget.fuel == 0 || budget.budget.wall_ms == 0 {
            return false;
        }
        budget.budget.fuel -= 1;
        true
    }

    fn consume_replay_close_opportunity(&mut self, actor: u64) -> bool {
        let Some(budget) = self.granted_budgets.get_mut(&actor) else { return false };
        if budget.budget.fuel == 0 || budget.budget.wall_ms == 0 {
            return false;
        }
        budget.budget.fuel -= 1;
        true
    }

    pub async fn drive_replay_refusal(&mut self) -> Result<bool, PluginHostError> {
        let Some(index) = self.replay_seed_refusals.iter().position(Option::is_some) else {
            return Ok(false);
        };
        let actor = self.replay_seed_refusals[index].as_ref().expect("selected replay refusal").actor;
        if !self.consume_replay_close_opportunity(actor) {
            return Ok(false);
        }
        let refusal = self.replay_seed_refusals[index].as_mut().expect("selected replay refusal");
        match refusal.phase {
            ReplaySpawnRefusalPhase::RetireInput => {
                drop(refusal.input.take().expect("rejected input closes once"));
                refusal.phase = ReplaySpawnRefusalPhase::RetireKind;
            }
            ReplaySpawnRefusalPhase::RetireKind => {
                drop(refusal.kind.take().expect("rejected kind closes once"));
                refusal.phase = ReplaySpawnRefusalPhase::Publish;
            }
            ReplaySpawnRefusalPhase::Publish => {
                let actor = refusal.actor;
                let job = refusal.job;
                let reason = refusal.reason.to_vec();
                let publish = refusal.publish;
                refusal.phase = ReplaySpawnRefusalPhase::RetireShell;
                if publish {
                    let lane = self.actor_lane(actor);
                    defer_completion(&mut self.pending_interactive, &mut self.pending_background, &mut self.terminal_authorities, lane, self.allocations.get(&actor).copied(), self.granted_budgets.get(&actor).map(|turn| turn.budget), actor, Event::JobCompleted { job, result: RequestOutcome::Err(reason) })?;
                }
            }
            ReplaySpawnRefusalPhase::RetireShell => {
                let refusal = self.replay_seed_refusals[index].take().expect("terminal replay refusal");
                drop(refusal);
            }
        }
        Ok(true)
    }

    pub fn validate_replay_request(&self, actor: u64, turn: JobTurn, request: JobReplayRequest) -> Result<(), PluginHostError> {
        let Some(seed) = self.replay_seeds.iter().flatten().find(|seed| seed.actor == actor && seed.job == turn.job) else {
            return Err(PluginHostError::Plugin(format!("ShardLoop::replay: no retained seed for actor {actor}, job {}", turn.job)));
        };
        if !seed.replayable {
            return Err(PluginHostError::Plugin(format!("ShardLoop::replay: job {} of actor {actor} is a live-only framework reserved tool job with no restore checkpoint", turn.job)));
        }
        let retained = seed.seed.as_ref().ok_or_else(|| PluginHostError::Plugin(format!("ShardLoop::replay: seed for actor {actor}, job {} is closing", turn.job)))?;
        if seed.authority.operation.operation != turn.operation.operation
            || seed.authority.operation.base_revision != turn.operation.base_revision
            || seed.authority.operation.generation != turn.operation.generation
            || seed.authority.operation.seed != turn.operation.seed
            || turn.step_sequence != 0
            || retained.request != request
        {
            return Err(PluginHostError::Plugin(format!("ShardLoop::replay: route, seed, generation, or first ordinal mismatch for actor {actor}, job {}", turn.job)));
        }
        Ok(())
    }

    pub fn begin_replay_seed(&mut self, actor: u64, turn: JobTurn, request: JobReplayRequest, worker_count: u16, worker_slot: u16) -> Result<(), PluginHostError> {
        self.validate_replay_request(actor, turn, request)?;
        if worker_count == 0 || worker_slot == u16::MAX {
            return Err(PluginHostError::Plugin(format!("ShardLoop::replay: invalid worker identity for actor {actor}, job {}", turn.job)));
        }
        let seed = self.replay_seeds.iter_mut().flatten().find(|seed| seed.actor == actor && seed.job == turn.job).expect("validated replay seed remains mounted");
        if !matches!(seed.phase, ReplaySeedPhase::Retained) {
            return Err(PluginHostError::Plugin(format!("ShardLoop::replay: seed for actor {actor}, job {} is busy", turn.job)));
        }
        seed.authority = turn;
        seed.worker_count = worker_count;
        seed.worker_slot = worker_slot;
        seed.kind_cursor = 0;
        seed.input_cursor = 0;
        seed.checkpoint_cursor = 0;
        seed.materialize_page = 0;
        seed.phase = ReplaySeedPhase::MaterializeKind;
        Ok(())
    }

    pub fn begin_replay_seed_close(&mut self, index: usize, reason: ReplaySeedCloseReason) {
        Self::begin_replay_seed_close_owned(&mut self.replay_seeds, &mut self.running_jobs, &mut self.job_turns, &mut self.job_authorities, &mut self.job_placement, index, reason);
    }

    pub(super) fn begin_replay_seed_close_owned(
        replay_seeds: &mut [Option<MountedReplaySeed>],
        running_jobs: &mut BTreeSet<(u64, u64)>,
        job_turns: &mut HashMap<(u64, u64), JobTurn>,
        job_authorities: &mut HashMap<(u64, u64), JobAuthority>,
        job_placement: &mut HashMap<(u64, u64), JobPlacement>,
        index: usize,
        reason: ReplaySeedCloseReason,
    ) {
        let Some(seed) = replay_seeds[index].as_mut() else { return };
        if seed.close_reason.is_none() {
            seed.close_reason = Some(reason);
        }
        if !matches!(seed.phase, ReplaySeedPhase::RetireSeedShell | ReplaySeedPhase::RetireMountedShell) {
            seed.phase = ReplaySeedPhase::Closing;
        }
        let key = (seed.actor, seed.job);
        running_jobs.remove(&key);
        job_turns.remove(&key);
        job_authorities.remove(&key);
        job_placement.remove(&key);
    }

    fn fail_replay_seed(&mut self, index: usize, stage: &'static str, detail: String) -> PluginHostError {
        self.begin_replay_seed_close(index, ReplaySeedCloseReason::Fault { stage, detail: detail.clone() });
        PluginHostError::Plugin(detail)
    }

    pub(super) fn close_replay_job(&mut self, actor: u64, job: u64, reason: ReplaySeedCloseReason) -> bool {
        let Some(index) = self.replay_seeds.iter().position(|seed| seed.as_ref().is_some_and(|seed| seed.actor == actor && seed.job == job)) else { return false };
        self.begin_replay_seed_close(index, reason);
        true
    }

    pub(super) fn retire_actor_replay_owners(&mut self, actor: u64) {
        for index in 0..JOB_REPLAY_SEED_SLOT_CAPACITY {
            if self.replay_seeds[index].as_ref().is_some_and(|seed| seed.actor == actor) {
                self.begin_replay_seed_close(index, ReplaySeedCloseReason::ActorLost);
            }
        }
        for refusal in self.replay_seed_refusals.iter_mut().flatten().filter(|refusal| refusal.actor == actor) {
            refusal.publish = false;
        }
    }

    pub(super) fn close_replay_seed_one(replay_seeds: &mut [Option<MountedReplaySeed>], index: usize) -> Result<(), PluginHostError> {
        let phase = replay_seeds[index].as_ref().expect("closing replay seed").phase;
        match phase {
            ReplaySeedPhase::Closing => {
                let seed = replay_seeds[index].as_mut().expect("closing replay seed");
                if let Some(owner) = seed.replay_kind_owner.take() {
                    let capacity = seed.seed.as_ref().expect("closing fixed seed").kind_length;
                    drop(owner);
                    seed.release_abi(capacity)?;
                } else if let Some(owner) = seed.materialized_checkpoint.take() {
                    let capacity = seed.seed.as_ref().expect("closing fixed seed").checkpoint_length;
                    drop(owner);
                    seed.release_abi(capacity)?;
                } else if let Some(owner) = seed.materialized_input.take() {
                    let capacity = seed.seed.as_ref().expect("closing fixed seed").input_length;
                    drop(owner);
                    seed.release_abi(capacity)?;
                } else if let Some(owner) = seed.materialized_kind.take() {
                    let capacity = seed.seed.as_ref().expect("closing fixed seed").kind_length;
                    drop(owner);
                    seed.release_abi(capacity)?;
                } else if let Some(owner) = seed.checkpoint_owner.take() {
                    drop(owner);
                } else if let Some(owner) = seed.input_owner.take() {
                    drop(owner);
                } else if let Some(owner) = seed.kind_owner.take() {
                    drop(owner);
                } else if seed.seed.as_mut().expect("closing fixed seed").close_one() {
                    seed.phase = ReplaySeedPhase::RetireSeedShell;
                }
            }
            ReplaySeedPhase::RetireSeedShell => {
                let seed = replay_seeds[index].as_mut().expect("terminal replay seed");
                drop(seed.seed.take().expect("terminal fixed seed"));
                seed.phase = ReplaySeedPhase::RetireMountedShell;
            }
            ReplaySeedPhase::RetireMountedShell => {
                drop(replay_seeds[index].take().expect("terminal replay seed"));
            }
            ReplaySeedPhase::Retained => replay_seeds[index].as_mut().expect("stale retained replay seed").phase = ReplaySeedPhase::Closing,
            _ => unreachable!("replay close opportunity requires a closing phase"),
        }
        Ok(())
    }

    pub async fn drive_replay_seed(&mut self) -> Result<bool, PluginHostError> {
        let Some(index) = (0..JOB_REPLAY_SEED_SLOT_CAPACITY)
            .map(|offset| (self.replay_seed_cursor + offset) % JOB_REPLAY_SEED_SLOT_CAPACITY)
            .find(|index| self.replay_seeds[*index].as_ref().is_some_and(|seed| !matches!(seed.phase, ReplaySeedPhase::Retained) || !self.instances.contains_key(&seed.actor)))
        else {
            return Ok(false);
        };
        let actor = self.replay_seeds[index].as_ref().expect("selected replay seed").actor;
        let closing = self.replay_seeds[index].as_ref().is_some_and(|seed| matches!(seed.phase, ReplaySeedPhase::Closing | ReplaySeedPhase::RetireSeedShell | ReplaySeedPhase::RetireMountedShell) || !self.instances.contains_key(&seed.actor));
        if if closing { !self.consume_replay_close_opportunity(actor) } else { !self.consume_replay_opportunity(actor) } {
            return Ok(false);
        }
        self.replay_seed_cursor = (index + 1) % JOB_REPLAY_SEED_SLOT_CAPACITY;
        let phase = self.replay_seeds[index].as_ref().expect("selected replay seed").phase;
        if !self.instances.contains_key(&actor) && !matches!(phase, ReplaySeedPhase::Closing | ReplaySeedPhase::RetireSeedShell | ReplaySeedPhase::RetireMountedShell) {
            self.begin_replay_seed_close(index, ReplaySeedCloseReason::ActorLost);
            return Ok(true);
        }
        match phase {
            ReplaySeedPhase::CaptureKind => {
                let copied = {
                    let seed = self.replay_seeds[index].as_mut().expect("capture seed");
                    seed.seed.as_mut().expect("capture fixed seed").copy_kind_page(seed.kind_owner.as_ref().expect("capture kind").as_bytes(), &mut seed.kind_cursor)
                };
                match copied {
                    Ok(true) => self.replay_seeds[index].as_mut().expect("capture seed").phase = ReplaySeedPhase::CaptureInput,
                    Ok(false) => {}
                    Err(()) => return Err(self.fail_replay_seed(index, "capture-kind", format!("ShardLoop::replay: kind page admission refused for actor {actor}"))),
                }
            }
            ReplaySeedPhase::CaptureInput => {
                let copied = {
                    let seed = self.replay_seeds[index].as_mut().expect("capture seed");
                    seed.seed.as_mut().expect("capture fixed seed").copy_input_page(seed.input_owner.as_ref().expect("capture input"), &mut seed.input_cursor)
                };
                match copied {
                    Ok(true) => {
                        let seed = self.replay_seeds[index].as_mut().expect("capture seed");
                        seed.phase = if seed.replayable { ReplaySeedPhase::Checkpoint } else { ReplaySeedPhase::Start };
                    }
                    Ok(false) => {}
                    Err(()) => return Err(self.fail_replay_seed(index, "capture-input", format!("ShardLoop::replay: input page admission refused for actor {actor}"))),
                }
            }
            ReplaySeedPhase::Checkpoint => {
                let checkpoint = match self.instances.get_mut(&actor) {
                    Some(instance) => match self.identities.get_mut(self.allocations.get(&actor).expect("registered actor allocation")).expect("registered actor owns original identity").checkpoint(&self.runtime,instance).await {
                        Ok(checkpoint) => checkpoint,
                        Err(fault) => return Err(self.fail_replay_seed(index, "checkpoint", format!("ShardLoop::replay: checkpoint failed for actor {actor}: {fault}"))),
                    },
                    None => {
                        self.begin_replay_seed_close(index, ReplaySeedCloseReason::ActorLost);
                        return Ok(true);
                    }
                };
                let seed = self.replay_seeds[index].as_mut().expect("checkpoint seed");
                seed.checkpoint_owner = Some(checkpoint);
                seed.phase = ReplaySeedPhase::CaptureCheckpoint;
            }
            ReplaySeedPhase::CaptureCheckpoint => {
                let copied = {
                    let seed = self.replay_seeds[index].as_mut().expect("checkpoint seed");
                    seed.seed.as_mut().expect("capture fixed seed").copy_checkpoint_page(seed.checkpoint_owner.as_ref().expect("checkpoint owner"), &mut seed.checkpoint_cursor)
                };
                match copied {
                    Ok(true) => self.replay_seeds[index].as_mut().expect("checkpoint seed").phase = ReplaySeedPhase::RetireCheckpointOwner,
                    Ok(false) => {}
                    Err(()) => return Err(self.fail_replay_seed(index, "capture-checkpoint", format!("ShardLoop::replay: checkpoint page admission refused for actor {actor}"))),
                }
            }
            ReplaySeedPhase::RetireCheckpointOwner => {
                let seed = self.replay_seeds[index].as_mut().expect("checkpoint seed");
                drop(seed.checkpoint_owner.take().expect("captured checkpoint owner retires once"));
                seed.phase = ReplaySeedPhase::Start;
            }
            ReplaySeedPhase::Start => {
                let (job, kind, input) = {
                    let seed = self.replay_seeds[index].as_mut().expect("start seed");
                    (seed.job, seed.kind_owner.take().expect("original kind checked out once"), seed.input_owner.take().expect("original input transfers once"))
                };
                let funds = self.job_funds(actor);
                let result = match self.instances.get_mut(&actor) {
                    Some(instance) => match funds { Ok(mut funds) => self.runtime.start_job_funded(instance, &mut funds, job, kind.clone(), input).await, Err(fault) => Err(fault) },
                    None => {
                        let seed = self.replay_seeds[index].as_mut().expect("lost start seed");
                        seed.kind_owner = Some(kind);
                        seed.input_owner = Some(input);
                        self.begin_replay_seed_close(index, ReplaySeedCloseReason::ActorLost);
                        return Ok(true);
                    }
                };
                let seed = self.replay_seeds[index].as_mut().expect("started seed");
                seed.kind_owner = Some(kind);
                if let Err(fault) = result {
                    return Err(self.fail_replay_seed(index, "live-start", format!("ShardLoop::replay: live start failed for actor {actor}, job {job}: {}", turn_fault_message(&fault))));
                }
                seed.phase = ReplaySeedPhase::RetireKindOwner;
            }
            ReplaySeedPhase::RetireKindOwner => {
                let seed = self.replay_seeds[index].as_mut().expect("started seed");
                drop(seed.kind_owner.take().expect("fixed kind pages supersede original owner"));
                seed.phase = ReplaySeedPhase::ActivateAuthority;
            }
            ReplaySeedPhase::ActivateAuthority => {
                let seed = self.replay_seeds[index].as_mut().expect("activate seed");
                self.job_authorities.insert((actor, seed.job), JobAuthority { turn: seed.authority, request: seed.seed.as_ref().expect("retained seed").request });
                seed.phase = ReplaySeedPhase::ActivateTurn;
            }
            ReplaySeedPhase::ActivateTurn => {
                let seed = self.replay_seeds[index].as_mut().expect("activate seed");
                self.job_turns.insert((actor, seed.job), seed.authority);
                seed.phase = ReplaySeedPhase::ActivateRunning;
            }
            ReplaySeedPhase::ActivateRunning => {
                let seed = self.replay_seeds[index].as_mut().expect("activate seed");
                self.running_jobs.insert((actor, seed.job));
                seed.phase = ReplaySeedPhase::ActivatePlacement;
            }
            ReplaySeedPhase::ActivatePlacement => {
                let seed = self.replay_seeds[index].as_mut().expect("activate seed");
                self.job_placement.insert((actor, seed.job), seed.placement);
                seed.phase = ReplaySeedPhase::ActivateReady;
            }
            ReplaySeedPhase::ActivateReady => {
                let seed = self.replay_seeds[index].as_mut().expect("activate seed");
                let replay = seed.worker_count != 0;
                let authority = seed.authority;
                seed.phase = ReplaySeedPhase::Retained;
                if replay {
                    if let Err(error) = self.send_outcome(&ShardOutcome::Resumed { actor, operation: authority.operation }).await {
                        return Err(self.fail_replay_seed(index, "resume-publication", format!("ShardLoop::replay: resume publication failed for actor {actor}: {error}")));
                    }
                }
            }
            ReplaySeedPhase::MaterializeKind => {
                let seed = self.replay_seeds[index].as_mut().expect("materialize seed");
                let fixed = seed.seed.as_ref().expect("materialize fixed seed");
                match seed.materialized_kind.as_mut() {
                    None => {
                        let buffer = match try_replay_abi_buffer(fixed.kind_length) {
                            Ok(buffer) => buffer,
                            Err(()) => return Err(self.fail_replay_seed(index, "materialize-kind", "ShardLoop::replay: kind ABI admission refused".to_string())),
                        };
                        seed.abi_reserved += fixed.kind_length;
                        seed.materialized_kind = Some(buffer);
                    }
                    Some(buffer) if seed.materialize_page < fixed.kind_pages => {
                        let page = fixed.kind[seed.materialize_page].as_ref().expect("fixed kind page");
                        buffer.extend_from_slice(page.bytes());
                        seed.materialize_page += 1;
                    }
                    Some(_) => {
                        seed.materialize_page = 0;
                        seed.phase = ReplaySeedPhase::MaterializeInput;
                    }
                }
            }
            ReplaySeedPhase::MaterializeInput => {
                let seed = self.replay_seeds[index].as_mut().expect("materialize seed");
                let fixed = seed.seed.as_ref().expect("materialize fixed seed");
                match seed.materialized_input.as_mut() {
                    None => {
                        let buffer = match try_replay_abi_buffer(fixed.input_length) {
                            Ok(buffer) => buffer,
                            Err(()) => return Err(self.fail_replay_seed(index, "materialize-input", "ShardLoop::replay: input ABI admission refused".to_string())),
                        };
                        seed.abi_reserved += fixed.input_length;
                        seed.materialized_input = Some(buffer);
                    }
                    Some(buffer) if seed.materialize_page < fixed.input_pages => {
                        let page = fixed.input[seed.materialize_page].as_ref().expect("fixed input page");
                        buffer.extend_from_slice(page.bytes());
                        seed.materialize_page += 1;
                    }
                    Some(_) => {
                        seed.materialize_page = 0;
                        seed.phase = ReplaySeedPhase::MaterializeCheckpoint;
                    }
                }
            }
            ReplaySeedPhase::MaterializeCheckpoint => {
                let seed = self.replay_seeds[index].as_mut().expect("materialize seed");
                let fixed = seed.seed.as_ref().expect("materialize fixed seed");
                match seed.materialized_checkpoint.as_mut() {
                    None => {
                        let buffer = match try_replay_abi_buffer(fixed.checkpoint_length) {
                            Ok(buffer) => buffer,
                            Err(()) => return Err(self.fail_replay_seed(index, "materialize-checkpoint", "ShardLoop::replay: checkpoint ABI admission refused".to_string())),
                        };
                        seed.abi_reserved += fixed.checkpoint_length;
                        seed.materialized_checkpoint = Some(buffer);
                    }
                    Some(buffer) if seed.materialize_page < fixed.checkpoint_pages => {
                        let page = fixed.checkpoint[seed.materialize_page].as_ref().expect("fixed checkpoint page");
                        buffer.extend_from_slice(page.bytes());
                        seed.materialize_page += 1;
                    }
                    Some(_) => {
                        seed.materialize_page = 0;
                        seed.phase = ReplaySeedPhase::Restore;
                    }
                }
            }
            ReplaySeedPhase::Restore => {
                let checkpoint = self.replay_seeds[index].as_ref().expect("restore seed").materialized_checkpoint.as_ref().expect("materialized checkpoint");
                match self.instances.get_mut(&actor) {
                    Some(instance) => {
                        if let Err(fault) = self.identities.get_mut(self.allocations.get(&actor).expect("registered actor allocation")).expect("registered actor owns original identity").restore(&self.runtime,instance,checkpoint).await {
                            return Err(self.fail_replay_seed(index, "restore", format!("ShardLoop::replay: restore failed for actor {actor}: {fault}")));
                        }
                    }
                    None => {
                        self.begin_replay_seed_close(index, ReplaySeedCloseReason::ActorLost);
                        return Ok(true);
                    }
                }
                self.replay_seeds[index].as_mut().expect("restored seed").phase = ReplaySeedPhase::RetireMaterializedCheckpoint;
            }
            ReplaySeedPhase::RetireMaterializedCheckpoint => {
                let seed = self.replay_seeds[index].as_mut().expect("restored seed");
                drop(seed.materialized_checkpoint.take().expect("restored checkpoint retires once"));
                let bytes = seed.seed.as_ref().expect("retained seed").checkpoint_length;
                seed.release_abi(bytes)?;
                seed.phase = ReplaySeedPhase::PrepareReplayKind;
            }
            ReplaySeedPhase::PrepareReplayKind => {
                let (bytes, capacity) = {
                    let seed = self.replay_seeds[index].as_mut().expect("prepare replay kind");
                    (seed.materialized_kind.take().expect("materialized replay kind"), seed.seed.as_ref().expect("retained seed").kind_length)
                };
                match String::from_utf8(bytes) {
                    Ok(kind) => {
                        let seed = self.replay_seeds[index].as_mut().expect("prepared replay kind");
                        seed.replay_kind_owner = Some(kind);
                        seed.phase = ReplaySeedPhase::Restart;
                    }
                    Err(_) => {
                        let released = self.replay_seeds[index].as_mut().expect("invalid replay kind").release_abi(capacity);
                        if let Err(error) = released {
                            return Err(self.fail_replay_seed(index, "prepare-kind-accounting", error.to_string()));
                        }
                        return Err(self.fail_replay_seed(index, "prepare-kind", "ShardLoop::replay: retained kind is not UTF-8".to_string()));
                    }
                }
            }
            ReplaySeedPhase::Restart => {
                let (job, kind, input, input_bytes) = {
                    let seed = self.replay_seeds[index].as_mut().expect("restart seed");
                    let kind = seed.replay_kind_owner.take().expect("prepared replay kind checks out once");
                    let input = seed.materialized_input.take().expect("materialized replay input");
                    let input_bytes = seed.seed.as_ref().expect("retained seed").input_length;
                    (seed.job, kind, input, input_bytes)
                };
                let funds = self.job_funds(actor);
                let result = match self.instances.get_mut(&actor) {
                    Some(instance) => match funds { Ok(mut funds) => self.runtime.start_job_funded(instance, &mut funds, job, kind.clone(), input).await, Err(fault) => Err(fault) },
                    None => {
                        let seed = self.replay_seeds[index].as_mut().expect("lost restart seed");
                        seed.replay_kind_owner = Some(kind);
                        seed.materialized_input = Some(input);
                        self.begin_replay_seed_close(index, ReplaySeedCloseReason::ActorLost);
                        return Ok(true);
                    }
                };
                let released = {
                    let seed = self.replay_seeds[index].as_mut().expect("restarted seed");
                    seed.replay_kind_owner = Some(kind);
                    seed.release_abi(input_bytes)
                };
                if let Err(fault) = result {
                    return Err(self.fail_replay_seed(index, "restart", format!("ShardLoop::replay: restart failed for actor {actor}, job {job}: {}", turn_fault_message(&fault))));
                }
                if let Err(error) = released {
                    return Err(self.fail_replay_seed(index, "restart-accounting", error.to_string()));
                }
                self.replay_seeds[index].as_mut().expect("restarted seed").phase = ReplaySeedPhase::RetireReplayKind;
            }
            ReplaySeedPhase::RetireReplayKind => {
                let seed = self.replay_seeds[index].as_mut().expect("restarted seed");
                drop(seed.replay_kind_owner.take().expect("fixed kind pages supersede replay ABI owner"));
                let bytes = seed.seed.as_ref().expect("retained seed").kind_length;
                seed.release_abi(bytes)?;
                seed.phase = ReplaySeedPhase::ActivateAuthority;
            }
            ReplaySeedPhase::Closing | ReplaySeedPhase::RetireSeedShell | ReplaySeedPhase::RetireMountedShell | ReplaySeedPhase::Retained => Self::close_replay_seed_one(&mut self.replay_seeds, index)?,
        }
        Ok(true)
    }

}
