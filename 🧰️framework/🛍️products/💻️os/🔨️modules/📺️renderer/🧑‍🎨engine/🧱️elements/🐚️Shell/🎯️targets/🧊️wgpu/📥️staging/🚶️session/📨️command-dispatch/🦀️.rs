//! 🚶️ `shell-session` staging of `command-dispatch`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellActions
impl<'ctx> SessionCtx<'ctx> {
    //#region 🔖️DirectoryAndIdentity
    /// 🛟️ Drives a rebootstrapping hub document back to Live — the same canonical-pair seed as its first open
    /// ([`seed_hub_document`]): the pair is fetched detached and admitted only as the actor's control's checkpoint, loaded into
    /// the guest, and handed to the actor ([`ArtifactActorMsg::Reseed`]), which then dials at once. A transient failure is asked
    /// again after [`SYNC_RESEED_RETRY_MS`] while the document stays open; closing the document drops the reseed. Answers
    /// whether the guest's document changed.
    pub async fn advance_sync_reseed(&mut self) -> bool {
        let Some(mut reseed) = self.sync.sync_reseed.take() else { return false };
        let Some(owner) = self.sync.sync_channel.as_ref().map(shell_sync_channel_owner) else { return false };
        match reseed.pending.as_ref().and_then(ShellDetached::take) {
            None if reseed.pending.is_some() => {
                self.sync.sync_reseed = Some(reseed);
                false
            }
            None => {
                if chrome_now_ms() < reseed.retry_at_ms {
                    self.sync.sync_reseed = Some(reseed);
                    return false;
                }
                let Some(client) = self.directory.directory_client.clone() else {
                    reseed.retry_at_ms = chrome_now_ms() + SYNC_RESEED_RETRY_MS;
                    self.sync.sync_reseed = Some(reseed);
                    return false;
                };
                let (ctx, control) = (self.directory.directory_ctx(), reseed.control.clone());
                reseed.pending = Some(ShellDetached::spawn(async move { client.rebootstrap_canonical_checkpoint_pair(&ctx, &control).await.map_err(|error| format!("canonical checkpoint pair: {error}")) }));
                self.sync.sync_reseed = Some(reseed);
                false
            }
            Some(Err(error)) => {
                debug_log(&format!("[TRACE] wgpu shell rebootstrap reseed refused: {error}"));
                reseed.pending = None;
                reseed.retry_at_ms = chrome_now_ms() + SYNC_RESEED_RETRY_MS;
                self.sync.sync_reseed = Some(reseed);
                false
            }
            Some(Ok(pair)) => {
                let Some(plugin) = self.plugins.plugins.iter().find(|entry| entry.plugin_id == owner.plugin_id).cloned() else { return false };
                if let Err(error) = plugin.load_app_document_pack(owner.instance_id, &pair.pack_bytes, &pair.spr_bytes).await {
                    let detail = self.prefs.note_refused_open(error, &mut self.chrome);
                    debug_log(&format!("[TRACE] wgpu shell rebootstrap reseed load failed: {detail}"));
                    reseed.pending = None;
                    reseed.retry_at_ms = chrome_now_ms() + SYNC_RESEED_RETRY_MS;
                    self.sync.sync_reseed = Some(reseed);
                    return false;
                }
                let message = ArtifactActorMsg::Reseed { pack: pair.pack_bytes, spr: pair.spr_bytes, baseline: pair.baseline_frontier };
                if let Some(channel) = self.sync.sync_channel.as_ref() {
                    let _ = channel.cmd_tx.send(message);
                }
                self.settle.owe_refresh(UiDirtyScope::Full);
                true
            }
        }
    }

    /// 📔️ Live directory WS events reach the mounted Space index through `foldDirectoryEvents`
    /// (browser parity), so its membership stays projected. Home is never folded: its projection's
    /// only writer is the sealed page lane, which the same events wake through `home.wake` below.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn dispatch_fold_directory_events(&mut self, events: &[semio_framework_os_kernel::os_directory::DirectoryEvent]) {
        let Some(session) = self.docs.session.clone().filter(|session| session_is_space_index(session, &self.plugins.plugins)) else { return };
        let Some(program) = self.plugins.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).cloned() else { return };
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
        self.settle.submit_shell_io_future(async move {
            ShellIoCompletion::Program(program.handle_action(instance_id, &action_json, &live_view_state).await.map(|_| ()))
        });
    }

    //#endregion 🔖️DirectoryAndIdentity
    pub async fn spawn_plugin(&mut self, plugin_id: &str, mut view_state: ViewModel) -> Result<(), String> {
        let workflows = self.prefs.build_space_workflows(&self.plugins);
        let Some(workflow) = workflows.iter().find(|entry| entry.plugin_id == plugin_id).cloned() else {
            return Ok(());
        };
        let (terminology, locale) = (self.prefs.active_terminology(), self.prefs.active_locale());
        let bridge = self.plugins.plugins.iter().find(|entry| entry.plugin_id == workflow.plugin_id).ok_or("spawn program missing")?;
        let instance_id = bridge.create_app(&workflow.app_id).await.map_err(|refusal| program_fault_text(&refusal, terminology, locale))?;
        let mut panel = panel_state_from_view(&view_state)?.unwrap_or(self.default_host_panel_state()?);
        let spawned_id = format!("{}-{}", bridge.plugin_id, instance_id);
        panel.spawned_apps.push(SpawnedAppEntry { id: spawned_id.clone(), plugin_id: bridge.plugin_id.clone(), instance_id, app_id: workflow.app_id.clone(), label: workflow.label.clone(), breadcrumb: workflow.breadcrumb.clone() });
        panel.active_spawned_id = Some(spawned_id);
        view_state.panel_json = Some(panel_json(&panel)?);
        if let Some(session) = self.docs.session.as_mut() {
            session.view_state = view_state;
        }
        Ok(())
    }
}

//#endregion ShellActions

