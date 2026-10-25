//! 🔌️ Plugin instance handle: the plugin instance handle with its debug implementation, the relay wake authority test constructor and the scripted relay release wait. See [`super`].

use super::*;

//#region 🔀️PostTurnRelay
/// 🧬️ design-runtime.md §2's post-turn dispatch replacement for the old `Arc<WasmPluginRuntime>`
/// synchronous-call handle. `IoRouter`/`ArtifactInferenceRouter` used to hold a full `WasmPluginRuntime`
/// (component + linker + `Store`) per plugin so a router method could call straight into the guest's
/// own exported functions (`artifact-compose`, `io-run`, `io-sniff`, `artifact-infer`) mid-turn. Those
/// exports do not exist on `world actor` at all — the ONLY guest entry points left are
/// `reactor::poll` and `jobs::{start-job,step-job,cancel-job}` (`📜️.wit`) — so a router now
/// drives a well-known cold job kind (`semio.io-run` / `semio.io-sniff` / `semio.infer`, all three
/// named in `jobs.wit`'s own doc comments) to completion via `GuestRuntime`, strictly AFTER the turn
/// that produced the triggering effect returned (never re-entrantly against that turn's own `Store`).
pub struct PluginInstanceHandle {
    pub actor: RuntimeActorId,
    original_input:semio_framework_actor::RetainedTurnInput,
    pub runtime: Arc<GuestRuntimes>,
    pub instance: Arc<Mutex<GuestInstanceSlot>>,
    pub(super) instance_gate: Arc<semio_framework_async_sync::Semaphore>,
    relay_registry: Arc<GuestRelayMountedRegistry>,
    next_job_id: std::sync::atomic::AtomicU64,
}

impl PluginInstanceHandle {
    pub async fn new(actor: RuntimeActorId, runtime: Arc<GuestRuntimes>, instance: GuestInstance,original_input:semio_framework_actor::RetainedTurnInput,wake_authority:GuestRelayWakeAuthority) -> Self {
        Self {
            actor,
            original_input,
            runtime,
            instance: Arc::new(Mutex::new(GuestInstanceSlot::Available(instance))),
            instance_gate: Arc::new(semio_framework_async_sync::Semaphore::new(1)),
            relay_registry: Arc::new(GuestRelayMountedRegistry::new(wake_authority)),
            next_job_id: std::sync::atomic::AtomicU64::new(1),
        }
    }

