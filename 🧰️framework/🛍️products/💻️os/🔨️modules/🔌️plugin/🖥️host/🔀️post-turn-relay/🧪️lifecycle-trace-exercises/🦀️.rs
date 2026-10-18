//! 🧪️ Lifecycle trace exercises: the lifecycle trace and projection, the barrier, wait and probe session helpers and the abandoned, live, stale and capacity trace exercises. See [`super`].

use super::*;

//#region 🔀️PostTurnRelay
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct GuestRelayLifecycleTrace {
    #[serde(rename = "🪪️id")]
    pub(super) id: String,
    machine: String,
    pub(super) generation: u64,
    initial: String,
    events: Vec<String>,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct GuestRelayLifecycleProjection {
    pub(super) state: String,
    first_reason: Option<String>,
    pub(super) cancel_admissions: usize,
    release_opportunities: usize,
    caller_output: Option<String>,
}

fn guest_relay_lifecycle_barrier(pool: &WorkerPool) -> Result<(), String> {
    let (sender, receiver) = semio_framework_async::oneshot::channel();
    pool.submit_at(
        pool.now_ms().saturating_add(1),
        Lane::Maintenance,
        Box::new(move || {
            let _ = sender.send(());
        }),
    );
    semio_framework_async::block_on(receiver).map_err(|_| "relay lifecycle worker barrier closed".to_string())
}

fn guest_relay_lifecycle_wait(pool: &WorkerPool, mut ready: impl FnMut() -> bool, detail: &'static str) -> Result<(), String> {
    for _ in 0..256 {
        if ready() {
            return Ok(());
        }
        guest_relay_lifecycle_barrier(pool)?;
    }
    Err(detail.to_string())
}

fn guest_relay_lifecycle_probe_session(control: &Arc<GuestRelayLifecycleProbeControl>, generation: u64, terminal_output: Option<Vec<u8>>, remaining: usize, retained:semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_job_worker::WorkerJobSession<GuestRelayLifecycleProbeJob>, String> {
    let job = GuestRelayLifecycleProbeJob { control: Arc::clone(control), remaining, terminal_output, publication: None, closing: false };
    let params = semio_framework_job_worker_authority::BatchJobParams {
        operation: semio_framework_job::OperationId(generation),
        generation: semio_framework_job::Generation(generation),
        cancel: semio_framework_job::root_cancel_token(),
        config: semio_framework_job_worker_authority::BatchDriveConfig {
            retained,
            site: "plugin-host.relay-lifecycle",
            stage: semio_framework_job::InteractiveStage::UserVisibleSimStep,
            fuel_per_step: semio_framework_job::USER_VISIBLE_LANE_FUEL,
            step_budget_us: semio_framework_job::USER_VISIBLE_LANE_WALL_US,
        },
        now_us: semio_framework_job::default_now_us,
    };
    admit_guest_relay_session(job, params, retained).map_err(|mut rejected| {
        rejected.begin_close();
        control.wake();
        control.release_one();
        control.release_one();
        "relay lifecycle probe admission refused".to_string()
    })
}

fn exercise_abandoned_relay_lifecycle_trace(trace: &GuestRelayLifecycleTrace,wake_authority:GuestRelayWakeAuthority) -> Result<GuestRelayLifecycleProjection, String> {
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    let result = (|| {
        let registry = Arc::new(GuestRelayMountedRegistry::with_pool(pool.clone(),wake_authority.clone()));
        registry.next_generation.store(trace.generation, std::sync::atomic::Ordering::Release);
        let (index, generation) = registry.reserve().ok_or_else(|| "relay lifecycle slot unavailable".to_string())?;
        let control = Arc::new(GuestRelayLifecycleProbeControl::new());
        let session = guest_relay_lifecycle_probe_session(&control, generation, None, 2,wake_authority.drive_policy)?;
        registry.mount(index, generation, GuestRelayMountedOwner::LifecycleProbe(session));
        let mut first_reason = None;
        for event in &trace.events {
            if let Some(dropped) = event.strip_prefix("drop:").and_then(|value| value.parse::<u64>().ok()) {
                registry.detach(index, dropped);
                let slot = registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                if matches!(&*slot, GuestRelayMountedSlot::Mounted(session) if session.lifecycle == GuestRelayMountedLifecycle::DetachedForReap) {
                    first_reason.get_or_insert_with(|| "detached".to_string());
                }
            } else if event == "blocked" {
                guest_relay_lifecycle_wait(&pool, || control.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some(), "relay lifecycle reaper did not park on the exact close wake")?;
            } else if event == "wake" {
                control.wake();
                guest_relay_lifecycle_wait(&pool, || control.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some(), "relay lifecycle reaper did not park for its first release")?;
            } else if event == "release" {
                let released = control.releases.load(std::sync::atomic::Ordering::Acquire);
                control.release_one();
                guest_relay_lifecycle_wait(&pool, || control.releases.load(std::sync::atomic::Ordering::Acquire) > released, "relay lifecycle reaper did not consume its admitted release")?;
            }
        }
        guest_relay_lifecycle_wait(&pool, || matches!(&*registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner), GuestRelayMountedSlot::Empty), "relay lifecycle reaper did not reach Empty")?;
        Ok(GuestRelayLifecycleProjection {
            state: "Empty".to_string(),
            first_reason,
            cancel_admissions: control.close_admissions.load(std::sync::atomic::Ordering::Acquire),
            release_opportunities: control.releases.load(std::sync::atomic::Ordering::Acquire),
            caller_output: None,
        })
    })();
    let shutdown = pool.shutdown().map_err(|error| format!("worker shutdown failed: {error:?}"));
    result.and_then(|projection| shutdown.map(|()| projection))
}

fn exercise_live_relay_lifecycle_trace(trace: &GuestRelayLifecycleTrace,wake_authority:GuestRelayWakeAuthority) -> Result<GuestRelayLifecycleProjection, String> {
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    let result = (|| {
        let registry = Arc::new(GuestRelayMountedRegistry::with_pool(pool.clone(),wake_authority.clone()));
        registry.next_generation.store(trace.generation, std::sync::atomic::Ordering::Release);
        let (index, generation) = registry.reserve().ok_or_else(|| "relay lifecycle slot unavailable".to_string())?;
        let mut first_reason = None;
        let mut releases = 0;
        let mut caller_output = None;
        for event in &trace.events {
            if let Some(output) = event.strip_prefix("terminal:") {
                let control = Arc::new(GuestRelayLifecycleProbeControl::new());
                control.wake();
                let session = guest_relay_lifecycle_probe_session(&control, generation, Some(output.as_bytes().to_vec()), 0,wake_authority.drive_policy)?;
                registry.mount(index, generation, GuestRelayMountedOwner::LifecycleProbe(session));
                for _ in 0..256 {
                    let _ = registry.pump(index, generation, std::task::Waker::noop());
                    let slot = registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    if matches!(&*slot, GuestRelayMountedSlot::Mounted(session) if session.lifecycle == GuestRelayMountedLifecycle::DrainingForCaller) {
                        break;
                    }
                }
                let slot = registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                if !matches!(&*slot, GuestRelayMountedSlot::Mounted(session) if session.lifecycle == GuestRelayMountedLifecycle::DrainingForCaller) {
                    return Err("relay lifecycle production terminal did not reach caller drain".to_string());
                }
                first_reason.get_or_insert_with(|| "caller".to_string());
            } else if event == "reap-other" {
                let control = Arc::new(GuestRelayLifecycleProbeControl::new());
                let (other_index, other_generation) = registry.reserve().ok_or_else(|| "relay lifecycle competing slot unavailable".to_string())?;
                let session = guest_relay_lifecycle_probe_session(&control, other_generation, None, 2,wake_authority.drive_policy)?;
                registry.mount(other_index, other_generation, GuestRelayMountedOwner::LifecycleProbe(session));
                registry.detach(other_index, other_generation);
                guest_relay_lifecycle_wait(&pool, || control.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some(), "relay lifecycle competing reaper did not park")?;
                control.wake();
                control.release_one();
                control.release_one();
                guest_relay_lifecycle_wait(&pool, || matches!(&*registry.slots[other_index].lock().unwrap_or_else(std::sync::PoisonError::into_inner), GuestRelayMountedSlot::Empty), "relay lifecycle competing detached owner was not reclaimed")?;
                let slot = registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                if !matches!(&*slot, GuestRelayMountedSlot::Mounted(session) if session.lifecycle == GuestRelayMountedLifecycle::DrainingForCaller) {
                    return Err("relay lifecycle reaper stole the live caller's exact owner".to_string());
                }
            } else if event == "caller-release" {
                for _ in 0..256 {
                    match registry.pump(index, generation, std::task::Waker::noop()) {
                        std::task::Poll::Ready(Ok(bytes)) => {
                            caller_output = Some(String::from_utf8(bytes).map_err(|_| "relay lifecycle caller output is not UTF-8".to_string())?);
                            releases += 1;
                            break;
                        }
                        std::task::Poll::Ready(Err(error)) => return Err(error.to_string()),
                        std::task::Poll::Pending => {}
                    }
                }
                if caller_output.is_none() {
                    return Err("relay lifecycle caller did not receive its exact terminal output".to_string());
                }
            }
        }
        let state = if matches!(&*registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner), GuestRelayMountedSlot::Empty) { "Empty" } else { "DrainingForCaller" };
        Ok(GuestRelayLifecycleProjection { state: state.to_string(), first_reason, cancel_admissions: 0, release_opportunities: releases, caller_output })
    })();
    let shutdown = pool.shutdown().map_err(|error| format!("worker shutdown failed: {error:?}"));
    result.and_then(|projection| shutdown.map(|()| projection))
}

