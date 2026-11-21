//! 🧩️ Instances of the plugin artifact channel: instance discard, open and scoped open, the real exchange entry and the sequence counter. See [`super`].

use super::*;
use super::native_poll::{close_poll, open_poll, poll_turn};

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

    /// 🎬️ Opens the dynamic component and acknowledges its genuine lifecycle receipts.
    pub(crate) fn ensure_instance(&mut self, instance: u32) -> Result<(), Fault> {
        self.ensure_instance_scoped(instance, &ActivationScope::detached())
    }

    /// 🎟️ Uses the actual caller scope and source-issued phased startup.
    pub(crate) fn ensure_instance_scoped(&mut self, instance: u32, scope: &ActivationScope) -> Result<(), Fault> {
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
        let mut guest = self.issue_native_poll(actor, origin, scope)?;
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
        let opened = open_poll(&mut guest, scope, deadline).and_then(|()| {
            loop {
                let (receipt, idle) = poll_turn(&mut guest, &owed, scope, deadline, |turn| Ok((turn.lifecycle_receipt, matches!(turn.status, semio_framework::kernel::TurnStatus::Idle))))?;
                owed.clear();
                if let Some(receipt) = receipt { owed.push(semio_framework::kernel::Event::InstanceLifecycleAck(semio_framework::kernel::ActorInstanceLifecycleAck { receipt })); }
                if idle && owed.is_empty() { return Ok(()); }
                if std::time::Instant::now() >= deadline { return Err(Self::budget_fault("InstanceOpen")); }
            }
        });
        self.instances.insert(instance, guest);
        if opened.is_err() { self.discard_instance(instance); }
        opened
    }

    /// 🔁️ Drives one `AppCommand` to its terminal frame. The command becomes a retained host batch [Design notes](README.md#source-notes-109).
    pub(crate) fn exchange_one_real(&mut self, instance: u32, mut real_command: store::AppCommand) -> Result<store::AppFrame, Fault> {
        let seq = self.take_seq();
        *app_command_seq_mut(&mut real_command) = seq;
        self.exchange_sequenced(instance, &real_command, seq)
    }
}
//#endregion 🔖️ArtifactChannel
