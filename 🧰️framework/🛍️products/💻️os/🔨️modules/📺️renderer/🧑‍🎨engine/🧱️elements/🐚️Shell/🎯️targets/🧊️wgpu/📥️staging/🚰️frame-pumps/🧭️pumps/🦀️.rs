//! 🚰️ `shell-frame-pumps` staging of `pumps`: the members the frame drives once per turn (directory events, media actions, retained document opening, auto check-in, operation publications and local-folder reattach), split from `shell-actions` so that crate stays inside its source budget. See [`super`].

use super::*;

//#region ShellActions
impl<'ctx> FramePumpsCtx<'ctx> {
    /// 📇️ The browser's every-frame directory turn — the same three steps the native
    /// `pump_directory_events` takes, minus the two halves whose transports this target has no door
    /// for: the retained Home projection (needs the directory WebSocket) and the document-sync
    /// backbone. Identity, the bounded administration operation and the command FIFO are the SAME
    /// code compiled for both targets.
    ///
    /// 🌉️ Packet W15e: the agent bridge's socket rides the SAME 100 ms slot rather than a timer of
    /// its own — one pump cadence for every out-of-process conversation this shell holds.
    #[cfg(target_arch = "wasm32")]
    pub async fn pump_directory_events(&mut self) -> bool {
        semio_ui_wgpu_layout::date_picker::set_today_epoch_day((semio_renderer_wgpu_host_base::app_now_ms() / 86_400_000.0).floor() as i64);
        let identity_changed = self.directory.poll_browser_identity().await;
        let administration_changed = self.directory.pump_space_administration().await;
        self.directory.flush_pending_directory_commands().await;
        let creation_changed = self.as_actions().pump_hub_artifact_creation().await;
        let hub_changed = self.sync.poll_hub_workspace_tasks(&mut self.directory);
        let bridge_changed = self.dock.pump_agent_bridge(&mut self.chrome, &mut self.directory, &mut self.windows);
        let media_changed = self.pump_media_actions().await;
        identity_changed || administration_changed || creation_changed || hub_changed || bridge_changed || media_changed
    }

