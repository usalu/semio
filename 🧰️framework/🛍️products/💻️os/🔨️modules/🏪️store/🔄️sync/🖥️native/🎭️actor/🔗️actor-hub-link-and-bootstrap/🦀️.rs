//! 🔗️ Actor hub link and bootstrap: the bootstrap control and requeue, the outbox, the hub connect and reconnect, the link events, the hub messages, the artifact bootstrap and the test hooks. See [`super`].

use super::*;

impl ArtifactActor {
    //#region 🔖️Hub
    pub fn bootstrap_control(&self, started_at: Instant) -> NativeBootstrapControl {
        NativeBootstrapControl { cancelled: self.closing, now_ms: started_at.elapsed().as_millis().min(u128::from(u64::MAX)) as u64, events: self.events.clone() }
    }

    pub fn abort_artifact_bootstrap(&mut self) {
        if let Some(mut pending) = self.artifact_bootstrap.take() {
            pending.assembler.abort();
        }
        self.pending_resume_token = None;
        self.required_tail_frontier = None;
    }

    pub fn requeue_pending_batches(&mut self) {
        let mut batches: Vec<(u64, Vec<MutationEnvelope>)> = self.pending_batches.drain().collect();
        batches.sort_by_key(|(batch_id, _)| *batch_id);
        for (_, envelopes) in batches {
            self.queue_outbox(envelopes);
        }
    }

    pub fn queue_outbox(&mut self, envelopes: impl IntoIterator<Item = MutationEnvelope>) {
        let mut queued: HashSet<String> = self.outbox.iter().map(|envelope| envelope.mutation_id.0.clone()).collect();
        for envelope in envelopes {
            if queued.insert(envelope.mutation_id.0.clone()) {
                self.outbox.push(envelope);
            }
        }
    }

    pub fn clear_socket_epoch(&mut self) {
        self.socket_actor = None;
        self.socket_actor_confirmed = false;
        self.session_color = None;
    }

    pub async fn fail_artifact_bootstrap(&mut self, detail: impl Into<String>) {
        self.abort_artifact_bootstrap();
        self.requeue_pending_batches();
        self.emit(ArtifactEvent::Conflict(MutationMessage { level: semio_framework_diagnostic::Severity::Error, code: semio_framework_diagnostic::FaultCode::new("artifactBootstrap"), message: detail.into(), target: vec![self.document_id.clone()], op_index: None }));
        self.semio_hub = None;
        self.clear_socket_epoch();
        self.schedule_reconnect().await;
    }

    /// ♻️ Hub lag forces a canonical pair refresh: clear the live projection tokens, keep unacked work queued,
    /// drop the socket and ask the host for the pair ([`ArtifactEvent::RebootstrapRequired`]); the actor dials again only
    /// once the host re-seeded it ([`Self::reseed_hub_document`]) — never into a Welcome it would have to refuse, and never
    /// after an outage backoff (the rebuild's own close is not a shortage).
    pub async fn require_artifact_rebootstrap(&mut self, control: semio_framework_os_directory::RebootstrapRequired) {
        self.requeue_pending_batches();
        self.abort_artifact_bootstrap();
        self.current_pack = None;
        self.current_spr = None;
        self.current_archive = None;
        self.server_frontier = None;
        self.resume_token = None;
        self.pending_resume_token = None;
        self.required_tail_frontier = None;
        self.artifact_rebootstrap_required = true;
        self.known_op_ids.clear();
        self.semio_hub = None;
        self.clear_socket_epoch();
        self.reconnect_at = None;
        self.set_remote_state(RemoteState::Connecting).await;
        self.emit(ArtifactEvent::RebootstrapRequired { control });
    }

    pub async fn flush_outbox(&mut self) {
        if self.outbox.is_empty() || self.semio_hub.is_none() {
            return;
        }
        let envelopes = std::mem::take(&mut self.outbox);
        self.relay_operations_to_hub(&envelopes).await;
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
        self.set_remote_state(RemoteState::Live { peer_count: 0 }).await;
        self.flush_outbox().await;
    }

