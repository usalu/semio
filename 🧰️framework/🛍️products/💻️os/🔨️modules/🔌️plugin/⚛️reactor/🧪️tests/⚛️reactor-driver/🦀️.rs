use super::*;

pub(crate) fn queue_external_patch(patch: UiPatch) {
    pending::with_state(|pending| pending.borrow_mut().push_external(patch)).unwrap();
}

pub(crate) fn patch_receipt_is_issued(receipt: ActorUiPatchReceipt) -> bool {
    pending::with_state(|pending| pending.borrow().receipt_is_issued(receipt))
}

pub(crate) async fn poll_with_patch_output_fault<PA: crate::app::PluginApp>(
    runtime: &crate::plugin_runtime::PluginRuntime<PA>,
    events: Vec<Event>,
    budget: semio_framework::kernel::Budget,
    late_clock: bool,
) -> (Result<(), semio_framework::Fault>, Option<ActorUiPatchReceipt>) {
    let mut receipt = None;
    let result = turn::poll_kernel_output(
        runtime,
        events,
        None,
        None,
        budget,
        |result| {
            receipt = result.ui_patch_receipt;
            if late_clock {
                turn::charge_turn_execution_us(semio_framework_trace::GUEST_LIFECYCLE_TURN_CEILING_US);
                Ok(())
            } else {
                Err(reactor_close_fault("injected output conversion failure"))
            }
        },
        |_, ()| (),
    )
    .await;
    (result, receipt)
}

pub(crate) async fn poll_with_output_failure<PA: crate::app::PluginApp>(runtime: &crate::plugin_runtime::PluginRuntime<PA>, events: Vec<Event>, budget: semio_framework::kernel::Budget) -> Result<(), semio_framework::Fault> {
    turn::poll_kernel_output(runtime, events, None, None, budget, |_| Err(reactor_close_fault("injected output conversion failure")), |_, ()| ()).await
}

/// ▶️ The exact `run_until_idle` call `poll` makes after routing events, exposed directly.
pub(crate) async fn run_until_idle(max_iterations: u32) -> bool {
    TASK_EXECUTOR.with(|executor| executor.run_until_idle(max_iterations))
}

/// ✅️ The exact `REGISTRY::resolve` call `poll`'s `Event::Completed` arm makes, exposed
/// directly — the native stand-in for "an injected `Event::Completed`".
pub(crate) async fn resolve_request(id: u64, result: Result<Vec<u8>, semio_framework::Fault>) {
    REGISTRY.with(|registry| registry.resolve(semio_framework::kernel::RequestId(id), result));
}

/// 📬️ Pops one `TASK_RESUMES` entry, erased to `plugin_runtime::TaskResumeInput` (a `Fault`
/// resolution surfaces as `Err` here instead — `drain_task_resumes` frames that straight to
/// the shell without ever reaching `plugin_runtime`, so a test asserting on it never needs
/// `plugin_resume_task` at all).
pub(crate) async fn pop_task_resume() -> Option<(u32, crate::app::ActionMeta, Result<crate::plugin_runtime::TaskResumeInput, semio_framework::Fault>)> {
    TASK_RESUMES.with(|resumes| resumes.borrow_mut().pop()).map(|resume| {
        let input = match resume.outcome {
            TaskResumeOutcome::Command(bytes) => Ok(crate::plugin_runtime::TaskResumeInput::Command(bytes)),
            #[cfg(test)]
            TaskResumeOutcome::Emit { artifact_ops, config_ops, draft_ops } => Ok(crate::plugin_runtime::TaskResumeInput::Emit { artifact_ops, config_ops, draft_ops }),
            #[cfg(test)]
            TaskResumeOutcome::Fault(fault) => Err(fault),
        };
        (resume.instance, resume.meta, input)
    })
}

pub(crate) async fn task_count_for_instance(instance: u32) -> usize {
    TASK_RECORDS.with(|records| records.borrow().count_instance(instance))
}

pub(crate) async fn task_key_is_live(instance: u32, key: &str) -> bool {
    TASK_RECORDS.with(|records| records.borrow().find_key(instance, key).is_some())
}

pub(crate) async fn set_instance_quota(instance: u32, outstanding_requests: u64) {
    INSTANCE_METADATA.with(|metadata| {
        let mut metadata = metadata.borrow_mut();
        if let Some(entry) = metadata.slots.get_mut(InstanceMetadataRegistry::index(instance)).filter(|entry| entry.instance == instance) {
            entry.quota.outstanding_requests = Some(outstanding_requests);
        } else {
            let _ = metadata.insert(instance, "test".into(), semio_framework::kernel::QuotaSchema { outstanding_requests: Some(outstanding_requests), ..Default::default() });
        }
    });
}

pub(crate) async fn pending_request_count() -> usize {
    REGISTRY.with(|registry| registry.pending_ids().len())
}

