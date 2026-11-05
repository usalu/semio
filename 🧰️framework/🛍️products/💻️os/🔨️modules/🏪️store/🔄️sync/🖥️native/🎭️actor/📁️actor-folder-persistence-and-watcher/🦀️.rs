//! 📁️ Actor folder persistence and watcher: the archive, genesis, operation, retraction and spr persistence, the external change handling and the folder endpoint and watcher builders. See [`super`].

use super::*;

/// 🔎️ What a re-read folder holds relative to the archive this actor last read or wrote.
pub(super) enum FolderProbe {
    Absent,
    Held,
    Foreign,
    Unreadable(String),
}

impl ArtifactActor {
    //#region 🔖️Folder
    /// ✍️ Persists one archive to the folder binding and reports whether the folder acknowledged it. Only an See [design notes](README.md).
    async fn persist_write_archive(&mut self, archive: &[u8]) -> bool {
        if self.folder.is_none() {
            return true;
        }
        let hash = document_archive_hash(archive);
        if self.folder_unread || self.folder_diverged {
            self.folder_write_failures = self.folder_write_failures.max(1);
            self.emit_status_if_changed().await;
            return false;
        }
        if self.folder_held_hash.as_deref() == Some(hash.as_str()) {
            return true;
        }
        match self.probe_folder().await {
            FolderProbe::Unreadable(message) => { self.block_folder("folderReadFailed", message).await; return false; }
            FolderProbe::Foreign => { self.diverge_folder().await; return false; }
            FolderProbe::Held | FolderProbe::Absent => {}
        }
        let Some(folder) = self.folder.as_ref() else { return true };
        let written = folder.write_archive(archive).await.is_ok();
        if written {
            self.last_written_hash = Some(hash.clone());
            self.folder_held_hash = Some(hash);
            self.folder_bootstrapped = true;
            self.folder_write_failures = 0;
            self.folder_retry_at = None;
            #[cfg(test)]
            {
                self.folder_writes += 1;
            }
        } else {
            self.folder_write_failures = self.folder_write_failures.saturating_add(1);
            self.schedule_folder_retry();
        }
        self.emit_status_if_changed().await;
        written
    }

    fn folder_conflict(&self, level: semio_framework_diagnostic::Severity, code: &str, message: impl Into<String>) -> ArtifactEvent {
        ArtifactEvent::Conflict(MutationMessage { level, code: semio_framework_diagnostic::FaultCode::new(code), message: message.into(), target: vec![format!("folder://{}", self.document_id)], op_index: None })
    }

    pub(super) fn schedule_folder_retry(&mut self) {
        self.folder_retry_at = Some(Instant::now() + Duration::from_millis(200u64 << self.folder_write_failures.min(5)));
    }

    /// 🚧️ Blocks every write to a folder that cannot be trusted (unreadable, or an archive that does not decode) until a later read answers; the conflict is reported once per episode.
    pub(super) async fn block_folder(&mut self, code: &'static str, message: String) {
        let first = self.folder_write_failures == 0;
        self.folder_unread = true;
        self.folder_write_failures = self.folder_write_failures.saturating_add(1);
        self.schedule_folder_retry();
        if first {
            self.emit(ArtifactEvent::Conflict(MutationMessage { level: semio_framework_diagnostic::Severity::Error, code: semio_framework_diagnostic::FaultCode::new(code), message, target: vec![format!("folder://{}", self.document_id)], op_index: None }));
        }
        self.emit_status_if_changed().await;
    }

    /// 🔎️ Re-reads the folder and classifies it against the archive this actor last read or wrote.
    pub(super) async fn probe_folder(&mut self) -> FolderProbe {
        let Some(folder) = self.folder.as_ref() else { return FolderProbe::Absent };
        match folder.read_archive().await {
            Ok(None) => FolderProbe::Absent,
            Ok(Some(bytes)) => match semio_os_spr::decode_document_archive_bytes(&bytes).await {
                Err(error) => FolderProbe::Unreadable(format!("the folder archive does not decode: {error}")),
                Ok(_) if self.folder_held_hash.as_deref() == Some(document_archive_hash(&bytes).as_str()) => FolderProbe::Held,
                Ok(_) => FolderProbe::Foreign,
            },
            Err(error) => FolderProbe::Unreadable(error),
        }
    }