    pub(super) async fn start_connect_hub(&mut self) {
        let Some(base_url) = self.hub_base_url.clone() else { return };
        if self.artifact_rebootstrap_required || self.semio_hub.is_some() || self.connect_future.is_some() || !self.link.admits_local_edits() || self.reconnect_at.is_some_and(|deadline| deadline > Instant::now()) {
            return;
        }
        let Some(credential) = self.credential.read().unwrap_or_else(std::sync::PoisonError::into_inner).clone() else {
            self.schedule_reconnect().await;
            return;
        };
        let Some(source) = self.socket_grant_source.read().unwrap_or_else(std::sync::PoisonError::into_inner).clone() else {
            self.schedule_reconnect().await;
            return;
        };
        let space_id = self.hub_space_id.clone().unwrap_or_default();
        let document_id = self.document_id.clone();
        let surface = self.hub_surface.clone();
        let schema = self.schema.clone();
        let Some(pack_schema_hash) = document_pack_schema_hash(&schema, self.document_execution_target_lease.as_ref()).await else {
            self.schedule_reconnect().await;
            return;
        };
        let expectation = semio_framework_os_directory_client::DocumentSocketExpectationV1 { artifact_schema: schema, pack_schema_hash, requested_surface_id: surface, lease: self.document_execution_target_lease.clone() };
        let client_instance_id = format!("native-document-{:016x}", self.hlc_seed);
        let actor_identity_grant=self.actor_identity_grant;
        if !actor_identity_admission_admits(actor_identity_grant){self.schedule_reconnect().await;return;}
        let operation_cancel = self.operation_cancel.child_now();
        let pool = self.pool.clone();
        self.set_remote_state(RemoteState::Connecting).await;
        self.connect_future = Some(Box::pin(async move {
            let ctx = semio_framework_async::OperationContext { actor: 0, generation: 0, trace: semio_framework_async::TraceId(0), lane: 1, deadline_ms: None, cancel: operation_cancel, capability: None };
            let admission_ctx = ctx.clone();
            let (admission_sender, admission_receiver) = semio_framework_async_sync::oneshot::channel();
            let admission_job = Box::new(move || {
                let admission = source.admit_document_socket(&admission_ctx, &space_id, &document_id, &expectation, &client_instance_id, 5_000);
                let _ = admission_sender.send(admission);
            });
            pool.submit_at(pool.now_ms(), semio_framework_async::Lane::Io, admission_job);
            let admission = admission_receiver.await.map_err(|_| DocumentConnectFailure::Short)?.map_err(|error| if document_admission_refuses_access(&error) { DocumentConnectFailure::Refused } else { DocumentConnectFailure::Short })?;
            if ctx.cancel.is_cancelled_now() || admission.authority.expires_at_unix_ms <= now_ms().await {
                return Err(DocumentConnectFailure::Short);
            }
            if base_url.trim_end_matches('/') != admission.authority.hub_origin.trim_end_matches('/') {
                return Err(DocumentConnectFailure::Short);
            }
            let semio_framework_os_directory_client::DocumentSocketAdmissionV1 { mut socket, authority } = admission;
            let url = hub_ws_url(
                &authority.hub_origin,
                &authority.scope.space_id,
                &authority.scope.document_id,
                Some(&authority.surface.surface_id),
            )
            .await;
            let mut request = url.into_client_request().map_err(|_| DocumentConnectFailure::Short)?;
            let session = credential.capability().map_err(|_| DocumentConnectFailure::Short)?;
            let protocol_header = format!("{}, {session}", socket.protocol);
            request.headers_mut().insert(
                tokio_tungstenite::tungstenite::http::header::SEC_WEBSOCKET_PROTOCOL,
                protocol_header.parse().map_err(|_| DocumentConnectFailure::Short)?,
            );
            let (mut stream, _response) = tokio::time::timeout(Duration::from_secs(5), tokio_tungstenite::connect_async(request)).await.map_err(|_| DocumentConnectFailure::Short)?.map_err(|_| DocumentConnectFailure::Short)?;
            if ctx.cancel.is_cancelled_now() || authority.expires_at_unix_ms <= now_ms().await {
                let _ = stream.close(None).await;
                return Err(DocumentConnectFailure::Short);
            }
            let(socket_actor,identity_progress)=semio_framework_value::SharedUtf8::admit(std::mem::take(&mut socket.actor_id),actor_identity_grant).unwrap_or_else(|(error,_)|panic!("original pre-admitted socket actor invariant: {error}"));
            Ok(ConnectedDocumentSocket { stream, socket_actor, authority,identity_progress })
        }));
    }

