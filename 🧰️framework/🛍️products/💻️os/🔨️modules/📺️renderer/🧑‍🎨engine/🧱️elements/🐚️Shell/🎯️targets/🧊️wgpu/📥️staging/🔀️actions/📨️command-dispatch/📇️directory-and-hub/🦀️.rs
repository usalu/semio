//! 📇️ Directory and hub: the replay shell command, the directory events pump, the hub workspace actions and artifact creation, the open artifact relay and the document opening. See [`super`].

use super::*;

//#region ShellActions
impl<'ctx> ActionsCtx<'ctx> {
    //#region 🔖️DirectoryAndIdentity
    //#region 📇️DirectoryLane
    /// 📇️ ticket §C6/§3/§4 — dispatches a `ReplayShellCommand`'s `action_id`/`args` onto the directory funnel, the opening
    /// relay or the creation door; an `os.*` command this shell cannot serve is refused out loud ([`Self::refuse_replay`]) —
    /// the local studio catalog (`os.local-catalog.*`) until route B's native half lands, any unknown one as unrouted.
    pub fn handle_replay_shell_command<'a>(&'a mut self, action_id: &'a str, args: Option<&'a DslValue>) -> ShellTurn<'a, ()> {
        shell_turn(async move {
            let args_json = args.map(dsl_value_as_json);
            if action_id == "os.directory.open-administration" {
                let space_id = args_json.as_ref().and_then(|args| args.get("spaceId")).and_then(Value::as_str).unwrap_or_default().to_string();
                self.directory.open_space_administration(&space_id);
                return;
            }
            if let Some(command) = directory_command_from_action(action_id, args_json.as_ref()) {
                self.directory.dispatch_directory_command(command).await;
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
                self.prefs.refuse_replay(reason, &mut self.chrome);
            }
        })
    }

    /// 🌱️ `os.create-space-artifact {kindChoice, name}` from the mounted Space index guest onto this shell's ONE creation door —
    /// ShellHost's route: a signed-in identity, the index's own space, a choice the space's ready catalog offers
    /// ([`space_artifact_creation_replay_choice`]); the door then seals the intent under its minted key, submits it and opens the
    /// ready artifact ([`Self::pump_hub_artifact_creation`]). A busy or not-yet-ready door asks the user to try again.
    pub async fn replay_create_space_artifact(&mut self, args: Option<&Value>) -> Result<(), ReplayRefusalReasonV1> {
        if self.directory.identity.is_none() {
            return Err(ReplayRefusalReasonV1::SignInRequired);
        }
        let index_mounted = self.sync.sync_channel.as_ref().is_some_and(|channel| channel.document_id == SPACE_INDEX_DOCUMENT_ID);
        let space_id = self.opening.open_space_id.clone().filter(|space_id| index_mounted && !space_id.is_empty()).ok_or(ReplayRefusalReasonV1::SpaceIndexRequired)?;
        if self.directory.hub_workspace.open_space_id.as_deref() != Some(space_id.as_str()) || self.directory.hub_workspace.creation.catalog.as_ref().is_none_or(|catalog| catalog.space_id != space_id) {
            self.directory.hub_workspace.open_space_id = Some(space_id.clone());
            self.directory.open_hub_artifact_creation(&space_id).await;
        }
        let (kind_id, name) = space_artifact_creation_replay_choice(args, self.directory.hub_workspace.creation.catalog.as_ref(), &space_id).await?;
        let creation = &mut self.directory.hub_workspace.creation;
        creation.kind_id = Some(kind_id);
        creation.name_draft = name;
        if hub_artifact_creation_intent(&self.directory.hub_workspace).is_none() {
            let busy = self.directory.hub_workspace.creation.catalog_phase != HubArtifactCatalogPhase::Ready || self.directory.hub_workspace.creation.operation.as_ref().is_some_and(|operation| !hub_artifact_creation_terminal(operation.phase));
            return Err(if busy { ReplayRefusalReasonV1::RouterNotReady } else { ReplayRefusalReasonV1::InvalidRequest });
        }
        self.directory.begin_hub_artifact_creation();
        self.pump_hub_artifact_creation().await;
        Ok(())
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
        let identity_changed = self.directory.poll_browser_identity().await;
        let administration_changed = self.directory.pump_space_administration().await;
        self.directory.flush_pending_directory_commands().await;
        let creation_changed = self.pump_hub_artifact_creation().await;
        let hub_changed = self.sync.poll_hub_workspace_tasks(&mut self.directory);
        let bridge_changed = self.dock.pump_agent_bridge(&mut self.chrome, &mut self.directory, &mut self.windows);
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
    pub fn handle_hub_workspace_action<'a>(&'a mut self, verb: &'a str, args: Option<DslValue>) -> ShellTurn<'a, ()> {
        shell_turn(async move {
            let value = args.as_ref().and_then(|args| args.get("value")).and_then(semio_framework_value::DslValue::as_str).unwrap_or_default().to_string();
            let space_id = args.as_ref().and_then(|args| args.get("spaceId")).and_then(semio_framework_value::DslValue::as_str).unwrap_or_default().to_string();
            let connection_id = args.as_ref().and_then(|args| args.get("connectionId")).and_then(semio_framework_value::DslValue::as_str).unwrap_or_default().to_string();
            match verb {
                hub_action::SET_EMAIL => self.directory.hub_workspace.email_draft = value,
                hub_action::SET_PASSWORD => self.directory.hub_workspace.password_draft = value,
                hub_action::SET_ADDRESS => self.directory.hub_workspace.address_draft = value,
                hub_action::SET_SEARCH => self.directory.hub_workspace.search_draft = value,
                hub_action::SET_SPACE_NAME => self.directory.hub_workspace.space_name_draft = value,
                hub_action::SET_INVITE_TEXT => self.directory.hub_workspace.invite_draft = value,
                hub_action::DISCARD_INVITE_LINK => self.directory.hub_workspace.invite_capability = None,
                hub_action::CLOSE_WORKSPACE => {
                    self.directory.hub_workspace.open_space_id = None;
                    self.directory.hub_workspace_open = false;
                }
                hub_action::SELECT_CONNECTION => {
                    let next = select_hub_connection(&self.directory.hub_workspace.book, &connection_id);
                    if next.selected_id != self.directory.hub_workspace.book.selected_id {
                        self.directory.hub_workspace.book = next;
                        self.directory.hub_workspace.session = reduce_hub_session(&self.directory.hub_workspace.session, &HubSessionEvent::SelectConnection { connection_id });
                        self.directory.clear_hub_session_owner();
                        self.directory.hub_workspace.rows.clear();
                        self.directory.hub_workspace.members.clear();
                        self.directory.hub_workspace.open_space_id = None;
                        self.directory.persist_hub_connection_book();
                    }
                }
                hub_action::ADD_CONNECTION => match parse_hub_origin(&self.directory.hub_workspace.address_draft) {
                    Some(origin) => {
                        let connection = HubConnection { id: hub_connection_id_for_origin(&origin), kind: HubConnectionKind::Remote, label: origin.clone(), last_user_id: None, origin };
                        let connection_id = connection.id.clone();
                        self.directory.hub_workspace.book = upsert_hub_connection(&self.directory.hub_workspace.book, connection);
                        self.directory.hub_workspace.session = reduce_hub_session(&self.directory.hub_workspace.session, &HubSessionEvent::SelectConnection { connection_id });
                        self.directory.hub_workspace.address_draft.clear();
                        self.directory.clear_hub_session_owner();
                        self.directory.persist_hub_connection_book();
                    }
                    None => self.directory.hub_workspace.session.error = Some(HubSignInErrorCode::MalformedRequest),
                },
                hub_action::FORGET_CONNECTION => {
                    let selected = self.directory.hub_workspace.book.selected_id.clone();
                    self.directory.hub_workspace.book = remove_hub_connection(&self.directory.hub_workspace.book, &selected);
                    let connection_id = self.directory.hub_workspace.book.selected_id.clone();
                    self.directory.hub_workspace.session = reduce_hub_session(&self.directory.hub_workspace.session, &HubSessionEvent::SelectConnection { connection_id });
                    self.directory.clear_hub_session_owner();
                    self.directory.persist_hub_connection_book();
                }
                hub_action::CANCEL_SIGN_IN => {
                    if let Some(task) = self.directory.hub_sign_in_task.take() {
                        task.cancel();
                    }
                    self.directory.hub_workspace.session = reduce_hub_session(&self.directory.hub_workspace.session, &HubSessionEvent::Failed { code: HubSignInErrorCode::Cancelled, retry_after_seconds: None });
                }
                hub_action::SIGN_IN => self.directory.start_hub_sign_in(),
                hub_action::SIGN_OUT => self.directory.run_hub_sign_out_turn().await,
                hub_action::REFRESH_SPACES => self.directory.start_hub_spaces_reload(),
                hub_action::OPEN_SPACE => {
                    self.directory.hub_workspace.open_space_id = (!space_id.is_empty()).then(|| space_id.clone());
                    self.directory.reload_hub_members(&space_id).await;
                    self.directory.open_hub_artifact_creation(&space_id).await;
                    if !space_id.is_empty() {
                        let uri = format!("/spaces/{space_id}");
                        self.opening.push_uri(uri.clone());
                        let _ = self.apply_shell_uri(&uri).await;
                    }
                }
                hub_action::CREATE_SPACE => {
                    let name = self.directory.hub_workspace.space_name_draft.clone();
                    if let Some(command) = semio_framework_os_renderer_wgpu_hub_contracts::space_browser::create_space_command(&name, DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private) {
                        self.directory.hub_workspace.phase = semio_framework_os_renderer_wgpu_hub_contracts::space_browser::SpaceBrowserPhase::Submitting;
                        self.directory.dispatch_directory_command(command).await;
                        self.directory.hub_workspace.space_name_draft.clear();
                        self.directory.start_hub_spaces_reload();
                    }
                }
                hub_action::CREATE_INVITE => self.directory.run_hub_create_invite_turn(&space_id).await,
                hub_action::REDEEM_INVITE => self.directory.run_hub_redeem_turn().await,
                hub_action::SELECT_ARTIFACT_KIND => {
                    let kind_id = args.as_ref().and_then(|args| args.get("kindId")).and_then(semio_framework_value::DslValue::as_str).unwrap_or_default();
                    let creation = &mut self.directory.hub_workspace.creation;
                    if creation.catalog.as_ref().is_some_and(|catalog| catalog.kinds.iter().any(|kind| kind.kind_id == kind_id)) {
                        creation.kind_id = Some(kind_id.to_string());
                    }
                }
                hub_action::SET_ARTIFACT_NAME => self.directory.hub_workspace.creation.name_draft = value,
                hub_action::CREATE_ARTIFACT => {
                    self.directory.begin_hub_artifact_creation();
                    self.pump_hub_artifact_creation().await;
                }
                hub_action::CANCEL_ARTIFACT_CREATION => {
                    if let Some(operation) = self.directory.hub_workspace.creation.operation.as_mut().filter(|operation| !hub_artifact_creation_terminal(operation.phase)) {
                        operation.cancel_requested = true;
                    }
                    self.pump_hub_artifact_creation().await;
                }
                hub_action::OPEN_CREATED_ARTIFACT => {
                    self.open_created_hub_artifact().await;
                }
                _ => {}
            }
            let _ = self.as_window_actions().refresh_ui(UiDirtyScope::Full).await;
        })
    }

    /// 🌱️ Drives the one creation by at most one bounded hub request per frame — its submission, its
    /// cancellation, or a status poll after the shared backoff ([`crate::hub_connection::space_artifact_creation_poll_delay_ms`]) —
    /// and opens the artifact once the receipt is `Ready`. The creation is followed for as long as the hub
    /// answers; only a hub silent past the shared bound makes it `Indeterminate` (the hub may still finish
    /// it), never `Failed` — React's worker follows the same contract (`🌱️creation-polling`). A `409` on the
    /// submission means the catalog generation moved: the door re-reads the catalog instead of
    /// retrying. Answers whether the door changed.
    pub async fn pump_hub_artifact_creation(&mut self) -> bool {
        let Some(operation) = self.directory.hub_workspace.creation.operation.clone() else { return false };
        if hub_artifact_creation_terminal(operation.phase) {
            if operation.ready.is_some() && operation.opening == HubArtifactOpening::Idle {
                self.open_created_hub_artifact().await;
                return true;
            }
            if operation.opening == HubArtifactOpening::Opening && !self.opening.document_opening.as_ref().is_some_and(ShellDocumentOpening::running) {
                let opened = operation.ready.as_ref().is_some_and(|ready| self.sync.sync_channel.as_ref().is_some_and(|channel| channel.document_id == ready.artifact_id));
                if let Some(current) = self.directory.hub_workspace.creation.operation.as_mut() {
                    current.opening = if opened { HubArtifactOpening::Opened } else { HubArtifactOpening::Failed };
                }
                return true;
            }
            return false;
        }
        let Some(client) = self.directory.directory_client.clone() else { return false };
        let now_ms = directory_now_ms();
        if semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::space_artifact_creation_unreachable(operation.last_answered_at_ms, now_ms) {
            if let Some(current) = self.directory.hub_workspace.creation.operation.as_mut() {
                current.phase = SpaceArtifactCreationPhaseV1::Indeterminate;
            }
            return true;
        }
        let request_id = operation.intent.request_id.as_str();
        let context = self.directory.directory_command_ctx();
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
            let Some(current) = self.directory.hub_workspace.creation.operation.as_mut().filter(|current| current.intent.request_id == operation.intent.request_id) else { return false };
            current.submitted = true;
            current.cancel_sent |= cancellation;
            let answered_at_ms = directory_now_ms();
            current.next_poll_at_ms = answered_at_ms.saturating_add(semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::space_artifact_creation_poll_delay_ms(current.polls));
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
            let operation = self.directory.hub_workspace.creation.operation.take();
            self.directory.open_hub_artifact_creation(&space_id).await;
            self.directory.hub_workspace.creation.operation = operation;
        }
        true
    }

    /// 🚪️ Opens a ready artifact through the ordinary document-open relay, bound to its space, with
    /// the dialect and schema the hub's receipt names; the relay resolves and mounts the owning app.
    /// The relay's open is frame-pumped, so the door stays `Opening` until it settles, and is `Opened`
    /// only when this shell's document binding then names the created artifact
    /// ([`Self::pump_hub_artifact_creation`]).
    pub async fn open_created_hub_artifact(&mut self) {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

        let Some((ready, space_id)) = self.directory.hub_workspace.creation.operation.as_ref().and_then(|operation| operation.ready.clone().map(|ready| (ready, operation.space_id.clone()))) else { return };
        if let Some(operation) = self.directory.hub_workspace.creation.operation.as_mut() {
            operation.opening = HubArtifactOpening::Opening;
        }
        let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: ready.parent_dialect.artifact_kind.clone(), standard: ready.parent_dialect.standard.clone(), subset: ready.parent_dialect.subset.clone() };
        let args = serde_json::json!({ "artifactRef": dialect.to_coordinate(), "documentId": ready.artifact_id, "schema": ready.artifact_schema, "spaceId": space_id });
        self.handle_open_artifact_relay("os.open-artifact", Some(&args)).await;
    }

    //#endregion 🔐️HubWorkspaceLane
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
    pub async fn handle_open_artifact_relay(&mut self, action_id: &str, args: Option<&Value>) {
        let target = match open_artifact_relay_target(action_id, args) {
            Ok(target) => target,
            Err(error) => {
                debug_log(&format!("[TRACE] wgpu shell os.open-artifact relay rejected: {error}"));
                return;
            }
        };
        if self.opening.document_opening.as_ref().is_some_and(ShellDocumentOpening::running) {
            let is_de = self.prefs.locale_id == "de";
            self.chrome.show_transient_notice(shell_chrome_string("document.open.busy", is_de), semio_framework::Severity::Info, Some("document.open.busy"));
            return;
        }
        if let Some(space_id) = target.space_id.clone() {
            self.opening.open_space_id = Some(space_id);
        }
        #[cfg(not(target_arch = "wasm32"))]
        if self.opening.begin_document_resolution(&target, &self.plugins, &self.prefs, &self.directory) {
            return;
        }
        let switch = match (&target.plugin_id, &target.app_id) {
            (Some(plugin_id), Some(app_id)) => match self.plugins.install_plugin(plugin_id).await {
                Err(error) => {
                    debug_log(&format!("[TRACE] wgpu shell os.open-artifact could not install {plugin_id}: {error}"));
                    None
                }
                Ok(()) => self.plugins.plugins.iter().find(|entry| entry.plugin_id == *plugin_id).and_then(|program| program.manifest.apps.iter().find(|app| app.id == *app_id).cloned()).map(|app| (plugin_id.clone(), app)),
            },
            _ => self.plugins.resolve_activation_owner_app(&target.dialect, target.role).await,
        };
        let document = match (target.document_id, target.schema) {
            (Some(document_id), Some(schema)) => Some(ShellDocumentOpenTarget { document_id, schema, checkpoint: None }),
            _ => None,
        };
        match switch {
            Some((plugin_id, app)) if !self.docs.session.as_ref().is_some_and(|session| session.plugin_id == plugin_id && session.app.id == app.id) => {
                let Some(program) = self.plugins.plugins.iter().find(|entry| entry.plugin_id == plugin_id).cloned() else {
                    debug_log(&format!("[TRACE] wgpu shell os.open-artifact could not switch to {plugin_id}: program missing"));
                    return;
                };
                let (terminology, locale) = (self.prefs.active_terminology(), self.prefs.active_locale());
                let label = app.label.resolve(terminology, locale).to_string();
                let app_id = app.id.clone();
                let pending = ShellDetached::spawn(async move { program.create_app(&app_id).await.map(ShellDocumentOpenAnswer::Instantiated) });
                self.opening.document_opening = Some(ShellDocumentOpening::new(label, ShellDocumentOpenPhase::Instantiating, plugin_id, app.id, document, Some(pending)));
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
    pub async fn begin_document_seed(&mut self, document: ShellDocumentOpenTarget) {
        let Some(session) = self.docs.session.clone() else {
            self.opening.fail_document_opening("document open has no mounted session".to_string(), &mut self.chrome, &self.prefs);
            return;
        };
        let label = session.app.label.resolve(self.prefs.active_terminology(), self.prefs.active_locale()).to_string();
        let opening = self.opening.document_opening.take();
        let (bindings, surface) = self.opening.default_bindings_for_current_session(&self.directory, &self.docs);
        let prepared = match self.prepare_document_open(document.document_id, document.schema, bindings, surface, None, document.checkpoint).await {
            Ok(prepared) => prepared,
            Err(error) => {
                self.opening.document_opening = opening;
                self.opening.fail_document_opening(error, &mut self.chrome, &self.prefs);
                return;
            }
        };
        let seed_cancel = prepared.seed.as_ref().map(|seed| seed.ctx.cancel.clone());
        let pending = ShellDetached::spawn({
            let (plugin, instance_id, seed) = (prepared.plugin.clone(), prepared.session.instance_id, prepared.seed.clone());
            async move { seed_hub_document(plugin, instance_id, seed).await.map(ShellDocumentOpenAnswer::Seeded).map_err(semio_framework_os_renderer_wgpu_program_bridge::ProgramFault::from) }
        });
        let mut opening = opening.unwrap_or_else(|| ShellDocumentOpening::new(label, ShellDocumentOpenPhase::Seeding, session.plugin_id.clone(), session.app.id.clone(), None, None));
        opening.phase = ShellDocumentOpenPhase::Seeding;
        opening.cancel = seed_cancel;
        opening.pending = Some(pending);
        opening.prepared = Some(prepared);
        self.opening.document_opening = Some(opening);
    }

    /// 🧩️ Continues a resolved hub open: a hub-resolved program replaces the local one of its plugin (a local
    /// program no longer mounts a hub document it is not the lease's component for), then the app instance
    /// is created on it; a local program that already is the lease's component seeds the document into the
    /// mounted session when it runs the app already.
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn continue_resolved_open(&mut self, mut opening: ShellDocumentOpening, resolved: Option<ProgramBridgeEntry>) {
        let fresh = resolved.is_some();
        if let Some(program) = resolved {
            self.plugins.plugins.retain(|entry| entry.plugin_id != program.plugin_id);
            self.plugins.plugins.push(program);
        }
        let Some(program) = self.plugins.plugins.iter().find(|entry| entry.plugin_id == opening.plugin_id).cloned() else {
            self.opening.document_opening = Some(opening);
            self.opening.fail_document_opening("the resolved plugin is not mounted".to_string(), &mut self.chrome, &self.prefs);
            return;
        };
        let Some(app) = program.manifest.apps.iter().find(|app| app.id == opening.app_id).cloned() else {
            let reason = format!("{} declares no app {}", program.plugin_id, opening.app_id);
            self.opening.document_opening = Some(opening);
            self.opening.fail_document_opening(reason, &mut self.chrome, &self.prefs);
            return;
        };
        opening.label = app.label.resolve(self.prefs.active_terminology(), self.prefs.active_locale()).to_string();
        if !fresh && self.docs.session.as_ref().is_some_and(|session| session.plugin_id == program.plugin_id && session.app.id == app.id) {
            let document = opening.document.take();
            self.opening.document_opening = Some(opening);
            if let Some(document) = document {
                self.begin_document_seed(document).await;
            }
            return;
        }
        let app_id = app.id.clone();
        let (terminology, locale) = (self.prefs.active_terminology(), self.prefs.active_locale());
        opening.pending = Some(ShellDetached::spawn(async move { program.create_app(&app_id).await.map(ShellDocumentOpenAnswer::Instantiated) }));
        opening.phase = ShellDocumentOpenPhase::Instantiating;
        self.opening.document_opening = Some(opening);
    }

    /// ⏭️ Advances the retained open by the one detached step that answered, if any: an instance mounts
    /// as the session (or is destroyed when the open was cancelled), a seeded document binds, and every
    /// render the open owes goes to the settle lane. Called once per frame from [`Self::pump_sync_events`];
    /// answers whether the open moved.
    pub async fn advance_document_opening(&mut self) -> bool {
        let Some(answer) = self.opening.document_opening.as_ref().and_then(|opening| opening.pending.as_ref()).and_then(ShellDetached::take) else { return false };
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
                self.continue_resolved_open(opening, program).await;
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
                        self.begin_document_seed(document).await;
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
                match self.finish_document_open(prepared).await {
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
        changed |= self.pump_hub_artifact_creation().await;
        changed |= self.opening.pump_hub_check_in(&self.directory, &mut self.sync).await;
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
                            home.retry_at_ms = semio_framework_os_renderer_wgpu_host_base::renderer_worker_pool().now_ms().saturating_add(DirectoryHomeProjection::RETRY_DELAY_MS);
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
                            } else if home.retry(semio_framework_os_renderer_wgpu_host_base::renderer_worker_pool().now_ms(), &expected.receipt_sha256).is_err() {
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

    //#endregion 🔖️DirectoryAndIdentity
}
//#endregion ShellActions
