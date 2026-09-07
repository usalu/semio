use super::*;

enum CommandIngressOwner {
    ReservedPresence { cursor: semio_framework::kernel::CommandPageCursor, admission: crate::app::PresenceRosterAdmission, page: semio_framework::kernel::FixedCommandPage },
    Presence { cursor: semio_framework::kernel::CommandPageCursor, publication_generation: u64 },
    PendingPresencePage { cursor: semio_framework::kernel::CommandPageCursor, publication_generation: u64, page: semio_framework::kernel::FixedCommandPage },
    GenericAssembly { cursor: semio_framework::kernel::CommandPageCursor, pages: semio_framework::kernel::CommandPageSet },
    ClosingAssembly { cursor: semio_framework::kernel::CommandPageCursor, pages: semio_framework::kernel::CommandPageSet },
    Generic { cursor: semio_framework::kernel::CommandPageCursor, command: crate::plugin_runtime::PluginCommandIngress },
}

struct RetainedCommandIngress {
    key: instance_lifetime::NativeCloseKey,
    state: CommandIngressOwner,
}

fn retire_command_ingress(state: CommandIngressOwner) -> Option<CommandIngressOwner> {
    match state {
        CommandIngressOwner::ReservedPresence { admission, .. } => {
            admission.cancel.cancel_now();
            None
        }
        CommandIngressOwner::Presence { .. } | CommandIngressOwner::PendingPresencePage { .. } => None,
        CommandIngressOwner::GenericAssembly { cursor, mut pages } | CommandIngressOwner::ClosingAssembly { cursor, mut pages } => {
            if pages.close_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES).0 {
                None
            } else {
                Some(CommandIngressOwner::ClosingAssembly { cursor, pages })
            }
        }
        CommandIngressOwner::Generic { cursor, command } => command.retire_step().map(|command| CommandIngressOwner::Generic { cursor, command }),
    }
}

pub(super) fn close_command_ingress_step(key: instance_lifetime::NativeCloseKey) -> Result<bool, semio_framework::Fault> {
    COMMAND_INGRESS.with(|ingress| {
        let mut ingress = ingress.try_borrow_mut().map_err(|_| reactor_close_fault("command ingress close authority busy"))?;
        let Some(index) = ingress.iter().position(|entry| entry.as_ref().is_some_and(|entry| entry.key == key)) else { return Ok(true) };
        let entry = ingress[index].take().expect("selected exact ingress owner");
        ingress[index] = retire_command_ingress(entry.state).map(|state| RetainedCommandIngress { key, state });
        Ok(!ingress.iter().any(|entry| entry.as_ref().is_some_and(|entry| entry.key == key)))
    })
}

crate::component_persistent_local! {
    static COMMAND_INGRESS: RefCell<[Option<RetainedCommandIngress>; 2]> = RefCell::new([None, None]);
}

fn native_close_key_fault(message: &'static str) -> semio_framework::Fault {
    reactor_close_fault(message)
}

fn native_close_key<PA: crate::app::PluginApp>(runtime: &crate::plugin_runtime::PluginRuntime<PA>, instance: u32) -> Result<instance_lifetime::NativeCloseKey, semio_framework::Fault> {
    let lifetimes = runtime.guest_lifetimes.try_borrow().map_err(|_| reactor_close_fault("lifecycle authority busy"))?;
    lifetimes.get(instance).filter(|slot| slot.cell.is_live()).and_then(|slot| slot.cell.owner()).map(|owner| owner.key()).ok_or_else(|| reactor_close_fault("instance has no acknowledged live lifetime"))
}

const DIRTY_RENDER_CAPACITY: usize = 64;
const DIRTY_INTENT_INSTANCE_CAPACITY: usize = 64;
const DIRTY_INTENT_CAPACITY: usize = 64;

struct DirtyIntentBatch {
    instance: u32,
    intents: ui_contract::UiFixedList<ui_contract::UiIntent, DIRTY_INTENT_CAPACITY>,
}

struct DirtyPollOwners {
    surfaces: ui_contract::UiFixedList<(u32, ui_contract::SurfaceId), DIRTY_RENDER_CAPACITY>,
    intents: ui_contract::UiFixedList<DirtyIntentBatch, DIRTY_INTENT_INSTANCE_CAPACITY>,
}

impl DirtyPollOwners {
    fn new() -> Self {
        Self { surfaces: ui_contract::UiFixedList::default(), intents: ui_contract::UiFixedList::default() }
    }

    fn try_surface(&mut self, instance: u32, surface: ui_contract::SurfaceId) -> Result<(), ui_contract::SurfaceId> {
        if self.surfaces.iter().any(|queued| queued.0 == instance && queued.1 == surface) {
            return Ok(());
        }
        self.surfaces.try_push((instance, surface)).map_err(|(_, surface)| surface)
    }

    fn try_intent(&mut self, instance: u32, intent: ui_contract::UiIntent) -> Result<(), ui_contract::UiIntent> {
        if let Some(batch) = self.intents.iter_mut().find(|batch| batch.instance == instance) {
            return batch.intents.try_push(intent);
        }
        if self.intents.len() == DIRTY_INTENT_INSTANCE_CAPACITY {
            return Err(intent);
        }
        let mut intents = ui_contract::UiFixedList::default();
        intents.try_push(intent)?;
        let _ = self.intents.try_push(DirtyIntentBatch { instance, intents });
        Ok(())
    }
}

/// 🧠️ Repository-owned actor ABI entrypoint. The component-model wrapper above and the native
/// interpreter both call this exact kernel reducer, so WIT lifting is no longer the production
/// host's semantic authority.
pub async fn poll_kernel<PA: crate::app::PluginApp + 'static>(
    runtime: &crate::plugin_runtime::PluginRuntime<PA>,
    events: Vec<Event>,
    command_page: Option<(semio_framework::kernel::CommandPageCursor, semio_framework::kernel::FixedCommandPage)>,
    cold_pair_page: Option<semio_framework::kernel::ColdDocumentPairPage>,
    budget: semio_framework::kernel::Budget,
) -> Result<semio_framework::kernel::TurnResult, semio_framework::Fault> {
    poll_kernel_output(runtime, events, command_page, cold_pair_page, budget, |_| Ok(()), |result, ()| result).await
}