    pub(super) async fn finish_connect_hub(&mut self, connection: Result<ConnectedDocumentSocket, DocumentConnectFailure>) {
        match connection {
            Ok(ConnectedDocumentSocket { mut stream, socket_actor, authority,identity_progress }) => {
                add_actor_identity_progress(&mut self.actor_identity_progress,identity_progress);
                let local_schema_hash = document_pack_schema_hash(&self.schema, self.document_execution_target_lease.as_ref()).await;
                let now = now_ms().await;
                if self.operation_cancel.is_cancelled_now()
                    || authority.expires_at_unix_ms <= now
                    || authority.hub_origin.trim_end_matches('/') != self.hub_base_url.as_deref().unwrap_or_default().trim_end_matches('/')
                    || authority.scope.space_id != self.hub_space_id.as_deref().unwrap_or_default()
                    || authority.scope.document_id != self.document_id
                    || authority.artifact.schema != self.schema
                    || local_schema_hash != Some(authority.pack_schema_hash)
                    || self.hub_surface.as_ref().is_some_and(|surface| surface != &authority.surface.surface_id)
                    || self.document_execution_target_lease.as_ref().is_some_and(|lease| !authority.matches_lease_fields(lease))
                {
                    let _ = stream.close(None).await;
                    self.clear_socket_epoch();
                    self.schedule_reconnect().await;
                    return;
                }
                let pack_schema_hash = authority.pack_schema_hash;
                let (write, read) = stream.split();
                self.semio_hub = Some(HubConn { write, read });
                self.socket_actor = Some(socket_actor);
                self.socket_actor_confirmed = false;
                self.hub_surface = Some(authority.surface.surface_id.clone());
                self.session_color = None;
                let hello = ClientFrame::SocketHelloV1 { wire_version: 1, protocol_version: 1, schema: self.schema.clone(), pack_schema_hash, resume_token: self.resume_token.clone(), frontier: self.server_frontier.clone() };
                self.send_client_frame(hello, Lane::Command).await;
            }
            Err(DocumentConnectFailure::Refused) => {
                self.clear_socket_epoch();
                self.link_event(DocumentLinkEvent::Refused { now_ms: wall_ms() }).await;
            }
            Err(DocumentConnectFailure::Short) => {
                self.clear_socket_epoch();
                self.schedule_reconnect().await;
            }
        }
    }

    /// 🔁️ A failed attempt or a lost socket: the link backs off along [`DOCUMENT_LINK_SHORTAGE_POLICY`] — or
    /// expires, once the shortage outlasts its bound.
    pub async fn schedule_reconnect(&mut self) {
        if let Some(retry_in_ms) = self.fail_link().await {
            self.set_remote_state(RemoteState::Backoff { retry_in_ms }).await;
        }
    }

    /// 🔌️ Applies `Failed` to the link and arms its two deadlines; the retry delay while it stays unlinked.
    async fn fail_link(&mut self) -> Option<u64> {
        let now_ms = wall_ms();
        self.link_event(DocumentLinkEvent::Failed { now_ms }).await;
        let retry_at_ms = self.link.retry_at_ms()?;
        self.reconnect_at = Some(instant_at(retry_at_ms, now_ms));
        self.link_expires_at = self.link.expires_at_ms(&DOCUMENT_LINK_SHORTAGE_POLICY).map(|at_ms| instant_at(at_ms, now_ms));
        Some(retry_at_ms.saturating_sub(now_ms))
    }

    /// 🔌️ Applies one link event; a link that turns terminal drops its socket and deadlines, reports
    /// `Detached` and emits the coded terminal message a shell localizes ([`document_link_terminal_message`]).
    pub async fn link_event(&mut self, event: DocumentLinkEvent) {
        let was_open = self.link.admits_local_edits();
        self.link = self.link.apply(&DOCUMENT_LINK_SHORTAGE_POLICY, event);
        if was_open && !self.link.admits_local_edits() {
            self.abort_artifact_bootstrap();
            self.requeue_pending_batches();
            self.connect_future = None;
            if let Some(mut connection) = self.semio_hub.take() {
                let _ = tokio::time::timeout(Duration::from_millis(4), connection.write.close()).await;
            }
            self.clear_socket_epoch();
            self.reconnect_at = None;
            self.link_expires_at = None;
            self.set_remote_state(RemoteState::Detached).await;
            let _ = self.events.send(ArtifactEvent::Conflict(document_link_terminal_message(&self.document_id, self.link.status())));
        }
    }

