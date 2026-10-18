//! 🐎️ Wasmtime runtime: the wasmtime instance state and the wasmtime runtime with its compile, instantiate and sqlite observation entry points. See [`super`].

use super::*;

//#region 🎭️GuestRuntime
//#region 🐎️WasmtimeRuntime
pub(crate) struct WasmtimeInstanceState {
    pub(crate) store: Store<ActorHostState>,
    pub(crate) bindings: semio_framework_plugin_host_wit::actor_bindings::Actor,
    /// ⏱️ This store's wall deadline, read by the callback [`EpochDeadlines::install`] put on it.
    pub(crate) deadline: Arc<EpochDeadlineCell>,
    /// 🪪️ "One actor per app instance is the default" (design-abi.md §4) — minted once at
    /// `instantiate` and used to fill the `instance` field WIT's per-instance events carry but
    /// `semio_framework::kernel::Event`'s own lifecycle variants (`InstanceClose`/`Activate`/
    /// `SuspendRequest`/`CapabilityChanged`/`QuotaChanged`) dropped (kernel's `Event` is already
    /// actor-scoped by construction, so re-carrying the instance id per-event was redundant there).
    pub(super) instance_id: u32,
}

/// 🐎️ ONE shared `Engine` per process (`build_shared_engine`), one `Store<ActorHostState>` per
/// actor (`instantiate`). Wraps the type-independent primitives in `//#region 🔧️SharedWasmtimeEngine`
/// above — this is where they get consumed.
pub struct WasmtimeRuntime {
    pub(super) original_clock:fn()->Option<u64>,
    pub(crate) engine: Engine,
    pub epoch: EpochDeadlines,
    pub(super) linker: Linker<ActorHostState>,
    pub(super) cache_root: PathBuf,
    pub(super) engine_config_hash: [u8; 32],
    pub fuel_metering: bool,
    pub(super) isolated_engine: String,
    pub(super) next_instance_id: std::sync::atomic::AtomicU32,
}

impl WasmtimeRuntime {
    /// 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (B1 world-collapse): `Actor::add_to_linker`
    /// defines BOTH of the collapsed world's imports in one call (`pure` + `host-async`) —
    /// adding only `pure` now leaves 24 imports unresolved and every instantiation fails with
    /// "a matching implementation was not found in the linker".
    ///
    /// 🌐️ `add_to_linker_async`, not `_sync`: the Store is component-model-async now, and the
    /// sync WASI shim installs host functions that cannot be called from an async-lifted guest.
    pub async fn new(cfg: SharedEngineConfig,original_clock:fn()->Option<u64>) -> Result<Self, PluginHostError> {
        let (engine, pooling_active) = build_shared_engine(cfg).await?;
        let epoch = EpochDeadlines::new(&engine, &plugin_host_worker_pool());
        let mut linker = Linker::new(&engine);
        semio_framework_plugin_host_wit::actor_bindings::Actor::add_to_linker::<ActorHostState, wasmtime::component::HasSelf<ActorHostState>>(&mut linker, |state: &mut ActorHostState| state).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        semio_framework_plugin_host_operation::register(&mut linker).map_err(|error|PluginHostError::Wasmtime(error.to_string()))?;
        wasmtime_wasi::p2::add_to_linker_async(&mut linker).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        let engine_config_hash = shared_engine_config_hash(&cfg, pooling_active).await;
        let isolated_engine = SharedEngineConfig { force_on_demand: !pooling_active, ..cfg }.to_isolated_arg();
        Ok(Self { original_clock,engine, epoch, linker, cache_root: default_compiled_cache_root().await, engine_config_hash, fuel_metering: cfg.fuel_metering, isolated_engine, next_instance_id: std::sync::atomic::AtomicU32::new(1) })
    }

    /// 🕰️ Returns the clock supplied by the original runtime embedding.
    pub fn clock_source(&self)->fn()->Option<u64>{self.original_clock}
}

