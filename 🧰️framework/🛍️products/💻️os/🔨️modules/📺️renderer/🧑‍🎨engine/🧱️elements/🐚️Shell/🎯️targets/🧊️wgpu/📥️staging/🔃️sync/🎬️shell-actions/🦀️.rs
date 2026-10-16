//! 🔃️ `shell-sync` staging of `shell-actions`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellActions
impl SyncState {
    //#region 🔖️NativeBackboneSync
    pub(super) fn mint_sync_binding_generation(&mut self) -> Result<u64, String> {
        let generation = self.next_sync_binding_generation;
        self.next_sync_binding_generation = generation.checked_add(1).ok_or("document-backbone binding generation exhausted")?;
        Ok(generation)
    }

    pub(super) fn active_sync_owner(&self) -> Option<ShellSyncOwner> {
        self.sync_channel.as_ref().map(shell_sync_channel_owner)
    }

    /// 🚦️ The `#s-sync-status` pill's state for the CURRENT session, the wgpu twin of
    /// `🛠️ShellHelpers/🟦️.tsx`'s `computeSyncPillState`: a non-live remote outranks a pending count,
    /// and "no status observed yet" reads as `Remote(Detached)`.
    ///
    /// Target-neutral because React's footer is: its browser shell paints `Remote: detached` from
    /// `computeSyncPillState(null)` with no backbone attached at all, and a document the browser
    /// host opened local-only never reports a remote, so it resolves to that same state.
    pub(crate) fn sync_pill(&self) -> ShellSyncPill {
        if let Some((received_bytes, total_bytes, received_chunks, total_chunks)) = self.sync_bootstrap_progress {
            return ShellSyncPill::Recovering { received_bytes, total_bytes, received_chunks, total_chunks };
        }
        let Some(status) = self.sync_status.as_ref() else { return ShellSyncPill::Remote(ShellSyncRemote::Detached) };
        if !matches!(status.remote, RemoteState::Live { .. }) {
            return ShellSyncPill::Remote(match &status.remote {
                RemoteState::Live { .. } => ShellSyncRemote::Connected,
                RemoteState::Connecting => ShellSyncRemote::Connecting,
                RemoteState::Backoff { .. } => ShellSyncRemote::Backoff,
                RemoteState::Detached => ShellSyncRemote::Detached,
            });
        }
        if status.pending_mutations > 0 {
            return ShellSyncPill::Pending(status.pending_mutations);
        }
        ShellSyncPill::Persisted
    }

    /// 📶️ This shell's hub projection over the schema's two shell axes and its attached documents. The
    /// session is held exactly while a verified `/auth/sessions/me` authority is: a hub is always
    /// configured here (the connection book keeps the local bootstrap hub), so this shell is never
    /// `none`. The link is what the identity lane last learned: a hub it could not reach while it kept
    /// the cached identity is `unreachable`, a verified authority `reachable`, anything else `verifying`.
    pub(crate) fn hub_projection(&self, directory: &DirectoryState) -> ShellHubProjectionV1 {
        let session = if directory.verified_session_authority.is_some() { HubSessionPresence::SignedIn } else { HubSessionPresence::SignedOut };
        let link = match (directory.identity_offline, directory.verified_session_authority.is_some()) {
            (true, _) => HubLink::Unreachable,
            (false, true) => HubLink::Reachable,
            (false, false) => HubLink::Verifying,
        };
        let documents = self.hub_documents.iter().map(|(document_key, remote)| ShellHubDocumentV1 { document_key: document_key.clone(), remote: remote.clone() }).collect();
        ShellHubProjectionV1 { session, link, documents }
    }

    pub(crate) fn publish_hub_document_status(&mut self, document_key: impl Into<String>, remote: ShellHubRemoteV1) {
        const HUB_DOCUMENT_CAPACITY: usize = 64;
        let document_key = document_key.into();
        if self.hub_documents.contains_key(&document_key) || self.hub_documents.len() < HUB_DOCUMENT_CAPACITY {
            self.hub_documents.insert(document_key, remote);
        }
    }

    pub(crate) fn retire_hub_document_status(&mut self, document_key: &str) {
        self.hub_documents.remove(document_key);
    }

    pub(crate) fn hub_connection_state(&self, directory: &DirectoryState) -> HubConnectionState {
        shell_hub_connection_summary_v1(&self.hub_projection(directory)).state
    }

    //#endregion 🔖️NativeBackboneSync
    //#region 🔖️CheckIn
    /// 📌️ Arms one hub Check In of the mounted hub document; a running one for the same document is
    /// kept (its head is whatever the hub acknowledges once it is submitted).
    #[cfg(not(target_arch = "wasm32"))]
    pub(super) fn request_hub_check_in(&mut self, space_id: &str, document_id: &str) {
        if self.hub_check_in.as_ref().is_some_and(|current| !current.status.as_ref().is_some_and(|status| status.phase.is_terminal()) && current.space_id == space_id && current.document_id == document_id) {
            return;
        }
        let now_ms = directory_now_ms();
        self.hub_check_in = Some(ShellHubCheckInV1 {
            space_id: space_id.to_string(),
            document_id: document_id.to_string(),
            request_id: mint_directory_command_request_id(),
            head: None,
            quiescent_by_ms: now_ms.saturating_add(HUB_CHECK_IN_QUIESCENCE_MS),
            deadline_at_ms: now_ms.saturating_add(HUB_CHECK_IN_DEADLINE_MS),
            next_poll_at_ms: now_ms,
            submitted: false,
            cancel_requested: false,
            cancel_sent: false,
            status: None,
        });
    }

    /// 📌️ The footer's hub badge, followed by the current Check In's status while one is shown.
    pub(crate) fn hub_footer_label(&self, prefs: &PrefsState, directory: &DirectoryState) -> String {
        let is_de = prefs.locale_id == "de";
        let hub = shell_hub_connection_text(self.hub_connection_state(directory), is_de);
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(status) = self.hub_check_in.as_ref().and_then(|operation| operation.status.as_ref()) {
            return format!("{hub} · {}", hub_check_in_status_text(status, is_de));
        }
        hub
    }

    //#endregion 🔖️CheckIn
    /// 🪢️ Hands the document's actor the canonical pair its guest was seeded from; nothing for a document bound to
    /// no hub.
    pub(crate) fn install_document_seed(&self, prepared: &ShellPreparedDocumentOpen, pair: Option<semio_framework_os_kernel::os_directory::CanonicalCheckpointPairV1>) -> Result<(), String> {
        let (Some(pair), Some(seed)) = (pair, prepared.seed.as_ref()) else { return Ok(()) };
        if !self.document_host.set_document_seed(&ArtifactDocumentKey::hub(seed.scope.space_id.clone(), seed.scope.document_id.clone()), pair) {
            return Err("the canonical checkpoint pair could not be handed to the document actor".into());
        }
        Ok(())
    }
}

//#endregion ShellActions