    /// 🧵️ Starts one cold job in the fixed mounted registry. Each host poll admits at most one
    /// relay or close opportunity; the pool never owns an internal run-to-completion chain.
    async fn run_job_on_worker(&self, kind: &str, input: Vec<u8>, cancel: semio_framework_async::CancelToken) -> Result<Vec<u8>, PluginHostError> {
        if input.len() > semio_framework_job_payload::JOB_PAYLOAD_OPERATION_BYTES {
            return Err(PluginHostError::Plugin(format!("{kind} input exceeds the retained operation byte limit")));
        }
        if let Some(detail) = self.instance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).admission_fault() {
            return Err(PluginHostError::Plugin(format!("{kind} job rejected: {}", String::from_utf8_lossy(&detail))));
        }
        let job = self.next_job_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let pool = plugin_host_worker_pool();
        let operation = semio_framework_job::OperationId(self.actor.0);
        let generation = semio_framework_job::Generation(job);
        let params = semio_framework_job_worker_authority::BatchJobParams {
            operation,
            generation,
            cancel: semio_framework_job::root_cancel_token(),
            config: semio_framework_job_worker_authority::BatchDriveConfig {
                retained:self.relay_registry.wake_authority.drive_policy,
                site: "plugin-host.cold-relay",
                stage: semio_framework_job::InteractiveStage::UserVisibleSimStep,
                fuel_per_step: semio_framework_job::USER_VISIBLE_LANE_FUEL,
                step_budget_us: semio_framework_job::USER_VISIBLE_LANE_WALL_US,
            },
            now_us: semio_framework_job::default_now_us,
        };
        let relay = GuestColdRelayJob::new(Arc::clone(&self.runtime), Arc::clone(&self.instance), Arc::clone(&self.instance_gate), pool.clone(), cancel, job, (kind.to_string(), input));
        let (index, mounted_generation) = self.relay_registry.reserve().ok_or_else(|| PluginHostError::Plugin(format!("{kind} mounted relay registry is full")))?;
        let owner = match admit_guest_relay_session(relay, params, self.relay_registry.wake_authority.drive_policy) {
            Ok(session) => GuestRelayMountedOwner::Session(session),
            Err(rejected) => GuestRelayMountedOwner::Rejected(rejected),
        };
        self.relay_registry.mount(index, mounted_generation, owner);
        GuestRelayMountedFuture { registry: Arc::clone(&self.relay_registry), index, generation: mounted_generation, complete: false }.await
    }

    /// 🌉️ Executes ONE hop of this plugin's own local io mechanism registry — the absorbed `io-run`
    /// guest export, now `jobs.wit`'s cold job kind `semio.io-run`. `payload` is already JSON
    /// `io_schema::IoPayload` bytes (this method's own caller contract, mirroring the deleted
    /// `WasmPluginRuntime::io_run`'s); `input` bundles what used to be three separate export
    /// parameters into the one JSON `(source, target, IoPayload)` tuple `jobs.wit`'s doc comment
    /// specifies, since a job only carries one opaque `list<u8>`.
    pub async fn io_run(&self, from: &str, into: &str, payload: Vec<u8>) -> Result<Vec<u8>, PluginHostError> {
        #[derive(ToValue)]
        struct IoRunInputWire<'a> {
            source: &'a str,
            target: &'a str,
            payload: semio_framework::io_schema::IoPayload,
        }
        let payload_text = std::str::from_utf8(&payload).map_err(|error| PluginHostError::Json(error.to_string()))?;
        let io_payload: semio_framework::io_schema::IoPayload = semio_framework_pack_json::from_json_str(payload_text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| PluginHostError::Json(error.to_string()))?;
        let input = semio_framework_pack_json::to_json_string(&IoRunInputWire { source: from, target: into, payload: io_payload }).into_bytes();
        self.run_job_on_worker("semio.io-run", input, semio_framework_job::root_cancel_token()).await
    }

    /// 🔍️ Sniffs this plugin's own `(from, into)` hop — the absorbed `io-sniff` guest export, now
    /// `semio.io-sniff`. Returns the raw `io_schema::Confidence::rank()` byte (`0`=None..`3`=High),
    /// mirroring the deleted `WasmPluginRuntime::io_sniff`'s return shape exactly.
    pub async fn io_sniff(&self, from: &str, into: &str, payload: &[u8]) -> Result<u8, PluginHostError> {
        #[derive(ToValue)]
        struct IoRunInputWire<'a> {
            source: &'a str,
            target: &'a str,
            payload: semio_framework::io_schema::IoPayload,
        }
        let payload_text = std::str::from_utf8(payload).map_err(|error| PluginHostError::Json(error.to_string()))?;
        let io_payload: semio_framework::io_schema::IoPayload = semio_framework_pack_json::from_json_str(payload_text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| PluginHostError::Json(error.to_string()))?;
        let input = semio_framework_pack_json::to_json_string(&IoRunInputWire { source: from, target: into, payload: io_payload }).into_bytes();
        let result = self.run_job_on_worker("semio.io-sniff", input, semio_framework_job::root_cancel_token()).await?;
        result.first().copied().ok_or_else(|| PluginHostError::Plugin("semio.io-sniff job returned an empty result".to_string()))
    }

    /// 💡️ Executes one guest inference call — the absorbed `contributor.artifact-infer` export, now
    /// `semio.infer` (`types.wit`'s own doc comment names this job kind explicitly). `request`/the
    /// result are the SAME JSON `io_schema::ArtifactInferenceRequest`/`ArtifactInferenceResult` bytes
    /// the deleted `WasmPluginRuntime::artifact_infer` used — no tuple wrapping needed, since that
    /// call already took exactly one opaque payload. The relay runs under a CHILD of `cancel`: the
    /// caller's cancel stops the in-flight guest job, while the relay's own close never cancels the
    /// caller's token (a dependency chain shares it).
    pub async fn infer(&self, request: &[u8], cancel: &semio_framework_async::CancelToken) -> Result<Vec<u8>, PluginHostError> {
        self.run_job_on_worker("semio.infer", request.to_vec(), cancel.child_now()).await
    }

    /// 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (cold-kinds): executes one guest mutation-plan
    /// call — the absorbed `contributor.artifact-mutation-plan` export, now `semio.mutation-plan`.
    /// `request`/the result are the SAME DSL wire-pack bytes `ArtifactMutationRouter::plan` already
    /// builds via `encode_wire_dsl`/decodes via `decode_wire_dsl` (this region's own helpers,
    /// `HostArtifactMutationPlanRequest`/`Result`'s field-for-field guest mirror) — no tuple
    /// wrapping needed, mirroring `infer`'s own doc note above.
    pub async fn mutation_plan(&self, request: &[u8]) -> Result<Vec<u8>, PluginHostError> {
        self.run_job_on_worker("semio.mutation-plan", request.to_vec(), semio_framework_job::root_cancel_token()).await
    }

    /// 🔀️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (cold-kinds): executes one guest versioned
    /// re-encode — the absorbed `migrate-artifact` export, now `semio.migrate`. `from`/`to` are
    /// `io_schema::ArtifactDialect::to_coordinate()` strings; `pack` is the bytes to re-encode.
    /// Bundles all three into one JSON tuple the SAME way `io_run` above bundles its own former
    /// three separate export parameters (`⚛️reactor/💼️jobs/🔀️migrate/🦀️.rs`'s own
    /// `MigrateInput` decodes a struct positionally from this tuple, matching `IoRunInput`'s
    /// established idiom). The caller that resolves WHICH `PluginInstanceHandle` to migrate through
    /// belongs to the pending runtime/db refactor (this method only provides the dispatch, not its
    /// call site — see this packet's report `## lease-requests`).
    pub async fn migrate(&self, from: &str, to: &str, pack: Vec<u8>) -> Result<Vec<u8>, PluginHostError> {
        #[derive(serde::Serialize, ToValue)]
        struct MigrateInputWire<'a> {
            from: &'a str,
            to: &'a str,
            pack: Vec<u8>,
        }
        let input = semio_framework_pack_json::to_json_string(&MigrateInputWire { from, to, pack }).into_bytes();
        self.run_job_on_worker("semio.migrate", input, semio_framework_job::root_cancel_token()).await
    }

    /// 🧩️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (cold-kinds): routes to this plugin's own
    /// `semio.compose` cold job — design-abi.md §2: "`artifact-compose` ... become well-known cold
    /// job kinds ... driven by `start-job` + `step-job`". The guest body is owned by the live
    /// `compose-await` packet (`ComposeStepper`/`ComposeState`, deliberately NOT defined here or
    /// anywhere in this file) — until it registers `"semio.compose"` via `register_bounded_job_kind`, this
    /// fails with the ordinary `job.unknown-kind` fault `step_job` already produces for any
    /// unregistered kind, not a hand-written host refusal. `key_bytes` is the JSON `IoKey`
    /// `IoRouter::compose` already resolved ownership from; `sources_bytes` passes through
    /// byte-for-byte. This wire shape is provisional until `compose-await`'s own guest decode is
    /// defined — coordinate any format change with that packet before altering it here.
    pub async fn compose(&self, key_bytes: &[u8], sources_bytes: &[u8]) -> Result<Vec<u8>, PluginHostError> {
        #[derive(serde::Serialize, ToValue)]
        struct ComposeInput {
            key: Vec<u8>,
            sources: Vec<u8>,
        }
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
    GuestRelayWakeAuthority{drive_policy:serde_json::from_value(serde_json::from_str::<serde_json::Value>(include_str!("../../⚡️effects/🧪️tests/🧫️fixtures/🔣️.json")).unwrap()["driveGrant"].clone()).unwrap(),policy:semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:262144,maximum_depth:64},issuer:semio_framework_job_worker::admit_original_thread_worker_wake,receiver:Arc::new(GuestRelayWakeDriver::new(semio_framework_pack_json::from_json_str::<semio_framework_plugin_host_wake::GuestRelayDriverPolicy>(include_str!("../../🔔️wake/📃️policy/🧪️tests/🧫️fixtures/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap(),semio_framework_job::default_now_us,Box::new(||false)).unwrap())}
}
/// 🎟️ The supplied retained turn that a mounted test handle keeps for its instance.
#[cfg(any(test, feature = "testkit"))]
pub fn test_relay_original_input()->semio_framework_actor::RetainedTurnInput{
    semio_framework_actor::RetainedTurnInput{operation:71,generation:3,epoch:19,work:semio_framework_actor::OriginalActorTurnWork{clock_binding:1871,absolute_deadline_us:1_000_000,fuel:64},grant:test_relay_wake_authority().drive_policy}
}
//#endregion 🔀️PostTurnRelay

//#region 🔧️SharedWasmtimeEngine
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
//#endregion 🔧️SharedWasmtimeEngine
