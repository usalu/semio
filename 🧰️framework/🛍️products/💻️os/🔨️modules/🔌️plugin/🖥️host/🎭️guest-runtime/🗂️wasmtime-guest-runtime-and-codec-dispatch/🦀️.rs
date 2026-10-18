//! 🗂️ Wasmtime guest runtime and codec dispatch: the guest codec dispatch over every runtime and the guest runtime implementation for the wasmtime runtime. See [`super`].

use super::*;

//#region 🎭️GuestRuntime
//#region 🐎️WasmtimeRuntime
//#region 🗂️GuestCodecDispatch
/// 🗂️ The component's own `codec` interface, dispatched across whichever runtime a caller actually
/// holds. Both concrete runtimes implement all four functions; [`GuestRuntimes`] did not forward any
/// of them, so a caller holding the enum — which is what `🏃️run` and `🌉️mcp` both hold, because a
/// `wasmtime::Component` belongs to the `Engine` that compiled it and cannot be moved to a second
/// runtime — could not reach a package's codec at all. That is what made a hub-bound MCP gateway
/// unable to register a `store::os_store::serialization::ArtifactCodec` for a fourth package's document kind, and therefore
/// unable to open that document's socket (ticket 26/09/18 slices M9 §8.1 / M10 §3.3).
///
/// ⛽️ Every one of these is pure by the WIT's own contract: it reads no storage, opens no window,
/// emits no effect, and runs on a throwaway instance the runtime creates and drops per call.
impl GuestRuntimes {
    /// ♻️ `package`'s compiled code when this runtime already has it on disk, without compiling.
    pub async fn load_compiled(&self, package: &PackageRef) -> Option<CompiledHandle> {
        let Self::Wasmtime(runtime) = self else { return None };
        let cache_path = compiled_cache_path(&runtime.cache_root, &runtime.engine_config_hash, &package.hash.0).await;
        load_compiled_component(&runtime.engine, &cache_path).await.map(|component| CompiledHandle { package_hash: package.hash.0, component: Some(Arc::new(component)), owned: None })
    }

    /// 🧊️ Where an isolated compile of `package` must land for this runtime to load it — `None` for a
    /// runtime whose compile is not a JIT (the owned interpreter prepares in memory).
    pub async fn isolated_compile_target(&self, package: &PackageRef) -> Option<IsolatedCompileTarget> {
        let Self::Wasmtime(runtime) = self else { return None };
        Some(IsolatedCompileTarget { cache_path: compiled_cache_path(&runtime.cache_root, &runtime.engine_config_hash, &package.hash.0).await, engine: runtime.isolated_engine.clone() })
    }