    pub(super) async fn diverge_folder(&mut self) {
        let first = !self.folder_diverged;
        self.folder_diverged = true;
        self.folder_write_failures = self.folder_write_failures.max(1);
        self.folder_retry_at = None;
        if first {
            self.emit(self.folder_conflict(semio_framework_diagnostic::Severity::Error, "externalDivergence", "the folder changed while local edits are not yet saved"));
        }
        self.emit_status_if_changed().await;
    }

    /// 💾️ Writes one archive and adopts it as the acknowledged state only when the folder took it; otherwise it is held as the unsaved newest archive for the retry and later edits build on it.
    pub(super) async fn commit_archive(&mut self, bytes: Vec<u8>, pack: Vec<u8>, spr: Vec<u8>) {
        if self.persist_write_archive(&bytes).await {
            self.unsaved = None;
            self.current_pack = Some(pack);
            self.current_spr = Some(spr);
            self.current_archive = Some(bytes);
        } else {
            self.unsaved = Some(bytes);
        }
    }

    /// 🧱️ The pack and spr new edits append to: the unsaved newest archive when one is held, else the acknowledged state.
    async fn working_pack_spr(&mut self) -> Option<(Vec<u8>, Vec<u8>)> {
        match self.unsaved.as_deref() {
            Some(bytes) => semio_os_spr::decode_document_archive_bytes(bytes).await.ok().map(|archive| (archive.parent_pack, archive.parent_spr)),
            None => Some((self.current_pack.clone()?, self.current_spr.clone()?)),
        }
    }

    /// 🔁️ Retries once due. A folder that was unreadable is re-read first. An unsaved archive is written only after the See [design notes](README.md).
    pub(super) async fn retry_folder_write(&mut self) {
        if !self.folder_retry_at.is_some_and(|due| due <= Instant::now()) {
            return;
        }
        self.folder_retry_at = None;
        if self.folder_unread {
            self.folder_unread = false;
            self.setup().await;
            if !self.folder_unread {
                if self.unsaved.is_some() && self.current_pack.is_some() {
                    self.diverge_folder().await;
                } else if self.unsaved.is_some() {
                    self.folder_retry_at = Some(Instant::now());
                } else {
                    self.folder_write_failures = 0;
                }
            }
            self.emit_status_if_changed().await;
            return;
        }
        let Some(bytes) = self.unsaved.clone() else { self.folder_write_failures = 0; return self.emit_status_if_changed().await; };
        match self.probe_folder().await {
            FolderProbe::Unreadable(message) => self.block_folder("folderReadFailed", message).await,
            FolderProbe::Foreign => self.diverge_folder().await,
            _ => {
                if let Ok(archive) = semio_os_spr::decode_document_archive_bytes(&bytes).await {
                    self.commit_archive(bytes, archive.parent_pack, archive.parent_spr).await;
                }
            }
        }
    }

    pub(super) async fn persist_archive(&mut self, bytes: Vec<u8>, archive: semio_os_spr::DocumentArchivePack) {
        if let Ok(op_ids) = spr_op_ids(&archive.parent_spr).await {
            self.known_op_ids = op_ids;
        }
        self.commit_archive(bytes, archive.parent_pack, archive.parent_spr).await;
    }

    /// 🧬️ Adopts the store's genesis: an empty binding persists it as the document's See [design notes](README.md).
    pub(super) async fn persist_genesis(&mut self, pack: Vec<u8>) {
        if let Some(current) = self.current_pack.as_ref() {
            if *current != pack {
                self.emit(self.folder_conflict(semio_framework_diagnostic::Severity::Error, "genesisMismatch", "the attached store names a different initial snapshot than the persisted document"));
            }
            return;
        }
        let spr = semio_os_store_text_io::text_format::empty_document_spr(&self.document_id, &self.schema).await;
        let archive = semio_os_spr::DocumentArchivePack { parent_pack: pack, parent_spr: spr, members: Vec::new() };
        if let Ok(bytes) = semio_os_spr::encode_document_archive_bytes(&archive) {
            self.persist_archive(bytes, archive).await;
        }
    }

