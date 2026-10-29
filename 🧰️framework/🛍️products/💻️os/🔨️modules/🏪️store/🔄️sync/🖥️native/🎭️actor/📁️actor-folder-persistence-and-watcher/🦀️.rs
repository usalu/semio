//! 📁️ Actor folder persistence and watcher: the archive, genesis, operation, retraction and spr persistence, the external change handling and the folder endpoint and watcher builders. See [`super`].

use super::*;

impl ArtifactActor {
    //#region 🔖️Folder
    /// ✍️ Persists the current recursive archive to the folder binding and records the
    /// content hash for self-write suppression. A write failure (e.g. no `crate::os_store::serialization::ArtifactCodec`
    /// registered for this document's schema on the `Pack` endpoint — see `FolderEndpoint::write_archive`)
    /// is swallowed here the same way every other best-effort path in this actor already is, but
    /// deliberately does NOT record `last_written_hash` on failure — a false "persisted" mark
    /// would make `handle_external_change` mistake the still-stale on-disk content for a
    /// self-write and ignore a real external change.
    ///
    /// Bytes the folder is known to hold already — the last archive read or written — are not written again.
    async fn persist_write_archive(&mut self, archive: &[u8]) {
        let Some(folder) = self.folder.as_ref() else { return };
        let hash = document_archive_hash(archive);
        if self.folder_held_hash.as_deref() == Some(hash.as_str()) {
            return;
        }
        if folder.write_archive(archive).await.is_ok() {
            self.last_written_hash = Some(hash.clone());
            self.folder_held_hash = Some(hash);
            self.folder_bootstrapped = true;
            #[cfg(test)]
            {
                self.folder_writes += 1;
            }
        }
    }

    pub(super) async fn persist_archive(&mut self, bytes: Vec<u8>, archive: semio_framework_os_spr::DocumentArchivePack) {
        if let Ok(op_ids) = spr_op_ids(&archive.parent_spr).await {
            self.known_op_ids = op_ids;
        }
        self.persist_write_archive(&bytes).await;
        self.current_pack = Some(archive.parent_pack);
        self.current_spr = Some(archive.parent_spr);
        self.current_archive = Some(bytes);
    }

    /// 🧬️ Adopts the store's genesis: an empty binding persists it as the document's
    /// initial snapshot with an event-free history; a bound document must share it, otherwise
    /// the store names a different document and the mismatch is reported, never merged.
    pub(super) async fn persist_genesis(&mut self, pack: Vec<u8>) {
        if let Some(current) = self.current_pack.as_ref() {
            if *current != pack {
                self.emit(ArtifactEvent::Conflict(MutationMessage {
                    level: semio_framework_diagnostic::Severity::Error,
                    code: semio_framework_diagnostic::FaultCode::new("genesisMismatch"),
                    message: "the attached store names a different initial snapshot than the persisted document".into(),
                    target: vec![format!("folder://{}", self.document_id)],
                    op_index: None,
                }));
            }
            return;
        }
        let spr = semio_framework_os_store_text_io::text_format::empty_document_spr(&self.document_id, &self.schema).await;
        let archive = semio_framework_os_spr::DocumentArchivePack { parent_pack: pack, parent_spr: spr, members: Vec::new() };
        if let Ok(bytes) = semio_framework_os_spr::encode_document_archive_bytes(&archive) {
            self.persist_archive(bytes, archive).await;
        }
    }

    /// ➕️ Appends locally-applied operations to the persisted spr log (append-only),
    /// keeping the on-disk copy coherent so self-writes are never mistaken for external edits.
    pub(super) async fn persist_operations(&mut self, envelopes: &[MutationEnvelope]) {
        if self.folder.is_none() {
            return;
        }
        let (Some(pack), Some(spr)) = (self.current_pack.clone(), self.current_spr.clone()) else { return };
        let mut new_edits: Vec<semio_framework_os_spr::HistoryEdit> = Vec::new();
        let mut new_transitions: Vec<semio_framework_os_spr::HistoryTransitionRecord> = Vec::new();
        for envelope in envelopes.iter().filter(|envelope| self.known_op_ids.insert(envelope.mutation_id.0.clone())) {
            if semio_framework_os_spr::is_history_transition(envelope) {
                new_transitions.push(semio_framework_os_spr::HistoryTransitionRecord::from_envelope(envelope));
            } else {
                new_edits.push(history_edit_from_envelope(envelope).await);
            }
        }
        if new_edits.is_empty() && new_transitions.is_empty() {
            return;
        }
        let Ok(new_spr) = semio_framework_os_store_text_io::text_format::append_history_events_to_spr(&spr, &new_edits, &new_transitions).await else { return };
        self.persist_spr(pack, new_spr).await;
    }

