//! 🏊️ Kernel pool: the kernel pool state implementation, request queue, pool futures and the pool runner.
use super::*;

//#region 🎠️KernelRuntime
//#region 🔖️KernelPoolState
impl KernelPoolState {
    /// ⏱️ P3a (INTERACTIVE-JOB-RUNTIME-REFACTOR, ui-thread-isolation): every method in this
    /// state machine is genuinely asynchronous and the whole request loop is mounted once on the
    /// injected renderer worker pool. No executor bridge or dedicated kernel thread remains in
    /// product logic.
    ///
    /// 🧵️ P1e: the injected process-wide pool (`crate::renderer_worker_pool`), never a pool this
    /// type mints for itself — see `ParallelRuntime::new`'s own doc.
    pub async fn new(request_queue: Arc<KernelRequestQueue>, identity_policy:semio_framework_plugin_host::shard::ShardIdentityPolicy, original_clock:fn()->Option<u64>) -> Self {
        let guest_runtime: Arc<GuestRuntimes> = Arc::new(GuestRuntimes::Owned(OwnedRuntime::new(original_clock)));
        let pool = Arc::new(crate::renderer_worker_pool());
        let worker_count = u16::try_from(pool.worker_count()).unwrap_or(u16::MAX);
        let runtime = crate::parallel_runtime::ParallelRuntime::new(pool, guest_runtime.clone(), native_shard_count(), 2, 64,|_|{let original_queue=Arc::clone(&request_queue);semio_framework_plugin_host::shard::native_identity_issuer(identity_policy,move |_|{let queue=Arc::clone(&original_queue);Box::new(move |_|!queue.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner).closing)})}).await;
        Self {
            request_queue,
            guest_runtime,
            runtime,
            worker_count,
            now_ms: 0,
            plugin_ordinals: HashMap::new(),
            instances: HashMap::new(),
            next_instance_id: 1,
            retained: RetainedSurfaceRegistry::new(),
            pending_rejections: PendingSurfaceRejectionRegistry::new(),
            retained_command_closes: semio_framework::kernel::CommandDriverRegistry::new(),
            queued_command_closes: semio_framework::kernel::CommandDriverRegistry::new(),
            command_document_closes: CommandDocumentRetirementRegistry::new(),
            rejected_command_builds: semio_framework::kernel::RejectedCommandBuildRegistry::new(),
            rejected_events: None,
            job_progress: JobProgressOverlayStore::new(),
            replay_routes: [None; JOB_PROGRESS_ACTIVE_CAPACITY],
            job_replays: std::array::from_fn(|_| None),
            reserved_jobs: [None; JOB_PROGRESS_ACTIVE_CAPACITY],
            product_replay_claims: std::array::from_fn(|_| None),
            product_replay_authorities: std::array::from_fn(|_| None),
            rejected_job_progress: std::array::from_fn(|_| None),
            pending_job_progress_presentations: std::array::from_fn(|_| None),
            closing_apps: std::array::from_fn(|_| None),
            fault_closing_actors: [None; JOB_PROGRESS_ACTIVE_CAPACITY],
            realm_progress_close_started: false,
            semantic_close_document_lane: false,
            serving: Vec::with_capacity(2),
            interleaving: false,
            compiled_components: Vec::with_capacity(COMPILED_COMPONENT_CAPACITY),
        }
    }

    fn mount_product_replay(&mut self, mut owner: MountedProductReplayRequest) -> Result<(), MountedProductReplayRequest> {
        let Some(&actor) = self.instances.get(&owner.instance) else { return Err(owner) };
        if owner.job == 0
            || owner.job_kind().is_empty()
            || owner.raw().job != owner.job
            || owner.raw().kind().as_bytes() != owner.job_kind()
            || owner.raw().placement != owner.placement
            || JobReplayRequest::from_spawn(owner.raw().kind(), owner.raw().input()) != owner.request
            || self.product_replay_claims.iter().flatten().any(|claim| claim.actor == actor && claim.job() == owner.job)
            || self.product_replay_authorities.iter().flatten().any(|authority| authority.actor == actor && authority.job() == owner.job)
        {
            return Err(owner);
        }
        let kind = std::str::from_utf8(owner.job_kind()).ok();
        let Some(kind) = kind else { return Err(owner) };
        let kind_identity = JobReplayRequest::from_spawn(kind, &[]);
        if kind_identity.controller != owner.request.controller || kind_identity.tool != owner.request.tool || kind_identity.schema != owner.request.schema || kind_identity.version != owner.request.version {
            return Err(owner);
        }
        let Some(index) = (0..JOB_PROGRESS_ACTIVE_CAPACITY).find(|index| self.product_replay_claims[*index].is_none() && self.product_replay_authorities[*index].is_none()) else { return Err(owner) };
        owner.raw_mut().acknowledge_mount();
        self.product_replay_claims[index] = Some(MountedProductReplayClaim { request: Some(owner), actor, record_cursor: 0, checkpoint: None });
        Ok(())
    }

    fn product_replay_pending_instance(&self) -> Option<u32> {
        self.product_replay_authorities.iter().flatten().next().map(MountedProductReplayAuthority::instance).or_else(|| self.product_replay_claims.iter().flatten().next().map(MountedProductReplayClaim::instance))
    }

    fn abort_product_replay_one(&mut self, instance: u32) {
        if let Some(index) = self.product_replay_authorities.iter().position(|authority| authority.as_ref().is_some_and(|authority| authority.instance() == instance)) {
            self.product_replay_authorities[index].take().expect("faulted product replay authority remains exact").retire();
        } else if let Some(index) = self.product_replay_claims.iter().position(|claim| claim.as_ref().is_some_and(|claim| claim.instance() == instance)) {
            self.product_replay_claims[index].take().expect("faulted product replay claim remains exact").retire();
        }
    }

    async fn advance_product_replay(&mut self, instance: u32) -> Result<ExchangeOutcome, ProgramFault> {
        let idle = || ExchangeOutcome { frames: Vec::new(), surfaces: UiFixedList::default(), effects: Vec::new(), command_ingress: semio_framework::kernel::CommandIngressStatus::Idle, typed_results: Vec::new() };
        if let Some(index) = self.product_replay_authorities.iter().position(|authority| authority.as_ref().is_some_and(|authority| authority.instance() == instance)) {
            let (actor, job, profile_cursor, profile_started) = {
                let authority = self.product_replay_authorities[index].as_ref().expect("selected product replay authority");
                (authority.actor, authority.job(), authority.profile_cursor, authority.profile_started)
            };
            if profile_cursor == PRODUCT_REPLAY_PROFILE_COUNT {
                self.product_replay_authorities[index].as_mut().expect("completed product replay authority remains exact").acknowledge_raw_retirement_boundary().map_err(str::to_string)?;
                self.product_replay_authorities[index].take().expect("completed product replay authority remains exact").retire();
                return Ok(idle());
            }
            let entry_index = self.job_replays.iter().position(|entry| entry.as_ref().is_some_and(|entry| entry.actor == actor && entry.authority.job == job)).ok_or_else(|| "kernel: product replay lost its mounted log".to_string())?;
            let process_slot = self.runtime.kernel().actor_record(actor).await.map_or(u16::MAX, |record| record.shard.0);
            let (turn, request, placement, route, checkpoint, terminal, expected_begin, expected_restore_start_ordinal) = {
                let entry = self.job_replays[entry_index].as_ref().expect("qualified product replay log");
                let terminal =
                    entry.log.record_header(entry.log.sealed_records().checked_sub(1).ok_or_else(|| "kernel: product replay has no terminal record".to_string())?).ok_or_else(|| "kernel: product replay terminal record is missing".to_string())?;
                let checkpoint = entry.log.last_checkpoint_header().map(|header| MountedProductReplayCheckpoint {
                    ordinal: header.ordinal,
                    digest: header.payload_digest,
                    pages: u8::try_from(header.payload_bytes.saturating_add(semio_framework_actor::JOB_REPLAY_PAGE_BYTES - 1) / semio_framework_actor::JOB_REPLAY_PAGE_BYTES).unwrap_or(u8::MAX),
                    applied_progress: header.applied_progress,
                });
                let (expected_begin, expected_restore_start_ordinal) = if profile_started {
                    if entry.replay_started {
                        (false, entry.accepted_replay_sequence)
                    } else {
                        (true, entry.replay_submit_sequence)
                    }
                } else {
                    (true, entry.accepted_replay_sequence)
                };
                (entry.authority, entry.request, entry.placement, entry.log.route(), checkpoint, terminal, expected_begin, expected_restore_start_ordinal)
            };
            let logical_worker_count = mounted_product_replay_profile(self.worker_count, profile_cursor).ok_or_else(|| "kernel: product replay profile is invalid".to_string())?;
            let logical_worker_slot = mounted_product_replay_worker_slot(process_slot, logical_worker_count).ok_or_else(|| "kernel: product replay slot is invalid".to_string())?;
            let expected = MountedProductReplayExpected {
                instance,
                actor,
                turn,
                request,
                placement,
                route,
                checkpoint,
                terminal,
                process_worker_count: self.worker_count,
                process_worker_slot: process_slot,
                logical_worker_count,
                logical_worker_slot,
                begin: expected_begin,
                restore_start_ordinal: expected_restore_start_ordinal,
            };
            let authority = self.product_replay_authorities[index].take().expect("selected product replay authority");
            match authority.validate(expected) {
                Ok(authority) => self.product_replay_authorities[index] = Some(authority),
                Err(authority) => {
                    self.product_replay_authorities[index] = Some(authority);
                    return Err(("kernel: mounted product replay authority became stale; exact owner remains mounted".to_string()).into());
                }
            }
            if profile_started {
                let (requested, complete, accepted_sequence) = {
                    let entry = self.job_replays[entry_index].as_ref().expect("qualified product replay log");
                    (entry.replay_requested, entry.log.replay_is_complete(), entry.accepted_replay_sequence)
                };
                if !requested && complete {
                    let authority = self.product_replay_authorities[index].as_mut().expect("completed product replay authority");
                    authority.profile_cursor += 1;
                    authority.profile_started = false;
                    authority.restore_start_ordinal = accepted_sequence;
                    if let Some(worker_count) = mounted_product_replay_profile(self.worker_count, authority.profile_cursor) {
                        authority.worker_count = worker_count;
                        authority.worker_slot = mounted_product_replay_worker_slot(process_slot, worker_count).ok_or_else(|| "kernel: next product replay slot is invalid".to_string())?;
                        authority.begin = true;
                    }
                    return Ok(idle());
                }
                let outcome = self.run_turn(actor, instance, Vec::new()).await?;
                let (replay_started, restore_start_ordinal) = {
                    let entry = self.job_replays[entry_index].as_ref().expect("advanced product replay remains mounted");
                    (entry.replay_started, if entry.replay_started { entry.accepted_replay_sequence } else { entry.replay_submit_sequence })
                };
                let authority = self.product_replay_authorities[index].as_mut().expect("advanced product replay authority");
                authority.begin = !replay_started;
                authority.restore_start_ordinal = restore_start_ordinal;
                return Ok(outcome);
            }
            let (worker_count, worker_slot, begin, restore_start_ordinal) = {
                let authority = self.product_replay_authorities[index].as_ref().expect("starting product replay authority");
                (authority.worker_count, authority.worker_slot, authority.begin, authority.restore_start_ordinal)
            };
            if !begin {
                return Err(("kernel: product replay start lost its qualified begin authority".to_string()).into());
            }
            self.request_job_replay(actor, job, worker_count, worker_slot, restore_start_ordinal).await?;
            let authority = self.product_replay_authorities[index].as_mut().expect("starting product replay authority");
            authority.profile_started = true;
            authority.restore_start_ordinal = None;
            let outcome = self.run_turn(actor, instance, Vec::new()).await?;
            let (replay_started, restore_start_ordinal) = {
                let entry = self.job_replays[entry_index].as_ref().expect("started product replay remains mounted");
                (entry.replay_started, if entry.replay_started { entry.accepted_replay_sequence } else { entry.replay_submit_sequence })
            };
            let authority = self.product_replay_authorities[index].as_mut().expect("started product replay authority");
            authority.begin = !replay_started;
            authority.restore_start_ordinal = restore_start_ordinal;
            return Ok(outcome);
        }

        let Some(index) = self.product_replay_claims.iter().position(|claim| claim.as_ref().is_some_and(|claim| claim.instance() == instance)) else { return Ok(idle()) };
        let (actor, job, request) = {
            let claim = self.product_replay_claims[index].as_ref().expect("selected product replay claim");
            (claim.actor, claim.job(), claim.request().request)
        };
        let Some(entry_index) = self.job_replays.iter().position(|entry| entry.as_ref().is_some_and(|entry| entry.actor == actor && entry.authority.job == job)) else {
            return self.run_turn(actor, instance, Vec::new()).await;
        };
        let entry = self.job_replays[entry_index].as_ref().expect("selected mounted replay log");
        if entry.request != request {
            return Err(("kernel: product replay request digest does not match the mounted session".to_string()).into());
        }
        if entry.log.has_pending_work() || entry.captured.is_some() || entry.policy.is_some() {
            return Ok(idle());
        }
        if !entry.terminal_seen {
            return self.run_turn(actor, instance, Vec::new()).await;
        }
        let record_cursor = self.product_replay_claims[index].as_ref().expect("selected product replay claim").record_cursor;
        if record_cursor < entry.log.sealed_records() {
            let header = entry.log.record_header(record_cursor).ok_or_else(|| "kernel: product replay record cursor is stale".to_string())?;
            let claim = self.product_replay_claims[index].as_mut().expect("selected product replay claim");
            if header.kind == JobReplayPublicationKind::Checkpoint {
                let pages = header.payload_bytes.saturating_add(semio_framework_actor::JOB_REPLAY_PAGE_BYTES - 1) / semio_framework_actor::JOB_REPLAY_PAGE_BYTES;
                claim.checkpoint = Some(MountedProductReplayCheckpoint { ordinal: header.ordinal, digest: header.payload_digest, pages: u8::try_from(pages).unwrap_or(u8::MAX), applied_progress: header.applied_progress });
            }
            claim.record_cursor += 1;
            return Ok(idle());
        }
        let terminal = entry.log.record_header(entry.log.sealed_records().checked_sub(1).ok_or_else(|| "kernel: product replay log is empty".to_string())?).ok_or_else(|| "kernel: product replay terminal record is missing".to_string())?;
        if !matches!(terminal.kind, JobReplayPublicationKind::Commit | JobReplayPublicationKind::Cancelled | JobReplayPublicationKind::Fault) {
            return Err(("kernel: product replay log has no typed terminal publication".to_string()).into());
        }
        if terminal.prefix_digest != entry.log.prefix_digest() {
            return Err(("kernel: product replay terminal prefix does not cover the retained log".to_string()).into());
        }
        let expected_checkpoint = entry.log.last_checkpoint_header().map(|header| MountedProductReplayCheckpoint {
            ordinal: header.ordinal,
            digest: header.payload_digest,
            pages: u8::try_from(header.payload_bytes.saturating_add(semio_framework_actor::JOB_REPLAY_PAGE_BYTES - 1) / semio_framework_actor::JOB_REPLAY_PAGE_BYTES).unwrap_or(u8::MAX),
            applied_progress: header.applied_progress,
        });
        if self.product_replay_claims[index].as_ref().expect("qualified product replay claim").checkpoint != expected_checkpoint {
            return Err(("kernel: product replay checkpoint witness does not match the retained log".to_string()).into());
        }
        let route = entry.log.route();
        let operation = entry.authority.operation;
        let process_slot = self.runtime.kernel().actor_record(actor).await.map_or(u16::MAX, |record| record.shard.0);
        if terminal.worker_count != self.worker_count || terminal.worker_slot != process_slot {
            return Err(("kernel: product replay physical process identity changed before qualification".to_string()).into());
        }
        let worker_count = mounted_product_replay_profile(self.worker_count, 0).ok_or_else(|| "kernel: first product replay profile is invalid".to_string())?;
        let worker_slot = mounted_product_replay_worker_slot(process_slot, worker_count).ok_or_else(|| "kernel: first product replay slot is invalid".to_string())?;
        let mut claim = self.product_replay_claims[index].take().expect("qualified product replay claim");
        let mut request_owner = claim.request.take().expect("qualified product replay request transfers once");
        request_owner.raw_mut().acknowledge_qualification(claim.checkpoint);
        self.product_replay_authorities[index] = Some(MountedProductReplayAuthority {
            request_owner: Some(request_owner),
            actor,
            route,
            operation,
            generation: operation.generation,
            seed: operation.seed,
            checkpoint: claim.checkpoint,
            terminal,
            worker_count,
            worker_slot,
            begin: true,
            restore_start_ordinal: None,
            profile_cursor: 0,
            profile_started: false,
        });
        Ok(idle())
    }

    async fn request_job_replay(&mut self, actor: ActorId, job: u64, worker_count: u16, worker_slot: u16, restore_start_ordinal: Option<u64>) -> Result<(), String> {
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

    fn begin_job_replay_capture(&mut self, actor: ActorId, authority: JobTurn, request: JobReplayRequest, placement: JobPlacement, worker_count: u16, worker_slot: u16, publication: JobPublication) {
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
        let mut context = semio_framework_job::StepContext::new(
            semio_framework_job::OperationId(publication.turn.operation.operation),
            semio_framework_job::Generation(publication.turn.operation.generation),
            now.and_then(|now| semio_framework_job::StepBudget::from_duration(1, now, semio_framework_job::INTERACTIVE_LANE_WALL_US)).unwrap_or(semio_framework_job::StepBudget::new(0, 0)),
            semio_framework_job::root_cancel_token(),
            semio_framework_job::default_now_us,
            &mut preview_sequence,
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
            let mut context = semio_framework_job::StepContext::new(
                semio_framework_job::OperationId(operation.operation),
                semio_framework_job::Generation(operation.generation),
                now.and_then(|now| semio_framework_job::StepBudget::from_duration(1, now, semio_framework_job::MAINTENANCE_LANE_WALL_US)).unwrap_or(semio_framework_job::StepBudget::new(0, 0)),
                semio_framework_job::root_cancel_token(),
                semio_framework_job::default_now_us,
                &mut preview_sequence,
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
        let mut context = semio_framework_job::StepContext::new(
            semio_framework_job::OperationId(operation.operation),
            semio_framework_job::Generation(operation.generation),
            now.and_then(|now| semio_framework_job::StepBudget::from_duration(1, now, semio_framework_job::INTERACTIVE_LANE_WALL_US)).unwrap_or(semio_framework_job::StepBudget::new(0, 0)),
            semio_framework_job::root_cancel_token(),
            semio_framework_job::default_now_us,
            &mut preview_sequence,
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

    fn command_maintenance_step(&mut self) -> bool {
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
            let mut context = semio_framework_job::StepContext::new(
                semio_framework_job::OperationId(0),
                semio_framework_job::Generation(0),
                now.and_then(|now| semio_framework_job::StepBudget::from_duration(1, now, semio_framework_job::MAINTENANCE_LANE_WALL_US)).unwrap_or(semio_framework_job::StepBudget::new(0, 0)),
                semio_framework_job::root_cancel_token(),
                semio_framework_job::default_now_us,
                &mut preview_sequence,
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

    fn command_maintenance_pending(&self) -> bool {
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

    fn begin_fault_close(&mut self, actor: ActorId) {
        if self.fault_closing_actors.iter().flatten().any(|closing| *closing == actor) {
            return;
        }
        if let Some(slot) = self.fault_closing_actors.iter_mut().find(|slot| slot.is_none()) {
            *slot = Some(actor);
        } else {
            self.job_progress.begin_close_all();
        }
    }

    fn acknowledge_job_progress(&mut self, token: JobProgressPresentationToken) {
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

    fn begin_job_progress(&mut self, actor: ActorId, turn: &JobTurn) -> Result<bool, String> {
        let live = JobProgressLiveAuthority::new(turn.operation.operation, turn.operation.base_revision, turn.operation.generation);
        if self.job_progress.live_authority(actor, turn.job) == Some(live) {
            return Ok(false);
        }
        self.job_progress.begin_operation(actor, turn.job, live).map_err(|fault| fault.to_string())?;
        Ok(true)
    }

    fn retain_job_progress_rejection(&mut self, rejected: JobProgressRejected) {
        let rejected = match self.job_progress.retain_rejected(rejected) {
            Ok(()) => return,
            Err(rejected) => rejected,
        };
        let slot = self.rejected_job_progress.iter_mut().find(|slot| slot.is_none()).expect("one kernel decision cannot exceed the fixed 64-actor rejected-publication handback registry");
        *slot = Some(rejected);
    }

    fn publish_captured_job_progress(&mut self, actor: ActorId, authority: JobTurn, publication: JobPublication) -> JobReplayPublicationPolicy {
        let live = JobProgressLiveAuthority::new(authority.operation.operation, authority.operation.base_revision, authority.operation.generation);
        let stable_identity_matches = authority.step_sequence == 0
            && authority.operation.preview_sequence == 0
            && authority.job == publication.turn.job
            && authority.operation.operation == publication.turn.operation.operation
            && authority.operation.base_revision == publication.turn.operation.base_revision
            && authority.operation.generation == publication.turn.operation.generation;
        if !stable_identity_matches {
            self.retain_job_progress_rejection(JobProgressRejected::new(semio_framework_actor::JobProgressFault::Stale, publication));
            return JobReplayPublicationPolicy::Rejected;
        }
        match self.job_progress.live_authority(actor, authority.job) {
            Some(expected) if expected == live => {}
            None => {
                if let Err(fault) = self.job_progress.begin_operation(actor, authority.job, live) {
                    self.retain_job_progress_rejection(JobProgressRejected::new(fault, publication));
                    return JobReplayPublicationPolicy::Rejected;
                }
            }
            Some(_) => {
                self.retain_job_progress_rejection(JobProgressRejected::new(semio_framework_actor::JobProgressFault::Stale, publication));
                return JobReplayPublicationPolicy::Rejected;
            }
        }
        let now = semio_framework_job::default_now_us();
        let mut preview_sequence = publication.turn.operation.preview_sequence;
        let mut context = semio_framework_job::StepContext::new(
            semio_framework_job::OperationId(publication.turn.operation.operation),
            semio_framework_job::Generation(publication.turn.operation.generation),
            now.and_then(|now| semio_framework_job::StepBudget::from_duration(2, now, semio_framework_job::INTERACTIVE_LANE_WALL_US)).unwrap_or(semio_framework_job::StepBudget::new(0, 0)),
            semio_framework_job::root_cancel_token(),
            semio_framework_job::default_now_us,
            &mut preview_sequence,
        );
        let admission = match self.job_progress.preflight(&mut context, actor, &publication, live) {
            Ok(admission) => admission,
            Err(fault) => {
                self.retain_job_progress_rejection(JobProgressRejected::new(fault, publication));
                return JobReplayPublicationPolicy::Rejected;
            }
        };
        let identity = JobProgressIdentity::from_publication(actor, &publication);
        let kind = match &publication.outcome {
            semio_framework_actor::JobStepOutcome::Yield => JobProgressKind::Yield,
            semio_framework_actor::JobStepOutcome::PreviewReady { .. } => JobProgressKind::Preview,
            semio_framework_actor::JobStepOutcome::CheckpointReady { .. } => JobProgressKind::Checkpoint,
            semio_framework_actor::JobStepOutcome::Complete { .. } => JobProgressKind::CommitValidated,
            semio_framework_actor::JobStepOutcome::Cancelled => JobProgressKind::Cancelled,
            semio_framework_actor::JobStepOutcome::Fault { .. } => JobProgressKind::Fault,
        };
        let displaced = kind == JobProgressKind::Preview && self.job_progress.has_visible_preview(actor, authority.job);
        let applied_progress = match &publication.outcome {
            semio_framework_actor::JobStepOutcome::CheckpointReady { checkpoint } => checkpoint.applied_progress,
            _ => publication.turn.step_sequence,
        };
        let Some(presentation_token) = job_progress_presentation_bridge().lock().expect("job progress presentation bridge lock").reserve(identity, kind, applied_progress) else {
            self.retain_job_progress_rejection(JobProgressRejected::new(semio_framework_actor::JobProgressFault::Busy, publication));
            return JobReplayPublicationPolicy::Rejected;
        };
        let receipt = match self.job_progress.publish_reserved(&mut context, admission, publication, live) {
            Ok(receipt) => receipt,
            Err(rejected) => {
                let _ = job_progress_presentation_bridge().lock().expect("job progress presentation bridge lock").cancel(presentation_token);
                self.retain_job_progress_rejection(rejected);
                return JobReplayPublicationPolicy::Rejected;
            }
        };
        if self.pending_job_progress_presentations[presentation_token.index].is_some() || !job_progress_presentation_bridge().lock().expect("job progress presentation bridge lock").publish(presentation_token) {
            let _ = job_progress_presentation_bridge().lock().expect("job progress presentation bridge lock").cancel(presentation_token);
            let actor = receipt.identity().actor;
            if self.job_progress.abort(receipt).is_err() {
                let _ = self.job_progress.begin_close_actor(actor);
            }
            return JobReplayPublicationPolicy::Rejected;
        }
        self.pending_job_progress_presentations[presentation_token.index] = Some(PendingJobProgressPresentation { token: presentation_token, receipt });
        if displaced {
            JobReplayPublicationPolicy::Displaced
        } else {
            JobReplayPublicationPolicy::Accepted
        }
    }

    fn plugin_ordinal(&mut self, plugin_id: &str) -> u16 {
        let next = self.plugin_ordinals.len() as u16;
        *self.plugin_ordinals.entry(plugin_id.to_string()).or_insert(next)
    }

    /// 🐛️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-extension-activation): compile
    /// remains a genuine suspension point on the worker-pool-owned request state machine.
    ///
    /// 🐣️ `InstanceOpen` is the first event a fresh instance must receive (`📓️design-abi.md`
    /// §2) — `actor`/`config`/`assets`/`capabilities` are placeholders until a real capability
    /// broker/asset-preload pipeline lands (A2b/T1 territory, not this packet's). It rides
    /// behind the declared `Activate` this app id justifies (`activation_turn_event`), so a
    /// guest woken by an artifact kind learns WHICH kind before it opens anything.
    ///
    /// 🧩️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-extension-activation): descriptor-driven
    /// native cascade — M6's own acceptance wording, "activating a parent brings up its N
    /// extension actors." `wasm_path` is always `<modules_root>/<plugin_id>/<file>.wasm`
    /// (`program_bridge::load_wasm_plugins`'s own layout convention), so the extensions' own
    /// wasm artifacts live as siblings under the same `modules_root`.
    async fn create_app(&mut self, wasm_path: PathBuf, plugin_id: String, app_id: String, artifact_schema: String) -> Result<u32, ProgramFault> {
        let replay_route_index = self.replay_routes.iter().position(Option::is_none).ok_or_else(|| "kernel: fixed replay route registry is full".to_string())?;
        let component = read_native_component(&wasm_path).await?;
        let bytes = component.as_slice();
        let hash = PackageHash(*semio_framework_hash::hash(bytes).as_bytes());
        let plugin_digest = JobReplayRequest::from_spawn(&plugin_id, &[]).tool;
        let artifact_digest = JobReplayRequest::from_spawn(&app_id, &[]).tool;
        let package_id = PackageId(plugin_id.clone());
        let package_ref = PackageRef { package: package_id.clone(), hash };
        let compiled = self.compile_serving_others(package_ref.clone(), bytes.to_vec()).await?;
        if !artifact_schema.is_empty() {
            let codec = semio_framework_plugin_host::OwnedComponentDocumentCodec::try_new(self.guest_runtime.clone(), compiled.clone(), artifact_schema).map_err(|error| error.to_string())?;
            semio_framework_os_kernel::os_store::codec_registry::register_component_document_codec(Arc::new(codec)).map_err(|error| error.to_string())?;
        }
        let instance_id = self.next_instance_id;
        self.next_instance_id += 1;
        let plugin_ordinal = self.plugin_ordinal(&plugin_id);
        let actor = self
            .runtime
            .activate(
                package_id.clone(),
                plugin_ordinal,
                ActorKind::PluginApp { plugin: package_id, app_id: app_id.clone(), instance_id },
                Lane::Interactive,
                None,
                ActivationEvent::Manual,
                &compiled,
                &[] as &[BrokerCapabilityGrant],
                &TURN_BUDGET,
            )
            .await
            .map_err(|refusal| refusal.reason)?;
        self.replay_routes[replay_route_index] = Some(MountedReplayRouteSeed { actor, plugin: plugin_digest, package: hash.0, window: u64::from(instance_id), artifact: artifact_digest });
        self.instances.insert(instance_id, actor);
        let assets = semio_framework_plugin_host::package_assets::package_assets_beside_module(&wasm_path).map_err(|message| format!("kernel: package assets of `{plugin_id}`: {message}"))?;
        let open = Event::InstanceOpen {
            request: semio_framework::kernel::ActorInstanceOpenRequest { activation_generation: 1, instance_id, request_sequence: 1 },
            app_id: semio_framework::kernel::AppInstanceId(app_id.clone()),
            actor: "local".to_string(),
            config: Vec::new(),
            assets,
            capabilities: Vec::new(),
            quotas: QuotaSchema::default(),
        };
        let mut first_turn = Vec::with_capacity(2);
        first_turn.extend(semio_framework_plugin_host::activation::activation_turn_event(&app_id));
        first_turn.push(open);
        self.run_turn(actor, instance_id, first_turn).await?;
        if let Some(modules_root) = wasm_path.parent().and_then(|dir| dir.parent()) {
            self.activate_extensions_of(&plugin_id, actor, modules_root).await;
        }
        Ok(instance_id)
    }

    /// 🧩️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-extension-activation): for every
    /// descriptor in the generated registry with `extends == plugin_id`, compile (`guest_runtime.
    /// compile` is itself keyed/cached by `PackageRef` content hash — no extra cache layer needed
    /// here) and activate it as `ActorKind::Extension`, `Lane::Background` (design doc M6's
    /// decided default — a UI-contributing extension is re-laned on `SurfaceVisible` by whichever
    /// packet wires that event), with `scoped_grants` = [`semio_framework_actor::
    /// intersect_capabilities`] of the parent's own granted set against the extension's requests
    /// — the never-escalate-past-the-parent property. Records the [`semio_framework_actor::Kernel::
    /// link_extension`] cascade edge so [`Self::destroy_app`] takes every extension down with its
    /// parent. Best-effort per extension (mirrors `program_bridge::load_wasm_plugins`'s own "one
    /// bad plugin does not hold the batch hostage" policy) — a missing/broken extension is logged
    /// and skipped, never fails the parent's own `create_app`.
    ///
    /// 🕳️ Honest gap, not worked around: this activates the extension via `ParallelRuntime::
    /// activate` (least-loaded shard), NOT pinned to the parent's exact shard — `ParallelRuntime`
    /// has no `activate_pinned` entry point today (that facade lives in `🎯️targets/🧊️wgpu/📦️packages/🦀️rust/
    /// 🎠️runtime.rs`, owned by a different packet, `kernel-async-native`). The kernel-level
    /// primitive this method WOULD call (`Kernel::activate_pinned`) is built, tested, and green in
    /// `semio-framework-actor`; only the host-level plumbing to reach it through `ParallelRuntime`
    /// is missing. A lease-request for a small additive method is open — see this ticket's report.
    /// `link_extension` (cascade topology, zero-orphan teardown) is unaffected by this gap and
    /// works correctly regardless of which shard the extension landed on.
    ///
    /// 🕳️ Honest gap: the REAL capability enforcement point for a guest instance is the
    /// `caps: &[BrokerCapabilityGrant]` argument below, `&[]` here because this native host
    /// has no capability broker wired up for ANY actor kind yet — the parent's own
    /// activation above passes the identical empty placeholder (A2b/T1 territory). The
    /// `intersect_capabilities` call still records the correctly-scoped grant kernel-side
    /// (`set_capabilities` below) so the intersection mechanism is exercised end-to-end and
    /// ready the moment a broker starts populating `parent_grants` for real.
    async fn activate_extensions_of(&mut self, plugin_id: &str, parent: ActorId, modules_root: &std::path::Path) {
        let extensions = extension_index().extensions_of(plugin_id);
        if extensions.is_empty() {
            return;
        }
        let parent_grants = self.runtime.kernel().actor_record(parent).await.map(|record| record.capabilities).unwrap_or_default();
        for extension in extensions.to_vec() {
            let Some(replay_route_index) = self.replay_routes.iter().position(Option::is_none) else {
                crate::log_debug("kernel: fixed replay route registry is full; extension activation refused");
                break;
            };
            let Some(parent_route) = self.replay_routes.iter().flatten().find(|route| route.actor == parent).copied() else {
                crate::log_debug("kernel: parent replay route disappeared before extension activation");
                break;
            };
            let extension_dir = modules_root.join(&extension.extension_id);
            let Some(extension_wasm_path) = find_wasm_artifact(&extension_dir).await else {
                crate::log_debug(&format!("kernel: extension {} of {plugin_id} has no compiled wasm under {}, skipping", extension.extension_id, extension_dir.display()));
                continue;
            };
            let mut extension_bytes_owner = match crate::run_renderer_io(semio_framework_os_services::NativeIoRequest::ReadBytes(extension_wasm_path.clone())).await {
                Ok(semio_framework_os_services::NativeIoValue::Bytes(bytes)) => bytes,
                Ok(_) => {
                    crate::log_debug(&format!("kernel: native I/O returned the wrong value for extension {}", extension.extension_id));
                    continue;
                }
                Err(error) => {
                    crate::log_debug(&format!("kernel: failed reading extension {} wasm ({}): {error}", extension.extension_id, extension_wasm_path.display()));
                    continue;
                }
            };
            let Some(extension_bytes) = extension_bytes_owner.single_page() else {
                crate::log_debug(&format!("kernel: extension {} exceeds the mounted single-page retained compiler authority", extension.extension_id));
                continue;
            };
            let extension_hash = PackageHash(*semio_framework_hash::hash(extension_bytes).as_bytes());
            let extension_package_ref = PackageRef { package: extension.package.clone(), hash: extension_hash };
            let extension_compiled = self.guest_runtime.compile(&extension_package_ref, extension_bytes).await;
            let _ = extension_bytes_owner.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            let extension_compiled = match extension_compiled {
                Ok(handle) => handle,
                Err(error) => {
                    crate::log_debug(&format!("kernel: compile failed for extension {}: {error}", extension.extension_id));
                    continue;
                }
            };
            let extension_ordinal = self.plugin_ordinal(&extension.extension_id);
            let extension_kind = ActorKind::Extension { plugin: PackageId(plugin_id.to_string()), extension_id: extension.extension_id.clone() };
            match self.runtime.activate(extension.package.clone(), extension_ordinal, extension_kind, Lane::Background, None, ActivationEvent::Manual, &extension_compiled, &[] as &[BrokerCapabilityGrant], &TURN_BUDGET).await {
                Ok(extension_actor) => {
                    self.replay_routes[replay_route_index] =
                        Some(MountedReplayRouteSeed { actor: extension_actor, plugin: JobReplayRequest::from_spawn(&extension.extension_id, &[]).tool, package: extension_hash.0, window: parent_route.window, artifact: parent_route.artifact });
                    let scoped_grants = intersect_capabilities(&parent_grants, &extension.capability_requests).await;
                    if let Err(error) = self.runtime.kernel_mut().set_capabilities(extension_actor, scoped_grants).await {
                        crate::log_debug(&format!("kernel: set_capabilities({extension_actor:?}) failed: {error}"));
                    }
                    if let Err(error) = self.runtime.kernel_mut().link_extension(parent, extension_actor).await {
                        crate::log_debug(&format!("kernel: link_extension({parent:?}, {extension_actor:?}) failed: {error}"));
                    }
                }
                Err(error) => crate::log_debug(&format!("kernel: activate failed for extension {}: {error}", extension.extension_id)),
            }
        }
    }

    async fn destroy_app_step(&mut self, instance: u32) -> bool {
        if let Some(index) = self.product_replay_claims.iter().position(|claim| claim.as_ref().is_some_and(|claim| claim.instance() == instance)) {
            self.product_replay_claims[index].take().expect("destroyed app product replay claim remains exact").retire();
            return false;
        }
        if let Some(index) = self.product_replay_authorities.iter().position(|authority| authority.as_ref().is_some_and(|authority| authority.instance() == instance)) {
            self.product_replay_authorities[index].take().expect("destroyed app product replay authority remains exact").retire();
            return false;
        }
        if mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").has_instance(instance) {
            let _ = mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").close_instance_one(instance);
            return false;
        }
        if mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").has_instance(instance) {
            let _ = mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").close_instance_one(instance);
            return false;
        }
        let _ = self.retained_command_closes.begin_close_key(u64::from(instance));
        let _ = self.queued_command_closes.begin_close_key(u64::from(instance));
        self.command_document_closes.begin_close_instance(u64::from(instance));
        let close_index = self.closing_apps.iter().position(|slot| slot.as_ref().is_some_and(|close| close.instance == instance));
        let close_index = match close_index {
            Some(index) => index,
            None => {
                let Some(actor) = self.instances.remove(&instance) else { return true };
                let removed = self.runtime.kernel_mut().deactivate(actor).await.unwrap_or_else(|_| vec![actor]);
                for slot in &mut self.reserved_jobs {
                    if slot.as_ref().is_some_and(|job| removed.contains(&job.actor)) {
                        *slot = None;
                    }
                }
                if removed.len() > JOB_PROGRESS_ACTIVE_CAPACITY {
                    let _ = self.job_progress.begin_close_all();
                    return false;
                }
                let Some(index) = self.closing_apps.iter().position(Option::is_none) else {
                    let _ = self.job_progress.begin_close_all();
                    return false;
                };
                let mut actors = [ActorId(0); JOB_PROGRESS_ACTIVE_CAPACITY];
                let actor_count = removed.len();
                actors[..actor_count].copy_from_slice(&removed);
                self.closing_apps[index] = Some(ClosingKernelApp { instance, actors, actor_count, replay_begin_cursor: 0, begin_cursor: 0, unregister_cursor: 0, unregister_refusal: None, route_cursor: 0 });
                return false;
            }
        };
        let mut closing = self.closing_apps[close_index].take().expect("closing app slot");
        if let Some(index) = self.pending_job_progress_presentations.iter().position(|pending| pending.as_ref().is_some_and(|pending| closing.contains(pending.receipt.identity().actor))) {
            let token = self.pending_job_progress_presentations[index].as_ref().expect("pending presentation").token;
            if !job_progress_presentation_bridge().lock().expect("job progress presentation bridge lock").cancel(token) {
                self.closing_apps[close_index] = Some(closing);
                return false;
            }
            let pending = self.pending_job_progress_presentations[index].take().expect("cancelled pending presentation");
            let actor = pending.receipt.identity().actor;
            if self.job_progress.abort(pending.receipt).is_err() {
                let _ = self.job_progress.begin_close_actor(actor);
            }
            self.closing_apps[close_index] = Some(closing);
            return false;
        }
        if closing.replay_begin_cursor < closing.actor_count {
            let actor = closing.actors[closing.replay_begin_cursor];
            if let Some(entry) = self.job_replays.iter_mut().flatten().find(|entry| entry.actor == actor && !entry.log.close_started()) {
                entry.log.begin_close();
            } else {
                closing.replay_begin_cursor += 1;
            }
            self.closing_apps[close_index] = Some(closing);
            return false;
        }
        if closing.begin_cursor < closing.actor_count {
            let actor = closing.actors[closing.begin_cursor];
            if self.job_progress.begin_close_actor(actor).is_ok() {
                closing.begin_cursor += 1;
            }
            self.closing_apps[close_index] = Some(closing);
            return false;
        }
        if closing.unregister_cursor < closing.actor_count {
            if let Err(error) = self.runtime.unregister(closing.actors[closing.unregister_cursor]).await {
                closing.unregister_refusal = Some(error);
                self.closing_apps[close_index] = Some(closing);
                return false;
            }
            closing.unregister_cursor += 1;
            self.closing_apps[close_index] = Some(closing);
            return false;
        }
        if closing.actors[..closing.actor_count].iter().any(|actor| !self.job_progress.actor_terminal_is_empty(*actor)) {
            let _ = self.command_maintenance_step();
            self.closing_apps[close_index] = Some(closing);
            return false;
        }
        if closing.actors[..closing.actor_count].iter().any(|actor| self.job_replays.iter().flatten().any(|entry| entry.actor == *actor)) {
            let _ = self.command_maintenance_step();
            self.closing_apps[close_index] = Some(closing);
            return false;
        }
        if !self.command_document_closes.instance_is_empty(u64::from(instance)) {
            let _ = self.command_maintenance_step();
            self.closing_apps[close_index] = Some(closing);
            return false;
        }
        if !self.retained.advance_instance_close_one(instance) {
            self.closing_apps[close_index] = Some(closing);
            return false;
        }
        if self.pending_rejections.take_instance_one(instance).is_some() {
            self.closing_apps[close_index] = Some(closing);
            return false;
        }
        if closing.route_cursor < closing.actor_count {
            let actor = closing.actors[closing.route_cursor];
            if let Some(index) = self.replay_routes.iter().position(|seed| seed.is_some_and(|seed| seed.actor == actor)) {
                self.replay_routes[index] = None;
            }
            closing.route_cursor += 1;
            self.closing_apps[close_index] = Some(closing);
            return false;
        }
        self.pending_rejections.instance_is_empty(instance)
    }

    fn close_realm_progress_step(&mut self) -> bool {
        if let Some(index) = self.product_replay_claims.iter().position(Option::is_some) {
            self.product_replay_claims[index].take().expect("realm product replay claim remains exact").retire();
            return false;
        }
        if let Some(index) = self.product_replay_authorities.iter().position(Option::is_some) {
            self.product_replay_authorities[index].take().expect("realm product replay authority remains exact").retire();
            return false;
        }
        if mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").has_any() {
            let _ = mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").step_one(false);
            return false;
        }
        if mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").has_close_work() {
            let _ = mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").close_one();
            return false;
        }
        if let Some(index) = self.pending_job_progress_presentations.iter().position(Option::is_some) {
            let token = self.pending_job_progress_presentations[index].as_ref().expect("realm pending presentation").token;
            if !job_progress_presentation_bridge().lock().expect("job progress presentation bridge lock").cancel(token) {
                return false;
            }
            let pending = self.pending_job_progress_presentations[index].take().expect("cancelled realm presentation");
            let actor = pending.receipt.identity().actor;
            if self.job_progress.abort(pending.receipt).is_err() {
                let _ = self.job_progress.begin_close_actor(actor);
            }
            return false;
        }
        if let Some(entry) = self.job_replays.iter_mut().flatten().find(|entry| !entry.log.close_started()) {
            entry.log.begin_close();
            return false;
        }
        if self.job_replays.iter().any(Option::is_some) {
            let _ = self.command_maintenance_step();
            return false;
        }
        if let Some(index) = self.replay_routes.iter().position(Option::is_some) {
            self.replay_routes[index] = None;
            return false;
        }
        if !self.realm_progress_close_started {
            self.job_progress.begin_close_all();
            self.realm_progress_close_started = true;
            return false;
        }
        if !self.job_progress.terminal_is_empty() || self.rejected_job_progress.iter().any(Option::is_some) || !job_progress_presentation_bridge().lock().expect("job progress presentation bridge lock").terminal_is_empty() {
            let _ = self.command_maintenance_step();
            return false;
        }
        if !self.retained.advance_realm_close_one() {
            return false;
        }
        if self.pending_rejections.take_any_one().is_some() {
            return false;
        }
        if !self.pending_rejections.terminal_is_empty() {
            return false;
        }
        self.realm_progress_close_started = false;
        true
    }

    /// 🎫️ `Event::PatchRejected` carries the issued-patch authority the guest minted with the
    /// patch. A rejection with no authority behind it (a surface the registry could not admit
    /// at all, so no turn ever reached a slot) has nothing to reject back and stays a local
    /// registry entry rather than a fabricated receipt.
    async fn exchange(&mut self, instance: u32, mut events: Vec<Event>) -> Result<ExchangeOutcome, ProgramFault> {
        let Some(&actor) = self.instances.get(&instance) else {
            return Err((format!("kernel: instance {instance} is not registered")).into());
        };
        if let Some(rejection) = self.pending_rejections.take_instance_one(instance) {
            if let Some(receipt) = rejection.receipt {
                events.insert(0, Event::PatchRejected { receipt, surface: rejection.surface.0.as_str().to_string(), revision: rejection.revision.0, reason: rejection.reason.to_string() });
            }
        }
        let mut outcome = self.run_turn(actor, instance, events).await?;
        self.settle_reserved_jobs(actor, instance, &mut outcome).await?;
        Ok(outcome)
    }

    async fn exchange_commands(&mut self, instance: u32, driver: semio_framework::kernel::CommandBatchDriver) -> Result<ExchangeOutcome, ProgramFault> {
        let key = u64::from(instance);
        let generation = driver.generation();
        if !self.retained_command_closes.terminal_is_empty() {
            if !self.queued_command_closes.can_insert(key) {
                return Err(("kernel: retained and queued command close registries are saturated; caller owner remains in the queue close lane".to_string()).into());
            }
            self.queued_command_closes.insert_admitted(key, generation, driver);
            self.queued_command_closes.begin_close(key, generation).map_err(|fault| fault.describe())?;
            let _ = self.command_maintenance_step();
            return Err(("kernel: previous cancelled command owner is closing; incoming exact batch moved to the queued close lane".to_string()).into());
        }
        if !self.retained_command_closes.can_insert(key) {
            return Err(("kernel: retained command close registry is saturated or collided".to_string()).into());
        }
        self.retained_command_closes.insert_admitted(key, generation, driver);
        let Some(&actor) = self.instances.get(&instance) else {
            self.retained_command_closes.begin_close(key, generation).map_err(|fault| fault.describe())?;
            let _ = self.command_maintenance_step();
            return Err((format!("kernel: instance {instance} is not registered; exact command owner entered bounded close")).into());
        };
        let mut combined = ExchangeOutcome { frames: Vec::new(), surfaces: UiFixedList::default(), effects: Vec::new(), command_ingress: semio_framework::kernel::CommandIngressStatus::Idle, typed_results: Vec::new() };
        loop {
            let mut destinations = match self.command_document_closes.try_reserve_page(key, generation) {
                Ok(destinations) => destinations,
                Err(()) => {
                    self.command_document_closes.begin_close_batch(key, generation);
                    let _ = self.retained_command_closes.begin_close(key, generation);
                    let _ = self.command_maintenance_step();
                    return Err(("kernel: command document destinations saturated before the next turn; retained batch and exact documents entered bounded close".to_string()).into());
                }
            };
            let page = self.retained_command_closes.with_driver_mut(key, generation, |driver| driver.next_page());
            let page = match page {
                Ok(Ok(page)) => page,
                Ok(Err(fault)) => {
                    self.command_document_closes.release(&mut destinations);
                    self.command_document_closes.begin_close_batch(key, generation);
                    let _ = self.retained_command_closes.begin_close(key, generation);
                    let _ = self.command_maintenance_step();
                    return Err((fault.describe()).into());
                }
                Err(fault) => {
                    self.command_document_closes.release(&mut destinations);
                    self.command_document_closes.begin_close_batch(key, generation);
                    let _ = self.retained_command_closes.begin_close(key, generation);
                    let _ = self.command_maintenance_step();
                    return Err((fault.describe()).into());
                }
            };
            let events = match page {
                Some((cursor, bytes)) => vec![Event::CommandIngressPage { cursor, bytes }],
                None => vec![Event::Wake],
            };
            if let Err(fault) = self.retained_command_closes.prepare_suspend(key, generation) {
                self.command_document_closes.release(&mut destinations);
                self.command_document_closes.begin_close_batch(key, generation);
                let _ = self.retained_command_closes.begin_close(key, generation);
                let _ = self.command_maintenance_step();
                return Err((fault.describe()).into());
            }
            let outcome = match self.run_turn(actor, instance, events).await {
                Ok(outcome) => outcome,
                Err(fault) => {
                    self.command_document_closes.release(&mut destinations);
                    let _ = self.retained_command_closes.resume(key, generation);
                    self.command_document_closes.begin_close_batch(key, generation);
                    let _ = self.retained_command_closes.begin_close(key, generation);
                    let _ = self.command_maintenance_step();
                    return Err((fault).into());
                }
            };
            let ExchangeOutcome { frames, surfaces, effects, command_ingress, typed_results } = outcome;
            for surface in surfaces {
                self.command_document_closes.admit(&mut destinations, surface);
            }
            self.command_document_closes.release(&mut destinations);
            if let Err(fault) = self.retained_command_closes.resume(key, generation) {
                self.command_document_closes.begin_close_batch(key, generation);
                let _ = self.retained_command_closes.begin_close(key, generation);
                let _ = self.command_maintenance_step();
                return Err((fault.describe()).into());
            }
            let progress = match self.retained_command_closes.with_driver_mut(key, generation, |driver| driver.observe(&command_ingress, semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES)) {
                Ok(Ok(progress)) => progress,
                Ok(Err(fault)) => {
                    self.command_document_closes.begin_close_batch(key, generation);
                    let _ = self.retained_command_closes.begin_close(key, generation);
                    let _ = self.command_maintenance_step();
                    return Err((fault.describe()).into());
                }
                Err(fault) => {
                    self.command_document_closes.begin_close_batch(key, generation);
                    let _ = self.retained_command_closes.begin_close(key, generation);
                    let _ = self.command_maintenance_step();
                    return Err((fault.describe()).into());
                }
            };
            combined.frames.extend(frames);
            combined.effects.extend(effects);
            combined.typed_results.extend(typed_results);
            combined.command_ingress = command_ingress;
            match progress {
                semio_framework::kernel::CommandBatchProgress::Complete => {
                    if let Err(fault) = self.retained_command_closes.remove_terminal(key, generation) {
                        self.command_document_closes.begin_close_batch(key, generation);
                        let _ = self.retained_command_closes.begin_close(key, generation);
                        let _ = self.command_maintenance_step();
                        return Err((fault.describe()).into());
                    }
                    self.command_document_closes.publish_batch(key, generation, &mut combined.surfaces);
                    self.settle_reserved_jobs(actor, instance, &mut combined).await?;
                    return Ok(combined);
                }
                semio_framework::kernel::CommandBatchProgress::Faulted => {
                    self.command_document_closes.begin_close_batch(key, generation);
                    self.retained_command_closes.begin_close(key, generation).map_err(|fault| fault.describe())?;
                    let _ = self.command_maintenance_step();
                    return Err(("kernel: command ingress faulted; exact driver and surface-document owners entered incremental close".to_string()).into());
                }
                semio_framework::kernel::CommandBatchProgress::PageReady | semio_framework::kernel::CommandBatchProgress::Waiting => {}
            }
        }
    }

    /// ⏯️ One request's turn for `actor`, settled the way the React host settles one
    /// (`🔌️PluginRuntime`'s `settlePluginTurn`, ticket 26/09/23 slice WG8, B1):
    ///
    /// - a grant the shard ended with the guest mid-call ([`ShardOutcome::Preempted`]) is resumed
    ///   inside [`Self::run_turn_once`] before anything else reaches the actor, so this method never
    ///   hands a mid-flight guest back to its caller;
    /// - a settled turn that published an `ActorInstanceLifecycleReceipt` is acknowledged on the next
    ///   turn (an open the host never acknowledges is re-offered on every later turn);
    /// - a completed turn answering `MoreWork` is continued with empty-event turns while its command
    ///   and cold-pair ingress are idle — a non-idle ingress belongs to the caller's own page driver
    ///   ([`Self::exchange_commands`] observes every ingress status, so a settle that swallowed one
    ///   would resend a page the guest already owns: measured as block2d's `addHandleKind` spinning
    ///   `MoreWork` until the settle budget, ticket 26/09/18 slice G7w run 13);
    /// - it stops once [`run_turn_quiescent_continuations`] continuations in a row carried nothing,
    ///   because a guest waiting on a host round trip (a typed-operation ACK, a backbone reply) is
    ///   answering `MoreWork` for something no empty turn can deliver.
    ///
    /// 🧵️ Events cross ONE per granted turn and each settles before the next is submitted: the shard
    /// executes one event per guest turn anyway, and a batch's later envelopes must not reach a guest
    /// the first one left owning ingress. [`RUN_TURN_SETTLE_BUDGET`] bounds the whole request.
    async fn run_turn(&mut self, actor: ActorId, instance: u32, events: Vec<Event>) -> Result<ExchangeOutcome, ProgramFault> {
        let deadline = std::time::Instant::now() + RUN_TURN_SETTLE_BUDGET;
        let mut batches = events.into_iter().map(|event| vec![event]).collect::<std::collections::VecDeque<_>>();
        if batches.is_empty() {
            batches.push_back(Vec::new());
        }
        let mut settled: Option<ExchangeOutcome> = None;
        while let Some(batch) = batches.pop_front() {
            let first = self.run_turn_once(actor, instance, batch).await?;
            let outcome = self.settle_turn(actor, instance, first, deadline).await?;
            match settled.as_mut() {
                Some(earlier) => earlier.absorb(outcome)?,
                None => settled = Some(outcome),
            }
        }
        settled.ok_or_else(|| ProgramFault::from("kernel: a turn with no batch produced no outcome"))
    }

    /// 🔁️ Continues one granted turn with what it still owes (see [`Self::run_turn`]) until it settled,
    /// [`run_turn_quiescent_continuations`] continuations in a row carried nothing, or `deadline` passed.
    async fn settle_turn(&mut self, actor: ActorId, instance: u32, (mut outcome, continuation): (ExchangeOutcome, Option<Vec<Event>>), deadline: std::time::Instant) -> Result<ExchangeOutcome, ProgramFault> {
        let mut owed = run_turn_continuation_turns(continuation);
        let mut quiet = 0usize;
        while let Some(events) = owed.pop_front() {
            if std::time::Instant::now() >= deadline || quiet >= run_turn_quiescent_continuations() {
                break;
            }
            let (later, next) = self.run_turn_once(actor, instance, events).await?;
            quiet = if later.carries_nothing() { quiet + 1 } else { 0 };
            outcome.absorb(later)?;
            owed.extend(run_turn_continuation_turns(next));
        }
        Ok(outcome)
    }

    /// 🧰️ Drives every live [`ReservedToolJob`] of `actor` to its end once the request that spawned it
    /// settled, folding what the drive produced into `outcome`: the native host's half of the one
    /// reserved-job mechanism (React: `driveReservedToolJob`; guest: `admit_reserved_spawned_job`, ticket
    /// 26/09/23 slices C8 and WG8 §1.4).
    ///
    /// - The shard started the job when it admitted the `Effect::SpawnJob` and reported its exact
    ///   [`JobTurn`] with that turn ([`ShardOutcome::Turn`]'s `jobs`); one `Payload::JobStep` is granted
    ///   per turn until a step publication ends the job.
    /// - The shard then runs the guest's deferred `Event::JobCompleted` turn by itself; this host waits
    ///   for it and settles it like any turn, so the guest commits the job one unit per `MoreWork`
    ///   continuation and answers the verb (`Invocation { in_reply_to: 0 }`, `OperationCompleted`).
    async fn settle_reserved_jobs(&mut self, actor: ActorId, instance: u32, outcome: &mut ExchangeOutcome) -> Result<(), ProgramFault> {
        while self.reserved_jobs.iter().flatten().any(|job| job.actor == actor) {
            if self.turn_cancelled() {
                return Err((self.retire_cancelled_turn(actor).await).into());
            }
            let first = self.run_reserved_job_once(actor, instance).await?;
            let later = self.settle_turn(actor, instance, first, std::time::Instant::now() + RUN_TURN_SETTLE_BUDGET).await?;
            outcome.absorb(later)?;
            if !self.reserved_jobs.iter().flatten().any(|job| job.actor == actor && job.step.is_none()) {
                self.yield_slice(actor, instance).await;
            }
        }
        Ok(())
    }

    /// 🛑️ Whether the turn being driven must end now: its requester cancelled it (a cancellable request whose
    /// response slot was abandoned — a document open's create) or the realm is closing (a realm close waits
    /// at the head of the queue). Nothing else ends a turn: a slow guest is never killed by wall-clock time.
    fn turn_cancelled(&self) -> bool {
        self.serving.last().is_some_and(|(slot, cancellable)| *cancellable && slot.abandoned()) || self.request_queue.head_is(|request| matches!(request, KernelRequest::CloseRealm { .. }))
    }

    /// ✂️ Retires an actor whose turn was cancelled mid-call: a mid-flight guest can never be handed back,
    /// so its shard instance is dropped, the scheduler stops granting it, and every host record of it goes.
    /// Answers the error its request reports.
    async fn retire_cancelled_turn(&mut self, actor: ActorId) -> ProgramFault {
        let _ = self.runtime.kernel_mut().suspend(actor, None).await;
        if let Err(error) = self.runtime.unregister(actor).await { return error.into(); }
        self.instances.retain(|_, instance_actor| *instance_actor != actor);
        for route in self.replay_routes.iter_mut().filter(|route| route.is_some_and(|route| route.actor == actor)) {
            *route = None;
        }
        for job in self.reserved_jobs.iter_mut().filter(|job| job.is_some_and(|job| job.actor == actor)) {
            *job = None;
        }
        self.begin_fault_close(actor);
        #[cfg(test)]
        CANCELLED_KERNEL_TURNS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        format!("kernel: actor {}'s turn was cancelled by its owner", actor.0).into()
    }

    /// 🧩️ Compiles `bytes` on the worker pool while this loop keeps serving the other instances' requests — a
    /// large component parses for minutes in a debug host, and the synchronous parse used to hold the whole
    /// loop — reusing the handle already compiled for the same content hash. The open's owner may cancel it
    /// while it parses ([`Self::turn_cancelled`]); the parse then finishes unobserved on its worker.
    async fn compile_serving_others(&mut self, package: PackageRef, bytes: Vec<u8>) -> Result<semio_framework_plugin_host::CompiledHandle, String> {
        if let Some((_, compiled)) = self.compiled_components.iter().find(|(hash, _)| *hash == package.hash) {
            return Ok(compiled.clone());
        }
        let handoff: Arc<Mutex<(Option<Result<semio_framework_plugin_host::CompiledHandle, String>>, Option<Waker>)>> = Arc::default();
        let worker = handoff.clone();
        let runtime = self.guest_runtime.clone();
        let key = package.clone();
        let _ = KernelPoolFuture::spawn(crate::renderer_worker_pool(), semio_framework_async::Lane::Background, async move {
            let result = runtime.compile(&key, &bytes).await.map_err(|error| error.to_string());
            let waker = {
                let mut slot = worker.lock().expect("compile handoff lock");
                slot.0 = Some(result);
                slot.1.take()
            };
            if let Some(waker) = waker {
                waker.wake();
            }
        });
        loop {
            let gap = std::future::poll_fn(|cx| {
                let mut slot = handoff.lock().expect("compile handoff lock");
                if let Some(result) = slot.0.take() {
                    return Poll::Ready(Some(result));
                }
                slot.1 = Some(cx.waker().clone());
                drop(slot);
                if self.turn_cancelled() {
                    return Poll::Ready(Some(Err("kernel: the open was cancelled while its component compiled".to_string())));
                }
                if !self.interleaving && self.request_queue.head_is(|request| kernel_request_interleavable(request, u32::MAX)) {
                    return Poll::Ready(None);
                }
                self.request_queue.watch(cx.waker());
                Poll::Pending
            })
            .await;
            match gap {
                Some(result) => {
                    let compiled = result?;
                    if self.compiled_components.len() == COMPILED_COMPONENT_CAPACITY {
                        self.compiled_components.pop();
                    }
                    self.compiled_components.insert(0, (package.hash, compiled.clone()));
                    return Ok(compiled);
                }
                None => self.serve_head_between(u32::MAX).await,
            }
        }
    }

    /// ⚖️ Serves the head request when it may run beside work mid-flight on instance `busy`, marking the gap so a
    /// request served in it never opens another.
    async fn serve_head_between(&mut self, busy: u32) {
        let Some((request, slot)) = self.request_queue.try_next_if(|request| kernel_request_interleavable(request, busy)) else { return };
        self.interleaving = true;
        let outcome = self.serve(request, &slot).await;
        self.interleaving = false;
        if let Some(outcome) = outcome {
            slot.deliver(outcome);
        }
    }

    /// ⚖️ Gives the other actors one request between two slices of `actor`'s turn on instance `busy` (the
    /// fairness contract's `requestsBetweenSlices`): the head of the queue runs when it may
    /// ([`kernel_request_interleavable`]) while `actor` is suspended in the scheduler, so no grant of that
    /// request can reach the mid-flight guest; then `actor` is resumed. A request running in a slice gap
    /// never opens another gap.
    async fn yield_slice(&mut self, actor: ActorId, busy: u32) {
        if self.interleaving || !self.request_queue.head_is(|request| kernel_request_interleavable(request, busy)) {
            return;
        }
        let suspended = self.runtime.kernel_mut().suspend(actor, None).await.is_ok();
        for _ in 0..TURN_REQUESTS_BETWEEN_SLICES {
            self.serve_head_between(busy).await;
        }
        if suspended {
            let _ = self.runtime.kernel_mut().resume(actor).await;
        }
    }

    /// 🧾️ Registers every reserved-kind `Effect::SpawnJob` of `actor`'s settled turn with the
    /// shard-minted [`JobTurn`] its admission reported; a spawn the shard refused has none and only owes
    /// its deferred `Event::JobCompleted` refusal.
    fn admit_reserved_jobs(&mut self, actor: ActorId, effects: &[Effect], jobs: &[JobTurn]) -> Result<(), String> {
        for effect in effects {
            let Effect::SpawnJob { job, kind, .. } = effect else { continue };
            if kind != semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND {
                continue;
            }
            let slot = self.reserved_jobs.iter_mut().find(|slot| slot.is_none()).ok_or_else(|| format!("kernel: actor {}'s reserved tool job {job} exceeds {JOB_PROGRESS_ACTIVE_CAPACITY} live reserved jobs", actor.0))?;
            *slot = Some(ReservedToolJob { actor, job: *job, step: jobs.iter().find(|turn| turn.job == *job).copied() });
        }
        Ok(())
    }

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
    async fn run_turn_once(&mut self, actor: ActorId, instance: u32, events: Vec<Event>) -> Result<(ExchangeOutcome, Option<Vec<Event>>), ProgramFault> {
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
    async fn run_reserved_job_once(&mut self, actor: ActorId, instance: u32) -> Result<(ExchangeOutcome, Option<Vec<Event>>), ProgramFault> {
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
                if retained.published.is_none() {
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
                    if requested.is_some() && retained.published.is_none() {
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
    let identity_policy=semio_framework_pack_json::from_json_str(include_str!("../../../../../../../🔌️plugin/🖥️host/🧵️shard/🪪️identity/⚙️configuration/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).expect("authored native shard identity policy");
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
    fn serve<'a>(&'a mut self, request: KernelRequest, slot: &Arc<ResponseSlot>) -> Pin<Box<dyn Future<Output = Option<KernelOutcome>> + Send + 'a>> {
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
                let (wasm_path, plugin_id, app_id, artifact_schema) = owner.into_parts();
                KernelOutcome::Created(self.create_app(wasm_path, plugin_id, app_id, artifact_schema).await)
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
