//! 📂️ Documents and backbone of the plugin artifact channel: construction, backbone egress, the session document load, reseed and the receive path. See [`super`].

use super::*;

//#region 🔖️ArtifactChannel
#[cfg(not(target_arch = "wasm32"))]
impl PluginArtifactChannel {
    fn command_closes_terminal_is_empty(&self) -> bool {
        self.pending_command_closes.terminal_is_empty()
    }

    pub fn new(repo_root: PathBuf, entry: PluginRegistryEntry, descriptor: semio_framework::PackageDescriptor, app_ref: semio_framework::AppRef, actor_label: String, host_driver_policy:semio_framework_plugin_host::GuestRelayDriverPolicy) -> Result<Self, GatewayError> {
        let wasm_path = resolve_plugin_wasm_path(&repo_root, &entry)?;
        let runtime = shared_plugin_runtime()?;
        let compiled = scoped_compiled_component(runtime.as_ref(), &entry.plugin_id, ComponentBytes::File(&wasm_path), &ActivationScope::detached())?;
        Ok(Self::from_compiled(runtime, compiled, entry.plugin_id, descriptor, app_ref, actor_label, host_driver_policy))
    }

    /// 🧩️ Opens a channel over component bytes the caller already holds, whatever their provenance —
    /// a repo build read from disk, or the execution-target component a Hub authorized and served.
    /// Everything past the bytes is identical, which is exactly the point: a hub-bound agent's guest
    /// is the SAME guest a local one gets, not a second code path with its own behaviour.
    pub fn from_component(bytes: &[u8], plugin_id: String, descriptor: semio_framework::PackageDescriptor, app_ref: semio_framework::AppRef, actor_label: String, host_driver_policy:semio_framework_plugin_host::GuestRelayDriverPolicy) -> Result<Self, GatewayError> {
        let runtime = shared_plugin_runtime()?;
        let compiled = shared_compiled_component(runtime.as_ref(), &plugin_id, bytes)?;
        Ok(Self::from_compiled(runtime, compiled, plugin_id, descriptor, app_ref, actor_label, host_driver_policy))
    }

    /// 🧩️ A channel over a component this process already compiled — see [`scoped_compiled_component`].
    pub fn from_compiled(runtime: Arc<GuestRuntimes>, compiled: semio_framework_plugin_host::CompiledHandle, plugin_id: String, descriptor: semio_framework::PackageDescriptor, app_ref: semio_framework::AppRef, actor_label: String, host_driver_policy:semio_framework_plugin_host::GuestRelayDriverPolicy) -> Self {
        Self {
            runtime,
            compiled,
            plugin_id,
            descriptor,
            app_ref,
            actor_label,
            host_driver_policy,
            host_driver:Arc::new(semio_framework_plugin_host::GuestRelayWakeDriver::new(host_driver_policy,semio_framework_job::default_now_us,Box::new(||false)).expect("headless wake driver admission")),
            instances: HashMap::new(),
            pending_exchanges: PendingExchangeRegistry::new(),
            pending_command_closes: semio_framework::kernel::CommandDriverRegistry::new(),
            rejected_command_builds: semio_framework::kernel::RejectedCommandBuildRegistry::new(),
            next_seq: 1,
            session_artifact_id: None,
            session_documents: HashMap::new(),
            backbone_egress: Vec::new(),
            backbone_bindings: std::collections::BTreeSet::new(),
            inference: None,
        }
    }

    /// 🧵️ Takes every document-backbone message this guest has published since the last drain. The
    /// caller owns the relay: this channel knows the plugin, not the document actor.
    pub fn drain_backbone_egress(&mut self) -> Vec<Vec<u8>> {
        std::mem::take(&mut self.backbone_egress)
    }

