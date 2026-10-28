//! 📨️ Actor hub frames, acks and delivery: the hub frame handling, the acks, the refused transition retraction, the relay to the hub, the raw sends and the remote operation, retraction and archive delivery with the status emission. See [`super`].

use super::*;

impl ArtifactActor {
    //#region 🔖️Hub
    /// 🪙️ Command-lane credit-based flow control: no client-side backpressure
    /// implemented this wave (scope is frame plumbing, not congestion control) —
    /// accepted and ignored.
    pub(super) async fn on_hub_frame(&mut self, frame: ServerFrame) {
        match frame {
            ServerFrame::Welcome { session_id: _, resume_token, server_frontier, bootstrap } => {
                self.requeue_pending_batches();
                match bootstrap {
                    Bootstrap::None => {
                        self.abort_artifact_bootstrap();
                        self.resume_token = Some(resume_token);
                        self.server_frontier = Some(server_frontier);
                        self.set_remote_state(RemoteState::Live { peer_count: 0 }).await;
                        self.flush_outbox().await;
                    }
                    Bootstrap::Tail => {
                        self.abort_artifact_bootstrap();
                        self.pending_resume_token = Some(resume_token);
                        self.required_tail_frontier = Some(server_frontier);
                        self.finish_catchup_if_ready().await;
                    }
                    Bootstrap::Snapshot { .. } => {
                        self.fail_artifact_bootstrap("database-private snapshot cannot seed an artifact client").await;
                    }
                    Bootstrap::ArtifactBootstrap(bootstrap) => {
                        self.start_artifact_bootstrap(*bootstrap, resume_token, server_frontier).await;
                    }
                }
            }
            ServerFrame::SnapshotChunk { .. } | ServerFrame::SnapshotDone { .. } => {
                self.fail_artifact_bootstrap("database-private snapshot frame cannot seed an artifact client").await;
            }
            ServerFrame::RebootstrapRequired { control } => {
                if control.document_id != self.document_id || self.hub_space_id.as_deref() != Some(control.space_id.as_str()) || control.baseline_frontier.document_id.0 != self.document_id {
                    self.fail_artifact_bootstrap("rebootstrap control scope mismatch").await;
                } else {
                    self.require_artifact_rebootstrap(rebootstrap_control(&control)).await;
                }
            }
            ServerFrame::ArtifactBootstrapChunk { descriptor_hash, index, bytes } => {
                let Some(mut pending) = self.artifact_bootstrap.take() else {
                    self.fail_artifact_bootstrap("artifact bootstrap chunk arrived without an active transfer").await;
                    return;
                };
                let mut control = self.bootstrap_control(pending.started_at);
                match pending.assembler.push(descriptor_hash, index, bytes.as_slice(), &mut control) {
                    Ok(_) => self.artifact_bootstrap = Some(pending),
                    Err(error) => self.fail_artifact_bootstrap(error.to_string()).await,
                }
            }
            ServerFrame::ArtifactBootstrapDone { descriptor_hash, chunk_count } => {
                let Some(mut pending) = self.artifact_bootstrap.take() else {
                    self.fail_artifact_bootstrap("artifact bootstrap completion arrived without an active transfer").await;
                    return;
                };
                let mut control = self.bootstrap_control(pending.started_at);
                match pending.assembler.finish(Some((descriptor_hash, chunk_count)), &mut control) {
                    Ok(pair) => {
                        if let Err(error) = self.install_artifact_bootstrap(pending, pair).await {
                            self.fail_artifact_bootstrap(error).await;
                        }
                    }
                    Err(error) => self.fail_artifact_bootstrap(error.to_string()).await,
                }
            }
            ServerFrame::Commands { envelopes, frontier, .. } => {
                if self.artifact_bootstrap.is_some() {
                    self.fail_artifact_bootstrap("tail arrived before artifact bootstrap completion").await;
                    return;
                }
                let settled = settle_committed_envelopes(&mut self.outbox, &mut self.pending_batches, &envelopes);
                self.document_backbone_retention.release(&settled);
                note_authored_envelopes(&mut self.applied_op_ids, &settled);
                let persisted = &self.known_op_ids;
                let fresh = admit_remote_envelopes(&mut self.applied_op_ids, envelopes.into_iter().filter(|envelope| !persisted.contains(&envelope.mutation_id.0)));
                if !fresh.is_empty() {
                    self.persist_operations(&fresh).await;
                    if !self.deliver_remote_operations(fresh).await {
                        self.fail_artifact_bootstrap("artifact tail could not be installed").await;
                        return;
                    }
                }
                self.server_frontier = Some(frontier);
                self.finish_catchup_if_ready().await;
            }
            ServerFrame::Ack { batch_id, stages, frontier } => {
                if self.artifact_bootstrap.is_some() || self.required_tail_frontier.is_some() {
                    self.fail_artifact_bootstrap("ack arrived before artifact catch-up completion").await;
                    return;
                }
                self.server_frontier = Some(frontier);
                self.handle_ack(batch_id, stages).await;
            }
            ServerFrame::Preview { actor, key, seq, payload } => {
                if self.socket_actor.as_deref() != Some(actor.0.as_str()) {
                    self.emit(ArtifactEvent::Preview { actor: actor.0, key, seq, payload });
                }
            }
            ServerFrame::Presence { peers } => {
                let mut decoded: Vec<PresencePeer> = Vec::new();
                for p in &peers {
                    if let Some(peer) = presence_from_bytes(p).await {
                        decoded.push(peer);
                    }
                }
                let peers = decoded;
                if matches!(self.remote_state, RemoteState::Live { .. }) && self.artifact_bootstrap.is_none() && self.required_tail_frontier.is_none() {
                    self.set_remote_state(RemoteState::Live { peer_count: peers.len() }).await;
                }
                self.emit(ArtifactEvent::Presence { peers });
            }
            ServerFrame::Session { actor, color } => {
                if self.socket_actor.as_deref() != Some(actor.as_str()) {
                    self.fail_artifact_bootstrap("socket receipt actor mismatch").await;
                    return;
                }
                self.socket_actor_confirmed = true;
                self.session_color = Some(color);
                self.emit(ArtifactEvent::Session { actor, color });
                self.flush_outbox().await;
            }
            ServerFrame::CreditGrant { .. } => {
            }
            ServerFrame::Error { code, message } => {
                self.emit(ArtifactEvent::Conflict(MutationMessage { level: semio_framework_diagnostic::Severity::Error, code: semio_framework_diagnostic::FaultCode::new(code), message, target: vec![self.hub_base_url.clone().unwrap_or_default()], op_index: None }));
            }
        }
    }

