//! 🚇️ Wasm actor relay and bootstrap: the operation relay, the oversized batch refusal, the command handling, the outbox and the artifact bootstrap start, abort, reseed and install. See [`super`].

use super::*;

impl WasmActor {
    //#region 🔖️Relay
    /// 🧺️ Builds + sends `Commands` batches under the hub-issued socket actor, tracking each in
    /// `pending_batches` for {@link WasmActor::handle_ack}. Mirrors the native `relay_operations_to_hub`:
    /// nothing leaves before the hub confirmed the socket actor and every catch-up completed. A batch
    /// whose frame exceeds the socket's own ceiling is halved until it fits; one envelope that can never
    /// fit is rolled back locally and reported, never retried into a reconnect loop.
    async fn relay_operations(&mut self, envelopes: &[MutationEnvelope]) {
        if envelopes.is_empty() {
            return;
        }
        note_authored_envelopes(&mut self.applied_op_ids, envelopes);
        let Some(socket_actor) = self.socket_actor.as_ref() else {
            self.queue_outbox(envelopes.iter().cloned());
            return;
        };
        if !self.socket_actor_confirmed || self.socket.is_none() || self.artifact_bootstrap.is_some() || self.required_tail_frontier.is_some() {
            self.queue_outbox(envelopes.iter().cloned());
            return;
        }
        let max_frame = self.socket.as_ref().map_or(usize::MAX, |socket| socket.max_frame_bytes());
        let(wire_envelopes,identity_receipt)=match wire_envelopes_with_original_actor(envelopes,socket_actor,self.actor_identity_grant){Ok(result)=>result,Err(_)=>{self.queue_outbox(envelopes.iter().cloned());return;}};
        add_actor_identity_progress(&mut self.actor_identity_progress,identity_receipt);
        let plan = commands_frames_within(max_frame, self.next_batch_id, envelopes.to_vec(), wire_envelopes).await;
        for frame in plan.frames {
            self.next_batch_id = frame.batch_id.wrapping_add(1);
            self.pending_batches.insert(frame.batch_id, frame.local);
            self.send_encoded(frame.bytes).await;
        }
        if !plan.oversized.is_empty() {
            self.refuse_oversized_batch(plan.oversized).await;
        }
        self.emit_status_if_changed();
    }

    /// 📏️ One envelope whose `Commands` frame exceeds the socket's ceiling on its own: the hub can never
    /// receive it, so its speculative local head is rolled back and the refusal is published.
    async fn refuse_oversized_batch(&mut self, local: Vec<MutationEnvelope>) {
        self.document_backbone_retention.release(&local);
        let mut rollbacks: Vec<MutationEnvelope> = Vec::with_capacity(local.len());
        for envelope in local.iter().rev() {
            rollbacks.extend(rollback_envelope(envelope).await);
        }
        let _ = self.deliver_remote_operations(rollbacks).await;
        let refusal = self.retract_refused_transitions(&local, "document socket frame exceeds the socket's ceiling").await;
        self.reject_document_backbone(CommandAckOutcome::local_rejected(CommandRejectionCode::LocalSocketFrameCeiling, "document socket frame exceeds the socket's ceiling", CommandRejectionDetail { envelopes: Some(local.len() as u64), ..CommandRejectionDetail::default() }));
        if let Some(refusal) = refusal {
            let _ = self.events.send(ArtifactEvent::Conflict(refusal));
        }
    }

    pub async fn relay_one_backbone(&mut self) -> Result<bool, semio_framework_os_vcs::VcsError> {
        let Some(message) = self.remote.try_pop_front()? else { return Ok(false) };
        match message {
            BackboneMessage::Mutations { envelopes } => {
                let envelopes = decode_envelopes(&envelopes).unwrap_or_default();
                self.relay_operations(&envelopes).await;
            }
            BackboneMessage::Genesis { .. } | BackboneMessage::Ack { .. } => {}
            BackboneMessage::Member { .. } => return Err(semio_framework_os_vcs::VcsError::Backbone("a composed member requires its exact member transport lane".into())),
            BackboneMessage::Retract { .. } => return Err(semio_framework_os_vcs::VcsError::Backbone("a retraction flows from the actor to its store, never back".into())),
        }
        Ok(true)
    }

