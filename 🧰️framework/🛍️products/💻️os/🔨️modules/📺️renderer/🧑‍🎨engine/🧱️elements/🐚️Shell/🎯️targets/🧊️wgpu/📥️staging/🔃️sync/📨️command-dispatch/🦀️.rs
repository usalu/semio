//! 🔃️ `shell-sync` staging of `command-dispatch`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellActions
impl SyncState {
    //#region 🔖️DirectoryAndIdentity
    //#region 🔐️HubWorkspaceLane
    /// 🔐️ Applies one sign-in answer: a verified session owns the directory client, the socket grants and the book's
    /// last user, then the spaces read starts on its own task; a session verified for a hub no longer selected is
    /// dropped; anything else is the session's localized error.
    pub fn apply_hub_sign_in(&mut self, answer: ShellHubSignInAnswer, directory: &mut DirectoryState) {
        match answer {
            ShellHubSignInAnswer::Failed { code, retry_after_seconds } => {
                directory.hub_workspace.session = reduce_hub_session(&directory.hub_workspace.session, &HubSessionEvent::Failed { code, retry_after_seconds });
            }
            ShellHubSignInAnswer::Verified { origin, user_id, credential, client, authority } if origin == directory.hub_workspace.origin() => {
                directory.identity = Some(Identity { user_id: authority.user_id.clone(), email: authority.email.clone(), display_name: authority.display_name.clone(), hub_base_url: origin, issued_at_ms: chrome_now_ms() as i64 });
                directory.verified_session_authority = Some(authority);
                directory.directory_client = Some(client.clone());
                self.document_host.set_local_hub_credential(credential);
                self.document_host.set_hub_socket_grant_source(client);
                let selected_id = directory.hub_workspace.book.selected_id.clone();
                if let Some(connection) = directory.hub_workspace.book.connections.iter_mut().find(|connection| connection.id == selected_id) {
                    connection.last_user_id = Some(user_id);
                }
                directory.persist_hub_connection_book();
                directory.project_verified_hub_authority();
                directory.start_hub_spaces_reload();
            }
            ShellHubSignInAnswer::Verified { .. } => {}
            ShellHubSignInAnswer::Unverified => {
                directory.clear_hub_session_owner();
                directory.hub_workspace.session = reduce_hub_session(&directory.hub_workspace.session, &HubSessionEvent::Failed { code: HubSignInErrorCode::InvalidResponse, retry_after_seconds: None });
            }
        }
    }

    /// 📬️ Reads the answers of the hub workspace's spawned requests, once each; answers whether the workspace changed.
    pub fn poll_hub_workspace_tasks(&mut self, directory: &mut DirectoryState) -> bool {
        let mut changed = false;
        if let Some(answer) = directory.hub_sign_in_task.as_ref().and_then(ShellHubTask::answer) {
            directory.hub_sign_in_task = None;
            self.apply_hub_sign_in(answer.unwrap_or(ShellHubSignInAnswer::Failed { code: HubSignInErrorCode::Unreachable, retry_after_seconds: None }), directory);
            changed = true;
        }
        if let Some(answer) = directory.hub_spaces_task.as_ref().and_then(ShellHubTask::answer) {
            directory.hub_spaces_task = None;
            match answer.and_then(|rows| rows) {
                Ok(rows) => {
                    directory.hub_workspace.rows = rows;
                    directory.hub_workspace.phase = semio_framework_os_renderer_wgpu_hub_contracts::space_browser::SpaceBrowserPhase::Ready;
                }
                Err(()) => directory.hub_workspace.phase = semio_framework_os_renderer_wgpu_hub_contracts::space_browser::SpaceBrowserPhase::Stale,
            }
            changed = true;
        }
        changed
    }

    //#endregion 🔐️HubWorkspaceLane
    /// 🪪️ Drains `identity_bootstrap_rx` and starts a fresh zero-frontier retained-Home epoch.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn poll_identity_bootstrap(&mut self, chrome: &mut ChromeState, directory: &mut DirectoryState) {
        let Some(rx) = directory.identity_bootstrap_rx.as_ref() else { return };
        match rx.try_recv() {
            Ok(Ok(outcome)) => {
                directory.identity_bootstrap_task.take();
                directory.identity_offline = outcome.status == IdentityStatus::Offline;
                directory.identity = Some(outcome.identity.clone());
                directory.verified_session_authority = Some(outcome.authority);
                let client = std::sync::Arc::new(DirectoryClient::authenticated(directory.directory_transport.clone(), outcome.credential.clone()));
                self.document_host.set_local_hub_credential(outcome.credential);
                self.document_host.set_hub_socket_grant_source(client.clone());
                directory.directory_client = Some(client);
                directory.project_verified_hub_authority();
                if let Some(home) = directory.directory_home.as_mut() {
                    if let Err(error) = home.begin_epoch(0) {
                        home.close();
                        chrome.error = Some(error.to_string());
                    }
                }
                directory.identity_bootstrap_rx = None;
            }
            Ok(Err(error)) => {
                directory.identity_bootstrap_task.take();
                debug_log(&format!("[TRACE] wgpu shell identity bootstrap failed: {error}"));
                directory.identity_bootstrap_rx = None;
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => {}
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                directory.identity_bootstrap_task.take();
                directory.identity_bootstrap_rx = None;
            }
        }
    }
    //#endregion 🔖️DirectoryAndIdentity
}

//#endregion ShellActions