    /// ⏳️ Lets time pass for an unlinked hub link: past the shortage bound it expires.
    pub async fn tick_link(&mut self) {
        if self.hub_base_url.is_some() && matches!(self.link, DocumentLink::Unlinked { .. }) {
            self.link_event(DocumentLinkEvent::Tick { now_ms: wall_ms() }).await;
        }
    }

    pub(super) async fn on_hub_message(&mut self, message: Option<Result<Message, tokio_tungstenite::tungstenite::Error>>) {
        match message {
            Some(Ok(Message::Binary(bytes))) => match decode_server_frame(&bytes).await {
                Ok((_lane, frame)) => self.on_hub_frame(frame).await,
                Err(error) => self.fail_artifact_bootstrap(format!("malformed hub frame: {error}")).await,
            },
            Some(Ok(Message::Ping(payload))) => {
                self.send_raw(Message::Pong(payload)).await;
            }
            Some(Ok(_)) => {}
            Some(Err(_)) | None => {
                self.abort_artifact_bootstrap();
                self.requeue_pending_batches();
                self.semio_hub = None;
                self.clear_socket_epoch();
                self.schedule_reconnect().await;
            }
        }
    }

    //#endregion 🔖️Hub
}

impl ArtifactActor {
    //#region 🔖️Hub
    pub async fn start_artifact_bootstrap(&mut self, bootstrap: ArtifactBootstrap, resume_token: String, server_frontier: RuntimeFrontierSummary) {
        self.abort_artifact_bootstrap();
        let Some(local_schema_hash) = document_pack_schema_hash(&self.schema, self.document_execution_target_lease.as_ref()).await else {
            self.fail_artifact_bootstrap(format!("no pack schema identity resolves schema {:?}", self.schema)).await;
            return;
        };
        if let Err(error) = validate_artifact_bootstrap_identity(&bootstrap, &self.document_id, &self.schema, local_schema_hash, &server_frontier) {
            self.fail_artifact_bootstrap(error).await;
            return;
        }
        let inline = bootstrap.inline.is_some();
        let started_at = Instant::now();
        let baseline_frontier = bootstrap.baseline_frontier.clone();
        let required_tail_frontier = bootstrap.required_tail_frontier.clone();
        let pack_schema_hash = bootstrap.pack_schema_hash;
        let mut control = self.bootstrap_control(started_at);
        let assembler = match ArtifactBootstrapAssembler::new(bootstrap.clone(), bootstrap.descriptor_hash, ArtifactBootstrapLimits::default(), Some(ARTIFACT_BOOTSTRAP_DEADLINE_MS), &mut control) {
            Ok(assembler) => assembler,
            Err(error) => {
                self.fail_artifact_bootstrap(error.to_string()).await;
                return;
            }
        };
        let mut pending = PendingArtifactBootstrap { assembler, started_at, resume_token, baseline_frontier, required_tail_frontier, pack_schema_hash };
        if !inline {
            self.artifact_bootstrap = Some(pending);
            return;
        }
        let mut control = self.bootstrap_control(started_at);
        match pending.assembler.finish(None, &mut control) {
            Ok(pair) => {
                if let Err(error) = self.install_artifact_bootstrap(pending, pair).await {
                    self.fail_artifact_bootstrap(error).await;
                }
            }
            Err(error) => self.fail_artifact_bootstrap(error.to_string()).await,
        }
    }