    /// ➕️ Appends locally-applied operations to the persisted spr log (append-only),
    /// keeping the on-disk copy coherent so self-writes are never mistaken for external edits.
    pub(super) async fn persist_operations(&mut self, envelopes: &[MutationEnvelope]) {
        if self.folder.is_none() {
            return;
        }
        let Some((pack, spr)) = self.working_pack_spr().await else { return };
        let mut new_edits: Vec<semio_os_spr::HistoryEdit> = Vec::new();
        let mut new_transitions: Vec<semio_os_spr::HistoryTransitionRecord> = Vec::new();
        let mut admitted: Vec<String> = Vec::new();
        for envelope in envelopes.iter().filter(|envelope| self.known_op_ids.insert(envelope.mutation_id.0.clone())) {
            admitted.push(envelope.mutation_id.0.clone());
            if semio_os_spr::is_history_transition(envelope) {
                new_transitions.push(semio_os_spr::HistoryTransitionRecord::from_envelope(envelope));
            } else {
                new_edits.push(history_edit_from_envelope(envelope).await);
            }
        }
        if new_edits.is_empty() && new_transitions.is_empty() {
            return;
        }
        let Ok(new_spr) = semio_os_store_text_io::text_format::append_history_events_to_spr(&spr, &new_edits, &new_transitions).await else {
            for id in &admitted {
                self.known_op_ids.remove(id);
            }
            return;
        };
        self.persist_spr(pack, new_spr).await;
    }

    /// 🪒️ Drops from the persisted log the transitions `refused` retracts — the same ones the store drops
    /// ([`crate::os_store::transition_retraction_closure`]) — so a reload never resurrects what the hub refused.
    pub(super) async fn persist_retraction(&mut self, refused: &[String]) {
        if self.folder.is_none() {
            return;
        }
        let Some((pack, spr)) = self.working_pack_spr().await else { return };
        let Ok((spr, retracted)) = semio_os_store_engine_checkpoint_graph::retract_history_transitions_from_spr(&spr, refused).await else { return };
        for id in &retracted {
            self.known_op_ids.remove(id);
        }
        self.persist_spr(pack, spr).await;
    }

    /// 💾️ Persists `spr` as the recursive archive's parent log beside `pack`.
    async fn persist_spr(&mut self, pack: Vec<u8>, spr: Vec<u8>) {
        let base = self.unsaved.as_deref().or(self.current_archive.as_deref());
        let mut archive = match base {
            Some(bytes) => semio_os_spr::decode_document_archive_bytes(bytes).await.ok(),
            None => None,
        }
        .unwrap_or(semio_os_spr::DocumentArchivePack { parent_pack: pack.clone(), parent_spr: spr.clone(), members: Vec::new() });
        archive.parent_spr = spr.clone();
        if let Ok(bytes) = semio_os_spr::encode_document_archive_bytes(&archive) {
            self.commit_archive(bytes, pack, spr).await;
        }
    }