    /// 📮️ Resolves one outbound `Commands` batch's terminal `Applied` stage: `Accepted`
    /// just clears the pending batch; `Transformed`/`Rejected` both roll back the speculative
    /// local head first (via {@link rollback_envelope}, replayed as remote operations), and
    /// `Transformed` then delivers the semio_hub's replacement envelope the same way.
    pub async fn handle_ack(&mut self, batch_id: u64, stages: Vec<AckStage>) {
        for stage in stages {
            let AckStage::Applied { outcome } = stage else { continue };
            let Some(sent) = self.pending_batches.remove(&batch_id) else { continue };
            self.document_backbone_retention.release(&sent);
            match *outcome {
                ApplyOutcome::Accepted => {
                    self.emit(ArtifactEvent::CommandOutcome { batch_id, outcome: CommandAckOutcome::Accepted });
                }
                ApplyOutcome::Transformed { envelope } => {
                    let mut rollbacks: Vec<MutationEnvelope> = Vec::new();
                    for envelope in sent.iter().rev() {
                        rollbacks.extend(rollback_envelope(envelope).await);
                    }
                    self.persist_operations(&rollbacks).await;
                    let _ = self.deliver_remote_operations(rollbacks).await;
                    let refusal = self.retract_refused_transitions(&sent, "transformed").await;
                    let converted = *envelope;
                    self.persist_operations(std::slice::from_ref(&converted)).await;
                    let _ = self.deliver_remote_operations(vec![converted]).await;
                    self.emit(ArtifactEvent::CommandOutcome { batch_id, outcome: CommandAckOutcome::Transformed });
                    if let Some(refusal) = refusal {
                        self.emit(ArtifactEvent::Conflict(refusal));
                    }
                }
                ApplyOutcome::Rejected { reason, messages } => {
                    let mut rollbacks: Vec<MutationEnvelope> = Vec::new();
                    for envelope in sent.iter().rev() {
                        rollbacks.extend(rollback_envelope(envelope).await);
                    }
                    self.persist_operations(&rollbacks).await;
                    let _ = self.deliver_remote_operations(rollbacks).await;
                    let refusal = self.retract_refused_transitions(&sent, &reason).await;
                    self.emit(ArtifactEvent::CommandOutcome { batch_id, outcome: CommandAckOutcome::hub_rejected(reason, &messages) });
                    if let Some(refusal) = refusal {
                        self.emit(ArtifactEvent::Conflict(refusal));
                    }
                }
            }
        }
        self.emit_status_if_changed().await;
    }

    /// 🔙️ Retracts the history transitions of `refused`, a batch the hub did not take as sent: the persisted log and
    /// the store drop them ([`transition_refusal`]); answers the typed refusal naming them.
    pub async fn retract_refused_transitions(&mut self, refused: &[MutationEnvelope], reason: &str) -> Option<MutationMessage> {
        let refusal = transition_refusal(refused, reason)?;
        self.persist_retraction(&refusal.target).await;
        let _ = self.deliver_retraction(refusal.target.clone()).await;
        Some(refusal)
    }

