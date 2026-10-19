//! 🧩️ App lifecycle and realm close: the captured job progress, the app creation, the extension activation and the app destroy and realm close steps. See [`super`].

use super::*;

//#region 🎠️KernelRuntime
//#region 🔖️KernelPoolState
impl KernelPoolState {
    pub(super) fn retain_job_progress_rejection(&mut self, rejected: JobProgressRejected) {
        let rejected = match self.job_progress.retain_rejected(rejected) {
            Ok(()) => return,
            Err(rejected) => rejected,
        };
        let slot = self.rejected_job_progress.iter_mut().find(|slot| slot.is_none()).expect("one kernel decision cannot exceed the fixed 64-actor rejected-publication handback registry");
        *slot = Some(rejected);
    }

    pub(super) fn publish_captured_job_progress(&mut self, actor: ActorId, authority: JobTurn, publication: JobPublication) -> JobReplayPublicationPolicy {
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
        let mut retained_progress = semio_framework_job::RetainedCloneProgress::default();
        let mut context = semio_framework_job::StepContext::new(
            semio_framework_job::OperationId(publication.turn.operation.operation),
            semio_framework_job::Generation(publication.turn.operation.generation),
            now.and_then(|now| semio_framework_job::StepBudget::from_duration(2, now, semio_framework_job::INTERACTIVE_LANE_WALL_US, JOB_REPLAY_STEP_GRANT)).unwrap_or(semio_framework_job::StepBudget::new(0, 0, JOB_REPLAY_STEP_GRANT)),
            semio_framework_job::root_cancel_token(),
            semio_framework_job::default_now_us,
            &mut preview_sequence,
            &mut retained_progress,
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
    pub(super) async fn create_app(&mut self, owner:CreateAppRequestOwner) -> Result<u32, ProgramFault> {
        let (wasm_path,plugin_id,app_id,artifact_schema)=(owner.path(),owner.plugin(),owner.app(),owner.schema());
        let replay_route_index = self.replay_routes.iter().position(Option::is_none).ok_or_else(|| "kernel: fixed replay route registry is full".to_string())?;
        let component = read_native_component(&wasm_path).await.map_err(|error|error.to_string())?;
        let bytes = component.bytes();
        let hash = PackageHash(*semio_framework_hash::hash(bytes).as_bytes());
        let plugin_digest = JobReplayRequest::from_spawn(&plugin_id, &[]).tool;
        let artifact_digest = JobReplayRequest::from_spawn(&app_id, &[]).tool;
        let compiled=match self.compile_serving_others(&owner,hash,component).await{Ok(compiled)=>compiled,Err(kind)=>return Err(owner.into_failure(kind).into())};
        let package_id=PackageId(plugin_id.to_owned());
        if !artifact_schema.is_empty() {
            let codec = semio_framework_plugin_host::OwnedComponentDocumentCodec::try_new(self.guest_runtime.clone(), compiled.clone(), artifact_schema.to_owned()).map_err(|error| error.to_string())?;
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
                ActorKind::PluginApp { plugin: package_id, app_id: app_id.to_owned(), instance_id },
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
            app_id: semio_framework::kernel::AppInstanceId(app_id.to_owned()),
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
            let extension_wasm_path=match find_wasm_artifact(&extension_dir).await{Ok(Some(path))=>path,Ok(None)=>{crate::log_debug(&format!("kernel: extension {} of {plugin_id} has no compiled wasm under {}, skipping",extension.extension_id,extension_dir.display()));continue},Err(error)=>{crate::log_debug(&format!("kernel: extension directory {} failed: {error}",extension_dir.display()));continue}};
            let extension_bytes_owner=match read_native_component(extension_wasm_path.path()).await{
                Ok(bytes)=>bytes,
                Err(error)=>{crate::log_debug(&format!("kernel: failed reading extension {} wasm ({}): {error}",extension.extension_id,extension_wasm_path.path().display()));continue}
            };
            let extension_bytes=extension_bytes_owner.bytes();
            let extension_hash = PackageHash(*semio_framework_hash::hash(extension_bytes).as_bytes());
            let extension_package_ref = PackageRef { package: extension.package.clone(), hash: extension_hash };
            let extension_compiled = self.guest_runtime.compile(&extension_package_ref, extension_bytes).await;
            drop(extension_bytes_owner);
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

    pub(super) async fn destroy_app_step(&mut self, instance: u32) -> bool {
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

}
//#endregion
//#endregion 🎠️KernelRuntime