    /// 👁️ Re-reads the folder binding and classifies the change: append-only → `RemoteMutations`, See [design notes](README.md).
    pub(super) async fn handle_external_change(&mut self) {
        let read = match self.folder.as_ref() {
            Some(folder) => folder.read_archive().await,
            None => return,
        };
        if self.unsaved.is_some() {
            match self.probe_folder().await {
                FolderProbe::Foreign => self.diverge_folder().await,
                FolderProbe::Unreadable(message) => self.block_folder("folderReadFailed", message).await,
                FolderProbe::Held | FolderProbe::Absent => if std::mem::take(&mut self.folder_diverged) { self.folder_retry_at = Some(Instant::now()); },
            }
            return;
        }
        let bytes = match read {
            Ok(Some(bytes)) => bytes,
            Ok(None) => {
                if !self.folder_bootstrapped {
                    self.emit(ArtifactEvent::DocumentArchiveAbsent);
                }
                self.folder_bootstrapped = true;
                self.folder_held_hash = None;
                return;
            }
            Err(_) => return,
        };
        self.folder_bootstrapped = true;
        let archive = match semio_os_spr::decode_document_archive_bytes(&bytes).await {
            Ok(archive) => archive,
            Err(error) => return self.block_folder("folderArchiveCorrupt", format!("the folder archive does not decode: {error}")).await,
        };
        let pack = archive.parent_pack.clone();
        let spr = archive.parent_spr.clone();
        let hash = document_archive_hash(&bytes);
        self.folder_held_hash = Some(hash.clone());
        if self.last_written_hash.as_deref() == Some(hash.as_str()) {
            return;
        }
        let Ok(file_ids) = spr_op_ids(&spr).await else { return };
        let lost: Vec<String> = self.known_op_ids.difference(&file_ids).cloned().collect();
        let new_ids: HashSet<String> = file_ids.difference(&self.known_op_ids).cloned().collect();

        if lost.is_empty() && !new_ids.is_empty() {
            let Ok(events) = spr_events(&spr, &self.document_id, &self.schema,self.actor_identity_grant).await else { return };
            let mut appended: Vec<MutationEnvelope> = events.into_iter().filter(|event| new_ids.contains(&event.mutation_id.0)).collect();
            for envelope in &mut appended {
                if envelope.timestamp == semio_os_spr::HybridLogicalTimestamp::new(0, 0) {
                    envelope.timestamp = next_timestamp(self.hlc_seed, &mut self.hlc_counter).await;
                }
            }
            self.known_op_ids.extend(new_ids);
            self.current_pack = Some(pack);
            self.current_spr = Some(spr);
            self.current_archive = Some(bytes);
            self.last_written_hash = Some(hash);
            let _ = self.deliver_remote_operations(appended).await;
        } else if !lost.is_empty() {
            if !self.pending_batches.is_empty() || !self.outbox.is_empty() {
                self.emit(self.folder_conflict(semio_framework_diagnostic::Severity::Error, "externalDivergence", "external history diverged while local operations are pending"));
            } else {
                self.emit(self.folder_conflict(semio_framework_diagnostic::Severity::Warning, "externalHistoryReplaced", "the folder no longer holds operations this document had acknowledged; the document is reloaded from the folder"));
                self.known_op_ids = file_ids;
                self.current_pack = Some(pack.clone());
                self.current_spr = Some(spr.clone());
                self.current_archive = Some(bytes.clone());
                self.last_written_hash = Some(hash);
                self.deliver_archive(bytes).await;
            }
        }
    }
    //#endregion 🔖️Folder

}

/// 🔀️ A binding path with a file extension addresses one document's text blob directly See [design notes](README.md).
pub(super) async fn build_folder_endpoint(path: &Path, document_id: &str, schema: &str) -> FolderEndpoint {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some(extension) => {
            let folder = path.parent().map(|parent| parent.to_path_buf()).unwrap_or_else(|| PathBuf::from("."));
            FolderEndpoint::Pack { storage: FolderTextStorage::new(folder).await, document_id: document_id.to_string(), extension: extension.to_string(), schema: schema.to_string() }
        }
        None => FolderEndpoint::EventLog { storage: FolderEventLogStorage::new(path.to_path_buf()), document_id: document_id.to_string(), schema: schema.to_string() },
    }
}

/// 📍️ The on-disk path a folder binding writes to: the `<document_id>.<extension>` text blob
/// itself, or the folder's append-only event log, the single canonical `<folder>/.semio/events.semio`.
pub(super) async fn folder_watch_path_for(path: &Path) -> PathBuf {
    if path.extension().is_some() {
        path.to_path_buf()
    } else {
        FolderEventLogStorage::new(path.to_path_buf()).event_path()
    }
}

/// 👁️ Creates the owned non-recursive watcher; its first probe establishes a
/// baseline and later snapshots feed the actor's existing 200 ms debounce.
pub(super) fn install_watcher(watch_path: &Path, pool: Arc<semio_framework_async::WorkerPool>, readiness: Arc<dyn Fn() + Send + Sync>) -> semio_os_services::OwnedFileChangeWatcher {
    semio_os_services::OwnedFileChangeWatcher::new(watch_path, pool, readiness)
}
