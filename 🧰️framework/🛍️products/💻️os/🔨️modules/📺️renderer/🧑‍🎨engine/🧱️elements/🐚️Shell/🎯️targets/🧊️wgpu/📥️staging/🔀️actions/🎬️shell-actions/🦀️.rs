//! 🔀️ `shell-actions` staging of `shell-actions`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

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
pub fn shell_turn<'a, R>(turn: impl std::future::Future<Output = R> + Send + 'a) -> ShellTurn<'a, R> {
    Box::pin(turn)
}

/// 📦️ See the native [`shell_turn`].
#[cfg(target_arch = "wasm32")]
pub fn shell_turn<'a, R>(turn: impl std::future::Future<Output = R> + 'a) -> ShellTurn<'a, R> {
    Box::pin(turn)
}

impl<'ctx> ActionsCtx<'ctx> {
    //#region 🔖️NativeBackboneSync
    /// ✂️ Retires the guest's exact generation before stopping the actor. A refusal still
    /// closes the local owner and is returned to the caller; it is never retried or hidden.
    pub async fn detach_sync_backbone_internal(&mut self) -> Result<(), String> {
        #[cfg(not(target_arch="wasm32"))]
        self.directory.cancel_inference_port();
        self.opening.document_execution_target_lease=None;
        let owner = self.sync.active_sync_owner();
        let retirement = if let Some(owner) = owner.as_ref() {
            match self.plugins.plugins.iter().find(|entry| entry.plugin_id == owner.plugin_id).cloned() {
                Some(plugin) => plugin.retire_document_backbone(owner.instance_id, owner.binding_generation, &owner.actor_uri).await,
                None => Err("document-backbone plugin owner is missing".into()),
            }
        } else {
            Ok(Vec::new())
        };
        let retirement = retirement.map_err(|refusal| self.prefs.note_refused_open(refusal, &mut self.chrome));
        if let (Some(owner), Ok(effects)) = (owner.as_ref(), retirement.as_ref()) {
            if self.sync.active_sync_owner().as_ref().is_some_and(|active| shell_sync_owner_matches(active, owner)) {
                if let Some(channel) = self.sync.sync_channel.as_ref() {
                    let remaining = route_document_backbone_effects(&channel.actor_uri, &channel.cmd_tx, effects.clone())?;
                    if let Some(controller_id) = self.docs.session.as_ref().map(|session| session.app.controller_id.clone()) {
                        self.as_window_actions().queue_host_effects(&controller_id, remaining);
                    }
                }
            }
        }
        #[cfg(not(target_arch="wasm32"))]
        self.directory.cancel_inference_port();
        self.opening.document_execution_target_lease=None;
        if let Some(channel) = self.sync.sync_channel.take() {
            self.sync.hub_documents.remove(&shell_hub_document_key(&channel.document_key));
            let _ = channel.cmd_tx.send(ArtifactActorMsg::Detach);
            self.sync.document_host.close_key(&channel.document_key);
        }
        self.sync.sync_status = None;
        self.sync.sync_bootstrap_progress = None;
        self.sync.sync_reseed = None;
        self.directory.presence_peers.clear();
        self.directory.presence_surface = None;
        self.directory.presence_self = None;
        retirement.map(|_| ())
    }

