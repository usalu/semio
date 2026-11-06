//! 🔁️ Exchange turns of the plugin artifact channel: child dependencies, the sequenced exchange, one command turn, reply frame admission and the response wait. See [`super`].

use super::*;

//#region 🔖️ArtifactChannel
#[cfg(not(target_arch = "wasm32"))]
impl PluginArtifactChannel {
    /// 🔢️ The next command sequence — the one number the kernel envelope and the command itself share.
    pub(super) fn take_seq(&mut self) -> u64 {
        let seq = self.next_seq;
        self.next_seq += 1;
        seq
    }

    /// 🪆️ The inference dependencies a composed artifact's readers compose on read (design §20.15): one
    /// `child:<slot>/<childId>` → HEAD snapshot pack per owned child of `artifact_id`, read with `ReadChildHeads`
    /// from the session guest that holds it. An artifact no session guest holds, or one without children, has none.
    pub(super) fn inference_child_dependencies(&mut self, artifact_id: &str) -> Result<Vec<(String, Vec<u8>)>, Fault> {
        let Some(instance) = self.session_documents.iter().find_map(|(instance, loaded)| (loaded == artifact_id).then_some(*instance)) else { return Ok(Vec::new()) };
        let seq = self.take_seq();
        match self.exchange_sequenced(instance, &store::AppCommand::ReadChildHeads { seq }, seq)? {
            store::AppFrame::ChildHeads { entries, .. } => Ok(child_head_dependencies(entries)),
            store::AppFrame::Error { fault, .. } => Err(decode_guest_fault(&fault)),
            other => Err(Self::not_wired("ReadChildHeads", format!("the guest answered seq {seq} with {}", app_frame_tag(&other)))),
        }
    }

    /// 🔁️ [`Self::exchange_one_real`] for a command already stamped with `seq` — the archive load's
    /// `store::DocumentArchiveLoadHost` stamps its own, because its operation IS its admission's sequence.
    pub(super) fn exchange_sequenced(&mut self, instance: u32, real_command: &store::AppCommand, seq: u64) -> Result<store::AppFrame, Fault> {
        let deadline = std::time::Instant::now() + COMMAND_RESUME_WALL_BUDGET;
        loop {
            match self.exchange_one_turn(instance, real_command, seq)? {
                CommandTurn::Settled(frame) => return Ok(frame),
                CommandTurn::MoreWork if std::time::Instant::now() < deadline => continue,
                CommandTurn::MoreWork => {
                    self.discard_instance(instance);
                    return Err(Fault { code: "budget.exceeded".to_string(), message: format!("AppCommand seq {seq} reached its wall budget before it settled; the instance was discarded and the next command starts on a fresh one") });
                }
            }
        }
    }

