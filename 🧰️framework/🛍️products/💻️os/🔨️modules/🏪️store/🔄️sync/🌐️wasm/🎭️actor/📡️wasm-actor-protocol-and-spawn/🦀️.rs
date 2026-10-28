//! 📡️ Wasm actor protocol and spawn: the binary frame handling, the acks, the remote operation delivery, the refused transition retraction and the actor spawn. See [`super`].

use super::*;

impl WasmActor {
    //#region 🔖️Protocol
    /// 📡️ The native actor's `on_hub_frame`, frame for frame: the socket actor the grant named is the
    /// only origin this replica filters as its own, `Session` must confirm it, and `Live` is reached
    /// only after every catch-up.
    pub(super) async fn on_binary(&mut self, bytes: &[u8]) {
        let frame = match decode_server_frame(bytes).await {
            Ok((_lane, frame)) => frame,
            Err(error) => {
                self.fail_artifact_bootstrap(format!("malformed hub frame: {error}"));
                return;
            }
        };
        match frame {
            ServerFrame::Welcome { session_id: _, resume_token, server_frontier, bootstrap } => {
                self.requeue_pending_batches();
                match bootstrap {
                    Bootstrap::None => {
                        self.abort_artifact_bootstrap();
                        self.resume_token = Some(resume_token);
                        self.server_frontier = Some(server_frontier);
                        self.set_remote_state(RemoteState::Live { peer_count: 0 });
                        self.flush_outbox().await;
                    }
                    Bootstrap::Tail => {
                        self.abort_artifact_bootstrap();
                        self.pending_resume_token = Some(resume_token);
                        self.required_tail_frontier = Some(server_frontier);
                        self.finish_catchup_if_ready().await;
                    }
                    Bootstrap::Snapshot { .. } => self.fail_artifact_bootstrap("database-private snapshot cannot seed an artifact client"),
                    Bootstrap::ArtifactBootstrap(bootstrap) => {
                        self.start_artifact_bootstrap(*bootstrap, resume_token, server_frontier).await;
                    }
                }
            }
            ServerFrame::SnapshotChunk { .. } | ServerFrame::SnapshotDone { .. } => self.fail_artifact_bootstrap("database-private snapshot frame cannot seed an artifact client"),
            ServerFrame::RebootstrapRequired { control } => {
                if control.document_id != self.document_id || self.hub_space_id.as_deref() != Some(control.space_id.as_str()) || control.baseline_frontier.document_id.0 != self.document_id {
                    self.fail_artifact_bootstrap("rebootstrap control scope mismatch");
                } else {
                    self.require_artifact_rebootstrap(rebootstrap_control(&control));
                }
            }
            ServerFrame::ArtifactBootstrapChunk { descriptor_hash, index, bytes } => {
                let Some(mut pending) = self.artifact_bootstrap.take() else {
                    self.fail_artifact_bootstrap("artifact bootstrap chunk arrived without an active transfer");
                    return;
                };
                let mut control = self.bootstrap_control(false);
                match pending.assembler.push(descriptor_hash, index, bytes.as_slice(), &mut control) {
                    Ok(_) => self.artifact_bootstrap = Some(pending),
                    Err(error) => self.fail_artifact_bootstrap(error.to_string()),
                }
            }
            ServerFrame::ArtifactBootstrapDone { descriptor_hash, chunk_count } => {
                let Some(mut pending) = self.artifact_bootstrap.take() else {
                    self.fail_artifact_bootstrap("artifact bootstrap completion arrived without an active transfer");
                    return;
                };
                let mut control = self.bootstrap_control(false);
                match pending.assembler.finish(Some((descriptor_hash, chunk_count)), &mut control) {
                    Ok(pair) => {
                        if let Err(error) = self.install_artifact_bootstrap(pending, pair).await {
                            self.fail_artifact_bootstrap(error);
                        }
                    }
                    Err(error) => self.fail_artifact_bootstrap(error.to_string()),
                }
            }
            ServerFrame::Commands { envelopes, frontier, .. } => {
                if self.artifact_bootstrap.is_some() {
                    self.fail_artifact_bootstrap("tail arrived before artifact bootstrap completion");
                    return;
                }
                let settled = settle_committed_envelopes(&mut self.outbox, &mut self.pending_batches, &envelopes);
                self.document_backbone_retention.release(&settled);
                note_authored_envelopes(&mut self.applied_op_ids, &settled);
                let fresh = admit_remote_envelopes(&mut self.applied_op_ids, envelopes);
                if !self.deliver_remote_operations(fresh).await {
                    self.fail_artifact_bootstrap("artifact tail could not be installed");
                    return;
                }
                self.server_frontier = Some(frontier);
                self.finish_catchup_if_ready().await;
            }
            ServerFrame::Ack { batch_id, stages, frontier } => {
                if self.artifact_bootstrap.is_some() || self.required_tail_frontier.is_some() {
                    self.fail_artifact_bootstrap("ack arrived before artifact catch-up completion");
                    return;
                }
                self.server_frontier = Some(frontier);
                self.handle_ack(batch_id, stages).await;
            }
            ServerFrame::Preview { actor, key, seq, payload } => {
                if self.socket_actor.as_deref() != Some(actor.0.as_str()) {
                    let _ = self.events.send(ArtifactEvent::Preview { actor: actor.0, key, seq, payload });
                }
            }
            ServerFrame::Presence { peers } => {
                let mut decoded: Vec<PresencePeer> = Vec::with_capacity(peers.len());
                for peer in &peers {
                    if let Some(peer) = presence_from_bytes(peer).await {
                        decoded.push(peer);
                    }
                }
                if matches!(self.remote_state, RemoteState::Live { .. }) && self.artifact_bootstrap.is_none() && self.required_tail_frontier.is_none() {
                    self.set_remote_state(RemoteState::Live { peer_count: decoded.len() });
                }
                let _ = self.events.send(ArtifactEvent::Presence { peers: decoded });
            }
            ServerFrame::Session { actor, color } => {
                if self.socket_actor.as_deref() != Some(actor.as_str()) {
                    self.fail_artifact_bootstrap("socket receipt actor mismatch");
                    return;
                }
                self.socket_actor_confirmed = true;
                self.session_color = Some(color);
                let _ = self.events.send(ArtifactEvent::Session { actor, color });
                self.flush_outbox().await;
            }
            ServerFrame::CreditGrant { .. } => {}
            ServerFrame::Error { code, message } => {
                let _ = self.events.send(ArtifactEvent::Conflict(MutationMessage {
                    level: semio_framework_diagnostic::Severity::Error,
                    code: semio_framework_diagnostic::FaultCode::new(code),
                    message,
                    target: vec![self.hub_base_url.clone().unwrap_or_default()],
                    op_index: None,
                }));
            }
        }
    }

