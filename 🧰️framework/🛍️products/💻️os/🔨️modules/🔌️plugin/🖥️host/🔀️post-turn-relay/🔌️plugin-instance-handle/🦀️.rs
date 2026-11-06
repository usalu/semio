//! 🔌️ Selected plugin cold-job dispatch.

use super::*;

/// 🧬️ See `README.md`.
pub struct PluginInstanceHandle {
    pub actor: RuntimeActorId,
    pub runtime: Arc<GuestRuntimes>,
    pub(super) compiled:semio_plugin_host_guest_runtime::CompiledHandle,
    pub instance: Arc<Mutex<GuestInstanceSlot>>,
    pub(super) instance_gate: Arc<semio_framework_async_sync::Semaphore>,
    relay_registry: Arc<GuestRelayMountedRegistry>,
    next_job_id: std::sync::atomic::AtomicU64,
}

impl PluginInstanceHandle {
    pub async fn new(actor: RuntimeActorId, runtime: Arc<GuestRuntimes>, compiled:semio_plugin_host_guest_runtime::CompiledHandle,instance: GuestInstance,wake_authority:GuestRelayWakeAuthority) -> Self {
        Self {
            actor,
            runtime,
            compiled,
            instance: Arc::new(Mutex::new(GuestInstanceSlot::Available(instance))),
            instance_gate: Arc::new(semio_framework_async_sync::Semaphore::new(1)),
            relay_registry: Arc::new(GuestRelayMountedRegistry::new(wake_authority)),
            next_job_id: std::sync::atomic::AtomicU64::new(1),
        }
    }

    /// 🧵️ See `README.md`.
    async fn run_job_on_worker(&self, kind: &str, input: Vec<u8>, cancel: semio_framework_async::CancelToken) -> Result<Vec<u8>, PluginHostError> {
        self.run_job_on_registry(kind, input, cancel, &self.relay_registry).await
    }

    async fn run_job_on_registry(&self, kind: &str, input: Vec<u8>, cancel: semio_framework_async::CancelToken, registry: &Arc<GuestRelayMountedRegistry>) -> Result<Vec<u8>, PluginHostError> {
        if input.len() > semio_framework_job_payload::JOB_PAYLOAD_OPERATION_BYTES {
            return Err(PluginHostError::Plugin(format!("{kind} input exceeds the retained operation byte limit")));
        }
        if let Some(detail) = self.instance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).admission_fault() {
            return Err(PluginHostError::Plugin(format!("{kind} job rejected: {}", String::from_utf8_lossy(&detail))));
        }
        let job = self.next_job_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let pool = plugin_host_worker_pool();
        let config = semio_framework_job_worker_authority::BatchDriveConfig {
            retained: registry.wake_authority.drive_policy,
            site: "plugin-host.cold-relay",
            stage: semio_framework_job::InteractiveStage::UserVisibleSimStep,
            fuel_per_step: semio_framework_job::USER_VISIBLE_LANE_FUEL,
            step_budget_us: semio_framework_job::USER_VISIBLE_LANE_WALL_US,
        };
        let relay = GuestColdRelayJob::new(Arc::clone(&self.runtime), Arc::clone(&self.instance), Arc::clone(&self.instance_gate), pool.clone(), cancel, job, (kind.to_string(), input));
        let (index, mounted_generation) = registry.reserve().ok_or_else(|| PluginHostError::Plugin(format!("{kind} mounted relay registry is full")))?;
        let owner = GuestRelayMountedOwner::Admitting(GuestRelayAdmission::new(relay, config));
        registry.mount(index, mounted_generation, owner);
        GuestRelayMountedFuture { registry: Arc::clone(registry), index, generation: mounted_generation, complete: false }.await
    }

    /// 🌉️ See `README.md`.
    pub async fn io_run(&self, from: &str, into: &str, payload: Vec<u8>) -> Result<Vec<u8>, PluginHostError> {
        #[derive(ToValue)]
        struct IoRunInputWire<'a>{source:&'a str,target:&'a str,payload:semio_framework::io_schema::IoPayload}
        let payload_text = std::str::from_utf8(&payload).map_err(|error| PluginHostError::Json(error.to_string()))?;
        let io_payload: semio_framework::io_schema::IoPayload = semio_framework_pack_json::from_json_str(payload_text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| PluginHostError::Json(error.to_string()))?;
        let input = semio_framework_pack_json::to_json_string(&IoRunInputWire { source: from, target: into, payload: io_payload }).into_bytes();
        self.run_job_on_worker("semio.io-run", input, semio_framework_job::root_cancel_token()).await
    }

    /// 🔍️ See `README.md`.
    pub async fn io_sniff(&self, from: &str, into: &str, payload: &[u8]) -> Result<u8, PluginHostError> {
        #[derive(ToValue)]
        struct IoRunInputWire<'a>{source:&'a str,target:&'a str,payload:semio_framework::io_schema::IoPayload}
        let payload_text = std::str::from_utf8(payload).map_err(|error| PluginHostError::Json(error.to_string()))?;
        let io_payload: semio_framework::io_schema::IoPayload = semio_framework_pack_json::from_json_str(payload_text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| PluginHostError::Json(error.to_string()))?;
        let input = semio_framework_pack_json::to_json_string(&IoRunInputWire { source: from, target: into, payload: io_payload }).into_bytes();
        let result = self.run_job_on_worker("semio.io-sniff", input, semio_framework_job::root_cancel_token()).await?;
        result.first().copied().ok_or_else(|| PluginHostError::Plugin("semio.io-sniff job returned an empty result".to_string()))
    }

    /// 💡️ See `README.md`.
    pub async fn infer(&self, request: &[u8], cancel: &semio_framework_async::CancelToken, funding: &GuestRelayWakeAuthority) -> Result<Vec<u8>, PluginHostError> {
        let registry = Arc::new(GuestRelayMountedRegistry::new(funding.clone()));
        self.run_job_on_registry("semio.infer", request.to_vec(), cancel.child_now(), &registry).await
    }

    /// 🧬️ See `README.md`.
    pub async fn mutation_plan(&self, request: &[u8]) -> Result<Vec<u8>, PluginHostError> {
        self.run_job_on_worker("semio.mutation-plan", request.to_vec(), semio_framework_job::root_cancel_token()).await
    }

    /// 🔀️ See `README.md`.
    pub async fn migrate(&self, from: &str, to: &str, pack: Vec<u8>) -> Result<Vec<u8>, PluginHostError> {
        #[derive(serde::Serialize, ToValue)]
        struct MigrateInputWire<'a>{from:&'a str,to:&'a str,pack:Vec<u8>}
        let input = semio_framework_pack_json::to_json_string(&MigrateInputWire { from, to, pack }).into_bytes();
        self.run_job_on_worker("semio.migrate", input, semio_framework_job::root_cancel_token()).await
    }

    /// 🧩️ See `README.md`.
    pub async fn compose(&self, key_bytes: &[u8], sources_bytes: &[u8]) -> Result<Vec<u8>, PluginHostError> {
        #[derive(serde::Serialize, ToValue)]
        struct ComposeInput{key:Vec<u8>,sources:Vec<u8>}
        let input = semio_framework_pack_json::to_json_string(&ComposeInput { key: key_bytes.to_vec(), sources: sources_bytes.to_vec() }).into_bytes();
        self.run_job_on_worker("semio.compose", input, semio_framework_job::root_cancel_token()).await
    }
}