    /// 🔁️ ONE guest turn, or one bounded host-side bookkeeping step, of the command above. Returns [Design notes](README.md#source-notes-44).
    fn exchange_one_turn(&mut self, instance: u32, real_command: &store::AppCommand, seq: u64) -> Result<CommandTurn, Fault> {
        if !self.rejected_command_builds.terminal_is_empty() {
            self.rejected_command_builds.close_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES);
            return Ok(CommandTurn::MoreWork);
        }
        begin_closing_abandoned_exchange(&self.pending_exchanges, &mut self.pending_command_closes, instance, seq)?;
        if let Some(seq) = self.pending_exchanges.get(instance).map(|pending| pending.seq) {
            if !self.pending_command_closes.is_active(u64::from(instance), seq) {
                let (response_complete, _) = self.pending_exchanges.close_response_step(instance, semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES);
                if !response_complete {
                    return Ok(CommandTurn::MoreWork);
                }
                let (complete, _, _) = self.pending_command_closes.close_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES);
                if complete {
                    let terminal = self.pending_exchanges.remove(instance).expect("closed pending exchange reached terminal empty");
                    assert!(terminal.response.terminal_is_empty(), "closed pending response reached terminal empty");
                }
                return Ok(CommandTurn::MoreWork);
            }
        }
        if self.pending_exchanges.get(instance).is_none() && !persistent_command_completion_port_ready() {
            return Err(Self::not_wired("AppCommand", "persistent command completion submit/poll/cancel authority is not admitted"));
        }
        if self.pending_exchanges.get(instance).is_none() {
            if !self.pending_exchanges.can_insert(instance) || !self.pending_command_closes.can_insert(u64::from(instance)) || !self.rejected_command_builds.can_insert(u64::from(instance)) {
                return Err(Fault { code: "budget.exceeded".to_string(), message: "fixed pending command authority is saturated or collided".to_string() });
            }
            let command = ::semio_framework_async::poll::resolve_ready(store::encode_app_command(real_command)).map_err(|fault| Self::not_wired("encoding AppCommand", format!("{}: {}", fault.code.0, fault.message)))?;
            let envelope = semio_framework::kernel::CommandEnvelope { instance, seq, command };
            let mut owners = semio_framework::kernel::CommandEnvelopeSet::try_new().map_err(|fault| Self::not_wired("reserving command batch", format!("{}: {}", fault.code.0, fault.message)))?;
            if let Err((fault, rejected)) = owners.try_push(envelope) {
                self.rejected_command_builds.insert_admitted(u64::from(instance), semio_framework::kernel::RejectedCommandBuild::new(owners, rejected));
                return Err(Self::not_wired("admitting command owner", format!("{}: {}", fault.code.0, fault.message)));
            }
            let batch = match semio_framework::kernel::CommandBatch::try_new(seq, owners) {
                Ok(batch) => batch,
                Err((fault, owners)) => {
                    self.rejected_command_builds.insert_admitted(u64::from(instance), semio_framework::kernel::RejectedCommandBuild::from_admitted(owners));
                    return Err(Self::not_wired("admitting command batch", format!("{}: {}", fault.code.0, fault.message)));
                }
            };
            self.pending_command_closes.insert_admitted(u64::from(instance), seq, semio_framework::kernel::CommandBatchDriver::new(seq, batch));
            self.pending_exchanges.insert_admitted(PendingExchange { instance, seq, response: PendingResponsePage::Empty, ingress_fault: None });
        }
        assert_eq!(self.pending_exchanges.get(instance).expect("pending exchange was admitted").seq, seq, "the retained exchange answers the command this turn drives");
        let mut event = self
            .pending_command_closes
            .with_driver_mut(u64::from(instance), seq, |driver| driver.next_page())
            .map_err(|fault| Self::not_wired("retained command driver", format!("{}: {}", fault.code.0, fault.message)))?
            .map_err(|fault| Self::not_wired("command page", format!("{}: {}", fault.code.0, fault.message)))?
            .map(|(cursor, bytes)| semio_framework::kernel::Event::CommandIngressPage { cursor, bytes })
            .unwrap_or(semio_framework::kernel::Event::Wake);
        let runtime = Arc::clone(&self.runtime);
        let guest = self.instances.get_mut(&instance).ok_or_else(|| Self::not_wired("exchange", format!("no open instance {instance}")))?;
        self.pending_command_closes.prepare_suspend(u64::from(instance), seq).map_err(|fault| Self::not_wired("suspending command owner", format!("{}: {}", fault.code.0, fault.message)))?;
        let outcome = with_headless_identity(|identity| semio_framework_async::block_on(runtime.execute_turn(guest, std::slice::from_mut(&mut event), headless_command_budget(), identity)));
        let turn = match outcome {
            Ok(turn) => {
                self.pending_command_closes.resume(u64::from(instance), seq).map_err(|fault| Self::not_wired("resuming command owner", format!("{}: {}", fault.code.0, fault.message)))?;
                turn
            }
            Err(error) => {
                self.pending_command_closes.resume(u64::from(instance), seq).map_err(|fault| Self::not_wired("resuming command owner", format!("{}: {}", fault.code.0, fault.message)))?;
                self.discard_instance(instance);
                return Err(match error {
                    semio_plugin_guest_contract::TurnFault::DeadlineExceeded | semio_plugin_guest_contract::TurnFault::FuelExhausted => {
                        Fault { code: "budget.exceeded".to_string(), message: format!("AppCommand exceeded one turn's ceiling ({error}); the instance was discarded") }
                    }
                    error => Self::not_wired("execute_turn", error),
                });
            }
        };
        let progress = self
            .pending_command_closes
            .with_driver_mut(u64::from(instance), seq, |driver| driver.observe(&turn.command_ingress, semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES))
            .map_err(|fault| Self::not_wired("retained command driver", format!("{}: {}", fault.code.0, fault.message)))?
            .map_err(|fault| Self::not_wired("command acknowledgement", format!("{}: {} — the guest answered {:?} while this gateway drives instance {instance} seq {seq}", fault.code.0, fault.message, turn.command_ingress)))?;
        let mut published: Vec<String> = Vec::new();
        let mut backbone_egress: Vec<Vec<u8>> = Vec::new();
        for effect in &turn.effects {
            published.push(effect_shape(instance, effect));
            if let Some(payload) = document_backbone_payload(&self.actor_label, effect) {
                backbone_egress.push(payload.to_vec());
            }
            Self::admit_reply_frame(self.pending_exchanges.get_mut(instance).expect("pending exchange was admitted"), instance, seq, effect);
        }
        self.backbone_egress.append(&mut backbone_egress);
        match progress {
            semio_framework::kernel::CommandBatchProgress::Complete => {
                self.pending_command_closes.remove_terminal(u64::from(instance), seq).map_err(|fault| Self::not_wired("terminal command owner", format!("{}: {}", fault.code.0, fault.message)))?;
                let mut pending = self.pending_exchanges.remove(instance).expect("terminal pending exchange");
                self.await_response(instance, seq, &mut pending.response).map_err(|fault| Fault { code: fault.code, message: format!("{}; the acknowledging turn published {}", fault.message, named_shapes(&published)) })?;
                pending.response.take(seq).map(CommandTurn::Settled)
            }
            semio_framework::kernel::CommandBatchProgress::Faulted => {
                if let Some(pending) = self.pending_exchanges.get_mut(instance) {
                    if pending.ingress_fault.is_none() {
                        pending.ingress_fault = Some(ingress_fault_text(&turn.command_ingress));
                    }
                }
                self.pending_command_closes.begin_close(u64::from(instance), seq).map_err(|fault| Self::not_wired("faulted command owner", format!("{}: {}", fault.code.0, fault.message)))?;
                let (response_complete, _) = self.pending_exchanges.close_response_step(instance, semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES);
                if !response_complete {
                    return Ok(CommandTurn::MoreWork);
                }
                let (complete, _, _) = self.pending_command_closes.close_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES);
                if complete {
                    let terminal = self.pending_exchanges.remove(instance).expect("faulted exchange reached an empty terminal owner");
                    assert!(terminal.response.terminal_is_empty(), "faulted pending response reached terminal empty");
                    Err(Self::not_wired("command ingress", format!("the guest refused seq {seq}: {}", terminal.ingress_fault.unwrap_or_else(|| "no reason reported".to_string()))))
                } else {
                    Ok(CommandTurn::MoreWork)
                }
            }
            semio_framework::kernel::CommandBatchProgress::PageReady | semio_framework::kernel::CommandBatchProgress::Waiting => Ok(CommandTurn::MoreWork),
        }
    }

    /// 📬️ Admits one effect, if it is THIS command's answer. [Design notes](README.md#source-notes-192).
    fn admit_reply_frame(pending: &mut PendingExchange, instance: u32, seq: u64, effect: &semio_framework::kernel::Effect) {
        let Some(payload) = shell_app_frame_payload(instance, effect) else {
            return;
        };
        let Some(prefix)=store::app_frame_reply_prefix(payload)else{return};
        if prefix.done && pending.response.holds_frame() {
            return;
        }
        if prefix.answers(seq) {
            pending.response.admit_frame(payload);
        }
    }

    /// 📮️ Wakes the guest until it publishes the frame answering `seq`, once its command ingress has [Design notes](README.md#source-notes-234).
    fn await_response(&mut self, instance: u32, seq: u64, response: &mut PendingResponsePage) -> Result<(), Fault> {
        let deadline = std::time::Instant::now() + COMMAND_RESUME_WALL_BUDGET;
        let mut foreign: Vec<u64> = Vec::new();
        while matches!(response, PendingResponsePage::Empty) {
            let runtime = Arc::clone(&self.runtime);
            let guest = self.instances.get_mut(&instance).ok_or_else(|| Self::not_wired("response", format!("no open instance {instance}")))?;
            let turn = match with_headless_identity(|identity| semio_framework_async::block_on(runtime.execute_turn(guest, std::slice::from_mut(&mut semio_framework::kernel::Event::Wake), headless_command_budget(), identity))) {
                Ok(turn) => turn,
                Err(error) => {
                    self.discard_instance(instance);
                    return Err(Self::not_wired("response", error));
                }
            };
            let idle = matches!(turn.status, semio_framework::kernel::TurnStatus::Idle) && turn.next_wake.is_none();
            for effect in &turn.effects {
                if let semio_framework::kernel::Effect::Respond { req, .. } = effect {
                    foreign.push(req.0);
                }
                let Some(payload) = shell_app_frame_payload(instance, effect) else {
                    continue;
                };
                let Some(prefix)=store::app_frame_reply_prefix(payload)else{continue};
                if prefix.answers(seq) {
                    response.admit_frame(payload);
                }
            }
            if matches!(response, PendingResponsePage::Empty) && idle {
                if !foreign.is_empty() {
                    return Err(Fault { code: "channel.not-wired".to_string(), message: format!("the guest acknowledged command seq {seq} and answered request(s) {foreign:?} instead") });
                }
                response.stamp_settled();
                return Ok(());
            }
            if std::time::Instant::now() >= deadline {
                return Err(Self::budget_fault(&format!("the guest acknowledged command seq {seq} but published no response in")));
            }
        }
        Ok(())
    }
}
//#endregion 🔖️ArtifactChannel