    /// 🪒️ Drops from the persisted log the transitions `refused` retracts — the same ones the store drops
    /// ([`crate::os_store::transition_retraction_closure`]) — so a reload never resurrects what the hub refused.
    pub(super) async fn persist_retraction(&mut self, refused: &[String]) {
        if self.folder.is_none() {
            return;
        }
        let (Some(pack), Some(spr)) = (self.current_pack.clone(), self.current_spr.clone()) else { return };
        let Ok((spr, retracted)) = semio_framework_os_store_engine_checkpoint_graph::retract_history_transitions_from_spr(&spr, refused).await else { return };
        for id in &retracted {
            self.known_op_ids.remove(id);
        }
        self.persist_spr(pack, spr).await;
    }

    /// 💾️ Persists `spr` as the recursive archive's parent log beside `pack`.
    async fn persist_spr(&mut self, pack: Vec<u8>, spr: Vec<u8>) {
        let mut archive = match self.current_archive.as_deref() {
            Some(bytes) => semio_framework_os_spr::decode_document_archive_bytes(bytes).await.ok(),
            None => None,
        }
        .unwrap_or(semio_framework_os_spr::DocumentArchivePack { parent_pack: pack.clone(), parent_spr: spr.clone(), members: Vec::new() });
        archive.parent_spr = spr.clone();
        if let Ok(bytes) = semio_framework_os_spr::encode_document_archive_bytes(&archive) {
            self.persist_write_archive(&bytes).await;
            self.current_archive = Some(bytes);
        }
        self.current_pack = Some(pack);
        self.current_spr = Some(spr);
    }

    /// 👁️ Re-reads the folder binding and classifies the change: append-only → `RemoteMutations`,
    /// divergence → `DocumentArchiveReplaced`, divergence with local pending operations → `Conflict`. Self-writes
    /// (content hash match) are ignored.
    ///
    /// Append-only spr events decoded without HistoryOpMeta carry HLT(0,0). Ingest merges
    /// by HLC, so a zero stamp would reorder under already-applied local edits and leave
    /// the live snapshot at the older head. Stamp each new envelope with this actor clock.
    ///
    /// The first answered read of an open that finds no archive, with none read or written before, announces
    /// [`ArtifactEvent::DocumentArchiveAbsent`] once; a later empty answer only forgets the archive the folder held.
    pub(super) async fn handle_external_change(&mut self) {
        let read = match self.folder.as_ref() {
            Some(folder) => folder.read_archive().await,
            None => return,
        };
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
        let Ok(archive) = semio_framework_os_spr::decode_document_archive_bytes(&bytes).await else { return };
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
                if envelope.timestamp == semio_framework_os_spr::HybridLogicalTimestamp::new(0, 0) {
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
            if !self.pending_batches.is_empty() {
                self.emit(ArtifactEvent::Conflict(MutationMessage {
                    level: semio_framework_diagnostic::Severity::Error,
                    code: semio_framework_diagnostic::FaultCode::new("externalDivergence"),
                    message: "external history diverged while local operations are pending".into(),
                    target: vec![format!("folder://{}", self.document_id)],
                    op_index: None,
                }));
            } else {
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

/// 🔀️ A binding path with a file extension addresses one document's text blob directly
/// (`Text`, generalizing the deleted single-file `FileJsonStorage` beyond `.json`); an extensionless
/// directory path is the canonical multi-document append-only store (`EventLog`).
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
/// itself, or the multi-document sqlite db under `<folder>/.semio/documents.db`.
pub(super) async fn folder_watch_path_for(path: &Path) -> PathBuf {
    if path.extension().is_some() {
        path.to_path_buf()
    } else {
        path.join(".semio").join("documents.db")
    }
}

/// 👁️ Creates the owned non-recursive watcher; its first probe establishes a
/// baseline and later snapshots feed the actor's existing 200 ms debounce.
pub(super) fn install_watcher(watch_path: &Path, pool: Arc<semio_framework_async::WorkerPool>, readiness: Arc<dyn Fn() + Send + Sync>) -> semio_framework_os_services::OwnedFileChangeWatcher {
    semio_framework_os_services::OwnedFileChangeWatcher::new(watch_path, pool, readiness)
}
