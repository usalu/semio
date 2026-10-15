//! 📨️ Shell actions, part two: command dispatch, hub workspace, documents and identity.
//!
//! `dispatch_command`, session and replay commands, space administration, hub workspace actions, document
//! services, directory event pumps, identity bootstrap, `apply_mutations`, extension records, plugin install
//! and `apply_shell_uri`.
//!
//! One chunk of the former `shell` monolith (ticket CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT); [`ShellState`] and the
//! shared imports stay in the parent module and are reached through `use super::*;`.

use super::*;

//#region ShellActions
impl ShellState {
    /// 🎛️ Sends one owner-qualified non-OS command through the program command boundary.
    ///
    /// 🪟️ The LIVE view, exactly as `dispatch_action` and React's `makeEffectDispatchOne`
    /// (`resolveViewState(baseSession)`) read it: the session's stored `view_state` carries no
    /// window-instance roster, so a window-addressed command — every `flowEvalTick` the guest
    /// re-arms — was refused with `target window is absent from the exact ViewModel window
    /// instance roster` (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    ///
    /// 🧾️ ticket §C5 — same fold `dispatch_action` performs; a command-boundary edit (e.g. a
    /// plugin-owned command dispatched from the command palette) is just as real an uncommitted
    /// edit as an action-boundary one.
    ///
    /// 🧾️ ONE host-effect funnel for both dispatch paths. This arm used to be a SECOND,
    /// shorter hand-rolled fold, and every effect it did not name was dropped without a trace —
    /// including `InvokeExtension`, so the `flowEvalTick` reply that finally asked for an
    /// extension call went nowhere and the preview chain stopped one hop before its first
    /// evaluate (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). React has the same single funnel:
    /// `makeEffectDispatchOne` hands BOTH `handleAction` and `handleCommand` results to
    /// `applyHostEffects`. Only the two effects `queue_host_effects` cannot own — a `Navigate`
    /// that must also RESOLVE the uri, and the replay relay — stay here, because both are
    /// `async` and that funnel is deliberately a plain `fn`.
    pub fn dispatch_command<'a>(&'a mut self, invocation: semio_framework::manifest::CommandInvocation) -> ShellTurn<'a, Result<(), String>> {
        shell_turn(async move {
            if matches!(invocation.address.owner, semio_framework::manifest::CommandOwnerAddress::Os) {
                return Err("os commands must be dispatched by the shell".into());
            }
            let window_id = invocation.arguments.get("windowId").and_then(semio_framework_value::DslValue::as_str);
            let session = self.command_session(&invocation.address.owner, window_id).cloned().ok_or("command has no mounted app owner")?;
            self.dispatch_session_command(invocation, session).await
        })
    }

    /// 🧵️ A retained command keeps the exact session selected by its source document.
    pub(crate) fn dispatch_session_command<'a>(&'a mut self, invocation: semio_framework::manifest::CommandInvocation, session: ActiveSession) -> ShellTurn<'a, Result<(), String>> {
        shell_turn(async move {
            let owner_plugin_id = match &invocation.address.owner {
                semio_framework::manifest::CommandOwnerAddress::Plugin { plugin_id } | semio_framework::manifest::CommandOwnerAddress::App { plugin_id, .. } | semio_framework::manifest::CommandOwnerAddress::Mode { plugin_id, .. } => plugin_id,
                semio_framework::manifest::CommandOwnerAddress::Os => return Err("os commands must be dispatched by the shell".into()),
            };
            let program = self.plugins.iter().find(|entry| entry.plugin_id == *owner_plugin_id).cloned().ok_or("command program missing")?;
            let command_json = semio_framework_pack_json::to_json_string(&invocation);
            let live_view_state = self.live_view_state(&session);
            let answer = program.handle_command(session.instance_id, &command_json, &live_view_state).await;
            let mut result = answer.map_err(|refusal| self.refused_guest_call(refusal, &session.app))?;
            Self::debug_log(&format!("[TRACE] wgpu-shell command {} settled effects={} mutations={}", invocation.address.command_id, result.requested_effects.len(), result.mutations.len()));
            self.observe_invocation_history(result.history_patch.as_ref()).await;
            self.observe_interaction_output(&result.output);
            let mut queued = Vec::with_capacity(result.requested_effects.len());
            for effect in core::mem::take(&mut result.requested_effects) {
                match effect {
                    semio_framework::kernel::Effect::Navigate { uri } => {
                        self.push_uri(uri.clone());
                        self.apply_shell_uri(&uri).await?;
                    }
                    semio_framework::kernel::Effect::ReplayShellCommand { action_id, args } => {
                        self.handle_replay_shell_command(&action_id, args.as_ref()).await;
                    }
                    other => queued.push(other),
                }
            }
            let scope = result.ui_scope.clone().merged_with(self.host_effect_earned_scope(&queued));
            Self::debug_log(&format!("[TRACE] wgpu-shell dispatch command={} scope={}", invocation.address.command_id, refresh_scope_label(&scope)));
            self.queue_host_effects(&session.app.controller_id.clone(), queued);
            let operations: Vec<String> = result.mutations.iter().filter_map(|operation| serde_json::to_string(&operation.diff.payload).ok()).collect();
            self.apply_ops_inner(&operations, true, scope, Some(session)).await
        })
    }

    //#region 🔖️DirectoryAndIdentity
    //#region 📇️DirectoryLane
    /// 📇️ ticket §C6/§3/§4 — dispatches a `ReplayShellCommand`'s `action_id`/`args` onto the directory funnel, the opening
    /// relay or the creation door; an `os.*` command this shell cannot serve is refused out loud ([`Self::refuse_replay`]) —
    /// the local studio catalog (`os.local-catalog.*`) until route B's native half lands, any unknown one as unrouted.
    pub(crate) fn handle_replay_shell_command<'a>(&'a mut self, action_id: &'a str, args: Option<&'a DslValue>) -> ShellTurn<'a, ()> {
        shell_turn(async move {
            let args_json = args.map(dsl_value_as_json);
            if action_id == "os.directory.open-administration" {
                let space_id = args_json.as_ref().and_then(|args| args.get("spaceId")).and_then(Value::as_str).unwrap_or_default().to_string();
                self.open_space_administration(&space_id);
                return;
            }
            if let Some(command) = directory_command_from_action(action_id, args_json.as_ref()) {
                self.dispatch_directory_command(command).await;
                return;
            }
            if action_id == "os.open-artifact" || action_id == "os.open-artifact-with" {
                self.handle_open_artifact_relay(action_id, args_json.as_ref()).await;
                return;
            }
            let refusal = if action_id == "os.create-space-artifact" {
                self.replay_create_space_artifact(args_json.as_ref()).await.err()
            } else if action_id.starts_with("os.local-catalog.") {
                Some(ReplayRefusalReasonV1::LocalCatalogUnavailable)
            } else if action_id.starts_with("os.directory.") {
                Some(ReplayRefusalReasonV1::InvalidRequest)
            } else if action_id.starts_with("os.") {
                Some(ReplayRefusalReasonV1::UnroutedCommand)
            } else {
                None
            };
            if let Some(reason) = refusal {
                self.refuse_replay(reason);
            }
        })
    }

    /// 📣️ Refuses one guest `replayShellCommand` out loud: the shared vocabulary's warning notice in this shell's tongue,
    /// carrying the reason's fault code — React's `refuseReplay`.
    fn refuse_replay(&mut self, reason: ReplayRefusalReasonV1) {
        let code = reason.code();
        self.show_transient_notice(reason.notice_text(&self.locale_id), semio_framework::Severity::Warning, Some(code.as_str()));
    }

    /// 🌱️ `os.create-space-artifact {kindChoice, name}` from the mounted Space index guest onto this shell's ONE creation door —
    /// ShellHost's route: a signed-in identity, the index's own space, a choice the space's ready catalog offers
    /// ([`space_artifact_creation_replay_choice`]); the door then seals the intent under its minted key, submits it and opens the
    /// ready artifact ([`Self::pump_hub_artifact_creation`]). A busy or not-yet-ready door asks the user to try again.
    async fn replay_create_space_artifact(&mut self, args: Option<&Value>) -> Result<(), ReplayRefusalReasonV1> {
        if self.identity.is_none() {
            return Err(ReplayRefusalReasonV1::SignInRequired);
        }
        let index_mounted = self.sync_channel.as_ref().is_some_and(|channel| channel.document_id == SPACE_INDEX_DOCUMENT_ID);
        let space_id = self.open_space_id.clone().filter(|space_id| index_mounted && !space_id.is_empty()).ok_or(ReplayRefusalReasonV1::SpaceIndexRequired)?;
        if self.hub_workspace.open_space_id.as_deref() != Some(space_id.as_str()) || self.hub_workspace.creation.catalog.as_ref().is_none_or(|catalog| catalog.space_id != space_id) {
            self.hub_workspace.open_space_id = Some(space_id.clone());
            self.open_hub_artifact_creation(&space_id).await;
        }
        let (kind_id, name) = space_artifact_creation_replay_choice(args, self.hub_workspace.creation.catalog.as_ref(), &space_id).await?;
        let creation = &mut self.hub_workspace.creation;
        creation.kind_id = Some(kind_id);
        creation.name_draft = name;
        if hub_artifact_creation_intent(&self.hub_workspace).is_none() {
            let busy = self.hub_workspace.creation.catalog_phase != HubArtifactCatalogPhase::Ready || self.hub_workspace.creation.operation.as_ref().is_some_and(|operation| !hub_artifact_creation_terminal(operation.phase));
            return Err(if busy { ReplayRefusalReasonV1::RouterNotReady } else { ReplayRefusalReasonV1::InvalidRequest });
        }
        self.begin_hub_artifact_creation();
        self.pump_hub_artifact_creation().await;
        Ok(())
    }

    /// 🏛️ Installs the one retained administration operation for exactly one space, retiring any
    /// predecessor first. Nothing is administrable until the hub's own canonical page has landed.
    pub fn open_space_administration(&mut self, space_id: &str) {
        if space_id.trim().is_empty() || self.identity.is_none() {
            return;
        }
        self.close_space_administration();
        self.space_administration_epoch = self.space_administration_epoch.saturating_add(1);
        self.space_administration = Some(ShellSpaceAdministrationOperationV1::open(self.space_administration_epoch, space_id));
    }

    /// 🧯️ Retires the operation: page, receipt, and capability are erased before anything observes it.
    pub fn close_space_administration(&mut self) {
        if let Some(operation) = self.space_administration.as_mut() {
            operation.terminate(ShellSpaceAdministrationPhaseV1::Cancelled, Some(DirectoryCommandErrorCodeV1::Cancelled));
        }
        self.space_administration = None;
    }

    /// 🎁️ Hands the one-shot invite capability to the caller exactly once, erasing it in the same turn.
    pub fn acknowledge_space_administration_capability(&mut self) -> Option<String> {
        self.space_administration.as_mut().and_then(ShellSpaceAdministrationOperationV1::acknowledge_capability)
    }

    /// 🎬️ Drives exactly one bounded administration turn against the real `DirectoryClient`. Returns
    /// `true` while the operation still has work, so the frame loop can pump it without spinning.
    /// A mutation is never retried: an indeterminate transport settles `Failed`, and only an exact
    /// server receipt advances the operation, always followed by a mandatory page refresh.
    pub async fn pump_space_administration(&mut self) -> bool {
        let Some(client) = self.directory_client.clone() else { return false };
        let epoch = self.space_administration_epoch;
        let Some(turn) = self.space_administration.as_mut().map(ShellSpaceAdministrationOperationV1::turn) else { return false };
        match turn {
            ShellSpaceAdministrationTurnV1::Idle => false,
            ShellSpaceAdministrationTurnV1::Closed => {
                self.space_administration = None;
                false
            }
            ShellSpaceAdministrationTurnV1::FetchPage { cursor } => {
                let space_id = self.space_administration.as_ref().map(|operation| operation.space_id().to_string()).unwrap_or_default();
                let result = client.space_administration_page(&self.directory_command_ctx(), &space_id, cursor.as_deref()).await;
                let Some(operation) = self.space_administration.as_mut().filter(|operation| operation.operation_epoch() == epoch) else { return false };
                match result {
                    Ok(page) => {
                        let canonical = page.canonical_json().to_string();
                        if !operation.apply_page(canonical, page.page().clone()) {
                            operation.terminate(ShellSpaceAdministrationPhaseV1::Failed, Some(DirectoryCommandErrorCodeV1::Invalid));
                        }
                    }
                    Err(error) => operation.fail_page(&error),
                }
                true
            }
            ShellSpaceAdministrationTurnV1::Submit { request } => {
                let result = client.command(&self.directory_command_ctx(), &request).await;
                let Some(operation) = self.space_administration.as_mut().filter(|operation| operation.operation_epoch() == epoch) else { return false };
                match result {
                    Ok(canonical) => {
                        if !operation.apply_receipt(&canonical.receipt) {
                            operation.terminate(ShellSpaceAdministrationPhaseV1::Failed, Some(DirectoryCommandErrorCodeV1::Invalid));
                        }
                    }
                    Err(code) => operation.fail_command(code),
                }
                true
            }
        }
    }

    /// 🪪️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-directory-and-run): builds this shell's
    /// `OperationContext` for one directory request — `cancel` is a `.child()` of `directory_cancel`
    /// (this shell's ONE root, see that field's own doc), so cancelling `directory_cancel` once
    /// transitively cancels every in-flight directory request/stream at once. No deadline yet (see
    /// the packet report's honest gaps).
    pub(crate) fn directory_ctx(&self) -> OperationContext {
        OperationContext { actor: 0, generation: 0, trace: TraceId(0), lane: 0, deadline_ms: None, cancel: self.directory_cancel.child_now(), capability: None }
    }

    /// 🛑️ One command-scoped context: a fresh `directory_cancel` child plus a finite deadline, so a
    /// hung hub can never retain a command turn forever. Cancelling the wait does NOT cancel a
    /// command already past its server linearization point — the operation becomes indeterminate and
    /// keeps its request id for an explicit later resolution, never a silently fresh one.
    pub(crate) fn directory_command_ctx(&self) -> OperationContext {
        let mut context = self.directory_ctx();
        context.deadline_ms = Some(Self::directory_now_ms().saturating_add(DIRECTORY_COMMAND_DEADLINE_MS));
        context
    }

    /// ⏱️ The clock a directory deadline is measured against — necessarily the SAME one the linked
    /// transport compares it to. Native hands the shared `WorkerPool`'s own monotonic millis (what
    /// `NativeDirectoryTransport`'s `HttpPool` reads); the browser door has no pool and the page
    /// answers every hop with `fetch`'s own timing, so `Date.now()` is the only shared reading.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn directory_now_ms() -> u64 {
        crate::renderer_worker_pool().now_ms()
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn directory_now_ms() -> u64 {
        let millis = chrome_now_ms();
        if millis.is_finite() && millis > 0.0 {
            millis as u64
        } else {
            0
        }
    }

    /// 📇️ ticket §4/§C6 — seals one `os.directory.*` command under a fresh idempotency correlation
    /// and issues it against the hub, dropping it when no identity is signed in at all, else leaving
    /// it queued on a transient failure (the "same offline queueing behaviour as the React side" the
    /// brief asks for — React's worker queues in-memory and flushes on reconnect; this shell's
    /// `directory_commands` + `flush_pending_directory_commands` (called every frame from
    /// `pump_directory_events`) is the native twin of that policy).
    async fn dispatch_directory_command(&mut self, command: DirectoryCommand) {
        if self.identity.is_none() {
            return;
        }
        self.directory_commands.admit_first(DirectoryCommandRequestV1::new(mint_directory_command_request_id(), command));
        self.flush_pending_directory_commands().await;
    }

    /// ♻️ Drives the FIFO in order: a transient fault retains the head and its byte-identical sealed
    /// request and stops the queue; a terminal auth/conflict/validation failure produces a result and
    /// lets the queue proceed, so one forbidden command can never wedge every later one. Every receipt's
    /// events are folded into the hub panel's space rows at once (read-your-writes,
    /// [`crate::space_browser::space_rows_after_events`]), so a space the user just created is listed
    /// whatever the following list query answers.
    async fn flush_pending_directory_commands(&mut self) {
        let Some(client) = self.directory_client.clone() else { return };
        while let Some(request) = self.directory_commands.head().cloned() {
            let outcome = client.command(&self.directory_command_ctx(), &request).await.map(|canonical| canonical.receipt);
            if let (Ok(receipt), Some(identity)) = (&outcome, self.identity.as_ref()) {
                self.hub_workspace.rows = crate::space_browser::space_rows_after_events(&self.hub_workspace.rows, &receipt.events, &identity.user_id);
                self.hub_workspace.phase = crate::space_browser::SpaceBrowserPhase::Ready;
            }
            if !self.directory_commands.settle(outcome) {
                break;
            }
        }
    }

    /// 🪪️ Browser identity bootstrap. The hub session on this target is a same-origin COOKIE, not an
    /// inherited one-shot fd-3 credential, so there is nothing to `restore_claimed`: the shell asks
    /// the SAME `GET /auth/sessions/me` the React shell's `directoryClient.me()` asks, through an
    /// unauthenticated client whose door sends `credentials: "include"`. A refusal parks the retry
    /// floor instead of retrying every frame, and never clears an identity already resolved.
    #[cfg(target_arch = "wasm32")]
    async fn poll_browser_identity(&mut self) -> bool {
        if self.identity.is_some() || self.identity_env.is_none() {
            return false;
        }
        let now_ms = Self::directory_now_ms();
        if now_ms < self.identity_retry_at_ms {
            return false;
        }
        self.identity_retry_at_ms = now_ms.saturating_add(BROWSER_IDENTITY_RETRY_MS);
        let client = std::sync::Arc::new(DirectoryClient::new(shell_directory_transport(), shell_directory_request_base_url()));
        match client.me(&self.directory_ctx()).await {
            Ok(session) => {
                self.identity = Some(Identity { user_id: session.user_id.clone(), email: session.email.clone(), display_name: session.display_name.clone(), hub_base_url: client.base_url().to_string(), issued_at_ms: chrome_now_ms() as i64 });
                self.verified_session_authority = Some(session);
                self.identity_offline = false;
                self.directory_client = Some(client);
                self.project_verified_hub_authority();
                true
            }
            Err(error) => {
                self.identity_offline = true;
                Self::debug_log(&format!("[TRACE] wgpu shell browser identity bootstrap failed: {error}"));
                false
            }
        }
    }

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
        let identity_changed = self.poll_browser_identity().await;
        let administration_changed = self.pump_space_administration().await;
        self.flush_pending_directory_commands().await;
        let creation_changed = self.pump_hub_artifact_creation().await;
        let hub_changed = self.poll_hub_workspace_tasks();
        let bridge_changed = self.pump_agent_bridge();
        identity_changed || administration_changed || creation_changed || hub_changed || bridge_changed
    }

    //#endregion 📇️DirectoryLane

    //#region 🔐️HubWorkspaceLane
    /// 🔐️ The ONE lane every `hub*` verb the hub workspace names runs through (slice WG6). Drafts
    /// are pure state writes; the four verbs that leave the process go through the shell's own
    /// `DirectoryTransport`, so this renderer's sign-in is the same transport its directory client
    /// already uses rather than a second HTTP door.
    ///
    /// 🏠️ Nothing here can ever stop local work: every failure lands in the session's `error` slot
    /// or in the spaces `Stale` phase, and the surface keeps rendering.
    ///
    /// 📋️ `COPY_INVITE_LINK` is deliberately unreachable on this renderer: the surface only paints
    /// that control when `clipboard_available` is set, and the wgpu frame Worker owns no clipboard
    /// door (see this file's own note in the retained editor lane).
    pub(crate) fn handle_hub_workspace_action<'a>(&'a mut self, verb: &'a str, args: Option<DslValue>) -> ShellTurn<'a, ()> {
        shell_turn(async move {
            let value = args.as_ref().and_then(|args| args.get("value")).and_then(semio_framework_value::DslValue::as_str).unwrap_or_default().to_string();
            let space_id = args.as_ref().and_then(|args| args.get("spaceId")).and_then(semio_framework_value::DslValue::as_str).unwrap_or_default().to_string();
            let connection_id = args.as_ref().and_then(|args| args.get("connectionId")).and_then(semio_framework_value::DslValue::as_str).unwrap_or_default().to_string();
            match verb {
                hub_action::SET_EMAIL => self.hub_workspace.email_draft = value,
                hub_action::SET_PASSWORD => self.hub_workspace.password_draft = value,
                hub_action::SET_ADDRESS => self.hub_workspace.address_draft = value,
                hub_action::SET_SEARCH => self.hub_workspace.search_draft = value,
                hub_action::SET_SPACE_NAME => self.hub_workspace.space_name_draft = value,
                hub_action::SET_INVITE_TEXT => self.hub_workspace.invite_draft = value,
                hub_action::DISCARD_INVITE_LINK => self.hub_workspace.invite_capability = None,
                hub_action::CLOSE_WORKSPACE => {
                    self.hub_workspace.open_space_id = None;
                    self.hub_workspace_open = false;
                }
                hub_action::SELECT_CONNECTION => {
                    let next = select_hub_connection(&self.hub_workspace.book, &connection_id);
                    if next.selected_id != self.hub_workspace.book.selected_id {
                        self.hub_workspace.book = next;
                        self.hub_workspace.session = reduce_hub_session(&self.hub_workspace.session, &HubSessionEvent::SelectConnection { connection_id });
                        self.clear_hub_session_owner();
                        self.hub_workspace.rows.clear();
                        self.hub_workspace.members.clear();
                        self.hub_workspace.open_space_id = None;
                        self.persist_hub_connection_book();
                    }
                }
                hub_action::ADD_CONNECTION => match parse_hub_origin(&self.hub_workspace.address_draft) {
                    Some(origin) => {
                        let connection = HubConnection { id: hub_connection_id_for_origin(&origin), kind: HubConnectionKind::Remote, label: origin.clone(), last_user_id: None, origin };
                        let connection_id = connection.id.clone();
                        self.hub_workspace.book = upsert_hub_connection(&self.hub_workspace.book, connection);
                        self.hub_workspace.session = reduce_hub_session(&self.hub_workspace.session, &HubSessionEvent::SelectConnection { connection_id });
                        self.hub_workspace.address_draft.clear();
                        self.clear_hub_session_owner();
                        self.persist_hub_connection_book();
                    }
                    None => self.hub_workspace.session.error = Some(HubSignInErrorCode::MalformedRequest),
                },
                hub_action::FORGET_CONNECTION => {
                    let selected = self.hub_workspace.book.selected_id.clone();
                    self.hub_workspace.book = remove_hub_connection(&self.hub_workspace.book, &selected);
                    let connection_id = self.hub_workspace.book.selected_id.clone();
                    self.hub_workspace.session = reduce_hub_session(&self.hub_workspace.session, &HubSessionEvent::SelectConnection { connection_id });
                    self.clear_hub_session_owner();
                    self.persist_hub_connection_book();
                }
                hub_action::CANCEL_SIGN_IN => {
                    if let Some(task) = self.hub_sign_in_task.take() {
                        task.cancel();
                    }
                    self.hub_workspace.session = reduce_hub_session(&self.hub_workspace.session, &HubSessionEvent::Failed { code: HubSignInErrorCode::Cancelled, retry_after_seconds: None });
                }
                hub_action::SIGN_IN => self.start_hub_sign_in(),
                hub_action::SIGN_OUT => self.run_hub_sign_out_turn().await,
                hub_action::REFRESH_SPACES => self.start_hub_spaces_reload(),
                hub_action::OPEN_SPACE => {
                    self.hub_workspace.open_space_id = (!space_id.is_empty()).then(|| space_id.clone());
                    self.reload_hub_members(&space_id).await;
                    self.open_hub_artifact_creation(&space_id).await;
                    if !space_id.is_empty() {
                        let uri = format!("/spaces/{space_id}");
                        self.push_uri(uri.clone());
                        let _ = self.apply_shell_uri(&uri).await;
                    }
                }
                hub_action::CREATE_SPACE => {
                    let name = self.hub_workspace.space_name_draft.clone();
                    if let Some(command) = crate::space_browser::create_space_command(&name, DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private) {
                        self.hub_workspace.phase = crate::space_browser::SpaceBrowserPhase::Submitting;
                        self.dispatch_directory_command(command).await;
                        self.hub_workspace.space_name_draft.clear();
                        self.start_hub_spaces_reload();
                    }
                }
                hub_action::CREATE_INVITE => self.run_hub_create_invite_turn(&space_id).await,
                hub_action::REDEEM_INVITE => self.run_hub_redeem_turn().await,
                hub_action::SELECT_ARTIFACT_KIND => {
                    let kind_id = args.as_ref().and_then(|args| args.get("kindId")).and_then(semio_framework_value::DslValue::as_str).unwrap_or_default();
                    let creation = &mut self.hub_workspace.creation;
                    if creation.catalog.as_ref().is_some_and(|catalog| catalog.kinds.iter().any(|kind| kind.kind_id == kind_id)) {
                        creation.kind_id = Some(kind_id.to_string());
                    }
                }
                hub_action::SET_ARTIFACT_NAME => self.hub_workspace.creation.name_draft = value,
                hub_action::CREATE_ARTIFACT => {
                    self.begin_hub_artifact_creation();
                    self.pump_hub_artifact_creation().await;
                }
                hub_action::CANCEL_ARTIFACT_CREATION => {
                    if let Some(operation) = self.hub_workspace.creation.operation.as_mut().filter(|operation| !hub_artifact_creation_terminal(operation.phase)) {
                        operation.cancel_requested = true;
                    }
                    self.pump_hub_artifact_creation().await;
                }
                hub_action::OPEN_CREATED_ARTIFACT => {
                    self.open_created_hub_artifact().await;
                }
                _ => {}
            }
            let _ = self.refresh_ui(UiDirtyScope::Full).await;
        })
    }

    /// 🌱️ Opens the creation door for `space_id`: a fresh door holding a minted idempotency key, then
    /// the space's selected current catalog. A hub with no ready catalog leaves the door
    /// `Unavailable`, saying so in both languages; local work never waits on it.
    async fn open_hub_artifact_creation(&mut self, space_id: &str) {
        self.hub_workspace.creation = HubArtifactCreationState { next_request_id: mint_directory_command_request_id(), ..HubArtifactCreationState::default() };
        let Some(client) = self.directory_client.clone().filter(|_| !space_id.is_empty()) else { return };
        self.hub_workspace.creation.catalog_phase = HubArtifactCatalogPhase::Loading;
        match client.space_artifact_creation_catalog(&self.directory_command_ctx(), space_id).await {
            Ok(catalog) => {
                self.hub_workspace.creation.catalog = Some(catalog);
                self.hub_workspace.creation.catalog_phase = HubArtifactCatalogPhase::Ready;
            }
            Err(_) => self.hub_workspace.creation.catalog_phase = HubArtifactCatalogPhase::Unavailable,
        }
    }

    /// 📥️ Seals the door's intent under its minted key and mints the next one at once, so no later
    /// click can resubmit this request id as a second creation. The frame pump submits it.
    fn begin_hub_artifact_creation(&mut self) {
        let Some(space_id) = self.hub_workspace.open_space_id.clone() else { return };
        let Some(intent) = hub_artifact_creation_intent(&self.hub_workspace) else { return };
        let now_ms = Self::directory_now_ms();
        let creation = &mut self.hub_workspace.creation;
        creation.next_request_id = mint_directory_command_request_id();
        creation.operation = Some(HubArtifactCreation {
            intent,
            space_id,
            phase: SpaceArtifactCreationPhaseV1::Accepted,
            submitted: false,
            cancel_requested: false,
            cancel_sent: false,
            ready: None,
            opening: HubArtifactOpening::Idle,
            last_answered_at_ms: now_ms,
            polls: 0,
            next_poll_at_ms: now_ms,
        });
    }

    /// 🌱️ Drives the one creation by at most one bounded hub request per frame — its submission, its
    /// cancellation, or a status poll after the shared backoff ([`crate::hub_connection::space_artifact_creation_poll_delay_ms`]) —
    /// and opens the artifact once the receipt is `Ready`. The creation is followed for as long as the hub
    /// answers; only a hub silent past the shared bound makes it `Indeterminate` (the hub may still finish
    /// it), never `Failed` — React's worker follows the same contract (`🌱️creation-polling`). A `409` on the
    /// submission means the catalog generation moved: the door re-reads the catalog instead of
    /// retrying. Answers whether the door changed.
    async fn pump_hub_artifact_creation(&mut self) -> bool {
        let Some(operation) = self.hub_workspace.creation.operation.clone() else { return false };
        if hub_artifact_creation_terminal(operation.phase) {
            if operation.ready.is_some() && operation.opening == HubArtifactOpening::Idle {
                self.open_created_hub_artifact().await;
                return true;
            }
            if operation.opening == HubArtifactOpening::Opening && !self.document_opening.as_ref().is_some_and(ShellDocumentOpening::running) {
                let opened = operation.ready.as_ref().is_some_and(|ready| self.sync_channel.as_ref().is_some_and(|channel| channel.document_id == ready.artifact_id));
                if let Some(current) = self.hub_workspace.creation.operation.as_mut() {
                    current.opening = if opened { HubArtifactOpening::Opened } else { HubArtifactOpening::Failed };
                }
                return true;
            }
            return false;
        }
        let Some(client) = self.directory_client.clone() else { return false };
        let now_ms = Self::directory_now_ms();
        if crate::hub_connection::space_artifact_creation_unreachable(operation.last_answered_at_ms, now_ms) {
            if let Some(current) = self.hub_workspace.creation.operation.as_mut() {
                current.phase = SpaceArtifactCreationPhaseV1::Indeterminate;
            }
            return true;
        }
        let request_id = operation.intent.request_id.as_str();
        let context = self.directory_command_ctx();
        let (receipt, submission, cancellation) = if !operation.submitted {
            (client.create_space_artifact(&context, &operation.space_id, &operation.intent).await, true, false)
        } else if operation.cancel_requested && !operation.cancel_sent {
            (client.cancel_space_artifact_creation(&context, &operation.space_id, request_id).await, false, true)
        } else if now_ms >= operation.next_poll_at_ms {
            (client.space_artifact_creation_status(&context, &operation.space_id, request_id).await, false, false)
        } else {
            return false;
        };
        let mut refresh_catalog = false;
        {
            let Some(current) = self.hub_workspace.creation.operation.as_mut().filter(|current| current.intent.request_id == operation.intent.request_id) else { return false };
            current.submitted = true;
            current.cancel_sent |= cancellation;
            let answered_at_ms = Self::directory_now_ms();
            current.next_poll_at_ms = answered_at_ms.saturating_add(crate::hub_connection::space_artifact_creation_poll_delay_ms(current.polls));
            current.polls = current.polls.saturating_add(1);
            if receipt.is_ok() {
                current.last_answered_at_ms = answered_at_ms;
            }
            match receipt {
                Ok(status) if status.catalog_generation_id == current.intent.expected_catalog_generation_id && status.ready.as_ref().is_none_or(|ready| ready.kind_id == current.intent.kind_id) => {
                    current.phase = status.phase;
                    current.ready = status.ready;
                }
                Ok(_) => current.phase = SpaceArtifactCreationPhaseV1::Failed,
                Err(DirectoryClientError::Http { status: 409, .. }) if submission => {
                    current.phase = SpaceArtifactCreationPhaseV1::Failed;
                    refresh_catalog = true;
                }
                Err(DirectoryClientError::Http { status, .. }) if (400..500).contains(&status) => current.phase = SpaceArtifactCreationPhaseV1::Failed,
                Err(DirectoryClientError::Unauthorized) => current.phase = SpaceArtifactCreationPhaseV1::Failed,
                Err(_) => {}
            }
        }
        if refresh_catalog {
            let space_id = operation.space_id.clone();
            let operation = self.hub_workspace.creation.operation.take();
            self.open_hub_artifact_creation(&space_id).await;
            self.hub_workspace.creation.operation = operation;
        }
        true
    }

    /// 🚪️ Opens a ready artifact through the ordinary document-open relay, bound to its space, with
    /// the dialect and schema the hub's receipt names; the relay resolves and mounts the owning app.
    /// The relay's open is frame-pumped, so the door stays `Opening` until it settles, and is `Opened`
    /// only when this shell's document binding then names the created artifact
    /// ([`Self::pump_hub_artifact_creation`]).
    async fn open_created_hub_artifact(&mut self) {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

        let Some((ready, space_id)) = self.hub_workspace.creation.operation.as_ref().and_then(|operation| operation.ready.clone().map(|ready| (ready, operation.space_id.clone()))) else { return };
        if let Some(operation) = self.hub_workspace.creation.operation.as_mut() {
            operation.opening = HubArtifactOpening::Opening;
        }
        let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: ready.parent_dialect.artifact_kind.clone(), standard: ready.parent_dialect.standard.clone(), subset: ready.parent_dialect.subset.clone() };
        let args = serde_json::json!({ "artifactRef": dialect.to_coordinate(), "documentId": ready.artifact_id, "schema": ready.artifact_schema, "spaceId": space_id });
        self.handle_open_artifact_relay("os.open-artifact", Some(&args)).await;
    }

    fn persist_hub_connection_book(&self) {
        prefs_set(HUB_CONNECTION_BOOK_STORAGE_KEY_V1, &serialize_hub_connection_book(&self.hub_workspace.book));
    }

    fn clear_hub_session_owner(&mut self) {
        if let Some(task) = self.hub_sign_in_task.take() {
            task.cancel();
        }
        if let Some(task) = self.hub_spaces_task.take() {
            task.cancel();
        }
        self.directory_client = None;
        #[cfg(not(target_arch="wasm32"))]
        self.cancel_inference_port();
        self.identity = None;
        self.verified_session_authority = None;
        self.hub_workspace.display_name = None;
    }

    fn project_verified_hub_authority(&mut self) {
        let Some(authority) = self.verified_session_authority.as_ref() else { return };
        let Some(client) = self.directory_client.as_ref() else { return };
        if client.base_url() != self.hub_workspace.origin() {
            return;
        }
        self.hub_workspace.session = reduce_hub_session(&self.hub_workspace.session, &HubSessionEvent::Minted { user_id: authority.user_id.clone(), expires_at_ms: Some(authority.expires_at) });
        self.hub_workspace.display_name = Some(authority.display_name.clone());
    }

    /// 🔐️ Arms one credential sign-in against the SELECTED hub's own origin: the mint, then the `me` read that carries
    /// the deadline the mint answer deliberately omits (AU1 §1.1, AU3 §4.2), run as a [`ShellHubTask`] so the frame keeps
    /// taking input while the hub hashes the password. A second submit while one runs is ignored (the form disables it).
    ///
    /// 🔑️ The password draft is cleared the moment the attempt takes it: the one request that needs it owns it.
    fn start_hub_sign_in(&mut self) {
        if self.hub_sign_in_task.is_some() {
            return;
        }
        let origin = self.hub_workspace.origin().to_string();
        let credential = HubSignInCredential {
            email: self.hub_workspace.email_draft.clone(),
            password: std::mem::take(&mut self.hub_workspace.password_draft),
            device_instance_id: shell_hub_device_instance_id(&self.shell_session_id),
            client_class: if cfg!(target_arch = "wasm32") { HubSignInClientClass::Browser } else { HubSignInClientClass::Native },
        };
        self.hub_workspace.session = reduce_hub_session(&self.hub_workspace.session, &HubSessionEvent::Submit);
        let mut ctx = self.directory_ctx();
        ctx.deadline_ms = Some(Self::directory_now_ms().saturating_add(HUB_SIGN_IN_DEADLINE_MS));
        let cancel = ctx.cancel.clone();
        let transport = self.directory_transport.clone();
        self.hub_sign_in_task = Some(ShellHubTask::spawn(cancel, async move {
            let result = match crate::hub_connection::run_hub_sign_in(&transport, &ctx, &origin, &credential).await {
                crate::hub_connection::HubSignInOutcome::Failed { code, retry_after_seconds } => return ShellHubSignInAnswer::Failed { code, retry_after_seconds },
                crate::hub_connection::HubSignInOutcome::Minted(result) => result,
            };
            let Ok(credential) = LocalHubCredential::from_minted_session(&origin, &result.token) else {
                return ShellHubSignInAnswer::Failed { code: HubSignInErrorCode::InvalidResponse, retry_after_seconds: None };
            };
            let credential = std::sync::Arc::new(credential);
            let client = std::sync::Arc::new(DirectoryClient::authenticated(transport, credential.clone()));
            match client.me(&ctx).await {
                Ok(authority) if authority.user_id == result.user_id => ShellHubSignInAnswer::Verified { origin, user_id: result.user_id, credential, client, authority },
                Ok(_) | Err(_) => ShellHubSignInAnswer::Unverified,
            }
        }));
    }

    /// 🔐️ Applies one sign-in answer: a verified session owns the directory client, the socket grants and the book's
    /// last user, then the spaces read starts on its own task; a session verified for a hub no longer selected is
    /// dropped; anything else is the session's localized error.
    fn apply_hub_sign_in(&mut self, answer: ShellHubSignInAnswer) {
        match answer {
            ShellHubSignInAnswer::Failed { code, retry_after_seconds } => {
                self.hub_workspace.session = reduce_hub_session(&self.hub_workspace.session, &HubSessionEvent::Failed { code, retry_after_seconds });
            }
            ShellHubSignInAnswer::Verified { origin, user_id, credential, client, authority } if origin == self.hub_workspace.origin() => {
                self.identity = Some(Identity { user_id: authority.user_id.clone(), email: authority.email.clone(), display_name: authority.display_name.clone(), hub_base_url: origin, issued_at_ms: chrome_now_ms() as i64 });
                self.verified_session_authority = Some(authority);
                self.directory_client = Some(client.clone());
                self.document_host.set_local_hub_credential(credential);
                self.document_host.set_hub_socket_grant_source(client);
                let selected_id = self.hub_workspace.book.selected_id.clone();
                if let Some(connection) = self.hub_workspace.book.connections.iter_mut().find(|connection| connection.id == selected_id) {
                    connection.last_user_id = Some(user_id);
                }
                self.persist_hub_connection_book();
                self.project_verified_hub_authority();
                self.start_hub_spaces_reload();
            }
            ShellHubSignInAnswer::Verified { .. } => {}
            ShellHubSignInAnswer::Unverified => {
                self.clear_hub_session_owner();
                self.hub_workspace.session = reduce_hub_session(&self.hub_workspace.session, &HubSessionEvent::Failed { code: HubSignInErrorCode::InvalidResponse, retry_after_seconds: None });
            }
        }
    }

    /// 🚪️ Sign-out. The in-memory capability is dropped whatever the hub answers, so an unreachable
    /// hub can never trap a human in a session they asked to leave.
    async fn run_hub_sign_out_turn(&mut self) {
        self.hub_workspace.session = reduce_hub_session(&self.hub_workspace.session, &HubSessionEvent::SignOut);
        if let Some(client) = self.directory_client.clone() {
            let _ = client.sign_out(&self.directory_command_ctx()).await;
        }
        self.clear_hub_session_owner();
        self.hub_workspace.rows.clear();
        self.hub_workspace.members.clear();
        self.hub_workspace.open_space_id = None;
        self.hub_workspace.session = reduce_hub_session(&self.hub_workspace.session, &HubSessionEvent::SignedOut);
    }

    /// 🎟️ One invitation, issued as a sealed `create-invite` command and read back from that
    /// command's OWN receipt — the only place the one-shot capability ever appears.
    async fn run_hub_create_invite_turn(&mut self, space_id: &str) {
        let Some(command) = crate::space_browser::create_invite_command(space_id, DirectorySpaceRole::Spectator, crate::space_browser::INVITE_TTL_CHOICES_SECS_V1[1]) else { return };
        if self.identity.is_none() || self.directory_client.is_none() {
            return;
        }
        let request = DirectoryCommandRequestV1::new(mint_directory_command_request_id(), command);
        let request_id = request.request_id.clone();
        self.hub_workspace.phase = crate::space_browser::SpaceBrowserPhase::Submitting;
        self.directory_commands.admit_first(request);
        self.flush_pending_directory_commands().await;
        match self.directory_commands.result(&request_id) {
            Some(ShellDirectoryCommandResultV1::Receipt(receipt)) => {
                self.hub_workspace.invite_capability = match &receipt.result {
                    DirectoryCommandResultV1::Invite { invite_token } => Some(invite_token.clone()),
                    DirectoryCommandResultV1::None => None,
                };
                self.hub_workspace.phase = crate::space_browser::SpaceBrowserPhase::Ready;
            }
            Some(ShellDirectoryCommandResultV1::Failed(_)) | None => self.hub_workspace.phase = crate::space_browser::SpaceBrowserPhase::Failed,
        }
    }

    /// 🎟️ Redeems the pasted capability against the selected hub, then reloads the list — a
    /// redemption that changed nothing visible would leave the human guessing.
    async fn run_hub_redeem_turn(&mut self) {
        let Some(token) = crate::space_browser::parse_invite_token(&self.hub_workspace.invite_draft) else {
            self.hub_workspace.redemption_error = Some(crate::space_browser::InviteRedemptionErrorCode::InvalidInvite);
            return;
        };
        let Some(client) = self.directory_client.clone() else {
            self.hub_workspace.redemption_error = Some(crate::space_browser::InviteRedemptionErrorCode::Unauthorized);
            return;
        };
        match client.redeem_invite(&self.directory_command_ctx(), &token).await {
            Ok(()) => {
                self.hub_workspace.redemption_error = None;
                self.hub_workspace.invite_draft.clear();
                self.start_hub_spaces_reload();
            }
            Err(DirectoryClientError::Unauthorized) => self.hub_workspace.redemption_error = Some(crate::space_browser::InviteRedemptionErrorCode::Unauthorized),
            Err(DirectoryClientError::Http { status, .. }) => self.hub_workspace.redemption_error = Some(crate::space_browser::invite_redemption_error_from_status(status)),
            Err(DirectoryClientError::Cancelled | DirectoryClientError::Transport(semio_framework_os_kernel::os_directory::client::TransportError::Cancelled)) => {
                self.hub_workspace.redemption_error = Some(crate::space_browser::InviteRedemptionErrorCode::Cancelled)
            }
            Err(DirectoryClientError::Decode(_)) => self.hub_workspace.redemption_error = Some(crate::space_browser::InviteRedemptionErrorCode::InvalidInvite),
            Err(_) => self.hub_workspace.redemption_error = Some(crate::space_browser::InviteRedemptionErrorCode::Unreachable),
        }
    }

    /// 🏘️ Reloads the space list through the shell's own directory client. A hub that stops
    /// answering leaves the rows on screen and moves the phase to `Stale` rather than emptying a
    /// list the human was reading.
    /// 🏘️ Arms one spaces read on a [`ShellHubTask`] (a replaced read is cancelled); the list shows `Loading` and keeps
    /// the rows it had, which is the local-first rule the `Stale` phase already defends.
    fn start_hub_spaces_reload(&mut self) {
        if let Some(task) = self.hub_spaces_task.take() {
            task.cancel();
        }
        let Some(client) = self.directory_client.clone() else {
            self.hub_workspace.phase = crate::space_browser::SpaceBrowserPhase::Stale;
            return;
        };
        self.hub_workspace.phase = crate::space_browser::SpaceBrowserPhase::Loading;
        let ctx = self.directory_ctx();
        let cancel = ctx.cancel.clone();
        self.hub_spaces_task = Some(ShellHubTask::spawn(cancel, async move { client.spaces(&ctx).await.map(|entries| crate::space_browser::space_rows(&entries)).map_err(|_| ()) }));
    }

    /// 📬️ Reads the answers of the hub workspace's spawned requests, once each; answers whether the workspace changed.
    pub(crate) fn poll_hub_workspace_tasks(&mut self) -> bool {
        let mut changed = false;
        if let Some(answer) = self.hub_sign_in_task.as_ref().and_then(ShellHubTask::answer) {
            self.hub_sign_in_task = None;
            self.apply_hub_sign_in(answer.unwrap_or(ShellHubSignInAnswer::Failed { code: HubSignInErrorCode::Unreachable, retry_after_seconds: None }));
            changed = true;
        }
        if let Some(answer) = self.hub_spaces_task.as_ref().and_then(ShellHubTask::answer) {
            self.hub_spaces_task = None;
            match answer.and_then(|rows| rows) {
                Ok(rows) => {
                    self.hub_workspace.rows = rows;
                    self.hub_workspace.phase = crate::space_browser::SpaceBrowserPhase::Ready;
                }
                Err(()) => self.hub_workspace.phase = crate::space_browser::SpaceBrowserPhase::Stale,
            }
            changed = true;
        }
        changed
    }

    /// 🔐️ Whether no hub workspace request is in flight.
    pub(crate) fn hub_workspace_settled(&self) -> bool {
        self.hub_sign_in_task.is_none() && self.hub_spaces_task.is_none()
    }

    /// 👥️ The open space's roster, joined with the hub identities this shell's presence lane sees.
    /// A public page carries no roster at all, which is an EMPTY roster and never an error.
    pub(crate) async fn reload_hub_members(&mut self, space_id: &str) {
        if space_id.is_empty() {
            self.hub_workspace.members.clear();
            return;
        }
        let Some(client) = self.directory_client.clone() else { return };
        let Ok(page) = client.space_administration_page(&self.directory_ctx(), space_id, None).await else {
            self.hub_workspace.members.clear();
            return;
        };
        let rows = match page.page() {
            DirectorySpaceAdministrationPageV1::Member { members, .. } | DirectorySpaceAdministrationPageV1::Author { members, .. } => members.rows.clone(),
            DirectorySpaceAdministrationPageV1::Public { .. } => Vec::new(),
        };
        self.hub_workspace.members = crate::space_browser::space_member_presence(&rows, &self.hub_online_user_ids());
    }

    /// 👥️ The hub identities the presence lane currently sees. A peer with no hub identity
    /// contributes nothing, so the roster can never show a member the directory does not know.
    fn hub_online_user_ids(&self) -> Vec<String> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let mut ids: Vec<String> = self.presence_peers.iter().filter_map(|peer| peer.user_id.clone()).collect();
            ids.sort_unstable();
            ids.dedup();
            return ids;
        }
        #[cfg(target_arch = "wasm32")]
        Vec::new()
    }
    //#endregion 🔐️HubWorkspaceLane

    //#region 💡️InferencePort
    /// 📦 Replaces the executable's finite installed owner inventory and retires prior work.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn install_document_services(&mut self, services: Vec<InstalledServiceContributionV1>) {
        self.cancel_inference_port();
        if services.len()>64 || services.iter().enumerate().any(|(index,entry)|services[..index].iter().any(|prior|prior.owner==entry.owner && prior.service_id==entry.service_id)) {self.document_services.clear();return;}
        self.document_services=services;
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn installed_document_http_port(&self, owner:&str, service_id:&str) -> Option<CompiledDocumentHttpPortV1> {
        let mut found=None;
        for plugin in &self.plugins {
            if plugin.plugin_id!=owner {continue;}
            for contribution in &plugin.manifest.topic_contributions {
                if contribution.topic!=DOCUMENT_HTTP_PORT_TOPIC {continue;}
                let declaration=DocumentHttpPortDeclarationV1::from_value(contribution.payload.clone()).ok()?;
                if declaration.service_id!=service_id || declaration.owner!=owner {continue;}
                if found.is_some() {return None;}
                found=Some(CompiledDocumentHttpPortV1::compile(owner,declaration).ok()?);
            }
        }
        found
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn open_service_port(&mut self, owner:&str,service_id:&str,action:&str,payload:DslValue) {
        if self.session.as_ref().map(|session|session.plugin_id.as_str())!=Some(owner) {return;}
        if let Some(runner)=self.inference_port.as_ref().filter(|runner|runner.owner==owner && runner.service_id==service_id) {
            runner.intend(action,payload);
            return;
        }
        self.cancel_inference_port();
        let Some(lease)=self.document_execution_target_lease.as_ref() else {return};
        if lease.package.plugin_id!=owner {return;}
        let Some(space_id)=self.open_space_id.clone() else {return};
        let Some(document_id)=self.sync_channel.as_ref().map(|channel|channel.document_id.clone()) else {return};
        let scope=DocumentScope {space_id,document_id};
        if lease.scope!=scope {return;}
        let mut matches=self.document_services.iter().filter(|entry|entry.owner==owner && entry.service_id==service_id);
        let Some(contribution)=matches.next().copied() else {return};
        if matches.next().is_some() {return;}
        let Some(port)=self.installed_document_http_port(owner,service_id) else {return};
        let Some(client)=self.directory_client.clone().filter(|_|self.identity.is_some()) else {return};
        let mut driver=(contribution.create)(scope.clone(),true);
        if driver.intend(action,payload).is_err() {return;}
        self.inference_port_status=Some(InstalledServiceStatusV1 {owner:owner.into(),service_id:service_id.into(),payload:driver.status()});
        self.inference_port=Some(ShellInferenceRunner::start(crate::renderer_worker_pool(),client,std::sync::Arc::new(port),self.directory_ctx(),contribution,scope,driver));
    }

    /// 🔄️ One bounded drain per frame; the runner never spins and never blocks the render loop.
    #[cfg(not(target_arch = "wasm32"))]
    fn pump_inference_port(&mut self) -> bool {
        let Some(runner) = self.inference_port.clone() else { return false };
        let statuses = runner.drain();
        let Some(latest) = statuses.into_iter().next_back() else { return false };
        let terminal = runner.driver.lock().expect("service driver mutex poisoned").terminal();
        self.inference_port_status = Some(latest);
        if terminal {
            runner.cancel();
            self.inference_port = None;
        }
        true
    }

    /// 🎬 Delivers a scoped action only to the retained installed owner.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn intend_document_service(&mut self,owner:&str,service_id:&str,action:&str,payload:DslValue) {
        if let Some(runner)=self.inference_port.as_ref().filter(|runner|runner.owner==owner && runner.service_id==service_id) {
            runner.intend(action,payload);
        }
    }

    /// 🛑️ Hard terminal: cancels every in-flight call and retires the runner.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn cancel_inference_port(&mut self) {
        self.inference_port_status=None;
        if let Some(runner) = self.inference_port.take() {
            runner.cancel();
        }
    }
    //#endregion 💡️InferencePort

    /// 📂️ Opens the relay's exact `documentId`/`schema` pair with `spaceId` pinning
    /// `open_space_id` first so default binding computation sees it. An app-only relay is a no-op.
    ///
    /// 🌐️ One body on both targets: resolving the owner of an artifact kind, installing it on demand
    /// (O2's `install_plugin`, whose progress + cancel band the chrome paints), switching the
    /// session to its app and binding that session to its document on the shell's `ArtifactHost`.
    /// A document the host cannot serve is refused out loud (`open-artifact.document-failed`), never
    /// dropped: the caller's `documentId` either binds or explains itself.
    ///
    /// 🎬️ The relay used to open every artifact inside whatever session happened to be mounted, so
    /// a hub that was showing `home` opened a cad document into `home`. The target's own app ref —
    /// or, when the caller has none, the kind's declared owner — decides the session, and the owner
    /// is installed on demand exactly as React's `openArtifactWithAppRef` installs it.
    async fn handle_open_artifact_relay(&mut self, action_id: &str, args: Option<&Value>) {
        let target = match open_artifact_relay_target(action_id, args) {
            Ok(target) => target,
            Err(error) => {
                Self::debug_log(&format!("[TRACE] wgpu shell os.open-artifact relay rejected: {error}"));
                return;
            }
        };
        if self.document_opening.as_ref().is_some_and(ShellDocumentOpening::running) {
            let is_de = self.locale_id == "de";
            self.show_transient_notice(shell_chrome_string("document.open.busy", is_de), semio_framework::Severity::Info, Some("document.open.busy"));
            return;
        }
        if let Some(space_id) = target.space_id.clone() {
            self.open_space_id = Some(space_id);
        }
        #[cfg(not(target_arch = "wasm32"))]
        if self.begin_document_resolution(&target) {
            return;
        }
        let switch = match (&target.plugin_id, &target.app_id) {
            (Some(plugin_id), Some(app_id)) => match self.install_plugin(plugin_id).await {
                Err(error) => {
                    Self::debug_log(&format!("[TRACE] wgpu shell os.open-artifact could not install {plugin_id}: {error}"));
                    None
                }
                Ok(()) => self.plugins.iter().find(|entry| entry.plugin_id == *plugin_id).and_then(|program| program.manifest.apps.iter().find(|app| app.id == *app_id).cloned()).map(|app| (plugin_id.clone(), app)),
            },
            _ => self.resolve_activation_owner_app(&target.dialect, target.role).await,
        };
        let document = match (target.document_id, target.schema) {
            (Some(document_id), Some(schema)) => Some(ShellDocumentOpenTarget { document_id, schema, checkpoint: None }),
            _ => None,
        };
        match switch {
            Some((plugin_id, app)) if !self.session.as_ref().is_some_and(|session| session.plugin_id == plugin_id && session.app.id == app.id) => {
                let Some(program) = self.plugins.iter().find(|entry| entry.plugin_id == plugin_id).cloned() else {
                    Self::debug_log(&format!("[TRACE] wgpu shell os.open-artifact could not switch to {plugin_id}: program missing"));
                    return;
                };
                let (terminology, locale) = (self.active_terminology(), self.active_locale());
                let label = app.label.resolve(terminology, locale).to_string();
                let app_id = app.id.clone();
                let pending = ShellDetached::spawn(async move { program.create_app(&app_id).await.map(ShellDocumentOpenAnswer::Instantiated) });
                self.document_opening = Some(ShellDocumentOpening::new(label, ShellDocumentOpenPhase::Instantiating, plugin_id, app.id, document, Some(pending)));
            }
            _ => {
                if let Some(document) = document {
                    self.begin_document_seed(document).await;
                }
            }
        }
    }

    /// 🌱️ Starts the document half of an open in the mounted session: the host half is prepared on
    /// this turn, and the guest's genesis load runs detached; [`Self::advance_document_opening`] binds the
    /// document once it answers. A preparation that fails settles the open as failed.
    async fn begin_document_seed(&mut self, document: ShellDocumentOpenTarget) {
        let Some(session) = self.session.clone() else {
            self.fail_document_opening("document open has no mounted session".to_string());
            return;
        };
        let label = session.app.label.resolve(self.active_terminology(), self.active_locale()).to_string();
        let opening = self.document_opening.take();
        let (bindings, surface) = self.default_bindings_for_current_session();
        let prepared = match self.prepare_document_open(document.document_id, document.schema, bindings, surface, None, document.checkpoint).await {
            Ok(prepared) => prepared,
            Err(error) => {
                self.document_opening = opening;
                self.fail_document_opening(error);
                return;
            }
        };
        let seed_cancel = prepared.seed.as_ref().map(|seed| seed.ctx.cancel.clone());
        let pending = ShellDetached::spawn({
            let (plugin, instance_id, seed) = (prepared.plugin.clone(), prepared.session.instance_id, prepared.seed.clone());
            async move { seed_hub_document(plugin, instance_id, seed).await.map(ShellDocumentOpenAnswer::Seeded).map_err(crate::program_bridge::ProgramFault::from) }
        });
        let mut opening = opening.unwrap_or_else(|| ShellDocumentOpening::new(label, ShellDocumentOpenPhase::Seeding, session.plugin_id.clone(), session.app.id.clone(), None, None));
        opening.phase = ShellDocumentOpenPhase::Seeding;
        opening.cancel = seed_cancel;
        opening.pending = Some(pending);
        opening.prepared = Some(prepared);
        self.document_opening = Some(opening);
    }

    /// 🧩️ Starts a hub document's open by resolving the component it runs by the SERVING catalog
    /// generation (binding decision of ticket 26/09/23, session 11 13:1x; React resolves the same generation
    /// through the hub's plugin-module routes): the local program only when its content hash is the lease's,
    /// else the verified store entry, else the hub's own execution-target bytes, verified and stored
    /// ([`DirectoryClient::resolve_execution_target_module`]) — all detached, with a cancellable band.
    /// The lease names the plugin and app that open the document, so a relay that names none (the creation
    /// door's, a Space index row's) resolves exactly like one that does. Answers `false` for a relay that names
    /// no hub document, which opens as before.
    #[cfg(not(target_arch = "wasm32"))]
    fn begin_document_resolution(&mut self, target: &OpenArtifactRelayTarget) -> bool {
        let (Some(document_id), Some(schema), Some(space_id), Some(client)) = (target.document_id.clone(), target.schema.clone(), self.open_space_id.clone(), self.directory_client.clone()) else {
            return false;
        };
        let local_app = target.app_id.as_ref().and_then(|app_id| self.plugins.iter().flat_map(|program| program.manifest.apps.iter()).find(|app| app.id == *app_id).cloned());
        let surface_id = local_app.as_ref().map_or_else(|| semio_framework::manifest::surface_app_id(&target.dialect, target.role), |app| semio_framework::manifest::surface_app_id(&app.dialect, app.role));
        let label = local_app.map_or_else(|| target.app_id.clone().unwrap_or_else(|| target.artifact_ref.clone()), |app| app.label.resolve(self.active_terminology(), self.active_locale()).to_string());
        let local_shas: Vec<(String, Option<String>)> = self.plugins.iter().map(|program| (program.plugin_id.clone(), program.component_sha256.clone())).collect();
        let intent = semio_framework_os_kernel::os_directory::DocumentOpenIntentV1 {
            schema: "semio.hub.document-open-intent/v1".into(),
            version: 1,
            scope: semio_framework_os_kernel::os_directory::DocumentScope::new(space_id.as_str(), document_id.as_str()),
            requested_surface_id: Some(surface_id),
            client_instance_id: format!("wgpu-shell-{}", self.shell_session_id),
        };
        let ctx = self.directory_ctx();
        let cancel = ctx.cancel.clone();
        let resolve_step = std::sync::Arc::new(std::sync::atomic::AtomicU8::new(0));
        let reported = resolve_step.clone();
        let expected_plugin = target.plugin_id.clone();
        let pending = ShellDetached::spawn(async move {
            let store = semio_framework_os_kernel::os_directory::client::ExecutionTargetModuleStore::for_user();
            reported.store(document_open_resolve_step_code(semio_framework_os_kernel::os_directory::client::ExecutionTargetModuleStep::Lease), std::sync::atomic::Ordering::Release);
            let lease = client.document_execution_target_lease(&ctx, &intent).await.map_err(|error| format!("document execution target: {error}"))?;
            let plugin_id = lease.package.plugin_id.clone();
            if expected_plugin.as_ref().is_some_and(|expected| *expected != plugin_id) {
                return Err(crate::program_bridge::ProgramFault::from(format!("the hub opens this document with {plugin_id}, not {}", expected_plugin.unwrap_or_default())));
            }
            let local_sha = local_shas.into_iter().find(|(local, _)| *local == plugin_id).and_then(|(_, sha)| sha);
            let resolved = client
                .resolve_execution_target_files(&ctx, &intent, lease, local_sha.as_deref(), &store, |step| reported.store(document_open_resolve_step_code(step), std::sync::atomic::Ordering::Release))
                .await
                .map_err(|error| format!("document execution target: {error}"))?;
            let app_id = resolved.lease.surface.app_id.clone();
            let program = match resolved.files {
                Some(files) => Some(crate::program_bridge::load_resolved_program(&plugin_id, &files.component, &files.descriptor, &resolved.lease.package.component_receipt).await?),
                None => None,
            };
            Ok::<ShellDocumentOpenAnswer, crate::program_bridge::ProgramFault>(ShellDocumentOpenAnswer::Resolved { plugin_id, app_id, program, checkpoint: resolved.lease.checkpoint.clone() })
        });
        let mut opening = ShellDocumentOpening::new(
            label,
            ShellDocumentOpenPhase::Resolving,
            target.plugin_id.clone().unwrap_or_default(),
            target.app_id.clone().unwrap_or_default(),
            Some(ShellDocumentOpenTarget { document_id, schema, checkpoint: None }),
            Some(pending),
        );
        opening.cancel = Some(cancel);
        opening.resolve_step = resolve_step;
        self.document_opening = Some(opening);
        true
    }

    /// 🧩️ Continues a resolved hub open: a hub-resolved program replaces the local one of its plugin (a local
    /// program no longer mounts a hub document it is not the lease's component for), then the app instance
    /// is created on it; a local program that already is the lease's component seeds the document into the
    /// mounted session when it runs the app already.
    #[cfg(not(target_arch = "wasm32"))]
    async fn continue_resolved_open(&mut self, mut opening: ShellDocumentOpening, resolved: Option<ProgramBridgeEntry>) {
        let fresh = resolved.is_some();
        if let Some(program) = resolved {
            self.plugins.retain(|entry| entry.plugin_id != program.plugin_id);
            self.plugins.push(program);
        }
        let Some(program) = self.plugins.iter().find(|entry| entry.plugin_id == opening.plugin_id).cloned() else {
            self.document_opening = Some(opening);
            self.fail_document_opening("the resolved plugin is not mounted".to_string());
            return;
        };
        let Some(app) = program.manifest.apps.iter().find(|app| app.id == opening.app_id).cloned() else {
            let reason = format!("{} declares no app {}", program.plugin_id, opening.app_id);
            self.document_opening = Some(opening);
            self.fail_document_opening(reason);
            return;
        };
        opening.label = app.label.resolve(self.active_terminology(), self.active_locale()).to_string();
        if !fresh && self.session.as_ref().is_some_and(|session| session.plugin_id == program.plugin_id && session.app.id == app.id) {
            let document = opening.document.take();
            self.document_opening = Some(opening);
            if let Some(document) = document {
                self.begin_document_seed(document).await;
            }
            return;
        }
        let app_id = app.id.clone();
        let (terminology, locale) = (self.active_terminology(), self.active_locale());
        opening.pending = Some(ShellDetached::spawn(async move { program.create_app(&app_id).await.map(ShellDocumentOpenAnswer::Instantiated) }));
        opening.phase = ShellDocumentOpenPhase::Instantiating;
        self.document_opening = Some(opening);
    }

    /// ⏭️ Advances the retained open by the one detached step that answered, if any: an instance mounts
    /// as the session (or is destroyed when the open was cancelled), a seeded document binds, and every
    /// render the open owes goes to the settle lane. Called once per frame from [`Self::pump_sync_events`];
    /// answers whether the open moved.
    pub(crate) async fn advance_document_opening(&mut self) -> bool {
        let Some(answer) = self.document_opening.as_ref().and_then(|opening| opening.pending.as_ref()).and_then(ShellDetached::take) else { return false };
        let Some(mut opening) = self.document_opening.take() else { return false };
        opening.pending = None;
        match answer {
            #[cfg(not(target_arch = "wasm32"))]
            Ok(ShellDocumentOpenAnswer::Resolved { plugin_id, app_id, program, checkpoint }) => {
                if opening.cancel_requested {
                    opening.phase = ShellDocumentOpenPhase::Cancelled;
                    self.document_opening = Some(opening);
                    return true;
                }
                opening.plugin_id = plugin_id;
                opening.app_id = app_id;
                if let Some(document) = opening.document.as_mut() {
                    document.checkpoint = Some(checkpoint);
                }
                self.continue_resolved_open(opening, program).await;
            }
            Ok(ShellDocumentOpenAnswer::Instantiated(instance_id)) => {
                if opening.cancel_requested {
                    if let Some(program) = self.plugins.iter().find(|entry| entry.plugin_id == opening.plugin_id) {
                        program.destroy_app(instance_id);
                    }
                    opening.phase = ShellDocumentOpenPhase::Cancelled;
                    self.document_opening = Some(opening);
                    return true;
                }
                let Some(app) = self.plugins.iter().find(|entry| entry.plugin_id == opening.plugin_id).and_then(|program| program.manifest.apps.iter().find(|app| app.id == opening.app_id).cloned()) else {
                    self.document_opening = Some(opening);
                    self.fail_document_opening("the opened app left its plugin".to_string());
                    return true;
                };
                self.install_app_session(&opening.plugin_id, app, instance_id);
                self.owe_refresh(UiDirtyScope::Full);
                self.owe_settle();
                match opening.document.take() {
                    Some(document) => {
                        self.document_opening = Some(opening);
                        self.begin_document_seed(document).await;
                    }
                    None => self.document_opening = None,
                }
            }
            Ok(ShellDocumentOpenAnswer::Seeded(pair)) => {
                let Some(prepared) = opening.prepared.take() else {
                    self.document_opening = Some(opening);
                    self.fail_document_opening("seeded document open lost its prepared owner".to_string());
                    return true;
                };
                if opening.cancel_requested {
                    opening.phase = ShellDocumentOpenPhase::Cancelled;
                    self.document_opening = Some(opening);
                    return true;
                }
                if let Err(error) = self.install_document_seed(&prepared, pair) {
                    self.document_opening = Some(opening);
                    self.fail_document_opening(error);
                    return true;
                }
                match self.finish_document_open(prepared).await {
                    Ok(()) => {
                        self.owe_refresh(UiDirtyScope::Full);
                        self.owe_settle();
                    }
                    Err(error) => {
                        self.document_opening = Some(opening);
                        self.fail_document_opening(error);
                        return true;
                    }
                }
            }
            Err(_) if opening.cancel_requested => {
                opening.phase = ShellDocumentOpenPhase::Cancelled;
                self.document_opening = Some(opening);
            }
            Err(error) => {
                self.document_opening = Some(opening);
                let detail = self.note_refused_open(error);
                self.fail_document_opening(detail);
            }
        }
        true
    }

    /// 🛟️ Drives a rebootstrapping hub document back to Live — the same canonical-pair seed as its first open
    /// ([`seed_hub_document`]): the pair is fetched detached and admitted only as the actor's control's checkpoint, loaded into
    /// the guest, and handed to the actor ([`ArtifactActorMsg::Reseed`]), which then dials at once. A transient failure is asked
    /// again after [`SYNC_RESEED_RETRY_MS`] while the document stays open; closing the document drops the reseed. Answers
    /// whether the guest's document changed.
    pub(crate) async fn advance_sync_reseed(&mut self) -> bool {
        let Some(mut reseed) = self.sync_reseed.take() else { return false };
        let Some(owner) = self.sync_channel.as_ref().map(shell_sync_channel_owner) else { return false };
        match reseed.pending.as_ref().and_then(ShellDetached::take) {
            None if reseed.pending.is_some() => {
                self.sync_reseed = Some(reseed);
                false
            }
            None => {
                if chrome_now_ms() < reseed.retry_at_ms {
                    self.sync_reseed = Some(reseed);
                    return false;
                }
                let Some(client) = self.directory_client.clone() else {
                    reseed.retry_at_ms = chrome_now_ms() + SYNC_RESEED_RETRY_MS;
                    self.sync_reseed = Some(reseed);
                    return false;
                };
                let (ctx, control) = (self.directory_ctx(), reseed.control.clone());
                reseed.pending = Some(ShellDetached::spawn(async move { client.rebootstrap_canonical_checkpoint_pair(&ctx, &control).await.map_err(|error| format!("canonical checkpoint pair: {error}")) }));
                self.sync_reseed = Some(reseed);
                false
            }
            Some(Err(error)) => {
                Self::debug_log(&format!("[TRACE] wgpu shell rebootstrap reseed refused: {error}"));
                reseed.pending = None;
                reseed.retry_at_ms = chrome_now_ms() + SYNC_RESEED_RETRY_MS;
                self.sync_reseed = Some(reseed);
                false
            }
            Some(Ok(pair)) => {
                let Some(plugin) = self.plugins.iter().find(|entry| entry.plugin_id == owner.plugin_id).cloned() else { return false };
                if let Err(error) = plugin.load_app_document_pack(owner.instance_id, &pair.pack_bytes, &pair.spr_bytes).await {
                    let detail = self.note_refused_open(error);
                    Self::debug_log(&format!("[TRACE] wgpu shell rebootstrap reseed load failed: {detail}"));
                    reseed.pending = None;
                    reseed.retry_at_ms = chrome_now_ms() + SYNC_RESEED_RETRY_MS;
                    self.sync_reseed = Some(reseed);
                    return false;
                }
                let message = ArtifactActorMsg::Reseed { pack: pair.pack_bytes, spr: pair.spr_bytes, baseline: pair.baseline_frontier };
                if let Some(channel) = self.sync_channel.as_ref() {
                    let _ = channel.cmd_tx.send(message);
                }
                self.owe_refresh(UiDirtyScope::Full);
                true
            }
        }
    }

    /// 🧯️ Settles the open as failed, out loud: its band keeps the reason and the notice names it.
    fn fail_document_opening(&mut self, error: String) {
        Self::debug_log(&format!("[TRACE] wgpu shell os.open-artifact relay failed: {error}"));
        let is_de = self.locale_id == "de";
        self.show_transient_notice(shell_chrome_string("open-artifact.document-failed", is_de), semio_framework::Severity::Warning, Some("open-artifact.document-failed"));
        if let Some(opening) = self.document_opening.as_mut() {
            opening.phase = ShellDocumentOpenPhase::Failed(error);
            opening.pending = None;
            opening.prepared = None;
        }
    }

    /// 🛑️ Cancels the open in flight, or clears a settled record. The phase moves only when the step in
    /// flight answers: an instance created after the cancel is destroyed, a seeded document never binds.
    pub fn cancel_document_opening(&mut self) {
        if self.document_opening.as_ref().is_some_and(|opening| !opening.running()) {
            self.document_opening = None;
            return;
        }
        if let Some(opening) = self.document_opening.as_mut() {
            opening.cancel_requested = true;
            if let Some(cancel) = opening.cancel.as_ref() {
                cancel.cancel_now();
            }
        }
    }

    /// 📔️ Live directory WS events reach the mounted Space index through `foldDirectoryEvents`
    /// (browser parity), so its membership stays projected. Home is never folded: its projection's
    /// only writer is the sealed page lane, which the same events wake through `home.wake` below.
    #[cfg(not(target_arch = "wasm32"))]
    fn dispatch_fold_directory_events(&mut self, events: &[semio_framework_os_kernel::os_directory::DirectoryEvent]) {
        let Some(session) = self.session.clone().filter(|session| session_is_space_index(session, &self.plugins)) else { return };
        let Some(program) = self.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).cloned() else { return };
        let events_json = semio_framework_pack_json::to_json_string(&events.to_vec());
        let live_view_state = self.live_view_state(&session);
        let window_kind_id = session.app.window_kinds.first().id.clone();
        let window_instance_id = live_view_state.window_id.clone().unwrap_or_else(|| window_kind_id.clone());
        let mode_id = live_view_state.active_mode_id.clone().unwrap_or_else(|| session.app.default_mode_id.clone());
        let invocation = semio_framework::manifest::ActionInvocation {
            address: semio_framework::manifest::ActionAddress { plugin_id: session.plugin_id.clone(), app_id: session.app.id.clone(), mode_id, window_kind_id, window_instance_id, action_id: "foldDirectoryEvents".into() },
            arguments: BTreeMap::from([("eventsJson".into(), semio_framework_value::DslValue::String(events_json))]),
        };
        let action_json = semio_framework_pack_json::to_json_string(&invocation);
        let instance_id = session.instance_id;
        self.submit_shell_io_future(async move {
            ShellIoCompletion::Program(program.handle_action(instance_id, &action_json, &live_view_state).await.map(|_| ()))
        });
    }

    /// 🏠️ Begins one bounded canonical-page fetch owned by the retained Home bootstrap epoch.
    #[cfg(not(target_arch = "wasm32"))]
    fn start_directory_event_page_fetch(&mut self) {
        if self.identity_offline {
            return;
        }
        let Some(client) = self.directory_client.clone() else { return };
        let pool = crate::renderer_worker_pool();
        let now_ms = pool.now_ms();
        let Some((epoch, after)) = self
            .directory_home
            .as_ref()
            .and_then(|home| (!home.closed && home.page_task.is_none() && home.home_task.is_none() && home.stream.is_none() && now_ms >= home.retry_at_ms).then_some((home.bootstrap.bootstrap_epoch(), home.bootstrap.after())))
        else {
            return;
        };
        let mut context = self.directory_ctx();
        context.deadline_ms = Some(now_ms.saturating_add(5_000));
        let cancel = context.cancel.clone();
        let (tx, rx) = std::sync::mpsc::channel();
        let task = ShellPoolFuture::spawn(pool, Lane::Io, async move {
            let result = client.event_page(&context, after).await;
            let _ = tx.send((epoch, result));
        });
        if let Some(home) = self.directory_home.as_mut().filter(|home| home.bootstrap.bootstrap_epoch() == epoch && !home.closed) {
            home.page_cancel = Some(cancel);
            home.page_rx = Some(rx);
            home.page_task = Some(task);
        } else {
            cancel.cancel_now();
            task.cancel();
        }
    }

    /// 🧾️ Invokes the stored Home instance and accepts only its terminal Config publication receipt.
    #[cfg(not(target_arch = "wasm32"))]
    fn start_directory_home_publication(&mut self, page: CanonicalDirectoryEventPageV1) -> Result<(), String> {
        let canonical_json = page.canonical_json().to_string();
        let session_identity = self.identity.as_ref().map(|identity| ViewSessionIdentity { user_id: identity.user_id.clone(), display_name: identity.display_name.clone() });
        let (epoch, instance_id, program, app, view_state, expected) = {
            let home = self.directory_home.as_mut().ok_or("retained Home projection is unavailable")?;
            let expected = home.bootstrap.present(page).map_err(|error| error.to_string())?;
            let mut view_state = self.session.as_ref().filter(|session| home.is_instance(&session.plugin_id, session.instance_id)).map(|session| session.view_state.clone()).unwrap_or_else(|| home.view_state.clone());
            view_state.session_identity = session_identity;
            let program = self.plugins.iter().find(|entry| entry.plugin_id == home.plugin_id).cloned().ok_or("retained Home program is unavailable")?;
            (home.bootstrap.bootstrap_epoch(), home.instance_id, program, home.app.clone(), view_state, expected)
        };
        let window_kind_id = app.window_kinds.first().id.clone();
        let window_instance_id = view_state.window_id.clone().unwrap_or_else(|| window_kind_id.clone());
        let invocation = semio_framework::manifest::ActionInvocation {
            address: semio_framework::manifest::ActionAddress {
                plugin_id: program.plugin_id.clone(),
                app_id: app.id.clone(),
                mode_id: view_state.active_mode_id.clone().unwrap_or_else(|| app.default_mode_id.clone()),
                window_kind_id,
                window_instance_id,
                action_id: "applyDirectoryEventPage".into(),
            },
            arguments: BTreeMap::from([("pageJson".into(), semio_framework_value::DslValue::String(canonical_json))]),
        };
        let action_json = semio_framework_pack_json::to_json_string(&invocation);
        let (tx, rx) = std::sync::mpsc::channel();
        let expected_for_task = expected.clone();
        let pool = crate::renderer_worker_pool();
        let task = ShellPoolFuture::spawn(pool, Lane::Io, async move {
            let outcome = match program.handle_action(instance_id, &action_json, &view_state).await {
                Err(error) => ShellDirectoryHomePublicationOutcome::Rejected(error),
                Ok(result) => match terminal_directory_home_ack(&result, epoch) {
                    Ok(actual) if actual == expected_for_task => ShellDirectoryHomePublicationOutcome::Published(actual),
                    Ok(_) => ShellDirectoryHomePublicationOutcome::Terminal("retained Home directory acknowledgement does not match the pending page".into()),
                    Err(error) => ShellDirectoryHomePublicationOutcome::Terminal(error),
                },
            };
            let _ = tx.send((epoch, instance_id, expected_for_task, outcome));
        });
        let home = self.directory_home.as_mut().ok_or("retained Home projection retired before publication admission")?;
        if home.bootstrap.bootstrap_epoch() != epoch || home.instance_id != instance_id || home.closed {
            task.cancel();
            return Err("retained Home projection changed before publication admission".into());
        }
        home.home_rx = Some(rx);
        home.home_task = Some(task);
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn poll_directory_page_result(&mut self) -> Option<ShellDirectoryPageResult> {
        let home = self.directory_home.as_mut()?;
        let result = match home.page_rx.as_ref()?.try_recv() {
            Ok(result) => Some(result),
            Err(std::sync::mpsc::TryRecvError::Empty) => None,
            Err(std::sync::mpsc::TryRecvError::Disconnected) => Some((home.bootstrap.bootstrap_epoch(), Err(DirectoryClientError::Transport(TransportError::Io("directory page task disconnected".into()))))),
        }?;
        home.page_rx.take();
        home.page_cancel.take();
        home.page_task.take();
        Some(result)
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn poll_directory_home_publication(&mut self) -> Option<ShellDirectoryHomePublicationResult> {
        let home = self.directory_home.as_mut()?;
        let result = match home.home_rx.as_ref()?.try_recv() {
            Ok(result) => Some(result),
            Err(std::sync::mpsc::TryRecvError::Empty) => None,
            Err(std::sync::mpsc::TryRecvError::Disconnected) => Some((
                home.bootstrap.bootstrap_epoch(),
                home.instance_id,
                DirectoryEventPageAckV1 { bootstrap_epoch: home.bootstrap.bootstrap_epoch(), session_binding_sha256: String::new(), authorization_generation: 0, through_seq_inclusive: home.bootstrap.after(), receipt_sha256: String::new() },
                ShellDirectoryHomePublicationOutcome::Terminal("retained Home publication task disconnected".into()),
            )),
        }?;
        home.home_rx.take();
        home.home_task.take();
        Some(result)
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn open_acknowledged_directory_stream(&mut self, since: u64) -> Result<(), String> {
        let client = self.directory_client.clone().ok_or("directory client is unavailable")?;
        let stream = client.stream_acknowledged(since).map_err(|error| error.to_string())?;
        let context = self.directory_ctx();
        let runner = ShellDirectoryRunner::start(crate::renderer_worker_pool(), stream, context);
        let home = self.directory_home.as_mut().ok_or("retained Home projection is unavailable")?;
        if home.closed || home.stream.is_some() {
            runner.cancel();
            return Err("retained Home stream admission raced with retirement".into());
        }
        home.stream = Some(runner);
        Ok(())
    }

    /// 🪪️ Starts deadline-aware identity bootstrap as a retained-waker I/O-lane future. Each pool
    /// turn polls once; the HTTP boundary itself is admitted on `Lane::Io`, and the UI only polls
    /// this result channel. No worker waits for the network future and no local executor is built.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn bootstrap_identity(&mut self) {
        let Some(env) = resolve_identity_env() else { return };
        self.identity_env = Some(env.clone());
        let (tx, rx) = std::sync::mpsc::channel::<Result<IdentityOutcome, String>>();
        let transport = self.directory_transport.clone();
        let pool = crate::renderer_worker_pool();
        let mut ctx = self.directory_ctx();
        ctx.deadline_ms = Some(pool.now_ms().saturating_add(5_000));
        self.identity_bootstrap_task = Some(ShellPoolFuture::spawn(pool, Lane::Io, async move {
            let outcome = restore_claimed(&ctx, transport, "native").await.map_err(|error| error.to_string());
            let _ = tx.send(outcome);
        }));
        self.identity_bootstrap_rx = Some(rx);
    }

    /// 🪪️ Drains `identity_bootstrap_rx` and starts a fresh zero-frontier retained-Home epoch.
    #[cfg(not(target_arch = "wasm32"))]
    fn poll_identity_bootstrap(&mut self) {
        let Some(rx) = self.identity_bootstrap_rx.as_ref() else { return };
        match rx.try_recv() {
            Ok(Ok(outcome)) => {
                self.identity_bootstrap_task.take();
                self.identity_offline = outcome.status == IdentityStatus::Offline;
                self.identity = Some(outcome.identity.clone());
                self.verified_session_authority = Some(outcome.authority);
                let client = std::sync::Arc::new(DirectoryClient::authenticated(self.directory_transport.clone(), outcome.credential.clone()));
                self.document_host.set_local_hub_credential(outcome.credential);
                self.document_host.set_hub_socket_grant_source(client.clone());
                self.directory_client = Some(client);
                self.project_verified_hub_authority();
                if let Some(home) = self.directory_home.as_mut() {
                    if let Err(error) = home.begin_epoch(0) {
                        home.close();
                        self.error = Some(error.to_string());
                    }
                }
                self.identity_bootstrap_rx = None;
            }
            Ok(Err(error)) => {
                self.identity_bootstrap_task.take();
                Self::debug_log(&format!("[TRACE] wgpu shell identity bootstrap failed: {error}"));
                self.identity_bootstrap_rx = None;
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => {}
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                self.identity_bootstrap_task.take();
                self.identity_bootstrap_rx = None;
            }
        }
    }

    /// 📡️ Advances the finite fetch → terminal Home receipt → exact ACK → acknowledged-live owner.
    ///
    /// 🌉️ Packet W15e: the agent bridge's socket rides the SAME frame pump as the directory lane.
    ///
    /// 🏛️ One bounded administration turn per frame: the retained operation never spins, because
    /// `pump_space_administration` answers `false` the moment it reaches `Idle` or a terminal phase.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) async fn pump_directory_events(&mut self) -> bool {
        self.poll_identity_bootstrap();
        let inference_changed = self.pump_inference_port();
        let bridge_changed = self.pump_agent_bridge();
        let mut changed = self.pump_space_administration().await || inference_changed || bridge_changed;
        changed |= self.pump_hub_artifact_creation().await;
        changed |= self.pump_hub_check_in().await;
        self.pump_agent_bridge_offer_scope().await;
        changed |= self.poll_hub_workspace_tasks();
        let runner = self.directory_home.as_ref().and_then(|home| home.stream.clone());
        if let Some(runner) = runner {
            if runner.take_terminal() {
                if let Some(home) = self.directory_home.as_mut() {
                    let _ = home.begin_epoch(0);
                }
                self.directory_client = None;
                #[cfg(not(target_arch="wasm32"))]
        self.cancel_inference_port();
        self.identity = None;
                self.verified_session_authority = None;
                self.bootstrap_identity();
                changed = true;
            }
            let mut dirty = false;
            let mut rebootstrap = false;
            let mut live_events: Vec<semio_framework_os_kernel::os_directory::DirectoryEvent> = Vec::new();
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
                self.dispatch_fold_directory_events(&live_events);
                changed = true;
            }
            if dirty || rebootstrap {
                if let Some(home) = self.directory_home.as_mut() {
                    match home.wake(rebootstrap) {
                        Ok(Some(_)) => changed = true,
                        Ok(None) => {}
                        Err(error) => {
                            home.close();
                            self.error = Some(error.to_string());
                        }
                    }
                }
            }
        }

        if let Some((epoch, result)) = self.poll_directory_page_result() {
            let current = self.directory_home.as_ref().is_some_and(|home| !home.closed && home.bootstrap.bootstrap_epoch() == epoch);
            if current {
                match result {
                    Ok(page) => {
                        if let Err(error) = self.start_directory_home_publication(page) {
                            if let Some(home) = self.directory_home.as_mut() {
                                home.close();
                            }
                            self.error = Some(error);
                        }
                    }
                    Err(DirectoryClientError::Unauthorized) => {
                        if let Some(home) = self.directory_home.as_mut() {
                            let _ = home.begin_epoch(0);
                        }
                        self.directory_client = None;
                        #[cfg(not(target_arch="wasm32"))]
        self.cancel_inference_port();
        self.identity = None;
                        self.verified_session_authority = None;
                        self.bootstrap_identity();
                    }
                    Err(DirectoryClientError::Transport(TransportError::Io(_) | TransportError::DeadlineExceeded | TransportError::BudgetExhausted) | DirectoryClientError::Http { status: 500..=599, .. }) => {
                        if let Some(home) = self.directory_home.as_mut() {
                            home.retry_at_ms = crate::renderer_worker_pool().now_ms().saturating_add(DirectoryHomeProjection::RETRY_DELAY_MS);
                        }
                    }
                    Err(DirectoryClientError::Cancelled | DirectoryClientError::Transport(TransportError::Cancelled)) => {}
                    Err(_) => {
                        if let Some(home) = self.directory_home.as_mut() {
                            home.close();
                        }
                        self.error = Some("directory event-page response was terminally rejected".into());
                    }
                }
                changed = true;
            }
        }

        if let Some((epoch, instance_id, expected, outcome)) = self.poll_directory_home_publication() {
            if let ShellDirectoryHomePublicationOutcome::Rejected(refusal) = &outcome {
                if self.chrome_build.refused_frame_fault.is_none() { self.chrome_build.refused_frame_fault = refusal.frame; }if self.chrome_build.refused_kernel_fault.is_none(){self.chrome_build.refused_kernel_fault=refusal.kernel;}if self.chrome_build.refused_actor_call.is_none(){self.chrome_build.refused_actor_call=refusal.actor_call;}
            }
            let current = self.directory_home.as_ref().is_some_and(|home| !home.closed && home.bootstrap.bootstrap_epoch() == epoch && home.instance_id == instance_id);
            if current {
                match outcome {
                    ShellDirectoryHomePublicationOutcome::Published(acknowledgement) => {
                        let transition = self.directory_home.as_mut().expect("checked retained Home").bootstrap.acknowledge(&acknowledgement).map_err(|error| error.to_string());
                        match transition {
                            Ok(DirectoryBootstrapTransition::Fetch { .. }) => {}
                            Ok(DirectoryBootstrapTransition::Live { since }) => {
                                if let Err(error) = self.open_acknowledged_directory_stream(since) {
                                    if let Some(home) = self.directory_home.as_mut() {
                                        home.close();
                                    }
                                    self.error = Some(error);
                                }
                            }
                            Err(error) => {
                                if let Some(home) = self.directory_home.as_mut() {
                                    home.close();
                                }
                                self.error = Some(error);
                            }
                        }
                    }
                    ShellDirectoryHomePublicationOutcome::Rejected(error) => {
                        let frame_refusal = error.frame.is_some();
                        let rebootstrap = error.text.contains("directory-event-page-rebootstrap-required");
                        let detail = self.note_refused_open(error);
                        if let Some(home) = self.directory_home.as_mut() {
                            if frame_refusal {
                                home.close();
                                self.error = Some(detail);
                            } else if rebootstrap {
                                let _ = home.begin_epoch(0);
                            } else if home.retry(crate::renderer_worker_pool().now_ms(), &expected.receipt_sha256).is_err() {
                                home.close();
                                self.error = Some("retained Home directory rejection no longer matches the pending page".into());
                            }
                        }
                    }
                    ShellDirectoryHomePublicationOutcome::Terminal(error) => {
                        if let Some(home) = self.directory_home.as_mut() {
                            home.close();
                        }
                        self.error = Some(error);
                    }
                }
                changed = true;
            }
        }

        self.start_directory_event_page_fetch();
        self.flush_pending_directory_commands().await;
        changed
    }
    //#endregion 🔖️DirectoryAndIdentity

    /// 🐢️ `scope` is what the dispatch that produced these mutations declared it dirtied, unioned
    /// with what applying its own effects dirtied — [`UiDirtyScope::Full`] for every host-owned
    /// caller that speaks for no guest.
    pub fn apply_mutations<'a>(&'a mut self, operations: &'a [String], scope: UiDirtyScope) -> ShellTurn<'a, Result<(), String>> {
        shell_turn(async move {
            self.apply_ops_inner(operations, true, scope, self.session.clone()).await
        })
    }

    /// 🔗️ Document sync flows through the `framework/sync` `ArtifactHost` actor + the program
    /// store's `ChannelBackbone` (see `attach_sync_backbone`), never through a shell mutation.
    ///
    /// 🪟️ `setPanel` names no body of its own and `panel_json` feeds EVERY section, so
    /// the widest scope is the only honest one — the same widening React's
    /// `hostEffectRefreshScopeV1` performs for the `setPanel`/`loadDocument` effects.
    ///
    /// 🗑️ The `downloadMediaExport`/`requestFileOpen` MUTATION arms lived here — two CRUD-era
    /// document operations no producer in the repo emits (`grep -rn '"requestFileOpen"'`
    /// answered this shell and nothing else). They were the only reader this renderer had
    /// for either journey, so `Export Document…` and `Import Document…` looked wired and
    /// reached nobody; both are now real `Effect` arms of the ONE host-effect funnel
    /// (`queue_host_effects`), which every dispatch path feeds
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub(crate) async fn apply_ops_inner(&mut self, operations: &[String], allow_navigate: bool, scope: UiDirtyScope, owner: Option<ActiveSession>) -> Result<(), String> {
        let mut pending: Vec<String> = operations.to_vec();
        let mut view_state = owner.as_ref().map(|s| s.view_state.clone());
        let mut panel_rewritten = false;
        let mut navigate_uri: Option<String> = None;
        while !pending.is_empty() {
            let batch = std::mem::take(&mut pending);
            for operation_json in batch {
                let operation: Value = serde_json::from_str(&operation_json).unwrap_or(Value::Null);
                if operation.get("operation").and_then(|v| v.as_str()) == Some("setPanel") {
                    panel_rewritten = true;
                    if let Some(panel) = operation.get("panel") {
                        if let Some(mut vs) = view_state.take() {
                            vs.panel_json = Some(panel.to_string());
                            view_state = Some(vs);
                        }
                    }
                }
                if operation.get("operation").and_then(|v| v.as_str()) == Some("requestFileSave") {
                    #[cfg(not(target_arch = "wasm32"))]
                    if let (Some(filename), Some(data), Some(space_id)) = (operation.get("filename").and_then(|v| v.as_str()), operation.get("data").and_then(|v| v.as_str()), operation.get("spaceId").and_then(|v| v.as_str())) {
                        if let Some(session) = owner.clone() {
                            let (filename, data, space_id) = (filename.to_string(), data.to_string(), space_id.to_string());
                            self.submit_shell_io_future(async move {
                                let Some(path) = request_file_save(&filename).await else { return ShellIoCompletion::Finished };
                                use std::fs as system_fs;
                                if system_fs::write(&path, data.as_bytes()).is_err() {
                                    return ShellIoCompletion::Finished;
                                }
                                ShellIoCompletion::Actions(vec![ActionDescriptor {
                                    controller_id: session.app.controller_id,
                                    action: "bindSpaceFile".into(),
                                    args: crate::action_args_json!({ "spaceId": space_id, "filePath": path.display().to_string() }),
                                }])
                            });
                        }
                    }
                }
                if operation.get("operation").and_then(|v| v.as_str()) == Some("requestFolderPick") {
                    #[cfg(not(target_arch = "wasm32"))]
                    if let Some(import_action) = operation.get("importAction").and_then(|v| v.as_str()) {
                        if let Some(session) = owner.clone() {
                            let import_action = import_action.to_string();
                            let mut args = operation.get("args").cloned().unwrap_or_else(|| serde_json::json!({}));
                            self.submit_shell_io_future(async move {
                                let Some(folder_path) = pick_folder().await else { return ShellIoCompletion::Finished };
                                if let Some(obj) = args.as_object_mut() {
                                    obj.insert("folderPath".into(), serde_json::json!(folder_path));
                                }
                                ShellIoCompletion::Actions(vec![ActionDescriptor { controller_id: session.app.controller_id, action: import_action, args: Some(semio_framework::DslValue::from(args)) }])
                            });
                        }
                    }
                }
                if operation.get("operation").and_then(|v| v.as_str()) == Some("spawnProgram") {
                    if let (Some(plugin_id), Some(session)) = (operation.get("pluginId").and_then(|v| v.as_str()), &self.session) {
                        self.spawn_plugin(plugin_id, session.view_state.clone()).await?;
                    }
                }
                if operation.get("operation").and_then(|v| v.as_str()) == Some("navigate") {
                    if let Some(uri) = operation.get("uri").and_then(|v| v.as_str()) {
                        navigate_uri = Some(uri.to_string());
                    }
                }
            }
        }
        if allow_navigate {
            if let Some(uri) = navigate_uri.take() {
                self.push_uri(uri.clone());
                self.apply_shell_uri(&uri).await?;
                return Ok(());
            }
        }
        let scope = if panel_rewritten { UiDirtyScope::Full } else { scope };
        if let (Some(owner), Some(vs)) = (owner.as_ref(), view_state) {
            if let Some(session) = self.session.as_mut().filter(|session| Self::same_session(session, owner)) {
                session.view_state = vs;
                self.sync_session_chrome();
            } else if let Some(session) = self.spawned_session.as_mut().filter(|session| Self::same_session(session, owner)) {
                session.view_state = vs;
            } else {
                return Err("mutation owner is no longer mounted".into());
            }
            self.refresh_ui(scope).await?;
        } else {
            self.owe_refresh(scope);
        }
        Ok(())
    }

    /// 🏠️🧳️ Generic replacement for the old `switch_to_s_app` — switches to either the host plugin's
    /// landing or host app by id (both resolved via `host_config()`, never a specific app's identity).
    ///
    /// 🎬️ The host plugin is resident on every healthy boot, so this is a one-scan no-op then. It is
    /// not decoration: `mountPluginHandles` skips a plugin whose module load faulted, and a boot
    /// that lost the HOST plugin that way used to answer `host program missing` for every later
    /// switch with no way back. One lazy install is the way back, on both targets.
    async fn switch_to_managed_app(&mut self, app_id: &str, view_state: Option<ViewModel>) -> Result<(), String> {
        let cfg = self.host_config().ok_or("host config missing")?;
        self.install_plugin(&cfg.plugin_id).await?;
        let semio_s_plugin_space = self.plugins.iter().find(|program| program.plugin_id == cfg.plugin_id).ok_or("host program missing")?;
        let app = semio_s_plugin_space.manifest.apps.iter().find(|candidate| candidate.id == app_id).ok_or("host app missing")?.clone();
        if let Some(session) = &self.session {
            if session.plugin_id == semio_s_plugin_space.plugin_id && session.app.id == app_id {
                if let Some(next_view_state) = view_state {
                    if let Some(mut current) = self.session.take() {
                        current.view_state = next_view_state;
                        self.session = Some(current);
                        self.refresh_ui(UiDirtyScope::Full).await?;
                    }
                }
                return Ok(());
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        if app_id == cfg.landing_app_id {
            if let Some(home) = self.directory_home.as_mut().filter(|home| home.plugin_id == semio_s_plugin_space.plugin_id && home.app.id == app_id) {
                if let Some(next_view_state) = view_state {
                    home.view_state = next_view_state;
                }
                let session = home.active_session();
                self.active_window_id = Some(session.app.window_kinds.first().id.clone());
                self.open_space_id = None;
                self.session = Some(session);
                return self.refresh_ui(UiDirtyScope::Full).await;
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        if let (Some(home), Some(current)) = (self.directory_home.as_mut(), self.session.as_ref()) {
            if home.is_instance(&current.plugin_id, current.instance_id) {
                home.view_state = current.view_state.clone();
            }
        }
        let instance_id = semio_s_plugin_space.create_app(&app.id).await?;
        let panel_state = self.default_host_panel_state()?;
        let default_view_state = ViewModel {
            active_mode_id: Some(app.default_mode_id.clone()),
            active_window_kind_id: Some(app.window_kinds.first().id.clone()),
            active_utility_id: None,
            panel_json: Some(Self::panel_json(&panel_state)?),
            extension_input_json: None,
            session_identity: None,
            locale: self.active_locale(),
            terminology: self.active_terminology(),
            window_id: None,
            focused_window_id: Some(app.window_kinds.first().id.clone()),
            window_instances: Vec::new(),
            active_tool_id: None,
            active_utility_by_window_id: Default::default(),
            tool_run_trace_cursor_by_window_id: Default::default(),
            tree_windows: Vec::new(),
            tree_viewport_rows: None,
        };
        let next_view_state = view_state.unwrap_or(default_view_state);
        self.active_window_id = Some(app.window_kinds.first().id.clone());
        if app_id == cfg.landing_app_id {
            self.open_space_id = None;
        }
        self.session = Some(ActiveSession { plugin_id: semio_s_plugin_space.plugin_id.clone(), instance_id, app, view_state: next_view_state });
        self.refresh_ui(UiDirtyScope::Full).await
    }

    /// 👁️✏️ Opens the sibling surface of the OPEN document's dialect — the wgpu twin of React's
    /// `switchToSessionRole` (`🧱️elements/🏛️ShellHost/🟦️.tsx`). A role the dialect does not declare,
    /// and the role already mounted, are both no-operations: `role_switch_target` answers `None` and
    /// nothing is created, retired or refreshed.
    pub(crate) async fn switch_to_session_role(&mut self, requested: semio_framework::manifest::AppRole) -> Result<(), String> {
        let Some(session) = self.session.clone() else {
            return Ok(());
        };
        let Some(program) = self.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).cloned() else {
            return Ok(());
        };
        let Some(target) = role_switch_target(&program.manifest.apps, &session.app.dialect, session.app.role, requested).cloned() else {
            return Ok(());
        };
        self.run_session_app_switch(&program, target).await
    }

    /// 🔀️ The ORDERED in-place session switch — `create → seal → retire → publish → seed → refresh`.
    /// The successor instance exists before the predecessor is retired, so no render ever observes
    /// two live instances of one plugin and no switch leaks a `create_app`. Twin of
    /// `runSessionAppSwitchV1` (`🧱️elements/🏛️ShellHost/🔀️surface-switch/🟦️.ts`), whose `switch`
    /// fixture rows pin both the order and the `created − retired === 0` invariant. "Seal" is this
    /// target's own shape of React's close ladder: every retained surface document of the outgoing
    /// app is driven to terminal before its instance goes, because the resident aggregate is a fixed
    /// process-wide permit and a leaked root refuses every later surface with `Capacity`.
    async fn run_session_app_switch(&mut self, program: &ProgramBridgeEntry, app: AppDefinition) -> Result<(), String> {
        let Some(previous) = self.session.clone() else {
            return Ok(());
        };
        let instance_id = program.create_app(&app.id).await.map_err(|refusal| self.note_refused_open(refusal))?;
        Self::debug_log(&format!("[TRACE] shell session switch {}", serde_json::json!({ "step": "create", "plugin": program.plugin_id, "from": previous.app.id, "to": app.id, "instance": instance_id })));
        self.retire_documents_outside(&[], true)?;
        self.retire_documents_outside(&[], false)?;
        Self::debug_log(&format!("[TRACE] shell session switch {}", serde_json::json!({ "step": "seal", "instance": previous.instance_id })));
        program.destroy_app(previous.instance_id);
        Self::debug_log(&format!("[TRACE] shell session switch {}", serde_json::json!({ "step": "retire", "instance": previous.instance_id })));
        let landing_window_id = app.window_kinds.first().id.clone();
        let view_state = ViewModel {
            active_mode_id: Some(app.default_mode_id.clone()),
            active_window_kind_id: Some(landing_window_id.clone()),
            active_utility_id: None,
            panel_json: None,
            extension_input_json: None,
            session_identity: None,
            locale: self.active_locale(),
            terminology: self.active_terminology(),
            window_id: None,
            focused_window_id: Some(landing_window_id.clone()),
            window_instances: Vec::new(),
            active_tool_id: None,
            active_utility_by_window_id: Default::default(),
            tool_run_trace_cursor_by_window_id: Default::default(),
            tree_windows: Vec::new(),
            tree_viewport_rows: None,
        };
        self.layout_override = None;
        self.active_window_id = Some(landing_window_id);
        self.session = Some(ActiveSession { plugin_id: previous.plugin_id.clone(), instance_id, app, view_state });
        self.sync_session_chrome();
        self.sync_dock();
        self.push_contributions().await?;
        self.announce_session_example().await?;
        Self::debug_log(&format!("[TRACE] shell session switch {}", serde_json::json!({ "step": "publish", "app": self.session.as_ref().map(|session| session.app.id.clone()) })));
        self.refresh_ui(UiDirtyScope::Full).await?;
        Self::debug_log(&format!(
            "[TRACE] shell session switch {}",
            serde_json::json!({ "step": "refresh", "app": self.session.as_ref().map(|session| session.app.id.clone()), "role": self.session.as_ref().map(|session| session.app.role.as_str()) })
        ));
        Ok(())
    }

    /// 🔁️ Steps the active mode one place along the app's declared order, wrapping — what the
    /// `mod+alt+→`/`mod+alt+←` chords fire. Reuses the very same body as clicking a
    /// `playground.navbar.modes.<id>` button, so keyboard and pointer cannot diverge.
    pub(crate) async fn apply_mode_step(&mut self, step: i32) -> Result<(), String> {
        let Some(session) = self.session.as_ref() else {
            return Ok(());
        };
        let mode_ids: Vec<String> = session.app.modes.iter().map(|mode| mode.id.clone()).collect();
        let Some(next) = step_mode_id(&mode_ids, session.view_state.active_mode_id.as_deref(), step) else {
            return Ok(());
        };
        self.apply_navbar_mode(&next).await
    }

    /// 🎛️ The ONE body behind a mode change, whether a navbar button or a chord asked for it.
    pub(crate) async fn apply_navbar_mode(&mut self, mode_id: &str) -> Result<(), String> {
        if let Some(session) = self.session.as_mut() {
            session.view_state.active_mode_id = Some(mode_id.to_string());
            if let Some(layout) = semio_framework::resolve_layout_for_mode(&session.app, mode_id) {
                self.layout_override = Some(layout);
                self.sync_dock();
                self.active_window_id = self.dock.active_window_id.clone();
            }
        }
        self.refresh_ui(UiDirtyScope::Full).await
    }

    fn validate_extension_record(record: &ShellExtensionStoreRecord) -> Result<(), String> {
        if record.extension_id.is_empty() || record.directory_name.is_empty() || record.version.is_empty() || record.extends_host.is_empty() || record.module_url.is_empty() || record.package_hash.is_empty() {
            return Err("extension store returned an incomplete record".into());
        }
        Ok(())
    }

    pub(crate) fn project_extension_record(&mut self, record: ShellExtensionStoreRecord) -> Result<usize, String> {
        Self::validate_extension_record(&record)?;
        if let Some(index) = self.extensions.iter().position(|entry| entry.record.extension_id == record.extension_id) {
            self.extensions[index].record = record;
            return Ok(index);
        }
        if self.extensions.len() >= SHELL_EXTENSION_LEDGER_CAPACITY {
            return Err("extension projection capacity exceeded".into());
        }
        self.extensions.push(ShellExtensionProjection { record, enabled: true, load_status: ShellExtensionLoadStatus::Available, program: None });
        Ok(self.extensions.len() - 1)
    }

    #[cfg(target_arch = "wasm32")]
    async fn load_extension_record(&mut self, record: ShellExtensionStoreRecord) -> Result<(), String> {
        let index = self.project_extension_record(record.clone())?;
        let record_json = serde_json::to_string(&record).map_err(|error| error.to_string())?;
        match crate::program_bridge::install_js_extension(&record_json, &record.extension_id, &record.version).await {
            Ok(program) => {
                let enabled = self.extensions[index].enabled;
                self.plugins.retain(|entry| entry.plugin_id != record.extension_id);
                if enabled {
                    self.plugins.push(program.clone());
                }
                self.extensions[index].program = Some(program);
                self.extensions[index].load_status = ShellExtensionLoadStatus::Loaded;
                Ok(())
            }
            Err(error) => {
                self.extensions[index].program = None;
                self.extensions[index].load_status = ShellExtensionLoadStatus::Failed(error.clone());
                Err(error)
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    async fn extension_store_answer(request: serde_json::Value) -> Result<serde_json::Value, String> {
        let answer = host_io_call(&request.to_string(), None).await?;
        let decoded: serde_json::Value = serde_json::from_str(&answer).map_err(|error| format!("extension store answer: {error}"))?;
        if let Some(error) = decoded.get("error").and_then(|value| value.as_str()) {
            return Err(error.to_string());
        }
        Ok(decoded)
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) async fn refresh_extension_store(&mut self) -> Result<(), String> {
        let answer = Self::extension_store_answer(serde_json::json!({ "op": "extension-store-list" })).await?;
        let records: Vec<ShellExtensionStoreRecord> = serde_json::from_value(answer.get("extensions").cloned().ok_or("extension store list missing extensions")?).map_err(|error| error.to_string())?;
        if records.len() > SHELL_EXTENSION_LEDGER_CAPACITY {
            return Err("extension store list exceeds projection capacity".into());
        }
        let retired: Vec<String> = self.extensions.iter().filter(|entry| !records.iter().any(|record| record.extension_id == entry.record.extension_id)).map(|entry| entry.record.extension_id.clone()).collect();
        self.plugins.retain(|entry| !retired.contains(&entry.plugin_id));
        self.extensions.retain(|entry| !retired.contains(&entry.record.extension_id));
        for record in records {
            let _ = self.load_extension_record(record).await;
        }
        Ok(())
    }

    pub(crate) async fn install_extension_from_store(&mut self, request: serde_json::Value) -> Result<bool, String> {
        #[cfg(target_arch = "wasm32")]
        {
            let answer = Self::extension_store_answer(request).await?;
            if answer.get("cancelled").and_then(serde_json::Value::as_bool) == Some(true) {
                return Ok(false);
            }
            let record: ShellExtensionStoreRecord = serde_json::from_value(answer.get("extension").cloned().ok_or("extension store install missing extension")?).map_err(|error| error.to_string())?;
            let load = self.load_extension_record(record).await;
            self.push_contributions().await?;
            self.refresh_ui(UiDirtyScope::Full).await?;
            load.map(|_| true)
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = request;
            Err("native extension installation is unavailable".into())
        }
    }

    pub(crate) fn set_extension_enabled<'a>(&'a mut self, extension_id: &'a str, enabled: bool) -> ShellTurn<'a, Result<bool, String>> {

        shell_turn(async move {
            let Some(index) = self.extensions.iter().position(|entry| entry.record.extension_id == extension_id) else { return Ok(false) };
            let program = self.extensions[index].program.clone();
            self.extensions[index].enabled = enabled;
            self.plugins.retain(|entry| entry.plugin_id != extension_id);
            if enabled {
                if let Some(program) = program {
                    self.plugins.push(program);
                }
            }
            self.push_contributions().await?;
            self.refresh_ui(UiDirtyScope::Full).await?;
            Ok(true)

        })

    }

    pub(crate) async fn uninstall_extension(&mut self, extension_id: &str) -> Result<bool, String> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = extension_id;
            return Err("native extension uninstallation is unavailable".into());
        }
        #[cfg(target_arch = "wasm32")]
        {
            let Some(index) = self.extensions.iter().position(|entry| entry.record.extension_id == extension_id) else { return Ok(false) };
            let record = self.extensions[index].record.clone();
            self.plugins.retain(|entry| entry.plugin_id != extension_id);
            self.push_contributions().await?;
            self.refresh_ui(UiDirtyScope::Full).await?;
            if let Err(error) = crate::program_bridge::retire_js_extension(extension_id).await {
                let recovery = self.load_extension_record(record.clone()).await;
                self.push_contributions().await?;
                self.refresh_ui(UiDirtyScope::Full).await?;
                if let Err(recovery) = recovery {
                    return Err(format!("{error}; recovery failed: {recovery}"));
                }
                return Err(error);
            }
            if let Err(error) = Self::extension_store_answer(serde_json::json!({ "op": "extension-store-uninstall", "extensionId": extension_id })).await {
                let recovery = self.load_extension_record(record).await;
                self.push_contributions().await?;
                self.refresh_ui(UiDirtyScope::Full).await?;
                if let Err(recovery) = recovery {
                    return Err(format!("{error}; recovery failed: {recovery}"));
                }
                return Err(error);
            }
            self.extensions.remove(index);
            self.refresh_ui(UiDirtyScope::Full).await?;
            Ok(true)
        }
    }

    /// 🎯️ Refuses an install this target could never complete, BEFORE any install record is minted —
    /// native needs a resident program to read the `<modules_root>/<plugin_id>/<file>.wasm` root back
    /// off, the browser needs the frame Worker's own module door. A refusal here is not a phase: there
    /// is nothing in flight to report, and chrome must not show a cancellable install that never began.
    #[cfg(not(target_arch = "wasm32"))]
    fn plugin_install_precondition(&self) -> Result<(), String> {
        plugin_modules_root_of(&self.plugins).map(|_| ()).ok_or_else(|| "plugin modules root is unavailable".to_string())
    }

    /// 🎯️ Browser twin of the native precondition — see that method's own doc.
    #[cfg(target_arch = "wasm32")]
    fn plugin_install_precondition(&self) -> Result<(), String> {
        crate::program_bridge::js_plugin_install_door_available().then_some(()).ok_or_else(|| "this isolate installs no semioWgpuInstallPlugin door".to_string())
    }

    /// 📦️ The one step that differs per target: hand back a `ProgramBridgeEntry` for `plugin_id`.
    /// Native re-reads the runtime manifest `load_wasm_plugins` publishes (instantiating nothing —
    /// `from_wasm` compiles at `create_app`); the browser asks the frame Worker's own module loader for
    /// the SAME `pluginHandleForBridge` handle the boot plan's eager mount produces, because the wasm
    /// renderer cannot fetch a module itself.
    #[cfg(not(target_arch = "wasm32"))]
    async fn acquire_plugin_program(&mut self, plugin_id: &str) -> Result<ProgramBridgeEntry, String> {
        let modules_root = plugin_modules_root_of(&self.plugins).ok_or_else(|| "plugin modules root is unavailable".to_string())?;
        let plugin_filter = self.plugin_filter.clone();
        let entries = crate::program_bridge::load_wasm_plugins(&plugin_filter, &modules_root).await?;
        entries.into_iter().find(|entry| entry.plugin_id == plugin_id).ok_or_else(|| format!("native runtime manifest declares no module for {plugin_id}"))
    }

    /// 📦️ Browser twin of the native acquisition — see that method's own doc.
    #[cfg(target_arch = "wasm32")]
    async fn acquire_plugin_program(&mut self, plugin_id: &str) -> Result<ProgramBridgeEntry, String> {
        crate::program_bridge::install_js_plugin(plugin_id).await
    }

    /// 🎬️ Makes `plugin_id` resident, the wgpu twin of React's `installPlugin`
    /// (`🏛️ShellHost/🟦️.tsx`), on BOTH targets. A hub session lists every registered plugin in its
    /// session but instantiates none of them eagerly on native — `ProgramBridgeEntry::from_wasm`
    /// compiles nothing until `create_app` — so a native miss means the manifest this shell booted
    /// from predates the module, which one re-read fixes. In the browser the boot plan DOES eager-mount
    /// the variant's plugins, so a miss means the owner was outside that plan or its mount faulted and
    /// was skipped (`mountPluginHandles`'s per-plugin isolation); either way the frame Worker can still
    /// load one more module on demand. Already-resident is the common case and costs one scan.
    ///
    /// ⏱️ Progress and cancellation ride [`ShellState::plugin_install`]: the phase advances
    /// `Resolving` → `Loading` → cleared, and every step boundary checks the retained `CancelToken`, so
    /// [`Self::cancel_plugin_install`] stops the install before it commits an entry rather than after.
    /// One install at a time — a second request for the SAME plugin while one runs is refused rather
    /// than racing a second acquisition against the first one's commit.
    pub async fn install_plugin(&mut self, plugin_id: &str) -> Result<(), String> {
        if self.plugins.iter().any(|entry| entry.plugin_id == plugin_id) {
            return Ok(());
        }
        if self.plugin_install.is_some() {
            return Err(format!("plugin install already in flight, {plugin_id} not admitted"));
        }
        self.plugin_install_precondition()?;
        let cancel = CancelToken::root_now();
        self.plugin_install = Some(ShellPluginInstall { plugin_id: plugin_id.to_string(), phase: ShellPluginInstallPhase::Resolving, cancel: cancel.clone() });
        if let Some(install) = self.plugin_install.as_mut() {
            install.phase = ShellPluginInstallPhase::Loading;
        }
        let loaded = self.acquire_plugin_program(plugin_id).await;
        let settled = match loaded {
            _ if cancel.is_cancelled_now() => Err(ShellPluginInstallPhase::Cancelled),
            Err(error) => Err(ShellPluginInstallPhase::Failed(error)),
            Ok(entry) => Ok(entry),
        };
        match settled {
            Ok(entry) => {
                if !self.plugins.iter().any(|resident| resident.plugin_id == plugin_id) {
                    self.plugins.push(entry);
                }
                self.plugin_install = None;
                Ok(())
            }
            Err(phase) => {
                let message = match &phase {
                    ShellPluginInstallPhase::Cancelled => format!("plugin install cancelled: {plugin_id}"),
                    ShellPluginInstallPhase::Failed(error) => error.clone(),
                    _ => format!("plugin install refused: {plugin_id}"),
                };
                if let Some(install) = self.plugin_install.as_mut() {
                    install.phase = phase;
                }
                Self::debug_log(&format!("[TRACE] wgpu shell plugin install failed: {message}"));
                Err(message)
            }
        }
    }

    /// 🛑️ Cancels the retained lazy install. The phase does NOT move here: only the install's own next
    /// step boundary may report `Cancelled`, so an install that already committed is never misreported
    /// as having been stopped. Clearing a settled `Failed`/`Cancelled` record is the same call. In the
    /// browser the frame Worker's own request is withdrawn too, so a cancel during a long module fetch
    /// settles the awaited promise now instead of at the end of the fetch.
    pub fn cancel_plugin_install(&mut self) {
        let settled = matches!(self.plugin_install.as_ref().map(|install| &install.phase), Some(ShellPluginInstallPhase::Failed(_)) | Some(ShellPluginInstallPhase::Cancelled));
        if settled {
            self.plugin_install = None;
            return;
        }
        if let Some(install) = self.plugin_install.as_ref() {
            install.cancel.cancel_now();
            #[cfg(target_arch = "wasm32")]
            crate::program_bridge::cancel_js_plugin_install(&install.plugin_id);
        }
    }

    /// 🎬️ The plugin that opens `dialect`, from the catalog's declared `on-artifact-kind:` rows — the
    /// wgpu twin of React's `artifactKindActivationOwner` lookup, and the only resolution that works
    /// before the owner's manifest is resident. Returns the owner and its app for `role` once that
    /// owner is installed, so the relay below can hand both straight to [`Self::switch_to_app`].
    /// Both targets: the table is generated into this crate and the install is [`Self::install_plugin`],
    /// which O2 made target-neutral.
    async fn resolve_activation_owner_app(&mut self, dialect: &semio_framework_artifact_reference::ArtifactDialect, role: semio_framework::manifest::AppRole) -> Option<(String, AppDefinition)> {
        let owner = crate::program_bridge::resolve_artifact_kind_activation_owner(&dialect.artifact_kind, crate::program_bridge::plugin_artifact_kind_activations())?;
        if let Err(error) = self.install_plugin(owner).await {
            Self::debug_log(&format!("[TRACE] wgpu shell os.open-artifact could not install {owner}: {error}"));
            return None;
        }
        let program = self.plugins.iter().find(|entry| entry.plugin_id == owner)?;
        let app = find_dialect_app(program, dialect, role).or_else(|| find_dialect_app(program, dialect, semio_framework::manifest::AppRole::Editor))?;
        Some((owner.to_string(), app.clone()))
    }

    /// 📇️ ticket §6 — generic app switch by definition (not the host's landing/host app via
    /// `host_config()`), used for the `s.space` artifact-index route below. Kept as a close sibling of
    /// `switch_to_managed_app` (CLAUDE.md: repeated code stays close together) rather than a forced
    /// shared abstraction that would blur the two call shapes (this one has no workflow-panel state).
    /// Both targets — the lazy install it opens with is [`Self::install_plugin`]; only the retained
    /// Home hand-back below is native, because `directory_home` is a native-only projection.
    async fn switch_to_app(&mut self, plugin_id: &str, app: AppDefinition) -> Result<(), String> {
        self.install_plugin(plugin_id).await?;
        let program = self.plugins.iter().find(|entry| entry.plugin_id == plugin_id).cloned().ok_or("program missing")?;
        if let Some(session) = &self.session {
            if session.plugin_id == plugin_id && session.app.id == app.id {
                return Ok(());
            }
        }
        let instance_id = program.create_app(&app.id).await.map_err(|refusal| self.note_refused_open(refusal))?;
        self.install_app_session(plugin_id, app, instance_id);
        self.refresh_ui(UiDirtyScope::Full).await
    }

    /// 🪪️ Mounts `instance_id` of `app` as this shell's session: the retained Home hands back its view
    /// state first, then the session and its first window become current. The synchronous tail of
    /// [`Self::switch_to_app`], shared with the frame-pumped open ([`Self::advance_document_opening`]),
    /// which renders on the settle lane instead of inline.
    fn install_app_session(&mut self, plugin_id: &str, app: AppDefinition, instance_id: u32) {
        #[cfg(not(target_arch = "wasm32"))]
        if let (Some(home), Some(current)) = (self.directory_home.as_mut(), self.session.as_ref()) {
            if home.is_instance(&current.plugin_id, current.instance_id) {
                home.view_state = current.view_state.clone();
            }
        }
        let view_state = ViewModel {
            active_mode_id: Some(app.default_mode_id.clone()),
            active_window_kind_id: Some(app.window_kinds.first().id.clone()),
            active_utility_id: None,
            panel_json: None,
            extension_input_json: None,
            session_identity: None,
            locale: self.active_locale(),
            terminology: self.active_terminology(),
            window_id: None,
            focused_window_id: Some(app.window_kinds.first().id.clone()),
            window_instances: Vec::new(),
            active_tool_id: None,
            active_utility_by_window_id: Default::default(),
            tool_run_trace_cursor_by_window_id: Default::default(),
            tree_windows: Vec::new(),
            tree_viewport_rows: None,
        };
        self.active_window_id = Some(app.window_kinds.first().id.clone());
        self.session = Some(ActiveSession { plugin_id: plugin_id.to_string(), instance_id, app, view_state });
    }

    /// 📇️ ticket §5/§6 — "/spaces/{id}/studio" keeps the pre-existing studio behaviour (the ONLY
    /// behaviour this route had before this lane); a bare "/spaces/{id}" now opens the `s.space`
    /// artifact-index app instead, mirroring the React shell's `applyShellUri` (`📓️w2-c-report.md`).
    ///
    /// 🧭️ Pin before the async switch so a concurrent chrome sync cannot boot the demo example over
    /// an explicit `/spaces/:id` route.
    ///
    /// 🎠️ H3-wgpu-native — `load_app_document_pack` is now async.
    pub(crate) fn apply_shell_uri<'a>(&'a mut self, uri: &'a str) -> ShellTurn<'a, Result<(), String>> {

        shell_turn(async move {
            let path = uri.split('?').next().unwrap_or(uri);
            if path.trim_end_matches('/') == "/hub" {
                self.hub_workspace_open = true;
                self.project_verified_hub_authority();
                if self.hub_workspace.session.phase == HubSessionPhase::SignedIn {
                    self.start_hub_spaces_reload();
                }
                return self.refresh_ui(UiDirtyScope::Full).await;
            }
            let Some(cfg) = self.host_config() else {
                return Ok(());
            };
            let raw = path.strip_prefix("/spaces/").map(|value| value.trim_end_matches('/').to_string()).filter(|value| !value.is_empty());
            if raw.is_none() {
                self.open_space_id = None;
                if self.session.as_ref().map(|session| session.app.id.as_str()) != Some(cfg.landing_app_id) {
                    self.switch_to_managed_app(cfg.landing_app_id, None).await?;
                }
                return Ok(());
            }
            let raw = raw.expect("space route");
            let space_route = match raw.split_once('/') {
                Some((id, "studio")) => (id.to_string(), true),
                _ => (raw.clone(), false),
            };
            let space_id = space_route.0;
            if !space_route.1 {
                self.open_space_id = Some(space_id.clone());
                let host_program = self.plugins.iter().find(|program| program.plugin_id == cfg.plugin_id).cloned();
                let space_index = host_program.as_ref().and_then(program_space_index);
                let space_app = host_program
                    .as_ref()
                    .and_then(|program| find_space_index_app(program, semio_framework::manifest::AppRole::Editor).or_else(|| find_space_index_app(program, semio_framework::manifest::AppRole::Viewer)))
                    .cloned();
                let (Some(space_index), Some(space_app)) = (space_index, space_app) else {
                    Self::debug_log(&format!("[TRACE] wgpu shell apply_shell_uri: the host plugin names no space index app (os.space-index) — not loaded yet"));
                    return Ok(());
                };
                self.switch_to_app(cfg.plugin_id, space_app).await?;
                let (bindings, surface) = self.default_bindings_for_current_session();
                return self.open_document(SPACE_INDEX_DOCUMENT_ID.to_string(), space_index.document_schema, bindings, surface, None).await;
            }
            let studio_changed = self.open_space_id.as_deref() != Some(space_id.as_str());
            self.open_space_id = Some(space_id.clone());
            self.switch_to_managed_app(cfg.host_app_id, None).await?;
            if !studio_changed {
                return Ok(());
            }
            let session = self.session.clone().ok_or("space session missing")?;
            let program = self.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).ok_or("space program missing")?;
            let action = ActionDescriptor { controller_id: session.app.controller_id.clone(), action: "openSpace".into(), args: crate::action_args_json!({ "spaceId": space_id }) };
            let action_json = serde_json::to_string(&action).map_err(|err| err.to_string())?;
            let result = program.handle_action(session.instance_id, &action_json, &session.view_state).await.map_err(|refusal| self.note_refused_open(refusal))?;
            for effect in &result.requested_effects {
                if let semio_framework::kernel::Effect::LoadDocument { pack, spr } = effect {
                    program.load_app_document_pack(session.instance_id, pack, spr).await.map_err(|refusal| self.note_refused_open(refusal))?;
                }
            }
            self.sync_session_chrome();
            self.refresh_ui(UiDirtyScope::Full).await

        })

    }

    pub async fn apply_pending_shell_uri(&mut self) -> Result<(), String> {
        let uri = self.shell_uri();
        self.apply_shell_uri(&uri).await
    }

    pub(crate) async fn spawn_plugin(&mut self, plugin_id: &str, mut view_state: ViewModel) -> Result<(), String> {
        let workflows = self.build_space_workflows();
        let Some(workflow) = workflows.iter().find(|entry| entry.plugin_id == plugin_id).cloned() else {
            return Ok(());
        };
        let (terminology, locale) = (self.active_terminology(), self.active_locale());
        let bridge = self.plugins.iter().find(|entry| entry.plugin_id == workflow.plugin_id).ok_or("spawn program missing")?;
        let instance_id = bridge.create_app(&workflow.app_id).await.map_err(|refusal| program_fault_text(&refusal, terminology, locale))?;
        let mut panel = Self::panel_state_from_view(&view_state)?.unwrap_or(self.default_host_panel_state()?);
        let spawned_id = format!("{}-{}", bridge.plugin_id, instance_id);
        panel.spawned_apps.push(SpawnedAppEntry { id: spawned_id.clone(), plugin_id: bridge.plugin_id.clone(), instance_id, app_id: workflow.app_id.clone(), label: workflow.label.clone(), breadcrumb: workflow.breadcrumb.clone() });
        panel.active_spawned_id = Some(spawned_id);
        view_state.panel_json = Some(Self::panel_json(&panel)?);
        if let Some(session) = self.session.as_mut() {
            session.view_state = view_state;
        }
        Ok(())
    }
}
//#endregion ShellActions
