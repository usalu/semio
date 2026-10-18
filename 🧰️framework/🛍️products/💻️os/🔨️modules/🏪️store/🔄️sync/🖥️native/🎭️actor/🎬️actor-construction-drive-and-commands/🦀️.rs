//! 🎬️ Actor construction, drive and commands: the actor constructor, the hub seed, the drive of one step, the hub message poll, the setup, the command handling and the backbone relay. See [`super`].

use super::*;

impl ArtifactActor {
    pub async fn new(
        pool: Arc<semio_framework_async::WorkerPool>,
        config: ArtifactActorConfig,
        remote: ChannelBackboneRemote,
        cmd_rx: ArtifactMailboxReceiver,
        events: broadcast::Sender<ArtifactEvent>,
        credential: Arc<std::sync::RwLock<Option<Arc<semio_framework_os_directory_client::LocalHubCredential>>>>,
        socket_grant_source: Arc<std::sync::RwLock<Option<Arc<dyn semio_framework_os_directory_client::HubSocketGrantSource>>>>,
        document_execution_target_lease: Option<semio_framework_os_directory::DocumentExecutionTargetLeaseFieldsV1>,
        operation_cancel: semio_framework_async::CancelToken,
    ) -> Self {
        let mut folder = None;
        let mut folder_watch_path = None;
        let mut hub_base_url = None;
        let mut hub_space_id = None;
        let mut hub_surface = None;
        for binding in &config.bindings {
            match binding {
                PersistenceBinding::Folder { path } => {
                    if folder.is_none() {
                        folder = Some(build_folder_endpoint(path, &config.document_id, &config.schema).await);
                        folder_watch_path = Some(folder_watch_path_for(path).await);
                    }
                }
                PersistenceBinding::Hub { base_url, space_id, surface } => {
                    if hub_base_url.is_none() {
                        hub_base_url = Some(base_url.clone());
                        hub_space_id = Some(space_id.clone());
                        hub_surface = surface.clone();
                    }
                }
            }
        }
        let hlc_seed = actor_seed(&config.actor).await;
        Self {
            pool,
            document_id: config.document_id,
            schema: config.schema,
            actor: config.actor,
            actor_identity_grant: config.actor_identity_grant,
            actor_identity_progress: Default::default(),
            remote,
            events,
            cmd_rx,
            folder,
            folder_watch_path,
            watch_external: config.watch_external,
            hub_base_url,
            hub_space_id,
            credential,
            socket_grant_source,
            document_execution_target_lease,
            operation_cancel,
            hub_surface,
            socket_actor: None,
            socket_actor_confirmed: false,
            session_color: None,
            semio_hub: None,
            connect_future: None,
            server_frontier: None,
            resume_token: None,
            pending_resume_token: None,
            required_tail_frontier: None,
            artifact_bootstrap: None,
            artifact_rebootstrap_required: false,
            link: DocumentLink::opened(wall_ms()),
            reconnect_at: None,
            link_expires_at: None,
            pending_batches: std::collections::HashMap::new(),
            outbox: Vec::new(),
            document_backbone_retention: DocumentBackboneRetentionV1::default(),
            next_batch_id: 0,
            next_local_rejection_batch_id: u64::MAX,
            hlc_seed,
            hlc_counter: 0,
            current_pack: None,
            current_spr: None,
            current_archive: None,
            known_op_ids: HashSet::new(),
            applied_op_ids: HashSet::new(),
            last_written_hash: None,
            folder_held_hash: None,
            folder_bootstrapped: false,
            #[cfg(test)]
            folder_writes: 0,
            remote_state: RemoteState::Detached,
            last_status: None,
            watcher: None,
            fs_deadline: None,
            started: false,
            command_turns: 0,
            drive_phase: ArtifactDrivePhase::Connect,
            closing: false,
            close_drain_deadline: None,
            readiness: None,
            readiness_requested: Arc::new(std::sync::atomic::AtomicBool::new(false)),
            #[cfg(test)]
            fail_bootstrap_local_replay_once: false,
        }
    }