    /// 🧵️ Binds this guest instance's DOCUMENT BACKBONE to this channel's own actor uri, once.
    ///
    /// 🐛️ A guest publishes `Effect::SendMessage { target: Backbone { uri } }` **only for an
    /// instance a host has explicitly bound** (`🔌️plugin/🦀️.rs`'s
    /// `plugin_handle_document_backbone_binding` → `document_backbone_effects`). Without this
    /// command the agent's committed transaction is real inside the guest — `action_invoke` answers
    /// `SUCCEEDED` with a moved `headEditId` — and its envelopes are published to nobody, so the hub
    /// ledger stays at `head_seq 0`. Measured live on hub 7681, 2026-09-22
    /// (`🗑️generated/m10-7681-note-status-after.txt`): an invoke that succeeded against a document
    /// whose ledger never moved. A silent success is the worst shape this lane can have, which is
    /// why the bind is a hard precondition of dispatch rather than a best-effort extra.
    ///
    /// 🪢 The uri is [`Self::actor_label`]'s value, which is the SAME string
    /// [`document_backbone_payload`] fences egress on — bind and relay cannot drift apart because
    /// they read one field.
    ///
    /// 🧭️ Ordering: the guest refuses a bind-then-load
    /// (`plugin.document-backbone.load-while-bound`), so this runs AFTER
    /// [`Self::load_session_document`], never before.
    ///
    /// 🧵️ A bind turn can already carry egress — the guest flushes whatever its store had
    /// pending onto the freshly bound backbone — so it is captured here exactly as a command
    /// turn's is, rather than left in the turn and lost.
    pub fn ensure_document_backbone(&mut self, instance: u32) -> Result<(), Fault> {
        if self.backbone_bindings.contains(&instance) {
            return Ok(());
        }
        self.ensure_instance(instance)?;
        let command = semio_framework_plugin::document_backbone_binding::DocumentBackboneBindingCommandV1 {
            operation: semio_framework_plugin::document_backbone_binding::DocumentBackboneBindingOperationV1::Bind,
            instance_id: instance,
            binding_generation: DOCUMENT_BACKBONE_BINDING_GENERATION,
            uri: self.actor_label.clone(),
        };
        let payload = command.encode().map_err(|error| Self::not_wired("document-backbone bind", error))?;
        let event = semio_framework::kernel::Event::Message { source: semio_framework::kernel::MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId(instance.to_string()) }, payload };
        let runtime = Arc::clone(&self.runtime);
        let guest = self.instances.get_mut(&instance).ok_or_else(|| Self::not_wired("document-backbone bind", format!("no open instance {instance}")))?;
        let turn = with_headless_identity(|identity| semio_framework_async::block_on(runtime.execute_turn(guest, std::slice::from_ref(&event), headless_command_budget(), identity))).map_err(|error| Self::not_wired("document-backbone bind", error))?;
        let receipts: Vec<&Vec<u8>> = turn
            .effects
            .iter()
            .filter_map(|effect| match effect {
                semio_framework::kernel::Effect::SendMessage { target: semio_framework::kernel::MessageEndpoint::Shell { instance: target }, payload } if target.0 == instance.to_string() => Some(payload),
                _ => None,
            })
            .collect();
        if receipts.len() != 1 {
            return Err(Self::not_wired("document-backbone bind", format!("the guest answered {} shell receipt(s) for one bind", receipts.len())));
        }
        semio_framework_plugin::document_backbone_binding::require_document_backbone_binding_receipt_v1(receipts[0], &command).map_err(|error| Self::not_wired("document-backbone bind", error))?;
        for effect in &turn.effects {
            if let Some(egress) = document_backbone_payload(&self.actor_label, effect) {
                self.backbone_egress.push(egress.to_vec());
            }
        }
        self.backbone_bindings.insert(instance);
        Ok(())
    }

    /// 🗿️ Names the artifact this channel's guest session document is, for the stamps it answers.
    /// `None` means the owning workspace has bound no artifact to this plugin, or more than one —
    /// see [`Self::session_artifact_id`].
    pub fn bind_session_artifact(&mut self, artifact_id: Option<String>) {
        self.session_artifact_id = artifact_id;
    }

    /// 📥️ Makes `artifact_id`'s canonical pair this guest's session document — one stepped archive
    /// load ([`Self::load_document_archive`]), the same whole-document load every host and this file's
    /// `ExportMedia` arm drive, so nothing about the bytes is interpreted here. `scope` hears its progress
    /// and may cancel it; a cancelled or faulted load leaves the guest's previous document as it was and
    /// records nothing.
    ///
    /// 🧭️ Idempotent per guest: a channel that already holds this artifact's pair does nothing, so
    /// the load happens once per (instance, artifact) and never once per command — re-loading on
    /// every exchange would discard the guest's own uncommitted transaction state between
    /// `TransactionPrepare` and `TransactionCommit`, which is the whole two-phase contract.
    /// [`Self::discard_instance`] forgets it, because a thrown-away guest holds nothing.
    ///
    /// 🔀️ A guest that holds ANOTHER artifact (its app's session document moved to a document the agent opened since,
    /// [`route_session_artifact`]) is thrown away first: a guest is only ever seeded fresh, never while its backbone is
    /// bound to a different document.
    pub fn load_session_document(&mut self, instance: u32, artifact_id: &str, pack: &[u8], spr: &[u8], scope: &ActivationScope) -> Result<(), Fault> {
        match self.session_documents.get(&instance) {
            Some(loaded) if loaded == artifact_id => return Ok(()),
            Some(_) => self.discard_instance(instance),
            None => {}
        }
        self.ensure_instance_scoped(instance, scope)?;
        self.load_document_archive(instance, pack, spr, scope)?;
        self.session_documents.insert(instance, artifact_id.to_string());
        Ok(())
    }

    /// 🗃️ Makes `pack`/`spr` `instance`'s whole document through the stepped, ACK-owned archive load
    /// (`📓️api-stepped-document-load.md` §2, driven by `store::DocumentArchiveLoadHost`): `LoadDocumentArchive`
    /// with no members, `PollDocumentArchiveLoad` until a terminal status, then `AcknowledgeDocumentArchiveLoad`,
    /// each one bounded command exchange, so a long history folds across many guest turns instead of inside one.
    ///
    /// 📊️ Every polled status reaches `scope` as [`ActivationPhase::LoadingDocument`] at `completed / total`.
    ///
    /// 🛑️ A cancel `scope` reports, or [`DOCUMENT_LOAD_WALL_BUDGET`] running out, cancels the load in the guest and
    /// keeps polling until the guest says it restored the previous document with zero trace; the caller gets
    /// `job.cancelled` or `budget.exceeded`. A load that won that race is a loaded document. A guest that does not
    /// settle its cancel within one more [`COMMAND_RESUME_WALL_BUDGET`] is discarded, so the next command starts on
    /// a fresh instance.
    ///
    /// ⚠️ A guest fault, terminal or refused, is the guest's own typed fault with its own code (`document.loading`, …).
    pub(crate) fn load_document_archive(&mut self, instance: u32, pack: &[u8], spr: &[u8], scope: &ActivationScope) -> Result<(), Fault> {
        let mut load = store::DocumentArchiveLoadHost::new(store::DocumentArchivePack { parent_pack: pack.to_vec(), parent_spr: spr.to_vec(), members: Vec::new() });
        let deadline = std::time::Instant::now() + DOCUMENT_LOAD_WALL_BUDGET;
        let mut stopped: Option<Fault> = None;
        loop {
            let now = std::time::Instant::now();
            if stopped.is_none() {
                stopped = scope.checkpoint().err().or_else(|| (now >= deadline).then(|| Self::budget_fault("the document load")));
                if stopped.is_some() {
                    load.request_cancel();
                }
            } else if now >= deadline + COMMAND_RESUME_WALL_BUDGET {
                self.discard_instance(instance);
                return Err(Fault { code: "budget.exceeded".to_string(), message: "the document load did not settle its cancel within its wall budget; the instance was discarded and the next command starts on a fresh one".to_string() });
            }
            let (seq, command) = match load.step(|| self.take_seq()) {
                store::DocumentArchiveLoadStep::Send { seq, command } => (seq, command),
                store::DocumentArchiveLoadStep::Finished(store::DocumentArchiveLoadOutcome::Ready) => return Ok(()),
                store::DocumentArchiveLoadStep::Finished(store::DocumentArchiveLoadOutcome::Cancelled) => {
                    return Err(stopped.unwrap_or_else(|| Fault { code: ACTIVATION_CANCELLED_FAULT_CODE.to_string(), message: "the guest cancelled the document load; its previous document is unchanged".to_string() }));
                }
                store::DocumentArchiveLoadStep::Finished(store::DocumentArchiveLoadOutcome::Fault(fault)) => return Err(decode_guest_fault(&fault)),
            };
            let frame = self.exchange_sequenced(instance, &command, seq)?;
            match load.answer(seq, &frame) {
                Ok(Some(status)) => {
                    let fraction = if status.total == 0 { 0.0 } else { status.completed as f64 / status.total as f64 };
                    if let (Err(cancelled), true) = (scope.advance(ActivationPhase::LoadingDocument, fraction), stopped.is_none()) {
                        stopped = Some(cancelled);
                        load.request_cancel();
                    }
                }
                Ok(None) => {}
                Err(store::DocumentArchiveLoadRefusal::Refused(fault)) => return Err(decode_guest_fault(&fault)),
                Err(store::DocumentArchiveLoadRefusal::Unanswered) => return Err(Self::not_wired("document archive load", format!("the guest answered seq {seq} with {}", app_frame_tag(&frame)))),
            }
        }
    }

    /// 🔁️ Throws this guest's session document away so the next [`Self::load_session_document`] seeds
    /// it from a new baseline — a guest can only be reseeded fresh, never while its backbone is bound.
    pub fn reseed_session_document(&mut self, instance: u32) {
        self.discard_instance(instance);
    }

    /// 📥️ Hands one mutation batch the hub document's actor delivered to this guest's bound document
    /// backbone — the same `Event::Message { source: Backbone }` turn the wgpu and React shells drive
    /// (`ProgramBridge::receive_document_backbone`) — and drives the guest until it is idle. Whatever it
    /// publishes back onto the backbone is kept for the relay, and an `AppFrame::Error` it answers on
    /// its shell lane is the refusal it is.
    pub fn receive_document_backbone(&mut self, instance: u32, message: Vec<u8>) -> Result<(), Fault> {
        store::os_store::backbone::decode_hot_backbone_message_exact(&message).map_err(|error| Self::not_wired("document-backbone ingress", error))?;
        self.ensure_instance(instance)?;
        let mut owed = vec![semio_framework::kernel::Event::Message { source: semio_framework::kernel::MessageEndpoint::Backbone { uri: self.actor_label.clone() }, payload: message }];
        let deadline = std::time::Instant::now() + COMMAND_RESUME_WALL_BUDGET;
        loop {
            let events = std::mem::take(&mut owed);
            let runtime = Arc::clone(&self.runtime);
            let guest = self.instances.get_mut(&instance).ok_or_else(|| Self::not_wired("document-backbone ingress", format!("no open instance {instance}")))?;
            let turn = match with_headless_identity(|identity| semio_framework_async::block_on(runtime.execute_turn(guest, &events, headless_command_budget(), identity))) {
                Ok(turn) => turn,
                Err(error) => {
                    self.discard_instance(instance);
                    return Err(Self::not_wired("document-backbone ingress", format!("{error}; the instance was discarded")));
                }
            };
            for effect in &turn.effects {
                if let Some(egress) = document_backbone_payload(&self.actor_label, effect) {
                    self.backbone_egress.push(egress.to_vec());
                }
                if let Some(fault) = shell_lane_fault(instance, effect) {
                    return Err(fault);
                }
            }
            if let Some(receipt) = turn.lifecycle_receipt {
                owed.push(semio_framework::kernel::Event::InstanceLifecycleAck(semio_framework::kernel::ActorInstanceLifecycleAck { receipt }));
            }
            match turn.status {
                semio_framework::kernel::TurnStatus::Idle if owed.is_empty() => return Ok(()),
                semio_framework::kernel::TurnStatus::Faulted(detail) => return Err(decode_guest_fault(&detail)),
                _ if owed.is_empty() => owed.push(semio_framework::kernel::Event::Wake),
                _ => {}
            }
            if std::time::Instant::now() >= deadline {
                return Err(Self::budget_fault("document-backbone ingress"));
            }
        }
    }

    /// 🗿️ What a `RevisionStamp` from this channel names. A channel with no bound artifact answers
    /// the plugin-scoped session pseudo-id `plugin:<id>` — which is what the document IS in that
    /// case, a plugin's genesis session no client can address — never the bare plugin id, which a
    /// client would read as an artifact id and pass straight back into `artifact_snapshot`.
    pub(crate) fn stamped_artifact_id(&self) -> String {
        self.session_artifact_id.clone().unwrap_or_else(|| format!("plugin:{}", self.plugin_id))
    }
}
//#endregion 🔖️ArtifactChannel
