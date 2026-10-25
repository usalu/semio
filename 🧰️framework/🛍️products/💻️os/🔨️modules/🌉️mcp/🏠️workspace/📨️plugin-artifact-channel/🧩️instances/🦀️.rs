//! 🧩️ Instances of the plugin artifact channel: instance discard, open and scoped open, the real exchange entry and the sequence counter. See [`super`].

use super::*;

//#region 🔖️ArtifactChannel
#[cfg(not(target_arch = "wasm32"))]
impl PluginArtifactChannel {
    /// ☠️ Throws an instance away. A turn cut by an epoch deadline or a trap leaves the component
    /// instance mid-call — wasmtime answers `cannot enter component instance` for every later call —
    /// so there is nothing to resume and nothing to salvage; the next `ensure_instance` builds a
    /// fresh one. `drop_instance` is driven rather than relying on `Drop` because it is what the
    /// `GuestRuntime` contract names for closing an instance, and under wasmtime it is exactly the
    /// `Store` release the pooling allocator needs to reclaim the slab.
    pub(super) fn discard_instance(&mut self, instance: u32) {
        self.session_documents.remove(&instance);
        self.backbone_bindings.remove(&instance);
        if let Some(guest) = self.instances.remove(&instance) {
            semio_framework_async::block_on(self.runtime.drop_instance(guest));
        }
    }

    /// 🎬️ Opens `instance`'s guest and drives it to settle: `Event::InstanceOpen`, then the
    /// `InstanceLifecycleAck` for whatever receipt that open published, until the guest reports
    /// `Idle` with nothing owed. Bounded by [`INSTANCE_OPEN_WALL_BUDGET`], with each turn bounded by
    /// [`headless_open_budget`].
    ///
    /// 🐎️ The mid-flight pump this replaces (slices, an `undelivered` queue, a `turn_in_flight`
    /// guard) existed because an OWNED-interpreter turn can yield mid-call and refuse new events
    /// until it settles. A compiled turn is atomic — it returns a `TurnResult` or it faults — so the
    /// queue has nothing to hold and the guard nothing to guard. What survives the move is the part
    /// that was never about the runtime: the acknowledgement below.
    ///
    /// 🧾️ The guest's `ActorInstanceLifecycleReceipt` is acknowledged here. An open the host never
    /// acknowledges is retained state the guest re-offers on every later turn — `🖥️host`'s own
    /// `🔬️poll-turn-memory` law measures exactly that — so without this the instance never reaches
    /// `Idle` and every command afterwards pays for an open that never finished.
    ///
    /// ♻️ The guest's own retained lifecycle-deadline verdict is runtime-independent too: it is the
    /// guest saying "I kept my progress, ask me again", so the events it did not consume are
    /// re-offered on the next turn rather than lost.
    pub(crate) fn ensure_instance(&mut self, instance: u32) -> Result<(), Fault> {
        self.ensure_instance_scoped(instance, &ActivationScope::detached())
    }