    pub async fn handle_cmd(&mut self, message: ArtifactActorMsg) {
        match message {
            ArtifactActorMsg::LocalMutations { envelopes } => {
                let drained = self.relay_one_backbone().await.unwrap_or(false);
                if !drained && !envelopes.is_empty() {
                    self.relay_operations(&envelopes).await;
                }
            }
            ArtifactActorMsg::DocumentBackbone { message } => {
                let envelopes = match decode_document_backbone_message_exact(&message) {
                    Ok(envelopes) if envelopes.iter().all(|envelope| envelope.document_id.0 == self.document_id) => envelopes,
                    Ok(envelopes) => {
                        self.reject_document_backbone(CommandAckOutcome::local_rejected(CommandRejectionCode::LocalBackboneScopeMismatch, "document backbone scope mismatch", CommandRejectionDetail { envelopes: Some(envelopes.len() as u64), ..CommandRejectionDetail::default() }));
                        return;
                    }
                    Err(_) => {
                        self.reject_document_backbone(CommandAckOutcome::local_rejected(CommandRejectionCode::LocalBackboneMalformed, "document backbone malformed", CommandRejectionDetail { bytes: Some(message.len() as u64), ..CommandRejectionDetail::default() }));
                        return;
                    }
                };
                if let Err(refusal) = self.document_backbone_retention.retain(message.len(), &envelopes) {
                    self.reject_document_backbone(refusal);
                    return;
                }
                if self.hub_space_id.is_some() {
                    self.relay_operations(&envelopes).await;
                } else {
                    self.document_backbone_retention.release(&envelopes);
                }
            }
            ArtifactActorMsg::LocalDocumentArchive { archive } => {
                let _ = semio_framework_os_spr::decode_document_archive_bytes(&archive).await;
            }
            ArtifactActorMsg::PresenceHeartbeat { mut peer } => {
                stamp_session(&mut peer, self.session_color, self.hub_surface.as_deref()).await;
                self.send_frame(&ClientFrame::Presence { peer: presence_to_bytes(&peer).await }, Lane::Preview).await;
            }
            ArtifactActorMsg::PublishPreview { key, seq, payload } => {
                self.send_frame(&ClientFrame::PreviewPublish { key, seq, payload }, Lane::Preview).await;
            }
            ArtifactActorMsg::Reseed { baseline, .. } => self.reseed_hub_document(baseline),
            ArtifactActorMsg::ExternalChanged | ArtifactActorMsg::Detach => {}
        }
    }

    pub fn reject_document_backbone(&mut self, outcome: CommandAckOutcome) {
        let batch_id = self.next_local_rejection_batch_id;
        self.next_local_rejection_batch_id = self.next_local_rejection_batch_id.wrapping_sub(1);
        let _ = self.events.send(ArtifactEvent::CommandOutcome { batch_id, outcome });
    }

    pub fn requeue_pending_batches(&mut self) {
        let mut batches: Vec<(u64, Vec<MutationEnvelope>)> = self.pending_batches.drain().collect();
        batches.sort_by_key(|(batch_id, _)| *batch_id);
        for (_, envelopes) in batches {
            self.queue_outbox(envelopes);
        }
    }

    pub fn queue_outbox(&mut self, envelopes: impl IntoIterator<Item = MutationEnvelope>) {
        let mut queued: std::collections::HashSet<String> = self.outbox.iter().map(|envelope| envelope.mutation_id.0.clone()).collect();
        for envelope in envelopes {
            if queued.insert(envelope.mutation_id.0.clone()) {
                self.outbox.push(envelope);
            }
        }
    }

    pub async fn flush_outbox(&mut self) {
        if self.outbox.is_empty() || self.socket.is_none() {
            return;
        }
        let envelopes = std::mem::take(&mut self.outbox);
        self.relay_operations(&envelopes).await;
    }
    //#endregion 🔖️Relay

