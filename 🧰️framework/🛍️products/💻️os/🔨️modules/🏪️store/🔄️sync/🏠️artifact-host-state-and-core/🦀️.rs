//! 🏠️ Artifact host state and core: the channels, the open document, the host state and the host core with its runner operations. See [`super`].

use super::*;

//#region 🔖️Host
/// 🎛️ The channels {@link ArtifactHost::open} hands back to a caller: attach `channel_backbone`
/// to your `ArtifactStore`, and send control messages (or wakes) on `cmd_tx`. `Runner` is the
/// platform's per-document runner ticket (`()` in the browser, which has no runner).
pub struct ArtifactChannelsOf<Runner> {
    pub cmd_tx: ArtifactMailboxSender,
    pub document_key: ArtifactDocumentKey,
    pub runner: Runner,
    /// 🔗️ The store-side backbone end. The caller owns store attachment:
    /// `store.attach_backbone(Backbones::Channel(channels.channel_backbone))`.
    pub channel_backbone: ChannelBackbone,
}

pub struct OpenDocument<Runner> {
    pub generation: u64,
    pub cancel: semio_framework_async::CancelToken,
    pub cmd_tx: ArtifactMailboxSender,
    pub events: broadcast::Sender<ArtifactEvent>,
    pub presence: PresenceHeartbeatProducer,
    pub runner: Runner,
}

pub struct ArtifactHostState<Runner> {
    pub documents: std::collections::HashMap<ArtifactDocumentKey, OpenDocument<Runner>>,
    pub document_execution_target_leases: std::collections::HashMap<ArtifactDocumentKey, semio_framework_os_directory::DocumentExecutionTargetLeaseFieldsV1>,
    pub document_seeds: std::collections::HashMap<ArtifactDocumentKey, semio_framework_os_directory::CanonicalCheckpointPairV1>,
    pub closing: std::collections::HashMap<u64, Runner>,
    pub next_generation: u64,
    pub host_references: usize,
}

impl<Runner> ArtifactHostState<Runner> {
    pub fn new() -> Self {
        Self {
            documents: std::collections::HashMap::new(),
            document_execution_target_leases: std::collections::HashMap::new(),
            document_seeds: std::collections::HashMap::new(),
            closing: std::collections::HashMap::new(),
            next_generation: 1,
            host_references: 1,
        }
    }

    pub fn claim_generation(&mut self) -> u64 {
        let generation = self.next_generation;
        self.next_generation = self.next_generation.checked_add(1).expect("artifact actor generation exhausted");
        generation
    }
}

/// 🏛️ The platform-neutral half of `ArtifactHost`: the registry of open per-document actors, their
/// event fan-out, presence cadence and the late-bound hub binding. Each platform crate owns the
/// `ArtifactHost` that composes this core with the actor it spawns, so this core never names an actor.
pub struct ArtifactHostCore<Runner> {
    pub inner: std::sync::Arc<std::sync::Mutex<ArtifactHostState<Runner>>>,
    pub pool: std::sync::Arc<semio_framework_async::WorkerPool>,
    pub credential: std::sync::Arc<std::sync::RwLock<Option<std::sync::Arc<semio_framework_os_directory_client::LocalHubCredential>>>>,
    pub socket_grant_source: std::sync::Arc<std::sync::RwLock<Option<std::sync::Arc<dyn semio_framework_os_directory_client::HubSocketGrantSource>>>>,
    pub cancel: semio_framework_async::CancelToken,
}

impl<Runner> ArtifactHostCore<Runner> {
    /// 🧩️ Creates a core on the process WorkerPool; callers must inject the same pool
    /// used by their service and renderer runtimes.
    pub fn new(pool: std::sync::Arc<semio_framework_async::WorkerPool>) -> Self {
        Self {
            inner: std::sync::Arc::new(std::sync::Mutex::new(ArtifactHostState::new())),
            pool,
            credential: std::sync::Arc::new(std::sync::RwLock::new(None)),
            socket_grant_source: std::sync::Arc::new(std::sync::RwLock::new(None)),
            cancel: semio_framework_async::CancelToken::root_now(),
        }
    }

    pub fn state(&self) -> std::sync::MutexGuard<'_, ArtifactHostState<Runner>> {
        self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// 🔗️ Counts one more owner of the shared registry and returns its handle.
    pub fn retain(&self) -> Self {
        let mut state = self.state();
        state.host_references = state.host_references.checked_add(1).expect("artifact host reference capacity exhausted");
        drop(state);
        Self { inner: self.inner.clone(), pool: self.pool.clone(), credential: self.credential.clone(), socket_grant_source: self.socket_grant_source.clone(), cancel: self.cancel.clone() }
    }

    /// 🔓️ Releases one owner; `true` when it was the last.
    pub fn release(&self) -> bool {
        let mut state = self.state();
        state.host_references = state.host_references.checked_sub(1).expect("artifact host reference underflow");
        state.host_references == 0
    }

    pub fn set_local_hub_credential(&self, credential: std::sync::Arc<semio_framework_os_directory_client::LocalHubCredential>) {
        *self.credential.write().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(credential);
    }

    pub fn set_hub_socket_grant_source(&self, source: std::sync::Arc<dyn semio_framework_os_directory_client::HubSocketGrantSource>) {
        *self.socket_grant_source.write().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(source);
    }

