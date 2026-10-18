//! 🏠️ Wasm artifact host: the artifact channels, the host handle with its clone, operations and drop. See [`super`].

use super::*;

/// 🎛️ The channels of a browser document actor, which has no runner.
pub type ArtifactChannels = ArtifactChannelsOf<()>;

/// 🏛️ Registry of open per-document actors of the browser wgpu build: one `ArtifactHost` per page
/// owns every open document's actor + event fan-out, and the page's document-socket dialer.
pub struct ArtifactHost {
    pub core: ArtifactHostCore<()>,
    document_socket_dialer: SharedDialer,
}

impl Clone for ArtifactHost {
    fn clone(&self) -> Self {
        Self { core: self.core.retain(), document_socket_dialer: self.document_socket_dialer.clone() }
    }
}

impl ArtifactHost {
    /// 🧩️ Creates a host on the process WorkerPool; callers must inject the same pool
    /// used by their service and renderer runtimes.
    pub fn new(pool: Arc<semio_framework_async::WorkerPool>) -> Self {
        Self { core: ArtifactHostCore::new(pool), document_socket_dialer: Arc::new(RwLock::new(None)) }
    }

    /// 🏛️ The platform-neutral registry this host composes with the browser actor.
    pub fn core(&self) -> &ArtifactHostCore<()> {
        &self.core
    }

    pub fn set_local_hub_credential(&self, credential: Arc<semio_framework_os_directory_client::LocalHubCredential>) {
        self.core.set_local_hub_credential(credential);
    }

    pub fn set_hub_socket_grant_source(&self, source: Arc<dyn semio_framework_os_directory_client::HubSocketGrantSource>) {
        self.core.set_hub_socket_grant_source(source);
    }

    pub fn set_document_execution_target_lease(&self, document_key: &ArtifactDocumentKey, surface: semio_framework_os_directory::DocumentExecutionTargetLeaseFieldsV1) -> bool {
        self.core.set_document_execution_target_lease(document_key, surface)
    }

    pub fn set_document_seed(&self, document_key: &ArtifactDocumentKey, pair: semio_framework_os_directory::CanonicalCheckpointPairV1) -> bool {
        self.core.set_document_seed(document_key, pair)
    }

    /// ☎️ Installs the browser's document-socket dialer; every later hub document actor dials through it.
    pub fn set_document_socket_dialer(&self, dialer: Arc<dyn DocumentSocketDialer>) {
        *self.document_socket_dialer.write().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(dialer);
    }

    /// 🟢️ Whether a hub document opened now can dial at once: a credential, a grant source and
    /// the socket dialer the page owns.
    pub fn local_hub_ready(&self) -> bool {
        self.core.hub_binding_present() && self.document_socket_dialer.read().unwrap_or_else(std::sync::PoisonError::into_inner).is_some()
    }

    /// 🚀️ Spawns (or replaces) the actor for `config.document_id` and returns the channels the
    /// caller wires into its store. Idempotent per id: opening an already-open id closes the old actor.
    pub async fn open(&self, config: ArtifactActorConfig) -> ArtifactChannels {
        let document_id = config.document_id.clone();
        let document_key = ArtifactDocumentKey::for_config(&config);
        let (document_execution_target_lease, seed) = self.core.take_open_inputs(&document_key);
        let _ = self.close_key(&document_key);
        let generation = self.core.state().claim_generation();
        let (channel_backbone, remote) = ChannelBackbone::pair(&format!("actor://{document_id}")).await;
        let (cmd_tx, cmd_rx) = artifact_mailbox_pair();
        let (event_tx, _event_rx) = broadcast::channel(256);
        let document_cancel = self.core.cancel.child_now();
        spawn_actor(
            config,
            remote,
            cmd_rx,
            event_tx.clone(),
            WasmActorHub { credential: self.core.credential.clone(), socket_grant_source: self.core.socket_grant_source.clone(), dialer: self.document_socket_dialer.clone(), lease: document_execution_target_lease, seed, cancel: document_cancel.clone() },
        )
        .await;
        let entry = OpenDocument { generation, cancel: document_cancel, cmd_tx: cmd_tx.clone(), events: event_tx, presence: PresenceHeartbeatProducer::default(), runner: () };
        self.core.insert_document(document_key.clone(), entry);
        ArtifactChannelsOf { cmd_tx, document_key, runner: (), channel_backbone }
    }

    pub async fn subscribe(&self, document_id: &str) -> broadcast::Receiver<ArtifactEvent> {
        self.core.subscribe(document_id).await
    }

    pub async fn subscribe_key(&self, document_key: &ArtifactDocumentKey) -> broadcast::Receiver<ArtifactEvent> {
        self.core.subscribe_key(document_key).await
    }

    pub async fn send(&self, document_id: &str, message: ArtifactActorMsg) {
        self.core.send(document_id, message).await;
    }

    pub async fn send_key(&self, document_key: &ArtifactDocumentKey, message: ArtifactActorMsg) {
        self.core.send_key(document_key, message).await;
    }

    pub fn presence_heartbeat(&self, document_id: &str, now_ms: u64, peer: PresencePeer) -> bool {
        self.core.presence_heartbeat(document_id, now_ms, peer)
    }

    pub fn observe_presence_tool_run_key(&self, document_key: &ArtifactDocumentKey, tool_run: Option<PresenceToolRun>) -> bool {
        self.core.observe_presence_tool_run_key(document_key, tool_run)
    }

    pub fn presence_heartbeat_key(&self, document_key: &ArtifactDocumentKey, now_ms: u64, peer: PresencePeer) -> bool {
        self.core.presence_heartbeat_key(document_key, now_ms, peer)
    }

    /// ✂️ Detaches a document's actor and drops its registry entry.
    /// 🚫️async: E1 pure lock/remove + sync channel send, no real suspension point — consumed by
    /// `impl Drop` (sync-only external trait); see R9.
    pub fn close(&self, document_id: &str) -> Option<u64> {
        self.close_key(&ArtifactDocumentKey::local(document_id))
    }

    pub fn close_key(&self, document_key: &ArtifactDocumentKey) -> Option<u64> {
        let document = self.core.remove_document(document_key)?;
        document.cancel.cancel_now();
        let _ = document.cmd_tx.send(ArtifactActorMsg::Detach);
        Some(document.generation)
    }

    pub fn open_artifacts(&self) -> Vec<ArtifactDocumentKey> {
        self.core.open_artifacts()
    }
}

impl Drop for ArtifactHost {
    fn drop(&mut self) {
        if !self.core.release() {
            return;
        }
        self.core.cancel.cancel_now();
        for document_key in self.core.open_artifacts() {
            self.close_key(&document_key);
        }
    }
}