    pub async fn install_artifact_bootstrap(&mut self, pending: PendingArtifactBootstrap, pair: ArtifactBootstrapPair) -> Result<(), String> {
        let codec = semio_framework_os_store_codec_registry::document_kind_codec(&self.schema).await.map_err(|error| error.to_string())?.ok_or_else(|| format!("no document codec resolves schema {:?}", self.schema))?;
        if codec.pack_schema_hash().await.map_err(|error| error.to_string())? != pending.pack_schema_hash {
            return Err("artifact bootstrap codec changed during transfer".into());
        }
        codec.print_mirror(&pair.pack, &pair.spr).await.map_err(|error| format!("artifact bootstrap decode failed: {error}"))?;
        let op_ids = spr_op_ids(&pair.spr).await.map_err(|error| format!("artifact bootstrap SPR failed: {error}"))?;
        let archive = semio_framework_os_spr::DocumentArchivePack { parent_pack: pair.pack.clone(), parent_spr: pair.spr.clone(), members: Vec::new() };
        let archive_bytes = semio_framework_os_spr::encode_document_archive_bytes(&archive).map_err(|error| format!("artifact bootstrap archive failed: {error}"))?;
        let previous = self.current_archive.clone();
        if let Some(folder) = self.folder.as_ref() {
            folder.write_archive(&archive_bytes).await.map_err(|error| format!("artifact bootstrap persistence failed: {error}"))?;
        }
        let events = spr_events(&pair.spr, &self.document_id, &self.schema,self.actor_identity_grant).await.map_err(|error| format!("artifact bootstrap SPR failed: {error}"))?;
        let pushed = match self.remote.push(BackboneMessage::Genesis { pack: pair.pack.clone() }).await {
            Ok(()) if !events.is_empty() => self.remote.push(BackboneMessage::Mutations { envelopes: encode_envelopes(&events) }).await,
            result => result,
        };
        if let Err(error) = pushed {
            if let (Some(folder), Some(previous)) = (self.folder.as_ref(), previous) {
                let _ = folder.write_archive(&previous).await;
            }
            return Err(format!("artifact bootstrap event delivery failed: {error}"));
        }
        self.known_op_ids = op_ids;
        self.current_pack = Some(pair.pack.clone());
        self.current_spr = Some(pair.spr.clone());
        self.current_archive = Some(archive_bytes.clone());
        if self.folder.is_some() {
            self.last_written_hash = Some(document_archive_hash(&archive_bytes));
        }
        self.emit(ArtifactEvent::DocumentArchiveReplaced { archive: archive_bytes });
        self.server_frontier = Some(pending.baseline_frontier);
        self.pending_resume_token = Some(pending.resume_token);
        self.required_tail_frontier = Some(pending.required_tail_frontier);
        if !self.outbox.is_empty() && self.replay_local_outbox_after_bootstrap().await.is_err() {
            self.emit(ArtifactEvent::Conflict(MutationMessage {
                level: semio_framework_diagnostic::Severity::Error,
                code: semio_framework_diagnostic::FaultCode::new("artifactBootstrapLocalReplay"),
                message: "artifact baseline committed; pending local replay will retry after reconnect".into(),
                target: vec![self.document_id.clone()],
                op_index: None,
            }));
            self.semio_hub = None;
            self.clear_socket_epoch();
            self.schedule_reconnect().await;
            return Ok(());
        }
        self.finish_catchup_if_ready().await;
        Ok(())
    }

    async fn replay_local_outbox_after_bootstrap(&mut self) -> Result<(), semio_framework_os_vcs::VcsError> {
        #[cfg(any(test, feature = "testing"))]
        if std::mem::take(&mut self.fail_bootstrap_local_replay_once) {
            return Err(semio_framework_os_vcs::VcsError::Backbone("injected bootstrap local replay failure".into()));
        }
        self.remote.push(BackboneMessage::Mutations { envelopes: encode_envelopes(&self.outbox) }).await
    }

    #[cfg(any(test, feature = "testing"))]
    pub async fn inject_hub_frame(&mut self, frame: ServerFrame) {
        self.on_hub_frame(frame).await;
    }

    #[cfg(any(test, feature = "testing"))]
    pub async fn handle_test_cmd(&mut self, message: ArtifactActorMsg) -> bool {
        self.handle_cmd(message).await
    }

    #[cfg(any(test, feature = "testing"))]
    pub fn inject_bootstrap_local_replay_failure(&mut self) {
        self.fail_bootstrap_local_replay_once = true;
    }

    #[cfg(any(test, feature = "testing"))]
    pub fn cancel_test_bootstrap(&mut self) {
        self.abort_artifact_bootstrap();
    }

    #[cfg(any(test, feature = "testing"))]
    pub fn queue_test_outbox(&mut self, envelopes: Vec<MutationEnvelope>) {
        self.queue_outbox(envelopes);
    }

    #[cfg(any(test, feature = "testing"))]
    pub fn install_test_socket_actor(&mut self, actor: &str) {
        let(original,receipt)=semio_framework_value::SharedUtf8::admit(actor.to_owned(),self.actor_identity_grant).unwrap_or_else(|(error,_)|panic!("original fixture actor admission: {error}"));add_actor_identity_progress(&mut self.actor_identity_progress,receipt);self.socket_actor = Some(original);
        self.socket_actor_confirmed = false;
    }