    /// 🪪️ Binds one complete verified execution-target lease before this document actor is opened.
    pub fn set_document_execution_target_lease(&self, document_key: &ArtifactDocumentKey, surface: semio_framework_os_directory::DocumentExecutionTargetLeaseFieldsV1) -> bool {
        if !matches!(document_key, ArtifactDocumentKey::Hub { .. }) {
            return false;
        }
        let mut state = self.state();
        if state.documents.contains_key(document_key) || state.document_execution_target_leases.contains_key(document_key) {
            return false;
        }
        state.document_execution_target_leases.insert(document_key.clone(), surface);
        true
    }

    /// 🪢️ Hands the next actor of a hub document the canonical checkpoint pair its shell verified and loaded into the
    /// guest ([`crate::os_directory::DirectoryClient::document_canonical_checkpoint_pair`]): the actor starts at that
    /// pair — its history known, its first `SocketHelloV1` naming the pair's baseline frontier, so the hub tails only
    /// what came after it. Consumed by the next open of the same key.
    pub fn set_document_seed(&self, document_key: &ArtifactDocumentKey, pair: semio_framework_os_directory::CanonicalCheckpointPairV1) -> bool {
        let ArtifactDocumentKey::Hub { space_id, document_id } = document_key else {
            return false;
        };
        if pair.scope.space_id != *space_id || pair.scope.document_id != *document_id {
            return false;
        }
        let mut state = self.state();
        if state.documents.contains_key(document_key) || state.document_seeds.contains_key(document_key) {
            return false;
        }
        state.document_seeds.insert(document_key.clone(), pair);
        true
    }

    /// 🟢️ Whether a credential and a grant source are bound; a browser adds its socket dialer on top.
    pub fn hub_binding_present(&self) -> bool {
        self.credential.read().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() && self.socket_grant_source.read().unwrap_or_else(std::sync::PoisonError::into_inner).is_some()
    }

    /// 🎫️ Takes the lease and the seed bound for the next open of `document_key`.
    pub fn take_open_inputs(&self, document_key: &ArtifactDocumentKey) -> (Option<semio_framework_os_directory::DocumentExecutionTargetLeaseFieldsV1>, Option<semio_framework_os_directory::CanonicalCheckpointPairV1>) {
        let mut state = self.state();
        (state.document_execution_target_leases.remove(document_key), state.document_seeds.remove(document_key))
    }

    pub fn insert_document(&self, document_key: ArtifactDocumentKey, document: OpenDocument<Runner>) {
        self.state().documents.insert(document_key, document);
    }

    pub fn remove_document(&self, document_key: &ArtifactDocumentKey) -> Option<OpenDocument<Runner>> {
        self.state().documents.remove(document_key)
    }

    /// 📬️ A fresh event receiver for `document_id`. If the document is not open the receiver's
    /// sender is dropped, so it simply reports closed.
    pub async fn subscribe(&self, document_id: &str) -> broadcast::Receiver<ArtifactEvent> {
        self.subscribe_key(&ArtifactDocumentKey::local(document_id)).await
    }

    pub async fn subscribe_key(&self, document_key: &ArtifactDocumentKey) -> broadcast::Receiver<ArtifactEvent> {
        let guard = self.state();
        match guard.documents.get(document_key) {
            Some(document) => document.events.subscribe(),
            None => {
                let (_tx, rx) = broadcast::channel(1);
                rx
            }
        }
    }

    /// 🔔️ Sends a control message to a document's actor (e.g. a presence heartbeat or a wake).
    pub async fn send(&self, document_id: &str, message: ArtifactActorMsg) {
        self.send_key(&ArtifactDocumentKey::local(document_id), message).await;
    }

    pub async fn send_key(&self, document_key: &ArtifactDocumentKey, message: ArtifactActorMsg) {
        if let Some(document) = self.state().documents.get(document_key) {
            let _ = document.cmd_tx.send(message);
        }
    }

    /// 💓️ Offers a generic cursor/viewport/app-presence heartbeat for one open document.
    /// Returns `true` only when the host actually queued a publish; faster offers are coalesced onto
    /// the document's producer and cannot flood the preview lane.
    pub fn presence_heartbeat(&self, document_id: &str, now_ms: u64, peer: PresencePeer) -> bool {
        self.presence_heartbeat_key(&ArtifactDocumentKey::local(document_id), now_ms, peer)
    }

    /// ⏯️ Records an open document's latest tool run summary for its presence heartbeats — the host
    /// calls this with each `AppFrame::Ephemeral.tool_run` it decodes. `false` when the document is not open.
    pub fn observe_presence_tool_run_key(&self, document_key: &ArtifactDocumentKey, tool_run: Option<PresenceToolRun>) -> bool {
        let mut state = self.state();
        let Some(document) = state.documents.get_mut(document_key) else { return false };
        document.presence.observe_tool_run(tool_run);
        true
    }

    pub fn presence_heartbeat_key(&self, document_key: &ArtifactDocumentKey, now_ms: u64, peer: PresencePeer) -> bool {
        let mut state = self.state();
        let Some(document) = state.documents.get_mut(document_key) else { return false };
        let Some(peer) = document.presence.offer(now_ms, peer) else { return false };
        document.cmd_tx.send(ArtifactActorMsg::PresenceHeartbeat { peer: Box::new(peer) }).is_ok()
    }

    /// 🧹️ Ids of every currently-open document.
    /// 🚫️async: E1 pure lock-and-collect consumed by `impl Drop` (sync-only external trait) — see
    /// R9. No I/O, no suspension point.
    pub fn open_artifacts(&self) -> Vec<ArtifactDocumentKey> {
        self.state().documents.keys().cloned().collect()
    }
}
//#endregion 🔖️Host