impl WasmtimeRuntime {
    /// ⏱️ Arms one guest crossing with the host's watchdog instead of the guest's own grant, for as
    /// long as the returned demand lives. Fuel is deliberately disarmed, for the reason the gateway's
    /// cold-open budget already states: a fuel yield is exactly as unresumable as an epoch cut under
    /// the JIT, so arming both only adds a second, harder-to-read way to say "wedged".
    pub(super) fn arm_guest_watchdog(&self, store: &mut Store<ActorHostState>, deadline: &EpochDeadlineCell) -> Result<EpochDemand, PluginHostError> {
        arm_store_fuel(store, self.fuel_metering, u64::MAX).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        Ok(self.epoch.arm(store, deadline, GUEST_JOB_WATCHDOG_MS))
    }

    /// 🌱️ The JIT half of [`OwnedRuntime::codec_call`]: one of the component's four pure `codec`
    /// functions on a throwaway instance. The hub itself runs these under the owned interpreter, so
    /// this exists as the A/B oracle — the two runtimes share nothing but the component bytes, and a
    /// fault that reproduces in both is in the GUEST rather than in an interpreter. That is exactly
    /// the bisect ticket 26/09/18 slice A1 used to clear the owned interpreter of the shadow-stack
    /// trap, and what slice TC3d used to place the `codec.genesis` trap in the guest's own resolver.
    pub(crate) async fn codec_instance(&self, compiled: &CompiledHandle, budget: &Budget) -> Result<GuestInstance, TurnFault> {
        self.instantiate(compiled, RuntimeActorId(0), &[], budget).await.map_err(TurnFault::Host)
    }

    /// 🏛️ Retrieves the component's handwritten SQL for an exact snapshot dialect.
    pub async fn codec_sqlite_schema(&self, compiled: &CompiledHandle, dialect: &str, budget: &Budget) -> Result<String, TurnFault> {
        let mut instance = self.codec_instance(compiled, budget).await?;
        let GuestInstanceState::Wasmtime(state) = &mut instance.state else { return Err(TurnFault::Trapped("SQLite schema requires Wasmtime instance".into())); };
        let WasmtimeInstanceState { store, bindings, deadline, .. } = state;
        let _epoch = self.epoch.arm_original(store, deadline, budget.deadline_us)?;
        store.run_concurrent(async |accessor| bindings.semio_framework_codec().call_sqlite_schema(accessor, dialect.to_string()).await).await.map_err(|error| TurnFault::Trapped(error.to_string()))?.map_err(|error| TurnFault::Trapped(error.to_string()))?.map_err(decode_guest_plugin_error)
    }

    /// 📤️ Executes a semantic snapshot projection within the guest's resource bounds.
    pub async fn codec_sqlite_export<O:FnMut(semio_framework_value::native_encoding::NativeEncodeProgress)->bool+?Sized>(&self, compiled: &CompiledHandle, dialect: &str, encoding: &str, payload: &[u8], limits: semio_framework::sqlite_snapshot::SqliteDatabaseLimits, budget: &Budget, progress: impl FnMut(u64, std::time::Duration), cancellation: &GuestCallCancellation, native_grant: semio_framework_value::RetainedCloneGrant, identity: &mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_,O>) -> Result<sqlite_wire::SnapshotFileResult, TurnFault> {
        let mut instance = self.codec_instance(compiled, budget).await?;
        let GuestInstanceState::Wasmtime(state) = &mut instance.state else { return Err(TurnFault::Trapped("SQLite export requires Wasmtime instance".into())); };
        let WasmtimeInstanceState { store, bindings, deadline, .. } = state;
        let _epoch = self.epoch.arm_original(store, deadline, budget.deadline_us)?;
        let encoding = match encoding { "binary" => semio_framework_plugin_host_wit::actor_bindings::exports::semio::framework::codec::SnapshotEncoding::Binary, "text" => semio_framework_plugin_host_wit::actor_bindings::exports::semio::framework::codec::SnapshotEncoding::Text, _ => return Err(TurnFault::Trapped("invalid snapshot encoding".into())) };
        let limits = wit_snapshot_limits(limits);
        let (coordinate,bytes)=snapshot_component_transport(dialect,payload,identity)?;
        let mut receiver=semio_framework_plugin_host_operation::OriginalOperationReceiver::new(identity);
        let output = observe_sqlite_guest(store.run_concurrent(async |accessor| semio_framework_plugin_host_operation::receive(accessor,&mut receiver,bindings.semio_framework_codec().call_sqlite_export(accessor, coordinate, encoding, bytes, limits, wit_snapshot_grant(native_grant))).await), &self.epoch, deadline, progress, cancellation).await?.map_err(|error| TurnFault::Trapped(error.to_string()))?.map_err(receiving_turn_fault)?.map_err(decode_guest_plugin_error)?;
        Ok(match output { semio_framework_plugin_host_wit::actor_bindings::exports::semio::framework::codec::SnapshotFileResult::Done(output) => sqlite_wire::SnapshotFileResult::Done(sqlite_wire::SnapshotFile { bytes: output.bytes, diagnostics: output.diagnostics }), semio_framework_plugin_host_wit::actor_bindings::exports::semio::framework::codec::SnapshotFileResult::Rejected(output) => sqlite_wire::SnapshotFileResult::Rejected(wit_snapshot_rejection(output)), semio_framework_plugin_host_wit::actor_bindings::exports::semio::framework::codec::SnapshotFileResult::Pending(output)=>sqlite_wire::SnapshotFileResult::Pending(wit_snapshot_retirement(output)) })
    }