    /// 🎬️ [`Self::ensure_instance`] under a caller's progress and cancel: a cancel observed between
    /// turns throws the half-open guest away, so the next open starts clean.
    ///
    /// 🎟️ The grants are handed to `instantiate` — the store's capability table is built with the
    /// instance — and travel in `Event::InstanceOpen` as well, which is where the guest reads them.
    ///
    /// ⏱️ A compiled turn cut by its own ceiling is not a yield to resume from: the
    /// instance is mid-call and unusable. Report it as the bounded budget fault it is,
    /// rather than as a wiring fault, and throw the half-open instance away.
    pub(crate) fn ensure_instance_scoped(&mut self, instance: u32, scope: &ActivationScope) -> Result<(), Fault> {
        if self.instances.contains_key(&instance) {
            return Ok(());
        }
        scope.enter(ActivationPhase::OpeningGuest)?;
        let actor = ::semio_framework_async::poll::resolve_ready(semio_framework_actor::ActorId::new((instance as u16).wrapping_add(1), 0, 1, 0));
        let caps: Vec<semio_framework::kernel::BrokerCapabilityGrant> = self
            .descriptor
            .capability_requests
            .iter()
            .map(|request| semio_framework::kernel::BrokerCapabilityGrant { token: semio_framework::CapabilityToken(0), id: request.id.clone(), scope: request.scope.clone(), expires_ms: None })
            .collect();
        let budget = headless_open_budget();
        let mut guest = semio_framework_async::block_on(self.runtime.instantiate(&self.compiled, actor, &caps, &budget)).map_err(|error| match error {
            semio_framework_plugin_guest_contract::PluginHostError::Refused(fault) => Fault { code: fault.code.0.clone(), message: fault.message.clone() },
            other => Self::not_wired("instantiate", other),
        })?;
        let assets = find_repo_root()
            .map_err(|error| Fault { code: "package-assets.root".to_string(), message: error.to_string() })
            .and_then(|root| semio_framework_plugin_host::package_assets::package_file_assets(&root, &self.descriptor.assets).map_err(|message| Fault { code: "package-assets.refused".to_string(), message }))?;
        let mut owed = vec![semio_framework::kernel::Event::InstanceOpen {
            request: semio_framework::kernel::ActorInstanceOpenRequest { activation_generation: 1, instance_id: instance, request_sequence: 1 },
            app_id: semio_framework::kernel::AppInstanceId(self.app_ref.app_id.clone()),
            actor: self.actor_label.clone(),
            config: Vec::new(),
            assets,
            capabilities: caps,
            quotas: self.descriptor.quotas.clone(),
        }];
        let deadline = std::time::Instant::now() + INSTANCE_OPEN_WALL_BUDGET;
        loop {
            let events = std::mem::take(&mut owed);
            match with_headless_identity(|identity| semio_framework_async::block_on(self.runtime.execute_turn(&mut guest, &events, headless_open_budget(), identity))) {
                Ok(turn) => {
                    if let Some(receipt) = turn.lifecycle_receipt {
                        owed.push(semio_framework::kernel::Event::InstanceLifecycleAck(semio_framework::kernel::ActorInstanceLifecycleAck { receipt }));
                    }
                    if matches!(turn.status, semio_framework::kernel::TurnStatus::Idle) && owed.is_empty() {
                        self.instances.insert(instance, guest);
                        return Ok(());
                    }
                }
                Err(error) if semio_framework_plugin_host::retryable_lifecycle_turn(&error, &events) => {
                    owed.splice(0..0, events);
                }
                Err(error @ (semio_framework_plugin_guest_contract::TurnFault::DeadlineExceeded | semio_framework_plugin_guest_contract::TurnFault::FuelExhausted)) => {
                    semio_framework_async::block_on(self.runtime.drop_instance(guest));
                    return Err(Fault { code: "budget.exceeded".to_string(), message: format!("InstanceOpen exceeded one turn's ceiling ({error}); the guest never came up") });
                }
                Err(error) => {
                    semio_framework_async::block_on(self.runtime.drop_instance(guest));
                    return Err(Self::not_wired("InstanceOpen", error));
                }
            }
            if let Err(cancelled) = scope.checkpoint() {
                semio_framework_async::block_on(self.runtime.drop_instance(guest));
                return Err(cancelled);
            }
            if std::time::Instant::now() >= deadline {
                semio_framework_async::block_on(self.runtime.drop_instance(guest));
                return Err(Self::budget_fault("InstanceOpen"));
            }
        }
    }

    /// 🔁️ Drives one `AppCommand` to its terminal frame. The command becomes a retained host batch
    /// whose exact current page is handed to one guest turn; the next page stays untouched until the
    /// guest acknowledges the one before it, so a command of N pages is N+1 turns. An interactive
    /// host takes those turns off its own frame loop; a headless gateway has no next frame, so it
    /// takes them here, bounded by [`COMMAND_RESUME_WALL_BUDGET`] and never unbounded.
    ///
    /// 🔢️ ONE sequence for the whole command, stamped into the command itself before it is
    /// encoded. The kernel envelope and the `AppCommand` must carry the same number: the guest
    /// answers with an `AppFrame` whose `in_reply_to` is the COMMAND's `seq`. Every call site in
    /// this file used to pass a literal `seq: 0` while the envelope counted from 1, so even a host
    /// reading the right lane would have correlated nothing.
    pub(crate) fn exchange_one_real(&mut self, instance: u32, mut real_command: store::AppCommand) -> Result<store::AppFrame, Fault> {
        let seq = self.take_seq();
        *app_command_seq_mut(&mut real_command) = seq;
        self.exchange_sequenced(instance, &real_command, seq)
    }
}
//#endregion 🔖️ArtifactChannel
