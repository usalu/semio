//! 🪟️ The window projections of the render bundle: engagements, measures, pending effects and the context menu, read off the committed document snapshot.

use std::collections::HashMap;
use semio_plugin_decl_app::ArtifactApp;
use semio_plugin_decl_views::child_view::ChildContentView;
use semio_plugin_decl_views::{ArtifactView, ConfigView, InteractionView};
use semio_plugin_window_transient::window_transient::TransientView;
use semio_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_framework::kernel::Effect;
use semio_framework::{Fault, FaultFrom, ViewModel};
use semio_ui_contract::*;
use semio_ui_layout::layout::{ContextMenuItemSpec, ContextMenuRequest, WindowEngagement, WindowMeasure};
use store::{MemberFactory, ReferenceSpaceMember};
use crate::render_bundle::RenderCtx;

impl<'ctx, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + Send + 'static, X> RenderCtx<'ctx, A, M, X> {
    /// 🕹️ Snapshots read-only interaction state before borrowing the mutable render components.
    pub async fn window_engagements(&mut self, view_state: &ViewModel) -> HashMap<String, WindowEngagement> {
        if self.refresh_cache().await.is_err() {
            return HashMap::new();
        }
        let interaction_state = self.interaction.interaction_state(&self.stores).await;
        let interaction_hover = self.interaction.interaction_hover.clone();
        let Some(interaction_peers) = self.roster.peer_presence.as_ref() else { return HashMap::new(); };
        let interaction_peers = std::sync::Arc::clone(interaction_peers);
        let draft_references = self.time_travel.draft_references();
        let interaction = InteractionView { state: &interaction_state, hover: &interaction_hover, peers: interaction_peers.as_ref(), draft_references: &draft_references };
        let render_operation = self.tools.live_render_operation(&self.mounted_policy, &self.stores);
        let parent_document_id = self.stores.store.envelope().id.clone();
        let RenderCtx { stores, history: command_history, child_composition, tool_runs, time_travel, tool_machines, .. } = self;
        let (_, snapshot, config, history) = command_history.cache.as_ref().expect("cache refreshed above");
        let doc = ArtifactView::with_render_context(
            time_travel.render_snapshot_or(tool_runs, tool_machines.overlay_or(snapshot)).as_ref(),
            history.as_ref(),
            ChildContentView::clone(time_travel.render_children_or(tool_runs, &child_composition.child_content_root)),
            render_operation,
            &parent_document_id,
            None,
        )
        .await
        .with_original_tool_run(tool_runs.original_view_for(view_state.window_id.as_deref()))
        .with_instance(self.tools.instance_operation_owner.clone());
        let mut engagements = HashMap::new();
        for window in view_state.window_instances.iter().take(UI_RESIDENT_SLOTS) {
            let Some(window_view_state) = view_state.for_window_instance(&window.id) else { continue };
            let window_config = match stores.window_config_store.capture(Some(&window_view_state)).await {
                Ok(snapshot) => snapshot,
                Err(_) => continue,
            };
            let window_transient = match stores.window_transient_store.capture(Some(&window_view_state)) {
                Ok(snapshot) => snapshot,
                Err(_) => continue,
            };
            let cfg = ConfigView { snapshot: tool_machines.config_overlay_or(config).as_ref(), window: window_config.as_ref().map(|authority| stores.window_config_store.preview_or(&authority.snapshot)) };
            let transient_root = stores.transient_store.current_root();
            let transient = TransientView { snapshot: transient_root.as_ref(), window: window_transient.as_ref().map(|authority| &authority.snapshot) };
            let mut projected = A::window_engagements_with_request_context(&doc, &cfg, &window_view_state, &transient, &interaction).await;
            if let Some(engagement) = projected.remove(&window.id).or_else(|| projected.remove(&window.window_kind_id)) {
                engagements.insert(window.id.clone(), engagement);
            }
        }
        engagements
    }

    pub async fn window_measures(&mut self, view_state: &ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        if self.refresh_cache().await.is_err() {
            return HashMap::new();
        }
        let interaction_state = self.interaction.interaction_state(&self.stores).await;
        let interaction_hover = self.interaction.interaction_hover.clone();
        let Some(interaction_peers) = self.roster.peer_presence.as_ref() else { return HashMap::new(); };
        let interaction_peers = std::sync::Arc::clone(interaction_peers);
        let draft_references = self.time_travel.draft_references();
        let interaction = InteractionView { state: &interaction_state, hover: &interaction_hover, peers: interaction_peers.as_ref(), draft_references: &draft_references };
        let render_operation = self.tools.live_render_operation(&self.mounted_policy, &self.stores);
        let parent_document_id = self.stores.store.envelope().id.clone();
        let RenderCtx { stores, history: command_history, child_composition, tool_runs, time_travel, tool_machines, .. } = self;
        let (_, snapshot, config, history) = command_history.cache.as_ref().expect("cache refreshed above");
        let doc = ArtifactView::with_render_context(
            time_travel.render_snapshot_or(tool_runs, tool_machines.overlay_or(snapshot)).as_ref(),
            history.as_ref(),
            ChildContentView::clone(time_travel.render_children_or(tool_runs, &child_composition.child_content_root)),
            render_operation,
            &parent_document_id,
            None,
        )
        .await
        .with_original_tool_run(tool_runs.original_view_for(view_state.window_id.as_deref()))
        .with_instance(self.tools.instance_operation_owner.clone());
        let mut measures = HashMap::new();
        for window in view_state.window_instances.iter().take(UI_RESIDENT_SLOTS) {
            let Some(window_view_state) = view_state.for_window_instance(&window.id) else { continue };
            let window_config = match stores.window_config_store.capture(Some(&window_view_state)).await {
                Ok(snapshot) => snapshot,
                Err(_) => continue,
            };
            let cfg = ConfigView { snapshot: tool_machines.config_overlay_or(config).as_ref(), window: window_config.as_ref().map(|authority| stores.window_config_store.preview_or(&authority.snapshot)) };
            let mut projected = A::window_measures_with_request_context(&doc, &cfg, &window_view_state, &interaction).await;
            if let Some(window_measures) = projected.remove(&window.id).or_else(|| projected.remove(&window.window_kind_id)) {
                measures.insert(window.id.clone(), window_measures);
            }
        }
        measures
    }

    pub async fn tool_measures(&mut self, view_state: &ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        if self.refresh_cache().await.is_err() {
            return HashMap::new();
        }
        let window_config = match self.stores.window_config_store.capture(Some(view_state)).await {
            Ok(snapshot) => snapshot,
            Err(_) => return HashMap::new(),
        };
        let render_operation = self.tools.live_render_operation(&self.mounted_policy, &self.stores);
        let parent_document_id = self.stores.store.envelope().id.clone();
        let RenderCtx { app: _, history: command_history, child_composition, tool_runs, time_travel, tool_machines, .. } = self;
        let (_, snapshot, config, history) = command_history.cache.as_ref().expect("cache refreshed above");
        let doc = ArtifactView::with_render_context(
            time_travel.render_snapshot_or(tool_runs, tool_machines.overlay_or(snapshot)).as_ref(),
            history.as_ref(),
            ChildContentView::clone(time_travel.render_children_or(tool_runs, &child_composition.child_content_root)),
            render_operation,
            &parent_document_id,
            None,
        )
        .await
        .with_original_tool_run(tool_runs.original_view_for(view_state.window_id.as_deref()))
        .with_instance(self.tools.instance_operation_owner.clone());
        let cfg = ConfigView { snapshot: tool_machines.config_overlay_or(config).as_ref(), window: window_config.as_ref().map(|authority| self.stores.window_config_store.preview_or(&authority.snapshot)) };
        A::tool_measures(&doc, &cfg, view_state).await
    }

    /// 🔒️ Polls the retained operation owner and live run against the committed document snapshot.
    pub async fn pending_effects(&mut self,view:Option<&ViewModel>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Vec<Effect>,Fault>{
        if cx.should_yield(){return Ok(Vec::new())}
        self.refresh_cache().await?;
        let render_operation=self.tools.live_render_operation(&self.mounted_policy, &self.stores);
        let Some(grant)=semio_plugin_decl_views::MountedSnapshotReadPreparation::grant(self.mounted_policy.preparation,cx)else{return Ok(Vec::new())};
        let snapshot_read={let(_,snapshot,_,_)=self.history.cache.as_ref().ok_or_else(||plugin_sdk_fault("pending poll cache unavailable"))?;let preparation=A::mounted_job_prepare_snapshot_read(render_operation,snapshot.as_ref(),&self.stores.window_config_store,view,grant);if preparation.record(grant,cx)?{Some(self.stores.store.snapshot_read().map_err(|error|error.into_fault())?)}else{None}};
        let effects={
            let(_,snapshot,config,_)=self.history.cache.as_ref().ok_or_else(||plugin_sdk_fault("pending poll cache unavailable"))?;
            let doc=semio_plugin_decl_views::ArtifactPendingView::new(snapshot.as_ref(),&self.child_composition.child_content_root,render_operation,&self.stores.store.envelope().id,snapshot_read,self.tool_runs.poll_view_for(view.and_then(|view|view.window_id.as_deref())),self.tool_machines.provisional_generation(),&self.tools.instance_operation_owner);
            let cfg=ConfigView{snapshot:config.as_ref(),window:None};
            A::pending_effects(&self.tools.instance_operation_owner,&doc,&cfg,view).await
        };
        Ok(self.stores.stamp_polled_load_document_effects(effects))
    }

    /// 🗂️ Applies the idempotent context-menu organizer at the common emission boundary.
    pub async fn context_menu(&mut self, request: &ContextMenuRequest, view_state: &ViewModel) -> Vec<ContextMenuItemSpec> {
        if self.refresh_cache().await.is_err() {
            return Vec::new();
        }
        let interaction_state = self.interaction.interaction_state(&self.stores).await;
        let interaction_hover = self.interaction.interaction_hover.clone();
        let Some(interaction_peers) = self.roster.peer_presence.as_ref() else { return Vec::new(); };
        let interaction_peers = std::sync::Arc::clone(interaction_peers);
        let draft_references = self.time_travel.draft_references();
        let interaction = InteractionView { state: &interaction_state, hover: &interaction_hover, peers: interaction_peers.as_ref(), draft_references: &draft_references };
        let window_config = match self.stores.window_config_store.capture(Some(view_state)).await {
            Ok(snapshot) => snapshot,
            Err(_) => return Vec::new(),
        };
        let render_operation = self.tools.live_render_operation(&self.mounted_policy, &self.stores);
        let parent_document_id = self.stores.store.envelope().id.clone();
        let RenderCtx { app: _, history: command_history, child_composition, tools, .. } = self;
        let (_, snapshot, config, history) = command_history.cache.as_ref().expect("cache refreshed above");
        let doc = ArtifactView::with_render_context(snapshot.as_ref(), history.as_ref(), ChildContentView::clone(&child_composition.child_content_root), render_operation, &parent_document_id, None).await.with_instance(tools.instance_operation_owner.clone());
        let cfg = ConfigView { snapshot: config.as_ref(), window: window_config.as_ref().map(|authority| &authority.snapshot) };
        let items = A::context_menu_with_request_context(request, &doc, &cfg, view_state, &interaction, &tools.registry).await;
        semio_ui_layout::layout::organize_context_menu(items, &|id| tools.registry.category_of(id))
    }
}