    #[cfg(any(test, feature = "testing"))]
    pub async fn connect_test_socket(&mut self, url: &str, actor: &str) {
        let (stream, _) = tokio_tungstenite::connect_async(url).await.expect("test socket connects");
        let (write, read) = stream.split();
        self.semio_hub = Some(HubConn { write, read });
        let(original,receipt)=semio_framework_value::SharedUtf8::admit(actor.to_owned(),self.actor_identity_grant).unwrap_or_else(|(error,_)|panic!("original fixture actor admission: {error}"));add_actor_identity_progress(&mut self.actor_identity_progress,receipt);self.socket_actor = Some(original);
        self.socket_actor_confirmed = false;
        self.session_color = None;
        self.send_client_frame(
            ClientFrame::SocketHelloV1 {
                wire_version: 1,
                protocol_version: 1,
                schema: self.schema.clone(),
                pack_schema_hash: semio_framework_os_store_codec_registry::document_codec(&self.schema).await.ok().flatten().map_or([0u8; 32], |codec| codec.pack_schema_hash),
                resume_token: self.resume_token.clone(),
                frontier: self.server_frontier.clone(),
            },
            Lane::Command,
        )
        .await;
    }

    #[cfg(any(test, feature = "testing"))]
    pub fn socket_epoch_test_state(&self) -> (Option<String>, bool, usize, Vec<String>) {
        (self.socket_actor.as_ref().map(|actor|actor.as_str().to_owned()), self.socket_actor_confirmed, self.pending_batches.len(), self.outbox.iter().map(|envelope| envelope.actor.0.as_str().to_owned()).collect())
    }

    #[cfg(any(test, feature = "testing"))]
    pub async fn fail_test_connection(&mut self) {
        self.on_hub_message(None).await;
    }

    #[cfg(any(test, feature = "testing"))]
    pub async fn fail_test_bootstrap(&mut self) {
        self.fail_artifact_bootstrap("injected bootstrap failure").await;
    }

    #[cfg(any(test, feature = "testing"))]
    pub async fn run_test_connect_attempt(&mut self) {
        self.start_connect_hub().await;
        if let Some(future) = self.connect_future.take() {
            let outcome = future.await;
            self.finish_connect_hub(outcome).await;
        }
    }

    #[cfg(any(test, feature = "testing"))]
    pub async fn relay_test_envelope(&mut self, envelope: MutationEnvelope) {
        self.relay_operations_to_hub(std::slice::from_ref(&envelope)).await;
    }

    #[cfg(any(test, feature = "testing"))]
    pub async fn handle_test_document_archive(&mut self, archive: Vec<u8>) {
        let _ = self.handle_cmd(ArtifactActorMsg::LocalDocumentArchive { archive }).await;
    }

    #[cfg(any(test, feature = "testing"))]
    pub fn current_archive_test(&self) -> Option<Vec<u8>> {
        self.current_archive.clone()
    }

    #[cfg(any(test, feature = "testing"))]
    pub async fn setup_test(&mut self) {
        self.setup().await;
    }

    #[cfg(any(test, feature = "testing"))]
    pub fn folder_writes_test(&self) -> usize {
        self.folder_writes
    }

    #[cfg(any(test, feature = "testing"))]
    pub fn bootstrap_test_state(&self) -> (Option<Vec<u8>>, Option<Vec<u8>>, Option<RuntimeFrontierSummary>, Option<RuntimeFrontierSummary>, Option<String>, Option<String>, RemoteState, Vec<String>, bool) {
        (
            self.current_pack.clone(),
            self.current_spr.clone(),
            self.server_frontier.clone(),
            self.required_tail_frontier.clone(),
            self.resume_token.clone(),
            self.pending_resume_token.clone(),
            self.remote_state.clone(),
            self.outbox.iter().map(|envelope| envelope.mutation_id.0.clone()).collect(),
            self.artifact_rebootstrap_required,
        )
    }

    #[cfg(any(test, feature = "testing"))]
    pub fn parity_test_state(&self) -> (Option<String>, Option<String>, RemoteState, Vec<String>, usize, bool, bool, Vec<String>) {
        (
            self.resume_token.clone(),
            self.server_frontier.as_ref().map(|frontier| frontier.head_edit_id.clone()),
            self.remote_state.clone(),
            self.outbox.iter().map(|envelope| envelope.mutation_id.0.clone()).collect(),
            self.pending_batches.len(),
            self.socket_actor_confirmed,
            self.artifact_rebootstrap_required,
            self.known_op_ids.union(&self.applied_op_ids).cloned().collect(),
        )
    }

    //#endregion 🔖️Hub
}
