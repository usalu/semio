//! 🏗️ Pool construction and product replay: the `KernelPoolState` constructor, the product replay mount and the product replay advance. See [`super`].

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

    pub(super) fn mount_product_replay(&mut self, mut owner: MountedProductReplayRequest) -> Result<(), MountedProductReplayRequest> {
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

    pub(super) fn product_replay_pending_instance(&self) -> Option<u32> {
        self.product_replay_authorities.iter().flatten().next().map(MountedProductReplayAuthority::instance).or_else(|| self.product_replay_claims.iter().flatten().next().map(MountedProductReplayClaim::instance))
    }

    pub(super) fn abort_product_replay_one(&mut self, instance: u32) {
        if let Some(index) = self.product_replay_authorities.iter().position(|authority| authority.as_ref().is_some_and(|authority| authority.instance() == instance)) {
            self.product_replay_authorities[index].take().expect("faulted product replay authority remains exact").retire();
        } else if let Some(index) = self.product_replay_claims.iter().position(|claim| claim.as_ref().is_some_and(|claim| claim.instance() == instance)) {
            self.product_replay_claims[index].take().expect("faulted product replay claim remains exact").retire();
        }
    }

    pub(super) async fn advance_product_replay(&mut self, instance: u32) -> Result<ExchangeOutcome, ProgramFault> {
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

}
//#endregion
//#endregion 🎠️KernelRuntime