    pub(super) async fn relay_operations_to_hub(&mut self, envelopes: &[MutationEnvelope]) {
        if envelopes.is_empty() {
            return;
        }
        note_authored_envelopes(&mut self.applied_op_ids, envelopes);
        let Some(socket_actor) = self.socket_actor.as_ref() else {
            self.queue_outbox(envelopes.iter().cloned());
            return;
        };
        if !self.socket_actor_confirmed || self.semio_hub.is_none() || self.artifact_bootstrap.is_some() || self.required_tail_frontier.is_some() {
            self.queue_outbox(envelopes.iter().cloned());
            return;
        }
        let batch_id = self.next_batch_id;
        self.next_batch_id = self.next_batch_id.wrapping_add(1);
        let(wire_envelopes,identity_receipt)=match wire_envelopes_with_original_actor(envelopes,socket_actor,self.actor_identity_grant){Ok(result)=>result,Err(_)=>{self.queue_outbox(envelopes.iter().cloned());return;}};
        add_actor_identity_progress(&mut self.actor_identity_progress,identity_receipt);
        self.pending_batches.insert(batch_id, envelopes.to_vec());
        self.send_client_frame(ClientFrame::Commands { batch_id, envelopes: wire_envelopes }, Lane::Command).await;
        self.emit_status_if_changed().await;
    }

    pub(super) async fn send_client_frame(&mut self, frame: ClientFrame, lane: Lane) {
        let bytes = encode_client_frame(&frame, lane).await;
        self.send_raw(Message::Binary(bytes.into())).await;
    }

    pub(super) async fn send_raw(&mut self, message: Message) {
        let mut failed = false;
        if let Some(conn) = self.semio_hub.as_mut() {
            if conn.write.send(message).await.is_err() {
                failed = true;
            }
        }
        if failed {
            self.abort_artifact_bootstrap();
            self.requeue_pending_batches();
            self.semio_hub = None;
            self.clear_socket_epoch();
            self.schedule_reconnect().await;
        }
    }
    //#endregion 🔖️Hub

}

impl ArtifactActor {
    //#region 🔖️Deliver
    /// 🕸️ Pushes remote operations into the store's inbound queue and notifies subscribers.
    pub async fn deliver_remote_operations(&mut self, envelopes: Vec<MutationEnvelope>) -> bool {
        if envelopes.is_empty() {
            return true;
        }
        if self.remote.push(BackboneMessage::from_envelopes(&envelopes)).await.is_err() {
            return false;
        }
        if self.hub_space_id.is_some() {
            let message = match (BackboneMessage::from_envelopes(&envelopes)).encode_op() {
                Ok(message) => message,
                Err(_) => return false,
            };
            self.emit(ArtifactEvent::DocumentBackbone { message });
        } else {
            self.emit(ArtifactEvent::RemoteMutations { envelopes });
        }
        true
    }

    /// 🛎️ Asks the store to retract `mutation_ids` (`BackboneMessage::Retract`), through the document backbone too when
    /// a hub space binds this document.
    pub async fn deliver_retraction(&mut self, mutation_ids: Vec<String>) -> bool {
        let message = BackboneMessage::Retract { mutation_ids };
        let encoded = message.encode_op();
        if self.remote.push(message).await.is_err() {
            return false;
        }
        if self.hub_space_id.is_some() {
            let Ok(message) = encoded else { return false };
            self.emit(ArtifactEvent::DocumentBackbone { message });
        }
        true
    }

    /// 🗃️ Publishes a replaced recursive archive. An external rewrite that LOST events cannot be
    /// expressed as new events, so it is a document replacement: the host reloads the store from
    /// the archive by replaying its events — nothing is pushed into (or merged by) the live store.
    pub(super) async fn deliver_archive(&mut self, bytes: Vec<u8>) {
        self.emit(ArtifactEvent::DocumentArchiveReplaced { archive: bytes });
    }

    /// 🚫️async: E1 pure sync body — `broadcast::Sender::send` never suspends. Declaring this
    /// `async fn` forced `&ArtifactActor` to live across an await, which demanded
    /// `ArtifactActor: Sync` from every `!Sync` field it owns (the folder watcher's `Receiver`,
    /// `connect_future`, the codec table's `dyn Fn`) and made the whole actor turn non-`Send`.
    pub(super) fn emit(&self, event: ArtifactEvent) {
        let _ = self.events.send(event);
    }

    /// 🚫️async: E1 pure sync body — see `emit` above; same `&self`-across-await mechanism.
    pub fn status(&self) -> ArtifactSyncStatus {
        let pending_mutations = self.pending_batches.values().map(Vec::len).sum();
        let acknowledged_head = if pending_mutations == 0 && self.outbox.is_empty() { self.server_frontier.as_ref().and_then(acknowledged_edited_head) } else { None };
        ArtifactSyncStatus { persisted: self.last_written_hash.is_some() || self.server_frontier.is_some(), pending_mutations, remote: self.remote_state.clone(), acknowledged_head }
    }

    pub async fn set_remote_state(&mut self, state: RemoteState) {
        if matches!(state, RemoteState::Live { .. }) {
            self.link = self.link.apply(&DOCUMENT_LINK_SHORTAGE_POLICY, DocumentLinkEvent::Restored { now_ms: wall_ms() });
            self.link_expires_at = None;
        }
        self.remote_state = state;
        self.emit_status_if_changed().await;
    }

    pub async fn emit_status_if_changed(&mut self) {
        let status = self.status();
        if self.last_status.as_ref() != Some(&status) {
            self.last_status = Some(status.clone());
            self.emit(ArtifactEvent::Status(status));
        }
    }
    //#endregion 🔖️Deliver
}