    pub(super) fn set_readiness(&mut self, readiness: Arc<dyn Fn() + Send + Sync>) {
        self.readiness = Some(readiness);
    }

    /// 🪢️ Starts this hub document at the canonical checkpoint pair its shell verified and already loaded into the
    /// guest: the pair's history is known (a tail never re-applies it), a folder binding persists it, and the first
    /// `SocketHelloV1` names its baseline, so the hub sends only what followed it. A pair whose SPR cannot be read
    /// is reported as a conflict and the document starts unseeded.
    pub async fn seed_hub_document(&mut self, pair: semio_framework_os_directory::CanonicalCheckpointPairV1) {
        self.install_hub_seed(pair.pack_bytes, pair.spr_bytes, &pair.baseline_frontier).await;
    }

    /// 🛟️ Restarts a rebootstrapping hub document at the pair the host verified and loaded into the guest
    /// ([`ArtifactActorMsg::Reseed`]): the pair becomes the baseline, the unacked local operations the pair does not already
    /// hold go back to the guest, and the actor dials at once — no outage backoff, Hello at the baseline.
    pub async fn reseed_hub_document(&mut self, pack: Vec<u8>, spr: Vec<u8>, baseline: semio_framework_os_directory::ArtifactFrontier) {
        if !self.artifact_rebootstrap_required {
            return;
        }
        if !self.install_hub_seed(pack, spr, &baseline).await {
            return;
        }
        let known = &self.known_op_ids;
        self.outbox.retain(|envelope| !known.contains(&envelope.mutation_id.0));
        if !self.outbox.is_empty() {
            self.emit(ArtifactEvent::RemoteMutations { envelopes: self.outbox.clone() });
        }
        self.artifact_rebootstrap_required = false;
        self.reconnect_at = None;
        self.start_connect_hub().await;
    }

    /// 🪢️ Adopts one verified pair as this document's baseline; `false` (and one conflict) when its SPR cannot be read.
    async fn install_hub_seed(&mut self, pack: Vec<u8>, spr: Vec<u8>, baseline: &semio_framework_os_directory::ArtifactFrontier) -> bool {
        let seeded = match spr_op_ids(&spr).await {
            Ok(op_ids) => semio_framework_os_spr::encode_document_archive_bytes(&semio_framework_os_spr::DocumentArchivePack { parent_pack: pack.clone(), parent_spr: spr.clone(), members: Vec::new() }).map(|archive| (op_ids, archive)).map_err(|error| error.to_string()),
            Err(error) => Err(error),
        };
        let (op_ids, archive) = match seeded {
            Ok(seeded) => seeded,
            Err(error) => {
                self.emit(ArtifactEvent::Conflict(MutationMessage {
                    level: semio_framework_diagnostic::Severity::Error,
                    code: semio_framework_diagnostic::FaultCode::new("hubDocumentSeed"),
                    message: format!("the canonical checkpoint pair could not seed the document: {error}"),
                    target: vec![self.document_id.clone()],
                    op_index: None,
                }));
                return false;
            }
        };
        if let Some(folder) = self.folder.as_ref() {
            if folder.write_archive(&archive).await.is_ok() {
                self.last_written_hash = Some(document_archive_hash(&archive));
            }
        }
        self.known_op_ids = op_ids;
        self.server_frontier = Some(canonical_pair_baseline(baseline));
        self.current_pack = Some(pack);
        self.current_spr = Some(spr);
        self.current_archive = Some(archive);
        true
    }

