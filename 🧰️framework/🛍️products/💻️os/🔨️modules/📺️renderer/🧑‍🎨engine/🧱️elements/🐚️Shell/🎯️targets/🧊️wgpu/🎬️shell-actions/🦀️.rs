//! 🎬️ Shell actions, part one: persistence bindings, sync, hub projection and action dispatch.
//!
//! `ShellTurn`, persistence bindings, the sync pill, hub projection and check-in, invocation history,
//! sync actions and `dispatch_action`.
//!
//! One chunk of the former `shell` monolith (ticket CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT); [`ShellState`] and the
//! shared imports stay in the parent module and are reached through `use super::*;`.

use super::*;

//#region ShellActions
/// 🧵️ One shell turn's state machine, boxed at the turn's definition (ticket 26/09/23 session 14b): a caller's await site holds this
/// pointer instead of the whole turn, which a debug build otherwise copies into its own stack slots at every await site
/// (`dispatch_action`'s poll frame was 841 KB and the live hub gate's first authored edit overflowed a 2 MiB test thread).
#[cfg(not(target_arch = "wasm32"))]
pub type ShellTurn<'a, R> = std::pin::Pin<Box<dyn std::future::Future<Output = R> + Send + 'a>>;
/// 🧵️ See the native [`ShellTurn`]; a browser turn is not `Send`.
#[cfg(target_arch = "wasm32")]
pub type ShellTurn<'a, R> = std::pin::Pin<Box<dyn std::future::Future<Output = R> + 'a>>;

/// 📦️ Moves one turn's body to the heap as a [`ShellTurn`]; `R` comes from the turn's declared output, so `?` inside infers.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn shell_turn<'a, R>(turn: impl std::future::Future<Output = R> + Send + 'a) -> ShellTurn<'a, R> {
    Box::pin(turn)
}
/// 📦️ See the native [`shell_turn`].
#[cfg(target_arch = "wasm32")]
pub(crate) fn shell_turn<'a, R>(turn: impl std::future::Future<Output = R> + 'a) -> ShellTurn<'a, R> {
    Box::pin(turn)
}

impl ShellState {
    //#region 🔖️NativeBackboneSync
    /// 🧭️ Parses a local sync-card uri into the `framework/sync` persistence bindings a
    /// document actor opens. `folder://` → the multi-document append-only event log; `file://x.json` → its
    /// parent folder's store (single-blob export demoted per the plan). A `remote://` uri names a hub
    /// document of its own and is read by [`parse_remote_backbone_uri`]; one missing a part is refused.
    pub(crate) fn parse_persistence_binding(uri: &str) -> Result<Vec<PersistenceBinding>, String> {
        if uri.starts_with("remote://") {
            return Err(format!("a remote backbone names host, space and document: {uri}"));
        }
        if let Some(path) = uri.strip_prefix("folder://") {
            return Ok(vec![PersistenceBinding::Folder { path: std::path::PathBuf::from(path) }]);
        }
        if let Some(path) = uri.strip_prefix("file://") {
            let parent = std::path::Path::new(path).parent().map(std::path::Path::to_path_buf).unwrap_or_else(|| std::path::PathBuf::from("."));
            return Ok(vec![PersistenceBinding::Folder { path: parent }]);
        }
        Err(format!("unsupported backbone uri: {uri}"))
    }

    fn mint_sync_binding_generation(&mut self) -> Result<u64, String> {
        let generation = self.next_sync_binding_generation;
        self.next_sync_binding_generation = generation.checked_add(1).ok_or("document-backbone binding generation exhausted")?;
        Ok(generation)
    }

    fn active_sync_owner(&self) -> Option<ShellSyncOwner> {
        self.sync_channel.as_ref().map(shell_sync_channel_owner)
    }

    /// ✂️ Retires the guest's exact generation before stopping the actor. A refusal still
    /// closes the local owner and is returned to the caller; it is never retried or hidden.
    async fn detach_sync_backbone_internal(&mut self) -> Result<(), String> {
        #[cfg(not(target_arch="wasm32"))]
        self.cancel_inference_port();
        self.document_execution_target_lease=None;
        let owner = self.active_sync_owner();
        let retirement = if let Some(owner) = owner.as_ref() {
            match self.plugins.iter().find(|entry| entry.plugin_id == owner.plugin_id).cloned() {
                Some(plugin) => plugin.retire_document_backbone(owner.instance_id, owner.binding_generation, &owner.actor_uri).await,
                None => Err("document-backbone plugin owner is missing".into()),
            }
        } else {
            Ok(Vec::new())
        };
        let retirement = retirement.map_err(|refusal| self.note_refused_open(refusal));
        if let (Some(owner), Ok(effects)) = (owner.as_ref(), retirement.as_ref()) {
            if self.active_sync_owner().as_ref().is_some_and(|active| shell_sync_owner_matches(active, owner)) {
                if let Some(channel) = self.sync_channel.as_ref() {
                    let remaining = route_document_backbone_effects(&channel.actor_uri, &channel.cmd_tx, effects.clone())?;
                    if let Some(session) = self.session.as_ref() {
                        self.queue_host_effects(&session.app.controller_id.clone(), remaining);
                    }
                }
            }
        }
        #[cfg(not(target_arch="wasm32"))]
        self.cancel_inference_port();
        self.document_execution_target_lease=None;
        if let Some(channel) = self.sync_channel.take() {
            self.hub_documents.remove(&shell_hub_document_key(&channel.document_key));
            let _ = channel.cmd_tx.send(ArtifactActorMsg::Detach);
            self.document_host.close_key(&channel.document_key);
        }
        self.sync_status = None;
        self.sync_bootstrap_progress = None;
        self.sync_reseed = None;
        self.presence_peers.clear();
        self.presence_surface = None;
        self.presence_self = None;
        retirement.map(|_| ())
    }

    /// 📬️ Drains the active document actor's event stream into the plugin store and the sync
    /// badge. Called once per frame-deferred pump on both targets — the render loop already redraws
    /// continuously, so a `try_recv` poll suffices and no `EventLoopProxy` wake is needed.
    /// `RemoteMutations` are force-applied via `apply_mutations` (idempotent by operation id), which also covers
    /// idle frames where the sandboxed store never pumps its `ChannelBackbone` on its own. Returns
    /// whether anything changed. Only a change to the guest's document refreshes the guest's bodies: a
    /// status, bootstrap or conflict change republishes the host-owned Sync panel, and a presence roster
    /// is painted from shell state by the footer on the next frame. Refreshing every guest body per
    /// presence frame (ten per second per peer) held each pump for 3.4–5.6 s in a debug build and froze
    /// the shell while a peer was online (ticket 26/09/23 slice WG8, two-user gate run 15).
    ///
    /// 📇️ ticket §1/§3 — non-blocking identity bootstrap result + directory-stream/command-queue
    /// drain, folded into this same every-frame pump (glue.rs's frame loop already calls this
    /// method once per tick; adding a second call site outside this lane's lease was avoidable).
    ///
    /// 📌️ ticket §C5 item 2 — the auto check-in poll, folded into the same every-frame pump for the
    /// identical reason (no timer wheel exists in this shell; see `auto_checkin_should_fire`'s doc).
    ///
    /// 🎠️ H3-wgpu-native — `apply_mutations` now runs its plugin turn on the
    /// dedicated kernel thread instead of in-process here; `.await` is the only
    /// change this call site needs (`pump_sync_events` was already `async fn`).
    ///
    /// 👥️ ticket §5 — shell-LOCAL roster (deliberately NOT the shared kernel
    /// `ViewModel`, which still has no presence field). Rendered by
    /// `render_presence_bar` in the footer, scoped to `presence_surface`.
    ///
    /// 👥️ This connection's hub-admitted identity (actor + colour), sent once per connection.
    /// The sync actor stamps it onto outbound heartbeats itself; the shell keeps it so its
    /// board overlays leave the local actor out and paint in the hub's own colour.
    ///
    /// 👻️ Ephemeral peer previews (wire v2's uncredited preview lane) have no native
    /// wgpu shell UI yet — same documented-follow-up status as `Presence` above.
    ///
    /// 📮️ Terminal batch dispositions (accepted/transformed/rejected) have no native
    /// wgpu shell surfacing yet — `RemoteMutations`/rollback already keep document
    /// state correct; this event is purely informational until a UI is built for it.
    pub async fn pump_sync_events(&mut self) -> bool {
        use tokio::sync::broadcast::error::TryRecvError;
        #[cfg(not(target_arch = "wasm32"))]
        let shell_io_changed = self.poll_shell_io();
        #[cfg(target_arch = "wasm32")]
        let shell_io_changed = false;
        let directory_changed = self.pump_directory_events().await || shell_io_changed;
        #[cfg(not(target_arch = "wasm32"))]
        let directory_changed = self.poll_time_travel_progress().await || directory_changed;
        let directory_changed = self.drain_operation_publications().await || directory_changed;
        let directory_changed = self.publish_peer_time_travel_notes() || directory_changed;
        let directory_changed = self.reattach_remembered_local_folder().await || directory_changed;
        #[cfg(target_arch = "wasm32")]
        let directory_changed = self.flush_host_route_folder().await || directory_changed;
        self.poll_auto_checkin().await;
        let directory_changed = self.advance_document_opening().await || directory_changed;
        let directory_changed = self.advance_sync_reseed().await || directory_changed;
        if let Some(error) = self.sync_terminal_fault.take() {
            Self::debug_log(&format!("[TRACE] wgpu shell retired document-backbone owner after terminal fault: {error}"));
            let _ = self.detach_sync_backbone_internal().await;
            self.sync_card_kind = Some("conflict".into());
            self.relabel_sync_tab();
            return true;
        }
        let (owner, events) = {
            let Some(channel) = self.sync_channel.as_mut() else {
                return directory_changed;
            };
            let mut events: Vec<ArtifactEvent> = Vec::new();
            loop {
                match channel.events.try_recv() {
                    Ok(event) => events.push(event),
                    Err(TryRecvError::Empty) | Err(TryRecvError::Closed) => break,
                    Err(TryRecvError::Lagged(_)) => continue,
                }
            }
            if events.is_empty() {
                return directory_changed;
            }
            (shell_sync_channel_owner(channel), events)
        };
        let instance_id = owner.instance_id;
        let plugin = self.plugins.iter().find(|entry| entry.plugin_id == owner.plugin_id).cloned();
        let mut changed = directory_changed;
        let mut document_changed = false;
        let mut sync_panel_changed = false;
        for event in events {
            match event {
                ArtifactEvent::RemoteMutations { envelopes } => {
                    if let Some(plugin) = plugin.as_ref() {
                        let operations = protocol::encode_envelopes(&envelopes);
                        match plugin.apply_mutations(instance_id, &operations).await {
                            Ok(()) => {
                                changed = true;
                                document_changed = true;
                            }
                            Err(error) => { let detail = self.note_refused_open(error); Self::debug_log(&format!("[TRACE] wgpu shell apply_mutations failed: {detail}")); },
                        }
                    }
                }
                ArtifactEvent::DocumentArchiveReplaced { archive } => {
                    self.sync_bootstrap_progress = None;
                    if let Some(plugin) = plugin.as_ref() {
                        let archive = match protocol::decode_document_archive_bytes(&archive).await {
                            Ok(archive) => archive,
                            Err(error) => {
                                Self::debug_log(&format!("[TRACE] wgpu shell recursive document archive decode failed: {error}"));
                                continue;
                            }
                        };
                        match plugin.load_app_document_archive(instance_id, &archive).await {
                            Ok(()) => {
                                self.refresh_history_snapshot().await;
                                changed = true;
                                document_changed = true;
                            }
                            Err(error) => { let detail = self.note_refused_open(error); Self::debug_log(&format!("[TRACE] wgpu shell load_app_document_archive failed: {detail}")); },
                        }
                    }
                }
                ArtifactEvent::BootstrapProgress { received_bytes, total_bytes, received_chunks, total_chunks } => {
                    self.sync_bootstrap_progress = Some((received_bytes, total_bytes, received_chunks, total_chunks));
                    changed = true;
                    sync_panel_changed = true;
                }
                ArtifactEvent::Status(status) => {
                    if let Some(document_key) = self.sync_channel.as_ref().map(|channel| shell_hub_document_key(&channel.document_key)) {
                        self.publish_hub_document_status(document_key, shell_hub_remote(&status.remote));
                    }
                    self.sync_link = shell_sync_link_after_status(self.sync_link, &status.remote);
                    self.sync_status = Some(status);
                    changed = true;
                    sync_panel_changed = true;
                }
                ArtifactEvent::RebootstrapRequired { control } => {
                    self.sync_reseed = Some(ShellSyncReseed { control, pending: None, retry_at_ms: 0.0 });
                    changed = true;
                    sync_panel_changed = true;
                }
                ArtifactEvent::Presence { peers } => {
                    self.presence_peers = peers;
                    changed = true;
                }
                ArtifactEvent::Conflict(message) => {
                    if let Some((code, text, severity)) = time_travel::history_refusal_notice(&message.code.0, self.active_locale()).filter(|(code, _, _)| !time_travel::history_refusal_is_silent(code)) {
                        self.show_transient_notice(text, severity, Some(code));
                    }
                    if let Some(terminal) = shell_sync_link_terminal(&message.code.0) {
                        self.sync_link = terminal;
                        self.sync_terminal_fault = Some(message.message);
                    }
                    self.sync_card_kind = Some("conflict".into());
                    changed = true;
                    sync_panel_changed = true;
                }
                ArtifactEvent::Session { actor, color } => {
                    self.presence_self = Some((actor, color));
                    changed = true;
                }
                ArtifactEvent::DocumentBackbone { message } => {
                    let result = match plugin.as_ref() {
                        Some(plugin) => plugin.receive_document_backbone(owner.instance_id, &owner.actor_uri, message).await,
                        None => Err("document-backbone plugin owner is missing".into()),
                    };
                    match result {
                        Ok(effects) if self.active_sync_owner().as_ref().is_some_and(|active| shell_sync_owner_matches(active, &owner)) => {
                            let routed = self.sync_channel.as_ref().ok_or_else(|| "document-backbone owner retired before completion".to_string()).and_then(|channel| route_document_backbone_effects(&channel.actor_uri, &channel.cmd_tx, effects));
                            match routed {
                                Ok(remaining) => {
                                    let controller_id = self.session.as_ref().map(|session| session.app.controller_id.clone()).unwrap_or_default();
                                    self.queue_host_effects(&controller_id, remaining);
                                    changed = true;
                                    document_changed = true;
                                }
                                Err(error) => self.sync_terminal_fault = Some(error),
                            }
                        }
                        Ok(_) => {}
                        Err(error) => self.sync_terminal_fault = Some(self.note_refused_open(error)),
                    }
                }
                ArtifactEvent::Preview { .. } => {
                }
                ArtifactEvent::CommandOutcome { outcome: store_sync::sync::CommandAckOutcome::Rejected { code, messages, .. }, .. } => {
                    let (code, text, severity) = command_rejection_notice(code, &messages, self.active_locale());
                    self.show_transient_notice(text, severity, Some(code));
                    changed = true;
                }
                ArtifactEvent::CommandOutcome { outcome: store_sync::sync::CommandAckOutcome::Transformed, .. } => {
                    self.show_transient_notice(shell_chrome_string("conflict.hubTransformed", self.locale_id == "de"), semio_framework::Severity::Info, Some(SYNC_COMMAND_TRANSFORMED_NOTICE_CODE));
                    changed = true;
                }
                ArtifactEvent::CommandOutcome { .. } => {}
                ArtifactEvent::DocumentArchiveAbsent => {}
            }
            if self.sync_terminal_fault.is_some() {
                break;
            }
        }
        if let Some(error) = self.sync_terminal_fault.take() {
            Self::debug_log(&format!("[TRACE] wgpu shell retired document-backbone owner after terminal fault: {error}"));
            let _ = self.detach_sync_backbone_internal().await;
            self.sync_card_kind = Some("conflict".into());
            changed = true;
            document_changed = true;
        }
        if changed {
            self.relabel_sync_tab();
        }
        if document_changed {
            let _ = self.refresh_ui(UiDirtyScope::Full).await;
        } else if sync_panel_changed {
            let _ = self.republish_shell_panel_document(FRAMEWORK_SYNC_PANEL_TAB_ID);
        }
        changed
    }