pub(super) async fn poll_kernel_output<PA: crate::app::PluginApp, T, Prepared>(
    runtime: &crate::plugin_runtime::PluginRuntime<PA>,
    events: Vec<Event>,
    command_page: Option<(semio_framework::kernel::CommandPageCursor, semio_framework::kernel::FixedCommandPage)>,
    cold_pair_page: Option<semio_framework::kernel::ColdDocumentPairPage>,
    budget: semio_framework::kernel::Budget,
    prepare: impl FnOnce(&semio_framework::kernel::TurnResult) -> Result<Prepared, semio_framework::Fault>,
    publish: impl FnOnce(semio_framework::kernel::TurnResult, Prepared) -> T,
) -> Result<T, semio_framework::Fault> {
    let started_us = semio_framework_job::default_now_us();
    let retryable_lifecycle = command_page.is_none() && cold_pair_page.is_none() && events.iter().all(|event| matches!(event, Event::InstanceOpen { .. } | Event::InstanceClose(_) | Event::InstanceLifecycleAck(_)));
    let mut dirty = DirtyPollOwners::new();
    let mut focus = None;
    for event in &events {
        let instance = match event {
            Event::InstanceOpen { request, .. } => Some(request.instance_id),
            Event::InstanceClose(request) => Some(request.lifetime.instance_id),
            Event::InstanceLifecycleAck(ack) => Some(match ack.receipt {
                ActorInstanceLifecycleReceipt::Captured { lifetime, .. } | ActorInstanceLifecycleReceipt::Accepted { lifetime, .. } | ActorInstanceLifecycleReceipt::Retired { lifetime, .. } => lifetime.instance_id,
            }),
            _ => None,
        };
        if let Some(instance) = instance {
            if focus.replace(instance).is_some() {
                return Err(reactor_close_fault("one lifecycle command is admitted per turn"));
            }
        }
    }
    for event in &events {
        match event {
            Event::InstanceClose(request) => {
                let mut lifetimes = runtime.guest_lifetimes.try_borrow_mut().map_err(|_| reactor_close_fault("lifecycle authority busy"))?;
                let slot = lifetimes.get_mut(request.lifetime.instance_id).ok_or_else(|| reactor_close_fault("close lifetime absent"))?;
                slot.cell.validate_close(*request).map_err(reactor_close_fault)?;
                slot.cell.owner_mut().ok_or_else(|| reactor_close_fault("close native owner absent"))?.request_close(*request, runtime)?;
            }
            Event::InstanceLifecycleAck(ack) => {
                let mut lifetimes = runtime.guest_lifetimes.try_borrow_mut().map_err(|_| reactor_close_fault("lifecycle authority busy"))?;
                lifetimes.get_mut(focus.expect("ACK focus")).ok_or_else(|| reactor_close_fault("ACK lifetime absent"))?.cell.stage_ack(*ack).map_err(reactor_close_fault)?;
            }
            _ => {}
        }
    }
    if focus.is_none() {
        focus = runtime.guest_lifetimes.borrow_mut().next_work();
    }
    if let Some(instance) = focus {
        let mut lifetimes = runtime.guest_lifetimes.borrow_mut();
        if let Some(slot) = lifetimes.get_mut(instance) {
            if let Some(admission) = slot.cell.owner_mut().map(|owner| owner.advance_admission(runtime)).transpose()?.flatten() {
                slot.cell.record_close_admission(admission.0, admission.1).map_err(reactor_close_fault)?;
            }
        }
    }
    let mut close_instances: Vec<u32> = events.iter().filter_map(|event| if let Event::InstanceClose(request) = event { Some(request.lifetime.instance_id) } else { None }).collect();
    let _ = step_reactor_close()?;
    let _ = COLD_PAIR_INGRESS.with(|ingress| ingress.borrow_mut().advance_close_one());
    PATCHES.with(|patches| {
        patches.close_step();
    });
    let _ = semio_framework_ui_runtime::close_surface_reconcile_handback_one().map_err(reactor_close_fault)?;
    let _ = ui_contract::close_ui_document_page_one();
    let _ = ui_contract::close_ui_patch_owner_one();
    let _ = ui_contract::close_ui_value_page_one();
    let _ = semio_framework::kernel::close_ui_turn_patch_owner_one();
    let _ = semio_framework::kernel::close_ui_turn_patch_transport_one();
    let _ = crate::app::close_table_rows_view_one();
    with_pending_patches(|pending| pending.borrow_mut().advance_rejection(|surface, generation| PATCHES.with(|patches| patches.mark_rejected(surface, generation))));
    with_pending_patches(|pending| pending.borrow_mut().close_step()).map_err(reactor_close_fault)?;
    let close_cleanup_work = crate::plugin_runtime::plugin_step_close_cleanup(runtime)?;
    let _ = crate::plugin_runtime::plugin_step_live_cleanup(runtime)?;
    if let Some(surface) = PATCHES.with(patches::PatchTracker::take_deferred_ready) {
        if let Some(instance) = parse_surface_instance(surface.as_ref()) {
            dirty.try_surface(instance, surface).map_err(|_| reactor_close_fault("fixed dirty surface authority saturated"))?;
        }
    }
    for event in events {
        match event {
            Event::InstanceOpen { request, app_id, actor, quotas, .. } => {
                let instance = request.instance_id;
                let fresh = runtime.guest_lifetimes.borrow_mut().admit(request).map_err(reactor_close_fault)?;
                if fresh {
                    if let Err(error) = INSTANCE_METADATA.with(|metadata| metadata.borrow_mut().insert(instance, app_id.0.clone(), quotas)) {
                        runtime.guest_lifetimes.borrow_mut().remove_uncreated(instance).map_err(reactor_close_fault)?;
                        return Err(error);
                    }
                    if let Err(error) = crate::plugin_runtime::plugin_open_actor_instance(runtime, request, &app_id.0, actor).await {
                        INSTANCE_METADATA.with(|metadata| drop(metadata.borrow_mut().remove(instance)));
                        runtime.guest_lifetimes.borrow_mut().remove_uncreated(instance).map_err(reactor_close_fault)?;
                        return Err(error);
                    }
                }
            }
            Event::InstanceClose(_) | Event::InstanceLifecycleAck(_) => {}
            Event::CommandIngressPage { .. } => {
                return Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.command-page-event-bypass"), "command page must use poll_kernel's dedicated owner argument"));
            }
            // 🎯️ M1 (ticket 26/08/17 `design-unified.md`): decodes the pack-encoded
            // `ui_contract::UiIntent`, drops it if it targets a tree the user can no longer see (the
            // revision guard, at the reconciler that owns the revision — `PATCHES.revision`,
            // `ui_runtime::is_stale_intent` imported rather than reimplemented), and otherwise
            // batches it per instance for the dispatch pass below (mirrors `app_commands`'
            // batch-then-dispatch shape). Real dispatch replaces the prior packet's "decode-and-
            // mark-dirty" interim — see `📓️terra-sdk-wire-report.md`'s M1 section for the full route.
            Event::UiIntent { instance, intent } => {
                let numeric_instance = instance.0.parse::<u32>().map_err(|_| reactor_close_fault("invalid intent instance"))?;
                native_close_key(runtime, numeric_instance)?;
                if let Ok(intent_value) = store::pack_rt::decode_wire_value(&intent) {
                    if let Ok(intent) = serde_json::from_value::<ui_contract::UiIntent>(intent_value.into()) {
                        if parse_surface_instance(intent.surface.as_ref()) != Some(numeric_instance) {
                            return Err(reactor_close_fault("intent surface names another lifetime"));
                        }
                        let current_revision = PATCHES.with(|patches| patches.revision(&intent.surface.0));
                        if !is_stale_intent(intent.revision, current_revision, DEFAULT_REVISION_TOLERANCE) {
                            dirty
                                .try_intent(numeric_instance, intent)
                                .map_err(|_| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.dirty-intent-capacity"), "fixed dirty intent authority is saturated"))?;
                        }
                    }
                }
            }
            Event::SurfaceVisible { surface } => {
                if let Some(instance) = parse_surface_instance(&surface) {
                    let surface =
                        ui_contract::SurfaceId::try_from(surface).map_err(|_| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.surface-capacity"), "surface id exceeds fixed text capacity"))?;
                    dirty.try_surface(instance, surface).map_err(|_| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.dirty-surface-capacity"), "fixed dirty surface authority is saturated"))?;
                }
            }
            Event::SurfaceHidden { .. } | Event::SurfaceResized { .. } => {}
            Event::PatchAck { receipt, surface, revision } => {
                if !live_patch_receipt(runtime, receipt) {
                    continue;
                }
                with_pending_patches(|pending| pending.borrow_mut().apply_issued_ack(receipt, &surface, revision, semio_framework_ui_runtime::SURFACE_RECONCILE_PAGE_BYTES, |ack| PATCHES.with(|patches| patches.mark_published_ack(ack))))
                    .map_err(|reason| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.patch-ack-authority"), reason))?;
            }
            Event::PatchRejected { receipt, surface, revision, .. } => {
                if !live_patch_receipt(runtime, receipt) {
                    continue;
                }
                with_pending_patches(|pending| pending.borrow_mut().apply_issued_rejection(receipt, &surface, revision, |generation| PATCHES.with(|patches| patches.mark_rejected(&surface, generation))));
            }
            Event::Completed { req, result } => {
                REGISTRY.with(|registry| registry.resolve(req, crate::host::outcome_to_result(result)));
            }
            Event::HttpChunk { req, bytes, done } => {
                // 🐛️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (sdk-async): used to discard every
                // non-final chunk outright (`if done { resolve(req, Ok(bytes)) }` — every earlier
                // `bytes` was simply dropped on the floor, silent data loss for any multi-chunk
                // response). `append_chunk` accumulates instead; `cap` is the owning instance's
                // `QuotaSchema.message_bytes` (default 64 MiB when unset/unknown — matches
                // `instance_task_quota`'s own `unwrap_or` fallback idiom above).
                REGISTRY.with(|registry| {
                    let cap = registry.instance_of(req).and_then(|instance| INSTANCE_METADATA.with(|metadata| metadata.borrow().get(instance).and_then(|entry| entry.quota.message_bytes))).unwrap_or(64 * 1024 * 1024) as usize;
                    registry.append_chunk(req, bytes, done, cap);
                });
            }
            Event::JobProgress { job, .. } => {
                if let Some(binding) = JOB_RENDER_BINDINGS.with(|bindings| bindings.borrow().accepted(job)) {
                    let surface = ui_contract::UiText::try_format(format_args!("{}:window", binding.instance))
                        .map(ui_contract::SurfaceId)
                        .ok_or_else(|| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.surface-capacity"), "surface id exceeds fixed text capacity"))?;
                    dirty
                        .try_surface(binding.instance, surface)
                        .map_err(|_| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.dirty-surface-capacity"), "fixed dirty surface authority is saturated"))?;
                }
            }
            Event::JobCompleted { job, result } => {
                // 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (J1, design-abi.md §4): a job spawned
                // through `host::jobs::spawn` (`🌐host/🦀️.rs`) allocates its `job` id from
                // THE SAME `RequestRegistry` counter as every other awaitable `host::*` call — the
                // `Effect::SpawnJob{job, ..}` this actor emitted carried `job == req.0` — so
                // `Event::JobCompleted{job, result}` resolves the identical parked `RequestFuture`
                // an `Event::Completed{req, result}` would, closing the "no `req`-per-job
                // correlation table yet" gap `📓️terra-M5-report.md` §4 named (no separate table
                // needed: the request id already IS the job id).
                if let Some(binding) = JOB_RENDER_BINDINGS.with(|bindings| bindings.borrow_mut().complete(job)) {
                    let surface = ui_contract::UiText::try_format(format_args!("{}:window", binding.instance))
                        .map(ui_contract::SurfaceId)
                        .ok_or_else(|| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.surface-capacity"), "surface id exceeds fixed text capacity"))?;
                    dirty
                        .try_surface(binding.instance, surface)
                        .map_err(|_| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.dirty-surface-capacity"), "fixed dirty surface authority is saturated"))?;
                }
                REGISTRY.with(|registry| registry.resolve(semio_framework::kernel::RequestId(job), crate::host::outcome_to_result(result)));
            }
            Event::Message { source: MessageEndpoint::Shell { instance }, payload } => {
                if let Some(token) = crate::app::TypedOperationResultPage::renderer_ack_token(&payload) {
                    if instance.0.parse::<u32>().ok() == Some(token.receiver) {
                        let _ = crate::plugin_runtime::plugin_acknowledge_typed_operation_result(runtime, token).await?;
                    }
                }
            }
            Event::Message { .. } => {}
            Event::Timer { id } => {
                ARMED_TIMERS.with(|timers| {
                    timers.borrow_mut().remove(id);
                });
                #[cfg(test)]
                TEST_FUTURE_EXECUTOR.with(|executor| executor.wake(id));
            }
            Event::Wake => {}
            Event::Request { .. } => {}
            Event::Activate { .. } | Event::SuspendRequest | Event::CapabilityChanged { .. } | Event::QuotaChanged { .. } => {}
        }
    }

    let mut effects: Vec<Effect> = Vec::new();
    let mut cold_pair_ingress = semio_framework::kernel::ColdPairIngressStatus::Idle;
    if let Some(page) = cold_pair_page {
        let lifetime = page.header.lifetime;
        let transfer_generation = page.header.transfer_generation;
        let terminal_cursor = page.header.cursor(page.header.page_count.saturating_sub(1));
        let live = native_close_key(runtime, lifetime.instance_id).ok().filter(|key| key.lifetime() == lifetime).map(|key| key.lifetime());
        cold_pair_ingress = COLD_PAIR_INGRESS.with(|ingress| ingress.borrow_mut().accept_page(page, live));
        if matches!(cold_pair_ingress, semio_framework::kernel::ColdPairIngressStatus::Loading(_)) {
            let live = native_close_key(runtime, lifetime.instance_id).ok().filter(|key| key.lifetime() == lifetime).map(|key| key.lifetime());
            let load = COLD_PAIR_INGRESS.with(|ingress| ingress.borrow_mut().begin_load(lifetime, transfer_generation, live));
            if let Some(load) = load {
                let result = crate::plugin_runtime::plugin_load_document_pack(runtime, lifetime.instance_id, load.files()).await.map_err(|fault| dsl::encode_fault_bytes(&fault));
                let live = native_close_key(runtime, lifetime.instance_id).ok().filter(|key| key.lifetime() == lifetime).map(|key| key.lifetime());
                cold_pair_ingress = COLD_PAIR_INGRESS.with(|ingress| ingress.borrow_mut().finish_load(load, result, live));
            } else {
                cold_pair_ingress = semio_framework::kernel::ColdPairIngressStatus::Fault { cursor: terminal_cursor, fault: b"cold-pair.load-admission".to_vec() };
            }
        }
    }
    let mut command_ingress = semio_framework::kernel::CommandIngressStatus::Idle;
    let (mut retained, retained_slot, mut retained_key) = COMMAND_INGRESS.with(|ingress| {
        let mut ingress = ingress.borrow_mut();
        if ingress[0].is_some() {
            let entry = ingress[0].take().expect("retained command");
            (Some(entry.state), 0, Some(entry.key))
        } else {
            match ingress[1].take() {
                Some(entry) => (Some(entry.state), 1, Some(entry.key)),
                None => (None, 1, None),
            }
        }
    });
    if let Some(key) = retained_key {
        if native_close_key(runtime, key.instance()).ok() != Some(key) {
            close_instances.push(key.instance());
        }
    }
    retained = match retained.take() {
        Some(CommandIngressOwner::ReservedPresence { cursor, admission, .. }) if close_instances.contains(&cursor.instance) => {
            admission.cancel.cancel_now();
            command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-cancelled-by-close".to_vec() };
            None
        }
        Some(CommandIngressOwner::PendingPresencePage { cursor, .. }) if close_instances.contains(&cursor.instance) => {
            command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-cancelled-by-close".to_vec() };
            None
        }
        Some(CommandIngressOwner::Generic { cursor, command }) if close_instances.contains(&cursor.instance) => {
            command_ingress = semio_framework::kernel::CommandIngressStatus::CommandPending(cursor.clone());
            Some(CommandIngressOwner::Generic {
                cursor,
                command: command.cancel(semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.command-cancelled-by-close"), "command ingress was cancelled by instance close")),
            })
        }
        Some(CommandIngressOwner::GenericAssembly { cursor, pages }) if close_instances.contains(&cursor.instance) => {
            command_ingress = semio_framework::kernel::CommandIngressStatus::CommandPending(cursor.clone());
            Some(CommandIngressOwner::ClosingAssembly { cursor, pages })
        }
        owner => owner,
    };
    retained = match retained.take() {
        Some(CommandIngressOwner::ClosingAssembly { cursor, mut pages }) => {
            let (complete, _) = pages.close_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES);
            if complete {
                command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-cancelled-by-close".to_vec() };
                None
            } else {
                command_ingress = semio_framework::kernel::CommandIngressStatus::CommandPending(cursor.clone());
                Some(CommandIngressOwner::ClosingAssembly { cursor, pages })
            }
        }
        owner => owner,
    };
    retained = match retained.take() {
        Some(CommandIngressOwner::ReservedPresence { cursor, admission, page }) => {
            let now_ms = crate::host::now_ms().await;
            match crate::plugin_runtime::plugin_admit_reserved_presence(runtime, cursor.instance, admission, cursor.seq, if cursor.metadata & 0x100 != 0 { Some((cursor.metadata & 0xff) as u8) } else { None }, cursor.item_count, page, now_ms).await {
                Ok(publication_generation) => {
                    command_ingress = semio_framework::kernel::CommandIngressStatus::PageAccepted(cursor.clone());
                    Some(CommandIngressOwner::Presence { cursor, publication_generation })
                }
                Err((_fault, admission, page)) => {
                    command_ingress = semio_framework::kernel::CommandIngressStatus::CommandPending(cursor.clone());
                    Some(CommandIngressOwner::ReservedPresence { cursor, admission, page })
                }
            }
        }
        owner => owner,
    };
    retained = match retained.take() {
        Some(CommandIngressOwner::PendingPresencePage { cursor, publication_generation, page }) => match crate::plugin_runtime::plugin_push_reserved_presence_page(runtime, cursor.instance, publication_generation, cursor.page_index, page).await {
            Ok(()) => {
                command_ingress = semio_framework::kernel::CommandIngressStatus::PageAccepted(cursor.clone());
                Some(CommandIngressOwner::Presence { cursor, publication_generation })
            }
            Err((_fault, page)) => {
                command_ingress = semio_framework::kernel::CommandIngressStatus::CommandPending(cursor.clone());
                Some(CommandIngressOwner::PendingPresencePage { cursor, publication_generation, page })
            }
        },
        owner => owner,
    };
    if matches!(command_ingress, semio_framework::kernel::CommandIngressStatus::Idle) {
        if let Some(CommandIngressOwner::Presence { cursor, .. }) = retained.as_ref() {
            let cursor = cursor.clone();
            if close_instances.contains(&cursor.instance) {
                retained = None;
                command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-cancelled-by-close".to_vec() };
            } else {
                match crate::plugin_runtime::plugin_exchange(runtime, cursor.instance, None).await {
                    Ok(output) => {
                        let instance = cursor.instance;
                        if output.presence_terminal == Some(cursor.seq) {
                            retained = None;
                            command_ingress = match advance_command_cursor(cursor) {
                                Ok(terminal) => match output.presence_terminal_fault.as_ref() {
                                    Some(fault) => semio_framework::kernel::CommandIngressStatus::Fault { cursor: terminal, fault: fault.clone() },
                                    None => semio_framework::kernel::CommandIngressStatus::CommandComplete(terminal),
                                },
                                Err(cursor) => semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-page-index-exhausted".to_vec() },
                            };
                        } else if cursor.page_index.checked_add(1) == Some(cursor.page_count) {
                            command_ingress = match advance_command_cursor(cursor.clone()) {
                                Ok(pending) => semio_framework::kernel::CommandIngressStatus::CommandPending(pending),
                                Err(cursor) => semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-page-index-exhausted".to_vec() },
                            };
                        }
                        route_exchange_output(instance, output, &mut effects);
                    }
                    Err(fault) => {
                        command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: dsl::encode_fault_bytes(&fault) };
                        retained = None;
                    }
                }
            }
        }
    }
    if let Some(owner) = retained.take() {
        match owner {
            CommandIngressOwner::Generic { cursor, command } => match crate::plugin_runtime::plugin_exchange(runtime, cursor.instance, Some((cursor.seq, command))).await {
                Ok(mut output) => {
                    match advance_command_cursor(cursor.clone()) {
                        Ok(terminal) => {
                            if let Some((_, command)) = output.retry_command.take() {
                                retained = Some(CommandIngressOwner::Generic { cursor: cursor.clone(), command });
                                command_ingress = semio_framework::kernel::CommandIngressStatus::CommandPending(terminal);
                            } else if let Some(fault) = output.command_terminal_fault.as_ref() {
                                command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor: terminal, fault: fault.clone() };
                            } else {
                                command_ingress = semio_framework::kernel::CommandIngressStatus::CommandComplete(terminal);
                            }
                        }
                        Err(cursor) => command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-page-index-exhausted".to_vec() },
                    }
                    route_exchange_output(cursor.instance, output, &mut effects);
                }
                Err(fault) => command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: dsl::encode_fault_bytes(&fault) },
            },
            owner => retained = Some(owner),
        }
    }
    if let Some((cursor, page)) = command_page {
        if retained.is_none() {
            retained_key = native_close_key(runtime, cursor.instance).ok();
        }
        if !runtime.guest_lifetimes.borrow().get(cursor.instance).is_some_and(|slot| slot.cell.is_live())
            || close_instances.contains(&cursor.instance)
            || page.len() > semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES
            || cursor.page_count == 0
            || cursor.page_count as usize > semio_framework::kernel::COMMAND_MAXIMUM_PAGES
            || cursor.command_count == 0
            || cursor.command_count as usize > semio_framework::kernel::COMMAND_BATCH_MAXIMUM_ITEMS
            || cursor.command_index >= cursor.command_count
            || cursor.page_index >= cursor.page_count
            || cursor.item_count as usize > semio_framework::kernel::COMMAND_BATCH_MAXIMUM_ITEMS
            || (cursor.kind == 28 && cursor.page_count != cursor.item_count.max(1))
            || (cursor.kind == 28 && ((cursor.item_count == 0) != page.is_empty()))
            || (cursor.kind != 28
                && (page.is_empty()
                    || cursor.item_count != 0
                    || cursor.metadata != 0
                    || (cursor.page_index == 0 && cursor.kind != page.as_slice()[0])
                    || (cursor.page_index.checked_add(1).is_some_and(|next| next < cursor.page_count) && page.len() != semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES)))
        {
            command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-page-invalid".to_vec() };
        } else if retained.as_ref().is_some_and(|owner| match owner {
            CommandIngressOwner::ReservedPresence { cursor: active, .. }
            | CommandIngressOwner::Presence { cursor: active, .. }
            | CommandIngressOwner::PendingPresencePage { cursor: active, .. }
            | CommandIngressOwner::GenericAssembly { cursor: active, .. }
            | CommandIngressOwner::ClosingAssembly { cursor: active, .. }
            | CommandIngressOwner::Generic { cursor: active, .. } => !same_command_cursor(active, &cursor),
        }) {
            command_ingress = semio_framework::kernel::CommandIngressStatus::Backpressure(cursor);
        } else if matches!(retained, Some(CommandIngressOwner::ReservedPresence { .. } | CommandIngressOwner::PendingPresencePage { .. })) {
            command_ingress = semio_framework::kernel::CommandIngressStatus::CommandPending(cursor);
        } else if cursor.kind == 28 {
            let own_color = if cursor.metadata & !0x1ff != 0 {
                command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor: cursor.clone(), fault: b"plugin.command-presence-metadata".to_vec() };
                None
            } else if cursor.metadata & 0x100 != 0 {
                Some((cursor.metadata & 0xff) as u8)
            } else {
                None
            };
            if !matches!(command_ingress, semio_framework::kernel::CommandIngressStatus::Fault { .. }) {
                if cursor.page_index == 0 && retained.is_none() {
                    match crate::plugin_runtime::plugin_reserve_presence_ingress(runtime, cursor.instance, cursor.seq).await {
                        Ok(admission) => {
                            let now_ms = crate::host::now_ms().await;
                            match crate::plugin_runtime::plugin_admit_reserved_presence(runtime, cursor.instance, admission, cursor.seq, own_color, cursor.item_count, page, now_ms).await {
                                Ok(publication_generation) => {
                                    retained = Some(CommandIngressOwner::Presence { cursor: cursor.clone(), publication_generation });
                                    command_ingress = semio_framework::kernel::CommandIngressStatus::PageAccepted(cursor);
                                }
                                Err((_fault, admission, page)) => {
                                    retained = Some(CommandIngressOwner::ReservedPresence { cursor: cursor.clone(), admission, page });
                                    command_ingress = semio_framework::kernel::CommandIngressStatus::CommandPending(cursor);
                                }
                            }
                        }
                        Err(_) => command_ingress = semio_framework::kernel::CommandIngressStatus::Backpressure(cursor),
                    }
                } else if let Some(publication_generation) = retained.as_ref().and_then(|owner| match owner {
                    CommandIngressOwner::Presence { publication_generation, .. } => Some(*publication_generation),
                    _ => None,
                }) {
                    match crate::plugin_runtime::plugin_push_reserved_presence_page(runtime, cursor.instance, publication_generation, cursor.page_index, page).await {
                        Ok(()) => {
                            if let Some(CommandIngressOwner::Presence { cursor: active, .. }) = retained.as_mut() {
                                active.page_index = cursor.page_index;
                            }
                            command_ingress = semio_framework::kernel::CommandIngressStatus::PageAccepted(cursor);
                        }
                        Err((_fault, page)) => {
                            retained = Some(CommandIngressOwner::PendingPresencePage { cursor: cursor.clone(), publication_generation, page });
                            command_ingress = semio_framework::kernel::CommandIngressStatus::CommandPending(cursor);
                        }
                    }
                }
            }
        } else if cursor.page_index == 0 && retained.is_none() {
            match semio_framework::kernel::CommandPageSet::try_new() {
                Ok(mut pages) => match pages.try_push(page) {
                    Err((fault, _page)) => command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: dsl::encode_fault_bytes(&fault) },
                    Ok(()) if cursor.page_count == 1 => match semio_framework::kernel::PagedCommand::try_from_pages(pages) {
                        Ok(command) => match crate::plugin_runtime::plugin_exchange(runtime, cursor.instance, Some((cursor.seq, crate::plugin_runtime::PluginCommandIngress::Encoded(command)))).await {
                            Ok(mut output) => {
                                if let Some((_, command)) = output.retry_command.take() {
                                    retained = Some(CommandIngressOwner::Generic { cursor: cursor.clone(), command });
                                    command_ingress = match advance_command_cursor(cursor.clone()) {
                                        Ok(pending) => semio_framework::kernel::CommandIngressStatus::CommandPending(pending),
                                        Err(cursor) => semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-page-index-exhausted".to_vec() },
                                    };
                                } else {
                                    command_ingress = terminal_command_ingress(cursor.clone(), output.command_terminal_fault.take());
                                }
                                route_exchange_output(cursor.instance, output, &mut effects);
                            }
                            Err(fault) => command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: dsl::encode_fault_bytes(&fault) },
                        },
                        Err((fault, _pages)) => command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: dsl::encode_fault_bytes(&fault) },
                    },
                    Ok(()) => {
                        retained = Some(CommandIngressOwner::GenericAssembly { cursor: cursor.clone(), pages });
                        command_ingress = semio_framework::kernel::CommandIngressStatus::PageAccepted(cursor);
                    }
                },
                Err(fault) => command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: dsl::encode_fault_bytes(&fault) },
            }
        } else if let Some(CommandIngressOwner::GenericAssembly { cursor: active, mut pages }) = retained.take() {
            if cursor.page_index as usize != pages.len() {
                command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-page-order".to_vec() };
            } else {
                match pages.try_push(page) {
                    Err((fault, _page)) => command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: dsl::encode_fault_bytes(&fault) },
                    Ok(()) if cursor.page_index.checked_add(1) == Some(cursor.page_count) => match semio_framework::kernel::PagedCommand::try_from_pages(pages) {
                        Ok(command) => match crate::plugin_runtime::plugin_exchange(runtime, cursor.instance, Some((cursor.seq, crate::plugin_runtime::PluginCommandIngress::Encoded(command)))).await {
                            Ok(mut output) => {
                                if let Some((_, command)) = output.retry_command.take() {
                                    retained = Some(CommandIngressOwner::Generic { cursor: cursor.clone(), command });
                                    command_ingress = match advance_command_cursor(cursor.clone()) {
                                        Ok(pending) => semio_framework::kernel::CommandIngressStatus::CommandPending(pending),
                                        Err(cursor) => semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-page-index-exhausted".to_vec() },
                                    };
                                } else {
                                    command_ingress = terminal_command_ingress(cursor.clone(), output.command_terminal_fault.take());
                                }
                                route_exchange_output(cursor.instance, output, &mut effects);
                            }
                            Err(fault) => command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: dsl::encode_fault_bytes(&fault) },
                        },
                        Err((fault, _pages)) => command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: dsl::encode_fault_bytes(&fault) },
                    },
                    Ok(()) => {
                        retained = Some(CommandIngressOwner::GenericAssembly { cursor: active, pages });
                        if let Some(CommandIngressOwner::GenericAssembly { cursor: active, .. }) = retained.as_mut() {
                            active.page_index = cursor.page_index;
                        }
                        command_ingress = semio_framework::kernel::CommandIngressStatus::PageAccepted(cursor);
                    }
                }
            }
        }
    }
    if let Some(retained) = retained {
        COMMAND_INGRESS.with(|ingress| {
            ingress.borrow_mut()[retained_slot] = Some(RetainedCommandIngress { key: retained_key.expect("admitted command retains its exact lifetime"), state: retained });
        });
    }
    // 🎯️ M1: surviving intents dispatch through the SAME `route_app_frame`/effects/events plumbing
    // as `app_commands` above, immediately after it (so a mutation an app command made this turn is
    // already visible to the intent's own dispatch) — via the NEW `plugin_dispatch_intents`, which
    // routes through the app's EXISTING typed command path (`PluginApp::handle_intent_frame` →
    // `ArtifactApp::command_from_intent` → `dispatch_typed_command_inner`), never a parallel path.
    // Each handled batch's surfaces feed the retained render set so the reply patch — the next `UiPatch`
    // revision bump — is produced in this SAME turn (design decision: no new reply channel).
    let intent_batches = std::mem::take(&mut dirty.intents);
    for DirtyIntentBatch { instance, intents } in intent_batches {
        native_close_key(runtime, instance)?;
        // 🚫️async: E5 executor bridge — `plugin_dispatch_intents` stays genuinely `async fn`; safe to
        // resolve synchronously here for the same reason as `plugin_exchange` above.
        match crate::plugin_runtime::plugin_dispatch_intents(runtime, instance, &intents).await {
            Ok(output) => {
                for frame_bytes in output.frames {
                    route_app_frame(instance, &frame_bytes, &mut effects);
                }
                for one in &output.effects {
                    if let Ok(effect) = decode_wire_effect(one) {
                        push_admitted_effect(&mut effects, instance, effect);
                    }
                }
                for one in &output.events {
                    if let Ok(event) = decode_wire_app_event(one) {
                        effects.push(Effect::PublishEvent { topic: event.kind, payload: store::pack_rt::encode_wire_value(&event.payload) });
                    }
                }
            }
            Err(fault) => effects.push(shell_fault_effect(instance, &fault)),
        }
        // 🌳️ Deduped per instance — several intents on the same surface this turn must not queue a
        // redundant re-render (the second `diff()` would return `None` anyway, but there is no reason
        // to pay for it).
        let mut surfaces: semio_framework_ui_contract::UiFixedList<semio_framework_ui_contract::UiText> = semio_framework_ui_contract::UiFixedList::default();
        for intent in &intents {
            if surfaces.iter().any(|surface| surface == &intent.surface.0) {
                continue;
            }
            if surfaces.try_push(intent.surface.0.clone()).is_err() {
                effects.push(shell_fault_effect(instance, &semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.surface-capacity"), "dirty render surface capacity exceeded")));
                break;
            }
        }
        for surface in surfaces {
            dirty
                .try_surface(instance, ui_contract::SurfaceId(surface))
                .map_err(|_| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.dirty-surface-capacity"), "fixed dirty surface authority is saturated"))?;
        }
    }

    let (continuation, typed_operation_work) = crate::plugin_runtime::plugin_continue_typed_operations(runtime).await?;
    if let Some((instance, output)) = continuation {
        route_exchange_output(instance, output, &mut effects);
    }

    // 👥️ M2 (ticket 26/08/17 `design-unified.md`): `now_ms` is read ONCE for both `record_peer`'s
    // expiry stamping below and `PRESENCE.expire` at the end of this turn — a single wall-clock
    // reading per poll, not one per presence update.
    let now_ms = u64::try_from(crate::host::now_ms().await).unwrap_or(0);

    for (instance, surface) in dirty.surfaces {
        if native_close_key(runtime, instance).is_err() {
            continue;
        }
        let body_key = surface_body_key(surface.as_ref()).to_owned();
        let mounted = match native_close_key(runtime, instance) {
            Ok(key) => PATCHES.with(|patches| patches.reserve_mounted(surface, key)),
            Err(_) => Err(surface),
        };
        match mounted {
            Ok(grant) => match crate::plugin_runtime::plugin_render(runtime, instance, &body_key, "{}").await {
                Ok(tree) => {
                    let _ = grant.commit_source(tree.root);
                }
                Err(fault) => {
                    grant.cancel();
                    effects.push(shell_fault_effect(instance, &fault));
                }
            },
            Err(surface) => PATCHES.with(|patches| {
                let _ = patches.defer(surface);
            }),
        }
        // 👥️ M2: drains this instance's render-plane presence outbox (`VcsArtifactApp::
        // pending_presence`, filled by `stamp_and_cache_interaction_ui` during the render just above)
        // into `PRESENCE` right after its render — the SAME turn that presented the tree also records
        // its presence. NEVER touches `PENDING_PATCHES`/the document store — the whole point of this
        // separate channel (see `PresenceHub`'s own doc: a mouse-move must never bump a revision).
        match crate::plugin_runtime::plugin_take_presence(runtime, instance).await {
            Ok(updates) => {
                for update in updates {
                    PRESENCE.with(|hub| {
                        let mut hub = hub.borrow_mut();
                        hub.record_own(update.surface.clone(), update.node_key.clone(), update.own, update.ttl_ms);
                        for peer in update.peers {
                            hub.record_peer(update.surface.clone(), update.node_key.clone(), peer, update.ttl_ms, now_ms);
                        }
                    });
                }
            }
            Err(fault) => effects.push(shell_fault_effect(instance, &fault)),
        }
    }
    let reconcile_work = PATCHES
        .with(|patches| -> Result<bool, &'static str> {
            let opportunities = reconcile_step_opportunities(budget.fuel);
            let deadline = std::time::Instant::now() + std::time::Duration::from_millis(u64::from(budget.deadline_ms));
            let mut more = patches.has_work();
            for opportunity in 0..opportunities {
                if !more {
                    break;
                }
                if opportunity > 0 && opportunity % 64 == 0 && std::time::Instant::now() >= deadline {
                    break;
                }
                more = patches.drive_one();
                let can_publish = with_pending_patches(|pending| pending.borrow().has_capacity());
                if can_publish {
                    if let Some((key, generation)) = patches.ready_patch_key()? {
                        let mut target = None;
                        if patches.take_ready_patch_into(key, generation, &mut target, semio_framework_ui_runtime::SURFACE_RECONCILE_PAGE_BYTES)? {
                            if let Some(patch) = target {
                                // 🩹️ `take_ready_patch_into` already committed this output to its closing
                                // lifecycle; there is no `return_ready_patch` any more, so losing this
                                // capacity race simply drops the extracted page instead of re-queueing it.
                                match with_pending_patches(|pending| pending.borrow_mut().push_reconcile(patch)) {
                                    Ok(()) => {}
                                    Err(_dropped) => {}
                                }
                            }
                        }
                    }
                }
            }
            Ok(more || patches.has_work() || with_pending_patches(|pending| pending.borrow().has_unpublished()))
        })
        .map_err(|reason| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.patch-reconcile-authority"), reason))?;
    if let Some((instance, message)) = PATCHES.with(patches::PatchTracker::take_render_fault) {
        effects.push(shell_fault_effect(instance, &semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.surface-render"), message)));
    }

    // 🚫️async: E5 executor bridge (× 2) — `LocalExecutor::{run_until_idle,has_ready}` stay
    // genuinely `async fn` (its own doc: "run_until_idle handles Pending without ever yielding
    // its own future" — matches `⚛️reactor/💼️jobs`'s identical use of this exact bridge).
    let more_work = REACTOR_EXECUTOR.with(|executor| executor.run_until_deadline(64, 256 * 1_024, std::time::Instant::now() + std::time::Duration::from_millis(8)));
    for effect in REGISTRY.with(|registry| registry.drain()) {
        push_admitted_effect(&mut effects, 0, effect);
    }

    // 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): resumed `AsyncTask` follow-
    // ups (and any replayed `task_restarts` from a `restore` before this turn) — AFTER
    // `run_until_idle` so a task that resolved just now is redispatched the SAME turn, not the
    // next one. A resume can itself spawn more tasks (`dispatch_emit` runs for real), so the
    // executor may have fresh ready work by the time this returns — folded into `more_work` below
    // rather than requiring a second `run_until_idle` pass this turn (the next `poll` picks it up).
    let resumes_remain = drain_task_resumes(runtime, &mut effects, 64);
    let more_work = more_work
        || close_cleanup_work
        || typed_operation_work
        || reconcile_work
        || resumes_remain
        || REACTOR_EXECUTOR.with(|executor| executor.has_pending())
        || COMMAND_INGRESS.with(|ingress| ingress.borrow().iter().any(Option::is_some))
        || runtime.guest_lifetimes.borrow().has_work();

    let lifecycle_receipt = focus.map(|instance| runtime.guest_lifetimes.borrow_mut().prepare_turn(instance)).transpose().map_err(reactor_close_fault)?.flatten();
    let mut ui_patches = semio_framework::kernel::UiTurnPatches::default();
    let mut ui_patch_receipt = None;
    let taken = with_pending_patches(|pending| pending.borrow_mut().take_one(semio_framework_ui_runtime::SURFACE_RECONCILE_PAGE_BYTES))
        .map_err(|reason| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.pending-patch-authority"), reason))?;
    if let Some(patch) = taken {
        let instance = parse_surface_instance(&patch.surface.0);
        match ui_patches.try_push_ui_patch(patch) {
            Ok(()) => ui_patch_receipt = instance.and_then(|instance| runtime.guest_lifetimes.borrow_mut().next_patch_receipt(instance)),
            Err(patch) => {
                with_pending_patches(|pending| pending.borrow_mut().hand_back_turn(patch)).expect("exact unpublished patch returns to its reserved slot");
            }
        }
    }
    // 👥️ M2: once per poll — expire ages-out peer marks, then flush drains every key touched since
    // the last flush into one coalesced `PresenceUpdate` each (free burst coalescing: a hover storm
    // between polls still costs exactly one update per `(surface, node_key)`).
    let presence = PRESENCE.with(|hub| {
        let mut hub = hub.borrow_mut();
        hub.expire(now_ms);
        hub.flush()
    });
    let status = if more_work { TurnStatus::MoreWork } else { TurnStatus::Idle };

    let mut result = semio_framework::kernel::TurnResult { ui_patches, effects, presence, next_wake: ARMED_TIMERS.with(|timers| timers.borrow().first()), status, fuel_used: 0, command_ingress, cold_pair_ingress, lifecycle_receipt, ui_patch_receipt };
    with_pending_patches(|pending| {
        let mut pending = pending.borrow_mut();
        let prepared = (|| {
            result.validate_ui_patch_receipt().map_err(reactor_close_fault)?;
            if let Some(receipt) = result.ui_patch_receipt {
                pending.stage_emission(receipt, result.ui_patches.iter().next().expect("paired patch owner")).map_err(reactor_close_fault)?;
            }
            let prepared = prepare(&result)?;
            if let Some(instance) = focus {
                runtime.guest_lifetimes.borrow_mut().finish_turn(instance, started_us).map_err(|reason| {
                    if reason == super::instance_lifetime::GUEST_LIFECYCLE_TURN_DEADLINE {
                        semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.reactor-turn-deadline"), reason).with_retryable(retryable_lifecycle)
                    } else {
                        reactor_close_fault(reason)
                    }
                })?;
            }
            Ok(prepared)
        })();
        match prepared {
            Err(fault) => {
                let returned = result.ui_patches.try_transfer_one(|patch| pending.hand_back_turn(patch));
                assert!(!matches!(returned, semio_framework::kernel::UiTurnPatchTransfer::Refused), "failed output retains its exact pending patch slot");
                Err(fault)
            }
            Ok(prepared) => {
                if result.ui_patch_receipt.is_some() {
                    pending.commit_emission();
                }
                Ok(publish(result, prepared))
            }
        }
    })
}

fn live_patch_receipt<PA: crate::app::PluginApp>(runtime: &crate::plugin_runtime::PluginRuntime<PA>, receipt: semio_framework::kernel::ActorUiPatchReceipt) -> bool {
    runtime.guest_lifetimes.borrow().get(receipt.lifetime.instance_id).is_some_and(|slot| slot.cell.is_live() && slot.cell.lifetime() == receipt.lifetime)
}

fn route_exchange_output(instance: u32, output: crate::plugin_runtime::PluginExchangeOutput, effects: &mut Vec<Effect>) {
    if let Some(page) = output.typed_operation_result.as_ref() {
        effects.push(Effect::SendMessage { target: MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId(instance.to_string()) }, payload: page.renderer_exchange_bytes() });
    }
    for frame_bytes in output.frames {
        route_app_frame(instance, &frame_bytes, effects);
    }
    for one in &output.effects {
        if let Ok(effect) = decode_wire_effect(one) {
            push_admitted_effect(effects, instance, effect);
        }
    }
    for one in &output.events {
        if let Ok(event) = decode_wire_app_event(one) {
            effects.push(Effect::PublishEvent { topic: event.kind, payload: store::pack_rt::encode_wire_value(&event.payload) });
        }
    }
}

fn same_command_cursor(left: &semio_framework::kernel::CommandPageCursor, right: &semio_framework::kernel::CommandPageCursor) -> bool {
    left.owner == right.owner
        && left.generation == right.generation
        && left.command_index == right.command_index
        && left.command_count == right.command_count
        && left.instance == right.instance
        && left.seq == right.seq
        && left.kind == right.kind
        && left.page_count == right.page_count
        && left.item_count == right.item_count
        && left.metadata == right.metadata
}

use super::pending::{parse_surface_instance, with_state as with_pending_patches};

fn surface_body_key(surface: &str) -> &str {
    surface.split_once(':').map(|(_, body_key)| body_key).unwrap_or(surface)
}

/// 🔀️ `AppFrame::UiPatch` → a real `kernel::UiPatch` passthrough into `PENDING_PATCHES` (the wire
/// frame is already `UiPatch`-shaped field-for-field — channel v12/A4 — so this is a decode, not a
/// render); `AppFrame::Effects`/`Events` no longer exist as frames (`poll` decodes
/// `plugin_exchange`'s `PluginExchangeOutput.effects`/`.events` directly instead — see there);
/// `AppFrame::UiSnapshotEnd` has no consumer yet in this wave (patches apply incrementally, no
/// snapshot-boundary bookkeeping); everything else → `Effect::SendMessage` to the shell, matching
/// design-abi.md §2's table verbatim.
fn route_app_frame(instance: u32, frame_bytes: &[u8], effects: &mut Vec<Effect>) {
    // 🚫️async: E5 executor bridge (× 3) — `protocol::{decode,encode}_app_frame` (`📡️spr/**`, out
    // of `path_scope`) and `store::pack_rt::decode_wire_value` stay genuinely `async fn`; safe to
    // resolve synchronously for the same reason as this file's other WIT-boundary bridges.
    let Ok(frame) = semio_framework::io::resolve_ready(protocol::decode_app_frame(frame_bytes)) else {
        return;
    };
    match frame {
        // 🎯️ `sdk-flip` (26/08/20): `protocol::AppFrame::UiPatch` still carries the PRE-flip shape
        // (`kind: String`, `ops` pack-encoding the old `kernel::PatchOp`) — its crate, `📡️spr/**`, is
        // FORBIDDEN to this packet, so the frame struct itself is untouched. `kind` is bound but
        // dropped (the new `UiPatch` has no such field); `ops` decodes into `UiPatchOp` on the
        // OPTIMISTIC assumption the sender re-encodes with the new op set too — genuinely stale
        // until whichever packet updates `📡️spr/🧵️channel` re-frames this variant to match (flagged
        // in `📓️terra-wit-flip-report.md`'s consumer inventory; not fixed here, out of `OWNS`).
        protocol::AppFrame::UiPatch { surface, kind: _, revision, base_revision, ops, .. } => {
            let Ok(ops_value) = store::pack_rt::decode_wire_value(&ops) else { return };
            let Ok(ops) = serde_json::from_value::<ui_contract::UiPatchOps>(ops_value.into()) else { return };
            let Ok(surface) = ui_contract::SurfaceId::try_from(surface) else { return };
            let patch = UiPatch { surface, base_revision: ui_contract::UiRevision(base_revision), revision: ui_contract::UiRevision(revision), ops };
            if let Err(patch) = with_pending_patches(|pending| pending.borrow_mut().push_external(patch)) {
                effects.push(Effect::SendMessage {
                    target: MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId(instance.to_string()) },
                    payload: format!("patch-capacity-refused:{}:{}", patch.surface.0, patch.revision.0).into_bytes(),
                });
            }
        }
        protocol::AppFrame::UiSnapshotEnd { .. } => {}
        other => {
            let payload = semio_framework::io::resolve_ready(protocol::encode_app_frame(&other));
            effects.push(Effect::SendMessage { target: MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId(instance.to_string()) }, payload });
        }
    }
}

/// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): drains `TASK_RESUMES` (bounded
/// to `max_rounds` entries — the SAME defensive-cap shape `run_until_idle` uses, so an endlessly
/// respawning follow-up chain cannot stall a turn forever) and routes each resolved task's outcome
/// back into the SAME instance's `dyn PluginApp` via `plugin_runtime::plugin_resume_task`: a
/// `Command` resume through the existing typed-command dispatch, an `Emit` resume through a
/// decode + `dispatch_emit`, and a `Fault` resume straight to the shell as a message. Either
/// dispatch path's frames are fed through the SAME `route_app_frame` every other frame this turn
/// goes through — one implementation, not two (the eventual `world actor-async` runner calls this
/// SAME function, which is why it is `pub`, not `pub(crate)`).
///
/// Returns whether entries remain queued (the round cap was hit) — folded into `poll`'s
/// `turn-status::more-work` so a saturated resume queue is never silently dropped.
pub fn drain_task_resumes<PA: crate::app::PluginApp>(runtime: &crate::plugin_runtime::PluginRuntime<PA>, effects: &mut Vec<Effect>, max_rounds: u32) -> bool {
    for _ in 0..max_rounds {
        let Some(resume) = TASK_RESUMES.with(|resumes| resumes.borrow_mut().pop()) else {
            return false;
        };
        if native_close_key(runtime, resume.instance).is_err() {
            TASK_RESUMES.with(|resumes| resumes.borrow_mut().push_admitted(resume));
            continue;
        }
        let input = match resume.outcome {
            TaskResumeOutcome::Fault(fault) => {
                effects.push(shell_fault_effect(resume.instance, &fault));
                continue;
            }
            TaskResumeOutcome::Command(bytes) => crate::plugin_runtime::TaskResumeInput::Command(bytes),
            TaskResumeOutcome::Emit { artifact_ops, config_ops, draft_ops } => crate::plugin_runtime::TaskResumeInput::Emit { artifact_ops, config_ops, draft_ops },
        };
        // 🚫️async: E5 executor bridge — `plugin_resume_task` stays genuinely `async fn`; see
        // `poll`'s `plugin_exchange` call for the same safety argument.
        let output = semio_framework::io::resolve_ready(crate::plugin_runtime::plugin_resume_task(runtime, resume.instance, &resume.meta, input));
        for frame_bytes in output.frames {
            route_app_frame(resume.instance, &frame_bytes, effects);
        }
        for one in &output.effects {
            if let Ok(effect) = decode_wire_effect(one) {
                push_admitted_effect(effects, resume.instance, effect);
            }
        }
        for one in &output.events {
            if let Ok(event) = decode_wire_app_event(one) {
                effects.push(Effect::PublishEvent { topic: event.kind, payload: store::pack_rt::encode_wire_value(&event.payload) });
            }
        }
    }
    !TASK_RESUMES.with(|resumes| resumes.borrow().is_empty())
}

// 🚫️async: E5 executor bridge — `store::pack_rt::decode_wire_value` is genuinely `async fn`
// (out of `path_scope`, `🏪️store/**`); `resolve_ready` is safe here for the same reason as
// `kernel_effect_to_wit`'s own `pack` helper above — `world actor` imports no `host-async`.
fn decode_wire_effect(bytes: &[u8]) -> Result<Effect, ()> {
    let value = store::pack_rt::decode_wire_value(bytes).map_err(|_| ())?;
    dsl::from_dsl_value(value).map_err(|_| ())
}

fn push_admitted_effect(effects: &mut Vec<Effect>, instance: u32, effect: Effect) {
    if let Effect::SetTimer { id, .. } = &effect {
        if ARMED_TIMERS.with(|timers| timers.borrow_mut().insert(instance, *id)).is_err() {
            let fault = semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.timer-capacity"), "fixed timer authority is saturated or collided");
            effects.push(shell_fault_effect(instance, &fault));
            return;
        }
    }
    if let Effect::SpawnJob { job, .. } = &effect {
        if JOB_RENDER_BINDINGS.with(|bindings| bindings.borrow_mut().bind(instance, *job)).is_err() {
            let fault = semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.job-render-binding-capacity"), "fixed job-to-surface render authority is saturated or collided");
            effects.push(shell_fault_effect(instance, &fault));
            return;
        }
    }
    effects.push(effect);
}

fn decode_wire_app_event(bytes: &[u8]) -> Result<semio_framework::kernel::AppEvent, ()> {
    let value = store::pack_rt::decode_wire_value(bytes).map_err(|_| ())?;
    dsl::from_dsl_value(value).map_err(|_| ())
}