    /// 📡️ Advances the finite fetch → terminal Home receipt → exact ACK → acknowledged-live owner.
    ///
    /// 🌉️ Packet W15e: the agent bridge's socket rides the SAME frame pump as the directory lane.
    ///
    /// 🏛️ One bounded administration turn per frame: the retained operation never spins, because
    /// `pump_space_administration` answers `false` the moment it reaches `Idle` or a terminal phase.
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn pump_directory_events(&mut self) -> bool {
        self.sync.poll_identity_bootstrap(&mut self.chrome, &mut self.directory);
        let inference_changed = self.directory.pump_inference_port();
        let bridge_changed = self.dock.pump_agent_bridge(&mut self.chrome, &mut self.directory, &mut self.windows);
        let mut changed = self.directory.pump_space_administration().await || inference_changed || bridge_changed;
        changed |= self.as_actions().pump_hub_artifact_creation().await;
        changed |= self.opening.pump_hub_check_in(&mut self.directory, &mut self.sync).await;
        self.opening.pump_agent_bridge_offer_scope(&mut self.directory).await;
        changed |= self.sync.poll_hub_workspace_tasks(&mut self.directory);
        let runner = self.directory.directory_home.as_ref().and_then(|home| home.stream.clone());
        if let Some(runner) = runner {
            if runner.take_terminal() {
                if let Some(home) = self.directory.directory_home.as_mut() {
                    let _ = home.begin_epoch(0);
                }
                self.directory.directory_client = None;
                #[cfg(not(target_arch="wasm32"))]
        self.directory.cancel_inference_port();
        self.directory.identity = None;
                self.directory.verified_session_authority = None;
                self.directory.bootstrap_identity();
                changed = true;
            }
            let mut dirty = false;
            let mut rebootstrap = false;
            let mut live_events: Vec<semio_os_kernel::os_directory::DirectoryEvent> = Vec::new();
            for message in runner.drain() {
                match message {
                    DirectoryStreamMessage::Event { event } => {
                        live_events.push(event.as_ref().clone());
                        dirty = true;
                    }
                    DirectoryStreamMessage::Heartbeat { .. } => dirty = true,
                    DirectoryStreamMessage::Connection { .. } | DirectoryStreamMessage::Presence { .. } => {}
                    DirectoryStreamMessage::RebootstrapRequired { .. } | DirectoryStreamMessage::AccessChanged { .. } => rebootstrap = true,
                }
            }
            if !live_events.is_empty() {
                self.as_session().dispatch_fold_directory_events(&live_events);
                changed = true;
            }
            if dirty || rebootstrap {
                if let Some(home) = self.directory.directory_home.as_mut() {
                    match home.wake(rebootstrap) {
                        Ok(Some(_)) => changed = true,
                        Ok(None) => {}
                        Err(error) => {
                            home.close();
                            self.chrome.error = Some(error.to_string());
                        }
                    }
                }
            }
        }

        if let Some((epoch, result)) = self.directory.poll_directory_page_result() {
            let current = self.directory.directory_home.as_ref().is_some_and(|home| !home.closed && home.bootstrap.bootstrap_epoch() == epoch);
            if current {
                match result {
                    Ok(page) => {
                        if let Err(error) = self.docs.start_directory_home_publication(page, &self.plugins, &mut self.directory) {
                            if let Some(home) = self.directory.directory_home.as_mut() {
                                home.close();
                            }
                            self.chrome.error = Some(error);
                        }
                    }
                    Err(DirectoryClientError::Unauthorized) => {
                        if let Some(home) = self.directory.directory_home.as_mut() {
                            let _ = home.begin_epoch(0);
                        }
                        self.directory.directory_client = None;
                        #[cfg(not(target_arch="wasm32"))]
        self.directory.cancel_inference_port();
        self.directory.identity = None;
                        self.directory.verified_session_authority = None;
                        self.directory.bootstrap_identity();
                    }
                    Err(DirectoryClientError::Transport(TransportError::Io(_) | TransportError::DeadlineExceeded | TransportError::BudgetExhausted) | DirectoryClientError::Http { status: 500..=599, .. }) => {
                        if let Some(home) = self.directory.directory_home.as_mut() {
                            home.retry_at_ms = semio_renderer_wgpu_host_base::renderer_worker_pool().now_ms().saturating_add(DirectoryHomeProjection::RETRY_DELAY_MS);
                        }
                    }
                    Err(DirectoryClientError::Cancelled | DirectoryClientError::Transport(TransportError::Cancelled)) => {}
                    Err(_) => {
                        if let Some(home) = self.directory.directory_home.as_mut() {
                            home.close();
                        }
                        self.chrome.error = Some("directory event-page response was terminally rejected".into());
                    }
                }
                changed = true;
            }
        }

        if let Some((epoch, instance_id, expected, outcome)) = self.directory.poll_directory_home_publication() {
            if let ShellDirectoryHomePublicationOutcome::Rejected(refusal) = &outcome {
                if self.chrome.chrome_build.refused_frame_fault.is_none() { self.chrome.chrome_build.refused_frame_fault = refusal.frame; }if self.chrome.chrome_build.refused_kernel_fault.is_none(){self.chrome.chrome_build.refused_kernel_fault=refusal.kernel;}if self.chrome.chrome_build.refused_actor_call.is_none(){self.chrome.chrome_build.refused_actor_call=refusal.actor_call;}
            }
            let current = self.directory.directory_home.as_ref().is_some_and(|home| !home.closed && home.bootstrap.bootstrap_epoch() == epoch && home.instance_id == instance_id);
            if current {
                match outcome {
                    ShellDirectoryHomePublicationOutcome::Published(acknowledgement) => {
                        let transition = self.directory.directory_home.as_mut().expect("checked retained Home").bootstrap.acknowledge(&acknowledgement).map_err(|error| error.to_string());
                        match transition {
                            Ok(DirectoryBootstrapTransition::Fetch { .. }) => {}
                            Ok(DirectoryBootstrapTransition::Live { since }) => {
                                if let Err(error) = self.directory.open_acknowledged_directory_stream(since) {
                                    if let Some(home) = self.directory.directory_home.as_mut() {
                                        home.close();
                                    }
                                    self.chrome.error = Some(error);
                                }
                            }
                            Err(error) => {
                                if let Some(home) = self.directory.directory_home.as_mut() {
                                    home.close();
                                }
                                self.chrome.error = Some(error);
                            }
                        }
                    }
                    ShellDirectoryHomePublicationOutcome::Rejected(error) => {
                        let frame_refusal = error.frame.is_some();
                        let rebootstrap = error.text.contains("directory-event-page-rebootstrap-required");
                        let detail = self.prefs.note_refused_open(error, &mut self.chrome);
                        if let Some(home) = self.directory.directory_home.as_mut() {
                            if frame_refusal {
                                home.close();
                                self.chrome.error = Some(detail);
                            } else if rebootstrap {
                                let _ = home.begin_epoch(0);
                            } else if home.retry(semio_renderer_wgpu_host_base::renderer_worker_pool().now_ms(), &expected.receipt_sha256).is_err() {
                                home.close();
                                self.chrome.error = Some("retained Home directory rejection no longer matches the pending page".into());
                            }
                        }
                    }
                    ShellDirectoryHomePublicationOutcome::Terminal(error) => {
                        if let Some(home) = self.directory.directory_home.as_mut() {
                            home.close();
                        }
                        self.chrome.error = Some(error);
                    }
                }
                changed = true;
            }
        }

        self.directory.start_directory_event_page_fetch();
        self.directory.flush_pending_directory_commands().await;
        changed
    }