    /// 📮️ Mirrors the native actor's `handle_ack` — see its doc comment.
    pub async fn handle_ack(&mut self, batch_id: u64, stages: Vec<AckStage>) {
        for stage in stages {
            let AckStage::Applied { outcome } = stage else { continue };
            let Some(sent) = self.pending_batches.remove(&batch_id) else { continue };
            self.document_backbone_retention.release(&sent);
            match *outcome {
                ApplyOutcome::Accepted => {
                    let _ = self.events.send(ArtifactEvent::CommandOutcome { batch_id, outcome: CommandAckOutcome::Accepted });
                }
                ApplyOutcome::Transformed { envelope } => {
                    let mut rollbacks: Vec<MutationEnvelope> = Vec::new();
                    for envelope in sent.iter().rev() {
                        rollbacks.extend(rollback_envelope(envelope).await);
                    }
                    let _ = self.deliver_remote_operations(rollbacks).await;
                    let refusal = self.retract_refused_transitions(&sent, "transformed").await;
                    let _ = self.deliver_remote_operations(vec![*envelope]).await;
                    let _ = self.events.send(ArtifactEvent::CommandOutcome { batch_id, outcome: CommandAckOutcome::Transformed });
                    if let Some(refusal) = refusal {
                        let _ = self.events.send(ArtifactEvent::Conflict(refusal));
                    }
                }
                ApplyOutcome::Rejected { reason, messages } => {
                    let mut rollbacks: Vec<MutationEnvelope> = Vec::new();
                    for envelope in sent.iter().rev() {
                        rollbacks.extend(rollback_envelope(envelope).await);
                    }
                    let _ = self.deliver_remote_operations(rollbacks).await;
                    let refusal = self.retract_refused_transitions(&sent, &reason).await;
                    let _ = self.events.send(ArtifactEvent::CommandOutcome { batch_id, outcome: CommandAckOutcome::hub_rejected(reason, &messages) });
                    if let Some(refusal) = refusal {
                        let _ = self.events.send(ArtifactEvent::Conflict(refusal));
                    }
                }
            }
        }
        self.emit_status_if_changed();
    }

