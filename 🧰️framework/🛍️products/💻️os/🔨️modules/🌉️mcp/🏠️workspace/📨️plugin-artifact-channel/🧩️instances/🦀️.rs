//! 🧩️ Instances of the plugin artifact channel: instance discard, open and scoped open, the real exchange entry and the sequence counter. See [`super`].

use super::*;

//#region 🔖️ArtifactChannel
#[cfg(not(target_arch = "wasm32"))]
impl PluginArtifactChannel {
    /// ♻️ Retires the actual instance on its lifetime source and retains every refused owner.
    pub(super) fn discard_instance(&mut self, instance: u32) {
        if !self.instances.contains_key(&instance){return}
        self.closing_instances.insert(instance);
        let deadline=std::time::Instant::now()+INSTANCE_OPEN_WALL_BUDGET;
        loop{
            let runtime=Arc::clone(&self.runtime);
            let guest=self.instances.get_mut(&instance).expect("closing instance remains owned");
            let mut closed=false;
            let turn=self.host_driver.dispatch_admission_step(false,|_,cx|semio_framework_plugin_host::GuestRelayAdmissionCall{result:runtime.close_original_instance_step(guest,&self.compiled,cx).map(|done|{closed=done;}),execution:None});
            match turn{
                Ok(true) if closed=>{
                    self.instances.remove(&instance);
                    self.closing_instances.remove(&instance);
                    self.session_documents.remove(&instance);
                    self.backbone_bindings.remove(&instance);
                    return
                }
                Err(failure)=>{self.startup_failure=Some(failure);return}
                _ if std::time::Instant::now()>=deadline=>return,
                _=>std::thread::yield_now(),
            }
        }
    }

    /// 🎬️ Opens the dynamic component and acknowledges its genuine lifecycle receipts.
    pub(crate) fn ensure_instance(&mut self, instance: u32) -> Result<(), Fault> {
        self.ensure_instance_scoped(instance, &ActivationScope::detached())
    }

    /// 🎟️ Uses the actual caller scope and source-issued phased startup.
    pub(crate) fn ensure_instance_scoped(&mut self, instance: u32, scope: &ActivationScope) -> Result<(), Fault> {
        if self.closing_instances.contains(&instance){return Err(Self::not_wired("instance retirement","the actual previous instance and its original source remain retained"))}
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
            semio_plugin_guest_contract::PluginHostError::Refused(fault) => Fault { code: fault.code.0.clone(), message: fault.message.clone() },
            other => Self::not_wired("instantiate", other),
        })?;
        let assets = find_repo_root()
            .map_err(|error| Fault { code: "package-assets.root".to_string(), message: error.to_string() })
            .and_then(|root| semio_plugin_host::package_assets::package_file_assets(&root, &self.descriptor.assets).map_err(|message| Fault { code: "package-assets.refused".to_string(), message }))?;
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
            let mut events = std::mem::take(&mut owed);
            match with_headless_identity(|identity| semio_framework_async::block_on(self.runtime.execute_turn(&mut guest, &mut events, headless_open_budget(), identity))) {
                Ok(turn) => {
                    if let Some(receipt) = turn.lifecycle_receipt {
                        owed.push(semio_framework::kernel::Event::InstanceLifecycleAck(semio_framework::kernel::ActorInstanceLifecycleAck { receipt }));
                    }
                    if matches!(turn.status, semio_framework::kernel::TurnStatus::Idle) && owed.is_empty() {
                        self.instances.insert(instance, guest);
                        return Ok(());
                    }
                }
                Err(error) if semio_plugin_host::retryable_lifecycle_turn(&error, &events) => {
                    owed.splice(0..0, events);
                }
                Err(error @ (semio_plugin_guest_contract::TurnFault::DeadlineExceeded | semio_plugin_guest_contract::TurnFault::FuelExhausted)) => {
                    self.instances.insert(instance,guest);
                    self.discard_instance(instance);
                    return Err(Fault { code: "budget.exceeded".to_string(), message: format!("InstanceOpen exceeded one turn's ceiling ({error}); the guest never came up") });
                }
                Err(error) => {
                    self.instances.insert(instance,guest);
                    self.discard_instance(instance);
                    return Err(Self::not_wired("InstanceOpen", error));
                }
            }
            if let Err(cancelled) = scope.checkpoint() {
                self.instances.insert(instance,guest);
                self.discard_instance(instance);
                return Err(cancelled);
            }
            if std::time::Instant::now() >= deadline {
                self.instances.insert(instance,guest);
                self.discard_instance(instance);
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
