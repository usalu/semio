//! 🧩️ Instances of the plugin artifact channel: instance discard, open and scoped open, the real exchange entry and the sequence counter. See [`super`].

use super::*;
use super::native_poll::{close_poll, open_poll, poll_turn, PollPurpose};
use semio_framework::kernel::{ActorInstanceLifecycleAck, ActorInstanceLifecycleReceipt, ActorInstanceOpenRequest, Event, TurnStatus};

enum InstanceOpenProgress {
    WaitForCapture,
    Acknowledge(ActorInstanceLifecycleReceipt),
    Live,
}

/// 🎟️ A completed ACK commits the captured lifetime before the guest publishes its turn status.
fn instance_open_progress(request: ActorInstanceOpenRequest, acknowledging: bool, status: &TurnStatus, receipt: Option<ActorInstanceLifecycleReceipt>) -> Result<InstanceOpenProgress, &'static str> {
    if !matches!(status, TurnStatus::Idle | TurnStatus::MoreWork) {
        return Err("unexpected instance opening turn status");
    }
    match (acknowledging, receipt) {
        (true, None) => Ok(InstanceOpenProgress::Live),
        (true, Some(_)) => Err("instance lifecycle acknowledgment returned a receipt"),
        (false, Some(receipt @ ActorInstanceLifecycleReceipt::Captured { lifetime, request_sequence }))
            if lifetime.activation_generation == request.activation_generation
                && lifetime.instance_id == request.instance_id
                && lifetime.guest_lifetime != 0
                && request_sequence == request.request_sequence => Ok(InstanceOpenProgress::Acknowledge(receipt)),
        (false, Some(_)) => Err("instance lifecycle receipt does not match its open request"),
        (false, None) if matches!(status, TurnStatus::MoreWork) => Ok(InstanceOpenProgress::WaitForCapture),
        (false, None) => Err("instance became idle before publishing its captured receipt"),
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

//#region 🔖️ArtifactChannel
#[cfg(not(target_arch = "wasm32"))]
impl PluginArtifactChannel {
    /// ♻️ Returns the guest to the original Source that opened it.
    pub(super) fn discard_instance(&mut self, instance: u32) {
        if let Some(guest) = self.instances.get_mut(&instance) {
            self.closing_instances.insert(instance);
            if !close_poll(guest) { return; }
            self.instances.remove(&instance);
            self.closing_instances.remove(&instance);
            self.session_documents.remove(&instance);
            self.backbone_bindings.remove(&instance);
        }
    }

    /// ♻️ Preserves the original cancellation and any guest Source that failed to retire.
    pub(crate) fn settle_instance_cancellation(&mut self, instance: u32, cancellation: Fault) -> Fault {
        if !self.closing_instances.contains(&instance) { self.discard_instance(instance); }
        if let Some(guest) = self.instances.get(&instance) {
            return Self::not_wired("cancelled command still has a guest Source", format!("{}; {:?}", cancellation.message, guest.cleanup_failure().or_else(|| guest.failure())));
        }
        cancellation
    }

    /// 🎬️ Opens the dynamic component and acknowledges its genuine lifecycle receipts.
    pub(crate) fn ensure_instance(&mut self, instance: u32) -> Result<(), Fault> {
        self.ensure_instance_scoped(instance, &ActivationScope::detached())
    }

    /// 🎟️ Uses the actual caller scope and source-issued phased startup.
    pub(crate) fn ensure_instance_scoped(&mut self, instance: u32, scope: &ActivationScope) -> Result<(), Fault> {
        self.ensure_instance_with_source_scope(instance, scope, scope)
    }

    /// 🗃️ Opens an export guest whose Source remains funded for archive cancellation and receipt closure.
    pub(crate) fn ensure_export_instance(&mut self, instance: u32, scope: &ActivationScope) -> Result<(), Fault> {
        self.ensure_instance_with_source_scope(instance, scope, &ActivationScope::detached())
    }

    fn ensure_instance_with_source_scope(&mut self, instance: u32, scope: &ActivationScope, source_scope: &ActivationScope) -> Result<(), Fault> {
        if self.closing_instances.contains(&instance) { return Err(Self::not_wired("instance retirement", "previous instance retains its original Source")); }
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
        let assets = find_repo_root()
            .map_err(|error| Fault { code: "package-assets.root".to_string(), message: error.to_string() })
            .and_then(|root| semio_plugin_host::package_assets::package_file_assets(&root, &self.descriptor.assets).map_err(|message| Fault { code: "package-assets.refused".to_string(), message }))?;
        let origin = crate::compiled_components::native_instance_origin(self.compiled.package_hash).map_err(|error| Self::not_wired("guest poll origin", format!("{error:?}")))?;
        let mut guest = self.issue_native_poll(actor, origin, source_scope)?;
        let request = ActorInstanceOpenRequest { activation_generation: 1, instance_id: instance, request_sequence: 1 };
        let mut owed = Some(Event::InstanceOpen {
            purpose: semio_framework::kernel::ActorInstanceOpenPurpose::Ordinary,
            request,
            app_id: semio_framework::kernel::AppInstanceId(self.app_ref.app_id.clone()),
            actor: self.actor_label.clone(),
            config: Vec::new(),
            assets,
            capabilities: caps,
            quotas: self.descriptor.quotas.clone(),
        });
        let deadline = std::time::Instant::now() + INSTANCE_OPEN_WALL_BUDGET;
        let opened = open_poll(&mut guest, scope, deadline).and_then(|()| {
            let mut completed_turns = 0_u64;
            loop {
                let acknowledging = matches!(owed, Some(Event::InstanceLifecycleAck(_)));
                let progress = poll_turn(&mut guest, PollPurpose::Lifecycle, owed.as_slice(), scope, deadline, |turn| {
                    instance_open_progress(request, acknowledging, &turn.status, turn.lifecycle_receipt)
                        .map_err(|message| Self::not_wired("instance opening", message))
                }).map_err(|mut fault| {
                    let phase = match owed.as_ref() {
                        Some(Event::InstanceOpen { .. }) => "opening request",
                        Some(_) => "lifecycle acknowledgment",
                        None => "waiting for capture",
                    };
                    fault.message = format!("instance {instance} {phase} after {completed_turns} completed turns: {}", fault.message);
                    fault
                })?;
                completed_turns += 1;
                owed = match progress {
                    InstanceOpenProgress::WaitForCapture => None,
                    InstanceOpenProgress::Acknowledge(receipt) => Some(Event::InstanceLifecycleAck(ActorInstanceLifecycleAck { receipt })),
                    InstanceOpenProgress::Live => return Ok(()),
                };
                if std::time::Instant::now() >= deadline { return Err(Self::budget_fault("InstanceOpen")); }
            }
        });
        self.instances.insert(instance, guest);
        if opened.is_err() { self.discard_instance(instance); }
        opened
    }

    /// 🔁️ Drives one `AppCommand` to its terminal frame. The command becomes a retained host batch [Design notes](README.md#source-notes-109).
    pub(crate) fn exchange_one_real(&mut self, instance: u32, real_command: store::AppCommand) -> Result<store::AppFrame, Fault> {
        self.exchange_one_real_scoped(instance, real_command, &ActivationScope::detached())
    }

    pub(crate) fn exchange_one_real_scoped(&mut self, instance: u32, mut real_command: store::AppCommand, scope: &ActivationScope) -> Result<store::AppFrame, Fault> {
        let seq = self.take_seq();
        *app_command_seq_mut(&mut real_command) = seq;
        self.exchange_sequenced_scoped(instance, &real_command, seq, scope)
    }
}
//#endregion 🔖️ArtifactChannel