    /// 🎞️ Runs every media-window action the page host left (the trim "Cut selection") through the same `dispatch_action` the UI intents use, so the response's mutations commit through the normal ingest path; a refused one is logged, never dropped silently. Answers whether anything ran.
    pub async fn pump_media_actions(&mut self) -> bool {
        let actions = self.dock.drain_media_actions();
        let ran = !actions.is_empty();
        for action in actions {
            let label = format!("{}/{}", action.controller_id, action.action);
            let dispatched = self.as_actions().dispatch_action(action).await;
            if let Err(error) = dispatched {
                debug_log(&format!("[TRACE] wgpu shell media action {label} refused: {error}"));
            }
        }
        ran
    }

    /// ⏭️ Advances the retained open by the one detached step that answered, if any: an instance mounts
    /// as the session (or is destroyed when the open was cancelled), a seeded document binds, and every
    /// render the open owes goes to the settle lane. Called once per frame from [`Self::pump_sync_events`];
    /// answers whether the open moved.
    pub async fn advance_document_opening(&mut self) -> bool {
        let Some(opening) = self.opening.document_opening.as_mut() else { return false; };
        let Some(answer) = opening.pending.as_ref().and_then(ShellDetached::take) else {
            let progress = opening.native_transfer_progress.load(std::sync::atomic::Ordering::Acquire);
            return std::mem::replace(&mut opening.observed_native_transfer_progress, progress) != progress;
        };
        let Some(mut opening) = self.opening.document_opening.take() else { return false };
        opening.pending = None;
        match answer {
            #[cfg(not(target_arch = "wasm32"))]
            Ok(ShellDocumentOpenAnswer::Resolved { plugin_id, app_id, program, checkpoint }) => {
                if opening.cancel_requested {
                    opening.phase = ShellDocumentOpenPhase::Cancelled;
                    self.opening.document_opening = Some(opening);
                    return true;
                }
                opening.plugin_id = plugin_id;
                opening.app_id = app_id;
                if let Some(document) = opening.document.as_mut() {
                    document.checkpoint = Some(checkpoint);
                }
                self.as_actions().continue_resolved_open(opening, program).await;
            }
            #[cfg(not(target_arch = "wasm32"))]
            Ok(ShellDocumentOpenAnswer::NativeFile(imported)) => {
                if opening.cancel_requested || imported.is_none() {
                    opening.phase = ShellDocumentOpenPhase::Cancelled;
                    self.opening.document_opening = Some(opening);
                } else {
                    let opened = self.as_actions().finish_native_open(&opening.app_id, imported.expect("checked imported file")).await;
                    if let Err(error) = opened {
                        self.opening.document_opening = Some(opening);
                        self.opening.fail_document_opening(error, &mut self.chrome, &self.prefs);
                    }
                }
            }
            #[cfg(target_arch = "wasm32")]
            Ok(ShellDocumentOpenAnswer::BrowserRouting(routing)) => {
                self.as_actions().continue_browser_routed(opening, routing).await;
            }
            #[cfg(target_arch = "wasm32")]
            Ok(ShellDocumentOpenAnswer::BrowserFile(imported)) => {
                if opening.cancel_requested || imported.is_none() {
                    opening.phase = ShellDocumentOpenPhase::Cancelled;
                    self.opening.document_opening = Some(opening);
                } else if let Err(error) = self.as_actions().finish_browser_open(imported.expect("checked imported file")).await {
                    self.opening.document_opening = Some(opening);
                    self.opening.fail_document_opening(error, &mut self.chrome, &self.prefs);
                }
            }
            Ok(ShellDocumentOpenAnswer::Instantiated(instance_id)) => {
                if opening.cancel_requested {
                    if let Some(program) = self.plugins.plugins.iter().find(|entry| entry.plugin_id == opening.plugin_id) {
                        program.destroy_app(instance_id);
                    }
                    opening.phase = ShellDocumentOpenPhase::Cancelled;
                    self.opening.document_opening = Some(opening);
                    return true;
                }
                let Some(app) = self.plugins.plugins.iter().find(|entry| entry.plugin_id == opening.plugin_id).and_then(|program| program.manifest.apps.iter().find(|app| app.id == opening.app_id).cloned()) else {
                    self.opening.document_opening = Some(opening);
                    self.opening.fail_document_opening("the opened app left its plugin".to_string(), &mut self.chrome, &self.prefs);
                    return true;
                };
                self.windows.install_app_session(&opening.plugin_id, app, instance_id, &self.prefs, &mut self.directory, &mut self.docs);
                self.settle.owe_refresh(UiDirtyScope::Full);
                self.settle.owe_settle();
                match opening.document.take() {
                    Some(document) => {
                        self.opening.document_opening = Some(opening);
                        self.as_actions().begin_document_seed(document).await;
                    }
                    None => self.opening.document_opening = None,
                }
            }
            Ok(ShellDocumentOpenAnswer::Seeded(pair)) => {
                let Some(prepared) = opening.prepared.take() else {
                    self.opening.document_opening = Some(opening);
                    self.opening.fail_document_opening("seeded document open lost its prepared owner".to_string(), &mut self.chrome, &self.prefs);
                    return true;
                };
                if opening.cancel_requested {
                    opening.phase = ShellDocumentOpenPhase::Cancelled;
                    self.opening.document_opening = Some(opening);
                    return true;
                }
                if let Err(error) = self.sync.install_document_seed(&prepared, pair) {
                    self.opening.document_opening = Some(opening);
                    self.opening.fail_document_opening(error, &mut self.chrome, &self.prefs);
                    return true;
                }
                let opened = self.as_actions().finish_document_open(prepared).await;
                match opened {
                    Ok(()) => {
                        self.settle.owe_refresh(UiDirtyScope::Full);
                        self.settle.owe_settle();
                    }
                    Err(error) => {
                        self.opening.document_opening = Some(opening);
                        self.opening.fail_document_opening(error, &mut self.chrome, &self.prefs);
                        return true;
                    }
                }
            }
            Err(_) if opening.cancel_requested => {
                opening.phase = ShellDocumentOpenPhase::Cancelled;
                self.opening.document_opening = Some(opening);
            }
            Err(error) => {
                self.opening.document_opening = Some(opening);
                let detail = self.prefs.note_refused_open(error, &mut self.chrome);
                self.opening.fail_document_opening(detail, &mut self.chrome, &self.prefs);
            }
        }
        true
    }