    //#region 🔖️Bootstrap
    pub fn bootstrap_control(&self, cancelled: bool) -> WasmBootstrapControl {
        WasmBootstrapControl { cancelled, now_ms: wall_ms(), events: self.events.clone() }
    }

    pub fn abort_artifact_bootstrap(&mut self) {
        if let Some(mut pending) = self.artifact_bootstrap.take() {
            pending.assembler.abort();
        }
        self.pending_resume_token = None;
        self.required_tail_frontier = None;
    }

    /// ⚠️ A bootstrap or protocol fault is reported as the native actor reports it — one `Conflict` —
    /// and the socket reconnects for a fresh catch-up.
    pub fn fail_artifact_bootstrap(&mut self, detail: impl Into<String>) {
        let _ = self.events.send(ArtifactEvent::Conflict(MutationMessage { level: semio_framework_diagnostic::Severity::Error, code: semio_framework_diagnostic::FaultCode::new("artifactBootstrap"), message: detail.into(), target: vec![self.document_id.clone()], op_index: None }));
        self.disconnect();
    }

    /// ♻️ Hub lag forces a canonical pair refresh: keep unacked work queued, clear the projection tokens, drop the socket
    /// and ask the host for the pair — the native `require_artifact_rebootstrap`; no dial until the host's reseed.
    pub fn require_artifact_rebootstrap(&mut self, control: semio_framework_os_directory::RebootstrapRequired) {
        self.requeue_pending_batches();
        self.abort_artifact_bootstrap();
        self.server_frontier = None;
        self.resume_token = None;
        self.artifact_rebootstrap_required = true;
        self.close_socket();
        self.set_remote_state(RemoteState::Connecting);
        let _ = self.events.send(ArtifactEvent::RebootstrapRequired { control });
    }

    /// 🛟️ The browser twin of the native reseed: the pair's baseline is the next Hello's frontier, the unacked local
    /// operations go back to the guest, and the next turn dials at once.
    pub fn reseed_hub_document(&mut self, baseline: semio_framework_os_directory::ArtifactFrontier) {
        if !self.artifact_rebootstrap_required {
            return;
        }
        self.server_frontier = Some(canonical_pair_baseline(&baseline));
        if !self.outbox.is_empty() {
            let _ = self.events.send(ArtifactEvent::RemoteMutations { envelopes: self.outbox.clone() });
        }
        self.artifact_rebootstrap_required = false;
    }

    pub async fn finish_catchup_if_ready(&mut self) {
        let Some(required) = self.required_tail_frontier.clone() else { return };
        let Some(actual) = self.server_frontier.as_ref() else { return };
        if !frontier_reaches(actual, &required) {
            return;
        }
        self.required_tail_frontier = None;
        if let Some(resume_token) = self.pending_resume_token.take() {
            self.resume_token = Some(resume_token);
        }
        self.set_remote_state(RemoteState::Live { peer_count: 0 });
        self.flush_outbox().await;
    }