    /// 🏛️ Queries the exact component's declared handwritten semantic snapshot schema.
    pub async fn codec_sqlite_schema(&self, compiled: &CompiledHandle, dialect: &str, budget: &Budget) -> Result<String, TurnFault> {
        match self { Self::Owned(runtime) => runtime.codec_sqlite_schema(owned_compiled(compiled)?, dialect, budget.clone()).await, Self::Wasmtime(runtime) => runtime.codec_sqlite_schema(compiled, dialect, budget).await, #[cfg(any(test, feature = "testkit"))] Self::Mock(_) | Self::Recording(_) => Err(TurnFault::Trapped("semantic SQLite schema requires a real component".into())) }
    }

    /// 📤️ Executes the component's typed projection with caller cancellation and resource bounds.
    pub async fn codec_sqlite_export<O:FnMut(semio_framework_value::native_encoding::NativeEncodeProgress)->bool+?Sized>(&self, compiled: &CompiledHandle, dialect: &str, encoding: &str, payload: &[u8], limits: semio_framework::sqlite_snapshot::SqliteDatabaseLimits, budget: &Budget, progress: impl FnMut(u64, std::time::Duration), cancellation: &GuestCallCancellation, native_grant: semio_framework_value::RetainedCloneGrant, identity: &mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_,O>) -> Result<sqlite_wire::SnapshotFileResult, TurnFault> {
        if cancellation.is_cancelled() { return Err(TurnFault::Cancelled); }
        match self { Self::Owned(runtime) => runtime.codec_sqlite_export(owned_compiled(compiled)?, dialect, encoding, payload, limits, budget.clone(), progress, cancellation, native_grant, identity).await, Self::Wasmtime(runtime) => runtime.codec_sqlite_export(compiled, dialect, encoding, payload, limits, budget, progress, cancellation, native_grant, identity).await, #[cfg(any(test, feature = "testkit"))] Self::Mock(_) | Self::Recording(_) => Err(TurnFault::Trapped("semantic SQLite export requires a real component".into())) }
    }

    /// 📥️ Executes exact semantic reconstruction with caller cancellation and resource bounds.
    pub async fn codec_sqlite_import<O:FnMut(semio_framework_value::native_encoding::NativeEncodeProgress)->bool+?Sized>(&self, compiled: &CompiledHandle, dialect: &str, payload: &[u8], limits: semio_framework::sqlite_snapshot::SqliteDatabaseLimits, budget: &Budget, progress: impl FnMut(u64, std::time::Duration), cancellation: &GuestCallCancellation, native_grant: semio_framework_value::RetainedCloneGrant, identity: &mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_,O>) -> Result<sqlite_wire::SnapshotPayloadResult, TurnFault> {
        if cancellation.is_cancelled() { return Err(TurnFault::Cancelled); }
        match self { Self::Owned(runtime) => runtime.codec_sqlite_import(owned_compiled(compiled)?, dialect, payload, limits, budget.clone(), progress, cancellation, native_grant, identity).await, Self::Wasmtime(runtime) => runtime.codec_sqlite_import(compiled, dialect, payload, limits, budget, progress, cancellation, native_grant, identity).await, #[cfg(any(test, feature = "testkit"))] Self::Mock(_) | Self::Recording(_) => Err(TurnFault::Trapped("semantic SQLite import requires a real component".into())) }
    }

    /// 🧬️ `codec.pack-schema-hash` — the kind's 32-byte structural snapshot fingerprint.
    pub async fn codec_pack_schema_hash(&self, compiled: &CompiledHandle, artifact_schema: &str, budget: &Budget) -> Result<[u8; 32], TurnFault> {
        match self {
            Self::Owned(runtime) => runtime.codec_pack_schema_hash(owned_compiled(compiled)?, artifact_schema, budget.clone()).await,
            Self::Wasmtime(runtime) => runtime.codec_pack_schema_hash(compiled, artifact_schema, budget).await,
            #[cfg(any(test, feature = "testkit"))]
            Self::Mock(_) | Self::Recording(_) => Err(TurnFault::Trapped("codec.pack-schema-hash has no scripted runtime — it needs a real component".to_string())),
        }
    }

    /// 🌱️ `codec.genesis` — the canonical empty document of `artifact_schema` at `document_id`.
    pub async fn codec_genesis(&self, compiled: &CompiledHandle, artifact_schema: &str, document_id: &str, budget: &Budget) -> Result<GuestDocumentPair, TurnFault> {
        match self {
            Self::Owned(runtime) => runtime.codec_genesis(owned_compiled(compiled)?, artifact_schema, document_id, budget.clone()).await,
            Self::Wasmtime(runtime) => runtime.codec_genesis(compiled, artifact_schema, document_id, budget).await,
            #[cfg(any(test, feature = "testkit"))]
            Self::Mock(_) | Self::Recording(_) => Err(TurnFault::Trapped("codec.genesis has no scripted runtime — it needs a real component".to_string())),
        }
    }

    /// 📥️ `codec.print-mirror` — the pair-validation fence.
    pub async fn codec_print_mirror(&self, compiled: &CompiledHandle, artifact_schema: &str, pack: &[u8], spr: &[u8], budget: &Budget) -> Result<GuestDocumentMirror, TurnFault> {
        match self {
            Self::Owned(runtime) => runtime.codec_print_mirror(owned_compiled(compiled)?, artifact_schema, pack, spr, budget.clone()).await,
            Self::Wasmtime(runtime) => runtime.codec_print_mirror(compiled, artifact_schema, pack, spr, budget).await,
            #[cfg(any(test, feature = "testkit"))]
            Self::Mock(_) | Self::Recording(_) => Err(TurnFault::Trapped("codec.print-mirror has no scripted runtime — it needs a real component".to_string())),
        }
    }

    /// 🧩️ `codec.apply-ops` — the host-authoritative edit apply.
    pub async fn codec_apply_ops(&self, compiled: &CompiledHandle, artifact_schema: &str, pack: &[u8], spr: &[u8], ops: &[u8], budget: &Budget, identity: &mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<GuestDocumentPair, TurnFault> {
        match self {
            Self::Owned(runtime) => runtime.codec_apply_ops(owned_compiled(compiled)?, artifact_schema, pack, spr, ops, budget.clone(), identity).await,
            Self::Wasmtime(runtime) => runtime.codec_apply_ops(compiled, artifact_schema, pack, spr, ops, budget, identity).await,
            #[cfg(any(test, feature = "testkit"))]
            Self::Mock(_) | Self::Recording(_) => Err(TurnFault::Trapped("codec.apply-ops has no scripted runtime — it needs a real component".to_string())),
        }
    }

    /// 📜️ `codec.replay-envelopes` — the hub's Check In fold; `progress` sees the owned interpreter's
    /// fuel observations and `cancellation` ends its interpretation between two steps.
    pub async fn codec_replay_envelopes(&self, compiled: &CompiledHandle, artifact_schema: &str, pack: &[u8], spr: &[u8], envelopes: &[u8], budget: &Budget, progress: impl FnMut(u64, std::time::Duration), cancellation: &GuestCallCancellation) -> Result<GuestDocumentPair, TurnFault> {
        match self {
            Self::Owned(runtime) => runtime.codec_replay_envelopes_observed(owned_compiled(compiled)?, artifact_schema, pack, spr, envelopes, budget.clone(), progress, cancellation).await,
            Self::Wasmtime(runtime) => runtime.codec_replay_envelopes(compiled, artifact_schema, pack, spr, envelopes, budget).await,
            #[cfg(any(test, feature = "testkit"))]
            Self::Mock(_) | Self::Recording(_) => Err(TurnFault::Trapped("codec.replay-envelopes has no scripted runtime — it needs a real component".to_string())),
        }
    }
}
//#endregion 🗂️GuestCodecDispatch

impl GuestRuntime for WasmtimeRuntime {
    async fn admit_actor_context(&self,inst:&mut GuestInstance,input:semio_framework::kernel::RetainedTurnInput,budget:Budget,identity:&mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)->Result<semio_framework_actor::retained_context::OriginalActorAdmissionReply,TurnFault>{let _=identity;let GuestInstanceState::Wasmtime(state)=&mut inst.state else{return Err(identity_turn_fault(semio_framework_plugin_guest_contract::operation_authority::refusal(4)))};let WasmtimeInstanceState{store,bindings,deadline,..}=state;arm_store_fuel(store,self.fuel_metering,budget.fuel).map_err(|error|TurnFault::Host(PluginHostError::Wasmtime(error.to_string())))?;let _epoch=self.epoch.arm_original(store,deadline,budget.deadline_us)?;let input=semio_framework_plugin_host_wit::retained_turn_wire::input_to_wit(input);let original=store.run_concurrent(async|accessor|bindings.semio_framework_reactor().call_admit_actor_context(accessor,input,semio_framework_plugin_host_wit::retained_turn_wire::grant_to_wit(budget.retained_policy)).await).await.and_then(|result|result).map_err(|trap|classify_guest_trap(&trap))?;semio_framework_plugin_host_wit::retained_turn_wire::admission_from_wit(original)}
    async fn close_actor_context(&self,inst:&mut GuestInstance,input:semio_framework::kernel::RetainedTurnInput,budget:Budget,identity:&mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)->Result<semio_framework_actor::retained_context::OriginalActorCloseReply,TurnFault>{let _=identity;let GuestInstanceState::Wasmtime(state)=&mut inst.state else{return Err(identity_turn_fault(semio_framework_plugin_guest_contract::operation_authority::refusal(4)))};let WasmtimeInstanceState{store,bindings,deadline,..}=state;arm_store_fuel(store,self.fuel_metering,budget.fuel).map_err(|error|TurnFault::Host(PluginHostError::Wasmtime(error.to_string())))?;let _epoch=self.epoch.arm_original(store,deadline,budget.deadline_us)?;let input=semio_framework_plugin_host_wit::retained_turn_wire::input_to_wit(input);let original=store.run_concurrent(async|accessor|bindings.semio_framework_reactor().call_close_actor_context(accessor,input,semio_framework_plugin_host_wit::retained_turn_wire::grant_to_wit(budget.retained_policy)).await).await.and_then(|result|result).map_err(|trap|classify_guest_trap(&trap))?;semio_framework_plugin_host_wit::retained_turn_wire::close_from_wit(original)}

    /// 🚫️async: R13/R14 corollary — `let _ = <async call>;` used to suppress the lint while
    /// silently dropping this call's future: the on-disk compilation cache write never ran, so
    /// `compile` was recompiling from wasm bytes on every single call regardless of
    /// `load_compiled_component`'s cache-hit path above. Best-effort by design (a failed cache
    /// write must not fail `compile` itself), so the `Result` still discards, only the future
    /// is now actually driven.
    async fn compile(&self, package: &PackageRef, bytes: &[u8]) -> Result<CompiledHandle, PluginHostError> {
        let cache_path = compiled_cache_path(&self.cache_root, &self.engine_config_hash, &package.hash.0).await;
        if let Some(component) = load_compiled_component(&self.engine, &cache_path).await {
            return Ok(CompiledHandle { package_hash: package.hash.0, component: Some(Arc::new(component)), owned: None });
        }
        let component = Component::from_binary(&self.engine, bytes).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        let _ = store_compiled_component(&component, &cache_path).await;
        Ok(CompiledHandle { package_hash: package.hash.0, component: Some(Arc::new(component)), owned: None })
    }

    /// 🐛️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-wasmtime-upgrade): wasmtime 22.0.1's
    /// `bindgen!`-generated `Actor::instantiate` returned `(Actor, wasmtime::component::Instance)`;
    /// wasmtime 47.0.3 dropped the raw `Instance` from the convenience wrapper (it was never used
    /// here anyway — every subsequent call goes through `bindings`' own typed accessors) and
    /// returns bare `Actor` instead. See `path2.rs`'s expanded bindgen output in
    /// `wasmtime-internal-component-macro-47.0.3/tests/expanded/` for the confirmed new shape.
    ///
    /// 🧬️ B1 world-collapse: `instantiate_async`, not `instantiate` — the world's `host-async`
    /// imports are async host functions, and the sync entry point refuses a Store whose linker
    /// carries any.
    async fn instantiate(&self, compiled: &CompiledHandle, actor: RuntimeActorId, caps: &[BrokerCapabilityGrant], budget: &Budget) -> Result<GuestInstance, PluginHostError> {
        let component = compiled.component.as_ref().ok_or_else(|| PluginHostError::Plugin("CompiledHandle has no wasmtime Component — built by MockGuestRuntime::compile, not WasmtimeRuntime::compile".to_string()))?;
        let instance_id = self.next_instance_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let (wasi, diagnostics) = guest_wasi_ctx();
        let host_state = ActorHostState {
            original_job_clock:None,
            operation_slot:semio_framework_plugin_host_operation::OperationSlot::new(),
            plugin_id: format!("actor-{}", actor.0),
            actor,
            caps: caps.to_vec(),
            emit_sink: Vec::new(),
            emit_patch_sink: Vec::new(),
            asset_map: HashMap::new(),
            limiter: BudgetLimiter::default(),
            wasi_ctx: wasi,
            diagnostics,
            resource_table: ResourceTable::new(),
        };
        let mut store = Store::new(&self.engine, host_state);
        store.limiter(|state| &mut state.limiter as &mut dyn ResourceLimiter);
        arm_store_fuel(&mut store, self.fuel_metering, budget.fuel).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        let deadline = Arc::new(EpochDeadlineCell(std::sync::atomic::AtomicU64::new(u64::MAX),std::sync::atomic::AtomicU64::new(u64::MAX),Some(self.original_clock)));
        self.epoch.install(&mut store, Arc::clone(&deadline));
        let remaining=remaining_original_deadline_us((self.original_clock)(),budget.deadline_us).map_err(|error|PluginHostError::NativeIo(semio_framework_value::ValueError::literal(match error{OriginalDeadlineError::MissingClock=>semio_framework_value::ValueRefusalKind::UnsupportedOwner,OriginalDeadlineError::Expired=>semio_framework_value::ValueRefusalKind::WorkLimit},"original guest admission requires its live caller deadline")))?;
        let _epoch = self.epoch.arm(&mut store, &deadline, remaining.div_ceil(1_000));
        deadline.1.store(budget.deadline_us,std::sync::atomic::Ordering::Release);
        let bindings = semio_framework_plugin_host_wit::actor_bindings::Actor::instantiate_async(&mut store, component, &self.linker).await.map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        let guest = store.run_concurrent(async |accessor| bindings.semio_framework_reactor().call_channel_version(accessor).await).await.and_then(|inner| inner).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        protocol::admit_guest_channel_version(guest, protocol::CHANNEL_VERSION).map_err(|fault| PluginHostError::Refused(Box::new(fault)))?;
        Ok(GuestInstance { actor, state: GuestInstanceState::Wasmtime(WasmtimeInstanceState { store, bindings, instance_id, deadline }) })
    }

    /// 🚫️async: R10 residue shape 1 — `kernel_event_to_wit` is async, hoisted out of the sync
    /// `Iterator::map` closure via a plain loop.
    ///
    /// 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (B1 world-collapse): `poll` is `async func`
    /// now, so it is driven through `Store::run_concurrent`'s `Accessor` rather than called
    /// directly against `&mut Store` — that is the ONLY shape wasmtime offers for an async-lifted
    /// export, and it is what lets the guest suspend on a `host-async` import mid-turn without
    /// unwinding the call. Args are owned (moved into the concurrent task), not borrowed.
    /// A turn's pages are staged FIRST, in the same concurrent task, so the guest reads them off
    /// its staging slot instead of carrying them through `poll`'s leaked parameter area.
    ///
    /// 🚪️ B1 world-collapse: everything the guest pushed through `host-async.emit` during THIS
    /// turn is delivered on the same `turn-result` as the effects it returned — one merged list,
    /// emitted-first (they happened earlier in the turn, by construction).
    ///
    /// 👥️ M2 render-plane presence (sol's ruling, 26/08/20): `presence-update.update` is a
    /// pack-encoded `ui_contract::PresenceUpdate`, NOT the replication `PresencePeer` the
    /// record originally declared — the consumer of a turn result is the renderer, which
    /// needs `(surface, node_key)` addressing and a TTL, while the collaboration roster keeps
    /// its own channel (`ephemeral_snapshot` out, `AppCommand::Presence`/`adopt_presence`
    /// in). Symmetric with the guest's `kernel_presence_update_to_wit`.
    ///
    /// A malformed entry is SKIPPED rather than failing the turn, matching `AppCommand::
    /// Presence`'s own roster-decode convention: presence is best-effort and TTL-scoped, so
    /// one bad update must not cost the actor an otherwise valid turn's patches and effects.
    async fn execute_turn(&self, inst: &mut GuestInstance, events: &[Event], budget: Budget, _identity: &mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<TurnResult, TurnFault> {
        let GuestInstanceState::Wasmtime(state) = &mut inst.state else {
            return Err(TurnFault::Trapped("execute_turn called on a non-wasmtime GuestInstance".to_string()));
        };
        let WasmtimeInstanceState { store, bindings, instance_id, deadline } = state;
        arm_store_fuel(store, self.fuel_metering, budget.fuel).map_err(|error| TurnFault::Host(PluginHostError::Wasmtime(error.to_string())))?;
        let _epoch = self.epoch.arm_original(store, deadline, budget.deadline_us)?;
        let wit_budget = wit_reactor::Budget { retained:semio_framework_plugin_host_wit::retained_turn_wire::input_to_wit(budget.retained), retained_policy:semio_framework_plugin_host_wit::retained_turn_wire::grant_to_wit(budget.retained_policy),work_fuel:budget.work_fuel, fuel: budget.fuel, deadline_us: budget.deadline_us, max_effects: budget.max_effects, max_patch_bytes: budget.max_patch_bytes, max_frames: budget.max_frames };
        let (wit_events, wit_command_page, wit_cold_pair_page) = kernel_turn_inputs_to_wit(events, *instance_id).await?;
        let call_result = store
            .run_concurrent(async |accessor| {
                if let Some((cursor, bytes)) = wit_command_page {
                    if let Err(error) = bindings.semio_framework_reactor().call_stage_command_page(accessor, cursor, bytes).await? {
                        return Ok(Err(decode_guest_plugin_error(error)));
                    }
                }
                if let Some(page) = wit_cold_pair_page {
                    if let Err(error) = bindings.semio_framework_reactor().call_stage_cold_pair_page(accessor, page).await? {
                        return Ok(Err(decode_guest_plugin_error(error)));
                    }
                }
                bindings.semio_framework_reactor().call_poll(accessor, wit_events, wit_budget).await.map(|original|original.map_err(semio_framework_plugin_host_wit::retained_turn_wire::failure_from_wit))
            })
            .await
            .and_then(|inner| inner);
        let poll_result = match call_result {
            Ok(inner) => inner,
            Err(trap) => return Err(classify_guest_trap(&trap)),
        };
        let wit_turn_result = poll_result?;
        let retained_receipt = semio_framework_plugin_host_wit::retained_turn_wire::receipt_from_wit(wit_turn_result.retained_receipt,budget.retained)?;
        let emitted: Vec<wit_effects::Effect> = std::mem::take(&mut store.data_mut().emit_sink);
        let emitted_patches: Vec<wit_ui::UiPatch> = std::mem::take(&mut store.data_mut().emit_patch_sink);
        let mut effects = Vec::with_capacity(emitted.len() + wit_turn_result.effects.len());
        for effect in emitted.into_iter().chain(wit_turn_result.effects) {
            effects.push(wit_effect_to_kernel(effect).await.map_err(TurnFault::Host)?);
        }
        let ui_patch_receipt = wit_turn_result.ui_patch_receipt.map(wit_patch_receipt_to_kernel);
        let ui_patches = semio_framework_plugin_host_wit::ui_patch::wit_ui_patches_to_kernel(*instance_id, budget.max_patch_bytes, emitted_patches, wit_turn_result.ui_patches, ui_patch_receipt).map_err(|error| TurnFault::Host(PluginHostError::Plugin(error)))?;
        Ok(TurnResult {
            retained_receipt,
            ui_patches,
            effects,
            presence: {
                let mut updates = Vec::with_capacity(wit_turn_result.presence.len());
                for entry in wit_turn_result.presence {
                    let Ok(value) = store::pack_rt::decode_wire_value(&entry.update) else { continue };
                    if let Ok(update) = serde_json::from_value::<semio_framework::kernel::PresenceUpdate>(serde_json::Value::from(&value)) {
                        updates.push(update);
                    }
                }
                updates
            },
            next_wake: wit_turn_result.next_wake,
            status: wit_turn_status_to_kernel(wit_turn_result.status).await,
            fuel_used: wit_turn_result.fuel_used,
            command_ingress: wit_command_ingress_to_kernel(wit_turn_result.command_ingress),
            cold_pair_ingress: wit_cold_ingress_to_kernel(wit_turn_result.cold_pair_ingress)?,
            lifecycle_receipt: wit_turn_result.lifecycle_receipt.map(wit_lifecycle_receipt_to_kernel),
            ui_patch_receipt,
        })
    }

    /// 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (B1 world-collapse): every export below is
    /// `async func` in the WIT now, so each goes through `Store::run_concurrent` + `Accessor` — the
    /// same shape `execute_turn` uses, and the only one wasmtime offers for an async-lifted export.
    /// The doubled `Result` is unchanged in meaning: outer = trap, inner = `plugin-error`.
    ///
    /// ⏱️ The host's own ceilings, never the guest's grant — see [`GUEST_JOB_WATCHDOG_MS`]. Armed
    /// HERE because `start-job` armed nothing at all and inherited whatever the preceding
    /// `execute_turn`/`step-job` left in the store: a start after a fully spent turn began with
    /// an already-passed epoch and no fuel.
    async fn start_job(&self,inst:&mut GuestInstance,source:Option<OriginalJobSource>,budget:OriginalJobBudget,execution:OriginalGuestExecutionBudget)->OriginalGuestJobCall<OriginalJobStartReply>{
        if let Err(error)=execution.validate_for(budget){return OriginalGuestJobCall::unsubmitted(execution,source,identity_turn_fault(error))}
        if !self.fuel_metering{return OriginalGuestJobCall::unsubmitted(execution,source,identity_turn_fault(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original job execution requires actual instruction metering")))}
        let GuestInstanceState::Wasmtime(state)=&mut inst.state else{return OriginalGuestJobCall::unsubmitted(execution,source,identity_turn_fault(semio_framework_plugin_guest_contract::operation_authority::refusal(4)))};
        let WasmtimeInstanceState{store,bindings,deadline,..}=state;
        if let Err(error)=arm_store_fuel(store,true,execution.fuel){return OriginalGuestJobCall::unsubmitted(execution,source,TurnFault::Host(PluginHostError::Wasmtime(error.to_string())))}
        let _epoch=match self.epoch.arm_original(store,deadline,execution.deadline_us){Ok(epoch)=>epoch,Err(error)=>return OriginalGuestJobCall::unsubmitted(execution,source,error.into())};
        let wit_budget=semio_framework_plugin_host_wit::budget_to_wit(budget);
        let returned=store.run_concurrent(async|accessor|bindings.semio_framework_jobs().call_start_job(accessor,source.map(semio_framework_plugin_host_wit::source_to_wit),wit_budget).await).await.and_then(|result|result).map_err(|error|classify_guest_trap(&error));
        let spent_fuel=store.get_fuel().ok().and_then(|remaining|execution.fuel.checked_sub(remaining));
        OriginalGuestJobCall{original:returned.and_then(semio_framework_plugin_host_wit::start_from_wit),execution:OriginalGuestExecutionReceipt{budget:execution,spent_fuel,submitted:true},returned_unsubmitted_source:None}
    }

    async fn step_job(&self,inst:&mut GuestInstance,job:u64,budget:OriginalJobBudget,execution:OriginalGuestExecutionBudget)->OriginalGuestJobCall<OriginalJobStepReply<OriginalJobOutcome>>{
        if let Err(error)=execution.validate_for(budget){return OriginalGuestJobCall::unsubmitted(execution,None,identity_turn_fault(error))}
        if !self.fuel_metering{return OriginalGuestJobCall::unsubmitted(execution,None,identity_turn_fault(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original job execution requires actual instruction metering")))}
        let GuestInstanceState::Wasmtime(state)=&mut inst.state else{return OriginalGuestJobCall::unsubmitted(execution,None,identity_turn_fault(semio_framework_plugin_guest_contract::operation_authority::refusal(4)))};
        let WasmtimeInstanceState{store,bindings,deadline,..}=state;
        if let Err(error)=arm_store_fuel(store,true,execution.fuel){return OriginalGuestJobCall::unsubmitted(execution,None,TurnFault::Host(PluginHostError::Wasmtime(error.to_string())))}
        let _epoch=match self.epoch.arm_original(store,deadline,execution.deadline_us){Ok(epoch)=>epoch,Err(error)=>return OriginalGuestJobCall::unsubmitted(execution,None,error.into())};
        let wit_budget=semio_framework_plugin_host_wit::budget_to_wit(budget);
        let returned=store.run_concurrent(async|accessor|bindings.semio_framework_jobs().call_step_job(accessor,job,wit_budget).await).await.and_then(|result|result).map_err(|error|classify_guest_trap(&error));
        let spent_fuel=store.get_fuel().ok().and_then(|remaining|execution.fuel.checked_sub(remaining));
        OriginalGuestJobCall{original:returned.and_then(semio_framework_plugin_host_wit::step_from_wit),execution:OriginalGuestExecutionReceipt{budget:execution,spent_fuel,submitted:true},returned_unsubmitted_source:None}
    }

    async fn read_job_output(&self,inst:&mut GuestInstance,job:u64,budget:OriginalJobBudget,execution:OriginalGuestExecutionBudget)->OriginalGuestJobCall<OriginalJobOutputReply>{
        if let Err(error)=execution.validate_for(budget){return OriginalGuestJobCall::unsubmitted(execution,None,identity_turn_fault(error))}
        if !self.fuel_metering{return OriginalGuestJobCall::unsubmitted(execution,None,identity_turn_fault(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original job execution requires actual instruction metering")))}
        let GuestInstanceState::Wasmtime(state)=&mut inst.state else{return OriginalGuestJobCall::unsubmitted(execution,None,identity_turn_fault(semio_framework_plugin_guest_contract::operation_authority::refusal(4)))};
        let WasmtimeInstanceState{store,bindings,deadline,..}=state;
        if let Err(error)=arm_store_fuel(store,true,execution.fuel){return OriginalGuestJobCall::unsubmitted(execution,None,TurnFault::Host(PluginHostError::Wasmtime(error.to_string())))}
        let _epoch=match self.epoch.arm_original(store,deadline,execution.deadline_us){Ok(epoch)=>epoch,Err(error)=>return OriginalGuestJobCall::unsubmitted(execution,None,error.into())};
        let wit_budget=semio_framework_plugin_host_wit::budget_to_wit(budget);
        let returned=store.run_concurrent(async|accessor|bindings.semio_framework_jobs().call_read_job_output(accessor,job,wit_budget).await).await.and_then(|result|result).map_err(|error|classify_guest_trap(&error));
        let spent_fuel=store.get_fuel().ok().and_then(|remaining|execution.fuel.checked_sub(remaining));
        OriginalGuestJobCall{original:returned.and_then(semio_framework_plugin_host_wit::output_from_wit),execution:OriginalGuestExecutionReceipt{budget:execution,spent_fuel,submitted:true},returned_unsubmitted_source:None}
    }

    async fn acknowledge_job_output(&self,inst:&mut GuestInstance,job:u64,budget:OriginalJobBudget,execution:OriginalGuestExecutionBudget)->OriginalGuestJobCall<OriginalJobOutputAckReply>{
        if let Err(error)=execution.validate_for(budget){return OriginalGuestJobCall::unsubmitted(execution,None,identity_turn_fault(error))}
        if !self.fuel_metering{return OriginalGuestJobCall::unsubmitted(execution,None,identity_turn_fault(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original job execution requires actual instruction metering")))}
        let GuestInstanceState::Wasmtime(state)=&mut inst.state else{return OriginalGuestJobCall::unsubmitted(execution,None,identity_turn_fault(semio_framework_plugin_guest_contract::operation_authority::refusal(4)))};
        let WasmtimeInstanceState{store,bindings,deadline,..}=state;
        if let Err(error)=arm_store_fuel(store,true,execution.fuel){return OriginalGuestJobCall::unsubmitted(execution,None,TurnFault::Host(PluginHostError::Wasmtime(error.to_string())))}
        let _epoch=match self.epoch.arm_original(store,deadline,execution.deadline_us){Ok(epoch)=>epoch,Err(error)=>return OriginalGuestJobCall::unsubmitted(execution,None,error.into())};
        let wit_budget=semio_framework_plugin_host_wit::budget_to_wit(budget);
        let returned=store.run_concurrent(async|accessor|bindings.semio_framework_jobs().call_acknowledge_job_output(accessor,job,wit_budget).await).await.and_then(|result|result).map_err(|error|classify_guest_trap(&error));
        let spent_fuel=store.get_fuel().ok().and_then(|remaining|execution.fuel.checked_sub(remaining));
        OriginalGuestJobCall{original:returned.and_then(semio_framework_plugin_host_wit::ack_from_wit),execution:OriginalGuestExecutionReceipt{budget:execution,spent_fuel,submitted:true},returned_unsubmitted_source:None}
    }

    async fn cancel_job(&self,inst:&mut GuestInstance,job:u64,budget:OriginalJobBudget,execution:OriginalGuestExecutionBudget)->OriginalGuestJobCall<OriginalJobCancelReply>{
        if let Err(error)=execution.validate_for(budget){return OriginalGuestJobCall::unsubmitted(execution,None,identity_turn_fault(error))}
        if !self.fuel_metering{return OriginalGuestJobCall::unsubmitted(execution,None,identity_turn_fault(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original job execution requires actual instruction metering")))}
        let GuestInstanceState::Wasmtime(state)=&mut inst.state else{return OriginalGuestJobCall::unsubmitted(execution,None,identity_turn_fault(semio_framework_plugin_guest_contract::operation_authority::refusal(4)))};
        let WasmtimeInstanceState{store,bindings,deadline,..}=state;
        if let Err(error)=arm_store_fuel(store,true,execution.fuel){return OriginalGuestJobCall::unsubmitted(execution,None,TurnFault::Host(PluginHostError::Wasmtime(error.to_string())))}
        let _epoch=match self.epoch.arm_original(store,deadline,execution.deadline_us){Ok(epoch)=>epoch,Err(error)=>return OriginalGuestJobCall::unsubmitted(execution,None,error.into())};
        let wit_budget=semio_framework_plugin_host_wit::budget_to_wit(budget);
        let returned=store.run_concurrent(async|accessor|bindings.semio_framework_jobs().call_cancel_job(accessor,job,wit_budget).await).await.and_then(|result|result).map_err(|error|classify_guest_trap(&error));
        let spent_fuel=store.get_fuel().ok().and_then(|remaining|execution.fuel.checked_sub(remaining));
        OriginalGuestJobCall{original:returned.and_then(semio_framework_plugin_host_wit::cancel_from_wit),execution:OriginalGuestExecutionReceipt{budget:execution,spent_fuel,submitted:true},returned_unsubmitted_source:None}
    }

    async fn close_jobs(&self,inst:&mut GuestInstance,budget:OriginalJobBudget,execution:OriginalGuestExecutionBudget)->OriginalGuestJobCall<OriginalJobCancelReply>{
        if let Err(error)=execution.validate_for(budget){return OriginalGuestJobCall::unsubmitted(execution,None,identity_turn_fault(error))}
        if !self.fuel_metering{return OriginalGuestJobCall::unsubmitted(execution,None,identity_turn_fault(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original job execution requires actual instruction metering")))}
        let GuestInstanceState::Wasmtime(state)=&mut inst.state else{return OriginalGuestJobCall::unsubmitted(execution,None,identity_turn_fault(semio_framework_plugin_guest_contract::operation_authority::refusal(4)))};
        let WasmtimeInstanceState{store,bindings,deadline,..}=state;
        if let Err(error)=arm_store_fuel(store,true,execution.fuel){return OriginalGuestJobCall::unsubmitted(execution,None,TurnFault::Host(PluginHostError::Wasmtime(error.to_string())))}
        let _epoch=match self.epoch.arm_original(store,deadline,execution.deadline_us){Ok(epoch)=>epoch,Err(error)=>return OriginalGuestJobCall::unsubmitted(execution,None,error.into())};
        let wit_budget=semio_framework_plugin_host_wit::budget_to_wit(budget);
        let returned=store.run_concurrent(async|accessor|bindings.semio_framework_jobs().call_close_jobs(accessor,wit_budget).await).await.and_then(|result|result).map_err(|error|classify_guest_trap(&error));
        let spent_fuel=store.get_fuel().ok().and_then(|remaining|execution.fuel.checked_sub(remaining));
        OriginalGuestJobCall{original:returned.and_then(semio_framework_plugin_host_wit::cancel_from_wit),execution:OriginalGuestExecutionReceipt{budget:execution,spent_fuel,submitted:true},returned_unsubmitted_source:None}
    }

    async fn checkpoint(&self, inst: &mut GuestInstance, _identity: &mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<Vec<u8>, PluginHostError> {
        let GuestInstanceState::Wasmtime(state) = &mut inst.state else {
            return Err(PluginHostError::Plugin("checkpoint called on a non-wasmtime GuestInstance".to_string()));
        };
        let WasmtimeInstanceState { store, bindings, deadline, .. } = state;
        let _epoch = self.arm_guest_watchdog(store, deadline)?;
        store
            .run_concurrent(async |accessor| bindings.semio_framework_checkpoint().call_checkpoint(accessor).await)
            .await
            .map_err(|error| PluginHostError::Wasmtime(error.to_string()))?
            .map_err(|error| PluginHostError::Wasmtime(error.to_string()))?
            .map_err(|error| PluginHostError::Plugin(format!("{error:?}")))
    }

    async fn restore(&self, inst: &mut GuestInstance, state_bytes: &[u8], _identity: &mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), PluginHostError> {
        let GuestInstanceState::Wasmtime(state) = &mut inst.state else {
            return Err(PluginHostError::Plugin("restore called on a non-wasmtime GuestInstance".to_string()));
        };
        let WasmtimeInstanceState { store, bindings, deadline, .. } = state;
        let _epoch = self.arm_guest_watchdog(store, deadline)?;
        let state_bytes = state_bytes.to_vec();
        store
            .run_concurrent(async |accessor| bindings.semio_framework_checkpoint().call_restore(accessor, state_bytes).await)
            .await
            .map_err(|error| PluginHostError::Wasmtime(error.to_string()))?
            .map_err(|error| PluginHostError::Wasmtime(error.to_string()))?
            .map_err(|error| PluginHostError::Plugin(format!("{error:?}")))
    }

    /// 🗑️ `Store<ActorHostState>` and its `Component` `Arc` drop with `_inst` — nothing else to
    /// release; the pooling allocator reclaims the instance's slab on `Store` drop.
    async fn drop_instance(&self, _inst: GuestInstance) {
    }
}
//#endregion 🐎️WasmtimeRuntime
//#endregion 🎭️GuestRuntime
