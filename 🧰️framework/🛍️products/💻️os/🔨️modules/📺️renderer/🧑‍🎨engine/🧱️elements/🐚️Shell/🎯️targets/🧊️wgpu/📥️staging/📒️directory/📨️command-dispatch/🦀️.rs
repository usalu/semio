//! 📒️ `shell-directory` staging of `command-dispatch`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellActions
impl DirectoryState {
    //#region 🔖️DirectoryAndIdentity
    //#region 📇️DirectoryLane
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
        context.deadline_ms = Some(directory_now_ms().saturating_add(DIRECTORY_COMMAND_DEADLINE_MS));
        context
    }

    /// 📇️ ticket §4/§C6 — seals one `os.directory.*` command under a fresh idempotency correlation
    /// and issues it against the hub, dropping it when no identity is signed in at all, else leaving
    /// it queued on a transient failure (the "same offline queueing behaviour as the React side" the
    /// brief asks for — React's worker queues in-memory and flushes on reconnect; this shell's
    /// `directory_commands` + `flush_pending_directory_commands` (called every frame from
    /// `pump_directory_events`) is the native twin of that policy).
    pub(super) async fn dispatch_directory_command(&mut self, command: DirectoryCommand) {
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
    pub(super) async fn flush_pending_directory_commands(&mut self) {
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
    pub(super) async fn poll_browser_identity(&mut self) -> bool {
        if self.identity.is_some() || self.identity_env.is_none() {
            return false;
        }
        let now_ms = directory_now_ms();
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
                debug_log(&format!("[TRACE] wgpu shell browser identity bootstrap failed: {error}"));
                false
            }
        }
    }

    //#endregion 📇️DirectoryLane
    //#region 🔐️HubWorkspaceLane
    /// 🌱️ Opens the creation door for `space_id`: a fresh door holding a minted idempotency key, then
    /// the space's selected current catalog. A hub with no ready catalog leaves the door
    /// `Unavailable`, saying so in both languages; local work never waits on it.
    pub(super) async fn open_hub_artifact_creation(&mut self, space_id: &str) {
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
    pub(super) fn begin_hub_artifact_creation(&mut self) {
        let Some(space_id) = self.hub_workspace.open_space_id.clone() else { return };
        let Some(intent) = hub_artifact_creation_intent(&self.hub_workspace) else { return };
        let now_ms = directory_now_ms();
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

    pub(super) fn persist_hub_connection_book(&self) {
        prefs_set(HUB_CONNECTION_BOOK_STORAGE_KEY_V1, &serialize_hub_connection_book(&self.hub_workspace.book));
    }

    pub(super) fn clear_hub_session_owner(&mut self) {
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

    pub(super) fn project_verified_hub_authority(&mut self) {
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
    pub(super) fn start_hub_sign_in(&mut self) {
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
        ctx.deadline_ms = Some(directory_now_ms().saturating_add(HUB_SIGN_IN_DEADLINE_MS));
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

    /// 🚪️ Sign-out. The in-memory capability is dropped whatever the hub answers, so an unreachable
    /// hub can never trap a human in a session they asked to leave.
    pub(super) async fn run_hub_sign_out_turn(&mut self) {
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
    pub(super) async fn run_hub_create_invite_turn(&mut self, space_id: &str) {
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
    pub(super) async fn run_hub_redeem_turn(&mut self) {
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
    pub(super) fn start_hub_spaces_reload(&mut self) {
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
    pub(super) fn hub_online_user_ids(&self) -> Vec<String> {
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
    /// 🔄️ One bounded drain per frame; the runner never spins and never blocks the render loop.
    #[cfg(not(target_arch = "wasm32"))]
    pub(super) fn pump_inference_port(&mut self) -> bool {
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
    /// 🏠️ Begins one bounded canonical-page fetch owned by the retained Home bootstrap epoch.
    #[cfg(not(target_arch = "wasm32"))]
    pub(super) fn start_directory_event_page_fetch(&mut self) {
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

    #[cfg(not(target_arch = "wasm32"))]
    pub(super) fn poll_directory_page_result(&mut self) -> Option<ShellDirectoryPageResult> {
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
    pub(super) fn poll_directory_home_publication(&mut self) -> Option<ShellDirectoryHomePublicationResult> {
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
    pub(super) fn open_acknowledged_directory_stream(&mut self, since: u64) -> Result<(), String> {
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
    //#endregion 🔖️DirectoryAndIdentity
}

//#endregion ShellActions