    /// 📥️ Reconstructs a native snapshot from the exact declared semantic SQLite schema.
    pub async fn codec_sqlite_import<O:FnMut(semio_framework_value::native_encoding::NativeEncodeProgress)->bool+?Sized>(&self, compiled: &CompiledHandle, dialect: &str, payload: &[u8], limits: semio_framework::sqlite_snapshot::SqliteDatabaseLimits, budget: &Budget, progress: impl FnMut(u64, std::time::Duration), cancellation: &GuestCallCancellation, native_grant: semio_framework_value::RetainedCloneGrant, identity: &mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_,O>) -> Result<sqlite_wire::SnapshotPayloadResult, TurnFault> {
        let mut instance = self.codec_instance(compiled, budget).await?;
        let GuestInstanceState::Wasmtime(state) = &mut instance.state else { return Err(TurnFault::Trapped("SQLite import requires Wasmtime instance".into())); };
        let WasmtimeInstanceState { store, bindings, deadline, .. } = state;
        let _epoch = self.epoch.arm_original(store, deadline, budget.deadline_us)?;
        let limits = wit_snapshot_limits(limits);
        let (coordinate,bytes)=snapshot_component_transport(dialect,payload,identity)?;
        let mut receiver=semio_framework_plugin_host_operation::OriginalOperationReceiver::new(identity);
        let output = observe_sqlite_guest(store.run_concurrent(async |accessor| semio_framework_plugin_host_operation::receive(accessor,&mut receiver,bindings.semio_framework_codec().call_sqlite_import(accessor, coordinate, bytes, limits, wit_snapshot_grant(native_grant))).await), &self.epoch, deadline, progress, cancellation).await?.map_err(|error| TurnFault::Trapped(error.to_string()))?.map_err(receiving_turn_fault)?.map_err(decode_guest_plugin_error)?;
        Ok(match output {
            semio_framework_plugin_host_wit::actor_bindings::exports::semio::framework::codec::SnapshotPayloadResult::Done(output) => {
                let encoding = match output.encoding { semio_framework_plugin_host_wit::actor_bindings::exports::semio::framework::codec::SnapshotEncoding::Binary => sqlite_wire::SnapshotEncoding::Binary, semio_framework_plugin_host_wit::actor_bindings::exports::semio::framework::codec::SnapshotEncoding::Text => sqlite_wire::SnapshotEncoding::Text };
                sqlite_wire::SnapshotPayloadResult::Done(sqlite_wire::SnapshotPayload { encoding, bytes: output.bytes, diagnostics: output.diagnostics })
            }
            semio_framework_plugin_host_wit::actor_bindings::exports::semio::framework::codec::SnapshotPayloadResult::Rejected(output) => sqlite_wire::SnapshotPayloadResult::Rejected(wit_snapshot_rejection(output)),
            semio_framework_plugin_host_wit::actor_bindings::exports::semio::framework::codec::SnapshotPayloadResult::Pending(output)=>sqlite_wire::SnapshotPayloadResult::Pending(wit_snapshot_retirement(output)),
        })
    }

