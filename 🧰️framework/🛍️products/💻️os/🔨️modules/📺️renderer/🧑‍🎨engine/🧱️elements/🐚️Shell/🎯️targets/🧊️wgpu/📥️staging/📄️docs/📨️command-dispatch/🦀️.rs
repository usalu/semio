//! 📄️ `shell-docs` staging of `command-dispatch`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellActions
impl DocumentState {
    //#region 🔖️DirectoryAndIdentity
    /// 🧾️ Invokes the stored Home instance and accepts only its terminal Config publication receipt.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn start_directory_home_publication(&mut self, page: CanonicalDirectoryEventPageV1, plugins: &PluginState, directory: &mut DirectoryState) -> Result<(), String> {
        let canonical_json = page.canonical_json().to_string();
        let session_identity = directory.identity.as_ref().map(|identity| ViewSessionIdentity { user_id: identity.user_id.clone(), display_name: identity.display_name.clone() });
        let (epoch, instance_id, program, app, view_state, expected) = {
            let home = directory.directory_home.as_mut().ok_or("retained Home projection is unavailable")?;
            let expected = home.bootstrap.present(page).map_err(|error| error.to_string())?;
            let mut view_state = self.session.as_ref().filter(|session| home.is_instance(&session.plugin_id, session.instance_id)).map(|session| session.view_state.clone()).unwrap_or_else(|| home.view_state.clone());
            view_state.session_identity = session_identity;
            let program = plugins.plugins.iter().find(|entry| entry.plugin_id == home.plugin_id).cloned().ok_or("retained Home program is unavailable")?;
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
        let pool = semio_framework_os_renderer_wgpu_host_base::renderer_worker_pool();
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
        let home = directory.directory_home.as_mut().ok_or("retained Home projection retired before publication admission")?;
        if home.bootstrap.bootstrap_epoch() != epoch || home.instance_id != instance_id || home.closed {
            task.cancel();
            return Err("retained Home projection changed before publication admission".into());
        }
        home.home_rx = Some(rx);
        home.home_task = Some(task);
        Ok(())
    }
    //#endregion 🔖️DirectoryAndIdentity
}

//#endregion ShellActions