/// 🔂️ Polls every task `instance` owns exactly once, whether or not it is on the ready queue —
/// the executor's own `poll_one`, which is what a real turn does for a task whose waker has
/// fired. Needed because retiring a request out from under a LIVE task deliberately issues no
/// wake (`RequestRegistry::begin_cancel_instance`), so `run_until_idle` alone would never visit
/// the parked task that must observe its retirement. Answers how many tasks completed on it.
pub(crate) async fn poll_instance_tasks_once(instance: u32) -> usize {
    let ids: Vec<_> = (0..REACTOR_TASK_SLOTS).filter_map(|index| TASK_RECORDS.with(|records| records.borrow().entry_at(index).and_then(|(id, record)| (record.instance == instance).then_some(id)))).collect();
    let mut completed = 0;
    for id in ids {
        if TASK_EXECUTOR.with(|executor| executor.poll_one(id)) == executor::TaskPoll::Complete {
            TASK_RECORDS.with(|records| drop(records.borrow_mut().remove(id)));
            completed += 1;
        }
    }
    completed
}

/// 🚫️ The exact `RequestRegistry::cancel_instance` call `poll`'s `Event::InstanceClose` arm
/// makes right after `cancel_instance_tasks`, exposed directly for a test to run the SAME
/// two-step sequence natively.
pub(crate) async fn cancel_instance_registry_requests(instance: u32) -> usize {
    REGISTRY.with(|registry| {
        let before = registry.pending_ids().len();
        let mut cursor = registry.begin_cancel_instance(instance);
        while registry.cancel_instance_step(&mut cursor) != requests::RequestCloseStep::Complete {}
        before.saturating_sub(registry.pending_ids().len())
    })
}

//#region 🔖️M2PresenceHooks
/// 🎯️ M1: the exact `PATCHES.revision` read `poll`'s intent-batching loop guards every
/// `UiIntent` against, exposed directly.
pub(crate) async fn patches_revision(surface: &str) -> ui_contract::UiRevision {
    PATCHES.with(|patches| patches.revision(surface))
}

/// 🩹️ Test-only completion driver over the same retained mounted authority.
pub(crate) async fn patches_diff(surface: &str, tree: semio_framework_ui_runtime::ComponentTree) -> Option<UiPatch> {
    PATCHES.with(|patches| {
        if !patches.can_begin(surface) {
            return None;
        }
        if let Err((surface, tree)) = patches.begin(surface.to_string(), tree) {
            let _ = patches.retain_unadmitted(surface, tree);
            return None;
        }
        for _ in 0..512 {
            patches.drive_one();
            if let Some(mut owner) = patches.take_ready_patch() {
                let mut payload = ui_contract::UiPendingPatch::default();
                let mut published = None;
                let bytes = owner.publish_into(&mut payload, &mut published, SurfaceReconcileReadyPatch::required_publish_bytes()).expect("test-owned publication grant");
                assert!(bytes > 0);
                let patch = payload.source_mut().expect("test-owned writable payload").take();
                while !owner.close_step_with_grant(1, 4096).expect("test ready close").complete {}
                let revision = patch.as_ref().map_or(0, |patch| patch.revision.0);
                let mut acknowledgement = None;
                if published.is_some()
                    && semio_framework_ui_runtime::SurfaceReconcilePublishedPatch::acknowledge_into(
                        &mut published,
                        &mut acknowledgement,
                        surface,
                        revision,
                        semio_framework_ui_runtime::SurfaceReconcilePublishedPatch::required_acknowledge_bytes(),
                    )
                    .expect("test-owned acknowledgement grant")
                {
                    let mut acknowledgement = acknowledgement.take().expect("acknowledged publication owner");
                    patches.mark_published_ack(&acknowledgement).expect("the tracker admits its own acknowledgement");
                    while !acknowledgement.close_step_with_grant(1, 4096).expect("test acknowledgement close").complete {}
                }
                if let Some(mut published) = published {
                    while !published.close_step_with_grant(1, 4096).expect("test published close").complete {}
                }
                for _ in 0..4_096 {
                    if patches.can_begin(surface) {
                        break;
                    }
                    patches.drive_one();
                    patches.close_step(1, 4096);
                }
                return patch;
            }
        }
        None
    })
}

/// 👥️ M2 (ticket 26/08/17 `design-unified.md`): the exact `PRESENCE.record_own` call `poll`'s
/// dirty-render loop makes for each drained `PresenceUpdate`, exposed directly.
pub(crate) async fn presence_record_own(surface: &str, node_key: &str, own: ui_contract::OwnPresence, ttl_ms: u32) {
    let surface = ui_contract::SurfaceId::try_from(surface).expect("bounded test surface");
    PRESENCE.with(|hub| hub.borrow_mut().record_own(surface, node_key.to_string(), own, ttl_ms));
}

/// 👥️ M2: the exact `PRESENCE.record_peer` call, exposed directly.
pub(crate) async fn presence_record_peer(surface: &str, node_key: &str, mark: ui_contract::PeerMark, ttl_ms: u32, now_ms: u64) {
    let surface = ui_contract::SurfaceId::try_from(surface).expect("bounded test surface");
    PRESENCE.with(|hub| hub.borrow_mut().record_peer(surface, node_key.to_string(), mark, ttl_ms, now_ms));
}

/// 👥️ M2: the exact `expire`-then-`flush` sequence `poll` runs once per turn, exposed directly.
pub(crate) async fn presence_expire_and_flush(now_ms: u64) -> Vec<ui_contract::PresenceUpdate> {
    PRESENCE.with(|hub| {
        let mut hub = hub.borrow_mut();
        hub.expire(now_ms);
        hub.flush()
    })
}
//#endregion 🔖️M2PresenceHooks