    /// 🏃️ Advances exactly one command, readiness source, timer, backbone owner, or status turn.
    /// It idles only after observing an empty mailbox and an empty store outbound in the same turn:
    /// a command that relayed one backbone owner consumes that owner's wake, so a second queued
    /// owner has no wake left (ticket 26/09/23 `📓️wp-h6.md`).
    pub(super) async fn drive_one(&mut self) -> ArtifactDrive {
        if !self.started {
            self.started = true;
            self.setup().await;
            return ArtifactDrive::MoreWork;
        }
        if self.closing {
            self.operation_cancel.cancel_now();
            self.document_backbone_retention.clear();
            if let Some(mut pending) = self.artifact_bootstrap.take() {
                pending.assembler.abort();
            }
            if self.cmd_rx.close_one() {
                return ArtifactDrive::MoreWork;
            }
            match self.relay_one_backbone().await {
                Ok(true) | Err(_) => return ArtifactDrive::MoreWork,
                Ok(false) => {}
            }
            if !self.outbox.is_empty() && self.semio_hub.is_some() {
                let deadline = *self.close_drain_deadline.get_or_insert_with(|| Instant::now() + ARTIFACT_CLOSE_OUTBOX_DRAIN);
                if Instant::now() < deadline {
                    return match self.poll_hub_message() {
                        Some(message) => {
                            self.on_hub_message(message).await;
                            ArtifactDrive::MoreWork
                        }
                        None => ArtifactDrive::Idle { deadline: Some(deadline) },
                    };
                }
            }
            self.connect_future = None;
            return ArtifactDrive::Terminal;
        }
        if self.command_turns < 32 {
            if let Some(message) = self.cmd_rx.try_recv() {
                self.command_turns += 1;
                if self.handle_cmd(message).await {
                    self.closing = true;
                    self.cmd_rx.close();
                }
                return ArtifactDrive::MoreWork;
            }
        }
        self.command_turns = 0;
        if self.readiness_requested.swap(false, std::sync::atomic::Ordering::AcqRel) {
            if self.connect_future.is_some() {
                self.drive_phase = ArtifactDrivePhase::ConnectResult;
            } else if self.semio_hub.is_some() {
                self.drive_phase = ArtifactDrivePhase::Hub;
            }
        }
        let phase = self.drive_phase;
        self.drive_phase = match phase {
            ArtifactDrivePhase::Connect => ArtifactDrivePhase::Hub,
            ArtifactDrivePhase::Hub => ArtifactDrivePhase::ConnectResult,
            ArtifactDrivePhase::ConnectResult => ArtifactDrivePhase::Watch,
            ArtifactDrivePhase::Watch => ArtifactDrivePhase::Reconnect,
            ArtifactDrivePhase::Reconnect => ArtifactDrivePhase::Folder,
            ArtifactDrivePhase::Folder => ArtifactDrivePhase::Backbone,
            ArtifactDrivePhase::Backbone => ArtifactDrivePhase::Status,
            ArtifactDrivePhase::Status => ArtifactDrivePhase::Connect,
        };
        match phase {
            ArtifactDrivePhase::Connect => self.start_connect_hub().await,
            ArtifactDrivePhase::Hub => {
                if let Some(message) = self.poll_hub_message() {
                    self.drive_phase = ArtifactDrivePhase::Hub;
                    self.on_hub_message(message).await;
                }
            }
            ArtifactDrivePhase::ConnectResult => {
                let connection = self.readiness.clone().and_then(|readiness| {
                    let waker = std::task::Waker::from(Arc::new(ArtifactReadinessWake(readiness)));
                    let mut context = std::task::Context::from_waker(&waker);
                    match self.connect_future.as_mut().map(|future| future.as_mut().poll(&mut context)) {
                        Some(std::task::Poll::Ready(connection)) => Some(connection),
                        Some(std::task::Poll::Pending) | None => None,
                    }
                });
                if let Some(connection) = connection {
                    self.connect_future = None;
                    self.finish_connect_hub(connection).await;
                    if self.semio_hub.is_some() {
                        self.drive_phase = ArtifactDrivePhase::Hub;
                    }
                }
            }
            ArtifactDrivePhase::Watch => {
                if self.watcher.as_mut().is_some_and(|watcher| watcher.poll_changed()) {
                    self.fs_deadline = Some(Instant::now() + Duration::from_millis(200));
                }
            }
            ArtifactDrivePhase::Reconnect => {
                if self.reconnect_at.is_some_and(|deadline| deadline <= Instant::now()) {
                    self.reconnect_at = None;
                    self.start_connect_hub().await;
                }
            }
            ArtifactDrivePhase::Folder => {
                if self.fs_deadline.is_some_and(|deadline| deadline <= Instant::now()) {
                    self.fs_deadline = None;
                    self.handle_external_change().await;
                }
            }
            ArtifactDrivePhase::Backbone => {
                if !matches!(self.relay_one_backbone().await, Ok(false)) {
                    self.drive_phase = ArtifactDrivePhase::Backbone;
                }
            }
            ArtifactDrivePhase::Status => {
                self.tick_link().await;
                self.emit_status_if_changed().await;
                if self.cmd_rx.close_handle().has_pending() {
                    return ArtifactDrive::MoreWork;
                }
                if !matches!(self.relay_one_backbone().await, Ok(false)) {
                    self.drive_phase = ArtifactDrivePhase::Backbone;
                    return ArtifactDrive::MoreWork;
                }
                return ArtifactDrive::Idle { deadline: [self.reconnect_at, self.link_expires_at, self.fs_deadline].into_iter().flatten().min() };
            }
        }
        ArtifactDrive::MoreWork
    }

