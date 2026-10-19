//! 🐙️ `shell-state` staging of `session-and-topology`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellLifecycle
impl ShellState {
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
        semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::install_ui_engine_locale(locale);
        let host_capability = resolve_plugin_host_config(&plugin_filter);
        let space_mode = host_capability.is_some();
        #[cfg(not(target_arch = "wasm32"))]
        let (directory_transport, directory_cancel): (ShellDirectoryTransport, CancelToken) = {
            const DIRECTORY_COMPUTE_CAPACITY: u32 = 4;
            let pool = semio_framework_os_renderer_wgpu_host_base::renderer_worker_pool();
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
            let host = ArtifactHost::new(std::sync::Arc::new(semio_framework_os_renderer_wgpu_host_base::renderer_worker_pool()));
            if let Some(credential) = claimed_local_hub_credential("native") {
                host.set_local_hub_credential(credential);
            }
            host
        };
        #[cfg(target_arch = "wasm32")]
        let document_host = {
            let host = ArtifactHost::new(std::sync::Arc::new(semio_framework_os_renderer_wgpu_host_base::renderer_worker_pool()));
            host.set_document_socket_dialer(std::sync::Arc::new(semio_framework_os_renderer_wgpu_doors::socket_door::browser::DoorDocumentSocketDialer));
            host
        };
        let mut state = Self {
            chrome: ChromeState {
                error: None,
                screen_w: 1280.0,
                screen_h: 720.0,
                fullscreen_toggle_requested: false,
                fullscreen_active: false,
                presented_chrome_accessibility: Vec::new(),
                presented_chrome_accessibility_generation: 0,
                chrome_floor_scope: None,
                dispatch_origin: SHELL_DISPATCH_ORIGIN_USER,
                chrome_build: ShellChromeBuildState::default(),
                chrome_present: ShellChromePresentState::default(),
            },
            surfaces: SurfaceState {
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
                tiled_map_states: AdmittedSurfaceMap::default(),
                icon_render_states: HashMap::new(),
                board2d_states: AdmittedSurfaceMap::default(),
            },
            tutorial: TutorialState {
                tutorial: None,
                tutorial_pending_document_ops: Vec::new(),
            },
            settle: SettleState {
                deferred_actions: Vec::new(),
                settling: false,
                owed_refresh_scope: UiDirtyScope::None,
                settle_pump: ShellSettlePump::default(),
                #[cfg(not(target_arch = "wasm32"))]
                shell_io_pending: std::collections::VecDeque::new(),
            },
            plugins: PluginState {
                plugins,
                extensions: Vec::new(),
                plugin_filter,
                host_capability,
                space_mode,
                tool_run_panel_runs: std::collections::BTreeSet::new(),
                plugin_faults: Vec::new(),
                surface_faults: Vec::new(),
                app_catalogue_json: String::new(),
                app_catalogue_instance: None,
                active_example_id: None,
                #[cfg(target_arch = "wasm32")]
                pending_extension_invocations: Vec::new(),
                default_app_pins: HashMap::new(),
                plugin_install: None,
            },
            history: HistoryState {
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
                local_folder_direct_reattach: LOCAL_FOLDER_DIRECT_REATTACH,
                host_route_folder: None,
            },
            prefs: PrefsState {
                appearance_id: "system".into(),
                locale_id: locale.as_str().into(),
                terminology_id: terminology.as_str().into(),
                driver_id: "default".into(),
                driver_draft: None,
                driver_save_label: String::new(),
                theme_draft: None,
                theme_save_label: String::new(),
                merge_policy: SHELL_DEFAULT_MERGE_POLICY.to_string(),
                pending_theme_import: false,
                open_conflicts: Vec::new(),
                selected_conflict_id: None,
                conflicts_seeded: false,
            },
            directory: DirectoryState {
                agent_chat_draft: String::new(),
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
                agent_bridge: semio_framework_os_renderer_wgpu_agent_elements::agent_bridge_door::AgentBridgeTransport::new(),
                agent_bridge_offer_scope: AgentBridgeOfferScopeOwnerV1::default(),
                #[cfg(not(target_arch = "wasm32"))]
                inference_port: None,
                #[cfg(not(target_arch = "wasm32"))]
                inference_port_status: None,
                #[cfg(not(target_arch = "wasm32"))]
                directory_home: None,
                directory_transport,
                directory_cancel,
                directory_commands: ShellDirectoryCommandQueueV1::default(),
                hub_workspace: semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::HubWorkspaceState::new(semio_framework_os_renderer_wgpu_hub_contracts::hub_sign_in::parse_hub_connection_book(prefs_get(HUB_CONNECTION_BOOK_STORAGE_KEY_V1).as_deref(), &shell_hub_bootstrap_origin(), semio_framework_ui_locale::Locale::En)),
                hub_sign_in_task: None,
                hub_spaces_task: None,
                hub_workspace_open: false,
                space_administration: None,
                space_administration_epoch: 0,
                presence_peers: Vec::new(),
                presence_surface: None,
                presence_self: None,
                presence_pointer: None,
            },
            sync: SyncState {
                sync_backbone_uri: None,
                sync_card_kind: None,
                sync_card_draft: String::new(),
                sync_card_anchor: None,
                document_host,
                sync_channel: None,
                next_sync_binding_generation: 1,
                sync_terminal_fault: None,
                sync_status: None,
                sync_link: DocumentLinkStatus::Linked,
                hub_documents: BTreeMap::new(),
                sync_bootstrap_progress: None,
                sync_reseed: None,
                last_uncommitted_edit_at_ms: None,
                auto_checkin_pending: false,
                checkpoint_dispatched: false,
                checkin_dialog_draft: None,
                #[cfg(not(target_arch = "wasm32"))]
                hub_check_in: None,
            },
            docs: DocumentState {
                session: None,
                window_ui: HashMap::new(),
                panel_documents: HashMap::new(),
                spawned_ui: None,
                spawned_session: None,
                app_document_identities: semio_framework_os_renderer_wgpu_media_slots::DocumentIdentityRegistry::default(),
                closing_documents: ShellDocumentRetirementRegistry::default(),
                window_actions_documents: HashMap::new(),
                window_search_documents: HashMap::new(),
                window_projection_documents: HashMap::new(),
                window_measures_documents: HashMap::new(),
            },
            opening: OpeningState {
                uri_history: vec!["/".into()],
                uri_index: 0,
                open_space_id: None,
                pending_shell_uri_apply: false,
                #[cfg(target_arch = "wasm32")]
                pending_file_opens: Vec::new(),
                import_transfers: VecDeque::new(),
                import_transfer_next: 1,
                last_envelope_dsl: None,
                #[cfg(not(target_arch = "wasm32"))]
                document_services: Vec::new(),
                document_opening: None,
                icon_export: None,
                document_execution_target_lease: None,
            },
            windows: WindowState {
                active_window_id: None,
                noted_active_window: None,
                host_event_active_window: None,
                engagement_expanded: HashMap::new(),
                engagement_activated: HashMap::new(),
                measures_folded: HashMap::new(),
                measures_expanded: HashMap::new(),
                measures_width: HashMap::new(),
                measures_resize_origin_width: 0.0,
                engagement_inputs: HashMap::new(),
                measures_resize_window_id: None,
                window_topology_actions: semio_framework_ui_action::BoundedActionQueue::default(),
                window_topology_publications: VecDeque::with_capacity(semio_framework_ui_action::ACTION_QUEUE_ITEM_CAPACITY),
                window_topology_action_tokens: VecDeque::with_capacity(semio_framework_ui_action::ACTION_QUEUE_ITEM_CAPACITY),
                next_window_topology_action_token: 1,
                window_topology_refresh_owed: false,
                window_topology_journal_dispatch_owed: false,
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
                tool_measures: HashMap::new(),
                window_engagements: HashMap::new(),
                window_measures_minted: HashMap::new(),
                utility_collection_expanded: HashMap::new(),
                contributor_instances: HashMap::new(),
            },
            dock: ShellDockState {
                panel_anchors: Default::default(),
                dock_tabs: ShellDock::default(),
                dock_override: None,
                dock_collapsed_branches: HashMap::new(),
                dock_tab_drag: None,
                dock_loaded_app_id: None,
                panel_resize_origin_width: DEFAULT_PANEL_WIDTH_PX,
                dock: DockState::default(),
                split_resize_path: None,
                split_resize_index: 0,
                split_resize_axis_total: 1.0,
                dock_drag: None,
                pending_dock_drag: None,
                dock_view: DockState::default(),
                dock_instance_owner: None,
                dock_input_identity: None,
                tree_windows: TreeWindowScheduler::default(),
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
                mobile_panel_visible: false,
                mobile_panel_path: Vec::new(),
                layout_save_label: String::new(),
                user_layouts: Vec::new(),
                window_content_rects: HashMap::new(),
                window_silhouettes: HashMap::new(),
            },
            input: ShellInputState {
                scroll_offsets: HashMap::new(),
                overlay_state: OverlayState::None,
                collapsed_sections: HashMap::new(),
                tree_open_states: BTreeMap::new(),
                accessibility_focused_control_id: None,
                interaction_selection: HashMap::new(),
                open_selects: HashMap::new(),
                context_menu: None,
                right_click: RightClickState::default(),
                tree_drag: None,
                tree_hovered_id: None,
                widget_maps: WidgetInteractionMaps::default(),
                widget_maps_staging: WidgetInteractionMaps::default(),
                pending_tree_drag: None,
                tree_drag_origin: (0.0, 0.0),
                pending_chrome_release: None,
                chrome_pointer_owner: None,
                keybinding_capture_control_id: None,
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
                pane_overlay_hits: Vec::new(),
                retained_hover_window: None,
            },
            commands: CommandState {
                search_open: false,
                find_open: false,
                search_query: String::new(),
                search_selected: 0,
                find_query: String::new(),
                find_items: ShellFindItems::default(),
                find_selected: 0,
                staged_action_args: HashMap::new(),
                expanded_command_id: None,
                staged_command_args: HashMap::new(),
                search_possibles_open: HashMap::new(),
            },
        };
        state
    }
}

//#endregion ShellLifecycle

