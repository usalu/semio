//! 🔄️ Backbone sync and check in: the shell turn alias and constructor, the native backbone sync, the invocation history, the auto check in, the checkpoints and the space index artifact. See [`super`].

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
}
//#endregion ShellActions