    /// 📡️ Polls one hub frame under the actor's readiness waker; `None` parks until the socket wakes it.
    fn poll_hub_message(&mut self) -> Option<Option<Result<Message, tokio_tungstenite::tungstenite::Error>>> {
        let readiness = self.readiness.clone()?;
        let waker = std::task::Waker::from(Arc::new(ArtifactReadinessWake(readiness)));
        let mut context = std::task::Context::from_waker(&waker);
        match self.semio_hub.as_mut().map(|connection| connection.read.poll_next_unpin(&mut context)) {
            Some(std::task::Poll::Ready(message)) => Some(message),
            Some(std::task::Poll::Pending) | None => None,
        }
    }

    /// 🌱️ Seeds persistence state from any already-stored recursive archive and installs the file watcher. A bound folder
    /// whose bootstrap read finds no archive is announced once ([`ArtifactEvent::DocumentArchiveAbsent`]).
    pub(super) async fn setup(&mut self) {
        let read = match self.folder.as_ref().filter(|_| self.current_pack.is_none()) {
            Some(folder) => Some(folder.read_archive().await),
            None => None,
        };
        if matches!(read, Some(Ok(None))) && !self.folder_bootstrapped {
            self.emit(ArtifactEvent::DocumentArchiveAbsent);
        }
        self.folder_bootstrapped |= matches!(read, Some(Ok(_)));
        let seeded = read.and_then(|read| read.ok().flatten());
        self.folder_held_hash = seeded.as_deref().map(document_archive_hash).or(self.folder_held_hash.take());
        if let Some(bytes) = seeded {
            if let Ok(archive) = semio_framework_os_spr::decode_document_archive_bytes(&bytes).await {
                let pack = archive.parent_pack;
                let spr = archive.parent_spr;
                if let Ok(op_ids) = spr_op_ids(&spr).await {
                self.known_op_ids = op_ids;
                self.last_written_hash = Some(document_archive_hash(&bytes));
                self.current_pack = Some(pack);
                self.current_spr = Some(spr);
                self.current_archive = Some(bytes);
                }
            }
        }
        if self.watch_external {
            if let Some(watch_path) = self.folder_watch_path.clone() {
                if let Some(readiness) = self.readiness.clone() {
                    self.watcher = Some(install_watcher(&watch_path, self.pool.clone(), readiness));
                }
            }
        }
    }