    /// 📌️ ticket §C5 item 2 — the per-frame auto check-in poll, called from `pump_sync_events` (see
    /// that method's own doc: "folded into the same every-frame pump"). Viewer sessions never arm
    /// (`can_check_in`, item 5's guard) — the SAME predicate the `#s-checkin` affordance itself checks.
    pub async fn poll_auto_checkin(&mut self) {
        let Some(session) = self.docs.session.as_ref() else { return };
        if !can_check_in(session.app.role) {
            return;
        }
        let count = uncommitted_edit_count(&self.history.history_entries);
        if count == 0 {
            self.sync.auto_checkin_pending = false;
            self.sync.last_uncommitted_edit_at_ms = None;
            return;
        }
        let now_ms = chrome_now_ms() as i64;
        if auto_checkin_should_fire(count, self.sync.auto_checkin_pending, self.sync.last_uncommitted_edit_at_ms, now_ms, AUTO_CHECKIN_IDLE_MS, AUTO_CHECKIN_EDIT_THRESHOLD) {
            self.sync.auto_checkin_pending = true;
            self.as_actions().dispatch_checkpoint("auto").await;
        }
    }

    /// 🏁️ Applies what the session's guest published between two dispatches — the wgpu twin of React's
    /// `subscribeOperationCompletions` and `subscribeOperationProgress` passes. A typed operation that outlives the host
    /// call that started it (an example load of a few hundred steps, the replay behind an accepted history edit) ends
    /// on the bridge's standing drain, and its completion is the only carrier of its dirty scope and command-log
    /// delta. Each publication's history patch takes the reply's own stale-guarded fold
    /// ([`ShellState::observe_invocation_history`]) and the union of what they owe ([`operation_publication_refresh`])
    /// goes to the settle lane ([`ShellState::owe_refresh`]) — the surface refreshes and the rows land with no further
    /// dispatch. A drain `fault` is told through the dispatch-fault funnel; `resync` re-reads the whole projection.
    /// `true` when the session, the rows or the owed refresh changed, or a fault was told.
    pub async fn drain_operation_publications(&mut self) -> bool {
        let publications = self.docs.session.as_ref().and_then(|session| self.plugins.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).map(|plugin| plugin.take_operation_publications(session.instance_id))).unwrap_or_default();
        if publications.is_empty() {
            return false;
        }
        let before = (self.history.history_time_travel.clone(), self.history.history_reprojection.clone(), self.history.history_cursor, self.history.history_entries.len());
        let mut owed = UiDirtyScope::None;
        let mut told = false;
        for publication in &publications {
            if let Some(fault) = publication.fault.as_deref() {
                debug_log(&format!("[TRACE] wgpu shell operation publication fault: {fault}"));
                self.prefs.note_dispatch_fault(fault, &mut self.chrome);
                told = true;
            }
            if publication.resync {
                self.as_session().reread_history_projection().await;
            }
            self.as_actions().observe_invocation_history(publication.history_patch.as_ref()).await;
            owed = owed.merged_with(operation_publication_refresh(publication));
        }
        let owes = !owed.asks_for_nothing();
        self.settle.owe_refresh(owed);
        told || owes || before != (self.history.history_time_travel.clone(), self.history.history_reprojection.clone(), self.history.history_cursor, self.history.history_entries.len())
    }

    /// 🔁️ The boot reattach: once per document, the remembered folder of the session's document is reopened without a
    /// gesture. A failure is told through the dispatch-fault funnel and leaves the band offering it. `true` when it ran.
    pub async fn reattach_remembered_local_folder(&mut self) -> bool {
        self.history.ensure_local_folder_bindings();
        let Some(offer) = self.docs.direct_reattach_candidate(&self.history, &self.sync) else { return false };
        self.history.local_folder_reattach_tried = Some(offer.document_id.clone());
        let reattached = self.as_actions().reattach_local_folder(&offer).await;
        if let Err(error) = reattached {
            self.prefs.note_dispatch_fault(&error, &mut self.chrome);
        }
        true
    }
}

//#endregion ShellActions
