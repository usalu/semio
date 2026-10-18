//! 🖼️ Render surface of the mounted artifact instance: the render of a body key, the window engagements and measures, the pending effects poll and the context menu funnel. See [`super`].

use super::*;

impl<A: ArtifactApp, M: SpaceMember + MemberFactory + Send + 'static> VcsArtifactApp<A, M> {
    /// 🕰️ Framework-owned, snapshot-independent — served before any app body-key match.
    ///
    /// 🕹️ Task 5: materialized once, before either branch, then used to stamp EVERY
    /// `interaction_domain`-bound `UiTree` this render produces — see `stamp_and_cache_interaction_ui`.
    ///
    /// 🕹️ Materialized owned BEFORE the field-wise destructure below (same reasoning as
    /// `stamp_and_cache_interaction_ui`'s own comment): `interaction_hover`/`peer_presence`
    /// are only ever READ here, so a snapshot avoids an aliasing conflict against `&mut self`.
    pub(super) async fn render(&mut self, body_key: &str, snapshot_override_json: Option<&str>, view_state: &ViewModel,cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<()>,semio_framework_job::OriginalTurnFailure> {
        if self.pending.receive_render_failure(cx)?||cx.should_yield(){return Ok(None)}
        if self.pending.pending_render_tree.is_none()&&self.pending.pending_render_tree_close.is_none()&&self.pending.pending_render_tree_ready{self.pending.pending_render_tree_identity=None;self.pending.pending_render_tree_ready=false;}
        if let Some(identity)=self.pending.pending_render_tree_identity.as_mut(){identity.validate(cx)?;}else{self.pending.pending_render_tree_identity=Some(OriginalRenderTreeIdentity::capture(cx)?);}
        if self.pending.pending_render_tree.is_some(){return Ok(self.pending.pending_render_tree_ready.then_some(()));}
        let original_render_result:crate::OriginalRenderResult<()>=async {
        if let Some(section)=semio_framework::UiRefreshSection::from_body_key(body_key){
            let tree=semio_framework_plugin_reactor_app::original_render_section(self,section,view_state)?;
            self.pending.retain_pending_render_tree(tree);
            self.pending.pending_render_tree_ready=true;
            return Ok(Some(()));
        }
        self.render_ctx().refresh_cache().await?;
        if body_key == FRAMEWORK_HISTORY_BODY_KEY {
            let Some((_, _, _, history)) = self.history.cache.as_ref() else {
                return Err(plugin_sdk_fault("render cache unavailable after refresh").into());
            };
            let mut time_travel = self.time_travel.panel();
            if let Some(panel) = time_travel.as_mut() {
                self.time_travel_ref().resolve_time_travel_snaps(panel, view_state);
                self.time_travel_ref().resolve_time_travel_options(panel);
                self.time_travel_ref().resolve_time_travel_reference_labels(panel);
            }
            let reprojection = self.render_ref().reprojection_status();
            let pages = self.child_composition.history_mutation_pages(&self.stores, history, time_travel.as_ref(), view_state);
            let root =
                ui_history_panel(history, time_travel.as_ref(), reprojection.as_ref(), &pages, &self.tools.registry.controller_id, view_state.locale, A::ROLE == AppRole::Viewer, self.time_travel_ref().time_travel_busy(), view_state).await?;
            self.pending.retain_pending_render_tree(built_to_component_tree(root));
            self.pending.pending_render_tree_ready=true;
            return Ok(Some(()));
        }
        if let Some(root) = InstanceExtensions::render_body(&mut self.extension_ctx(), body_key, view_state) {
            self.pending.retain_pending_render_tree(built_to_component_tree(root?));
            self.pending.pending_render_tree_ready=true;
            return Ok(Some(()));
        }
        let interaction_state = self.interaction.interaction_state(&self.stores).await;
        let interaction_hover = self.interaction.interaction_hover.clone();
        let interaction_peers = std::sync::Arc::clone(&self.roster.peer_presence);
        let draft_references = self.time_travel.draft_references();
        let interaction = InteractionView { state: &interaction_state, hover: &interaction_hover, peers: interaction_peers.as_ref(), draft_references: &draft_references };
        let window_config = self.stores.window_config_store.capture(Some(view_state)).await?;
        let window_transient = self.stores.window_transient_store.capture(Some(view_state))?;
        if let Some(json) = snapshot_override_json {
            let snapshot: A::Snapshot = semio_framework_pack_json::from_json_str(json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error|plugin_sdk_fault(error.to_string()))?;
            let history = std::sync::Arc::clone(&self.history.cache.as_ref().ok_or_else(|| plugin_sdk_fault("render cache unavailable after refresh"))?.3);
            let doc = ArtifactView::new(&snapshot, history.as_ref()).with_instance(self.tools.instance_operation_owner.clone());
            let config = self.stores.config_store.snapshot().unwrap_or_else(|_| A::Config::default());
            let cfg = ConfigView { snapshot: &config, window: window_config.as_ref().map(|authority| &authority.snapshot) };
            let transient = self.stores.transient_store.current_root();
            let transient = TransientView { snapshot: transient.as_ref(), window: window_transient.as_ref().map(|authority| &authority.snapshot) };
            let Some(node) = A::render_with_request_context(&self.tools.instance_operation_owner, body_key, &doc, &cfg, view_state, &transient, &interaction,cx).await? else{return Ok(None)};
            self.pending.retain_pending_render_tree(node);
            self.render_ctx().stamp_pending_render_interaction_ui(&interaction_state, body_key).await?;
            self.pending.pending_render_tree_ready=true;
            return Ok(Some(()));
        }
        let operation_progress = self.tools.operation_progress_snapshot();
        let render_operation = self.tools.live_render_operation(&self.mounted_policy, &self.stores);
        let parent_document_id = self.stores.store.envelope().id.clone();
        let Some(node) = ({
            let VcsArtifactApp { app: _, cache, child_content_root, tool_runs, time_travel, tool_machines, .. } = self;
            let Some((_, snapshot, config, history)) = cache.as_ref() else {
                return Err(plugin_sdk_fault("render cache unavailable after refresh").into());
            };
            let doc = ArtifactView::with_render_context(
                time_travel.render_snapshot_or(tool_runs, tool_machines.overlay_or(snapshot)).as_ref(),
                history.as_ref(),
                ChildContentView::clone(time_travel.render_children_or(tool_runs, child_content_root)),
                render_operation,
                parent_document_id,
                None,
            )
            .await
            .with_tool_run(tool_runs.view_for(view_state.window_id.as_deref()))
            .with_instance(self.tools.instance_operation_owner.clone())
            .with_operations(&operation_progress);
            let cfg = ConfigView { snapshot: tool_machines.config_overlay_or(config).as_ref(), window: window_config.as_ref().map(|authority| tool_machines.window_config_overlay_or(&authority.snapshot)) };
            let transient = self.stores.transient_store.current_root();
            let transient = TransientView { snapshot: transient.as_ref(), window: window_transient.as_ref().map(|authority| &authority.snapshot) };
            A::render_with_request_context(&self.tools.instance_operation_owner, body_key, &doc, &cfg, view_state, &transient, &interaction,cx).await?
        }) else{return Ok(None)};
        self.pending.retain_pending_render_tree(node);
        InstanceExtensions::after_app_render(&mut self.extension_ctx(), body_key, view_state)?;
        self.render_ctx().stamp_pending_render_interaction_ui(&interaction_state, body_key).await?;
        self.pending.pending_render_tree_ready=true;
        Ok(Some(()))
        }.await;
        match original_render_result{Ok(tree)=>Ok(tree),Err(semio_framework_plugin_ownership::sdk_fault::OriginalRenderFailure::Original(failure))=>self.pending.retain_render_failure(semio_framework_plugin_ownership::sdk_fault::OriginalRenderFailure::Original(failure), cx),Err(source)=>self.pending.retain_render_failure(source, cx)}
    }

    /// 🕹️ Same materialize-before-destructure shape the `render`/`window_measures`/`context_menu`
    /// paths use: `interaction_hover`/`peer_presence` are read-only here, so an owned snapshot
    /// avoids aliasing the `&mut self` field-wise destructure below.
    pub(super) async fn window_engagements(&mut self, view_state: &ViewModel) -> HashMap<String, WindowEngagement> {
        if self.render_ctx().refresh_cache().await.is_err() {
            return HashMap::new();
        }
        let interaction_state = self.interaction.interaction_state(&self.stores).await;
        let interaction_hover = self.interaction.interaction_hover.clone();
        let interaction_peers = std::sync::Arc::clone(&self.roster.peer_presence);
        let draft_references = self.time_travel.draft_references();
        let interaction = InteractionView { state: &interaction_state, hover: &interaction_hover, peers: interaction_peers.as_ref(), draft_references: &draft_references };
        let render_operation = self.tools.live_render_operation(&self.mounted_policy, &self.stores);
        let parent_document_id = self.stores.store.envelope().id.clone();
        let VcsArtifactApp { window_config_store, window_transient_store, cache, child_content_root, transient_store, tool_runs, time_travel, tool_machines, .. } = self;
        let (_, snapshot, config, history) = cache.as_ref().expect("cache refreshed above");
        let doc = ArtifactView::with_render_context(
            time_travel.render_snapshot_or(tool_runs, tool_machines.overlay_or(snapshot)).as_ref(),
            history.as_ref(),
            ChildContentView::clone(time_travel.render_children_or(tool_runs, child_content_root)),
            render_operation,
            parent_document_id,
            None,
        )
        .await
        .with_tool_run(tool_runs.view_for(view_state.window_id.as_deref()))
        .with_instance(self.tools.instance_operation_owner.clone());
        let mut engagements = HashMap::new();
        for window in view_state.window_instances.iter().take(UI_RESIDENT_SLOTS) {
            let Some(window_view_state) = view_state.for_window_instance(&window.id) else { continue };
            let window_config = match window_config_store.capture(Some(&window_view_state)).await {
                Ok(snapshot) => snapshot,
                Err(_) => continue,
            };
            let window_transient = match window_transient_store.capture(Some(&window_view_state)) {
                Ok(snapshot) => snapshot,
                Err(_) => continue,
            };
            let cfg = ConfigView { snapshot: tool_machines.config_overlay_or(config).as_ref(), window: window_config.as_ref().map(|authority| tool_machines.window_config_overlay_or(&authority.snapshot)) };
            let transient_root = transient_store.current_root();
            let transient = TransientView { snapshot: transient_root.as_ref(), window: window_transient.as_ref().map(|authority| &authority.snapshot) };
            let mut projected = A::window_engagements_with_request_context(&doc, &cfg, &window_view_state, &transient, &interaction).await;
            if let Some(engagement) = projected.remove(&window.id).or_else(|| projected.remove(&window.window_kind_id)) {
                engagements.insert(window.id.clone(), engagement);
            }
        }
        engagements
    }

    pub(super) async fn window_measures(&mut self, view_state: &ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        if self.render_ctx().refresh_cache().await.is_err() {
            return HashMap::new();
        }
        let interaction_state = self.interaction.interaction_state(&self.stores).await;
        let interaction_hover = self.interaction.interaction_hover.clone();
        let interaction_peers = std::sync::Arc::clone(&self.roster.peer_presence);
        let draft_references = self.time_travel.draft_references();
        let interaction = InteractionView { state: &interaction_state, hover: &interaction_hover, peers: interaction_peers.as_ref(), draft_references: &draft_references };
        let render_operation = self.tools.live_render_operation(&self.mounted_policy, &self.stores);
        let parent_document_id = self.stores.store.envelope().id.clone();
        let VcsArtifactApp { window_config_store, cache, child_content_root, tool_runs, time_travel, tool_machines, .. } = self;
        let (_, snapshot, config, history) = cache.as_ref().expect("cache refreshed above");
        let doc = ArtifactView::with_render_context(
            time_travel.render_snapshot_or(tool_runs, tool_machines.overlay_or(snapshot)).as_ref(),
            history.as_ref(),
            ChildContentView::clone(time_travel.render_children_or(tool_runs, child_content_root)),
            render_operation,
            parent_document_id,
            None,
        )
        .await
        .with_tool_run(tool_runs.view_for(view_state.window_id.as_deref()))
        .with_instance(self.tools.instance_operation_owner.clone());
        let mut measures = HashMap::new();
        for window in view_state.window_instances.iter().take(UI_RESIDENT_SLOTS) {
            let Some(window_view_state) = view_state.for_window_instance(&window.id) else { continue };
            let window_config = match window_config_store.capture(Some(&window_view_state)).await {
                Ok(snapshot) => snapshot,
                Err(_) => continue,
            };
            let cfg = ConfigView { snapshot: tool_machines.config_overlay_or(config).as_ref(), window: window_config.as_ref().map(|authority| tool_machines.window_config_overlay_or(&authority.snapshot)) };
            let mut projected = A::window_measures_with_request_context(&doc, &cfg, &window_view_state, &interaction).await;
            if let Some(window_measures) = projected.remove(&window.id).or_else(|| projected.remove(&window.window_kind_id)) {
                measures.insert(window.id.clone(), window_measures);
            }
        }
        measures
    }

    pub(super) async fn tool_measures(&mut self, view_state: &ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        if self.render_ctx().refresh_cache().await.is_err() {
            return HashMap::new();
        }
        let window_config = match self.stores.window_config_store.capture(Some(view_state)).await {
            Ok(snapshot) => snapshot,
            Err(_) => return HashMap::new(),
        };
        let render_operation = self.tools.live_render_operation(&self.mounted_policy, &self.stores);
        let parent_document_id = self.stores.store.envelope().id.clone();
        let VcsArtifactApp { app: _, cache, child_content_root, tool_runs, time_travel, tool_machines, .. } = self;
        let (_, snapshot, config, history) = cache.as_ref().expect("cache refreshed above");
        let doc = ArtifactView::with_render_context(
            time_travel.render_snapshot_or(tool_runs, tool_machines.overlay_or(snapshot)).as_ref(),
            history.as_ref(),
            ChildContentView::clone(time_travel.render_children_or(tool_runs, child_content_root)),
            render_operation,
            parent_document_id,
            None,
        )
        .await
        .with_tool_run(tool_runs.view_for(view_state.window_id.as_deref()))
        .with_instance(self.tools.instance_operation_owner.clone());
        let cfg = ConfigView { snapshot: tool_machines.config_overlay_or(config).as_ref(), window: window_config.as_ref().map(|authority| tool_machines.window_config_overlay_or(&authority.snapshot)) };
        A::tool_measures(&doc, &cfg, view_state).await
    }

    /// 🔒️ The app instance's RETAINED operation owner, handed to the poll for the same
    /// reason `render_with_request_context` gets it: window-scoped background work an app
    /// arms from here is latched on state only that owner holds, and a poll that cannot see
    /// it re-arms an already-pending chain once per refresh
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    ///
    /// ⏯️ The poll sees the LIVE RUN, exactly as the four render and measure passes above do.
    /// `ArtifactApp::pending_effects` is where a surface starts a run once its work is owed and
    /// finalizes a complete one, and that whole ladder is written over `doc.tool_run()` — handed a
    /// view that always answered `None` it could only ever read "no run": the generation3d preview
    /// run started, ticked, completed and was never finalized, so the next `toolRunStart` a gesture
    /// asked for was a no-op against a run the framework still held and the 3d preview stopped
    /// re-evaluating for the rest of the session
    /// (`📓️preview-rearm-after-inspector-edit-2026-09-14.md`, contract §3.7). The document itself
    /// stays the COMMITTED snapshot: a poll decides about work over what has landed, never over a
    /// run's own provisional overlay.
    pub(super) async fn pending_effects(&mut self, view: Option<&ViewModel>) -> Vec<Effect> {
        if self.render_ctx().refresh_cache().await.is_err() {
            return Vec::new();
        }
        let render_operation = self.tools.live_render_operation(&self.mounted_policy, &self.stores);
        let parent_document_id = self.stores.store.envelope().id.clone();
        let snapshot_is_admitted = {
            let (_, snapshot, _, _) = self.history.cache.as_ref().expect("cache refreshed above");
            A::mounted_job_prepare_snapshot_read(render_operation, snapshot.as_ref())
        };
        let snapshot_read = if snapshot_is_admitted {
            match self.stores.store.snapshot_read() {
                Ok(snapshot_read) => Some(snapshot_read),
                Err(_) => return Vec::new(),
            }
        } else {
            None
        };
        let instance_operation_owner = self.tools.instance_operation_owner.clone();
        let effects = {
            let VcsArtifactApp { app: _, cache, child_content_root, tool_runs, tool_machines, .. } = self;
            let (_, snapshot, config, history) = cache.as_ref().expect("cache refreshed above");
            let doc = ArtifactView::with_render_context(snapshot.as_ref(), history.as_ref(), ChildContentView::clone(child_content_root), render_operation, parent_document_id, snapshot_read)
                .await
                .with_instance(instance_operation_owner.clone())
                .with_tool_run(tool_runs.view_for(view.and_then(|view| view.window_id.as_deref())))
                .with_provisional_generation(tool_machines.provisional_generation());
            let cfg = ConfigView { snapshot: config.as_ref(), window: None };
            A::pending_effects(&instance_operation_owner, &doc, &cfg, view).await
        };
        self.stores.stamp_polled_load_document_effects(effects)
    }

    /// 🗂️ Every context menu is organized (D2 of the grouped-context-menu mechanism design) at this
    /// single funnel — a raw-vec emitter is grouped for free, and an emitter that already built its own
    /// `Menu::group(...)` rows is never re-flattened (`organize_context_menu` is idempotent on already-organized input).
    pub(super) async fn context_menu(&mut self, request: &ContextMenuRequest, view_state: &ViewModel) -> Vec<ContextMenuItemSpec> {
        if self.render_ctx().refresh_cache().await.is_err() {
            return Vec::new();
        }
        let interaction_state = self.interaction.interaction_state(&self.stores).await;
        let interaction_hover = self.interaction.interaction_hover.clone();
        let interaction_peers = std::sync::Arc::clone(&self.roster.peer_presence);
        let draft_references = self.time_travel.draft_references();
        let interaction = InteractionView { state: &interaction_state, hover: &interaction_hover, peers: interaction_peers.as_ref(), draft_references: &draft_references };
        let window_config = match self.stores.window_config_store.capture(Some(view_state)).await {
            Ok(snapshot) => snapshot,
            Err(_) => return Vec::new(),
        };
        let render_operation = self.tools.live_render_operation(&self.mounted_policy, &self.stores);
        let parent_document_id = self.stores.store.envelope().id.clone();
        let VcsArtifactApp { app: _, cache, registry, child_content_root, instance_operation_owner, .. } = self;
        let (_, snapshot, config, history) = cache.as_ref().expect("cache refreshed above");
        let doc = ArtifactView::with_render_context(snapshot.as_ref(), history.as_ref(), ChildContentView::clone(child_content_root), render_operation, parent_document_id, None).await.with_instance(instance_operation_owner.clone());
        let cfg = ConfigView { snapshot: config.as_ref(), window: window_config.as_ref().map(|authority| &authority.snapshot) };
        let items = A::context_menu_with_request_context(request, &doc, &cfg, view_state, &interaction, registry).await;
        semio_framework_ui_layout::layout::organize_context_menu(items, &|id| registry.category_of(id))
    }
}
