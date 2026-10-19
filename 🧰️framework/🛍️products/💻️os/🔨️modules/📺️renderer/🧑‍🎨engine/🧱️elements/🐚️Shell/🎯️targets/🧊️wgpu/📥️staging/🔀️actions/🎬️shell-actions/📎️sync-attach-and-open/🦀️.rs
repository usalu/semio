//! 📎️ Sync attach and document open: the sync backbone attach, the document open preparation and finish and the sync card actions. See [`super`].

use super::*;

//#region ShellActions
impl<'ctx> ActionsCtx<'ctx> {
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

}
//#endregion ShellActions