impl std::fmt::Debug for PluginInstanceHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PluginInstanceHandle").field("actor", &self.actor).finish()
    }
}

#[cfg(any(test, feature = "testkit"))]
pub fn test_relay_wake_authority()->GuestRelayWakeAuthority{
    GuestRelayWakeAuthority{drive_policy:serde_json::from_value(serde_json::from_str::<serde_json::Value>(include_str!("../../⚡️effects/🧪️tests/🧫️fixtures/🔣️.json")).unwrap()["driveGrant"].clone()).unwrap(),policy:semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:262144,maximum_depth:64},issuer:semio_framework_job_worker::admit_original_thread_worker_wake,receiver:Arc::new(GuestRelayWakeDriver::new(semio_framework_pack_json::from_json_str::<semio_plugin_host_wake::GuestRelayDriverPolicy>(include_str!("../../🔔️wake/📃️policy/🧪️tests/🧫️fixtures/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap(),semio_framework_job::default_now_us,Box::new(||false)).unwrap())}
}
/// 🎟️ See `README.md`.
#[cfg(any(test, feature = "testkit"))]
pub fn test_relay_original_input()->semio_framework_actor::RetainedTurnInput{
    semio_framework_actor::RetainedTurnInput{operation:71,generation:3,epoch:19,work:semio_framework_actor::OriginalActorTurnWork{clock_binding:1871,absolute_deadline_us:1_000_000,fuel:64},grant:test_relay_wake_authority().drive_policy}
}

#[cfg(any(test, feature = "testkit"))]
pub(super) async fn wait_for_scripted_guest_relay_release(runtime: &GuestRuntimes, completion: &GuestRelayCompletion) {
    let GuestRuntimes::Mock(mock) = runtime else {
        return;
    };
    let barrier = match completion {
        GuestRelayCompletion::Started(Err(_)) => mock.take_relay_start_failure_release(),
        GuestRelayCompletion::Stepped(Err(_)) => mock.take_relay_step_failure_release(),
        _ => None,
    };
    if let Some(barrier) = barrier {
        barrier.wait().await;
    }
}