    /// 🚦️ Rewrites the live dock's sync leaf label from the CURRENT pill. The label is baked into the dock
    /// when it is (re)built, so without this the footer pill — and its accessible name — kept `Remote:
    /// detached` after the actor reported `Live` until something else rebuilt the dock (measured, run s12c).
    pub(crate) fn relabel_sync_tab(&mut self) {
        let label = shell_sync_pill_text(self.sync_pill(), self.locale_id == "de");
        for anchor in PanelAnchor::ALL {
            if let Some(tab) = self.dock_tabs.tabs_mut(anchor).iter_mut().find(|tab| tab.id == FRAMEWORK_SYNC_PANEL_TAB_ID) {
                tab.label = label.clone();
            }
        }
    }

    /// 👥️ The roster `#s-presence-peers` paints, scoped to the attached surface. A document opened
    /// without a hub binding carries no surface and answers an empty roster — exactly the state
    /// React's footer renders as `No one else is here`, not a reason to omit the pill.
    ///
    /// ⏪️ A peer editing the history wears its time-travel activity (`time_travel::time_travel_peer_presence`).
    pub(crate) fn footer_presence_rows(&self) -> Vec<ui_wgpu::wgpu::PresencePeerRow> {
        let Some(surface) = self.presence_surface.as_deref() else { return Vec::new() };
        let mut rows = presence_peer_rows_for_surface(&self.presence_peers, Some(surface), surface);
        let mut activities = self.peer_time_travel_presence().activities;
        if !activities.is_empty() {
            for row in &mut rows {
                if let Some(index) = activities.iter().position(|(actor, _)| *actor == row.actor) {
                    row.activity = Some(activities.swap_remove(index).1);
                }
            }
        }
        rows
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
    pub(crate) fn hub_projection(&self) -> ShellHubProjectionV1 {
        let session = if self.verified_session_authority.is_some() { HubSessionPresence::SignedIn } else { HubSessionPresence::SignedOut };
        let link = match (self.identity_offline, self.verified_session_authority.is_some()) {
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

    pub(crate) fn hub_connection_state(&self) -> HubConnectionState {
        shell_hub_connection_summary_v1(&self.hub_projection()).state
    }

    /// 🎭️ ticket §1 — contract §C0's `user:{userId}#{sessionId}` once identity is minted/restored,
    /// else the pre-identity local default this shell always used (`shell_actor`'s pure decision).
    pub(crate) fn current_shell_actor(&self, instance_id: u32) -> String {
        shell_actor(self.identity.as_ref(), &self.shell_session_id, instance_id)
    }
    //#endregion 🔖️NativeBackboneSync

    //#region 🔖️CheckIn
    /// 🧾️ ticket §C5 — resets the local history projection and re-seeds it from a full
    /// `ProgramBridgeEntry::read_history` snapshot (`replace=true`), called right after a document
    /// attaches (`open_document`/`attach_sync_backbone`'s native branch) so a just-opened document's
    /// own uncommitted-edit count starts from its OWN history, never a leftover projection from
    /// whatever was open before. Best-effort: a read failure just leaves the projection empty (the
    /// same posture every other native exchange call in this file already takes on error — logged, not
    /// propagated, since a stale/absent history must never block opening a document).
    pub(crate) async fn refresh_history_snapshot(&mut self) {
        self.history_cursor = 0;
        self.history_entries.clear();
        self.history_current_checkpoint_id = None;
        self.history_time_travel = None;
        self.history_reprojection = None;
        self.last_uncommitted_edit_at_ms = None;
        self.auto_checkin_pending = false;
        self.checkpoint_dispatched = false;
        self.seed_history_snapshot().await;
    }

    /// 📖️ Reads the session program's whole history projection over the local one (`replace = true`): its rows, head,
    /// history-edit session and waiting reprojection. `false` when no session's program could be asked. What a session
    /// start seeds from ([`Self::seed_history_snapshot`]) and what a lossy publication lane falls back to
    /// ([`Self::drain_operation_publications`]).
    pub(crate) async fn reread_history_projection(&mut self) -> bool {
        let Some(session) = self.session.as_ref() else { return false };
        let Some(plugin) = self.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id) else { return false };
        match plugin.read_history(session.instance_id).await {
            Ok(patch) => {
                fold_history_patch(&mut self.history_entries, &mut self.history_cursor, &patch, true);
                self.observe_history_time_travel(patch.time_travel.as_ref());
                self.observe_history_reprojection(patch.reprojection.as_ref());
                self.history_current_checkpoint_id = patch.current_checkpoint_id;
            }
            Err(error) => { let detail = self.note_refused_open(error); Self::debug_log(&format!("[TRACE] wgpu shell read_history failed: {detail}")); },
        }
        true
    }

    /// 🧾️ The seeding half of [`Self::refresh_history_snapshot`]: the program's whole history projection, read through
    /// the native exchange's `ReadHistory` or the browser bridge's `readHistory` alike, so a session start or a restored
    /// archive shows its rows and head on both builds before the next `history_patch` folds in.
    ///
    /// ⚔️ A freshly attached document brings its own open conflicts with it — React seeds the
    /// Conflicts panel on the same session-start/switch edge; the `ReadConflicts` exchange is native-only.
    async fn seed_history_snapshot(&mut self) {
        if !self.reread_history_projection().await {
            return;
        }
        self.conflicts_seeded = false;
        self.open_conflicts.clear();
        self.selected_conflict_id = None;
        #[cfg(not(target_arch = "wasm32"))]
        self.seed_open_conflicts().await;
    }

    /// 🧾️ ticket §C5 — folds an `InvocationResult.history_patch` (present on every `handleAction`/
    /// `handleCommand` response, per `🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs`'s `AppFrame::Invocation` decode)
    /// into the running projection, updates the idle clock, and — the instant a checkpoint THIS shell
    /// itself asked for (`checkpoint_dispatched`) is observed to have actually landed — fires
    /// `TouchArtifact` on the space index (§C5 item 6). Called from both `dispatch_action` and
    /// `dispatch_command` right after their own `program.handle_action`/`handle_command` call.
    ///
    /// 🪐️ `TouchArtifact` on the space index needs the native `ArtifactHost` document-sync
    /// backbone (it opens a second actor for the `s.space` index document). The browser build
    /// links no such host, so the checkpoint still lands and the projection still folds — only
    /// the index touch is skipped, and skipping it is visible in this one place.
    pub(crate) fn observe_invocation_history<'a>(&'a mut self, history_patch: Option<&'a semio_framework::kernel::HistoryPatch>) -> ShellTurn<'a, ()> {
        shell_turn(async move {
            let Some(patch) = history_patch else { return };
            if patch.cursor >= self.history_cursor {
                self.observe_history_time_travel(patch.time_travel.as_ref());
                self.observe_history_reprojection(patch.reprojection.as_ref());
            }
            if !fold_history_patch(&mut self.history_entries, &mut self.history_cursor, patch, false) {
                return;
            }
            let count = uncommitted_edit_count(&self.history_entries);
            if count == 0 {
                self.last_uncommitted_edit_at_ms = None;
                self.auto_checkin_pending = false;
            } else {
                self.last_uncommitted_edit_at_ms = Some(chrome_now_ms() as i64);
            }
            let landed = checkpoint_landed(self.history_current_checkpoint_id.as_deref(), patch.current_checkpoint_id.as_deref(), self.checkpoint_dispatched);
            if patch.current_checkpoint_id.is_some() {
                self.history_current_checkpoint_id = patch.current_checkpoint_id.clone();
            }
            if !landed {
                return;
            }
            self.checkpoint_dispatched = false;
            #[cfg(not(target_arch = "wasm32"))]
            {
                let Some(space_id) = self.open_space_id.clone() else { return };
                let Some(document_id) = self.sync_channel.as_ref().map(|channel| channel.document_id.clone()) else { return };
                self.request_hub_check_in(&space_id, &document_id);
                if document_id != SPACE_INDEX_DOCUMENT_ID {
                    self.touch_space_index_artifact(&space_id, &document_id).await;
                }
            }
        })
    }

    /// 📌️ ticket §C5 item 2 — the per-frame auto check-in poll, called from `pump_sync_events` (see
    /// that method's own doc: "folded into the same every-frame pump"). Viewer sessions never arm
    /// (`can_check_in`, item 5's guard) — the SAME predicate the `#s-checkin` affordance itself checks.
    async fn poll_auto_checkin(&mut self) {
        let Some(session) = self.session.as_ref() else { return };
        if !can_check_in(session.app.role) {
            return;
        }
        let count = uncommitted_edit_count(&self.history_entries);
        if count == 0 {
            self.auto_checkin_pending = false;
            self.last_uncommitted_edit_at_ms = None;
            return;
        }
        let now_ms = chrome_now_ms() as i64;
        if auto_checkin_should_fire(count, self.auto_checkin_pending, self.last_uncommitted_edit_at_ms, now_ms, AUTO_CHECKIN_IDLE_MS, AUTO_CHECKIN_EDIT_THRESHOLD) {
            self.auto_checkin_pending = true;
            self.dispatch_checkpoint("auto").await;
        }
    }

    /// 📌️ ticket §C5 items 2/3/4 — fires `commitCheckpoint` through the same action funnel the
    /// History panel's own "Checkpoint" button uses (`history_command` in `🔌️plugin/🦀️.rs`),
    /// with `authors` riding along (`history_command` still hardcodes `authors: Vec::new()` today per
    /// `📓️w3-a-report.md`'s own `sharedFileRequest` — sent anyway so this is ready the moment that
    /// lands). Viewer-guarded (item 5) so a stray call from `checkpoint_before_detach`/`poll_auto_checkin`
    /// can never reach a viewer session even if their own callers' guards were ever relaxed.
    async fn dispatch_checkpoint(&mut self, message: &str) {
        let Some(session) = self.session.clone() else { return };
        if !can_check_in(session.app.role) {
            return;
        }
        self.checkpoint_dispatched = true;
        let authors: Vec<semio_framework::DslValue> = self.identity.as_ref().map(|identity| vec![semio_framework::dsl_value!({ "id": identity.user_id, "name": identity.display_name })]).unwrap_or_default();
        let action = ActionDescriptor { controller_id: session.app.controller_id.clone(), action: "commitCheckpoint".into(), args: crate::action_args_json!({ "message": message, "authors": authors }) };
        if let Err(error) = self.dispatch_action(action).await {
            Self::debug_log(&format!("[TRACE] wgpu shell check-in checkpoint dispatch failed: {error}"));
            self.checkpoint_dispatched = false;
        }
    }

    /// 📌️ ticket §C5 item 4 — checkpoint-on-close: called right before EVERY
    /// `detach_sync_backbone_internal()` call site (this shell keeps exactly one document mounted at a
    /// time, so attaching a different backbone IS closing whatever was open — same posture the React
    /// shell's own report documents). Best-effort (mirrors React's "fire-and-forget, not gated on the
    /// success-detection effect" note) — `dispatch_checkpoint`/`observe_invocation_history` still run
    /// the normal detection+`TouchArtifact` path; this call site just doesn't await or fail the caller
    /// on their outcome, since a failed close-time checkpoint must never block navigating away.
    async fn checkpoint_before_detach(&mut self) {
        let Some(session) = self.session.as_ref() else { return };
        let count = uncommitted_edit_count(&self.history_entries);
        if !should_checkpoint_before_detach(session.app.role, self.sync_channel.is_some(), count) {
            return;
        }
        self.dispatch_checkpoint("auto").await;
    }

    /// 📌️ ticket §C5 item 6 — `TouchArtifact` on the space's `index` document after a successful
    /// checkpoint, mirroring the React shell's `touchSpaceIndexArtifact`. Reuses the CURRENTLY visible
    /// session outright when it already IS this exact space's own index document (avoiding a second
    /// `document_host` actor under the shared, non-space-scoped `"index"` key entirely); otherwise
    /// spawns a FRESH, non-visible `s.space` editor instance + backbone attach for the duration of the
    /// one `touchArtifact` dispatch, then tears both down immediately. Unlike the React shell's cached
    /// background instance (`spaceIndexInstanceRef`), this does not cache across calls: `document_host`
    /// keys its actor registry by the bare document id, not `(spaceId, documentId)`, so a PERSISTENT
    /// background actor under `"index"` risks silently being replaced by a later legitimate
    /// `open_document`/`attach_sync_backbone` call for a DIFFERENT space's index (or vice versa) with
    /// no way for either side to detect the swap — spinning up fresh each time trades a little
    /// efficiency for provable correctness under that shared-key constraint.
    ///
    /// 🐚️ Reuse the live session outright when it's already this exact space's own index document.
    /// Calls `program.handle_command` DIRECTLY rather than `self.dispatch_command` (which would
    /// recurse back into `observe_invocation_history` → `touch_space_index_artifact` — `rustc`
    /// rightly refuses that as unbounded async recursion without `Box::pin`; a direct low-level
    /// call is also the more honest shape here, since `touchArtifact` requests no `Effect`s for
    /// `dispatch_command`'s own effect loop to process).
    ///
    /// 🐚️ `document_host` keys its actor registry by the bare document id ("index", not
    /// space-scoped) — opening a background actor under that key while the MAIN slot is ALSO
    /// showing "index" for a DIFFERENT space would silently sever that live session's own backbone
    /// (`ArtifactHost::open`'s own "idempotent per id" contract). Skip rather than risk it.
    #[cfg(not(target_arch = "wasm32"))]
    async fn touch_space_index_artifact(&mut self, space_id: &str, artifact_id: &str) {
        let now_ms = chrome_now_ms();
        let actor = self.identity.as_ref().map(|identity| identity.user_id.clone()).unwrap_or_else(|| self.shell_session_id.clone());
        let arguments: BTreeMap<String, DslValue> = BTreeMap::from([("id".to_string(), semio_framework_value::DslValue::String(artifact_id.to_string())), ("nowMs".to_string(), semio_framework_value::DslValue::float(now_ms)), ("actor".to_string(), semio_framework_value::DslValue::String(actor.clone()))]);
        if self.sync_channel.as_ref().map(|channel| channel.document_id.as_str()) == Some(SPACE_INDEX_DOCUMENT_ID) && self.open_space_id.as_deref() == Some(space_id) {
            let Some(session) = self.session.clone() else { return };
            let Some(program) = self.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).cloned() else { return };
            let Some(app) = find_space_index_app(&program, semio_framework::manifest::AppRole::Editor).cloned() else { return };
            let invocation = semio_framework::manifest::CommandInvocation {
                address: semio_framework::manifest::CommandAddress { owner: semio_framework::manifest::CommandOwnerAddress::App { plugin_id: session.plugin_id.clone(), app_id: app.id.clone() }, command_id: "touchArtifact".into() },
                arguments,
            };
            let command_json = semio_framework_pack_json::to_json_string(&invocation);
            match program.handle_command(session.instance_id, &command_json, &session.view_state).await {
                Ok(result) => self.queue_host_effects(&session.app.controller_id, result.requested_effects),
                Err(error) => { let detail = self.note_refused_open(error); Self::debug_log(&format!("[TRACE] wgpu shell touchArtifact (live session) failed: {detail}")); },
            }
            return;
        }
        if self.sync_channel.as_ref().map(|channel| channel.document_id.as_str()) == Some(SPACE_INDEX_DOCUMENT_ID) {
            Self::debug_log(&format!("[TRACE] wgpu shell touchArtifact skipped: main session already holds the shared \"index\" actor for a different space"));
            return;
        }
        let Some(program) = self.plugins.iter().find(|entry| find_space_index_app(entry, semio_framework::manifest::AppRole::Editor).is_some()).cloned() else { return };
        let Some(app) = find_space_index_app(&program, semio_framework::manifest::AppRole::Editor).cloned() else { return };
        let Some(space_index) = program_space_index(&program) else { return };
        let instance_id = match program.create_app(&app.id).await {
            Ok(id) => id,
            Err(error) => {
                Self::debug_log(&format!("[TRACE] wgpu shell touchArtifact background instance failed: {error}"));
                return;
            }
        };
        let data_dir = self.identity_env.as_ref().and_then(|env| env.data_dir.as_deref());
        let surface = semio_framework::manifest::surface_app_id(&app.dialect, semio_framework::manifest::AppRole::Editor);
        let bindings = default_persistence_bindings(self.identity.as_ref(), Some(space_id), data_dir, Some(surface.as_str()));
        let actor_uri = format!("actor://{SPACE_INDEX_DOCUMENT_ID}");
        if let Err(error) = bind_wgpu_document_socket_surface(&self.document_host, SPACE_INDEX_DOCUMENT_ID, &space_index.document_schema, &bindings, &program, &app, &app.window_kinds.first().id) {
            Self::debug_log(&format!("[TRACE] wgpu shell touchArtifact document admission failed: {error}"));
            program.destroy_app(instance_id);
            return;
        }
        let channels = self.document_host.open(ArtifactActorConfig { actor_identity_grant: semio_framework_value::RetainedCloneGrant {maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64},  document_id: SPACE_INDEX_DOCUMENT_ID.to_string(), schema: S_SPACE_INDEX_DOCUMENT_SCHEMA.to_string(), bindings, watch_external: true, actor: actor.clone() }).await;
        let binding_generation = match self.mint_sync_binding_generation() {
            Ok(generation) => generation,
            Err(error) => {
                Self::debug_log(&format!("[TRACE] wgpu shell touchArtifact binding generation failed: {error}"));
                self.document_host.close_key(&channels.document_key);
                program.destroy_app(instance_id);
                return;
            }
        };
        let binding_effects = match program.bind_document_backbone(instance_id, binding_generation, &actor_uri).await {
            Ok(effects) => effects,
            Err(error) => {
                let detail = self.note_refused_open(error);
                if let Err(refusal) = program.retire_document_backbone(instance_id, binding_generation, &actor_uri).await { self.note_refused_open(refusal); }
                Self::debug_log(&format!("[TRACE] wgpu shell touchArtifact document-backbone bind failed: {detail}"));
                let _ = channels.cmd_tx.send(ArtifactActorMsg::Detach);
                self.document_host.close_key(&channels.document_key);
                program.destroy_app(instance_id);
                return;
            }
        };
        match route_document_backbone_effects(&actor_uri, &channels.cmd_tx, binding_effects) {
            Ok(remaining) => self.queue_host_effects(&app.controller_id, remaining),
            Err(error) => Self::debug_log(&format!("[TRACE] wgpu shell touchArtifact document-backbone egress failed: {error}")),
        }
        let _ = channels.cmd_tx.send(ArtifactActorMsg::LocalMutations { envelopes: Vec::new() });
        let invocation = semio_framework::manifest::CommandInvocation {
            address: semio_framework::manifest::CommandAddress { owner: semio_framework::manifest::CommandOwnerAddress::App { plugin_id: program.plugin_id.clone(), app_id: app.id.clone() }, command_id: "touchArtifact".into() },
            arguments,
        };
        let command_json = semio_framework_pack_json::to_json_string(&invocation);
        let view_state = ViewModel::new(self.active_locale(), self.active_terminology());
        match program.handle_command(instance_id, &command_json, &view_state).await {
            Ok(result) => match route_document_backbone_effects(&actor_uri, &channels.cmd_tx, result.requested_effects) {
                Ok(remaining) => self.queue_host_effects(&app.controller_id, remaining),
                Err(error) => Self::debug_log(&format!("[TRACE] wgpu shell touchArtifact document-backbone egress failed: {error}")),
            },
            Err(error) => { let detail = self.note_refused_open(error); Self::debug_log(&format!("[TRACE] wgpu shell touchArtifact dispatch failed: {detail}")); },
        }
        if let Err(error) = program.retire_document_backbone(instance_id, binding_generation, &actor_uri).await {
            let detail = self.note_refused_open(error);
            Self::debug_log(&format!("[TRACE] wgpu shell touchArtifact document-backbone retirement failed: {detail}"));
        }
        let _ = channels.cmd_tx.send(ArtifactActorMsg::Detach);
        self.document_host.close_key(&channels.document_key);
        program.destroy_app(instance_id);
    }

    /// 📌️ ticket §C5 item 3 — explicit check-in's message-prompt dialog funnel (`#s-checkin`'s own
    /// dedicated controller, routed alongside `"framework.sync"` in `dispatch_action`): `open` shows
    /// the dialog, `cancel` closes it without dispatching, `submit` dispatches with the typed message
    /// (falling back to `"check-in"` for an empty/whitespace-only draft, matching the React shell's own
    /// `submitCheckin`).
    fn handle_checkin_action<'a>(&'a mut self, action: ActionDescriptor) -> ShellTurn<'a, Result<(), String>> {
        shell_turn(async move {
            match action.action.as_str() {
                "open" => {
                    self.checkin_dialog_draft = Some(String::new());
                    Ok(())
                }
                "cancel" => {
                    self.checkin_dialog_draft = None;
                    Ok(())
                }
                "submit" => {
                    let message = action.args.as_ref().and_then(|args| args.get("message")).and_then(semio_framework_value::DslValue::as_str).map(str::trim).filter(|value| !value.is_empty()).unwrap_or("check-in").to_string();
                    self.checkin_dialog_draft = None;
                    self.dispatch_checkpoint(&message).await;
                    Ok(())
                }
                _ => Ok(()),
            }
        })
    }

    /// 📌️ Arms one hub Check In of the mounted hub document; a running one for the same document is
    /// kept (its head is whatever the hub acknowledges once it is submitted).
    #[cfg(not(target_arch = "wasm32"))]
    fn request_hub_check_in(&mut self, space_id: &str, document_id: &str) {
        if self.hub_check_in.as_ref().is_some_and(|current| !current.status.as_ref().is_some_and(|status| status.phase.is_terminal()) && current.space_id == space_id && current.document_id == document_id) {
            return;
        }
        let now_ms = Self::directory_now_ms();
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

    /// 📌️ Drives the Check In by at most one bounded hub request per frame: waits (bounded) for the
    /// document's sync status to name an acknowledged head, submits it, then polls or cancels to a
    /// terminal status. Answers whether the footer's status changed.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) async fn pump_hub_check_in(&mut self) -> bool {
        use semio_framework_os_kernel::os_directory::{DocumentCheckInPhaseV1, DocumentCheckInRefusalV1, DocumentCheckInV1, DOCUMENT_CHECK_IN_SCHEMA_V1};
        let Some(operation) = self.hub_check_in.clone() else { return false };
        if operation.status.as_ref().is_some_and(|status| status.phase.is_terminal()) {
            return false;
        }
        let now_ms = Self::directory_now_ms();
        let local_refusal = |refusal| Some(hub_check_in_local_status(&operation.request_id, DocumentCheckInPhaseV1::Failed, Some(refusal)));
        let Some(client) = self.directory_client.clone() else {
            self.hub_check_in.as_mut().expect("armed check-in").status = local_refusal(DocumentCheckInRefusalV1::AuthorityChanged);
            return true;
        };
        if now_ms >= operation.deadline_at_ms {
            self.hub_check_in.as_mut().expect("armed check-in").status = local_refusal(DocumentCheckInRefusalV1::Unavailable);
            return true;
        }
        let mounted = self.sync_channel.as_ref().is_some_and(|channel| channel.document_id == operation.document_id) && self.open_space_id.as_deref() == Some(operation.space_id.as_str());
        let Some(head) = operation.head.clone() else {
            if !mounted || now_ms >= operation.quiescent_by_ms {
                self.hub_check_in.as_mut().expect("armed check-in").status = local_refusal(DocumentCheckInRefusalV1::Unavailable);
                return true;
            }
            let Some(head) = self.sync_status.as_ref().and_then(|status| status.acknowledged_head.clone()).filter(|head| head.document_id == operation.document_id) else { return false };
            let current = self.hub_check_in.as_mut().expect("armed check-in");
            current.head = Some(head);
            current.status = Some(hub_check_in_local_status(&current.request_id, DocumentCheckInPhaseV1::Accepted, None));
            return true;
        };
        let cancel_due = operation.cancel_requested && !operation.cancel_sent;
        if operation.submitted && !cancel_due && now_ms < operation.next_poll_at_ms {
            return false;
        }
        let context = self.directory_command_ctx();
        let answer = if !operation.submitted {
            let request = DocumentCheckInV1 { schema: DOCUMENT_CHECK_IN_SCHEMA_V1.to_string(), request_id: operation.request_id.clone(), head };
            client.document_check_in(&context, &operation.space_id, &request).await
        } else if cancel_due {
            client.cancel_document_check_in(&context, &operation.space_id, &operation.document_id, &operation.request_id).await
        } else {
            client.document_check_in_status(&context, &operation.space_id, &operation.document_id, &operation.request_id).await
        };
        let Some(current) = self.hub_check_in.as_mut().filter(|current| current.request_id == operation.request_id) else { return false };
        current.next_poll_at_ms = Self::directory_now_ms().saturating_add(HUB_CHECK_IN_POLL_MS);
        current.submitted = true;
        current.cancel_sent |= cancel_due;
        match answer {
            Ok(status) => current.status = Some(status),
            Err(DirectoryClientError::Unauthorized) | Err(DirectoryClientError::Http { status: 403, .. }) => {
                current.status = Some(hub_check_in_local_status(&current.request_id, DocumentCheckInPhaseV1::Failed, Some(DocumentCheckInRefusalV1::AuthorityChanged)))
            }
            Err(DirectoryClientError::Http { status, .. }) if (400..500).contains(&status) => current.status = Some(hub_check_in_local_status(&current.request_id, DocumentCheckInPhaseV1::Failed, Some(DocumentCheckInRefusalV1::Unavailable))),
            Err(_) => {}
        }
        true
    }

    /// 📌️ The footer's hub badge, followed by the current Check In's status while one is shown.
    pub(crate) fn hub_footer_label(&self) -> String {
        let is_de = self.locale_id == "de";
        let hub = shell_hub_connection_text(self.hub_connection_state(), is_de);
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(status) = self.hub_check_in.as_ref().and_then(|operation| operation.status.as_ref()) {
            return format!("{hub} · {}", hub_check_in_status_text(status, is_de));
        }
        hub
    }
    //#endregion 🔖️CheckIn

    /// 🔗️ The sync card's manual attach: parses the card's uri into bindings and opens the
    /// session's own document through the ONE [`Self::open_document`] body, so the card and the
    /// `os.open-artifact` relay cannot drift into two attach sequences (the React shell's
    /// `openDocument` is the TS twin of that body).
    ///
    /// 🌐️ A `remote://` uri names the hub document itself — host, space AND document
    /// ([`parse_remote_backbone_uri`]) — and binds the session app's own canonical surface, so two
    /// shells attaching the same uri open the same hub document (the React shell's C1c fix).
    ///
    /// 🪪️ A folder or file attach addresses the program's OWN document identity — the id its store stamps on every
    /// envelope — so the actor admits the program's edits as its own document's instead of refusing each batch as another
    /// document's (`local.backbone-scope-mismatch`), React's `openSyncTarget`. A program holding no document is told so.
    /// A folder attach is remembered for this device (`os.config.local-folders`) and, natively, the folder's archive is
    /// restored into the program like a fresh load.
    pub(crate) async fn attach_sync_backbone(&mut self, uri: String) -> Result<(), String> {
        let session = self.session.clone().ok_or("session missing")?;
        let (document_id, bindings, surface) = match parse_remote_backbone_uri(&uri) {
            Some(remote) => {
                let surface = semio_framework::manifest::surface_app_id(&session.app.dialect, session.app.role);
                (remote.document_id, vec![PersistenceBinding::Hub { base_url: format!("http://{}", remote.host_port), space_id: remote.space_id, surface: Some(surface.clone()) }], Some(surface))
            }
            None => {
                let Some(identity) = self.sync_program_identity() else {
                    let text = local_folders::local_folder_text(local_folders::LocalFolderText::Unidentified, "", self.active_locale());
                    self.show_transient_notice(text, semio_framework::Severity::Warning, Some(local_folders::SYNC_DOCUMENT_UNIDENTIFIED_CODE));
                    return Ok(());
                };
                #[cfg(target_arch = "wasm32")]
                if !SHELL_DOCUMENT_TRANSPORTS.folder && local_folders::is_host_route_uri(&uri) {
                    return self.attach_host_route_backbone(uri, identity).await;
                }
                (identity.document_id, Self::parse_persistence_binding(&uri)?, None)
            }
        };
        let folder = bindings.iter().find_map(|binding| match binding {
            PersistenceBinding::Folder { path } => Some(path.display().to_string()),
            PersistenceBinding::Hub { .. } => None,
        });
        self.open_document(document_id.clone(), session.app.io.artifact_schema.clone(), bindings, surface, Some(uri)).await?;
        let Some(path) = folder else { return Ok(()) };
        self.ensure_local_folder_bindings();
        self.remember_local_folder(semio_framework_os_config::opening_config::mutations::LocalFolderBinding {
            document_id: document_id.clone(),
            plugin_id: session.plugin_id.clone(),
            app_id: session.app.id.clone(),
            folder: semio_framework_os_config::opening_config::mutations::LocalFolderRef::Path { path: path.clone() },
        });
        #[cfg(not(target_arch = "wasm32"))]
        self.restore_folder_archive(&path, &document_id).await?;
        Ok(())
    }

    /// 📇️ ticket §2/§C6 — opens an explicit document id/schema on the same `framework/sync`
    /// `ArtifactHost` sequence `attach_sync_backbone` uses, but with caller-supplied bindings (task
    /// 2's default-binding computation, or an explicit override) instead of parsing a manual sync-
    /// card uri — independent of `attach_sync_backbone`, which stays the untouched `remote://`
    /// override path. Used by the `os.open-artifact{documentId}` opening relay (§4), by the sync
    /// card's manual attach (`backbone_uri` names what the card shows) and by the identity-driven
    /// space-index auto-bind on the `/spaces/{id}` route (§6). One body on both targets: the only
    /// platform split is [`document_bindings_admitted`], refused before any actor exists.
    ///
    /// 🪢️ A hub document's guest opens on the hub's canonical checkpoint pair ([`seed_hub_document`]) before its
    /// document actor exists, and the actor starts at the same pair ([`Self::install_document_seed`]), so every
    /// mutation it authors names the document it opened and its first Hello asks the hub only for what followed the
    /// pair (ticket 26/09/23: slice WG8 gate run 12 measured the identity-less guest; slice WG10 made the pair the
    /// one seed of every shell).
    pub(crate) async fn open_document(&mut self, document_id: String, schema: String, bindings: Vec<PersistenceBinding>, surface: Option<String>, backbone_uri: Option<String>) -> Result<(), String> {
        let prepared = self.prepare_document_open(document_id, schema, bindings, surface, backbone_uri, None).await?;
        let pair = seed_hub_document(prepared.plugin.clone(), prepared.session.instance_id, prepared.seed.clone()).await.map_err(|refusal| self.note_refused_open(refusal))?;
        self.install_document_seed(&prepared, pair)?;
        self.finish_document_open(prepared).await?;
        self.refresh_ui(UiDirtyScope::Full).await
    }

    /// 🪢️ Hands the document's actor the canonical pair its guest was seeded from; nothing for a document bound to
    /// no hub.
    pub(crate) fn install_document_seed(&self, prepared: &ShellPreparedDocumentOpen, pair: Option<semio_framework_os_kernel::os_directory::CanonicalCheckpointPairV1>) -> Result<(), String> {
        let (Some(pair), Some(seed)) = (pair, prepared.seed.as_ref()) else { return Ok(()) };
        if !self.document_host.set_document_seed(&ArtifactDocumentKey::hub(seed.scope.space_id.clone(), seed.scope.document_id.clone()), pair) {
            return Err("the canonical checkpoint pair could not be handed to the document actor".into());
        }
        Ok(())
    }

    /// 🧷️ The host half of an open, before the guest holds the document: the mounted document is
    /// checkpointed and retired, the socket surface and the execution target bound.
    ///
    /// 🎠️ H3-wgpu-native — `wasm_runtime()`/`register_host_backbone` retired, see
    /// `detach_sync_backbone_internal`'s note.
    /// 📌️ ticket §C5 item 4 — checkpoint-on-close, same "switch away IS close" posture as
    /// `attach_sync_backbone` above (this shell keeps exactly one document mounted at a time).
    pub(crate) async fn prepare_document_open(
        &mut self,
        document_id: String,
        schema: String,
        bindings: Vec<PersistenceBinding>,
        surface: Option<String>,
        backbone_uri: Option<String>,
        checkpoint: Option<semio_framework_os_kernel::os_directory::DocumentOpenCheckpointV1>,
    ) -> Result<ShellPreparedDocumentOpen, String> {
        document_bindings_admitted(SHELL_DOCUMENT_TRANSPORTS, &bindings)?;
        let session = self.session.clone().ok_or("session missing")?;
        let plugin = self.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).cloned().ok_or("plugin missing")?;
        let window_id = self.active_window_id.as_deref().or(session.view_state.window_id.as_deref()).unwrap_or_else(|| session.app.window_kinds.first().id.as_str());
        let window_kind_id = self.live_window_kind_id(&session, window_id).unwrap_or_else(|| session.app.window_kinds.first().id.as_str()).to_string();
        self.checkpoint_before_detach().await;
        self.detach_sync_backbone_internal().await?;
        self.presence_surface = surface;
        bind_wgpu_document_socket_surface(&self.document_host, &document_id, &schema, &bindings, &plugin, &session.app, &window_kind_id)?;
        let seed = self.bind_document_execution_target(&document_id, &schema, &bindings, &plugin, &session.app, &window_kind_id, checkpoint).await?;
        Ok(ShellPreparedDocumentOpen { document_id, schema, bindings, backbone_uri, seed, plugin, session })
    }

    /// 🔗️ Binds the prepared document once the guest holds it: the document actor opens, the guest's
    /// backbone binds to it and the history projection re-seeds. The caller owes the refresh.
    pub(crate) async fn finish_document_open(&mut self, prepared: ShellPreparedDocumentOpen) -> Result<(), String> {
        let ShellPreparedDocumentOpen { document_id, schema, bindings, backbone_uri, plugin, session, .. } = prepared;
        let actor_uri = format!("actor://{document_id}");
        let actor = self.current_shell_actor(session.instance_id);
        let channels = self.document_host.open(ArtifactActorConfig { actor_identity_grant: semio_framework_value::RetainedCloneGrant {maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64},  document_id: document_id.clone(), schema, bindings, watch_external: true, actor }).await;
        let events = self.document_host.subscribe_key(&channels.document_key).await;
        let binding_generation = self.mint_sync_binding_generation()?;
        let binding_effects = match plugin.bind_document_backbone(session.instance_id, binding_generation, &actor_uri).await {
            Ok(effects) => effects,
            Err(error) => {
                let detail = self.note_refused_open(error);
                if let Err(refusal) = plugin.retire_document_backbone(session.instance_id, binding_generation, &actor_uri).await { self.note_refused_open(refusal); }
                let _ = channels.cmd_tx.send(ArtifactActorMsg::Detach);
                self.document_host.close_key(&channels.document_key);
                return Err(detail);
            }
        };
        let cmd_tx = channels.cmd_tx.clone();
        let _ = cmd_tx.send(ArtifactActorMsg::LocalMutations { envelopes: Vec::new() });
        self.sync_link = DocumentLinkStatus::Linked;
        self.sync_backbone_uri = Some(backbone_uri.unwrap_or_else(|| actor_uri.clone()));
        self.sync_channel =
            Some(ShellSyncChannel { document_id, document_key: channels.document_key, actor_uri, instance_id: session.instance_id, plugin_id: session.plugin_id.clone(), binding_generation, cmd_tx, events, connected_at_ms: chrome_now_ms() as i64 });
        let channel = self.sync_channel.as_ref().expect("new sync channel is installed");
        let remaining = match route_document_backbone_effects(&channel.actor_uri, &channel.cmd_tx, binding_effects) {
            Ok(remaining) => remaining,
            Err(error) => {
                let _ = self.detach_sync_backbone_internal().await;
                return Err(error);
            }
        };
        self.queue_host_effects(&session.app.controller_id, remaining);
        if let Some(error) = self.sync_terminal_fault.take() {
            let _ = self.detach_sync_backbone_internal().await;
            return Err(error);
        }
        self.sync_status = Some(ArtifactSyncStatus::default());
        if let Some(document_key) = self.sync_channel.as_ref().map(|channel| shell_hub_document_key(&channel.document_key)) {
            self.publish_hub_document_status(document_key, ShellHubRemoteV1::Detached);
        }
        self.sync_card_kind = None;
        self.refresh_history_snapshot().await;
        Ok(())
    }

    /// 🪪️ A hub document's execution-target lease for the exact scope and the surface this shell requests: its
    /// `checkpoint` is what the document's canonical pair must be ([`ShellHubDocumentSeed`]), and for a kind this
    /// process resolves no codec for (neither linked nor a mounted component's,
    /// [`store_sync::os_store::codec_registry::document_kind_codec`]) it is also the kind identity, bound onto the document host
    /// only when it names the package this shell mounted ([`document_execution_target_admitted`]). A hub
    /// resolution that already read the lease hands its `checkpoint` in, and a kind that resolves a codec then
    /// needs no second read. A document with no hub binding has no seed. Both targets walk this one body.
    #[allow(clippy::too_many_arguments)]
    async fn bind_document_execution_target(
        &mut self,
        document_id: &str,
        schema: &str,
        bindings: &[PersistenceBinding],
        program: &ProgramBridgeEntry,
        app: &AppDefinition,
        window_kind_id: &str,
        checkpoint: Option<semio_framework_os_kernel::os_directory::DocumentOpenCheckpointV1>,
    ) -> Result<Option<ShellHubDocumentSeed>, String> {
        let Some(space_id) = bindings.iter().find_map(|binding| match binding {
            PersistenceBinding::Hub { space_id, .. } => Some(space_id.clone()),
            PersistenceBinding::Folder { .. } => None,
        }) else {
            return Ok(None);
        };
        let client = self.directory_client.clone().ok_or("document open requires a signed-in hub session")?;
        let scope = semio_framework_os_kernel::os_directory::DocumentScope::new(space_id.as_str(), document_id);
        let codec_resolves = store_sync::os_store::codec_registry::document_kind_codec(schema).await.map_err(|error| format!("document codec registry: {error}"))?.is_some();
        let surface_id = wgpu_document_socket_surface(program, app, window_kind_id)?;
        let intent = semio_framework_os_kernel::os_directory::DocumentOpenIntentV1 {
            schema: "semio.hub.document-open-intent/v1".into(),
            version: 1,
            scope: semio_framework_os_kernel::os_directory::DocumentScope::new(space_id.as_str(), document_id),
            requested_surface_id: Some(surface_id.clone()),
            client_instance_id: format!("wgpu-shell-{}", self.shell_session_id),
        };
        let lease = client.document_execution_target_manifest(&self.directory_command_ctx(), &intent).await.map_err(|error| format!("document execution target: {error}"))?;
        let checkpoint = lease.checkpoint.clone();
        document_execution_target_admitted(&lease, &program.plugin_id, program.package_id.as_deref(), program.component_sha256.as_deref(), schema, &surface_id)?;
        #[cfg(not(target_arch="wasm32"))]
        self.cancel_inference_port();
        self.document_execution_target_lease=Some(lease.clone());
        if !codec_resolves {
            document_execution_target_admitted(&lease, &program.plugin_id, program.package_id.as_deref(), program.component_sha256.as_deref(), schema, &surface_id)?;
            if !self.document_host.set_document_execution_target_lease(&ArtifactDocumentKey::hub(space_id, document_id), lease) {
                return Err("document execution target is already bound for this document".into());
            }
        }
        Ok(Some(ShellHubDocumentSeed { client, ctx: self.directory_ctx(), scope, checkpoint }))
    }

    /// 📇️ Default bindings for a document opened against the CURRENTLY mounted session's own
    /// dialect/role — `default_persistence_bindings`'s pure decision, fed the live identity/space/
    /// data-dir/surface this shell already holds.
    pub(crate) fn default_bindings_for_current_session(&self) -> (Vec<PersistenceBinding>, Option<String>) {
        let space_id = self.open_space_id.clone();
        let data_dir = self.identity_env.as_ref().and_then(|env| env.data_dir.as_deref());
        let surface = self.session.as_ref().filter(|_| self.identity.is_some() && space_id.is_some()).map(|session| semio_framework::manifest::surface_app_id(&session.app.dialect, session.app.role));
        let bindings = default_persistence_bindings(self.identity.as_ref(), space_id.as_deref(), data_dir, surface.as_deref());
        (bindings, surface)
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn schedule_sync_path_pick(&mut self, pick: impl std::future::Future<Output = Option<String>> + Send + 'static) {
        self.submit_shell_io_future(async move {
            let Some(path) = pick.await else { return ShellIoCompletion::Finished };
            ShellIoCompletion::Actions(vec![ActionDescriptor { controller_id: "framework.sync".into(), action: "setSyncDraft".into(), args: Some(semio_framework::dsl_value!({ "path": path })) }])
        });
    }

    /// 📌️ ticket §C5 item 4 — the one EXPLICIT close (a user-initiated "Detach"), same
    /// checkpoint-before-detach guard as the two switch-triggered closes above.
    pub(crate) fn handle_sync_action<'a>(&'a mut self, action: ActionDescriptor) -> ShellTurn<'a, Result<(), String>> {

        shell_turn(async move {
            match action.action.as_str() {
                "selectFile" => {
                    self.sync_card_kind = Some("file".into());
                    self.sync_card_draft = self.sync_backbone_uri.as_deref().filter(|uri| uri.starts_with("file://")).map(|uri| uri.trim_start_matches("file://").to_string()).unwrap_or_default();
                    #[cfg(not(target_arch = "wasm32"))]
                    self.schedule_sync_path_pick(pick_file_path());
                    #[cfg(target_arch = "wasm32")]
                    if let Some(path) = pick_file_path().await {
                        self.sync_card_draft = path;
                    }
                    Ok(())
                }
                "selectFolder" => {
                    self.sync_card_kind = Some("folder".into());
                    self.sync_card_draft = self.sync_backbone_uri.as_deref().filter(|uri| uri.starts_with("folder://")).map(|uri| uri.trim_start_matches("folder://").to_string()).unwrap_or_default();
                    #[cfg(not(target_arch = "wasm32"))]
                    self.schedule_sync_path_pick(pick_folder());
                    #[cfg(target_arch = "wasm32")]
                    if let Some(path) = pick_folder().await {
                        self.sync_card_draft = path;
                    }
                    Ok(())
                }
                "browse" => match self.sync_card_kind.as_deref() {
                    Some("folder") => {
                        #[cfg(not(target_arch = "wasm32"))]
                        self.schedule_sync_path_pick(pick_folder());
                        #[cfg(target_arch = "wasm32")]
                        if let Some(path) = pick_folder().await {
                            self.sync_card_draft = path;
                        }
                        Ok(())
                    }
                    Some("file") => {
                        #[cfg(not(target_arch = "wasm32"))]
                        self.schedule_sync_path_pick(pick_file_path());
                        #[cfg(target_arch = "wasm32")]
                        if let Some(path) = pick_file_path().await {
                            self.sync_card_draft = path;
                        }
                        Ok(())
                    }
                    _ => Ok(()),
                },
                "setSyncDraft" => {
                    if let Some(path) = action.args.as_ref().and_then(|args| args.get("path").or_else(|| args.get("value"))).and_then(|value| value.as_str()) {
                        self.sync_card_draft = path.to_string();
                    }
                    Ok(())
                }
                "selectRemote" => {
                    self.sync_card_kind = Some("remote".into());
                    self.sync_card_draft = self.sync_backbone_uri.as_deref().filter(|uri| uri.starts_with("remote://")).map(|uri| uri.trim_start_matches("remote://").to_string()).unwrap_or_default();
                    Ok(())
                }
                "attach" => {
                    let path = action.args.as_ref().and_then(|args| args.get("path")).and_then(|value| value.as_str()).unwrap_or(self.sync_card_draft.as_str());
                    if path.trim().is_empty() {
                        return Ok(());
                    }
                    let kind = action.args.as_ref().and_then(|args| args.get("kind")).and_then(|value| value.as_str()).unwrap_or(self.sync_card_kind.as_deref().unwrap_or("file"));
                    let uri = match kind {
                        "folder" => format!("folder://{path}"),
                        "remote" => format!("remote://{path}"),
                        _ => format!("file://{path}"),
                    };
                    self.attach_sync_backbone(uri).await
                }
                "detach" => {
                    #[cfg(target_arch = "wasm32")]
                    if self.detach_host_route_folder().await {
                        return Ok(());
                    }
                    let folder_document = self.sync_backbone_uri.as_deref().filter(|uri| uri.starts_with("folder://") || uri.starts_with("file://")).and_then(|_| self.sync_channel.as_ref()).map(|channel| channel.document_id.clone());
                    self.checkpoint_before_detach().await;
                    self.detach_sync_backbone_internal().await?;
                    self.sync_backbone_uri = None;
                    self.sync_card_kind = None;
                    self.last_envelope_dsl = None;
                    if let Some(document_id) = folder_document {
                        self.ensure_local_folder_bindings();
                        self.forget_local_folder(&document_id);
                    }
                    Ok(())
                }
                local_folders::FOLDER_RECONNECT_ACTION => {
                    self.ensure_local_folder_bindings();
                    let Some(offer) = self.folder_reconnect_offer() else { return Ok(()) };
                    if let Err(error) = self.reattach_local_folder(&offer).await {
                        self.note_dispatch_fault(&error);
                    }
                    Ok(())
                }
                local_folders::FOLDER_FORGET_ACTION => {
                    self.ensure_local_folder_bindings();
                    if let Some(offer) = self.folder_reconnect_offer() {
                        self.forget_local_folder(&offer.document_id);
                    }
                    Ok(())
                }
                _ => Ok(()),
            }

        })

    }

    /** 🪪️ One action a live ENGINE SURFACE derived from raw pointer/wheel input — the world pane's
     * hover, pick, navigation note and camera report, the board's and the map's twins. Identical to
     * [`Self::dispatch_action`] except for the provenance the chrome ledger records, which is what
     * separates a gesture stream from a control press on both renderers. */
    pub async fn dispatch_gesture_action(&mut self, action: ActionDescriptor) -> Result<(), String> {
        self.dispatch_origin = SHELL_DISPATCH_ORIGIN_GESTURE;
        let outcome = self.dispatch_action(action).await;
        self.dispatch_origin = SHELL_DISPATCH_ORIGIN_USER;
        outcome
    }

    /// 🎯️ The chrome ledger's one tap: this fn is the single funnel EVERY input's action crosses, so
    /// recording here is the target's complete answer to "what did that click do"
    /// (`crate::interpreter`'s 🎯️ChromeLedger, diagnostics-gated and bounded).
    ///
    /// 🎬️ Tutorial playback remains shell-local. Recording is armed only after the plugin's shared
    /// retained `recordTutorial` route accepts and commits below.
    ///
    /// 🎬️ Deviation detection + recorder tap — every OTHER real dispatch funnels through here exactly
    /// once, before any of this function's own side effects, and skips itself while the tutorial
    /// player's own history-action replay is mid-flight (`tutorial_flush_pending_document_ops`'s
    /// `TutorialDispatchGuard`).
    ///
    /// 🔁️ An `Effect::DispatchAction` whose id is an APP COMMAND crosses as a command, never as an
    /// action — React's `makeEffectDispatchOne` picks `handleCommand` on exactly this predicate
    /// (`🛠️ShellHelpers/🟦️.tsx`, `isAppCommand`). The guest's re-arm chain is `flowEvalTick`, an
    /// app command with no action route at all, so sending it as an `ActionInvocation` faulted the
    /// whole dispatch and — through `flush_deferred_actions` — the boot
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    ///
    /// 🎨️ Backs `build_settings_theme_ui`'s theme select/reset/delete in the owned build
    /// snapshot and mirrors the value to the present-side bridge consumed by `frame()`.
    ///
    /// 🎨️ React drops the draft on every selection change (`setThemeId` dispatches
    /// `SET_UI_THEME_DRAFT: null` first) — otherwise the previous theme's unsaved
    /// edits would follow the user onto the newly picked one.
    ///
    /// 🎛️ The bottom-middle Commands panel's zero-arg rows: every os command that carries no
    /// argument fires straight through `apply_os_command`, the same entry point the ⌘️K
    /// search's `"os-command:"` redirect uses — so `os.resetDock` is reachable from the dock
    /// panel itself now, matching React's always-fireable command rows.
    ///
    /// ⚙️ The Settings General leaf's layout row — React's `setLayout` + its
    /// `os.setLayout` history note.
    ///
    /// 👁️✏️ The Default Apps leaf's per-(dialect × role) pin — `"none"` clears it, React's
    /// `clearDefault`/`setDefault` pair over one `Select`.
    ///
    /// ⌨️ Arms (or disarms) chord capture for one shortcut row — the next key event becomes
    /// that row's override instead of a shell verb.
    ///
    /// ⌨️ Drops one row's override, which restores its `SHELL_SHORTCUT_ROWS` default —
    /// React's `resetKeybindingOverride`, whose fold is a `delete` on the override map.
    ///
    /// 🎛️ A command row with arguments owns expansion only. The stable key is re-resolved
    /// against the current palette on every click so an old mode/session cannot open a stale form.
    ///
    /// 🎛️ A zero-argument tree row executes through the same resolved-command authority as
    /// staged forms. The opaque owner-qualified key is never parsed into an invocation.
    ///
    /// 📝️ Buffers one staged command argument — React's `STAGE_COMMAND_ARG`, which writes
    /// only to the buffer and dispatches nothing.
    ///
    /// 🎛️ Execute of the staged form — the one place the buffered arguments leave the buffer,
    /// React's `onExecute(expanded, effective)`.
    ///
    /// 🎛️ `apply_os_command` takes the ONE option value every os command
    /// carries (React's own single-`value` shape), so the staged form's
    /// first argument is what it receives.
    ///
    /// 📇️ The per-window Actions pane's own four verbs — React's `SET_ACTION_PANE_EXPANDED`,
    /// `STAGE_ACTION_ARG`, `RESET_ACTION_ARGS` and `onExecute` (`windowActionPaneNode`,
    /// `🛠️ShellHelpers/🟦️.tsx:4221`). Only the last one dispatches anything to the app.
    ///
    /// 🔎️ The per-window Search pane's own two local verbs — the typed line's buffer and the
    /// possibles chevron. React keeps both in the `Search` component's own state.
    ///
    /// 🔀️ One retained Projection Tree row. Both pointer Activate and Enter/Space emit
    /// this exact action, so the camera, selected row, icon, title and AX snapshot advance
    /// through one accepted per-window boundary.
    ///
    /// 💬️ The chat panel's own two verbs — React's composer `setDraft`/`submit`. There is no
    /// clear verb: the feed is live bridge state, not a local buffer a shell may discard.
    /// 🖥️ The two Display leaves' own verbs — React's `DisplayHostApi`
    /// (`📌️ChromePanels/🟦️.tsx`): the save-name box, Save current layout, apply and delete.
    ///
    /// 🖥️ React's `NamedLayoutStore::save` persists on the same call that mutates its
    /// roster — a layout saved and not written would be gone on the next boot.
    ///
    /// 🖥️ React's `NamedLayoutStore::remove` persists the shortened roster too.
    ///
    /// 🖥️ One Display "Windows" row — React drags this payload onto the dock; with no panel
    /// drag lane here the press opens the window directly, carrying the same
    /// `{windowKindId, templateId}` the drag would have.
    ///
    /// ⚔️ The Conflicts leaf's two verbs — React's `onSelect`/`onResolve`.
    ///
    /// 🛍️ The Marketplace leaf's three verbs — React's
    /// `installPlugin`/`reloadPlugin`/`uninstallPlugin`.
    ///
    /// 🔐️ Slice WG6: every `hub*` verb the hub workspace names goes through ONE lane, so
    /// this dispatch carries one arm rather than eighteen and the surface's vocabulary and
    /// the shell's cannot drift apart.
    ///
    /// ⚖️ React's `setMergePolicy` — local authority state, never wire-carried, so it moves
    /// no document and notes no history entry beyond the shell setting itself.
    ///
    /// 🎨️ The eight theme-editor verbs — React's `setThemeColor`/`setThemeSpacing`/
    /// `setThemeFontStack`/`setThemeStroke`/`setThemeRadius`/`setThemeOpacity`/
    /// `setThemeMetric`/`setThemeAppearancePaint`, each a `draftThemePatch` over the SAME
    /// document. The key rides the row's authored args (merged with the committed value by
    /// `merge_committed_args`), where React's closure captured it.
    ///
    /// 💾️ React's `saveTheme(label)`: slug the trimmed name, store the canonical document
    /// under `custom.<slug>`, select it, and drop the draft.
    ///
    /// ⬇️ React's `exportTheme`: `downloadMediaExport(`${id}.theme.dsl`, "text/plain",
    /// serializeUiTheme(theme))` — the same filename, mime and canonical 2-space JSON.
    ///
    /// ⬆️ React's `importTheme`: `requestFileOpen(".theme.dsl,.dsl,text/plain")`, parse,
    /// then save under the parsed label. The picker is user-activation gated, so it parks
    /// for the gesture lane rather than opening from this async funnel.
    ///
    /// ⬆️ The picked file's text, parsed and saved — React's
    /// `saveTheme(parsed.label || parsed.id)`. An invalid file is ignored, exactly as
    /// React's `catch {}` ignores it.
    ///
    /// 📌️ ticket §C5 item 3 — explicit check-in's own dedicated controller (`#s-checkin`'s
    /// open/cancel/submit dialog funnel), same shell-owned-chrome treatment as `framework.sync`
    /// above.
    ///
    /// 🧰️ Intercept the framework `setActiveUtility` View action to update the host-owned active-utility
    /// map before forwarding to the plugin (which reacts by clearing its live-preview scratch). The
    /// authoritative state is the shell map + the `ViewModel.active_utility_id` it injects on render.
    ///
    /// 🧾️ ticket §C5 — fold this dispatch's own `history_patch` into the check-in projection (idle
    /// clock, checkpoint-landed detection + `TouchArtifact`) before anything else touches `self`.
    ///
    /// 🎓️ Advance-by-doing: this action was actually performed (the plugin call above succeeded), so
    /// a tour step whose `advance` targets it moves on now — see `chrome_tour_note_action_performed`.
    ///
    /// 🧾️ ONE host-effect funnel, byte for byte the fold `dispatch_command` performs. This arm used
    /// to be a SECOND, shorter, hand-rolled match ending in `_ => {}`, and every effect it did not
    /// name was dropped WITHOUT A TRACE — not even the funnel's own `effect dropped` line. That is
    /// how `Export Document…` ended: the guest ran it, answered `DownloadMediaExport`, the bridge
    /// handed it back as `requested_effects`, and this silent arm threw it away, so no plugin could
    /// export anything through an ACTION on this renderer while the command path exported fine
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-end-to-end-verification-2026-09-14.md` §C).
    /// Only the two effects `queue_host_effects` cannot own stay here — a `Navigate` that must also
    /// RESOLVE the uri, and the replay relay — because both are `async` and that funnel is
    /// deliberately a plain `fn`.
    pub fn dispatch_action<'a>(&'a mut self, mut action: ActionDescriptor) -> ShellTurn<'a, Result<(), String>> {

        shell_turn(async move {
            self.chrome_build.refused_guest_fault = None;
            if let Some(owner) = action.args.as_ref().and_then(|args| args.get("windowId")).and_then(semio_framework_value::DslValue::as_str).and_then(window_pane_owner).map(str::to_string) {
                scope_action_to_window(&mut action, &owner);
            }
            crate::interpreter::note_dispatched_action(&action, self.dispatch_origin);
            if action.action == semio_framework::START_TUTORIAL_ACTION_ID {
                if let Some(tutorial_id) = action.args.as_ref().and_then(|args| args.get("tutorialId")).and_then(|v| v.as_str()) {
                    self.tutorial_start(tutorial_id);
                }
                return Ok(());
            }
            let record_tutorial_after_acceptance = action.action == semio_framework::RECORD_TUTORIAL_ACTION_ID;
            if !self.chrome_build.tutorial_dispatch_internal {
                self.tutorial_note_real_dispatch(&action);
            }
            if let Some(result) = self.apply_host_panel_selection(&action) {
                return result;
            }
            if !record_tutorial_after_acceptance && action.controller_id != "framework" {
                if let Some(session) = self.action_session(&action).cloned() {
                    if Self::app_owns_command(&session.app, &action.action) {
                        if action.args.as_ref().and_then(|args| args.get("windowId")).and_then(semio_framework_value::DslValue::as_str) == Some("spawned") {
                            if let Some(id) = session.view_state.window_id.as_deref() { scope_action_to_window(&mut action, id); }
                        }
                        let arguments = action.args.as_ref().and_then(semio_framework_value::DslValue::as_object).map(|entries| entries.iter().cloned().collect()).unwrap_or_default();
                        return self
                            .dispatch_session_command(semio_framework::manifest::CommandInvocation {
                                address: semio_framework::manifest::CommandAddress { owner: semio_framework::manifest::CommandOwnerAddress::App { plugin_id: session.plugin_id.clone(), app_id: session.app.id.clone() }, command_id: action.action.clone() },
                                arguments,
                            }, session)
                            .await;
                    }
                }
            }
            if action.controller_id == "framework" {
                match action.action.as_str() {
                    "setAppearance" => {
                        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                            self.appearance_id = value.to_string();
                            self.note_shell_setting_command("os.setAppearance", Some(value)).await?;
                            self.republish_shell_panel_document(FRAMEWORK_SETTINGS_GENERAL_TAB_ID)?;
                        }
                        return Ok(());
                    }
                    "setDriver" => {
                        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                            self.driver_id = value.to_string();
                            self.clear_driver_draft();
                            self.note_shell_setting_command("os.setDriver", Some(value)).await?;
                            self.republish_shell_panel_document(FRAMEWORK_SETTINGS_GENERAL_TAB_ID)?;
                        }
                        return Ok(());
                    }
                    "setDriverField" => {
                        if let (Some(key), Some(value)) = (action.args.as_ref().and_then(|args| args.get("key")).and_then(|value| value.as_str()), action.args.as_ref().and_then(|args| args.get("value")).and_then(|value| value.as_str())) {
                            if self.set_driver_field(key, value) {
                                self.republish_shell_panel_document(FRAMEWORK_SETTINGS_GENERAL_TAB_ID)?;
                            }
                        }
                        return Ok(());
                    }
                    "setDriverSaveLabel" => {
                        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|value| value.as_str()) {
                            if self.driver_save_label != value {
                                self.driver_save_label = value.to_string();
                                self.republish_shell_panel_document(FRAMEWORK_SETTINGS_GENERAL_TAB_ID)?;
                            }
                        }
                        return Ok(());
                    }
                    "saveDriver" => {
                        let label = self.driver_save_label.trim().to_string();
                        if let Some(id) = custom_ui_driver_id(&label) {
                            let mut driver = self.driver_document();
                            driver.driver_id = id.clone();
                            driver.label = label;
                            self.chrome_build.preferences.custom_drivers.insert(id.clone(), driver);
                            self.driver_id = id;
                            self.driver_save_label.clear();
                            self.clear_driver_draft();
                            self.republish_shell_panel_document(FRAMEWORK_SETTINGS_GENERAL_TAB_ID)?;
                        }
                        return Ok(());
                    }
                    "deleteDriver" => {
                        if let Some(id) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|value| value.as_str()).filter(|id| id.starts_with("custom.")) {
                            let changed = self.chrome_build.preferences.custom_drivers.remove(id).is_some() || self.driver_id == id || self.driver_draft.is_some();
                            if self.driver_id == id {
                                self.driver_id = "default".into();
                            }
                            self.clear_driver_draft();
                            if changed {
                                self.republish_shell_panel_document(FRAMEWORK_SETTINGS_GENERAL_TAB_ID)?;
                            }
                        }
                        return Ok(());
                    }
                    "setLocale" => {
                        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                            let changed = self.locale_id != value;
                            self.locale_id = value.to_string();
                            crate::interpreter::install_ui_engine_locale(self.active_locale());
                            if let Some(status) = self.plugin_fault_status() {
                                self.error = Some(status);
                            }
                            self.trace_resolved_locale("setLocale");
                            self.note_shell_setting_command("os.setLocale", Some(value)).await?;
                            if changed {
                                self.sync_dock_tabs();
                                self.owe_refresh(UiDirtyScope::Full);
                                self.owe_settle();
                            }
                        }
                        return Ok(());
                    }
                    "setTerminology" => {
                        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                            let changed = self.terminology_id != value;
                            self.terminology_id = value.to_string();
                            self.note_shell_setting_command("os.setTerminology", Some(value)).await?;
                            if changed {
                                self.sync_dock_tabs();
                                self.owe_refresh(UiDirtyScope::Full);
                                self.owe_settle();
                            }
                        }
                        return Ok(());
                    }
                    "setThemeId" => {
                        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                            let changed = self.chrome_build.preferences.theme_id != value || self.theme_draft.is_some();
                            set_active_theme_id(value);
                            self.clear_theme_draft();
                            self.chrome_build.preferences.theme_id = value.to_string();
                            self.note_shell_setting_command("os.setThemeId", Some(value)).await?;
                            if changed {
                                self.republish_shell_panel_document(FRAMEWORK_SETTINGS_THEME_TAB_ID)?;
                            }
                        }
                        return Ok(());
                    }
                    "runOsCommand" => {
                        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                            let command_id = value.to_string();
                            self.apply_os_command(&command_id, None).await?;
                        }
                        return Ok(());
                    }
                    "setLayout" => {
                        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                            self.chrome_build.preferences.ui_layout = if value == "tablet" { "tablet".to_string() } else { "desktop".to_string() };
                            self.note_shell_setting_command("os.setLayout", Some(value)).await?;
                            self.republish_shell_panel_document(FRAMEWORK_SETTINGS_GENERAL_TAB_ID)?;
                        }
                        return Ok(());
                    }
                    "setDefaultApp" => {
                        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                            let key = action.args.as_ref().and_then(|args| args.get("row")).and_then(|v| v.as_str()).unwrap_or_default().to_string();
                            if value == DEFAULT_APP_NONE_VALUE {
                                self.default_app_pins.remove(&key);
                            } else {
                                self.default_app_pins.insert(key, value.to_string());
                            }
                        }
                        return Ok(());
                    }
                    "captureKeybinding" => {
                        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                            let already = self.keybinding_capture_control_id.as_deref() == Some(value);
                            self.keybinding_capture_control_id = (!already).then(|| value.to_string());
                        }
                        return Ok(());
                    }
                    "resetKeybinding" => {
                        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                            self.chrome_build.preferences.keybinding_overrides.remove(value);
                        }
                        return Ok(());
                    }
                    "setCommandExpanded" => {
                        if let Some(key) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|value| value.as_str()) {
                            let live = self.resolved_commands().into_iter().any(|entry| entry.definition.in_palette && !entry.definition.args.is_empty() && command_address_stable_key(&entry.address) == key);
                            if live {
                                self.expanded_command_id = (self.expanded_command_id.as_deref() != Some(key)).then(|| key.to_string());
                            }
                        }
                        return Ok(());
                    }
                    "executeResolvedCommand" => {
                        if let Some(key) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|value| value.as_str()).map(ToOwned::to_owned) {
                            if let Some(entry) = self.resolved_commands().into_iter().find(|entry| entry.definition.in_palette && entry.definition.args.is_empty() && command_address_stable_key(&entry.address) == key) {
                                match &entry.address.owner {
                                    semio_framework::manifest::CommandOwnerAddress::Os => {
                                        self.apply_os_command(&entry.definition.id, None).await?;
                                    }
                                    _ => {
                                        self.dispatch_command(semio_framework::manifest::CommandInvocation { address: entry.address, arguments: Default::default() }).await?;
                                    }
                                }
                            }
                        }
                        return Ok(());
                    }
                    "stageCommandArg" => {
                        let args = action.args.as_ref().map(dsl_value_as_json).unwrap_or(Value::Null);
                        if let (Some(command), Some(arg)) = (args.get("command").and_then(Value::as_str), args.get("arg").and_then(Value::as_str)) {
                            let staged = self.staged_command_args.entry(command.to_string()).or_default();
                            let value = staged_arg_value(&args, staged.get(arg));
                            staged.insert(arg.to_string(), value);
                        }
                        return Ok(());
                    }
                    "resetStagedCommand" => {
                        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                            self.staged_command_args.remove(value);
                        }
                        return Ok(());
                    }
                    "executeStagedCommand" => {
                        if let Some(key) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()).map(ToOwned::to_owned) {
                            let staged = self.staged_command_args.get(&key).cloned().unwrap_or_default();
                            if let Some(entry) = self.resolved_commands().into_iter().find(|entry| entry.definition.in_palette && command_address_stable_key(&entry.address) == key) {
                                let Some(effective) = Self::resolved_execute_args(&entry.definition.args, &staged) else { return Ok(()) };
                                match &entry.address.owner {
                                    semio_framework::manifest::CommandOwnerAddress::Os => {
                                        let command_id = entry.definition.id.clone();
                                        let option = entry.definition.args.first().and_then(|arg| effective.get(&arg.id)).and_then(|value| value.as_str().map(ToOwned::to_owned));
                                        self.apply_os_command(&command_id, option.as_deref()).await?;
                                    }
                                    _ => {
                                        let arguments = effective.into_iter().filter_map(|(id, value)| serde_json::from_value::<DslValue>(value).ok().map(|value| (id, value))).collect();
                                        self.dispatch_command(semio_framework::manifest::CommandInvocation { address: entry.address.clone(), arguments }).await?;
                                    }
                                }
                            }
                            self.staged_command_args.remove(&key);
                            self.expanded_command_id = None;
                        }
                        return Ok(());
                    }
                    "setActionExpanded" => {
                        let args = action.args.as_ref().map(dsl_value_as_json).unwrap_or(Value::Null);
                        if let (Some(window_id), Some(action_id)) = (args.get("window").and_then(Value::as_str), args.get("action").and_then(Value::as_str)) {
                            if self.action_panel_expanded.get(window_id).map(String::as_str) == Some(action_id) {
                                self.action_panel_expanded.remove(window_id);
                            } else {
                                self.action_panel_expanded.insert(window_id.to_string(), action_id.to_string());
                            }
                        }
                        return self.republish_window_action_panes();
                    }
                    IMPORT_TRANSFER_CANCEL_ACTION => {
                        if let Some(transfer) = action.args.as_ref().map(dsl_value_as_json).and_then(|args| args.get("transfer").and_then(Value::as_u64)) {
                            self.cancel_import_transfer(transfer);
                        }
                        return Ok(());
                    }
                    "stageActionArg" => {
                        let args = action.args.as_ref().map(dsl_value_as_json).unwrap_or(Value::Null);
                        if let (Some(window_id), Some(action_id), Some(arg_id)) = (args.get("window").and_then(Value::as_str), args.get("action").and_then(Value::as_str), args.get("arg").and_then(Value::as_str)) {
                            let (window_id, action_id, arg_id) = (window_id.to_string(), action_id.to_string(), arg_id.to_string());
                            let value = staged_arg_value(&args, self.staged_map_for(&window_id, &action_id).get(&arg_id));
                            self.stage_arg(&window_id, &action_id, &arg_id, value);
                        }
                        return self.republish_window_action_panes();
                    }
                    "resetActionArgs" => {
                        let args = action.args.as_ref().map(dsl_value_as_json).unwrap_or(Value::Null);
                        if let (Some(window_id), Some(action_id)) = (args.get("window").and_then(Value::as_str), args.get("action").and_then(Value::as_str)) {
                            let (window_id, action_id) = (window_id.to_string(), action_id.to_string());
                            self.reset_staged_args(&window_id, &action_id);
                        }
                        return self.republish_window_action_panes();
                    }
                    "executeStagedAction" => {
                        let args = action.args.as_ref().map(dsl_value_as_json).unwrap_or(Value::Null);
                        if let (Some(window_id), Some(action_id)) = (args.get("window").and_then(Value::as_str), args.get("action").and_then(Value::as_str)) {
                            let (window_id, action_id) = (window_id.to_string(), action_id.to_string());
                            self.execute_staged_action(&window_id, &action_id).await?;
                        }
                        return Ok(());
                    }
                    "setEngagementInput" => {
                        let args = action.args.as_ref().map(dsl_value_as_json).unwrap_or(Value::Null);
                        if let Some(window_id) = args.get("window").and_then(Value::as_str) {
                            let value = args.get("value").and_then(Value::as_str).unwrap_or_default().to_string();
                            self.engagement_inputs.insert(window_id.to_string(), value);
                        }
                        return Ok(());
                    }
                    "toggleSearchPossibles" => {
                        let args = action.args.as_ref().map(dsl_value_as_json).unwrap_or(Value::Null);
                        if let Some(window_id) = args.get("window").and_then(Value::as_str) {
                            let open = self.search_possibles_open.get(window_id).copied().unwrap_or(false);
                            self.search_possibles_open.insert(window_id.to_string(), !open);
                        }
                        return self.republish_window_action_panes();
                    }
                    "setWorldProjectionTemplate" => {
                        let args = action.args.as_ref().map(dsl_value_as_json).unwrap_or(Value::Null);
                        if let (Some(window_id), Some(template_id)) = (args.get("windowId").and_then(Value::as_str), args.get("templateId").and_then(Value::as_str)) {
                            let (window_id, template_id) = (window_id.to_string(), template_id.to_string());
                            self.select_world_projection_template(&window_id, &template_id)?;
                        }
                        return Ok(());
                    }
                    "setLayoutSaveLabel" => {
                        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                            self.layout_save_label = value.to_string();
                        }
                        return Ok(());
                    }
                    "saveCurrentLayout" => {
                        let label = self.layout_save_label.trim().to_string();
                        if label.is_empty() {
                            return Ok(());
                        }
                        if let Some(layout) = self.current_named_layout(&label) {
                            let id = layout.id.clone();
                            self.user_layouts.retain(|entry| entry.id != id);
                            self.user_layouts.push(layout);
                            self.layout_save_label.clear();
                            self.persist_named_layouts();
                            self.note_shell_setting_command("os.saveNamedLayout", Some(&id)).await?;
                        }
                        return Ok(());
                    }
                    "applyNamedLayout" => {
                        if let Some(layout_id) = action.args.as_ref().and_then(|args| args.get("layoutId")).and_then(|v| v.as_str()).map(ToOwned::to_owned) {
                            if self.apply_named_layout(&layout_id) {
                                self.note_shell_setting_command("os.applyNamedLayout", Some(&layout_id)).await?;
                            }
                        }
                        return Ok(());
                    }
                    "deleteUserLayout" => {
                        if let Some(layout_id) = action.args.as_ref().and_then(|args| args.get("layoutId")).and_then(|v| v.as_str()).map(ToOwned::to_owned) {
                            self.user_layouts.retain(|entry| entry.id != layout_id);
                            self.persist_named_layouts();
                            self.note_shell_setting_command("os.deleteNamedLayout", Some(&layout_id)).await?;
                        }
                        return Ok(());
                    }
                    "openDisplayWindow" => {
                        let window_kind_id = action.args.as_ref().and_then(|args| args.get("windowKindId")).and_then(|v| v.as_str()).map(ToOwned::to_owned);
                        let template_id = action.args.as_ref().and_then(|args| args.get("templateId")).and_then(|v| v.as_str()).map(ToOwned::to_owned);
                        if let Some(window_kind_id) = window_kind_id {
                            self.open_display_window(&window_kind_id, template_id.as_deref());
                        }
                        return Ok(());
                    }
                    "selectConflict" => {
                        if let Some(conflict_id) = action.args.as_ref().and_then(|args| args.get("conflictId")).and_then(|v| v.as_str()) {
                            self.selected_conflict_id = (self.selected_conflict_id.as_deref() != Some(conflict_id)).then(|| conflict_id.to_string());
                        }
                        return Ok(());
                    }
                    "resolveConflict" => {
                        let conflict_id = action.args.as_ref().and_then(|args| args.get("conflictId")).and_then(|v| v.as_str()).map(ToOwned::to_owned);
                        let accept = action.args.as_ref().and_then(|args| args.get("accept")).and_then(semio_framework_value::DslValue::as_bool).unwrap_or(true);
                        if let Some(conflict_id) = conflict_id {
                            self.resolve_open_conflict(&conflict_id, accept).await;
                        }
                        return Ok(());
                    }
                    "installPlugin" | "reloadPlugin" => {
                        if let Some(plugin_id) = action.args.as_ref().and_then(|args| args.get("pluginId")).and_then(|v| v.as_str()).map(ToOwned::to_owned) {
                            if action.action == "reloadPlugin" {
                                self.uninstall_plugin(&plugin_id);
                            }
                            let _ = self.install_plugin(&plugin_id).await;
                            self.note_shell_setting_command(if action.action == "reloadPlugin" { "os.reloadPlugin" } else { "os.installPlugin" }, Some(&plugin_id)).await?;
                        }
                        return Ok(());
                    }
                    "uninstallPlugin" => {
                        if let Some(plugin_id) = action.args.as_ref().and_then(|args| args.get("pluginId")).and_then(|v| v.as_str()).map(ToOwned::to_owned) {
                            if self.uninstall_plugin(&plugin_id) {
                                self.note_shell_setting_command("os.uninstallPlugin", Some(&plugin_id)).await?;
                            }
                        }
                        return Ok(());
                    }
                    "installExtensionUrl" => {
                        let url = action.args.as_ref().and_then(|args| args.get("url")).and_then(|value| value.as_str()).filter(|url| !url.is_empty()).map(ToOwned::to_owned);
                        let mut request = serde_json::json!({
                            "op": "extension-store-install-url",
                            "prompt": shell_chrome_string("plugins.extension.urlPrompt", self.locale_id == "de")
                        });
                        if let Some(url) = url.as_deref() {
                            request["url"] = serde_json::Value::String(url.to_string());
                        }
                        if self.install_extension_from_store(request).await? {
                            self.note_shell_setting_command("os.installExtensionUrl", url.as_deref()).await?;
                        }
                        return Ok(());
                    }
                    "installExtensionFile" => {
                        if self.install_extension_from_store(serde_json::json!({ "op": "extension-store-install-file" })).await? {
                            self.note_shell_setting_command("os.installExtensionFile", None).await?;
                        }
                        return Ok(());
                    }
                    "setExtensionEnabled" => {
                        let extension_id = action.args.as_ref().and_then(|args| args.get("extensionId")).and_then(|value| value.as_str()).map(ToOwned::to_owned);
                        let enabled = action.args.as_ref().and_then(|args| args.get("enabled")).and_then(semio_framework_value::DslValue::as_bool).unwrap_or(true);
                        if let Some(extension_id) = extension_id {
                            if self.set_extension_enabled(&extension_id, enabled).await? {
                                self.note_shell_setting_command(if enabled { "os.enableExtension" } else { "os.disableExtension" }, Some(&extension_id)).await?;
                            }
                        }
                        return Ok(());
                    }
                    "uninstallExtension" => {
                        if let Some(extension_id) = action.args.as_ref().and_then(|args| args.get("extensionId")).and_then(|value| value.as_str()).map(ToOwned::to_owned) {
                            if self.uninstall_extension(&extension_id).await? {
                                self.note_shell_setting_command("os.uninstallExtension", Some(&extension_id)).await?;
                            }
                        }
                        return Ok(());
                    }
                    "setChatDraft" => {
                        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                            self.agent_chat_draft = value.to_string();
                        }
                        return Ok(());
                    }
                    "sendChatDraft" => {
                        self.send_agent_chat_draft();
                        return Ok(());
                    }
                    "cancelToolCall" => {
                        if let Some(invocation_id) = action.args.as_ref().and_then(|args| args.get("invocationId")).and_then(|value| value.as_str()).map(ToOwned::to_owned) {
                            self.cancel_agent_tool_call(&invocation_id);
                        }
                        return Ok(());
                    }
                    other if other.starts_with("hub") => {
                        self.handle_hub_workspace_action(&action.action.clone(), action.args.clone()).await;
                        return Ok(());
                    }
                    "setMergePolicy" => {
                        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                            if SHELL_MERGE_POLICY_OPTIONS.iter().any(|(option, _)| *option == value) {
                                self.merge_policy = value.to_string();
                                self.note_shell_setting_command("os.setMergePolicy", Some(value)).await?;
                                self.republish_shell_panel_document(FRAMEWORK_SETTINGS_GENERAL_TAB_ID)?;
                            }
                        }
                        return Ok(());
                    }
                    "setThemeColor" | "setThemeSpacing" | "setThemeFontStack" | "setThemeStroke" | "setThemeRadius" | "setThemeOpacity" | "setThemeMetric" | "setThemeAppearancePaint" => {
                        let args = action.args.as_ref().map(dsl_value_as_json).unwrap_or(Value::Null);
                        let verb = action.action.clone();
                        let before = self.theme_document();
                        self.apply_theme_editor_commit(&verb, &args);
                        if self.theme_document() != before {
                            self.republish_shell_panel_document(FRAMEWORK_SETTINGS_THEME_TAB_ID)?;
                        }
                        return Ok(());
                    }
                    "setThemeSaveLabel" => {
                        let value = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()).unwrap_or_default().to_string();
                        if self.theme_save_label != value {
                            self.theme_save_label = value;
                            self.republish_shell_panel_document(FRAMEWORK_SETTINGS_THEME_TAB_ID)?;
                        }
                        return Ok(());
                    }
                    "saveTheme" => {
                        let label = self.theme_save_label.trim().to_string();
                        if let Some(id) = custom_theme_id_for_label(&label) {
                            let mut document = self.theme_document();
                            document.id = id.clone();
                            document.label = label;
                            if let Some(raw) = save_custom_theme_document(&id, &document) {
                                self.chrome_build.preferences.custom_themes.insert(id.clone(), raw);
                                self.chrome_build.preferences.theme_id = id.clone();
                                self.theme_draft = None;
                                self.theme_save_label.clear();
                                self.note_shell_setting_command("os.setThemeId", Some(id.as_str())).await?;
                                self.republish_shell_panel_document(FRAMEWORK_SETTINGS_THEME_TAB_ID)?;
                            }
                        }
                        return Ok(());
                    }
                    "exportTheme" => {
                        let document = self.theme_document();
                        if let Some(text) = document.canonical_json() {
                            download_media_export(&format!("{}.theme.dsl", document.id), "text/plain", &text, None);
                        }
                        return Ok(());
                    }
                    "importTheme" => {
                        #[cfg(target_arch = "wasm32")]
                        {
                            self.pending_theme_import = true;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        self.submit_shell_io_future(async move {
                            let opened = request_file_open(SHELL_THEME_IMPORT_ACCEPT, Some("text"), false).await;
                            ShellIoCompletion::Actions(
                                opened.into_iter().take(1).map(|file| ActionDescriptor { controller_id: "framework".into(), action: "applyImportedTheme".into(), args: crate::action_args_json!({ "value": file.contents }) }).collect(),
                            )
                        });
                        return Ok(());
                    }
                    "applyImportedTheme" => {
                        if let Some(text) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                            if let Some(document) = ThemeDocument::parse(text) {
                                let label = if document.label.is_empty() { document.id.clone() } else { document.label.clone() };
                                if let Some(id) = custom_theme_id_for_label(&label) {
                                    let saved = ThemeDocument { id: id.clone(), label, ..document };
                                    if let Some(raw) = save_custom_theme_document(&id, &saved) {
                                        self.chrome_build.preferences.custom_themes.insert(id.clone(), raw);
                                        self.chrome_build.preferences.theme_id = id.clone();
                                        self.theme_draft = None;
                                        self.note_shell_setting_command("os.setThemeId", Some(id.as_str())).await?;
                                        self.republish_shell_panel_document(FRAMEWORK_SETTINGS_THEME_TAB_ID)?;
                                    }
                                }
                            }
                        }
                        return Ok(());
                    }
                    "resetThemeId" => {
                        let changed = self.chrome_build.preferences.theme_id != "semio" || self.theme_draft.is_some();
                        set_active_theme_id("semio");
                        self.clear_theme_draft();
                        self.chrome_build.preferences.theme_id = "semio".to_string();
                        self.note_shell_setting_command("os.resetThemeId", Some("semio")).await?;
                        if changed {
                            self.republish_shell_panel_document(FRAMEWORK_SETTINGS_THEME_TAB_ID)?;
                        }
                        return Ok(());
                    }
                    "deleteThemeId" => {
                        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                            delete_custom_theme(value);
                            let removed = self.chrome_build.preferences.custom_themes.remove(value).is_some();
                            let selected = self.chrome_build.preferences.theme_id == value;
                            if selected {
                                self.chrome_build.preferences.theme_id = "semio".to_string();
                            }
                            self.note_shell_setting_command("os.deleteThemeId", Some(value)).await?;
                            if removed || selected {
                                self.republish_shell_panel_document(FRAMEWORK_SETTINGS_THEME_TAB_ID)?;
                            }
                        }
                        return Ok(());
                    }
                    _ => {}
                }
            }
            if action.controller_id == "framework.sync" {
                self.handle_sync_action(action).await?;
                return self.republish_shell_panel_document(FRAMEWORK_SYNC_PANEL_TAB_ID);
            }
            if action.controller_id == "framework.checkin" {
                return self.handle_checkin_action(action).await;
            }
            if action.action == semio_framework::SET_ACTIVE_UTILITY_ACTION_ID {
                if let Some(session) = self.session.clone() {
                    if action.controller_id == session.app.controller_id {
                        if let Some(utility_id) = action.args.as_ref().and_then(|args| args.get("utilityId")).and_then(|value| value.as_str()) {
                            let window_id = action
                                .args
                                .as_ref()
                                .and_then(|args| args.get("windowId"))
                                .and_then(|value| value.as_str())
                                .map(String::from)
                                .or_else(|| self.active_window_id.clone())
                                .unwrap_or_else(|| self.active_utility_bar_window_kind(&session).id.clone());
                            self.toggle_active_utility(&window_id, utility_id);
                        }
                    }
                }
            }
            let Some(session) = self.action_session(&action).cloned() else {
                return Err("action has no mounted app owner".into());
            };
            let program = self.plugins.iter().find(|p| p.plugin_id == session.plugin_id).ok_or("action program missing")?;
            let live_view_state = self.live_view_state(&session);
            let invocation = self.session_action_invocation(&session, &action, &live_view_state)?;
            let action_json = semio_framework_pack_json::to_json_string(&invocation);
            let answer = program.handle_action(session.instance_id, &action_json, &live_view_state).await;
            let mut result = answer.map_err(|refusal| self.refused_guest_call(refusal, &session.app))?;
            self.observe_invocation_history(result.history_patch.as_ref()).await;
            self.observe_interaction_output(&result.output);
            if record_tutorial_after_acceptance {
                self.tutorial_start_recording();
            }
            self.chrome_tour_note_action_performed(&action.action);
            let mut queued = Vec::with_capacity(result.requested_effects.len());
            for effect in core::mem::take(&mut result.requested_effects) {
                match effect {
                    semio_framework::kernel::Effect::Navigate { uri } => {
                        self.push_uri(uri.clone());
                        if let Err(error) = self.apply_shell_uri(&uri).await {
                            Self::debug_log(&format!("[TRACE] wgpu shell navigate effect failed: {error}"));
                        }
                    }
                    semio_framework::kernel::Effect::ReplayShellCommand { action_id, args } => {
                        self.handle_replay_shell_command(&action_id, args.as_ref()).await;
                    }
                    other => queued.push(other),
                }
            }
            let scope = result.ui_scope.clone().merged_with(self.host_effect_earned_scope(&queued));
            Self::debug_log(&format!("[TRACE] wgpu-shell dispatch action={} scope={}", action.action, refresh_scope_label(&scope)));
            self.queue_host_effects(&action.controller_id.clone(), queued);
            let operations: Vec<String> = result.mutations.iter().filter_map(|operation| serde_json::to_string(&operation.diff.payload).ok()).collect();
            self.apply_ops_inner(&operations, true, scope, Some(session)).await

        })

    }
}
//#endregion ShellActions