    /// 📨️ Handles a caller control message. Returns `true` when the actor should stop.
    pub async fn handle_cmd(&mut self, message: ArtifactActorMsg) -> bool {
        match message {
            ArtifactActorMsg::LocalMutations { envelopes } => {
                let drained = self.relay_one_backbone().await.unwrap_or(false);
                if !drained && !envelopes.is_empty() {
                    self.persist_operations(&envelopes).await;
                    self.relay_operations_to_hub(&envelopes).await;
                }
                false
            }
            ArtifactActorMsg::DocumentBackbone { message } => {
                let envelopes = match decode_document_backbone_message_exact(&message) {
                    Ok(envelopes) if envelopes.iter().all(|envelope| envelope.document_id.0 == self.document_id) => envelopes,
                    Ok(envelopes) => {
                        self.reject_document_backbone(CommandAckOutcome::local_rejected(CommandRejectionCode::LocalBackboneScopeMismatch, "document backbone scope mismatch", CommandRejectionDetail { envelopes: Some(envelopes.len() as u64), ..CommandRejectionDetail::default() }));
                        return false;
                    }
                    Err(_) => {
                        self.reject_document_backbone(CommandAckOutcome::local_rejected(CommandRejectionCode::LocalBackboneMalformed, "document backbone malformed", CommandRejectionDetail { bytes: Some(message.len() as u64), ..CommandRejectionDetail::default() }));
                        return false;
                    }
                };
                if let Err(refusal) = self.document_backbone_retention.retain(message.len(), &envelopes) {
                    self.reject_document_backbone(refusal);
                    return false;
                }
                self.persist_operations(&envelopes).await;
                if self.hub_space_id.is_some() {
                    self.relay_operations_to_hub(&envelopes).await;
                } else {
                    self.document_backbone_retention.release(&envelopes);
                }
                false
            }
            ArtifactActorMsg::LocalDocumentArchive { archive } => {
                if let Ok(decoded) = semio_framework_os_spr::decode_document_archive_bytes(&archive).await {
                    self.persist_archive(archive, decoded).await;
                }
                false
            }
            ArtifactActorMsg::PresenceHeartbeat { mut peer } => {
                stamp_session(&mut peer, self.session_color, self.hub_surface.as_deref()).await;
                self.send_client_frame(ClientFrame::Presence { peer: presence_to_bytes(&peer).await }, Lane::Preview).await;
                false
            }
            ArtifactActorMsg::PublishPreview { key, seq, payload } => {
                self.send_client_frame(ClientFrame::PreviewPublish { key, seq, payload }, Lane::Preview).await;
                false
            }
            ArtifactActorMsg::ExternalChanged => {
                self.handle_external_change().await;
                false
            }
            ArtifactActorMsg::Reseed { pack, spr, baseline } => {
                self.reseed_hub_document(pack, spr, baseline).await;
                false
            }
            ArtifactActorMsg::Detach => true,
        }
    }

    pub fn reject_document_backbone(&mut self, outcome: CommandAckOutcome) {
        let batch_id = self.next_local_rejection_batch_id;
        self.next_local_rejection_batch_id = self.next_local_rejection_batch_id.wrapping_sub(1);
        self.emit(ArtifactEvent::CommandOutcome { batch_id, outcome });
    }

    /// 📤️ Pops and advances exactly one store-to-actor FIFO owner.
    pub async fn relay_one_backbone(&mut self) -> Result<bool, semio_framework_os_vcs::VcsError> {
        let Some(message) = self.remote.try_pop_front()? else { return Ok(false) };
        match message {
            BackboneMessage::Mutations { envelopes } => {
                let envelopes = decode_envelopes(&envelopes).unwrap_or_default();
                self.persist_operations(&envelopes).await;
                self.relay_operations_to_hub(&envelopes).await;
            }
            BackboneMessage::Genesis { pack } => self.persist_genesis(pack).await,
            BackboneMessage::Ack { .. } => {}
            BackboneMessage::Member { .. } => return Err(semio_framework_os_vcs::VcsError::Backbone("a composed member requires its exact member transport lane".into())),
            BackboneMessage::Retract { .. } => return Err(semio_framework_os_vcs::VcsError::Backbone("a retraction flows from the actor to its store, never back".into())),
        }
        Ok(true)
    }

}