fn exercise_stale_relay_lifecycle_trace(trace: &GuestRelayLifecycleTrace,wake_authority:GuestRelayWakeAuthority) -> Result<GuestRelayLifecycleProjection, String> {
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    let result = (|| {
        let registry = Arc::new(GuestRelayMountedRegistry::with_pool(pool.clone(),wake_authority.clone()));
        registry.next_generation.store(trace.generation, std::sync::atomic::Ordering::Release);
        let (index, generation) = registry.reserve().ok_or_else(|| "relay lifecycle slot unavailable".to_string())?;
        registry.mount(index, generation, GuestRelayMountedOwner::Empty);
        for event in &trace.events {
            if let Some(dropped) = event.strip_prefix("drop:").and_then(|value| value.parse::<u64>().ok()) {
                registry.detach(index, dropped);
            }
        }
        let state = {
            let slot = registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if matches!(&*slot, GuestRelayMountedSlot::Mounted(session) if session.generation == generation && session.lifecycle == GuestRelayMountedLifecycle::Running) {
                "Running"
            } else {
                return Err("relay lifecycle stale generation changed the live owner".to_string());
            }
        };
        *registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner) = GuestRelayMountedSlot::Empty;
        Ok(GuestRelayLifecycleProjection { state: state.to_string(), first_reason: None, cancel_admissions: 0, release_opportunities: 0, caller_output: None })
    })();
    let shutdown = pool.shutdown().map_err(|error| format!("worker shutdown failed: {error:?}"));
    result.and_then(|projection| shutdown.map(|()| projection))
}

