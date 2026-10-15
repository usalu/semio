//! 🧩️ Shell lifecycle, part one: IO, session, boot and window topology.
//!
//! First half of the shell lifecycle `impl ShellState`: shell IO, session and boot, dock sync, window
//! instance ids, window-topology publication and settle faults.
//!
//! One chunk of the former `shell` monolith (ticket CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT); [`ShellState`] and the
//! shared imports stay in the parent module and are reached through `use super::*;`.

use super::*;

//#region ShellLifecycle
impl ShellState {
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(test)]
    pub(crate) fn submit_shell_io(&mut self, operation: impl FnOnce() -> ShellIoCompletion + Send + 'static) {
        const MAX_PENDING_SHELL_IO: usize = 64;
        if self.shell_io_pending.len() >= MAX_PENDING_SHELL_IO {
            return;
        }
        let (sender, receiver) = std::sync::mpsc::channel();
        crate::renderer_worker_pool().submit(
            Lane::Io,
            Box::new(move || {
                let _ = sender.send(operation());
            }),
        );
        self.shell_io_pending.push_back(PendingShellIo { receiver, task: None });
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn submit_shell_io_future(&mut self, operation: impl std::future::Future<Output = ShellIoCompletion> + Send + 'static) {
        const MAX_PENDING_SHELL_IO: usize = 64;
        if self.shell_io_pending.len() >= MAX_PENDING_SHELL_IO {
            return;
        }
        let (sender, receiver) = std::sync::mpsc::channel();
        let task = ShellPoolFuture::spawn(crate::renderer_worker_pool(), Lane::Io, async move {
            let _ = sender.send(operation.await);
        });
        self.shell_io_pending.push_back(PendingShellIo { receiver, task: Some(task) });
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn poll_shell_io(&mut self) -> bool {
        const MAX_COMPLETIONS_PER_FRAME: usize = 8;
        let mut changed = false;
        for _ in 0..MAX_COMPLETIONS_PER_FRAME {
            let result = match self.shell_io_pending.front() {
                Some(pending) => pending.receiver.try_recv(),
                None => break,
            };
            match result {
                Ok(ShellIoCompletion::Actions(actions)) => {
                    if let Some(pending) = self.shell_io_pending.pop_front() {
                        if let Some(task) = pending.task {
                            task.cancel();
                        }
                    }
                    changed |= !actions.is_empty();
                    self.deferred_actions.extend(actions);
                }
                Ok(ShellIoCompletion::Import(picked)) => {
                    if let Some(pending) = self.shell_io_pending.pop_front() {
                        if let Some(task) = pending.task {
                            task.cancel();
                        }
                    }
                    changed |= self.begin_import_transfer(picked);
                }
                Ok(ShellIoCompletion::MediaFrames { controller_id, frames }) => {
                    if let Some(pending) = self.shell_io_pending.pop_front() {
                        if let Some(task) = pending.task {
                            task.cancel();
                        }
                    }
                    changed |= self.begin_media_frames_transfer(controller_id, frames);
                }
                Ok(ShellIoCompletion::Program(result)) => {
                    if let Err(refusal) = result { self.note_refused_open(refusal); changed = true; }
                    if let Some(pending) = self.shell_io_pending.pop_front() { if let Some(task) = pending.task { task.cancel(); } }
                }
                Ok(ShellIoCompletion::Finished) | Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    if let Some(pending) = self.shell_io_pending.pop_front() {
                        if let Some(task) = pending.task {
                            task.cancel();
                        }
                    }
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => break,
            }
        }
        changed
    }

    //#region 🏷️LabelResolution
    /// 🌐️ Active `Locale` derived from `locale_id`. This engine is not manifest-aware the way the
    /// React renderer is (no per-window locale/terminology context is threaded through render
    /// calls), so every `LocalizedLabel` in this file resolves against this shell-wide value.
    pub fn active_locale(&self) -> Locale {
        Locale::parse(&self.locale_id).expect("shell locale has explicit admitted authority")
    }

    /// 🗣️ Active `Terminology` derived from `terminology_id`. See `active_locale` doc.
    pub fn active_terminology(&self) -> Terminology {
        Terminology::parse(&self.terminology_id).expect("shell terminology has explicit admitted authority")
    }

    pub(crate) fn canonical_tree_open(&self, id: &str, default_open: bool) -> bool {
        self.tree_open_states.get(id).copied().unwrap_or(default_open)
    }

    pub(crate) fn set_canonical_tree_open(&mut self, id: &str, open: bool) {
        self.tree_open_states.insert(id.to_string(), open);
    }

    /// 🖱️ Folds one immediate-mode tree row. The chevron and the row share this, so a row with
    /// children opens from its label.
    pub(crate) fn toggle_immediate_tree_item(&mut self, item_id: &str) {
        let key = format!("tree.{item_id}");
        let collapsed = self.collapsed_sections.get(&key).copied().unwrap_or(false);
        let next_open = !self.canonical_tree_open(item_id, !collapsed);
        self.collapsed_sections.insert(key, !next_open);
        self.set_canonical_tree_open(item_id, next_open);
    }

    pub(crate) fn set_search_open(&mut self, open: bool) {
        self.search_open = open;
        if open {
            self.find_open = false;
            self.overlay_state = OverlayState::Search;
        } else if self.overlay_state == OverlayState::Search {
            self.overlay_state = OverlayState::None;
        }
    }

    pub(crate) fn observe_interaction_output(&mut self, output: &DslValue) {
        let output = dsl_value_as_json(output);
        let Some(view) = output.get("interactionView") else { return };
        if view.get("selectionCleared").and_then(Value::as_bool) == Some(true) {
            self.set_interaction_selection_projection(HashMap::new());
        }
        let Some(selection) = view.get("selection") else { return };
        if let Ok(selection) = serde_json::from_value::<HashMap<String, semio_framework::DomainSelection>>(selection.clone()) {
            self.set_interaction_selection_projection(selection);
        }
    }

    pub(crate) fn set_interaction_selection_projection(&mut self, selection: HashMap<String, semio_framework::DomainSelection>) {
        self.interaction_selection = selection;
    }

    pub(crate) fn chrome_dialog_request(&self, dialog_id: &str, args: Option<DslValue>) -> Option<ChromeDialogRequest> {
        let session = self.session.as_ref()?;
        let dialog = session.app.dialogs.iter().find(|dialog| dialog.id == dialog_id)?;
        Some(ChromeDialogRequest::from_definition(&session.app.controller_id, dialog, args, self.active_terminology(), self.active_locale()))
    }
    //#endregion 🏷️LabelResolution

    /// 🌀️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-directory-and-run, sol-extended lease):
    /// the ONE `TokioHostRuntime`/`ComputePool`/`NativeDirectoryTransport` this shell's directory
    /// client and finite pool turns share — minted ONCE here, never per-call.
    ///
    /// 🧵️ P1e (INTERACTIVE-JOB-RUNTIME-REFACTOR, one-pool-worker-runtime): this shell no longer
    /// sizes or owns its own thread pool — `TokioHostRuntime::new()`'s own `thread_plan`/
    /// `ThreadBudget` construction path was DELETED by P1a with no replacement. Instead this
    /// injects `crate::renderer_worker_pool()`, the ONE process-wide `WorkerPool` this whole
    /// renderer crate shares (also used by `🦀️.rs::kernel_runtime`'s `ParallelRuntime` for
    /// shard-turn scheduling — see that fn's own doc for why it cannot instead reuse
    /// `semio-framework-os-services`'s own private `global_worker_pool()` singleton). `open_scope`/
    /// `ComputePool::new`/`with_new_http_pool` are async constructors bridged once here. Runtime,
    /// scope, and compute ownership then lives in the transport; no directory thread is retained.
    ///
    /// 🌐️ The browser half of the same seam: the page door owns no pool, no scope and no socket —
    /// every hop is one `fetch` the page makes on this shell's behalf — so the transport is a unit
    /// value and only the cancellation root has to be minted.
    pub fn new(plugins: Vec<ProgramBridgeEntry>, plugin_filter: String, locale: Locale, terminology: Terminology) -> Self {
        crate::interpreter::install_ui_engine_locale(locale);
        let host_capability = resolve_plugin_host_config(&plugin_filter);
        let space_mode = host_capability.is_some();
        #[cfg(not(target_arch = "wasm32"))]
        let (directory_transport, directory_cancel): (ShellDirectoryTransport, CancelToken) = {
            const DIRECTORY_COMPUTE_CAPACITY: u32 = 4;
            let pool = crate::renderer_worker_pool();
            let runtime = std::sync::Arc::new(TokioHostRuntime::with_pool(pool.clone()));
            let scope = runtime.open_scope_now(ScopeOwner::Service("directory_client"), None);
            let compute = std::sync::Arc::new(ComputePool::with_pool(DIRECTORY_COMPUTE_CAPACITY, pool));
            let transport = ShellDirectoryTransport::with_new_http_pool_now(runtime, scope, compute, SHELL_DIRECTORY_NETWORK_BYTES_PER_MINUTE, 8, DirectoryPackageId("os.directory-client".to_string()), DirectoryActorId(0));
            (transport, CancelToken::root_now())
        };
        #[cfg(target_arch = "wasm32")]
        let (directory_transport, directory_cancel): (ShellDirectoryTransport, CancelToken) = (shell_directory_transport(), CancelToken::root_now());
        #[cfg(not(target_arch = "wasm32"))]
        let document_host = {
            let host = ArtifactHost::new(std::sync::Arc::new(crate::renderer_worker_pool()));
            if let Some(credential) = claimed_local_hub_credential("native") {
                host.set_local_hub_credential(credential);
            }
            host
        };
        #[cfg(target_arch = "wasm32")]
        let document_host = {
            let host = ArtifactHost::new(std::sync::Arc::new(crate::renderer_worker_pool()));
            host.set_document_socket_dialer(std::sync::Arc::new(crate::socket_door::browser::DoorDocumentSocketDialer));
            host
        };
        let mut state = Self {
            plugins,
            extensions: Vec::new(),
            plugin_filter,
            host_capability,
            space_mode,
            session: None,
            window_ui: HashMap::new(),
            panel_documents: HashMap::new(),
            tool_run_panel_runs: std::collections::BTreeSet::new(),
            spawned_ui: None,
            spawned_session: None,
            app_document_identities: crate::media_slots::DocumentIdentityRegistry::default(),
            closing_documents: ShellDocumentRetirementRegistry::default(),
            active_window_id: None,
            noted_active_window: None,
            host_event_active_window: None,
            panel_anchors: Default::default(),
            dock_tabs: ShellDock::default(),
            dock_override: None,
            dock_collapsed_branches: HashMap::new(),
            dock_tab_drag: None,
            dock_loaded_app_id: None,
            scroll_offsets: HashMap::new(),
            overlay_state: OverlayState::None,
            collapsed_sections: HashMap::new(),
            tree_open_states: BTreeMap::new(),
            accessibility_focused_control_id: None,
            interaction_selection: HashMap::new(),
            open_selects: HashMap::new(),
            context_menu: None,
            search_open: false,
            find_open: false,
            appearance_id: "system".into(),
            locale_id: locale.as_str().into(),
            terminology_id: terminology.as_str().into(),
            right_click: RightClickState::default(),
            uri_history: vec!["/".into()],
            uri_index: 0,
            open_space_id: None,
            pending_shell_uri_apply: false,
            panel_resize_origin_width: DEFAULT_PANEL_WIDTH_PX,
            error: None,
            plugin_faults: Vec::new(),
            surface_faults: Vec::new(),
            screen_w: 1280.0,
            screen_h: 720.0,
            world3d_states: AdmittedSurfaceMap::default(),
            world3d_window_ids: HashMap::new(),
            retired_world3d_states: VecDeque::with_capacity(SCENE_SURFACE_CAPACITY),
            world3d_close_cancels: BTreeMap::new(),
            component_world3d_retirement: None,
            world3d_retirement_epoch: 0,
            world3d_retirement_sequence: 0,
            world3d_retirement_fault: None,
            world3d_status: HashMap::new(),
            world3d_status_pill_trace: HashMap::new(),
            node_graph_states: AdmittedSurfaceMap::default(),
            app_catalogue_json: String::new(),
            app_catalogue_instance: None,
            tiled_map_states: AdmittedSurfaceMap::default(),
            icon_render_states: HashMap::new(),
            board2d_states: AdmittedSurfaceMap::default(),
            dock: DockState::default(),
            search_query: String::new(),
            search_selected: 0,
            find_query: String::new(),
            split_resize_path: None,
            split_resize_index: 0,
            split_resize_axis_total: 1.0,
            active_example_id: None,
            find_items: ShellFindItems::default(),
            find_selected: 0,
            engagement_expanded: HashMap::new(),
            engagement_activated: HashMap::new(),
            measures_folded: HashMap::new(),
            measures_expanded: HashMap::new(),
            measures_width: HashMap::new(),
            measures_resize_origin_width: 0.0,
            engagement_inputs: HashMap::new(),
            driver_id: "default".into(),
            driver_draft: None,
            driver_save_label: String::new(),
            tree_drag: None,
            tree_hovered_id: None,
            widget_maps: WidgetInteractionMaps::default(),
            widget_maps_staging: WidgetInteractionMaps::default(),
            pending_tree_drag: None,
            tree_drag_origin: (0.0, 0.0),
            dock_drag: None,
            pending_dock_drag: None,
            pending_chrome_release: None,
            chrome_pointer_owner: None,
            dock_view: DockState::default(),
            dock_instance_owner: None,
            dock_input_identity: None,
            tree_windows: tree_windows::TreeWindowScheduler::default(),
            tree_window_served: HashMap::new(),
            dock_canvas_bounds: Rect::new(0.0, 0.0, 0.0, 0.0),
            dock_axis_separator: 0.0,
            dock_drop_tab_bars: Vec::new(),
            dock_drop_bodies: Vec::new(),
            dock_window_plan: Vec::new(),
            dock_plan_trace: None,
            layout_override: None,
            split_resize_origin: Vec::new(),
            split_resize_secondary_path: None,
            split_resize_secondary_index: 0,
            split_resize_secondary_axis_total: 1.0,
            split_resize_secondary_origin: Vec::new(),
            measures_resize_window_id: None,
            deferred_actions: Vec::new(),
            window_topology_actions: ui_wgpu::wgpu::BoundedActionQueue::default(),
            window_topology_publications: VecDeque::with_capacity(ui_wgpu::wgpu::action::ACTION_QUEUE_ITEM_CAPACITY),
            window_topology_action_tokens: VecDeque::with_capacity(ui_wgpu::wgpu::action::ACTION_QUEUE_ITEM_CAPACITY),
            next_window_topology_action_token: 1,
            window_topology_refresh_owed: false,
            window_topology_journal_dispatch_owed: false,
            settling: false,
            owed_refresh_scope: UiDirtyScope::None,
            settle_pump: ShellSettlePump::default(),
            #[cfg(target_arch = "wasm32")]
            pending_extension_invocations: Vec::new(),
            #[cfg(target_arch = "wasm32")]
            pending_file_opens: Vec::new(),
            #[cfg(not(target_arch = "wasm32"))]
            shell_io_pending: std::collections::VecDeque::new(),
            import_transfers: VecDeque::new(),
            import_transfer_next: 1,
            fullscreen_toggle_requested: false,
            fullscreen_active: false,
            active_utilities: Vec::new(),
            active_utility_by_window: HashMap::new(),
            action_panel_folded: HashMap::new(),
            search_panel_folded: HashMap::new(),
            utility_bar_folded: HashMap::new(),
            projection_pane_folded: HashMap::new(),
            world_projection_template: HashMap::new(),
            window_icon_overrides: HashMap::new(),
            window_title_overrides: HashMap::new(),
            action_panel_expanded: HashMap::new(),
            staged_action_args: HashMap::new(),
            expanded_command_id: None,
            staged_command_args: HashMap::new(),
            keybinding_capture_control_id: None,
            default_app_pins: HashMap::new(),
            theme_draft: None,
            theme_save_label: String::new(),
            merge_policy: SHELL_DEFAULT_MERGE_POLICY.to_string(),
            pending_theme_import: false,
            agent_chat_draft: String::new(),
            mobile_panel_visible: false,
            mobile_panel_path: Vec::new(),
            sync_backbone_uri: None,
            sync_card_kind: None,
            sync_card_draft: String::new(),
            sync_card_anchor: None,
            last_envelope_dsl: None,
            document_host,
            sync_channel: None,
            next_sync_binding_generation: 1,
            sync_terminal_fault: None,
            sync_status: None,
            sync_link: DocumentLinkStatus::Linked,
            hub_documents: BTreeMap::new(),
            sync_bootstrap_progress: None,
            sync_reseed: None,
            identity: None,
            verified_session_authority: None,
            identity_offline: false,
            identity_env: None,
            shell_session_id: mint_shell_session_id(),
            #[cfg(target_arch = "wasm32")]
            identity_retry_at_ms: 0,
            #[cfg(not(target_arch = "wasm32"))]
            identity_bootstrap_rx: None,
            #[cfg(not(target_arch = "wasm32"))]
            identity_bootstrap_task: None,
            directory_client: None,
            agent_bridge: crate::agent_bridge_door::AgentBridgeTransport::new(),
            agent_bridge_offer_scope: AgentBridgeOfferScopeOwnerV1::default(),
            #[cfg(not(target_arch = "wasm32"))]
            inference_port: None,
            #[cfg(not(target_arch = "wasm32"))]
            document_services: Vec::new(),
            #[cfg(not(target_arch = "wasm32"))]
            inference_port_status: None,
            plugin_install: None,
            document_opening: None,
            icon_export: None,
            document_execution_target_lease: None,
            #[cfg(not(target_arch = "wasm32"))]
            directory_home: None,
            directory_transport,
            directory_cancel,
            directory_commands: ShellDirectoryCommandQueueV1::default(),
            hub_workspace: crate::hub_connection::HubWorkspaceState::new(crate::hub_sign_in::parse_hub_connection_book(prefs_get(HUB_CONNECTION_BOOK_STORAGE_KEY_V1).as_deref(), &shell_hub_bootstrap_origin(), semio_framework_ui_locale::Locale::En)),
            hub_sign_in_task: None,
            hub_spaces_task: None,
            hub_workspace_open: false,
            space_administration: None,
            space_administration_epoch: 0,
            presence_peers: Vec::new(),
            presence_surface: None,
            presence_self: None,
            presence_pointer: None,
            history_cursor: 0,
            history_entries: BTreeMap::new(),
            history_current_checkpoint_id: None,
            history_time_travel: None,
            history_reprojection: None,
            time_travel_focus: None,
            #[cfg(not(target_arch = "wasm32"))]
            time_travel_polled_at_ms: 0.0,
            local_folder_bindings: None,
            local_folder_reattach_tried: None,
            local_folder_direct_reattach: local_folders::LOCAL_FOLDER_DIRECT_REATTACH,
            #[cfg(any(target_arch = "wasm32", test))]
            host_route_folder: None,
            last_uncommitted_edit_at_ms: None,
            auto_checkin_pending: false,
            checkpoint_dispatched: false,
            checkin_dialog_draft: None,
            #[cfg(not(target_arch = "wasm32"))]
            hub_check_in: None,
            layout_save_label: String::new(),
            user_layouts: Vec::new(),
            open_conflicts: Vec::new(),
            selected_conflict_id: None,
            conflicts_seeded: false,
            tool_measures: HashMap::new(),
            window_engagements: HashMap::new(),
            window_actions_documents: HashMap::new(),
            window_search_documents: HashMap::new(),
            window_projection_documents: HashMap::new(),
            search_possibles_open: HashMap::new(),
            window_measures_documents: HashMap::new(),
            window_measures_minted: HashMap::new(),
            utility_collection_expanded: HashMap::new(),
            contributor_instances: HashMap::new(),
            window_content_rects: HashMap::new(),
            retained_hit_windows: HashMap::new(),
            retained_hit_windows_staging: HashMap::new(),
            retained_scene_hits: HashMap::new(),
            retained_scene_hits_staging: HashMap::new(),
            presented_input_epoch: 0,
            next_presented_input_epoch: Some(1),
            presented_input_candidate: None,
            presented_input_geometry: PresentedInputGeometry::default(),
            presented_input_geometry_staging: PresentedInputGeometry::default(),
            retained_body_rects_staging: Vec::new(),
            presented_chrome_accessibility: Vec::new(),
            presented_chrome_accessibility_generation: 0,
            pane_overlay_hits: Vec::new(),
            chrome_floor_scope: None,
            retained_hover_window: None,
            window_silhouettes: HashMap::new(),
            tutorial: None,
            tutorial_pending_document_ops: Vec::new(),
            dispatch_origin: SHELL_DISPATCH_ORIGIN_USER,
            chrome_build: ShellChromeBuildState::default(),
            chrome_present: ShellChromePresentState::default(),
        };
        state.load_persisted_dock();
        state
    }

    //#region 🏠️🧳️PluginHostConfig
    /// 🏠️🧳️ This filter's host config (landing/host app-id roles), or `None` when it doesn't offer a
    /// host-style multi-app experience — see `program_bridge::PluginHostConfig`.
    pub(crate) fn host_config(&self) -> Option<&'static PluginHostConfig> {
        self.host_capability
    }

    /// 🏠️🧳️ The host plugin's own host-role app, self-declaring its `controller_id`/`panel_tabs` — the
    /// generic source of truth for what were previously separate `S_PLAY_CONTROLLER_ID`/
    /// `S_PLAY_CATALOGUE_TAB_ID` literals.
    fn host_app(&self) -> Option<&AppDefinition> {
        let cfg = self.host_config()?;
        let program = self.plugins.iter().find(|p| p.plugin_id == cfg.plugin_id)?;
        program.manifest.apps.iter().find(|app| app.id == cfg.host_app_id)
    }

    pub(crate) fn host_controller_id(&self) -> Option<String> {
        self.host_app().map(|app| app.controller_id.clone())
    }

    /// 🕒️ Who a `noteShellCommand` is addressed to: the LIVE session's app first, and only then the
    /// host program's own app — the same order `document_controller_id` resolves a document's action
    /// bindings in, and the one React uses (`noteShellCommand` closes over the mounted app).
    ///
    /// 🩸️ The command journal used to ask [`Self::host_controller_id`] alone. A play serve mounts NO
    /// host app (there is no space/studio around the plugin), so `host_config()` is `None`, the
    /// controller resolved to nothing and every window-cap and split-resize command returned before it
    /// dispatched: the dock's close, focus and resize journalled NOTHING while the panel toggles right
    /// beside them journalled fine, because those had always read the session (ticket
    /// 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w9c-behaviour-parity-run-2.md` steps 15–17).
    pub(crate) fn shell_command_controller_id(&self) -> Option<String> {
        self.session.as_ref().map(|session| session.app.controller_id.clone()).or_else(|| self.host_controller_id())
    }

    /// 🎬️ The controller every retained document's own action bindings dispatch to: the live
    /// session's app, falling back to the host program's own app when no session is mounted (a
    /// framework panel painted before the guest opened).
    pub(crate) fn document_controller_id(&self) -> String {
        self.session.as_ref().map(|session| session.app.controller_id.clone()).or_else(|| self.host_controller_id()).unwrap_or_default()
    }

    /// 🪟️ The session whose exact retained document occupies this dock body.
    pub(crate) fn window_document_session(&self, window_id: &str) -> Option<&ActiveSession> {
        if window_id == "spawned" { self.spawned_session.as_ref().filter(|_| self.space_mode) } else { self.session.as_ref() }
    }

    /// 🎯️ The app instance addressed by a retained action's controller and surface.
    pub(crate) fn action_session(&self, action: &ActionDescriptor) -> Option<&ActiveSession> {
        let requested = action.args.as_ref().and_then(|args| args.get("windowId")).and_then(semio_framework_value::DslValue::as_str);
        let spawned = self.spawned_session.as_ref().filter(|_| self.space_mode);
        if requested == Some("spawned") || spawned.is_some_and(|session| requested.is_some_and(|id| session.view_state.window_instances.iter().any(|window| window.id == id))) {
            return spawned.filter(|session| session.app.controller_id == action.controller_id);
        }
        if let Some(spawned) = spawned.filter(|session| session.app.controller_id == action.controller_id && self.session.as_ref().is_none_or(|host| host.app.controller_id != action.controller_id)) {
            return Some(spawned);
        }
        self.session.as_ref().filter(|session| session.app.controller_id == action.controller_id)
    }

    /// 📨️ Commands select the declared plugin and app among the mounted sessions.
    pub(crate) fn command_session(&self, owner: &semio_framework::manifest::CommandOwnerAddress, window_id: Option<&str>) -> Option<&ActiveSession> {
        use semio_framework::manifest::CommandOwnerAddress;
        let matches = |session: &&ActiveSession| match owner {
            CommandOwnerAddress::Os => false,
            CommandOwnerAddress::Plugin { plugin_id } => &session.plugin_id == plugin_id,
            CommandOwnerAddress::App { plugin_id, app_id } | CommandOwnerAddress::Mode { plugin_id, app_id, .. } => &session.plugin_id == plugin_id && &session.app.id == app_id,
        };
        let spawned = self.spawned_session.as_ref().filter(|_| self.space_mode).filter(matches);
        if let Some(session) = spawned.filter(|session| window_id.is_some_and(|id| id == "spawned" || session.view_state.window_instances.iter().any(|window| window.id == id))) {
            return Some(session);
        }
        self.session.as_ref().filter(matches).or(spawned)
    }

    /// 🪪️ Session equality follows the plugin's exact instance and declared app.
    pub(crate) fn same_session(left: &ActiveSession, right: &ActiveSession) -> bool {
        left.plugin_id == right.plugin_id && left.instance_id == right.instance_id && left.app.id == right.app.id
    }

    /// 🧭️ A shell surface alias resolves to its owner's concrete guest window address.
    pub(crate) fn session_action_invocation(&self, session: &ActiveSession, action: &ActionDescriptor, view: &ViewModel) -> Result<semio_framework::manifest::ActionInvocation, String> {
        let supplied_window = action.args.as_ref().and_then(|args| args.get("windowId")).and_then(semio_framework_value::DslValue::as_str);
        let requested_window = if supplied_window == Some("spawned") { session.view_state.window_id.as_deref() } else { supplied_window };
        let panel_leaves: Vec<&str> = Self::flatten_panel_tab_leaves(&session.app.panel_tabs).into_iter().map(|tab| tab.id()).collect();
        let window_instance_id = action_window_instance_id(requested_window, &panel_leaves, session.view_state.window_id.as_deref(), view.focused_window_id.as_deref(), &session.app.window_kinds.first().id);
        let window_kind_id = view.window_instances.iter().find(|instance| instance.id == window_instance_id).map(|instance| instance.window_kind_id.clone()).or_else(|| session.app.window_kinds.iter().find(|kind| kind.id == window_instance_id).map(|kind| kind.id.clone())).ok_or_else(|| format!("action window instance {window_instance_id} has no declared kind"))?;
        let panel_origin = requested_window.is_some_and(|id| panel_leaves.contains(&id));
        let mut arguments: std::collections::BTreeMap<String, DslValue> = action.args.as_ref().and_then(semio_framework_value::DslValue::as_object).map(|entries| entries.iter().filter(|(key, _)| !(panel_origin && key == "windowId")).cloned().collect()).unwrap_or_default();
        if supplied_window == Some("spawned") {
            arguments.insert("windowId".into(), semio_framework_value::DslValue::from(serde_json::json!(window_instance_id)));
        }
        Ok(semio_framework::manifest::ActionInvocation {
            address: semio_framework::manifest::ActionAddress { plugin_id: session.plugin_id.clone(), app_id: session.app.id.clone(), mode_id: session.view_state.active_mode_id.clone().unwrap_or_else(|| session.app.default_mode_id.clone()), window_kind_id, window_instance_id, action_id: action.action.clone() },
            arguments,
        })
    }

    fn host_catalogue_tab_id(&self) -> Option<String> {
        self.host_app().and_then(|app| Self::flatten_panel_tab_leaves(&app.panel_tabs).first().map(|tab| tab.id().to_string()))
    }
    //#endregion 🏠️🧳️PluginHostConfig

    /// 🌌️ TEMP(Wave 3): replace with workflow_palette() once the shell palette wiring lands. Reads
    /// `program.manifest.apps` directly (one `SpaceProgramEntry` per app) — `PluginManifest.workflows`/
    /// `WorkflowDefinition` were deleted in Wave 0 (WP-0.1); the real palette derivation moves to
    /// `semio-framework-os`'s `registry::workflow_palette()` (`AppIo`-driven) once the browser shell wires
    /// it in. `yields` is empty here (no registry lookup at this layer) — matches every other Wave-1
    /// `WorkflowNode.yields` derivation until Wave 2 populates apps' declared output ports.
    pub fn build_space_workflows(&self) -> Vec<SpaceProgramEntry> {
        self.plugins
            .iter()
            .flat_map(|program| {
                program.manifest.apps.iter().map(|app| SpaceProgramEntry {
                    plugin_id: program.plugin_id.clone(),
                    workflow_step_id: app.id.clone(),
                    app_id: app.id.clone(),
                    label: app.label.resolve(self.active_terminology(), self.active_locale()).to_string(),
                    breadcrumb: app.breadcrumb.clone(),
                    yields: String::new(),
                })
            })
            .collect()
    }

    fn panel_identifier(value: &str) -> bool {
        !value.is_empty() && value.chars().count() <= 256 && !value.chars().any(|character| character <= '\u{1f}' || character == '\u{7f}')
    }

    fn validate_panel_state(state: &SpacePanelState) -> Result<(), String> {
        if !Self::panel_identifier(&state.active_panel_tab) {
            return Err("host-panel.invalid-active-tab".into());
        }
        if state.spawned_apps.len() > 64 {
            return Err("host-panel.spawned-capacity".into());
        }
        let mut ids = std::collections::BTreeSet::new();
        for spawned in &state.spawned_apps {
            if !Self::panel_identifier(&spawned.id) || !Self::panel_identifier(&spawned.plugin_id) || !Self::panel_identifier(&spawned.app_id) {
                return Err("host-panel.invalid-spawned-identity".into());
            }
            if !ids.insert(spawned.id.as_str()) {
                return Err("host-panel.duplicate-spawned-identity".into());
            }
            if spawned.label.chars().count() > 65_536 || spawned.breadcrumb.len() > 64 || spawned.breadcrumb.iter().any(|part| part.chars().count() > 65_536) {
                return Err("host-panel.spawned-text-capacity".into());
            }
        }
        if state.active_spawned_id.as_ref().is_some_and(|id| !Self::panel_identifier(id) || !ids.contains(id.as_str())) {
            return Err("host-panel.invalid-active-spawned".into());
        }
        Ok(())
    }

    pub fn panel_state_from_view(view_state: &ViewModel) -> Result<Option<SpacePanelState>, String> {
        let Some(json) = view_state.panel_json.as_ref() else {
            return Ok(None);
        };
        if json.chars().count() > 65_536 {
            return Err("host-panel.json-capacity".into());
        }
        let state: SpacePanelState = serde_json::from_str(json).map_err(|_| "host-panel.invalid-json".to_string())?;
        Self::validate_panel_state(&state)?;
        Ok(Some(state))
    }

    pub fn panel_json(state: &SpacePanelState) -> Result<String, String> {
        Self::validate_panel_state(state)?;
        let json = serde_json::to_string(state).map_err(|_| "host-panel.encode".to_string())?;
        if json.chars().count() > 65_536 {
            return Err("host-panel.json-capacity".into());
        }
        Ok(json)
    }

    pub(crate) fn default_host_panel_state(&self) -> Result<SpacePanelState, String> {
        let active_panel_tab = self.host_catalogue_tab_id().ok_or_else(|| "host-panel.missing-configured-leaf".to_string())?;
        let state = SpacePanelState { active_panel_tab, spawned_apps: vec![], active_spawned_id: None };
        Self::validate_panel_state(&state)?;
        Ok(state)
    }

    pub fn prepare_hot_reload(&mut self, plugins: Vec<ProgramBridgeEntry>) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let retained_home = self.directory_home.as_ref().map(|home| (home.plugin_id.clone(), home.instance_id));
            if let Some(mut home) = self.directory_home.take() {
                if let Some((plugin_id, instance_id)) = home.take_destroy_authority() {
                    if let Some(program) = self.plugins.iter().find(|entry| entry.plugin_id == plugin_id) {
                        program.destroy_app(instance_id);
                    }
                }
            }
            if let Some(session) = self.session.take() {
                if retained_home.as_ref().is_none_or(|(plugin_id, instance_id)| plugin_id != &session.plugin_id || *instance_id != session.instance_id) {
                    if let Some(program) = self.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id) {
                        program.destroy_app(session.instance_id);
                    }
                }
            }
        }
        #[cfg(target_arch = "wasm32")]
        if let Some(session) = self.session.take() {
            if let Some(program) = self.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id) {
                program.destroy_app(session.instance_id);
            }
        }
        self.plugins = plugins;
    }

    pub async fn hot_reload_plugins(&mut self, plugins: Vec<ProgramBridgeEntry>) -> Result<(), String> {
        self.prepare_hot_reload(plugins);
        self.boot().await
    }

    /// 🧯 Records one plugin's boot activation fault and refreshes the shell's status line. The shell
    /// stays alive: a trapped guest is that plugin's failure, not the renderer's.
    fn record_plugin_fault(&mut self, plugin_id: &str, app_id: &str, detail: String) {
        Self::debug_log(&format!("[TRACE] wgpu shell boot: plugin {plugin_id} app {app_id} faulted: {detail}"));
        self.plugin_faults.push(ShellPluginFault { plugin_id: plugin_id.to_string(), app_id: app_id.to_string(), detail,#[cfg(not(target_arch="wasm32"))] frame:None,#[cfg(not(target_arch="wasm32"))] kernel:None,#[cfg(not(target_arch="wasm32"))] actor_call:None });
        self.error = self.fault_status();
    }

    /// 🧯 Records one surface's render fault and refreshes the shell's status line, WITHOUT failing the
    /// refresh. A surface repeats across refreshes, so the same surface never accumulates two entries.
    ///
    /// 📣️ `eprintln!` is a no-op in a `wasm32-unknown-unknown` Worker — a fault recorded there
    /// left no trace at all, which is why a blank body read as "nothing happened". `debug_log` is
    /// this type's ONE trace sink for exactly that reason; no diagnostic in this file may use
    /// `eprintln!` except `debug_log`'s own native arm.
    pub(crate) fn record_surface_fault(&mut self, surface_id: &str, body_key: &str, refusal: crate::program_bridge::ProgramFault) {
        #[cfg(not(target_arch = "wasm32"))]
        let (frame,kernel,actor_call)=(refusal.frame,refusal.kernel,refusal.actor_call);
        #[cfg(not(target_arch = "wasm32"))]
        let detail = if frame.is_some()||kernel.is_some()||actor_call.is_some() { LocalizedLabel::native("The input could not be delivered.", "Die Eingabe konnte nicht zugestellt werden.").resolve(self.active_terminology(), self.active_locale()).to_string() } else { refusal.text };
        #[cfg(target_arch = "wasm32")]
        let detail = refusal.text;
        Self::debug_log(&format!("[TRACE] wgpu-shell surface fault surface={surface_id} body={body_key} detail={detail}"));
        if let Some(existing) = self.surface_faults.iter_mut().find(|fault| fault.surface_id == surface_id) {
            existing.body_key = body_key.to_string();
            existing.detail = detail;
            #[cfg(not(target_arch = "wasm32"))]
            { existing.frame = frame;existing.kernel=kernel;existing.actor_call=actor_call; }
        } else {
            self.surface_faults.push(ShellSurfaceFault { surface_id: surface_id.to_string(), body_key: body_key.to_string(), detail, #[cfg(not(target_arch = "wasm32"))] frame,#[cfg(not(target_arch="wasm32"))] kernel,#[cfg(not(target_arch="wasm32"))] actor_call });
        }
        self.error = self.fault_status();
    }

    /// 🧯️ A retained document whose per-frame cursor reached its TERMINAL fault phase must release the
    /// chrome step, not hold it. `render_ui_document_step` answers `false` for "not complete", and the
    /// chrome walk used to read that as "come back next opportunity" — which for a terminal fault is
    /// never, so one unreadable body wedged the whole frame transaction and the shell presented
    /// nothing at all (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-blank-paint-2026-09-12.md`).
    /// The surface's own fault card is the right consequence; a black canvas is not.
    pub(crate) fn record_document_paint_fault(&mut self, surface_id: &str) {
        if self.surface_faults.iter().any(|fault| fault.surface_id == surface_id) {
            return;
        }
        let body_key = self.document_body_key(surface_id).unwrap_or_else(|| surface_id.to_string());
        self.record_surface_fault(surface_id, &body_key, "retained document ingress reached its terminal fault".into());
    }

    /// 🪟️ One retained body finished a paint: its paint fault (if any) clears, and its windowed tree containers are measured and
    /// reported ([`Self::observe_tree_windows`]).
    pub(crate) fn note_retained_body_painted(&mut self, surface_id: &str) {
        if self.surface_faults.iter().any(|fault| fault.surface_id == surface_id) {
            self.surface_faults.retain(|fault| fault.surface_id != surface_id);
            self.error = self.fault_status();
        }
        self.observe_tree_windows(surface_id);
    }

    pub(crate) fn document_body_key(&self, surface_id: &str) -> Option<String> {
        let session = self.session.as_ref()?;
        let window_kind_id = self.live_window_kind_id(session, surface_id);
        session
            .app
            .window_kinds
            .iter()
            .find(|kind| Some(kind.id.as_str()) == window_kind_id)
            .map(|kind| kind.body_key.clone())
            .or_else(|| Self::flatten_panel_tab_leaves(&session.app.panel_tabs).into_iter().find(|tab| tab.id() == surface_id).and_then(|tab| tab.body_key.clone()))
    }

    /// 🗣️ The per-plugin boot status, in English or German with no default language — `None` while
    /// every plugin the boot asked for activated.
    pub fn plugin_fault_status(&self) -> Option<String> {
        if self.plugin_faults.is_empty() {
            return None;
        }
        let is_de = self.locale_id == "de";
        let label = shell_chrome_string("plugin.faulted", is_de);
        Some(self.plugin_faults.iter().map(|fault| format!("{label}: {} ({}) — {}", fault.plugin_id, fault.app_id, fault.detail)).collect::<Vec<_>>().join(" · "))
    }

    /// 🗣️ The per-surface render status, in English or German with no default language — `None` while
    /// every surface this refresh asked for published.
    pub fn surface_fault_status(&self) -> Option<String> {
        if self.surface_faults.is_empty() {
            return None;
        }
        let is_de = self.locale_id == "de";
        let label = shell_chrome_string("surface.faulted", is_de);
        Some(self.surface_faults.iter().map(|fault| format!("{label}: {} ({}) — {}", fault.surface_id, fault.body_key, fault.detail)).collect::<Vec<_>>().join(" · "))
    }

    /// 🧯 Every live per-plugin and per-surface fault as one status line — the shell's typed fault card.
    pub fn fault_status(&self) -> Option<String> {
        let parts: Vec<String> = [self.plugin_fault_status(), self.surface_fault_status()].into_iter().flatten().collect();
        (!parts.is_empty()).then(|| parts.join(" · "))
    }

    /// 👁️✏️ The boot-resolved surface role, as `AppRole`. The renderer keeps it as the UI crate's
    /// `ChromeRole` (that is the type its chrome reads); the boot app selection needs the manifest's
    /// own spelling, and the two are wire-identical by contract (`"viewer"`/`"editor"`).
    fn boot_app_role() -> semio_framework::manifest::AppRole {
        match crate::boot_app_role() {
            ui_wgpu::wgpu::component::role_chrome::ChromeRole::Viewer => semio_framework::manifest::AppRole::Viewer,
            ui_wgpu::wgpu::component::role_chrome::ChromeRole::Editor => semio_framework::manifest::AppRole::Editor,
        }
    }

    /// 🚪️ Opens one plugin's app for the boot session, isolating an activation fault as a per-plugin
    /// status. Returns `None` when the plugin is not loaded or its actor refused to open.
    async fn open_boot_instance(&mut self, plugin_id: &str, app_id: &str) -> Option<u32> {
        let Some(program) = self.plugins.iter().find(|entry| entry.plugin_id == plugin_id).cloned() else {
            self.record_plugin_fault(plugin_id, app_id, "plugin is not loaded".to_string());
            return None;
        };
        match program.create_app(app_id).await {
            Ok(instance_id) => Some(instance_id),
            Err(refusal) => {
                let detail = self.note_refused_open(refusal);
                self.record_plugin_fault(plugin_id, app_id, detail);
                None
            }
        }
    }

    pub async fn boot(&mut self) -> Result<(), String> {
        self.plugin_faults.clear();
        self.error = None;
        Self::debug_log("[TRACE] wgpu-shell boot: begin");
        #[cfg(target_arch = "wasm32")]
        if let Err(error) = self.refresh_extension_store().await {
            Self::debug_log(&format!("[TRACE] wgpu extension inventory unavailable: {error}"));
        }
        if let Some(cfg) = self.host_config() {
            let host_plugin_id = cfg.plugin_id.to_string();
            let semio_s_plugin_space = self.plugins.iter().find(|p| p.plugin_id == host_plugin_id).ok_or("host program missing")?;
            let s_app = semio_s_plugin_space.manifest.apps.iter().find(|app| app.id == cfg.landing_app_id).or_else(|| semio_s_plugin_space.manifest.apps.first()).ok_or("host program missing landing app")?.clone();
            let panel_state = self.default_host_panel_state()?;
            let Some(instance_id) = self.open_boot_instance(&host_plugin_id, &s_app.id).await else {
                return self.settle_boot().await;
            };
            let view_state = ViewModel {
                active_mode_id: Some(s_app.default_mode_id.clone()),
                active_window_kind_id: Some(s_app.window_kinds.first().id.clone()),
                active_utility_id: None,
                panel_json: Some(Self::panel_json(&panel_state)?),
                extension_input_json: None,
                session_identity: None,
                locale: self.active_locale(),
                terminology: self.active_terminology(),
                window_id: None,
                focused_window_id: Some(s_app.window_kinds.first().id.clone()),
                window_instances: Vec::new(),
                active_tool_id: None,
                active_utility_by_window_id: Default::default(),
                tool_run_trace_cursor_by_window_id: Default::default(),
                tree_windows: Vec::new(),
                tree_viewport_rows: None,
            };
            self.active_window_id = Some(s_app.window_kinds.first().id.clone());
            let session = ActiveSession { plugin_id: host_plugin_id, instance_id, app: s_app, view_state };
            #[cfg(not(target_arch = "wasm32"))]
            {
                self.directory_home = Some(DirectoryHomeProjection::new(session.plugin_id.clone(), session.instance_id, session.app.clone(), session.view_state.clone()).map_err(|error| error.to_string())?);
            }
            self.session = Some(session);
        } else {
            let selection = {
                let programs: Vec<(&str, &semio_framework::PluginManifest)> = self.plugins.iter().map(|entry| (entry.plugin_id.as_str(), &entry.manifest)).collect();
                let request = BootSelectionRequest { plugin_id: resolve_registry_plugin_id(&self.plugin_filter), app_id: resolve_playground_app_id(&self.plugin_filter) };
                select_boot_program(&programs, request, Self::boot_app_role(), crate::boot_app_id().as_deref())
            };
            let Some((index, app)) = selection else {
                let plugin_id = resolve_registry_plugin_id(&self.plugin_filter).to_string();
                let app_id = resolve_playground_app_id(&self.plugin_filter).unwrap_or("").to_string();
                self.record_plugin_fault(&plugin_id, &app_id, "the requested plugin contributed no app to this boot".to_string());
                return self.settle_boot().await;
            };
            let plugin_id = self.plugins[index].plugin_id.clone();
            let Some(instance_id) = self.open_boot_instance(&plugin_id, &app.id).await else {
                return self.settle_boot().await;
            };
            self.active_window_id = Some(app.window_kinds.first().id.clone());
            self.session = Some(ActiveSession {
                plugin_id,
                instance_id,
                app: app.clone(),
                view_state: ViewModel {
                    active_mode_id: Some(app.default_mode_id.clone()),
                    active_window_kind_id: self.active_window_id.clone(),
                    active_utility_id: None,
                    panel_json: None,
                    extension_input_json: None,
                    session_identity: None,
                    locale: self.active_locale(),
                    terminology: self.active_terminology(),
                    window_id: None,
                    focused_window_id: self.active_window_id.clone(),
                    window_instances: Vec::new(),
                    active_tool_id: None,
                    active_utility_by_window_id: Default::default(),
                    tool_run_trace_cursor_by_window_id: Default::default(),
                    tree_windows: Vec::new(),
                    tree_viewport_rows: None,
                },
            });
        }
        self.apply_boot_mode();
        if let Some(session) = &self.session {
            Self::debug_log(&format!("[TRACE] wgpu-shell boot: program={} app={} mode={:?}", session.plugin_id, session.app.id, session.view_state.active_mode_id));
        }
        self.settle_boot().await
    }

    /// 🎭️ Applies the boot-requested mode (`?mode=`) to the session the boot just opened: the mode
    /// becomes active AND its authored layout becomes the dock's, exactly as
    /// `handle_control_command`'s `playground.navbar.modes.*` arm does for a click. A mode the open
    /// app does not declare is ignored — a url is not a place to hard-fail a shell, the same rule
    /// `ChromeRole::from_boot_env` follows for `?role=`.
    fn apply_boot_mode(&mut self) {
        let Some(mode_id) = crate::boot_app_mode() else { return };
        let Some(session) = self.session.as_mut() else { return };
        if !session.app.modes.iter().any(|mode| mode.id == mode_id) {
            Self::debug_log(&format!("[TRACE] wgpu-shell boot mode {mode_id:?} is not declared by app {} — keeping {}", session.app.id, session.app.default_mode_id));
            return;
        }
        session.view_state.active_mode_id = Some(mode_id.clone());
        let layout = semio_framework::resolve_layout_for_mode(&session.app, &mode_id);
        if let Some(layout) = layout {
            self.layout_override = Some(layout);
            self.sync_dock();
            self.active_window_id = self.dock.active_window_id.clone();
        }
    }

    /// 🏁️ The boot tail every path shares — chrome sync plus the first UI refresh. A boot that opened
    /// no session (every requested plugin faulted) still runs it, so the shell paints its chrome and
    /// its per-plugin status instead of leaving the page on the loader.

    pub(crate) fn instant_now_ms() -> f64 {
        #[cfg(target_arch = "wasm32")]
        {
            js_sys::Date::now()
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            0.0
        }
    }

    pub(crate) fn debug_log(line: &str) {
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(line));
        #[cfg(not(target_arch = "wasm32"))]
        eprintln!("{line}");
    }

    fn declare_boot_subphase(phase: &str, state: &str, elapsed_ms: f64) {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            let global = js_sys::global();
            if let Ok(func) = js_sys::Reflect::get(&global, &wasm_bindgen::JsValue::from_str("semioDeclareBootSubphase")) {
                if let Ok(func) = func.dyn_into::<js_sys::Function>() {
                    let _ = func.call3(&wasm_bindgen::JsValue::NULL, &wasm_bindgen::JsValue::from_str(phase), &wasm_bindgen::JsValue::from_str(state), &wasm_bindgen::JsValue::from_f64(elapsed_ms));
                }
            }
        }
        let _ = (phase, state, elapsed_ms);
    }

    pub(crate) fn app_owns_command(app: &semio_framework::AppDefinition, command_id: &str) -> bool {
        app.commands.iter().any(|command| command.id == command_id)
    }

    /// 🧩️ The guest answers `setContributions` with the effects it could not arm before
    /// its extension registry moved — a `flowEvalTick` per attached preview window. React's
    /// `ShellHost` dispatches exactly these through `applyHostEffects`; this target dropped
    /// the whole answer on the floor, so the guest's latch stayed armed and no preview ever
    /// evaluated (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub(crate) async fn push_contributions(&mut self) -> Result<(), String> {
        let Some(session) = self.session.clone() else {
            return Ok(());
        };
        if !Self::app_owns_command(&session.app, "setContributions") {
            Self::debug_log(&format!("[TRACE] contributions push {}", serde_json::json!({ "plugin": session.plugin_id, "app": session.app.id, "skipped": "app-owns-no-setContributions", "encoding": "pack", "crossings": 0 })));
            return Ok(());
        }
        Self::declare_boot_subphase("shell-boot:contributions-push", "enter", 0.0);
        let started = Self::instant_now_ms();
        #[cfg(target_arch = "wasm32")]
        {
            let view_json = serde_json::to_string(&self.live_view_state(&session)).unwrap_or_else(|_| "{}".into());
            if let Some(plugin) = self.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).cloned() {
                Self::debug_log(&format!("[TRACE] contributions push {}", serde_json::json!({ "plugin": plugin.plugin_id, "app": session.app.id, "active": true, "encoding": "pack", "crossings": 1 })));
                match plugin.push_scoped_contributions(session.instance_id, &session.app.id, &view_json).await {
                    Ok(result) => {
                        Self::debug_log(&format!("[TRACE] setContributions deferred effects {}", serde_json::json!({ "plugin": plugin.plugin_id, "app": session.app.id, "effects": result.requested_effects.len() })));
                        self.queue_host_effects(&session.app.controller_id, result.requested_effects);
                    }
                    Err(error) => Self::debug_log(&format!("[TRACE] setContributions command failed {} {error}", plugin.plugin_id)),
                }
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = session;
            Self::debug_log("[TRACE] contributions push skipped native-no-pack-sender");
        }
        Self::declare_boot_subphase("shell-boot:contributions-push", "leave", (Self::instant_now_ms() - started).max(0.0));
        Ok(())
    }

    /// 📇️ ticket §1 — non-blocking (submits finite shared-pool turns and returns immediately, see
    /// `bootstrap_identity`'s own doc): a slow/unreachable hub must never delay the first frame.
    ///
    /// 🌐️ Browser: there is no credential to claim and no pool future to submit — only the boot
    /// environment to record. `poll_browser_identity` (every-frame, retry-floored) does the one
    /// `GET /auth/sessions/me` round trip, so this stays as non-blocking as the native arm.
    ///
    /// 🧩️ Contributions cross BEFORE the first render, the way React's `refreshUi` starts its
    /// publisher before its own first await: a surface rendered against an EMPTY flow-extension
    /// registry arms nothing at all (the guest's `may_rearm` gate — a graph whose operator kinds
    /// the registry cannot serve owes no continuation), so pushing after the refresh left every
    /// preview window with no tick and no chain to re-arm
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    ///
    /// 🫀️ The boot ARMS the settle lane and returns; it does not converge. `flush_deferred_actions`
    /// used to run the whole chain to a fixed point right here, so an `?example=` boot spent its
    /// entire evaluation INSIDE `boot_shell` — measured on 6118 as `boot_shell leave 7 226 ms` with
    /// the first chrome at 9 212 ms against 1 880 ms / 4 101 ms for the same build with no example,
    /// i.e. 5.3 s of convergence with nothing painted and no status pill to show it
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-progress-visibility-2026-09-14.md` §8.1).
    /// The frame loop takes it from here, one [`Self::settle_pump_step`] per frame, which is what
    /// React's refresh lane has always done with its owed slot.
    async fn settle_boot(&mut self) -> Result<(), String> {
        self.sync_dock();
        self.sync_session_chrome();
        #[cfg(not(target_arch = "wasm32"))]
        self.bootstrap_identity();
        #[cfg(target_arch = "wasm32")]
        {
            self.identity_env = resolve_identity_env();
        }
        self.push_contributions().await?;
        self.announce_session_example().await?;
        Self::declare_boot_subphase("shell-boot:refresh-ui", "enter", 0.0);
        let refresh_started = Self::instant_now_ms();
        self.refresh_ui(UiDirtyScope::Full).await?;
        Self::declare_boot_subphase("shell-boot:refresh-ui", "leave", (Self::instant_now_ms() - refresh_started).max(0.0));
        self.owe_settle();
        Ok(())
    }

    /// 📚️ Announces the RESOLVED example to a freshly mounted instance — the wgpu twin of React's
    /// per-instance boot-example effect (`🏛️ShellHost/🟦️.tsx`), which runs for EVERY new session
    /// instance: the boot one (from `settle_boot`, AFTER `push_contributions` armed the guest's
    /// flow-extension registry and before the first `refresh_ui`) and every successor of a role switch.
    /// `setActiveExample` is dispatched exactly as `handle_control_command`'s `shell.example.*` arm
    /// does for a click, so the guest opens the document the navbar picker shows and journals it.
    ///
    /// 🩸️ The boot used to announce only an example a `?example=` query named: without the query the
    /// picker showed the dialect's first example while the guest still held its empty genesis document —
    /// an empty board and a History with no row where React boots on "Set Active Example"
    /// (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING, live fault F11). Before that a role switch
    /// journalled nothing (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, packet W10b).
    /// `sync_session_chrome` has already run `resolve_boot_example_id` for the instance's dialect (a
    /// query id the dialect does not author falls back, never a boot failure), so the id announced here
    /// is exactly the one the navbar picker shows.
    pub(crate) async fn announce_session_example(&mut self) -> Result<(), String> {
        let Some(example_id) = self.active_example_id.clone().filter(|id| !id.is_empty()) else {
            return Ok(());
        };
        let Some(controller_id) = self.session.as_ref().map(|session| session.app.controller_id.clone()) else {
            return Ok(());
        };
        Self::debug_log(&format!("[TRACE] wgpu-shell session example {}", serde_json::json!({ "exampleId": example_id, "controller": controller_id })));
        self.dispatch_action(ActionDescriptor { controller_id, action: "setActiveExample".into(), args: crate::action_args_json!({ "exampleId": example_id }) }).await
    }

    /// 📚️ The examples the OPEN surface offers, resolved by DIALECT through the one shared predicate
    /// (`manifest::examples_for_app`, twin of React's `examplesForApp`) rather than by plugin id: an
    /// example is a document of the artifact's subset, so a plugin that publishes several dialects
    /// must never offer another artifact's fixtures here (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub(crate) fn session_example_rows(&self) -> Vec<ShellExampleRow> {
        let Some(session) = &self.session else {
            return Vec::new();
        };
        let Some(plugin) = self.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id) else {
            return Vec::new();
        };
        shell_example_rows(&plugin.manifest.examples, &session.app, self.active_example_id.as_deref(), self.active_terminology(), self.active_locale())
    }

    /// 🧭️ The dock's tab sources are the session's own panel tabs, so every chrome sync rebuilds it.
    /// The persisted override layer is keyed by app id, so a NEW app additionally reloads that app's
    /// layer — reloading on every sync would clobber the reveal `apply_host_panel_selection` just made.
    pub(crate) fn sync_session_chrome(&mut self) {
        let Some(session) = &self.session else {
            return;
        };
        let examples: Vec<String> = self
            .plugins
            .iter()
            .find(|entry| entry.plugin_id == session.plugin_id)
            .map(|entry| shell_offered_examples(&entry.manifest.examples, &session.app).into_iter().map(|example| example.id.clone()).collect())
            .unwrap_or_default();
        let resolved = resolve_boot_example_id(self.active_example_id.as_deref().unwrap_or(""), &examples, crate::boot_app_example().as_deref());
        self.active_example_id = (!resolved.is_empty()).then_some(resolved);
        let app_id = self.dock_store_app_id();
        if self.dock_loaded_app_id != app_id {
            self.dock_loaded_app_id = app_id;
            self.load_persisted_dock();
        } else {
            self.sync_dock_tabs();
        }
    }

    pub(crate) fn flatten_panel_tab_leaves(tabs: &[PanelTabDefinition]) -> Vec<&PanelTabDefinition> {
        tabs.iter().flat_map(|tab| if tab.children.is_empty() { vec![tab] } else { Self::flatten_panel_tab_leaves(&tab.children) }).collect()
    }

    /// 🧭️ Where the tab lives NOW wins over its declared group — a tab dragged to another anchor
    /// must open where the user put it, which is exactly why `reveal_dock_tab` locates it in the live
    /// dock first and only falls back on `PanelGroup::anchor()` when the dock has no node for it yet.
    pub(crate) fn apply_host_panel_selection(&mut self, action: &ActionDescriptor) -> Option<Result<(), String>> {
        if self.host_config().is_none() || action.action != "setActivePanelTab" {
            return None;
        }
        let target_app = self.session.as_ref().filter(|session| session.app.controller_id == action.controller_id).map(|session| &session.app).or_else(|| self.host_app().filter(|app| app.controller_id == action.controller_id));
        let Some(target_app) = target_app else {
            return None;
        };
        let tab_id = match action.args.as_ref().and_then(|args| args.get("tabId")).and_then(semio_framework_value::DslValue::as_str) {
            Some(tab_id) if Self::panel_identifier(tab_id) => tab_id.to_string(),
            _ => return Some(Err("host-panel.invalid-tab-id".into())),
        };
        let Some(group) = Self::flatten_panel_tab_leaves(&target_app.panel_tabs).into_iter().find(|tab| tab.id() == tab_id).map(|tab| tab.group) else {
            return Some(Err("host-panel.tab-is-not-configured-leaf".into()));
        };
        let Some(current) = self.session.as_ref() else {
            return Some(Err("host-panel.session-missing".into()));
        };
        let mut panel = match Self::panel_state_from_view(&current.view_state) {
            Ok(Some(panel)) => panel,
            Ok(None) => match self.default_host_panel_state() {
                Ok(panel) => panel,
                Err(error) => return Some(Err(error)),
            },
            Err(error) => return Some(Err(error)),
        };
        panel.active_panel_tab = tab_id.clone();
        let panel_json = match Self::panel_json(&panel) {
            Ok(panel_json) => panel_json,
            Err(error) => return Some(Err(error)),
        };
        let current_identity = (current.plugin_id.clone(), current.instance_id);
        if let Some(session) = self.session.as_mut() {
            session.view_state.panel_json = Some(panel_json.clone());
        }
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(home) = self.directory_home.as_mut().filter(|home| home.is_instance(&current_identity.0, current_identity.1)) {
            home.view_state.panel_json = Some(panel_json);
        }
        if self.reveal_dock_tab(&tab_id).is_none() {
            let anchor = PanelAnchor::from_group(group);
            let state = self.anchor_state_mut(anchor);
            state.path = vec![tab_id];
            state.visible = true;
        }
        self.sync_session_chrome();
        Some(Ok(()))
    }

    pub(crate) fn sync_dock(&mut self) {
        if let Some(session) = &self.session {
            let owner=(session.plugin_id.clone(),session.instance_id,session.app.id.clone());
            let incoming_layout = self.layout_override.as_ref().or(session.app.default_layout.as_ref());
            let roster = Self::session_window_instances(session, &self.dock).into_iter().map(|instance| instance.id).collect::<Vec<_>>();
            let owner_changed = self.dock_instance_owner.as_ref().is_none_or(|(plugin_id, instance_id, app_id)| plugin_id != &session.plugin_id || *instance_id != session.instance_id || app_id != &session.app.id);
            let source_changed = owner_changed || self.dock_input_identity.as_ref().is_none_or(|input| input.source.as_ref() != incoming_layout);
            let mode_layout = incoming_layout.map(dock_mode_layout_identity);
            let input_changed = owner_changed || self.dock_input_identity.as_ref().is_none_or(|input| input.mode_layout != mode_layout || input.roster != roster);
            if source_changed || (incoming_layout.is_none() && input_changed) {
                let maximized = self.dock.maximized_stack.clone();
                if let Some(layout) = &self.layout_override {
                    self.dock.apply_layout_diff(layout);
                    if self.dock.active_window_id.is_none() {
                        self.dock.active_window_id = self.active_window_id.clone().or_else(|| session.view_state.active_window_kind_id.clone());
                    }
                } else {
                    let mobile = self.dock.mobile;
                    self.dock = DockState::from_app(&session.app, self.active_window_id.as_deref());
                    self.dock.mobile = mobile;
                }
                if !input_changed {
                    self.dock.maximized_stack = maximized;
                }
            }
            if source_changed || input_changed {
                self.dock.reconcile_app_windows(&session.app);
            }
            if input_changed {
                self.dock.maximized_stack = None;
            }
            let roster = Self::session_window_instances(session, &self.dock).into_iter().map(|instance| instance.id).collect();
            self.dock_input_identity = Some(DockInputIdentity { source: incoming_layout.cloned(), mode_layout, roster });
            if let Some(seed) = dock_seed_active_window_id_v1(&mode_layout_stacks_v1(&self.dock.root), self.active_window_id.as_deref()) {
                self.active_window_id = Some(seed);
            }
            if let Some(id) = &self.active_window_id {
                self.dock.sync_active_window(id);
            }
            let live = self.dock.collect_window_ids();
            if owner_changed {
                self.world_projection_template.clear();
            }
            let retired_projection_documents: Vec<String> = self
                .window_projection_documents
                .keys()
                .filter(|window_id| owner_changed || !live.contains(*window_id))
                .cloned()
                .collect();
            for window_id in retired_projection_documents {
                let previous = self.window_projection_documents.remove(&window_id);
                if let Err(error) = self.retire_one_surface_document(previous) {
                    self.error = Some(error);
                }
            }
            self.world_projection_template.retain(|window_id, _| live.contains(window_id));
            self.window_icon_overrides.retain(|window_id, icon| icon.plugin_id == owner.0 && icon.app_id == owner.2 && icon.app_instance_id == owner.1 && live.contains(window_id));
            self.window_title_overrides.retain(|window_id, title| title.plugin_id == owner.0 && title.app_id == owner.2 && title.app_instance_id == owner.1 && live.contains(window_id));
            for window_id in live {
                if let Some(template_id) = self.dock.window_template_id(&window_id) {
                    self.world_projection_template.entry(window_id).or_insert_with(|| template_id.to_string());
                }
            }
            self.dock_instance_owner = Some(owner);
        } else {
            self.world_projection_template.clear();
            self.window_icon_overrides.clear();
            self.window_title_overrides.clear();
            self.dock_instance_owner = None;
            self.dock_input_identity = None;
        }
    }

    /// 🖼️ Applies React's renderer-local `SetWindowIconContext` command only to an exact live dock
    /// instance. The owner token prevents a late command from one app instance mutating its successor,
    /// while the shared view-context capacity bounds both entry count and identifier size.
    pub(crate) fn apply_window_icon_host_command(&mut self, window_id: &str, icon_id: &str) -> bool {
        let Some(session) = self.session.as_ref() else { return false };
        if self.dock_instance_owner.as_ref().is_none_or(|(plugin_id, instance_id, app_id)| plugin_id != &session.plugin_id || *instance_id != session.instance_id || app_id != &session.app.id)
            || self.dock.window_kind_id(window_id).is_none()
            || icon_id.is_empty()
            || icon_id.chars().count() > semio_framework::VIEW_CONTEXT_IDENTIFIER_CHARS
            || (!self.window_icon_overrides.contains_key(window_id) && self.window_icon_overrides.len() >= semio_framework::VIEW_CONTEXT_WINDOW_INSTANCES)
        {
            return false;
        }
        self.window_icon_overrides.insert(window_id.to_string(), WindowIconOverride { plugin_id: session.plugin_id.clone(), app_id: session.app.id.clone(), app_instance_id: session.instance_id, icon_id: icon_id.to_string() });
        true
    }

    pub(crate) fn window_icon_override(&self, window_id: &str) -> Option<&str> {
        let session = self.session.as_ref()?;
        self.window_icon_overrides.get(window_id).filter(|icon| icon.plugin_id == session.plugin_id && icon.app_id == session.app.id && icon.app_instance_id == session.instance_id).map(|icon| icon.icon_id.as_str())
    }

    /// 🏷️ Applies an explicit title to one live pane within its current session owner.
    pub(crate) fn apply_window_title_host_command(&mut self, window_id: &str, title: &str) -> bool {
        let Some(session) = self.session.as_ref() else { return false };
        if self.dock_instance_owner.as_ref().is_none_or(|(plugin_id, instance_id, app_id)| plugin_id != &session.plugin_id || *instance_id != session.instance_id || app_id != &session.app.id)
            || self.dock.window_kind_id(window_id).is_none()
            || (!self.window_title_overrides.contains_key(window_id) && self.window_title_overrides.len() >= semio_framework::VIEW_CONTEXT_WINDOW_INSTANCES)
        {
            return false;
        }
        self.window_title_overrides.insert(window_id.to_string(), WindowTitleOverride { plugin_id: session.plugin_id.clone(), app_id: session.app.id.clone(), app_instance_id: session.instance_id, title: title.to_string() });
        true
    }

    pub(crate) fn window_title_override(&self, window_id: &str) -> Option<&str> {
        let session = self.session.as_ref()?;
        self.window_title_overrides.get(window_id).filter(|title| title.plugin_id == session.plugin_id && title.app_id == session.app.id && title.app_instance_id == session.instance_id).map(|title| title.title.as_str())
    }

    /// 🪟️ The window roster a `ViewModel` carries — the Rust twin of React's `sessionWindowInstances`
    /// (`🛠️ShellHelpers/🟦️.tsx:4927`): EVERY window kind the open app declares, then the live dock
    /// instances that are not one of those kinds (a spawned or duplicated pane).
    ///
    /// 🩸️ It used to be the dock alone. `focused_window_id` is seeded from `app.window_kinds.first()`
    /// and a mode layout need not place that kind at all, so in generate mode the roster held only
    /// `generation3d-{generations,generate-form,generate-preview}` while the focus named
    /// `procedural-main` — and `WindowConfigOwnerRegistry::capture` (`🔌️plugin/🪟️window/🎚️config/🦀️.rs:541`)
    /// answers a roster miss with `window-config.window-context`. That fault aborted the whole
    /// `setContributions` crossing, so the guest's flow-extension registry stayed EMPTY and every
    /// generate-mode preview faulted `flow.extension-not-contributed {"extensionId":"brep"}` with
    /// `meshesHead="[]"` forever — measured verbatim on 6118
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-journey-probe-2026-09-13.md` §3).
    pub(crate) fn session_window_instances(session: &ActiveSession, dock: &DockState) -> Vec<semio_framework::ViewWindowInstance> {
        let mut instances: Vec<semio_framework::ViewWindowInstance> = session.app.window_kinds.iter().map(|kind| semio_framework::ViewWindowInstance { id: kind.id.clone(), window_kind_id: kind.id.clone() }).collect();
        for (id, window_kind_id) in dock.window_instances() {
            if instances.iter().any(|instance| instance.id == id) || !session.app.window_kinds.iter().any(|kind| kind.id == window_kind_id) {
                continue;
            }
            instances.push(semio_framework::ViewWindowInstance { id, window_kind_id });
        }
        instances
    }

    /// 🪟️ The host-owned fields every refresh restamps onto the session's view state — including the tree windows this shell's
    /// observer measured (`tree_windows`/`tree_viewport_rows`, [`tree_windows::TreeWindowScheduler::view_state_fields`]), the
    /// same pair React's `TreeWindowHostV1::viewStateFields` stamps (`🛠️ShellHelpers/🟦️.tsx`).
    pub(crate) fn live_view_state(&self, session: &ActiveSession) -> ViewModel {
        let mut view_state = session.view_state.clone();
        view_state.locale = self.active_locale();
        view_state.terminology = self.active_terminology();
        let spawned = self.spawned_session.as_ref().is_some_and(|owner| Self::same_session(owner, session));
        if !spawned {
            view_state.window_instances = Self::session_window_instances(session, &self.dock);
        }
        view_state.active_utility_by_window_id = self.active_utility_by_window.clone();
        view_state.tool_run_trace_cursor_by_window_id = infinite_world::world::world3d_tool_run_trace_cursors(self.world3d_states.values());
        view_state.tool_run_trace_cursor_by_window_id.extend(crate::engine_canvas::board2d_tool_run_trace_cursors());
        if !spawned {
            view_state.focused_window_id = self.active_window_id.clone();
        }
        view_state.session_identity = self.session_identity_view();
        (view_state.tree_windows, view_state.tree_viewport_rows) = self.tree_windows.view_state_fields();
        view_state
    }

    /// 🪪️ The session identity a `ViewModel` carries. ONE accessor on both targets since ticket
    /// 26/09/17/WGPU-RENDERER-REACT-PARITY packet W1e gave the browser build a real identity of its
    /// own (`poll_browser_identity`); it used to be a cfg pair whose browser half answered a hard
    /// `None`, so every guest saw an anonymous session on the browser renderer alone.
    fn session_identity_view(&self) -> Option<ViewSessionIdentity> {
        self.identity.as_ref().map(|identity| ViewSessionIdentity { user_id: identity.user_id.clone(), display_name: identity.display_name.clone() })
    }

    pub(crate) fn live_window_kind_id<'a>(&'a self, session: &'a ActiveSession, window_id: &str) -> Option<&'a str> {
        self.dock.window_kind_id(window_id).or_else(|| session.app.window_kinds.iter().find(|kind| kind.id == window_id).map(|kind| kind.id.as_str()))
    }

    pub(crate) fn context_window_instance_id(&self, x: f32, y: f32) -> Option<&str> {
        self.presented_input_geometry.dock_drop_bodies.iter().find(|(_, bounds, _)| bounds.contains(x, y)).map(|(_, _, window_id)| window_id.as_str())
    }

    pub(crate) fn persist_dock_layout(&mut self) {
        self.layout_override = Some(self.dock.to_window_layout());
    }

    pub(crate) fn allocate_window_instance_id(&self, window_kind_id: &str) -> String {
        let taken: Vec<String> = self.dock.window_instances().into_iter().map(|(id, _)| id).collect();
        (2..).map(|index| format!("{window_kind_id}-{index}")).find(|candidate| !taken.contains(candidate)).expect("an unbounded counter always clears a finite dock roster")
    }

    /// 🎬️ Arms a dock drag without touching the committed tree — a drag is promoted only once the
    /// pointer clears [`DOCK_DRAG_PROMOTION_DISTANCE_SQ`], and what floats on the cursor from then on
    /// is `DockState::render_view`'s derivation, never an edit.
    ///
    /// 🩸️ This used to snapshot the whole `WindowLayout` because the very next pointer move REMOVED
    /// the dragged window from the committed tree; an abandoned drag then had to be undone by
    /// re-applying that snapshot, and any drop that refused left the shell one `sync_dock` away from
    /// losing the user's layout.
    pub(crate) fn claim_chrome_pointer(&mut self, pointer_id: ui_render::PointerId) -> bool {
        if self.chrome_pointer_owner.is_some_and(|owner| owner != pointer_id) {
            return false;
        }
        self.chrome_pointer_owner = Some(pointer_id);
        true
    }

    pub(crate) fn begin_pending_dock_drag(&mut self, pointer_id: ui_render::PointerId, payload: DockDragPayload, x: f32, y: f32) -> bool {
        if !self.claim_chrome_pointer(pointer_id) {
            return false;
        }
        self.pending_dock_drag = Some((payload, (x, y)));
        true
    }

    /// 🧳️ Bridges one semantic Tree transfer into the dock authority before retained routing takes
    /// ownership of the gesture. Both shell-painted and retained Tree hits use this one decoder.
    pub(crate) fn begin_window_template_drag_from_hit(&mut self, pointer_id: ui_render::PointerId, hit: &HitTarget<ActionDescriptor>, x: f32, y: f32) -> bool {
        let Some((item_id, drag_data)) = hit.control_id.as_deref().and_then(tree_hit_item_id).zip(hit.drag_data.as_ref()) else {
            return false;
        };
        let Some(payload) = decode_window_template_drag(drag_data) else {
            return false;
        };
        if !self.session.as_ref().is_some_and(|session| session.app.window_kinds.iter().any(|kind| kind.id == payload.window_kind_id)) {
            return false;
        }
        let window_id = self.allocate_window_instance_id(&payload.window_kind_id);
        let ghost_label = item_id.rsplit('.').next().unwrap_or(item_id).to_string();
        self.begin_pending_dock_drag(pointer_id, DockDragPayload { kind: DockDragKind::NewWindow, window_id, window_kind_id: payload.window_kind_id, template_id: payload.template_id, source_path: Vec::new(), tab_index: 0, ghost_label }, x, y)
    }

    /// 🌫️ React's `deactivateActiveWindow` (`🖱️ui/🧱️elements/🎨️Canvas/🟦️.tsx:1455-1472`), reached by
    /// pressing the canvas background or a resize gutter: the mode keeps its layout and simply has no
    /// focused window any more.
    pub(crate) fn deactivate_active_window(&mut self) {
        if !self.dock.deactivate_active_window() && self.active_window_id.is_none() {
            return;
        }
        self.active_window_id = None;
    }

    /// 🐢️ Declares dirt without taking a refresh pass — the ONE way a producer (a dispatched action's
    /// `InvocationResult::ui_scope`, a command's, an extension answer's, or a host-effect fold) says
    /// what it invalidated. The next [`Self::refresh_ui`] drains the union.
    pub(crate) fn owe_refresh(&mut self, scope: UiDirtyScope) {
        self.owed_refresh_scope = core::mem::replace(&mut self.owed_refresh_scope, UiDirtyScope::None).merged_with(scope);
    }

    /// 🪟️ A changed window-instance roster invalidates every guest body because the live view carries
    /// the complete roster and focused instance. The settle lane performs the refresh on a later
    /// frame, keeping pointer and command turns bounded while guaranteeing a newly mounted instance
    /// receives its first document and removed instances enter the normal retirement lane.
    pub(crate) fn owe_window_topology_refresh(&mut self) {
        self.window_topology_refresh_owed = true;
        self.owe_refresh(UiDirtyScope::Full);
        self.owe_settle();
    }

    pub(crate) fn window_topology_publication_has_credit(&self) -> bool {
        self.window_topology_publications.len() < ui_wgpu::wgpu::action::ACTION_QUEUE_ITEM_CAPACITY
    }

    /// 🪟️ Commits one already-admitted roster mutation and arms its required Full refresh.
    /// Physical transfers bring an action token reserved before the dock changes; direct Display
    /// opens intentionally carry no React journal.
    pub(crate) fn commit_window_topology_publication(&mut self, window_id: &str, body_key: &str, journal_token: Option<u64>) {
        debug_assert!(self.window_topology_publication_has_credit());
        self.window_topology_publications.push_back(WindowTopologyPublication { window_id: window_id.to_string(), body_key: body_key.to_string(), attempts: 0, journal_token });
        self.owe_window_topology_refresh();
    }

    /// 🧾️ Reserves and builds one physical transfer note before the dock tree can change.
    /// The returned token binds release to this exact owner even when later bodies publish first.
    pub(crate) fn reserve_window_topology_action(&mut self, action: ActionDescriptor) -> Result<u64, ui_wgpu::wgpu::BoundedActionFault> {
        match self.window_topology_actions.reserve(&action.controller_id, &action.action, ui_wgpu::wgpu::action::ACTION_ITEM_BYTE_CAPACITY) {
            Ok(mut reservation) => action.args.as_ref().map_or(Ok(()), |args| reservation.builder().value(None, args)).and_then(|()| reservation.publish()),
            Err(error) => Err(error),
        }?;
        let token = self.next_window_topology_action_token;
        self.next_window_topology_action_token = self.next_window_topology_action_token.wrapping_add(1).max(1);
        self.window_topology_action_tokens.push_back(token);
        Ok(token)
    }

    pub(crate) fn take_window_topology_action(&mut self, token: u64) -> Option<ui_wgpu::wgpu::BoundedAction> {
        let index = self.window_topology_action_tokens.iter().position(|candidate| *candidate == token)?;
        debug_assert_eq!(self.window_topology_action_tokens.remove(index), Some(token));
        self.window_topology_actions.remove_at(index)
    }

    fn discard_window_topology_journal(&mut self, publication: &WindowTopologyPublication) {
        if publication.journal_token.is_some_and(|token| self.take_window_topology_action(token).is_none()) {
            self.record_surface_fault(&publication.window_id, &publication.body_key, "window topology journal owner was absent".into());
        }
    }

    /// 📯️ Releases each transfer journal exactly once only after its required retained body exists.
    /// Each refresh visits the cohort present at entry once, so a refused body retains its own token
    /// without blocking a later ready body. The retry ceiling retires both owners with one fault.
    pub(crate) fn complete_window_topology_refresh(&mut self) {
        let live: Vec<String> = self.dock.window_instances().into_iter().map(|(window_id, _)| window_id).collect();
        let cohort = self.window_topology_publications.len();
        for _ in 0..cohort {
            let Some(mut publication) = self.window_topology_publications.pop_front() else {
                break;
            };
            if self.window_ui.contains_key(&publication.window_id) {
                if let Some(token) = publication.journal_token {
                    match self.take_window_topology_action(token) {
                        Some(action) => match action.into_descriptor() {
                            Ok(action) => {
                                self.deferred_actions.push(action);
                                self.window_topology_journal_dispatch_owed = true;
                            }
                            Err(error) => self.record_surface_fault(&publication.window_id, &publication.body_key, format!("window topology journal decode refused: {error:?}").into()),
                        },
                        None => self.record_surface_fault(&publication.window_id, &publication.body_key, "window topology journal owner was absent".into()),
                    }
                }
                continue;
            }
            if !live.iter().any(|window_id| window_id == &publication.window_id) {
                self.discard_window_topology_journal(&publication);
                continue;
            }
            publication.attempts = publication.attempts.saturating_add(1);
            if publication.attempts < WINDOW_TOPOLOGY_PUBLICATION_ATTEMPTS {
                self.window_topology_publications.push_back(publication);
                continue;
            }
            self.discard_window_topology_journal(&publication);
            self.record_surface_fault(&publication.window_id, &publication.body_key, format!("initial window body publication refused after {} attempts; topology journal retired without dispatch", WINDOW_TOPOLOGY_PUBLICATION_ATTEMPTS).into());
        }
        self.window_topology_refresh_owed = !self.window_topology_publications.is_empty();
        if self.window_topology_refresh_owed {
            self.owe_refresh(UiDirtyScope::Full);
            self.owe_settle();
        } else if !self.deferred_actions.is_empty() {
            self.owe_settle();
        }
    }

    /// 🧰️ The scope a host-effect pass owes ON TOP of what its dispatch declared — the Rust twin of
    /// `🛠️ShellHelpers/🟦️.tsx`'s `hostEffectRefreshScopeV1`, and unioned with the guest's own scope,
    /// never substituted for it.
    ///
    /// `SetPanel`/`LoadDocument` name no body of their own and the panel payload feeds every section,
    /// so the only honest answer for those is the widest one. `SetActiveUtility`/`SetActiveTool`
    /// rewrite a render input the guest reads back, so they earn every window body plus the
    /// host-derived sections — the defect measured on the puzzle 3d editor, where an armed utility's
    /// `UiDirtyScope::None` meant the armed body was never fetched at all
    /// (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B37).
    pub(crate) fn host_effect_earned_scope(&self, effects: &[semio_framework::kernel::Effect]) -> UiDirtyScope {
        use semio_framework::kernel::Effect;
        if effects.iter().any(|effect| matches!(effect, Effect::LoadDocument { .. })) {
            return UiDirtyScope::Full;
        }
        if !effects.iter().any(|effect| matches!(effect, Effect::SetActiveUtility { .. })) {
            return UiDirtyScope::None;
        }
        let Some(session) = self.session.as_ref() else { return UiDirtyScope::Full };
        UiDirtyScope::Partial { window_bodies: session.app.window_kinds.iter().map(|kind| kind.body_key.clone()).collect(), panel_bodies: Vec::new(), utilities: true, tools: true, engagements: false, measures: true, labels: false }
    }

    /// 🐢️ One refresh pass. `ask` is the [`UiDirtyScope`] the dispatch that caused it declared,
    /// unioned with everything owed since the last pass ([`Self::owe_refresh`]).
    ///
    /// **The scope is honoured**: a window body, a panel body, the engagements section and the
    /// measures section are re-rendered exactly when the union names them, over the same kernel
    /// predicates ([`UiDirtyScope::wants_window_body`] and siblings) whose TypeScript twins React's
    /// `🛠️ShellHelpers/🟦️.tsx` runs against the same fixture — one selection law per language, never
    /// one per renderer.
    ///
    /// ⚖️ Narrowing was measured as a product STALL before the settle pump existed, and the reason is
    /// worth keeping: a window body's render is also the guest CROSSING that funds the guest's own
    /// background evaluation (`render_with_document` submits a turn, and that turn is what pumps the
    /// guest's worker pool), so a refresh that skips every surface withdraws compute from a brep solve
    /// the UI has nothing to do with — 14 of 16 battery examples froze mid-solve, `box-fillet-preview`
    /// never converging in 180 s at 36 renders against 21.13 s at 96 with the scope forced to `Full`
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-dirty-scope-refresh-2026-09-14.md` §4).
    ///
    /// That funding is no longer this function's job. [`ShellSettlePump`] owns it: the frame loop
    /// takes one settle step per frame and, whenever nothing is armed and a producer still reports
    /// `computing`, spends exactly one crossing on that producer's own window body. The refresh is
    /// therefore free to render only what a settle actually dirtied, which on a converging edit is
    /// nothing at all for all seven `flowEvalTick` hops and the whole shell only for `toolRunStart`.
    ///
    /// ⚠️ A surface's retirement (`window_ui` remove → `retire_one_surface_document`) stays INSIDE the
    /// per-surface loop, never ahead of it, so a skipped surface keeps the exact document it already
    /// owns instead of being retired and never re-minted.
    pub async fn refresh_ui(&mut self, ask: UiDirtyScope) -> Result<(), String> {
        self.refresh_ui_rendered(ask, None).await
    }

    /// 🖼️ [`Self::refresh_ui`] with the guest bodies and reserved sections some of whose reads already
    /// happened off the shell: a surface or section found in `rendered` takes that document, every other
    /// wanted one is read here, and a read no longer wanted — or read for an instance that is no longer
    /// the session's — is handed back by its own `Drop`.
    ///
    /// 🧾️ The SHELL-owned leaves (the Command dock, the five Settings leaves, the Marketplace leaf,
    /// every `tool.<id>` leaf and the chat transcript) are not guest-published: they are retained
    /// beside the guest's own panel surfaces so `retire_documents_outside` never sweeps them.
    ///
    /// 🧰️ The utility bar is derived from the app's declared `AppDefinition.utilities` (scoped to the active
    /// window kind) via `ui_wgpu::wgpu::derive_utility_nodes` — the old per-call `plugin.utilities()` fetch and the
    /// `find_active_utility_id` "first pressed toggle" heuristic are gone (Architecture Decision 5).
    /// 🩸️ The `framework.sync.{file,folder,remote}` toggles used to be appended here and painted in
    /// the footer. React builds the same three leaves but hands them to the bottom-left `s-sync-status`
    /// tab's `SyncAttachCard` (`🏛️ShellHost/🟦️.tsx:9076`) — its footer never carries them.
    ///
    /// 🛍️ Deliberately unscoped: the GUEST fetch inside is already claimed to once per app
    /// instance, and what remains is the local republish onto every live node-graph engine host —
    /// the moment a surface minted by THIS pass gets its palette. Skipping that on a partial scope
    /// is how a spotlight silently opens empty.
    ///
    /// 🎬️ Deliberately UNSCOPED, like the shell-owned panel leaves above: these two bodies are the
    /// shell's own projection of state the user moves without any guest round trip (the expanded
    /// action, its staged buffer, the possibles chevron), so a scope that skipped them would leave
    /// a pressed row painting its previous frame forever. The ingress is revision-keyed, so an
    /// unchanged body still pays nothing.
    ///
    /// 🛠️ The TOOL half of the same reader — React's `toolMeasuresByToolId` ref, refreshed on the
    /// scope that names tools so a slider tick inside an armed tool's options repaints its leaf.
    pub(crate) async fn refresh_ui_rendered(&mut self, ask: UiDirtyScope, rendered: Option<ShellRenderedRefresh>) -> Result<(), String> {
        let mut latency = crate::frame_latency::FrameLatencyTimer::start(crate::frame_latency::latest_frame_authority(), crate::frame_latency::FrameLatencyStage::ShellRefresh, 1);
        let scope = core::mem::replace(&mut self.owed_refresh_scope, UiDirtyScope::None).merged_with(ask);
        let Some(session) = self.session.clone() else {
            return Ok(());
        };
        let mut read_ahead = rendered.filter(|rendered| rendered.instance_id == session.instance_id).unwrap_or_else(|| ShellRenderedRefresh::empty(session.instance_id));
        self.drain_retained_document_arenas();
        self.sync_dock();
        let view_state = self.live_view_state(&session);
        let live_windows = self.dock.window_instances();
        latency.set_work_items(live_windows.len());
        let measure_windows: Vec<String> = live_windows.iter().map(|(window_id, _)| window_id.clone()).collect();
        let mut refresh_effects = Vec::new();
        let mut faults: Vec<(String, String, crate::program_bridge::ProgramFault)> = Vec::new();
        let mut rendered = 0usize;
        let mut visited: Vec<String> = Vec::new();
        {
            let program = self.plugins.iter().find(|p| p.plugin_id == session.plugin_id).cloned().ok_or("session program missing")?;
            let retained: Vec<String> = live_windows.iter().map(|(window_id, _)| window_id.clone()).collect();
            self.retire_documents_outside(&retained, true)?;
            for (window_id, window_kind_id) in live_windows {
                let kind = session.app.window_kinds.iter().find(|kind| kind.id == window_kind_id).ok_or_else(|| format!("window kind '{}' is absent from the app", window_kind_id))?;
                if !scope.wants_window_body(&kind.body_key) {
                    continue;
                }
                let window_view = view_state.for_window_instance(&window_id).ok_or_else(|| format!("window '{}' is absent from the live view", window_id))?;
                let previous = self.window_ui.remove(&window_id);
                self.retire_one_surface_document(previous)?;
                rendered += 1;
                visited.push(window_id.clone());
                Self::debug_log(&format!("[TRACE] wgpu-shell render begin surface={window_id} body={}", kind.body_key));
                let render_started = Self::instant_now_ms();
                Self::declare_boot_subphase(&format!("shell-boot:render:{window_id}"), "enter", 0.0);
                let document = match read_ahead.surfaces.remove(&window_id) {
                    Some((document, mut effects)) => {
                        refresh_effects.append(&mut effects);
                        document
                    }
                    None => program.render_with_document(session.instance_id, &window_id, &kind.body_key, &window_view, None, Some(&mut refresh_effects)).await,
                };
                match document {
                    Ok(document) => {
                        self.window_ui.insert(window_id.clone(), document);
                    }
                    Err(error) => faults.push((window_id.clone(), kind.body_key.clone(), error.into())),
                }
                let elapsed = (Self::instant_now_ms() - render_started).max(0.0);
                Self::declare_boot_subphase(&format!("shell-boot:render:{window_id}"), "leave", elapsed);
                Self::debug_log(&format!("[TRACE] wgpu-shell render leave surface={window_id} {elapsed:.0} ms{}", Self::resident_census()));
            }
        }
        let program = self.plugins.iter().find(|p| p.plugin_id == session.plugin_id).cloned().ok_or("session program missing")?;
        if let Err(error) = self.refresh_app_document_identity(&program, session.instance_id).await {
            faults.push(("document-identity".into(), "document-identity".into(), error.into()));
        }
        let panel_view = view_state.for_panel();
        let panel_leaves: Vec<(String, String)> = Self::flatten_panel_tab_leaves(&session.app.panel_tabs).into_iter().filter_map(|tab| tab.body_key.as_deref().map(|body_key| (tab.id().to_string(), body_key.to_string()))).collect();
        let shell_leaves: Vec<String> = self.shell_owned_panel_leaves();
        let retained: Vec<String> = panel_leaves.iter().map(|(id, _)| id.clone()).chain(shell_leaves.iter().cloned()).collect();
        self.retire_documents_outside(&retained, false)?;
        for tab_id in shell_leaves {
            match self.publish_shell_panel_document(&tab_id) {
                Ok(Some(document)) => {
                    let previous = self.panel_documents.insert(tab_id.clone(), document);
                    self.retire_one_surface_document(previous)?;
                }
                Ok(None) => {}
                Err(error) => faults.push((tab_id.clone(), tab_id.clone(), error.into())),
            }
        }
        for (tab_id, body_key) in panel_leaves {
            if !scope.wants_panel_body(&body_key) {
                continue;
            }
            let previous = self.panel_documents.remove(&tab_id);
            self.retire_one_surface_document(previous)?;
            rendered += 1;
            visited.push(tab_id.clone());
            Self::debug_log(&format!("[TRACE] wgpu-shell render begin surface={tab_id} body={body_key}"));
            let render_started = Self::instant_now_ms();
            Self::declare_boot_subphase(&format!("shell-boot:render:{tab_id}"), "enter", 0.0);
            let document = match read_ahead.surfaces.remove(&tab_id) {
                Some((document, mut effects)) => {
                    refresh_effects.append(&mut effects);
                    document
                }
                None => program.render_with_document(session.instance_id, &tab_id, &body_key, &panel_view, None, Some(&mut refresh_effects)).await,
            };
            match document {
                Ok(document) => {
                    let reveal = tab_id == FRAMEWORK_PANEL_TAB_TOOL_RUN_ID && tool_run_panel_reveals(&mut self.tool_run_panel_runs, &document)?;
                    self.panel_documents.insert(tab_id.clone(), document);
                    if reveal {
                        self.chrome_tour_reveal_panel_tab(&session, &tab_id);
                    }
                }
                Err(error) => faults.push((tab_id.clone(), body_key.clone(), error.into())),
            }
            let elapsed = (Self::instant_now_ms() - render_started).max(0.0);
            Self::declare_boot_subphase(&format!("shell-boot:render:{tab_id}"), "leave", elapsed);
            Self::debug_log(&format!("[TRACE] wgpu-shell render leave surface={tab_id} {elapsed:.0} ms{}", Self::resident_census()));
        }
        self.active_utilities = self.derive_utility_nodes(&session);
        self.refresh_app_catalogue(&program, session.instance_id, &panel_view, read_ahead.catalogue.take()).await;
        if scope.wants_section(UiDirtySection::Engagements) {
            visited.push(semio_framework::UiRefreshSection::Engagements.body_key().to_string());
            self.refresh_window_engagements(&program, session.instance_id, &view_state, read_ahead.engagements.take(), &mut faults).await?;
        }
        visited.extend(measure_windows.iter().map(|window_id| window_actions_surface_id(window_id)));
        visited.extend(measure_windows.iter().map(|window_id| window_search_surface_id(window_id)));
        self.refresh_window_action_panes(&measure_windows, &mut faults)?;
        if scope.wants_section(UiDirtySection::Measures) {
            visited.push(semio_framework::UiRefreshSection::Measures.body_key().to_string());
            visited.extend(measure_windows.iter().map(|window_id| window_measures_surface_id(window_id)));
            self.refresh_window_measures(&program, session.instance_id, &view_state, read_ahead.measures.take(), &measure_windows, &mut faults).await?;
        }
        if scope.wants_section(UiDirtySection::Tools) && !self.tool_panel_tabs().is_empty() {
            visited.push(semio_framework::UiRefreshSection::Tools.body_key().to_string());
            self.refresh_tool_measures(&program, session.instance_id, &view_state, read_ahead.tools.take(), &mut faults).await?;
        }
        let previous_spawned_session = self.spawned_session.clone();
        if let Some(document) = self.spawned_ui.take() {
            if let Err(document) = self.retain_document_for_close(document) {
                self.spawned_ui = Some(document);
                return Err("shell: spawned document retirement registry refused the exact prior owner".to_string());
            }
        }
        self.spawned_session = None;
        if self.space_mode {
            if let Some(panel) = Self::panel_state_from_view(&session.view_state)? {
                if let Some(spawned) = panel.active_spawned_id.as_ref().and_then(|id| panel.spawned_apps.iter().find(|app| &app.id == id)) {
                    if let Some(spawn_plugin) = self.plugins.iter().find(|p| p.plugin_id == spawned.plugin_id).cloned() {
                        let spawned_app = spawn_plugin.manifest.apps.iter().find(|app| app.id == spawned.app_id).cloned();
                        if let Some(app) = spawned_app {
                            if let Err(error) = self.refresh_app_document_identity(&spawn_plugin, spawned.instance_id).await {
                                faults.push((spawned.id.clone(), "document-identity".into(), error.into()));
                            }
                            let body_key = app.window_kinds.first().body_key.clone();
                            let fresh_view = ViewModel {
                                active_mode_id: Some(app.default_mode_id.clone()),
                                active_window_kind_id: Some(app.window_kinds.first().id.clone()),
                                active_utility_id: None,
                                panel_json: None,
                                extension_input_json: None,
                                session_identity: self.session_identity_view(),
                                locale: self.active_locale(),
                                terminology: self.active_terminology(),
                                window_id: Some(spawned.id.clone()),
                                focused_window_id: Some(spawned.id.clone()),
                                window_instances: vec![semio_framework::ViewWindowInstance { id: spawned.id.clone(), window_kind_id: app.window_kinds.first().id.clone() }],
                                active_tool_id: None,
                                active_utility_by_window_id: Default::default(),
                                tool_run_trace_cursor_by_window_id: Default::default(),
                                tree_windows: Vec::new(),
                                tree_viewport_rows: None,
                            };
                            let mut view_state = previous_spawned_session.as_ref().filter(|owner| owner.plugin_id == spawned.plugin_id && owner.instance_id == spawned.instance_id && owner.app.id == app.id).map(|owner| owner.view_state.clone()).unwrap_or(fresh_view);
                            view_state.locale = self.active_locale();
                            view_state.terminology = self.active_terminology();
                            view_state.session_identity = self.session_identity_view();
                            visited.push(spawned.id.clone());
                            match spawn_plugin.render(spawned.instance_id, &spawned.id, &body_key, &view_state).await {
                                Ok(document) => {
                                    self.spawned_ui = Some(document);
                                    self.spawned_session = Some(ActiveSession { plugin_id: spawned.plugin_id.clone(), instance_id: spawned.instance_id, app, view_state });
                                }
                                Err(error) => faults.push((spawned.id.clone(), body_key.clone(), error.into())),
                            }
                        }
                    }
                }
            }
        }
        let earned = self.host_effect_earned_scope(&refresh_effects);
        self.queue_host_effects(&session.app.controller_id, refresh_effects);
        self.owe_refresh(earned);
        self.settle_surface_faults(faults, &visited);
        if self.window_topology_refresh_owed {
            self.complete_window_topology_refresh();
        }
        Self::debug_log(&format!("[TRACE] wgpu-shell refresh scope={} rendered={rendered}", refresh_scope_label(&scope)));
        Ok(())
    }

    /// 🧯 Replaces the per-surface fault set FOR THE SURFACES THIS PASS VISITED, so a surface that
    /// recovered stops showing a card and one that faulted keeps showing the same typed card. The
    /// refresh itself always succeeds: an unreadable surface is that surface's failure, never the
    /// renderer's — see [`ShellSurfaceFault`].
    ///
    /// ⚠️ `visited`, not `clear()`: a scoped pass re-renders only what its `UiDirtyScope` named, and
    /// clearing the whole set would silently dismiss the fault card of every surface it never looked
    /// at — a broken body that repairs itself the moment an unrelated window settles.
    pub(crate) fn settle_surface_faults(&mut self, faults: Vec<(String, String, crate::program_bridge::ProgramFault)>, visited: &[String]) {
        self.surface_faults.retain(|fault| !visited.contains(&fault.surface_id));
        for (surface_id, body_key, detail) in faults {
            self.record_surface_fault(&surface_id, &body_key, detail);
        }
        self.error = self.fault_status();
    }

    /// 🛍️ Fetches the app-static operator/palette catalogue once per app instance and pushes it into
    /// every live node-graph engine host, so the canvas spotlight and the full palette have the same
    /// operator set the React shell threads through `AppCatalogueContext`.
    ///
    /// The catalogue rides the reserved `framework.section.catalogue` retained surface, which is the
    /// ordinary `SurfaceVisible` → `AdvanceRetained` protocol every window body already uses — no
    /// second channel, and no `UiDirtyScope` concept the native shell does not have. A fetch that
    /// fails leaves the previous catalogue in place rather than blanking the palette.
    ///
    /// ⚠️ The instance is marked ATTEMPTED before the fetch, not after a successful one: the refresh
    /// that calls this runs on every settled command, so recording only successes turned one failed
    /// catalogue into a re-fetch per refresh. The second such fetch never returned from its own
    /// `submit_turn`, which wedged `boot_shell` itself and took every later pointer event with it
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-input-hit-runtime-2026-09-13.md`). "Once
    /// per app instance" is what the first line of this doc promises; the failure path now keeps it.
    async fn refresh_app_catalogue(&mut self, program: &ProgramBridgeEntry, instance_id: u32, view_state: &ViewModel, read_ahead: Option<Result<UiDocumentLease, crate::program_bridge::ProgramFault>>) {
        let body_key = semio_framework::UiRefreshSection::Catalogue.body_key();
        let fetched = match read_ahead {
            Some(fetched) => fetched,
            None if claim_app_catalogue_fetch(&mut self.app_catalogue_instance, instance_id) => program.render_with_document(instance_id, body_key, body_key, view_state, None, None).await,
            None => {
                self.publish_app_catalogue();
                return;
            }
        };
        let document = match fetched {
            Ok(document) => document,
            Err(error) => {
                let detail = self.note_refused_open(error);
                Self::debug_log(&format!("[TRACE] wgpu shell app catalogue fetch failed: {detail}"));
                return;
            }
        };
        let payload = document.read_paged_text();
        if let Err(document) = self.retain_document_for_close(document) {
            drop(document);
        }
        match payload {
            Ok(payload) => {
                Self::debug_log(&format!("[TRACE] wgpu shell app catalogue ready bytes={}", payload.len()));
                self.app_catalogue_json = payload;
                self.publish_app_catalogue();
            }
            Err(error) => Self::debug_log(&format!("[TRACE] wgpu shell app catalogue reassembly failed: {error:?}")),
        }
    }
}
//#endregion ShellLifecycle
