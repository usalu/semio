//! 🖐 `shell-pointer` staging of `shell-actions`: RENDERER-WP5, the part of the former chunk that plan Appendix B and C, the WP4 re-homing by the call graph and the WP5 size spill assign to this crate.

use super::*;

//#region ShellActions
//#region 🔖️NativeBackboneSync
impl<'ctx> PointerCtx<'ctx> {
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
        let shell_io_changed = self.as_session().poll_shell_io();
        #[cfg(target_arch = "wasm32")]
        let shell_io_changed = false;
        let directory_changed = self.as_actions().pump_directory_events().await || shell_io_changed;
        #[cfg(not(target_arch = "wasm32"))]
        let directory_changed = self.as_session().poll_time_travel_progress().await || directory_changed;
        let directory_changed = self.as_actions().drain_operation_publications().await || directory_changed;
        let directory_changed = self.directory.publish_peer_time_travel_notes(&self.chrome, &self.history, &self.prefs) || directory_changed;
        let directory_changed = self.as_actions().reattach_remembered_local_folder().await || directory_changed;
        #[cfg(target_arch = "wasm32")]
        let directory_changed = self.as_session().flush_host_route_folder().await || directory_changed;
        self.as_actions().poll_auto_checkin().await;
        let directory_changed = self.as_actions().advance_document_opening().await || directory_changed;
        let directory_changed = self.as_session().advance_sync_reseed().await || directory_changed;
        if let Some(error) = self.sync.sync_terminal_fault.take() {
            debug_log(&format!("[TRACE] wgpu shell retired document-backbone owner after terminal fault: {error}"));
            let _ = self.as_actions().detach_sync_backbone_internal().await;
            self.sync.sync_card_kind = Some("conflict".into());
            self.dock.relabel_sync_tab(&self.prefs, &self.sync);
            return true;
        }
        let (owner, events) = {
            let Some(channel) = self.sync.sync_channel.as_mut() else {
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
        let plugin = self.plugins.plugins.iter().find(|entry| entry.plugin_id == owner.plugin_id).cloned();
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
                            Err(error) => { let detail = self.prefs.note_refused_open(error, &mut self.chrome); debug_log(&format!("[TRACE] wgpu shell apply_mutations failed: {detail}")); },
                        }
                    }
                }
                ArtifactEvent::DocumentArchiveReplaced { archive } => {
                    self.sync.sync_bootstrap_progress = None;
                    if let Some(plugin) = plugin.as_ref() {
                        let archive = match protocol::decode_document_archive_bytes(&archive).await {
                            Ok(archive) => archive,
                            Err(error) => {
                                debug_log(&format!("[TRACE] wgpu shell recursive document archive decode failed: {error}"));
                                continue;
                            }
                        };
                        match plugin.load_app_document_archive(instance_id, &archive).await {
                            Ok(()) => {
                                self.as_session().refresh_history_snapshot().await;
                                changed = true;
                                document_changed = true;
                            }
                            Err(error) => { let detail = self.prefs.note_refused_open(error, &mut self.chrome); debug_log(&format!("[TRACE] wgpu shell load_app_document_archive failed: {detail}")); },
                        }
                    }
                }
                ArtifactEvent::BootstrapProgress { received_bytes, total_bytes, received_chunks, total_chunks } => {
                    self.sync.sync_bootstrap_progress = Some((received_bytes, total_bytes, received_chunks, total_chunks));
                    changed = true;
                    sync_panel_changed = true;
                }
                ArtifactEvent::Status(status) => {
                    if let Some(document_key) = self.sync.sync_channel.as_ref().map(|channel| shell_hub_document_key(&channel.document_key)) {
                        self.sync.publish_hub_document_status(document_key, shell_hub_remote(&status.remote));
                    }
                    self.sync.sync_link = shell_sync_link_after_status(self.sync.sync_link, &status.remote);
                    self.sync.sync_status = Some(status);
                    changed = true;
                    sync_panel_changed = true;
                }
                ArtifactEvent::RebootstrapRequired { control } => {
                    self.sync.sync_reseed = Some(ShellSyncReseed { control, pending: None, retry_at_ms: 0.0 });
                    changed = true;
                    sync_panel_changed = true;
                }
                ArtifactEvent::Presence { peers } => {
                    self.directory.presence_peers = peers;
                    changed = true;
                }
                ArtifactEvent::Conflict(message) => {
                    if let Some((code, text, severity)) = time_travel::history_refusal_notice(&message.code.0, self.prefs.active_locale()).filter(|(code, _, _)| !time_travel::history_refusal_is_silent(code)) {
                        self.chrome.show_transient_notice(text, severity, Some(code));
                    }
                    if let Some(terminal) = shell_sync_link_terminal(&message.code.0) {
                        self.sync.sync_link = terminal;
                        self.sync.sync_terminal_fault = Some(message.message);
                    }
                    self.sync.sync_card_kind = Some("conflict".into());
                    changed = true;
                    sync_panel_changed = true;
                }
                ArtifactEvent::Session { actor, color } => {
                    self.directory.presence_self = Some((actor, color));
                    changed = true;
                }
                ArtifactEvent::DocumentBackbone { message } => {
                    let result = match plugin.as_ref() {
                        Some(plugin) => plugin.receive_document_backbone(owner.instance_id, &owner.actor_uri, message).await,
                        None => Err("document-backbone plugin owner is missing".into()),
                    };
                    match result {
                        Ok(effects) if self.sync.active_sync_owner().as_ref().is_some_and(|active| shell_sync_owner_matches(active, &owner)) => {
                            let routed = self.sync.sync_channel.as_ref().ok_or_else(|| "document-backbone owner retired before completion".to_string()).and_then(|channel| route_document_backbone_effects(&channel.actor_uri, &channel.cmd_tx, effects));
                            match routed {
                                Ok(remaining) => {
                                    let controller_id = self.docs.session.as_ref().map(|session| session.app.controller_id.clone()).unwrap_or_default();
                                    self.as_window_actions().queue_host_effects(&controller_id, remaining);
                                    changed = true;
                                    document_changed = true;
                                }
                                Err(error) => self.sync.sync_terminal_fault = Some(error),
                            }
                        }
                        Ok(_) => {}
                        Err(error) => self.sync.sync_terminal_fault = Some(self.prefs.note_refused_open(error, &mut self.chrome)),
                    }
                }
                ArtifactEvent::Preview { .. } => {
                }
                ArtifactEvent::CommandOutcome { outcome: store_sync::sync::CommandAckOutcome::Rejected { code, messages, .. }, .. } => {
                    let (code, text, severity) = command_rejection_notice(code, &messages, self.prefs.active_locale());
                    self.chrome.show_transient_notice(text, severity, Some(code));
                    changed = true;
                }
                ArtifactEvent::CommandOutcome { outcome: store_sync::sync::CommandAckOutcome::Transformed, .. } => {
                    self.chrome.show_transient_notice(shell_chrome_string("conflict.hubTransformed", self.prefs.locale_id == "de"), semio_framework::Severity::Info, Some(SYNC_COMMAND_TRANSFORMED_NOTICE_CODE));
                    changed = true;
                }
                ArtifactEvent::CommandOutcome { .. } => {}
                ArtifactEvent::DocumentArchiveAbsent => {}
            }
            if self.sync.sync_terminal_fault.is_some() {
                break;
            }
        }
        if let Some(error) = self.sync.sync_terminal_fault.take() {
            debug_log(&format!("[TRACE] wgpu shell retired document-backbone owner after terminal fault: {error}"));
            let _ = self.as_actions().detach_sync_backbone_internal().await;
            self.sync.sync_card_kind = Some("conflict".into());
            changed = true;
            document_changed = true;
        }
        if changed {
            self.dock.relabel_sync_tab(&self.prefs, &self.sync);
        }
        if document_changed {
            let _ = self.as_window_actions().refresh_ui(UiDirtyScope::Full).await;
        } else if sync_panel_changed {
            let _ = self.as_session().republish_shell_panel_document(FRAMEWORK_SYNC_PANEL_TAB_ID);
        }
        changed
    }
}
//#endregion 🔖️NativeBackboneSync
//#endregion ShellActions

//#region ShellActions
impl<'ctx> PointerCtx<'ctx> {
    pub async fn dispatch_gesture_action(&mut self, action: ActionDescriptor) -> Result<(), String> {
        self.chrome.dispatch_origin = SHELL_DISPATCH_ORIGIN_GESTURE;
        let outcome = self.as_actions().dispatch_action(action).await;
        self.chrome.dispatch_origin = SHELL_DISPATCH_ORIGIN_USER;
        outcome
    }
}
//#endregion ShellActions