    //#endregion 🔖️NativeBackboneSync
    //#region 🔖️CheckIn
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
    pub fn observe_invocation_history<'a>(&'a mut self, history_patch: Option<&'a semio_framework::kernel::HistoryPatch>) -> ShellTurn<'a, ()> {
        shell_turn(async move {
            let Some(patch) = history_patch else { return };
            if patch.cursor >= self.history.history_cursor {
                self.as_session().observe_history_time_travel(patch.time_travel.as_ref());
                self.history.observe_history_reprojection(patch.reprojection.as_ref());
            }
            if !fold_history_patch(&mut self.history.history_entries, &mut self.history.history_cursor, patch, false) {
                return;
            }
            let count = uncommitted_edit_count(&self.history.history_entries);
            if count == 0 {
                self.sync.last_uncommitted_edit_at_ms = None;
                self.sync.auto_checkin_pending = false;
            } else {
                self.sync.last_uncommitted_edit_at_ms = Some(chrome_now_ms() as i64);
            }
            let landed = checkpoint_landed(self.history.history_current_checkpoint_id.as_deref(), patch.current_checkpoint_id.as_deref(), self.sync.checkpoint_dispatched);
            if patch.current_checkpoint_id.is_some() {
                self.history.history_current_checkpoint_id = patch.current_checkpoint_id.clone();
            }
            if !landed {
                return;
            }
            self.sync.checkpoint_dispatched = false;
            #[cfg(not(target_arch = "wasm32"))]
            {
                let Some(space_id) = self.opening.open_space_id.clone() else { return };
                let Some(document_id) = self.sync.sync_channel.as_ref().map(|channel| channel.document_id.clone()) else { return };
                self.sync.request_hub_check_in(&space_id, &document_id);
                if document_id != SPACE_INDEX_DOCUMENT_ID {
                    self.touch_space_index_artifact(&space_id, &document_id).await;
                }
            }
        })
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
            self.dispatch_checkpoint("auto").await;
        }
    }

    /// 📌️ ticket §C5 items 2/3/4 — fires `commitCheckpoint` through the same action funnel the
    /// History panel's own "Checkpoint" button uses (`history_command` in `🔌️plugin/🦀️.rs`),
    /// with `authors` riding along (`history_command` still hardcodes `authors: Vec::new()` today per
    /// `📓️w3-a-report.md`'s own `sharedFileRequest` — sent anyway so this is ready the moment that
    /// lands). Viewer-guarded (item 5) so a stray call from `checkpoint_before_detach`/`poll_auto_checkin`
    /// can never reach a viewer session even if their own callers' guards were ever relaxed.
    pub async fn dispatch_checkpoint(&mut self, message: &str) {
        let Some(session) = self.docs.session.clone() else { return };
        if !can_check_in(session.app.role) {
            return;
        }
        self.sync.checkpoint_dispatched = true;
        let authors: Vec<semio_framework::DslValue> = self.directory.identity.as_ref().map(|identity| vec![semio_framework::dsl_value!({ "id": identity.user_id, "name": identity.display_name })]).unwrap_or_default();
        let action = ActionDescriptor { controller_id: session.app.controller_id.clone(), action: "commitCheckpoint".into(), args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "message": message, "authors": authors }) };
        if let Err(error) = self.dispatch_action(action).await {
            debug_log(&format!("[TRACE] wgpu shell check-in checkpoint dispatch failed: {error}"));
            self.sync.checkpoint_dispatched = false;
        }
    }

    /// 📌️ ticket §C5 item 4 — checkpoint-on-close: called right before EVERY
    /// `detach_sync_backbone_internal()` call site (this shell keeps exactly one document mounted at a
    /// time, so attaching a different backbone IS closing whatever was open — same posture the React
    /// shell's own report documents). Best-effort (mirrors React's "fire-and-forget, not gated on the
    /// success-detection effect" note) — `dispatch_checkpoint`/`observe_invocation_history` still run
    /// the normal detection+`TouchArtifact` path; this call site just doesn't await or fail the caller
    /// on their outcome, since a failed close-time checkpoint must never block navigating away.
    pub async fn checkpoint_before_detach(&mut self) {
        let Some(session) = self.docs.session.as_ref() else { return };
        let count = uncommitted_edit_count(&self.history.history_entries);
        if !should_checkpoint_before_detach(session.app.role, self.sync.sync_channel.is_some(), count) {
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
    pub async fn touch_space_index_artifact(&mut self, space_id: &str, artifact_id: &str) {
        let now_ms = chrome_now_ms();
        let actor = self.directory.identity.as_ref().map(|identity| identity.user_id.clone()).unwrap_or_else(|| self.directory.shell_session_id.clone());
        let arguments: BTreeMap<String, DslValue> = BTreeMap::from([("id".to_string(), semio_framework_value::DslValue::String(artifact_id.to_string())), ("nowMs".to_string(), semio_framework_value::DslValue::float(now_ms)), ("actor".to_string(), semio_framework_value::DslValue::String(actor.clone()))]);
        if self.sync.sync_channel.as_ref().map(|channel| channel.document_id.as_str()) == Some(SPACE_INDEX_DOCUMENT_ID) && self.opening.open_space_id.as_deref() == Some(space_id) {
            let Some(session) = self.docs.session.clone() else { return };
            let Some(program) = self.plugins.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).cloned() else { return };
            let Some(app) = find_space_index_app(&program, semio_framework::manifest::AppRole::Editor).cloned() else { return };
            let invocation = semio_framework::manifest::CommandInvocation {
                address: semio_framework::manifest::CommandAddress { owner: semio_framework::manifest::CommandOwnerAddress::App { plugin_id: session.plugin_id.clone(), app_id: app.id.clone() }, command_id: "touchArtifact".into() },
                arguments,
            };
            let command_json = semio_framework_pack_json::to_json_string(&invocation);
            match program.handle_command(session.instance_id, &command_json, &session.view_state).await {
                Ok(result) => self.as_window_actions().queue_host_effects(&session.app.controller_id, result.requested_effects),
                Err(error) => { let detail = self.prefs.note_refused_open(error, &mut self.chrome); debug_log(&format!("[TRACE] wgpu shell touchArtifact (live session) failed: {detail}")); },
            }
            return;
        }
        if self.sync.sync_channel.as_ref().map(|channel| channel.document_id.as_str()) == Some(SPACE_INDEX_DOCUMENT_ID) {
            debug_log(&format!("[TRACE] wgpu shell touchArtifact skipped: main session already holds the shared \"index\" actor for a different space"));
            return;
        }
        let Some(program) = self.plugins.plugins.iter().find(|entry| find_space_index_app(entry, semio_framework::manifest::AppRole::Editor).is_some()).cloned() else { return };
        let Some(app) = find_space_index_app(&program, semio_framework::manifest::AppRole::Editor).cloned() else { return };
        let Some(space_index) = program_space_index(&program) else { return };
        let instance_id = match program.create_app(&app.id).await {
            Ok(id) => id,
            Err(error) => {
                debug_log(&format!("[TRACE] wgpu shell touchArtifact background instance failed: {error}"));
                return;
            }
        };
        let data_dir = self.directory.identity_env.as_ref().and_then(|env| env.data_dir.as_deref());
        let surface = semio_framework::manifest::surface_app_id(&app.dialect, semio_framework::manifest::AppRole::Editor);
        let bindings = default_persistence_bindings(self.directory.identity.as_ref(), Some(space_id), data_dir, Some(surface.as_str()));
        let actor_uri = format!("actor://{SPACE_INDEX_DOCUMENT_ID}");
        if let Err(error) = bind_wgpu_document_socket_surface(&self.sync.document_host, SPACE_INDEX_DOCUMENT_ID, &space_index.document_schema, &bindings, &program, &app, &app.window_kinds.first().id) {
            debug_log(&format!("[TRACE] wgpu shell touchArtifact document admission failed: {error}"));
            program.destroy_app(instance_id);
            return;
        }
        let channels = self.sync.document_host.open(ArtifactActorConfig { actor_identity_grant: semio_framework_value::RetainedCloneGrant {maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64},  document_id: SPACE_INDEX_DOCUMENT_ID.to_string(), schema: S_SPACE_INDEX_DOCUMENT_SCHEMA.to_string(), bindings, watch_external: true, actor: actor.clone() }).await;
        let binding_generation = match self.sync.mint_sync_binding_generation() {
            Ok(generation) => generation,
            Err(error) => {
                debug_log(&format!("[TRACE] wgpu shell touchArtifact binding generation failed: {error}"));
                self.sync.document_host.close_key(&channels.document_key);
                program.destroy_app(instance_id);
                return;
            }
        };
        let binding_effects = match program.bind_document_backbone(instance_id, binding_generation, &actor_uri).await {
            Ok(effects) => effects,
            Err(error) => {
                let detail = self.prefs.note_refused_open(error, &mut self.chrome);
                if let Err(refusal) = program.retire_document_backbone(instance_id, binding_generation, &actor_uri).await { self.prefs.note_refused_open(refusal, &mut self.chrome); }
                debug_log(&format!("[TRACE] wgpu shell touchArtifact document-backbone bind failed: {detail}"));
                let _ = channels.cmd_tx.send(ArtifactActorMsg::Detach);
                self.sync.document_host.close_key(&channels.document_key);
                program.destroy_app(instance_id);
                return;
            }
        };
        match route_document_backbone_effects(&actor_uri, &channels.cmd_tx, binding_effects) {
            Ok(remaining) => self.as_window_actions().queue_host_effects(&app.controller_id, remaining),
            Err(error) => debug_log(&format!("[TRACE] wgpu shell touchArtifact document-backbone egress failed: {error}")),
        }
        let _ = channels.cmd_tx.send(ArtifactActorMsg::LocalMutations { envelopes: Vec::new() });
        let invocation = semio_framework::manifest::CommandInvocation {
            address: semio_framework::manifest::CommandAddress { owner: semio_framework::manifest::CommandOwnerAddress::App { plugin_id: program.plugin_id.clone(), app_id: app.id.clone() }, command_id: "touchArtifact".into() },
            arguments,
        };
        let command_json = semio_framework_pack_json::to_json_string(&invocation);
        let view_state = ViewModel::new(self.prefs.active_locale(), self.prefs.active_terminology());
        match program.handle_command(instance_id, &command_json, &view_state).await {
            Ok(result) => match route_document_backbone_effects(&actor_uri, &channels.cmd_tx, result.requested_effects) {
                Ok(remaining) => self.as_window_actions().queue_host_effects(&app.controller_id, remaining),
                Err(error) => debug_log(&format!("[TRACE] wgpu shell touchArtifact document-backbone egress failed: {error}")),
            },
            Err(error) => { let detail = self.prefs.note_refused_open(error, &mut self.chrome); debug_log(&format!("[TRACE] wgpu shell touchArtifact dispatch failed: {detail}")); },
        }
        if let Err(error) = program.retire_document_backbone(instance_id, binding_generation, &actor_uri).await {
            let detail = self.prefs.note_refused_open(error, &mut self.chrome);
            debug_log(&format!("[TRACE] wgpu shell touchArtifact document-backbone retirement failed: {detail}"));
        }
        let _ = channels.cmd_tx.send(ArtifactActorMsg::Detach);
        self.sync.document_host.close_key(&channels.document_key);
        program.destroy_app(instance_id);
    }

    /// 📌️ ticket §C5 item 3 — explicit check-in's message-prompt dialog funnel (`#s-checkin`'s own
    /// dedicated controller, routed alongside `"framework.sync"` in `dispatch_action`): `open` shows
    /// the dialog, `cancel` closes it without dispatching, `submit` dispatches with the typed message
    /// (falling back to `"check-in"` for an empty/whitespace-only draft, matching the React shell's own
    /// `submitCheckin`).
    pub fn handle_checkin_action<'a>(&'a mut self, action: ActionDescriptor) -> ShellTurn<'a, Result<(), String>> {
        shell_turn(async move {
            match action.action.as_str() {
                "open" => {
                    self.sync.checkin_dialog_draft = Some(String::new());
                    Ok(())
                }
                "cancel" => {
                    self.sync.checkin_dialog_draft = None;
                    Ok(())
                }
                "submit" => {
                    let message = action.args.as_ref().and_then(|args| args.get("message")).and_then(semio_framework_value::DslValue::as_str).map(str::trim).filter(|value| !value.is_empty()).unwrap_or("check-in").to_string();
                    self.sync.checkin_dialog_draft = None;
                    self.dispatch_checkpoint(&message).await;
                    Ok(())
                }
                _ => Ok(()),
            }
        })
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
    pub async fn attach_sync_backbone(&mut self, uri: String) -> Result<(), String> {
        let session = self.docs.session.clone().ok_or("session missing")?;
        let (document_id, bindings, surface) = match parse_remote_backbone_uri(&uri) {
            Some(remote) => {
                let surface = semio_framework::manifest::surface_app_id(&session.app.dialect, session.app.role);
                (remote.document_id, vec![PersistenceBinding::Hub { base_url: format!("http://{}", remote.host_port), space_id: remote.space_id, surface: Some(surface.clone()) }], Some(surface))
            }
            None => {
                let Some(identity) = self.docs.sync_program_identity() else {
                    let text = local_folder_text(LocalFolderText::Unidentified, "", self.prefs.active_locale());
                    self.chrome.show_transient_notice(text, semio_framework::Severity::Warning, Some(SYNC_DOCUMENT_UNIDENTIFIED_CODE));
                    return Ok(());
                };
                #[cfg(target_arch = "wasm32")]
                if !SHELL_DOCUMENT_TRANSPORTS.folder && is_host_route_uri(&uri) {
                    return self.as_window_actions().attach_host_route_backbone(uri, identity).await;
                }
                (identity.document_id, parse_persistence_binding(&uri)?, None)
            }
        };
        let folder = bindings.iter().find_map(|binding| match binding {
            PersistenceBinding::Folder { path } => Some(path.display().to_string()),
            PersistenceBinding::Hub { .. } => None,
        });
        self.open_document(document_id.clone(), session.app.io.artifact_schema.clone(), bindings, surface, Some(uri)).await?;
        let Some(path) = folder else { return Ok(()) };
        self.history.ensure_local_folder_bindings();
        self.history.remember_local_folder(semio_framework_os_config::opening_config::mutations::LocalFolderBinding {
            document_id: document_id.clone(),
            plugin_id: session.plugin_id.clone(),
            app_id: session.app.id.clone(),
            folder: semio_framework_os_config::opening_config::mutations::LocalFolderRef::Path { path: path.clone() },
        });
        #[cfg(not(target_arch = "wasm32"))]
        self.as_window_actions().restore_folder_archive(&path, &document_id).await?;
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
    pub async fn open_document(&mut self, document_id: String, schema: String, bindings: Vec<PersistenceBinding>, surface: Option<String>, backbone_uri: Option<String>) -> Result<(), String> {
        let prepared = self.prepare_document_open(document_id, schema, bindings, surface, backbone_uri, None).await?;
        let pair = seed_hub_document(prepared.plugin.clone(), prepared.session.instance_id, prepared.seed.clone()).await.map_err(|refusal| self.prefs.note_refused_open(refusal, &mut self.chrome))?;
        self.sync.install_document_seed(&prepared, pair)?;
        self.finish_document_open(prepared).await?;
        self.as_window_actions().refresh_ui(UiDirtyScope::Full).await
    }

    /// 🧷️ The host half of an open, before the guest holds the document: the mounted document is
    /// checkpointed and retired, the socket surface and the execution target bound.
    ///
    /// 🎠️ H3-wgpu-native — `wasm_runtime()`/`register_host_backbone` retired, see
    /// `detach_sync_backbone_internal`'s note.
    /// 📌️ ticket §C5 item 4 — checkpoint-on-close, same "switch away IS close" posture as
    /// `attach_sync_backbone` above (this shell keeps exactly one document mounted at a time).
    pub async fn prepare_document_open(
        &mut self,
        document_id: String,
        schema: String,
        bindings: Vec<PersistenceBinding>,
        surface: Option<String>,
        backbone_uri: Option<String>,
        checkpoint: Option<semio_framework_os_kernel::os_directory::DocumentOpenCheckpointV1>,
    ) -> Result<ShellPreparedDocumentOpen, String> {
        document_bindings_admitted(SHELL_DOCUMENT_TRANSPORTS, &bindings)?;
        let session = self.docs.session.clone().ok_or("session missing")?;
        let plugin = self.plugins.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).cloned().ok_or("plugin missing")?;
        let window_id = self.windows.active_window_id.as_deref().or(session.view_state.window_id.as_deref()).unwrap_or_else(|| session.app.window_kinds.first().id.as_str());
        let window_kind_id = self.dock.live_window_kind_id(&session, window_id).unwrap_or_else(|| session.app.window_kinds.first().id.as_str()).to_string();
        self.checkpoint_before_detach().await;
        self.detach_sync_backbone_internal().await?;
        self.directory.presence_surface = surface;
        bind_wgpu_document_socket_surface(&self.sync.document_host, &document_id, &schema, &bindings, &plugin, &session.app, &window_kind_id)?;
        let seed = self.opening.bind_document_execution_target(&document_id, &schema, &bindings, &plugin, &session.app, &window_kind_id, checkpoint, &mut self.directory, &self.sync).await?;
        Ok(ShellPreparedDocumentOpen { document_id, schema, bindings, backbone_uri, seed, plugin, session })
    }

    /// 🔗️ Binds the prepared document once the guest holds it: the document actor opens, the guest's
    /// backbone binds to it and the history projection re-seeds. The caller owes the refresh.
    pub async fn finish_document_open(&mut self, prepared: ShellPreparedDocumentOpen) -> Result<(), String> {
        let ShellPreparedDocumentOpen { document_id, schema, bindings, backbone_uri, plugin, session, .. } = prepared;
        let actor_uri = format!("actor://{document_id}");
        let actor = self.directory.current_shell_actor(session.instance_id);
        let channels = self.sync.document_host.open(ArtifactActorConfig { actor_identity_grant: semio_framework_value::RetainedCloneGrant {maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64},  document_id: document_id.clone(), schema, bindings, watch_external: true, actor }).await;
        let events = self.sync.document_host.subscribe_key(&channels.document_key).await;
        let binding_generation = self.sync.mint_sync_binding_generation()?;
        let binding_effects = match plugin.bind_document_backbone(session.instance_id, binding_generation, &actor_uri).await {
            Ok(effects) => effects,
            Err(error) => {
                let detail = self.prefs.note_refused_open(error, &mut self.chrome);
                if let Err(refusal) = plugin.retire_document_backbone(session.instance_id, binding_generation, &actor_uri).await { self.prefs.note_refused_open(refusal, &mut self.chrome); }
                let _ = channels.cmd_tx.send(ArtifactActorMsg::Detach);
                self.sync.document_host.close_key(&channels.document_key);
                return Err(detail);
            }
        };
        let cmd_tx = channels.cmd_tx.clone();
        let _ = cmd_tx.send(ArtifactActorMsg::LocalMutations { envelopes: Vec::new() });
        self.sync.sync_link = DocumentLinkStatus::Linked;
        self.sync.sync_backbone_uri = Some(backbone_uri.unwrap_or_else(|| actor_uri.clone()));
        self.sync.sync_channel =
            Some(ShellSyncChannel { document_id, document_key: channels.document_key, actor_uri, instance_id: session.instance_id, plugin_id: session.plugin_id.clone(), binding_generation, cmd_tx, events, connected_at_ms: chrome_now_ms() as i64 });
        let channel = self.sync.sync_channel.as_ref().expect("new sync channel is installed");
        let remaining = match route_document_backbone_effects(&channel.actor_uri, &channel.cmd_tx, binding_effects) {
            Ok(remaining) => remaining,
            Err(error) => {
                let _ = self.detach_sync_backbone_internal().await;
                return Err(error);
            }
        };
        self.as_window_actions().queue_host_effects(&session.app.controller_id, remaining);
        if let Some(error) = self.sync.sync_terminal_fault.take() {
            let _ = self.detach_sync_backbone_internal().await;
            return Err(error);
        }
        self.sync.sync_status = Some(ArtifactSyncStatus::default());
        if let Some(document_key) = self.sync.sync_channel.as_ref().map(|channel| shell_hub_document_key(&channel.document_key)) {
            self.sync.publish_hub_document_status(document_key, ShellHubRemoteV1::Detached);
        }
        self.sync.sync_card_kind = None;
        self.as_session().refresh_history_snapshot().await;
        Ok(())
    }

    /// 📌️ ticket §C5 item 4 — the one EXPLICIT close (a user-initiated "Detach"), same
    /// checkpoint-before-detach guard as the two switch-triggered closes above.
    pub fn handle_sync_action<'a>(&'a mut self, action: ActionDescriptor) -> ShellTurn<'a, Result<(), String>> {

        shell_turn(async move {
            match action.action.as_str() {
                "selectFile" => {
                    self.sync.sync_card_kind = Some("file".into());
                    self.sync.sync_card_draft = self.sync.sync_backbone_uri.as_deref().filter(|uri| uri.starts_with("file://")).map(|uri| uri.trim_start_matches("file://").to_string()).unwrap_or_default();
                    #[cfg(not(target_arch = "wasm32"))]
                    self.surfaces.schedule_sync_path_pick(pick_file_path(), &mut self.settle);
                    #[cfg(target_arch = "wasm32")]
                    if let Some(path) = pick_file_path().await {
                        self.sync.sync_card_draft = path;
                    }
                    Ok(())
                }
                "selectFolder" => {
                    self.sync.sync_card_kind = Some("folder".into());
                    self.sync.sync_card_draft = self.sync.sync_backbone_uri.as_deref().filter(|uri| uri.starts_with("folder://")).map(|uri| uri.trim_start_matches("folder://").to_string()).unwrap_or_default();
                    #[cfg(not(target_arch = "wasm32"))]
                    self.surfaces.schedule_sync_path_pick(pick_folder(), &mut self.settle);
                    #[cfg(target_arch = "wasm32")]
                    if let Some(path) = pick_folder().await {
                        self.sync.sync_card_draft = path;
                    }
                    Ok(())
                }
                "browse" => match self.sync.sync_card_kind.as_deref() {
                    Some("folder") => {
                        #[cfg(not(target_arch = "wasm32"))]
                        self.surfaces.schedule_sync_path_pick(pick_folder(), &mut self.settle);
                        #[cfg(target_arch = "wasm32")]
                        if let Some(path) = pick_folder().await {
                            self.sync.sync_card_draft = path;
                        }
                        Ok(())
                    }
                    Some("file") => {
                        #[cfg(not(target_arch = "wasm32"))]
                        self.surfaces.schedule_sync_path_pick(pick_file_path(), &mut self.settle);
                        #[cfg(target_arch = "wasm32")]
                        if let Some(path) = pick_file_path().await {
                            self.sync.sync_card_draft = path;
                        }
                        Ok(())
                    }
                    _ => Ok(()),
                },
                "setSyncDraft" => {
                    if let Some(path) = action.args.as_ref().and_then(|args| args.get("path").or_else(|| args.get("value"))).and_then(|value| value.as_str()) {
                        self.sync.sync_card_draft = path.to_string();
                    }
                    Ok(())
                }
                "selectRemote" => {
                    self.sync.sync_card_kind = Some("remote".into());
                    self.sync.sync_card_draft = self.sync.sync_backbone_uri.as_deref().filter(|uri| uri.starts_with("remote://")).map(|uri| uri.trim_start_matches("remote://").to_string()).unwrap_or_default();
                    Ok(())
                }
                "attach" => {
                    let path = action.args.as_ref().and_then(|args| args.get("path")).and_then(|value| value.as_str()).unwrap_or(self.sync.sync_card_draft.as_str());
                    if path.trim().is_empty() {
                        return Ok(());
                    }
                    let kind = action.args.as_ref().and_then(|args| args.get("kind")).and_then(|value| value.as_str()).unwrap_or(self.sync.sync_card_kind.as_deref().unwrap_or("file"));
                    let uri = match kind {
                        "folder" => format!("folder://{path}"),
                        "remote" => format!("remote://{path}"),
                        _ => format!("file://{path}"),
                    };
                    self.attach_sync_backbone(uri).await
                }
                "detach" => {
                    #[cfg(target_arch = "wasm32")]
                    if self.as_session().detach_host_route_folder().await {
                        return Ok(());
                    }
                    let folder_document = self.sync.sync_backbone_uri.as_deref().filter(|uri| uri.starts_with("folder://") || uri.starts_with("file://")).and_then(|_| self.sync.sync_channel.as_ref()).map(|channel| channel.document_id.clone());
                    self.checkpoint_before_detach().await;
                    self.detach_sync_backbone_internal().await?;
                    self.sync.sync_backbone_uri = None;
                    self.sync.sync_card_kind = None;
                    self.opening.last_envelope_dsl = None;
                    if let Some(document_id) = folder_document {
                        self.history.ensure_local_folder_bindings();
                        self.history.forget_local_folder(&document_id);
                    }
                    Ok(())
                }
                FOLDER_RECONNECT_ACTION => {
                    self.history.ensure_local_folder_bindings();
                    let Some(offer) = self.docs.folder_reconnect_offer(&self.history, &self.sync) else { return Ok(()) };
                    if let Err(error) = self.reattach_local_folder(&offer).await {
                        self.prefs.note_dispatch_fault(&error, &mut self.chrome);
                    }
                    Ok(())
                }
                FOLDER_FORGET_ACTION => {
                    self.history.ensure_local_folder_bindings();
                    if let Some(offer) = self.docs.folder_reconnect_offer(&self.history, &self.sync) {
                        self.history.forget_local_folder(&offer.document_id);
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
            self.chrome.chrome_build.refused_guest_fault = None;
            if let Some(owner) = action.args.as_ref().and_then(|args| args.get("windowId")).and_then(semio_framework_value::DslValue::as_str).and_then(window_pane_owner).map(str::to_string) {
                scope_action_to_window(&mut action, &owner);
            }
            semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::note_dispatched_action(&action, self.chrome.dispatch_origin);
            if action.action == semio_framework::onboarding::START_TUTORIAL_ACTION_ID {
                if let Some(tutorial_id) = action.args.as_ref().and_then(|args| args.get("tutorialId")).and_then(|v| v.as_str()) {
                    self.as_window_actions().tutorial_start(tutorial_id);
                }
                return Ok(());
            }
            let record_tutorial_after_acceptance = action.action == semio_framework::onboarding::RECORD_TUTORIAL_ACTION_ID;
            if !self.chrome.chrome_build.tutorial_dispatch_internal {
                self.tutorial.tutorial_note_real_dispatch(&action);
            }
            if let Some(result) = self.as_session().apply_host_panel_selection(&action) {
                return result;
            }
            if !record_tutorial_after_acceptance && action.controller_id != "framework" {
                if let Some(session) = self.docs.action_session(&action, &self.plugins).cloned() {
                    if app_owns_command(&session.app, &action.action) {
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
                            self.prefs.appearance_id = value.to_string();
                            self.note_shell_setting_command("os.setAppearance", Some(value)).await?;
                            self.as_session().republish_shell_panel_document(FRAMEWORK_SETTINGS_GENERAL_TAB_ID)?;
                        }
                        return Ok(());
                    }
                    "setDriver" => {
                        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                            self.prefs.driver_id = value.to_string();
                            self.prefs.clear_driver_draft(&mut self.chrome);
                            self.note_shell_setting_command("os.setDriver", Some(value)).await?;
                            self.as_session().republish_shell_panel_document(FRAMEWORK_SETTINGS_GENERAL_TAB_ID)?;
                        }
                        return Ok(());
                    }
                    "setDriverField" => {
                        if let (Some(key), Some(value)) = (action.args.as_ref().and_then(|args| args.get("key")).and_then(|value| value.as_str()), action.args.as_ref().and_then(|args| args.get("value")).and_then(|value| value.as_str())) {
                            if self.prefs.set_driver_field(key, value, &mut self.chrome) {
                                self.as_session().republish_shell_panel_document(FRAMEWORK_SETTINGS_GENERAL_TAB_ID)?;
                            }
                        }
                        return Ok(());
                    }
                    "setDriverSaveLabel" => {
                        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|value| value.as_str()) {
                            if self.prefs.driver_save_label != value {
                                self.prefs.driver_save_label = value.to_string();
                                self.as_session().republish_shell_panel_document(FRAMEWORK_SETTINGS_GENERAL_TAB_ID)?;
                            }
                        }
                        return Ok(());
                    }
                    "saveDriver" => {
                        let label = self.prefs.driver_save_label.trim().to_string();
                        if let Some(id) = custom_ui_driver_id(&label) {
                            let mut driver = self.prefs.driver_document(&self.chrome);
                            driver.driver_id = id.clone();
                            driver.label = label;
                            self.chrome.chrome_build.preferences.custom_drivers.insert(id.clone(), driver);
                            self.prefs.driver_id = id;
                            self.prefs.driver_save_label.clear();
                            self.prefs.clear_driver_draft(&mut self.chrome);
                            self.as_session().republish_shell_panel_document(FRAMEWORK_SETTINGS_GENERAL_TAB_ID)?;
                        }
                        return Ok(());
                    }
                    "deleteDriver" => {
                        if let Some(id) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|value| value.as_str()).filter(|id| id.starts_with("custom.")) {
                            let changed = self.chrome.chrome_build.preferences.custom_drivers.remove(id).is_some() || self.prefs.driver_id == id || self.prefs.driver_draft.is_some();
                            if self.prefs.driver_id == id {
                                self.prefs.driver_id = "default".into();
                            }
                            self.prefs.clear_driver_draft(&mut self.chrome);
                            if changed {
                                self.as_session().republish_shell_panel_document(FRAMEWORK_SETTINGS_GENERAL_TAB_ID)?;
                            }
                        }
                        return Ok(());
                    }
                    "setLocale" => {
                        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                            let changed = self.prefs.locale_id != value;
                            self.prefs.locale_id = value.to_string();
                            semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::install_ui_engine_locale(self.prefs.active_locale());
                            if let Some(status) = self.prefs.plugin_fault_status(&self.plugins) {
                                self.chrome.error = Some(status);
                            }
                            self.docs.trace_resolved_locale("setLocale", &self.chrome, &self.prefs);
                            self.note_shell_setting_command("os.setLocale", Some(value)).await?;
                            if changed {
                                self.as_session().sync_dock_tabs();
                                self.settle.owe_refresh(UiDirtyScope::Full);
                                self.settle.owe_settle();
                            }
                        }
                        return Ok(());
                    }
                    "setTerminology" => {
                        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                            let changed = self.prefs.terminology_id != value;
                            self.prefs.terminology_id = value.to_string();
                            self.note_shell_setting_command("os.setTerminology", Some(value)).await?;
                            if changed {
                                self.as_session().sync_dock_tabs();
                                self.settle.owe_refresh(UiDirtyScope::Full);
                                self.settle.owe_settle();
                            }
                        }
                        return Ok(());
                    }
                    "setThemeId" => {
                        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                            let changed = self.chrome.chrome_build.preferences.theme_id != value || self.prefs.theme_draft.is_some();
                            set_active_theme_id(value);
                            self.prefs.clear_theme_draft();
                            self.chrome.chrome_build.preferences.theme_id = value.to_string();
                            self.note_shell_setting_command("os.setThemeId", Some(value)).await?;
                            if changed {
                                self.as_session().republish_shell_panel_document(FRAMEWORK_SETTINGS_THEME_TAB_ID)?;
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
                            self.chrome.chrome_build.preferences.ui_layout = if value == "tablet" { "tablet".to_string() } else { "desktop".to_string() };
                            self.note_shell_setting_command("os.setLayout", Some(value)).await?;
                            self.as_session().republish_shell_panel_document(FRAMEWORK_SETTINGS_GENERAL_TAB_ID)?;
                        }
                        return Ok(());
                    }
                    "setDefaultApp" => {
                        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                            let key = action.args.as_ref().and_then(|args| args.get("row")).and_then(|v| v.as_str()).unwrap_or_default().to_string();
                            if value == DEFAULT_APP_NONE_VALUE {
                                self.plugins.default_app_pins.remove(&key);
                            } else {
                                self.plugins.default_app_pins.insert(key, value.to_string());
                            }
                        }
                        return Ok(());
                    }
                    "captureKeybinding" => {
                        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                            let already = self.input.keybinding_capture_control_id.as_deref() == Some(value);
                            self.input.keybinding_capture_control_id = (!already).then(|| value.to_string());
                        }
                        return Ok(());
                    }
                    "resetKeybinding" => {
                        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                            self.chrome.chrome_build.preferences.keybinding_overrides.remove(value);
                        }
                        return Ok(());
                    }
                    "setCommandExpanded" => {
                        if let Some(key) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|value| value.as_str()) {
                            let live = self.docs.resolved_commands(&self.chrome, &self.plugins).into_iter().any(|entry| entry.definition.in_palette && !entry.definition.args.is_empty() && command_address_stable_key(&entry.address) == key);
                            if live {
                                self.commands.expanded_command_id = (self.commands.expanded_command_id.as_deref() != Some(key)).then(|| key.to_string());
                            }
                        }
                        return Ok(());
                    }
                    "executeResolvedCommand" => {
                        if let Some(key) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|value| value.as_str()).map(ToOwned::to_owned) {
                            if let Some(entry) = self.docs.resolved_commands(&self.chrome, &self.plugins).into_iter().find(|entry| entry.definition.in_palette && entry.definition.args.is_empty() && command_address_stable_key(&entry.address) == key) {
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
                            let staged = self.commands.staged_command_args.entry(command.to_string()).or_default();
                            let value = staged_arg_value(&args, staged.get(arg));
                            staged.insert(arg.to_string(), value);
                        }
                        return Ok(());
                    }
                    "resetStagedCommand" => {
                        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                            self.commands.staged_command_args.remove(value);
                        }
                        return Ok(());
                    }
                    "executeStagedCommand" => {
                        if let Some(key) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()).map(ToOwned::to_owned) {
                            let staged = self.commands.staged_command_args.get(&key).cloned().unwrap_or_default();
                            if let Some(entry) = self.docs.resolved_commands(&self.chrome, &self.plugins).into_iter().find(|entry| entry.definition.in_palette && command_address_stable_key(&entry.address) == key) {
                                let Some(effective) = resolved_execute_args(&entry.definition.args, &staged) else { return Ok(()) };
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
                            self.commands.staged_command_args.remove(&key);
                            self.commands.expanded_command_id = None;
                        }
                        return Ok(());
                    }
                    "setActionExpanded" => {
                        let args = action.args.as_ref().map(dsl_value_as_json).unwrap_or(Value::Null);
                        if let (Some(window_id), Some(action_id)) = (args.get("window").and_then(Value::as_str), args.get("action").and_then(Value::as_str)) {
                            if self.windows.action_panel_expanded.get(window_id).map(String::as_str) == Some(action_id) {
                                self.windows.action_panel_expanded.remove(window_id);
                            } else {
                                self.windows.action_panel_expanded.insert(window_id.to_string(), action_id.to_string());
                            }
                        }
                        return self.as_window_actions().republish_window_action_panes();
                    }
                    IMPORT_TRANSFER_CANCEL_ACTION => {
                        if let Some(transfer) = action.args.as_ref().map(dsl_value_as_json).and_then(|args| args.get("transfer").and_then(Value::as_u64)) {
                            self.as_session().cancel_import_transfer(transfer);
                        }
                        return Ok(());
                    }
                    "stageActionArg" => {
                        let args = action.args.as_ref().map(dsl_value_as_json).unwrap_or(Value::Null);
                        if let (Some(window_id), Some(action_id), Some(arg_id)) = (args.get("window").and_then(Value::as_str), args.get("action").and_then(Value::as_str), args.get("arg").and_then(Value::as_str)) {
                            let (window_id, action_id, arg_id) = (window_id.to_string(), action_id.to_string(), arg_id.to_string());
                            let value = staged_arg_value(&args, self.commands.staged_map_for(&window_id, &action_id).get(&arg_id));
                            self.commands.stage_arg(&window_id, &action_id, &arg_id, value);
                        }
                        return self.as_window_actions().republish_window_action_panes();
                    }
                    "resetActionArgs" => {
                        let args = action.args.as_ref().map(dsl_value_as_json).unwrap_or(Value::Null);
                        if let (Some(window_id), Some(action_id)) = (args.get("window").and_then(Value::as_str), args.get("action").and_then(Value::as_str)) {
                            let (window_id, action_id) = (window_id.to_string(), action_id.to_string());
                            self.commands.reset_staged_args(&window_id, &action_id);
                        }
                        return self.as_window_actions().republish_window_action_panes();
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
                            self.windows.engagement_inputs.insert(window_id.to_string(), value);
                        }
                        return Ok(());
                    }
                    "toggleSearchPossibles" => {
                        let args = action.args.as_ref().map(dsl_value_as_json).unwrap_or(Value::Null);
                        if let Some(window_id) = args.get("window").and_then(Value::as_str) {
                            let open = self.commands.search_possibles_open.get(window_id).copied().unwrap_or(false);
                            self.commands.search_possibles_open.insert(window_id.to_string(), !open);
                        }
                        return self.as_window_actions().republish_window_action_panes();
                    }
                    "setWorldProjectionTemplate" => {
                        let args = action.args.as_ref().map(dsl_value_as_json).unwrap_or(Value::Null);
                        if let (Some(window_id), Some(template_id)) = (args.get("windowId").and_then(Value::as_str), args.get("templateId").and_then(Value::as_str)) {
                            let (window_id, template_id) = (window_id.to_string(), template_id.to_string());
                            self.dock.select_world_projection_template(&window_id, &template_id, &mut self.surfaces, &mut self.docs, &mut self.windows)?;
                        }
                        return Ok(());
                    }
                    "setLayoutSaveLabel" => {
                        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                            self.dock.layout_save_label = value.to_string();
                        }
                        return Ok(());
                    }
                    "saveCurrentLayout" => {
                        let label = self.dock.layout_save_label.trim().to_string();
                        if label.is_empty() {
                            return Ok(());
                        }
                        if let Some(layout) = self.dock.current_named_layout(&label, &self.docs) {
                            let id = layout.id.clone();
                            self.dock.user_layouts.retain(|entry| entry.id != id);
                            self.dock.user_layouts.push(layout);
                            self.dock.layout_save_label.clear();
                            self.dock.persist_named_layouts(&self.docs);
                            self.note_shell_setting_command("os.saveNamedLayout", Some(&id)).await?;
                        }
                        return Ok(());
                    }
                    "applyNamedLayout" => {
                        if let Some(layout_id) = action.args.as_ref().and_then(|args| args.get("layoutId")).and_then(|v| v.as_str()).map(ToOwned::to_owned) {
                            if self.as_session().apply_named_layout(&layout_id) {
                                self.note_shell_setting_command("os.applyNamedLayout", Some(&layout_id)).await?;
                            }
                        }
                        return Ok(());
                    }
                    "deleteUserLayout" => {
                        if let Some(layout_id) = action.args.as_ref().and_then(|args| args.get("layoutId")).and_then(|v| v.as_str()).map(ToOwned::to_owned) {
                            self.dock.user_layouts.retain(|entry| entry.id != layout_id);
                            self.dock.persist_named_layouts(&self.docs);
                            self.note_shell_setting_command("os.deleteNamedLayout", Some(&layout_id)).await?;
                        }
                        return Ok(());
                    }
                    "openDisplayWindow" => {
                        let window_kind_id = action.args.as_ref().and_then(|args| args.get("windowKindId")).and_then(|v| v.as_str()).map(ToOwned::to_owned);
                        let template_id = action.args.as_ref().and_then(|args| args.get("templateId")).and_then(|v| v.as_str()).map(ToOwned::to_owned);
                        if let Some(window_kind_id) = window_kind_id {
                            self.dock.open_display_window(&window_kind_id, template_id.as_deref(), &mut self.settle, &self.docs, &mut self.windows);
                        }
                        return Ok(());
                    }
                    "selectConflict" => {
                        if let Some(conflict_id) = action.args.as_ref().and_then(|args| args.get("conflictId")).and_then(|v| v.as_str()) {
                            self.prefs.selected_conflict_id = (self.prefs.selected_conflict_id.as_deref() != Some(conflict_id)).then(|| conflict_id.to_string());
                        }
                        return Ok(());
                    }
                    "resolveConflict" => {
                        let conflict_id = action.args.as_ref().and_then(|args| args.get("conflictId")).and_then(|v| v.as_str()).map(ToOwned::to_owned);
                        let accept = action.args.as_ref().and_then(|args| args.get("accept")).and_then(semio_framework_value::DslValue::as_bool).unwrap_or(true);
                        if let Some(conflict_id) = conflict_id {
                            self.docs.resolve_open_conflict(&conflict_id, accept, &mut self.chrome, &self.plugins, &mut self.prefs).await;
                        }
                        return Ok(());
                    }
                    "installPlugin" | "reloadPlugin" => {
                        if let Some(plugin_id) = action.args.as_ref().and_then(|args| args.get("pluginId")).and_then(|v| v.as_str()).map(ToOwned::to_owned) {
                            if action.action == "reloadPlugin" {
                                self.docs.uninstall_plugin(&plugin_id, &mut self.plugins, &mut self.directory);
                            }
                            let _ = self.plugins.install_plugin(&plugin_id).await;
                            self.note_shell_setting_command(if action.action == "reloadPlugin" { "os.reloadPlugin" } else { "os.installPlugin" }, Some(&plugin_id)).await?;
                        }
                        return Ok(());
                    }
                    "uninstallPlugin" => {
                        if let Some(plugin_id) = action.args.as_ref().and_then(|args| args.get("pluginId")).and_then(|v| v.as_str()).map(ToOwned::to_owned) {
                            if self.docs.uninstall_plugin(&plugin_id, &mut self.plugins, &mut self.directory) {
                                self.note_shell_setting_command("os.uninstallPlugin", Some(&plugin_id)).await?;
                            }
                        }
                        return Ok(());
                    }
                    "installExtensionUrl" => {
                        let url = action.args.as_ref().and_then(|args| args.get("url")).and_then(|value| value.as_str()).filter(|url| !url.is_empty()).map(ToOwned::to_owned);
                        let mut request = serde_json::json!({
                            "op": "extension-store-install-url",
                            "prompt": shell_chrome_string("plugins.extension.urlPrompt", self.prefs.locale_id == "de")
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
                            self.directory.agent_chat_draft = value.to_string();
                        }
                        return Ok(());
                    }
                    "sendChatDraft" => {
                        self.directory.send_agent_chat_draft(&mut self.chrome);
                        return Ok(());
                    }
                    "cancelToolCall" => {
                        if let Some(invocation_id) = action.args.as_ref().and_then(|args| args.get("invocationId")).and_then(|value| value.as_str()).map(ToOwned::to_owned) {
                            self.chrome.cancel_agent_tool_call(&invocation_id);
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
                                self.prefs.merge_policy = value.to_string();
                                self.note_shell_setting_command("os.setMergePolicy", Some(value)).await?;
                                self.as_session().republish_shell_panel_document(FRAMEWORK_SETTINGS_GENERAL_TAB_ID)?;
                            }
                        }
                        return Ok(());
                    }
                    "setThemeColor" | "setThemeSpacing" | "setThemeFontStack" | "setThemeStroke" | "setThemeRadius" | "setThemeOpacity" | "setThemeMetric" | "setThemeAppearancePaint" => {
                        let args = action.args.as_ref().map(dsl_value_as_json).unwrap_or(Value::Null);
                        let verb = action.action.clone();
                        let before = self.prefs.theme_document(&self.chrome);
                        self.prefs.apply_theme_editor_commit(&verb, &args, &self.chrome);
                        if self.prefs.theme_document(&self.chrome) != before {
                            self.as_session().republish_shell_panel_document(FRAMEWORK_SETTINGS_THEME_TAB_ID)?;
                        }
                        return Ok(());
                    }
                    "setThemeSaveLabel" => {
                        let value = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()).unwrap_or_default().to_string();
                        if self.prefs.theme_save_label != value {
                            self.prefs.theme_save_label = value;
                            self.as_session().republish_shell_panel_document(FRAMEWORK_SETTINGS_THEME_TAB_ID)?;
                        }
                        return Ok(());
                    }
                    "saveTheme" => {
                        let label = self.prefs.theme_save_label.trim().to_string();
                        if let Some(id) = custom_theme_id_for_label(&label) {
                            let mut document = self.prefs.theme_document(&self.chrome);
                            document.id = id.clone();
                            document.label = label;
                            if let Some(raw) = save_custom_theme_document(&id, &document) {
                                self.chrome.chrome_build.preferences.custom_themes.insert(id.clone(), raw);
                                self.chrome.chrome_build.preferences.theme_id = id.clone();
                                self.prefs.theme_draft = None;
                                self.prefs.theme_save_label.clear();
                                self.note_shell_setting_command("os.setThemeId", Some(id.as_str())).await?;
                                self.as_session().republish_shell_panel_document(FRAMEWORK_SETTINGS_THEME_TAB_ID)?;
                            }
                        }
                        return Ok(());
                    }
                    "exportTheme" => {
                        let document = self.prefs.theme_document(&self.chrome);
                        if let Some(text) = document.canonical_json() {
                            download_media_export(&format!("{}.theme.dsl", document.id), "text/plain", &text, None);
                        }
                        return Ok(());
                    }
                    "importTheme" => {
                        #[cfg(target_arch = "wasm32")]
                        {
                            self.prefs.pending_theme_import = true;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        self.settle.submit_shell_io_future(async move {
                            let opened = request_file_open(SHELL_THEME_IMPORT_ACCEPT, Some("text"), false).await;
                            ShellIoCompletion::Actions(
                                opened.into_iter().take(1).map(|file| ActionDescriptor { controller_id: "framework".into(), action: "applyImportedTheme".into(), args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "value": file.contents }) }).collect(),
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
                                        self.chrome.chrome_build.preferences.custom_themes.insert(id.clone(), raw);
                                        self.chrome.chrome_build.preferences.theme_id = id.clone();
                                        self.prefs.theme_draft = None;
                                        self.note_shell_setting_command("os.setThemeId", Some(id.as_str())).await?;
                                        self.as_session().republish_shell_panel_document(FRAMEWORK_SETTINGS_THEME_TAB_ID)?;
                                    }
                                }
                            }
                        }
                        return Ok(());
                    }
                    "resetThemeId" => {
                        let changed = self.chrome.chrome_build.preferences.theme_id != "semio" || self.prefs.theme_draft.is_some();
                        set_active_theme_id("semio");
                        self.prefs.clear_theme_draft();
                        self.chrome.chrome_build.preferences.theme_id = "semio".to_string();
                        self.note_shell_setting_command("os.resetThemeId", Some("semio")).await?;
                        if changed {
                            self.as_session().republish_shell_panel_document(FRAMEWORK_SETTINGS_THEME_TAB_ID)?;
                        }
                        return Ok(());
                    }
                    "deleteThemeId" => {
                        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                            delete_custom_theme(value);
                            let removed = self.chrome.chrome_build.preferences.custom_themes.remove(value).is_some();
                            let selected = self.chrome.chrome_build.preferences.theme_id == value;
                            if selected {
                                self.chrome.chrome_build.preferences.theme_id = "semio".to_string();
                            }
                            self.note_shell_setting_command("os.deleteThemeId", Some(value)).await?;
                            if removed || selected {
                                self.as_session().republish_shell_panel_document(FRAMEWORK_SETTINGS_THEME_TAB_ID)?;
                            }
                        }
                        return Ok(());
                    }
                    _ => {}
                }
            }
            if action.controller_id == "framework.sync" {
                self.handle_sync_action(action).await?;
                return self.as_session().republish_shell_panel_document(FRAMEWORK_SYNC_PANEL_TAB_ID);
            }
            if action.controller_id == "framework.checkin" {
                return self.handle_checkin_action(action).await;
            }
            if action.action == semio_framework::SET_ACTIVE_UTILITY_ACTION_ID {
                if let Some(session) = self.docs.session.clone() {
                    if action.controller_id == session.app.controller_id {
                        if let Some(utility_id) = action.args.as_ref().and_then(|args| args.get("utilityId")).and_then(|value| value.as_str()) {
                            let window_id = action
                                .args
                                .as_ref()
                                .and_then(|args| args.get("windowId"))
                                .and_then(|value| value.as_str())
                                .map(String::from)
                                .or_else(|| self.windows.active_window_id.clone())
                                .unwrap_or_else(|| self.dock.active_utility_bar_window_kind(&session, &self.windows).id.clone());
                            self.windows.toggle_active_utility(&window_id, utility_id, &mut self.chrome, &mut self.docs);
                        }
                    }
                }
            }
            let Some(session) = self.docs.action_session(&action, &self.plugins).cloned() else {
                return Err("action has no mounted app owner".into());
            };
            let program = self.plugins.plugins.iter().find(|p| p.plugin_id == session.plugin_id).ok_or("action program missing")?;
            let live_view_state = self.as_session().live_view_state(&session);
            let invocation = self.prefs.session_action_invocation(&session, &action, &live_view_state)?;
            let action_json = semio_framework_pack_json::to_json_string(&invocation);
            let answer = program.handle_action(session.instance_id, &action_json, &live_view_state).await;
            let mut result = answer.map_err(|refusal| self.prefs.refused_guest_call(refusal, &session.app, &mut self.chrome))?;
            self.observe_invocation_history(result.history_patch.as_ref()).await;
            self.input.observe_interaction_output(&result.output);
            if record_tutorial_after_acceptance {
                self.as_window_actions().tutorial_start_recording();
            }
            self.docs.chrome_tour_note_action_performed(&action.action, &mut self.chrome);
            let mut queued = Vec::with_capacity(result.requested_effects.len());
            for effect in core::mem::take(&mut result.requested_effects) {
                match effect {
                    semio_framework::kernel::Effect::Navigate { uri } => {
                        self.opening.push_uri(uri.clone());
                        if let Err(error) = self.apply_shell_uri(&uri).await {
                            debug_log(&format!("[TRACE] wgpu shell navigate effect failed: {error}"));
                        }
                    }
                    semio_framework::kernel::Effect::ReplayShellCommand { action_id, args } => {
                        self.handle_replay_shell_command(&action_id, args.as_ref()).await;
                    }
                    other => queued.push(other),
                }
            }
            let scope = result.ui_scope.clone().merged_with(self.docs.host_effect_earned_scope(&queued));
            debug_log(&format!("[TRACE] wgpu-shell dispatch action={} scope={}", action.action, refresh_scope_label(&scope)));
            self.as_window_actions().queue_host_effects(&action.controller_id.clone(), queued);
            let operations: Vec<String> = result.mutations.iter().filter_map(|operation| serde_json::to_string(&operation.diff.payload).ok()).collect();
            self.apply_ops_inner(&operations, true, scope, Some(session)).await

        })

    }
}

//#endregion ShellActions