fn exercise_capacity_relay_lifecycle_trace(trace: &GuestRelayLifecycleTrace,wake_authority:GuestRelayWakeAuthority) -> Result<GuestRelayLifecycleProjection, String> {
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    let registry = GuestRelayMountedRegistry::with_pool(pool.clone(),wake_authority.clone());
    registry.next_generation.store(trace.generation, std::sync::atomic::Ordering::Release);
    let mut reserved = Vec::new();
    let mut state = "Empty";
    for event in &trace.events {
        let Some(requested) = event.strip_prefix("reserve:").and_then(|value| value.parse::<usize>().ok()) else { continue };
        for _ in 0..requested {
            match registry.reserve() {
                Some(owner) => {
                    reserved.push(owner);
                    state = "ReservedFull";
                }
                None => state = "CapacityRefused",
            }
        }
    }
    for (index, _) in reserved {
        *registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner) = GuestRelayMountedSlot::Empty;
    }
    pool.shutdown().map_err(|error| format!("worker shutdown failed: {error:?}"))?;
    Ok(GuestRelayLifecycleProjection { state: state.to_string(), first_reason: None, cancel_admissions: 0, release_opportunities: 0, caller_output: None })
}

/// 🧪️ Drives one schema-owned lifecycle trace through the production replay or mounted-relay transition machinery.
#[doc(hidden)]
pub fn exercise_relay_lifecycle_trace(trace_json: &str,wake_authority:GuestRelayWakeAuthority) -> Result<String, String> {
    let trace: GuestRelayLifecycleTrace = serde_json::from_str(trace_json).map_err(|error| format!("invalid relay lifecycle trace: {error}"))?;
    let projection = match (trace.id.as_str(), trace.machine.as_str(), trace.initial.as_str()) {
        ("replay-first-fault-wins", "replay", "CaptureKind") => {
            let actual = semio_framework_plugin_host_shard::exercise_replay_lifecycle_trace(&trace.events)?;
            GuestRelayLifecycleProjection { state: actual.state.to_string(), first_reason: actual.first_reason, cancel_admissions: 0, release_opportunities: actual.release_opportunities, caller_output: None }
        }
        ("relay-abandoned-blocked-wake", "relay", "Running") => exercise_abandoned_relay_lifecycle_trace(&trace,wake_authority.clone())?,
        ("relay-live-terminal-caller-output", "relay", "Running") => exercise_live_relay_lifecycle_trace(&trace,wake_authority.clone())?,
        ("relay-stale-generation-refused", "relay", "Running") => exercise_stale_relay_lifecycle_trace(&trace,wake_authority.clone())?,
        ("relay-max-plus-one-refused", "relay", "Empty") => exercise_capacity_relay_lifecycle_trace(&trace,wake_authority.clone())?,
        _ => return Err(format!("unsupported relay lifecycle trace {:?}", trace.id)),
    };
    serde_json::to_string(&projection).map_err(|error| format!("relay lifecycle projection failed: {error}"))
}
//#endregion 🔀️PostTurnRelay