    pub async fn start_artifact_bootstrap(&mut self, bootstrap: ArtifactBootstrap, resume_token: String, server_frontier: RuntimeFrontierSummary) {
        self.abort_artifact_bootstrap();
        let Some(pack_schema_hash) = document_pack_schema_hash(&self.schema, self.document_execution_target_lease.as_ref()).await else {
            self.fail_artifact_bootstrap(format!("no pack schema identity for schema {:?}", self.schema));
            return;
        };
        if let Err(error) = validate_artifact_bootstrap_identity(&bootstrap, &self.document_id, &self.schema, pack_schema_hash, &server_frontier) {
            self.fail_artifact_bootstrap(error);
            return;
        }
        let inline = bootstrap.inline.is_some();
        let started_ms = wall_ms();
        let baseline_frontier = bootstrap.baseline_frontier.clone();
        let required_tail_frontier = bootstrap.required_tail_frontier.clone();
        let mut control = self.bootstrap_control(false);
        let assembler = match ArtifactBootstrapAssembler::new(bootstrap.clone(), bootstrap.descriptor_hash, ArtifactBootstrapLimits::default(), Some(started_ms.saturating_add(ARTIFACT_BOOTSTRAP_DEADLINE_MS)), &mut control) {
            Ok(assembler) => assembler,
            Err(error) => {
                self.fail_artifact_bootstrap(error.to_string());
                return;
            }
        };
        let mut pending = PendingWasmArtifactBootstrap { assembler, started_ms, resume_token, baseline_frontier, required_tail_frontier, pack_schema_hash };
        if !inline {
            self.artifact_bootstrap = Some(pending);
            return;
        }
        let mut control = self.bootstrap_control(false);
        match pending.assembler.finish(None, &mut control) {
            Ok(pair) => {
                if let Err(error) = self.install_artifact_bootstrap(pending, pair).await {
                    self.fail_artifact_bootstrap(error);
                }
            }
            Err(error) => self.fail_artifact_bootstrap(error.to_string()),
        }
    }

    /// 🗃️ Installs one assembled canonical pair. With a codec this process resolves for the kind
    /// (linked, or the mounted component's — [`crate::os_store::codec_registry::document_kind_codec`]) the pair is
    /// decoded here first; without one the pair's identity was already bound to the verified lease's
    /// pack schema hash, and the mounted guest that owns the kind validates the archive when the shell
    /// loads it.
    pub async fn install_artifact_bootstrap(&mut self, pending: PendingWasmArtifactBootstrap, pair: ArtifactBootstrapPair) -> Result<(), String> {
        if let Some(codec) = semio_framework_os_store_codec_registry::document_kind_codec(&self.schema).await.map_err(|error| error.to_string())? {
            if codec.pack_schema_hash().await.map_err(|error| error.to_string())? != pending.pack_schema_hash {
                return Err("artifact bootstrap codec changed during transfer".into());
            }
            codec.print_mirror(&pair.pack, &pair.spr).await.map_err(|error| format!("artifact bootstrap decode failed: {error}"))?;
        }
        let events = spr_events(&pair.spr, &self.document_id, &self.schema,self.actor_identity_grant).await.map_err(|error| format!("artifact bootstrap SPR failed: {error}"))?;
        self.remote.push(BackboneMessage::Genesis { pack: pair.pack.clone() }).await.map_err(|error| format!("artifact bootstrap event delivery failed: {error}"))?;
        if !events.is_empty() {
            self.remote.push(BackboneMessage::from_envelopes(&events)).await.map_err(|error| format!("artifact bootstrap event delivery failed: {error}"))?;
        }
        let archive = semio_framework_os_spr::DocumentArchivePack { parent_pack: pair.pack, parent_spr: pair.spr, members: Vec::new() };
        let archive = semio_framework_os_spr::encode_document_archive_bytes(&archive).map_err(|error| format!("artifact bootstrap archive failed: {error}"))?;
        let _ = self.events.send(ArtifactEvent::DocumentArchiveReplaced { archive });
        self.server_frontier = Some(pending.baseline_frontier);
        self.pending_resume_token = Some(pending.resume_token);
        self.required_tail_frontier = Some(pending.required_tail_frontier);
        if !self.outbox.is_empty() && self.remote.push(BackboneMessage::from_envelopes(&self.outbox)).await.is_err() {
            let _ = self.events.send(ArtifactEvent::Conflict(MutationMessage {
                level: semio_framework_diagnostic::Severity::Error,
                code: semio_framework_diagnostic::FaultCode::new("artifactBootstrapLocalReplay"),
                message: "artifact baseline committed; pending local replay will retry after reconnect".into(),
                target: vec![self.document_id.clone()],
                op_index: None,
            }));
            self.close_socket();
            self.schedule_reconnect();
            return Ok(());
        }
        self.finish_catchup_if_ready().await;
        Ok(())
    }
    //#endregion 🔖️Bootstrap

}