    pub async fn deliver_remote_operations(&self, envelopes: Vec<MutationEnvelope>) -> bool {
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
            let _ = self.events.send(ArtifactEvent::DocumentBackbone { message });
        } else {
            let _ = self.events.send(ArtifactEvent::RemoteMutations { envelopes });
        }
        true
    }

    /// 🪞️ Mirrors the native actor's `retract_refused_transitions`; a browser actor persists no log of its own.
    pub async fn retract_refused_transitions(&self, refused: &[MutationEnvelope], reason: &str) -> Option<MutationMessage> {
        let refusal = transition_refusal(refused, reason)?;
        let _ = self.deliver_retraction(refusal.target.clone()).await;
        Some(refusal)
    }

    /// 🪩️ Mirrors the native actor's `deliver_retraction`.
    pub async fn deliver_retraction(&self, mutation_ids: Vec<String>) -> bool {
        let message = BackboneMessage::Retract { mutation_ids };
        let encoded = message.encode_op();
        if self.remote.push(message).await.is_err() {
            return false;
        }
        if self.hub_space_id.is_some() {
            let Ok(message) = encoded else { return false };
            let _ = self.events.send(ArtifactEvent::DocumentBackbone { message });
        }
        true
    }
    //#endregion 🔖️Protocol
}

/// 🚀️ Spawns one browser document actor on the isolate's executor. Its loop owns one cadence: the
/// mailbox, or — when the mailbox is quiet — the next socket poll or scheduled redial, whichever
/// comes first. A cancelled document (closed by the host) ends the loop and closes its socket.
pub(in super::super) async fn spawn_actor(config: ArtifactActorConfig, remote: ChannelBackboneRemote, cmd_rx: ArtifactMailboxReceiver, events: broadcast::Sender<ArtifactEvent>, hub: WasmActorHub) {
    let mut hub_base_url = None;
    let mut hub_space_id = None;
    let mut hub_surface = None;
    for binding in &config.bindings {
        if let PersistenceBinding::Hub { base_url, space_id, surface } = binding {
            if hub_base_url.is_none() {
                hub_base_url = Some(base_url.clone());
                hub_space_id = Some(space_id.clone());
                hub_surface = surface.clone();
            }
        }
    }
    let mut actor = WasmActor {
        actor_identity_grant: config.actor_identity_grant,
        actor_identity_progress: Default::default(),
        document_id: config.document_id,
        schema: config.schema,
        remote,
        events,
        hub_base_url,
        hub_space_id,
        hub_surface,
        credential: hub.credential,
        socket_grant_source: hub.socket_grant_source,
        dialer: hub.dialer,
        document_execution_target_lease: hub.lease,
        operation_cancel: hub.cancel,
        socket: None,
        hello: None,
        socket_actor: None,
        socket_actor_confirmed: false,
        session_color: None,
        server_frontier: hub.seed.as_ref().map(|pair| canonical_pair_baseline(&pair.baseline_frontier)),
        resume_token: None,
        pending_resume_token: None,
        required_tail_frontier: None,
        artifact_rebootstrap_required: false,
        artifact_bootstrap: None,
        pending_batches: std::collections::HashMap::new(),
        outbox: Vec::new(),
        applied_op_ids: std::collections::HashSet::new(),
        document_backbone_retention: DocumentBackboneRetentionV1::default(),
        next_batch_id: 0,
        next_local_rejection_batch_id: u64::MAX,
        hlc_seed: replica_hlc_seed().ok(),
        remote_state: RemoteState::Detached,
        last_status: None,
        link: DocumentLink::opened(wall_ms()),
    };
    semio_framework_async::browser::spawn_local(async move {
        loop {
            actor.tick_link();
            actor.connect().await;
            let wake_ms = actor.next_wake_ms();
            tokio::select! {
                biased;
                cmd = cmd_rx.recv() => match cmd {
                    None => {
                        actor.document_backbone_retention.clear();
                        actor.close_socket();
                        break;
                    }
                    Some(ArtifactActorMsg::Detach) => {
                        actor.document_backbone_retention.clear();
                        let _ = actor.relay_one_backbone().await;
                        actor.close_socket();
                        break;
                    }
                    Some(message) => actor.handle_cmd(message).await,
                },
                _ = sleep_ms(wake_ms) => actor.pump_socket().await,
            }
            if actor.operation_cancel.is_cancelled_now() {
                actor.close_socket();
                break;
            }
        }
    });
}