    /// 🧬️ `codec.pack-schema-hash` — the kind's 32-byte structural snapshot fingerprint.
    ///
    /// 🚫️ A guest's own `plugin-error` from any `codec` export is its typed refusal, decoded as the owned interpreter
    /// decodes it ([`decode_guest_plugin_error`]) — never a "trap" carrying the fault's raw bytes, which is what every
    /// agent opening a `stdio` document on hub 7800/p33 read instead of the guest's reason.
    pub async fn codec_pack_schema_hash(&self, compiled: &CompiledHandle, artifact_schema: &str, budget: &Budget) -> Result<[u8; 32], TurnFault> {
        let mut instance = self.codec_instance(compiled, budget).await?;
        let GuestInstanceState::Wasmtime(state) = &mut instance.state else {
            return Err(TurnFault::Trapped("codec.pack-schema-hash called on a non-wasmtime GuestInstance".to_string()));
        };
        let WasmtimeInstanceState { store, bindings, deadline, .. } = state;
        let _epoch = self.epoch.arm_original(store, deadline, budget.deadline_us)?;
        let artifact_schema = artifact_schema.to_string();
        let bytes = store
            .run_concurrent(async |accessor| bindings.semio_framework_codec().call_pack_schema_hash(accessor, artifact_schema).await)
            .await
            .map_err(|error| TurnFault::Trapped(error.to_string()))?
            .map_err(|error| TurnFault::Trapped(error.to_string()))?
            .map_err(decode_guest_plugin_error)?;
        <[u8; 32]>::try_from(bytes.as_slice()).map_err(|_| TurnFault::Trapped("guest pack schema hash is not 32 bytes".to_string()))
    }

    /// 🌱️ `codec.genesis` — the canonical empty document of `artifact_schema` at `document_id`.
    pub async fn codec_genesis(&self, compiled: &CompiledHandle, artifact_schema: &str, document_id: &str, budget: &Budget) -> Result<GuestDocumentPair, TurnFault> {
        let mut instance = self.codec_instance(compiled, budget).await?;
        let GuestInstanceState::Wasmtime(state) = &mut instance.state else {
            return Err(TurnFault::Trapped("codec.genesis called on a non-wasmtime GuestInstance".to_string()));
        };
        let WasmtimeInstanceState { store, bindings, deadline, .. } = state;
        let _epoch = self.epoch.arm_original(store, deadline, budget.deadline_us)?;
        let artifact_schema = artifact_schema.to_string();
        let document_id = document_id.to_string();
        let pair = store
            .run_concurrent(async |accessor| bindings.semio_framework_codec().call_genesis(accessor, artifact_schema, document_id).await)
            .await
            .map_err(|error| TurnFault::Trapped(error.to_string()))?
            .map_err(|error| TurnFault::Trapped(error.to_string()))?
            .map_err(decode_guest_plugin_error)?;
        Ok(GuestDocumentPair { pack: pair.pack, spr: pair.spr })
    }

    /// 📥️ `codec.print-mirror` — the host's pair-validation fence for a package whose Rust codec the
    /// host does not link. A pair the component cannot print back is not a document of this kind,
    /// which is what makes this callable as a DISCRIMINATOR and not only as a printer.
    pub async fn codec_print_mirror(&self, compiled: &CompiledHandle, artifact_schema: &str, pack: &[u8], spr: &[u8], budget: &Budget) -> Result<GuestDocumentMirror, TurnFault> {
        let mut instance = self.codec_instance(compiled, budget).await?;
        let GuestInstanceState::Wasmtime(state) = &mut instance.state else {
            return Err(TurnFault::Trapped("codec.print-mirror called on a non-wasmtime GuestInstance".to_string()));
        };
        let WasmtimeInstanceState { store, bindings, deadline, .. } = state;
        let _epoch = self.epoch.arm_original(store, deadline, budget.deadline_us)?;
        let artifact_schema = artifact_schema.to_string();
        let pair = semio_framework_plugin_host_wit::actor_bindings::exports::semio::framework::codec::DocumentPair { pack: pack.to_vec(), spr: spr.to_vec() };
        let mirror = store
            .run_concurrent(async |accessor| bindings.semio_framework_codec().call_print_mirror(accessor, artifact_schema, pair).await)
            .await
            .map_err(|error| TurnFault::Trapped(error.to_string()))?
            .map_err(|error| TurnFault::Trapped(error.to_string()))?
            .map_err(decode_guest_plugin_error)?;
        Ok(GuestDocumentMirror { dsl: mirror.0, ops: mirror.1 })
    }

    /// 🧩️ `codec.apply-ops` — the host-authoritative edit apply for a package whose Rust codec the
    /// host does not link.
    pub async fn codec_apply_ops(&self, compiled: &CompiledHandle, artifact_schema: &str, pack: &[u8], spr: &[u8], ops: &[u8], budget: &Budget, _identity: &mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<GuestDocumentPair, TurnFault> {
        let mut instance = self.codec_instance(compiled, budget).await?;
        let GuestInstanceState::Wasmtime(state) = &mut instance.state else {
            return Err(TurnFault::Trapped("codec.apply-ops called on a non-wasmtime GuestInstance".to_string()));
        };
        let WasmtimeInstanceState { store, bindings, deadline, .. } = state;
        let _epoch = self.epoch.arm_original(store, deadline, budget.deadline_us)?;
        let artifact_schema = artifact_schema.to_string();
        let pair = semio_framework_plugin_host_wit::actor_bindings::exports::semio::framework::codec::DocumentPair { pack: pack.to_vec(), spr: spr.to_vec() };
        let ops = ops.to_vec();
        let next = store
            .run_concurrent(async |accessor| bindings.semio_framework_codec().call_apply_ops(accessor, artifact_schema, pair, ops).await)
            .await
            .map_err(|error| TurnFault::Trapped(error.to_string()))?
            .map_err(|error| TurnFault::Trapped(error.to_string()))?
            .map_err(decode_guest_plugin_error)?;
        Ok(GuestDocumentPair { pack: next.pack, spr: next.spr })
    }

    /// 📜️ `codec.replay-envelopes` — the hub's Check In fold for a package whose Rust codec the host
    /// does not link.
    pub async fn codec_replay_envelopes(&self, compiled: &CompiledHandle, artifact_schema: &str, pack: &[u8], spr: &[u8], envelopes: &[u8], budget: &Budget) -> Result<GuestDocumentPair, TurnFault> {
        let mut instance = self.codec_instance(compiled, budget).await?;
        let GuestInstanceState::Wasmtime(state) = &mut instance.state else {
            return Err(TurnFault::Trapped("codec.replay-envelopes called on a non-wasmtime GuestInstance".to_string()));
        };
        let WasmtimeInstanceState { store, bindings, deadline, .. } = state;
        let _epoch = self.epoch.arm_original(store, deadline, budget.deadline_us)?;
        let artifact_schema = artifact_schema.to_string();
        let pair = semio_framework_plugin_host_wit::actor_bindings::exports::semio::framework::codec::DocumentPair { pack: pack.to_vec(), spr: spr.to_vec() };
        let envelopes = envelopes.to_vec();
        let next = store
            .run_concurrent(async |accessor| bindings.semio_framework_codec().call_replay_envelopes(accessor, artifact_schema, pair, envelopes).await)
            .await
            .map_err(|error| TurnFault::Trapped(error.to_string()))?
            .map_err(|error| TurnFault::Trapped(error.to_string()))?
            .map_err(decode_guest_plugin_error)?;
        Ok(GuestDocumentPair { pack: next.pack, spr: next.spr })
    }
}

pub async fn observe_sqlite_guest<F: std::future::Future>(future: F, epoch: &EpochDeadlines, deadline: &EpochDeadlineCell, mut progress: impl FnMut(u64, std::time::Duration), cancellation: &GuestCallCancellation) -> Result<F::Output, TurnFault> {
    let started = std::time::Instant::now();
    let pool = &epoch.inner.pool;
    let mut next_tick = pool.now_ms();
    let mut future = std::pin::pin!(future);
    loop {
        match semio_framework_async::select2(future.as_mut(), pool.timer().sleep_until(next_tick)).await {
            semio_framework_async::Either::Left(result) => return if cancellation.is_cancelled() { Err(TurnFault::Cancelled) } else { Ok(result) },
            semio_framework_async::Either::Right(()) => {
                progress(0, started.elapsed());
                if cancellation.is_cancelled() {
                    deadline.0.store(0, std::sync::atomic::Ordering::Release);
                    epoch.inner.engine.increment_epoch();
                }
                next_tick = pool.now_ms().saturating_add(16);
            }
        }
    }
}
//#endregion 🐎️WasmtimeRuntime
//#endregion 🎭️GuestRuntime
