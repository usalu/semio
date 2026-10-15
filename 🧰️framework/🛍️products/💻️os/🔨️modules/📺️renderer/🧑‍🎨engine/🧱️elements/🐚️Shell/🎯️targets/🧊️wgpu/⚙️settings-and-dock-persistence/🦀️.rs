//! ⚙️ Shell lifecycle, part two: settings panels and dock persistence.
//!
//! Second half of the shell lifecycle `impl ShellState`: live window ids, world3d close, engine surfaces, the app
//! catalogue, document retention, settings panel builders, anchors and dock/named-layout persistence.
//!
//! One chunk of the former `shell` monolith (ticket CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT); [`ShellState`] and the
//! shared imports stay in the parent module and are reached through `use super::*;`.

use super::*;

//#region ShellLifecycle
impl ShellState {
    /// 🪟️ Every declared window instance, including siblings hidden by a focused/maximized view.
    /// The committed dock is the retention authority; the paint plan deliberately omits hidden
    /// siblings and therefore cannot decide that their engine owners have closed.
    pub(crate) fn live_window_ids(&self) -> Vec<String> {
        self.dock.collect_window_ids().into_iter().chain(self.panel_documents.keys().cloned()).collect()
    }

    /// 🕸️ Mirrors the engine surfaces a paint pass attached into `node_graph_states` /
    /// `tiled_map_states` / `board2d_states`, the maps the OS event loop hit-tests pointer/wheel
    /// events against (`node_graph_pointer_down_into` and siblings). The engine host itself is owned
    /// by the worker's `ENGINE_SURFACES` registry; this side carries only the surface's screen bounds
    /// and the controller its observations are addressed to.
    ///
    /// A host constructed this frame is also the moment the app-static catalogue must land on it —
    /// `refresh_app_catalogue` runs on the UI refresh, which for a freshly attached surface has
    /// already happened.
    pub fn sync_engine_surface_states(&mut self) {
        let registrations = crate::engine_canvas::take_engine_surface_registrations();
        let live_windows = self.live_window_ids();
        let live_windows: Vec<&str> = live_windows.iter().map(String::as_str).collect();
        Self::trace_engine_surface_sync(&self.node_graph_states, &registrations, &live_windows);
        let created = Self::mirror_engine_surface_states(&mut self.node_graph_states, &mut self.tiled_map_states, &mut self.board2d_states, &mut self.world3d_status, &mut self.world3d_window_ids, registrations, &live_windows);
        if !self.app_catalogue_json.is_empty() {
            for surface_id in created {
                crate::engine_canvas::node_graph_set_catalogue_json(&surface_id, &self.app_catalogue_json);
            }
        }
        self.retire_closed_world3d_windows(&live_windows);
        self.sync_world3d_declared_actions();
    }

    /// 🧯️ Whether a closed World3d window may retire now: at once unless it holds a gesture to abort
    /// (`world3d_cancel_owed`: a live gumball that streamed, an open paint stroke). Then its first sighting enqueues
    /// `cancel(CaptureLost)` (zero trace, as React's unmount does), takes the surface off the pointer's way (empty bounds) and
    /// waits for the frame's world authority to drive the cancel; it retires once its queue drained, or after
    /// [`WORLD3D_CLOSE_CANCEL_SIGHTINGS`] sightings. A cancel the full queue refuses retires at once.
    fn world3d_close_cancel_settled(&mut self, host_id: &str) -> bool {
        let Some(state) = self.world3d_states.get_mut(host_id) else { return true };
        let Some(sightings) = self.world3d_close_cancels.get_mut(host_id) else {
            if !infinite_world::world::world3d_cancel_owed(state) {
                return true;
            }
            let owed = enqueue_world3d_events(state, [WorldInteractionIntent::cancel(infinite_world::world::WorldCancelReason::CaptureLost)]).is_ok();
            if owed {
                state.bounds = Rect::new(0.0, 0.0, 0.0, 0.0);
                self.world3d_close_cancels.insert(host_id.to_owned(), 0);
            }
            return !owed;
        };
        *sightings = sightings.saturating_add(1);
        let settled = infinite_world::world::world3d_interaction_front_generation(state).is_none() || *sightings >= WORLD3D_CLOSE_CANCEL_SIGHTINGS;
        if settled {
            self.world3d_close_cancels.remove(host_id);
        }
        settled
    }

    /// 🪟️ Retires a closed World3d window at the layout-to-input sync boundary, once its capture-lost cancel was driven
    /// ([`Self::world3d_close_cancel_settled`]). The scene owner and every shell-local projection keyed by that owner
    /// disappear in one bounded pass before input.
    pub(crate) fn retire_closed_world3d_windows(&mut self, live_window_ids: &[&str]) {
        let retired: Vec<String> = self.world3d_window_ids.iter().filter(|(_, window_id)| !live_window_ids.contains(&window_id.as_str())).map(|(host_id, _)| host_id.clone()).collect();
        for host_id in retired {
            if !self.world3d_close_cancel_settled(&host_id) {
                continue;
            }
            let Some(window_id) = self.world3d_window_ids.remove(&host_id) else { continue };
            let Some(mut state) = self.world3d_states.remove(&host_id) else { continue };
            begin_world3d_dynamic_retirement(&mut state);
            self.world3d_retirement_epoch = self.world3d_retirement_epoch.wrapping_add(1).max(1);
            let retirement_id = format!("{host_id}#{}", self.world3d_retirement_epoch);
            assert!(self.retired_world3d_states.len() < SCENE_SURFACE_CAPACITY, "retired World3d owners share the fixed scene-owner grants");
            self.retired_world3d_states.push_back((retirement_id, state));
            assert!(self.world3d_states.set_external_reservations(self.retired_world3d_states.len()), "retired World3d reservation follows active owner removal");
            self.world3d_status.remove(&host_id);
            self.world3d_status_pill_trace.remove(&host_id);
            self.settle_pump.watches.remove(&host_id);
            self.active_utility_by_window.remove(&window_id);
            self.action_panel_folded.remove(&window_id);
            self.search_panel_folded.remove(&window_id);
            self.utility_bar_folded.remove(&window_id);
            self.projection_pane_folded.remove(&window_id);
            self.world_projection_template.remove(&window_id);
            self.window_icon_overrides.remove(&window_id);
            self.window_title_overrides.remove(&window_id);
            self.action_panel_expanded.remove(&window_id);
            self.search_possibles_open.remove(&window_id);
            self.measures_folded.remove(&window_id);
            self.measures_expanded.remove(&window_id);
            self.measures_width.remove(&window_id);
            self.engagement_expanded.remove(&window_id);
            self.engagement_activated.remove(&window_id);
            self.window_engagements.remove(&window_id);
            self.window_content_rects.remove(&window_id);
            self.window_silhouettes.remove(&window_id);
            let prefix = format!("{window_id}:");
            self.staged_action_args.retain(|key, _| !key.starts_with(&prefix));
        }
        self.world3d_status.retain(|host_id, _| self.world3d_window_ids.contains_key(host_id));
    }

    /// 📇️ Republishes, onto every live World3d surface, the action ids its WINDOW KIND declares.
    ///
    /// A World3d surface is keyed by its window instance id (the same identity `setCamera`'s
    /// `windowId` argument addresses), so its owning kind is exactly what the dock says that instance
    /// is, and the kind's `actions` are `window_kind_action_refs` after `build_definition` resolved
    /// them. This is the ONE declaration of which verbs a surface may emit: the world module offers
    /// no gumball handle whose verb is absent from it
    /// (`infinite_world::world::world3d_offers_transform_gumball`), rather than guessing per role.
    fn sync_world3d_declared_actions(&mut self) {
        let Some(session) = self.session.clone() else { return };
        let declarations: Vec<(String, Vec<String>)> = self
            .world3d_states
            .keys()
            .filter_map(|host_id| {
                let window_id = self.world3d_window_ids.get(host_id)?;
                let kind_id = self.live_window_kind_id(&session, window_id);
                let action_ids = session.app.window_kinds.iter().find(|kind| Some(kind.id.as_str()) == kind_id).map(|kind| kind.actions.iter().map(|action| action.id.clone()).collect()).unwrap_or_default();
                Some((host_id.clone(), action_ids))
            })
            .collect();
        for (host_id, action_ids) in declarations {
            if let Some(state) = self.world3d_states.get_mut(&host_id) {
                infinite_world::world::set_world3d_declared_actions(state, &action_ids);
            }
        }
    }

    /// 🩺️ One line per chrome-walk sync, naming the surfaces a DRAIN-eviction rule would have retired
    /// here — the ones that are live, whose window is live, and that this drain does not mention. A
    /// non-empty `old-rule-would-evict` followed by a press that still reaches the graph is the
    /// runtime witness that engine-surface state is owned by the window instance and not by a painted
    /// frame (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
    /// `🧑‍🎨engine/🧫️fixtures/🧲️engine-surface-retention/🔣️.json`). One line per completed chrome walk —
    /// tens per minute, not per frame.
    fn trace_engine_surface_sync(states: &AdmittedSurfaceMap<NodeGraphSurface>, registrations: &[crate::engine_canvas::EngineSurfaceRegistration], live_window_ids: &[&str]) {
        if !semio_framework_trace::runtime_diagnostics_enabled() {
            return;
        }
        use std::sync::atomic::{AtomicU64, Ordering};
        static SYNCS: AtomicU64 = AtomicU64::new(0);
        static DRAINS_WITH_GRAPH: AtomicU64 = AtomicU64::new(0);
        let painted = registrations.iter().any(|entry| matches!(entry.detail, crate::engine_canvas::EngineSurfaceKindDetail::NodeGraph));
        let syncs = SYNCS.fetch_add(1, Ordering::Relaxed) + 1;
        let with_graph = if painted { DRAINS_WITH_GRAPH.fetch_add(1, Ordering::Relaxed) + 1 } else { DRAINS_WITH_GRAPH.load(Ordering::Relaxed) };
        let would_evict: Vec<&str> = states.iter().filter(|(id, _)| !registrations.iter().any(|entry| &entry.host_id == *id)).map(|(id, _)| id.as_str()).collect();
        let live: Vec<String> = states.iter().map(|(id, surface)| format!("{id}@{}x{}+{},{}", surface.bounds.w.round(), surface.bounds.h.round(), surface.bounds.x.round(), surface.bounds.y.round())).collect();
        Self::debug_log(&format!("[TRACE] wgpu-shell engine surfaces syncs={syncs} drains-with-graph={with_graph} drain={} live={live:?} old-rule-would-evict={would_evict:?} windows={live_window_ids:?}", registrations.len()));
    }

    /// 🧲️ The retention rule itself, stated once for all three vello-composited kinds: an engine
    /// surface's pointer state belongs to its WINDOW INSTANCE, never to a painted frame. A drain
    /// REFRESHES what it mentions and retires only the surfaces whose owning window has left
    /// `live_window_ids`. Answers the surfaces whose engine host was constructed on this drain — the
    /// ones the app-static catalogue must be installed on, and only those.
    ///
    /// 🩸️ This used to evict every surface absent from the drain. Retained painting means an
    /// unchanged document does not repaint, so the node graph fell out of `node_graph_states` in
    /// nearly every frame — and a graph that is not in that map cannot be reached by a press, a
    /// wheel, a context menu, a catalogue drop or the `Fit graph` control, because every one of those
    /// paths starts by hit-testing this map. `world3d_states` was never a drain projection and its
    /// input worked throughout, which is the asymmetry this closes (ticket
    /// 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-retained-controls-wires-2026-09-13.md` §7.3(b)).
    ///
    /// `Self`-less so its law can drive the real body without a `ShellState` fixture —
    /// `🐚️Shell/🧪️tests/🧲️engine-surface-retention/🦀️.rs` over
    /// `🧑‍🎨engine/🧫️fixtures/🧲️engine-surface-retention/🔣️.json`.
    pub(crate) fn mirror_engine_surface_states(
        node_graph_states: &mut AdmittedSurfaceMap<NodeGraphSurface>,
        tiled_map_states: &mut AdmittedSurfaceMap<TiledMapSurface>,
        board2d_states: &mut AdmittedSurfaceMap<Board2dSurface>,
        world3d_status: &mut HashMap<String, String>,
        world3d_window_ids: &mut HashMap<String, String>,
        registrations: Vec<crate::engine_canvas::EngineSurfaceRegistration>,
        live_window_ids: &[&str],
    ) -> Vec<String> {
        let retired: Vec<String> = node_graph_states.iter().filter(|(_, surface)| !live_window_ids.contains(&surface.window_id.as_str())).map(|(id, _)| id.clone()).collect();
        for id in retired {
            node_graph_states.remove(&id);
        }
        let retired: Vec<String> = tiled_map_states.iter().filter(|(_, surface)| !live_window_ids.contains(&surface.window_id.as_str())).map(|(id, _)| id.clone()).collect();
        for id in retired {
            tiled_map_states.remove(&id);
        }
        let retired: Vec<String> = board2d_states.iter().filter(|(_, surface)| !live_window_ids.contains(&surface.window_id.as_str())).map(|(id, _)| id.clone()).collect();
        for id in retired {
            board2d_states.remove(&id);
        }
        let mut constructed = Vec::new();
        for registration in registrations {
            let crate::engine_canvas::EngineSurfaceRegistration { host_id, surface_id, engine_token: _, window_id, bounds, controller_id, detail, created } = registration;
            match detail {
                crate::engine_canvas::EngineSurfaceKindDetail::NodeGraph => {
                    if let Some(surface) = node_graph_states.get_or_insert_with(host_id.clone(), || NodeGraphSurface { surface_id: surface_id.clone(), bounds, controller_id: controller_id.clone(), window_id: window_id.clone() }) {
                        surface.surface_id = surface_id;
                        surface.bounds = bounds;
                        surface.controller_id = controller_id;
                        surface.window_id = window_id;
                    }
                    if created {
                        constructed.push(host_id);
                    }
                }
                crate::engine_canvas::EngineSurfaceKindDetail::TiledMap { selection_method } => {
                    if let Some(surface) = tiled_map_states.get_or_insert_with(host_id.clone(), || TiledMapSurface {
                        surface_id: surface_id.clone(),
                        bounds,
                        controller_id: controller_id.clone(),
                        selection_method: selection_method.clone(),
                        window_id: window_id.clone(),
                    }) {
                        surface.surface_id = surface_id;
                        surface.bounds = bounds;
                        surface.controller_id = controller_id;
                        surface.selection_method = selection_method;
                        surface.window_id = window_id;
                    }
                    if created {
                        constructed.push(host_id);
                    }
                }
                crate::engine_canvas::EngineSurfaceKindDetail::Board2d { snapshot_json } => {
                    if let Some(surface) =
                        board2d_states.get_or_insert_with(host_id.clone(), || Board2dSurface { surface_id: surface_id.clone(), bounds, controller_id: controller_id.clone(), snapshot_json: snapshot_json.clone(), window_id: window_id.clone() })
                    {
                        surface.surface_id = surface_id;
                        surface.bounds = bounds;
                        surface.controller_id = controller_id;
                        surface.snapshot_json = snapshot_json;
                        surface.window_id = window_id;
                    }
                    if created {
                        constructed.push(host_id);
                    }
                }
                crate::engine_canvas::EngineSurfaceKindDetail::World3d { status_json } => {
                    world3d_window_ids.insert(host_id.clone(), window_id);
                    match status_json {
                        Some(status_json) => world3d_status.insert(host_id, status_json),
                        None => world3d_status.remove(&host_id),
                    };
                }
            }
        }
        constructed
    }

    /// 🛍️ Installs the cached catalogue on every node-graph surface whose engine host does not
    /// already carry it — the wgpu twin of React's `AppCatalogueContext` provider.
    pub(crate) fn publish_app_catalogue(&self) {
        if self.app_catalogue_json.is_empty() {
            return;
        }
        for (host_id, _) in self.node_graph_states.iter() {
            crate::engine_canvas::node_graph_set_catalogue_json(host_id, &self.app_catalogue_json);
        }
    }

    pub(crate) fn retain_document_for_close(&mut self, document: UiDocumentLease) -> Result<(), UiDocumentLease> {
        self.closing_documents.try_admit(document)
    }

    /// 🔑️ Lends the chrome walk the EXACT window-body owner for one step and takes it back with
    /// [`Self::restore_window_document`]. A retained document read used to be reached through
    /// `UiDocumentLease::try_alias`, which mints a new arena alias: the arena admits
    /// [`UI_DOCUMENT_LEASE_ALIASES`] of them per slot and only `close_read_step_with_grant` gives one
    /// back, so a per-frame-step read that simply dropped its alias exhausted the slot after seven
    /// steps and every later read answered `AliasCapacity`. The chrome step reads that as "this
    /// window has no document", advances past the body, and the surface's page ingress freezes
    /// forever — a booted shell with an empty `UI_ENGINE` tree and a black canvas
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-blank-paint-2026-09-12.md`). Moving the one
    /// owner instead of aliasing it needs no credit at all, which is why it cannot run out.
    pub(crate) fn take_window_document(&mut self, window_id: &str) -> Option<UiDocumentLease> {
        if window_id == "spawned" {
            self.spawned_ui.take()
        } else {
            self.window_ui.remove(window_id)
        }
    }

    pub(crate) fn restore_window_document(&mut self, window_id: &str, document: UiDocumentLease) {
        if window_id == "spawned" {
            self.spawned_ui = Some(document);
        } else {
            self.window_ui.insert(window_id.to_string(), document);
        }
    }

    /// ♻️ Drives the retained-document retirement registry AND the `UiValue` arena it feeds to idle.
    ///
    /// ⚖️ One page per FRAME (`render_chrome_step`'s `CloseDocument` phase) is enough while the shell
    /// only refreshes on interaction. A converging guest chain is different: `flush_deferred_actions`
    /// runs many refreshes back to back inside ONE flush and no frame happens in between, so the
    /// registry only grows — and the `UiValue` collection slots a retired document's records hold are
    /// a FIXED process-wide arena. At `ui_value_headroom().collections == 0` every later `UiValue`
    /// deserialization fails as `data did not match any variant of untagged enum UiValue` and the
    /// shell stops painting entirely (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, measured on 6118 as
    /// `headroom collections=0`). A refresh therefore returns the previous refresh's arenas before it
    /// asks the guest for the next document.
    pub(crate) fn drain_retained_document_arenas(&mut self) {
        for _ in 0..SHELL_DOCUMENT_RETIREMENT_STEPS {
            let consumed = self.close_document_one();
            if !consumed && self.closing_documents.terminal_is_empty() {
                break;
            }
        }
        for _ in 0..SHELL_DOCUMENT_RETIREMENT_STEPS {
            if ui_contract::close_ui_value_page_one() {
                break;
            }
        }
    }

    /// ♻️ Retires ONE surface's previous document to terminal before its replacement is asked for.
    ///
    /// ⚖️ A refresh used to move EVERY live document into the retirement registry and only then ask
    /// the guest for the replacements, so the process-wide resident aggregate had to carry two full
    /// sets at once — and `UiResidentPermit::try_reserve` is a fixed aggregate, so the second set was
    /// refused `Capacity` on every surface and the shell kept the stale documents forever
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-dock-layout-world3d-2026-09-12.md` §6.2).
    /// One root at a time is the whole fix: the peak is the live surface count, never twice it.
    pub(crate) fn retire_one_surface_document(&mut self, document: Option<UiDocumentLease>) -> Result<(), String> {
        let Some(document) = document else { return Ok(()) };
        if self.retain_document_for_close(document).is_err() {
            return Err("shell: document retirement registry refused the exact prior owner".to_string());
        }
        self.drain_retained_document_arenas();
        Ok(())
    }

    /// 🧹 Retires every window (or panel) document whose surface the active layout no longer names.
    pub(crate) fn retire_documents_outside(&mut self, retained: &[String], windows: bool) -> Result<(), String> {
        let stale: Vec<String> = if windows { self.window_ui.keys().filter(|id| !retained.contains(id)).cloned().collect() } else { self.panel_documents.keys().filter(|id| !retained.contains(id)).cloned().collect() };
        for id in stale {
            if !crate::interpreter::request_ui_document_close(&id) {
                return Err(format!("shell: retained UI close registry refused surface '{id}'"));
            }
            self.chrome_build.clear_content_focus(&id);
            let document = if windows { self.window_ui.remove(&id) } else { self.panel_documents.remove(&id) };
            self.retire_one_surface_document(document)?;
        }
        Ok(())
    }

    /// 🎟️ The process-wide resident census, appended to every render's own log line so a `Capacity`
    /// refusal is never the first thing anyone learns about the aggregate.
    pub(crate) fn resident_census() -> String {
        ui_contract::UiResidentPermit::snapshot().map_or_else(|_| String::new(), |snapshot| format!(" resident-roots={} resident-bytes={}/{}", snapshot.used_slots, snapshot.bytes, ui_contract::UI_RESIDENT_AGGREGATE_BYTES))
    }

    pub(crate) fn close_document_one(&mut self) -> bool {
        if self.closing_documents.terminal_is_empty() {
            return !ui_contract::close_ui_document_page_one();
        }
        self.closing_documents.close_one()
    }

    /// 🎠️ terra-shell-unpark — this used to synchronously drive the kernel
    /// round trip right here. Unlike the two `glue.rs` call sites, `ShellState`
    /// has no `Weak<RefCell<AppRuntime>>` to re-borrow after the `.await` (only
    /// `AppRuntime` carries `self_weak`), which is exactly why an earlier packet
    /// left this one alone rather than force a borrow that doesn't exist. It
    /// doesn't need one: nothing here mutates shell state after firing, only logs
    /// on failure, so `plugin` (`ProgramBridgeEntry` is `Clone`) and the already-
    /// owned `pack`/`spr` from the matched `Effect` are enough to detach this into
    /// a fully self-contained `spawn_app_task` future — `queue_host_effects`
    /// itself stays a plain, non-async `fn`.
    ///
    /// 💡️ Slice D — the host-owned ephemeral inference port. The program named only an
    /// intent; this shell resolves the document scope, checks the execution-target lease,
    /// and drives the whole lifecycle itself. It never writes the document.
    ///
    /// 📥️ The extension door. This used to fire a DETACHED `spawn_app_task` and throw the
    /// answer away — so the `flowEvalResolve` the answer arms never reached the shell and
    /// the chain stopped after exactly one evaluate (ticket
    /// 26/09/09/PROCEDURAL-3D-END-TO-END). React's `dispatchInvokeExtensionEffect`
    /// publishes the response through `applyHostEffects`; this parks the request for the
    /// flush below, which owns `&mut self` and can do the same.
    ///
    /// ⬇️ Every plugin's export door. This arm did not exist: a `DownloadMediaExport` fell
    /// into the loud drop below, so no plugin could export ANYTHING on this renderer while
    /// React's shell was saving the same export as corrupt base64 text (ticket
    /// 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️io-surface-2026-09-13.md` §7.3).
    ///
    /// 📤️ Every plugin's IMPORT door. This arm did not exist either, and the only
    /// `requestFileOpen` reader this shell had read it out of a document MUTATION payload
    /// no producer in the repo emits — so `Import Document…` reached the guest, the guest
    /// emitted its effect, and the user never saw a picker (ticket
    /// 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-end-to-end-verification-2026-09-14.md` §C).
    /// Parked, never opened here: a picker ends on a user gesture and this funnel is a
    /// plain `fn`; `drain_deferred_actions` owns the `&mut self` the pick needs.
    pub(crate) fn queue_host_effects(&mut self, controller_id: &str, effects: Vec<semio_framework::kernel::Effect>) {
        for effect in effects {
            match effect {
                semio_framework::kernel::Effect::SetActiveUtility { window_id, utility_id } => {
                    self.set_active_utility(&window_id, (!utility_id.is_empty()).then_some(utility_id.as_str()));
                }
                semio_framework::kernel::Effect::Navigate { uri } => {
                    self.push_uri(uri);
                }
                semio_framework::kernel::Effect::OpenDialog { dialog_id, args, .. } => {
                    if let Some(request) = self.chrome_dialog_request(&dialog_id, args) {
                        self.chrome_build.open_dialog(request);
                    } else {
                        Self::debug_log(&format!("[TRACE] wgpu shell ignored unknown dialog {dialog_id}"));
                    }
                }
                semio_framework::kernel::Effect::LoadDocument { pack, spr } => {
                    if let Some(session) = self.session.clone() {
                        if let Some(plugin) = self.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).cloned() {
                            let instance_id = session.instance_id;
                            #[cfg(not(target_arch = "wasm32"))]
                            self.submit_shell_io_future(async move { ShellIoCompletion::Program(plugin.load_app_document_pack(instance_id, &pack, &spr).await) });
                            #[cfg(target_arch = "wasm32")]
                            crate::spawn_app_task(async move {
                                if let Err(error) = plugin.load_app_document_pack(instance_id, &pack, &spr).await { Self::debug_log(&format!("[TRACE] wgpu shell loadDocument effect failed: {error}")); }
                            });
                        }
                    }
                }
                semio_framework::kernel::Effect::DispatchAction { action: dispatch_action_id, args, .. } => {
                    self.deferred_actions.push(ActionDescriptor { controller_id: controller_id.to_string(), action: dispatch_action_id, args });
                }
                semio_framework::kernel::Effect::RequestMediaFrames { accept, frame_action, done_action, fallback_action, sample_stride, max_frames, max_long_edge_px, fps_hint, payload, args, .. } => {
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        let controller_id = controller_id.to_string();
                        let args = optional_dsl_value_as_json(args);
                        self.submit_shell_io_future(async move {
                            let frames = request_media_frames(&controller_id, &accept, &frame_action, &done_action, &fallback_action, sample_stride, max_frames, max_long_edge_px, fps_hint, payload.as_deref(), args).await;
                            ShellIoCompletion::MediaFrames { controller_id, frames }
                        });
                    }
                    #[cfg(target_arch = "wasm32")]
                    {
                        let frames = request_media_frames(controller_id, &accept, &frame_action, &done_action, &fallback_action, sample_stride, max_frames, max_long_edge_px, fps_hint, payload.as_deref(), optional_dsl_value_as_json(args));
                        self.begin_media_frames_transfer(controller_id.to_string(), frames);
                    }
                }
                semio_framework::kernel::Effect::SendMessage { target: semio_framework::kernel::MessageEndpoint::Backbone { uri }, payload } => {
                    let outcome =
                        self.sync_channel.as_ref().ok_or_else(|| "document-backbone effect has no active shell owner".to_string()).and_then(|channel| {
                            route_document_backbone_effects(&channel.actor_uri, &channel.cmd_tx, vec![semio_framework::kernel::Effect::SendMessage { target: semio_framework::kernel::MessageEndpoint::Backbone { uri }, payload }])
                                .and_then(|remaining| if remaining.is_empty() { Ok(()) } else { Err("document-backbone routing left an owned effect behind".into()) })
                        });
                    if let Err(error) = outcome {
                        self.sync_terminal_fault = Some(error);
                    }
                }
                #[cfg(not(target_arch = "wasm32"))]
                semio_framework::kernel::Effect::RequestServiceOperation {owner,service_id,action,payload} => {
                    self.open_service_port(&owner,&service_id,&action,payload);
                }
                semio_framework::kernel::Effect::InvokeExtension { req, extension_id, capability, request_json, .. } => {
                    #[cfg(target_arch = "wasm32")]
                    self.pending_extension_invocations.push(PendingExtensionInvocation { req: req.0, extension_id, capability, request_json });
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        let _ = (req, extension_id, capability, request_json);
                    }
                }
                semio_framework::kernel::Effect::DownloadMediaExport { filename, mime_type, data, encoding } => {
                    download_media_export(&filename, &mime_type, &data, encoding.as_deref());
                }
                semio_framework::kernel::Effect::IconRenderExport { items } => {
                    self.enqueue_icon_export(items);
                }
                semio_framework::kernel::Effect::RequestFileOpen { accept, read_as, import_action, args, multiple, .. } => {
                    #[cfg(target_arch = "wasm32")]
                    self.pending_file_opens.push(PendingFileOpen { controller_id: controller_id.to_string(), accept, read_as, import_action, args, multiple });
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        let controller_id = controller_id.to_string();
                        self.submit_shell_io_future(async move { ShellIoCompletion::Import(PickedImport { opened: request_file_open(&accept, read_as.as_deref(), multiple).await, controller_id, import_action, args, multiple }) });
                    }
                }
                other => {
                    Self::debug_log(&format!("[TRACE] wgpu-shell effect dropped tag={other:?}"));
                }
            }
        }
    }

    /// 🎨️ React-equivalent canonical theme tree with independently expandable virtualized groups.
    pub(crate) fn build_settings_theme_ui(&self) -> UiNode {
        self.build_settings_theme_editor_tree(&self.theme_document(), self.locale_id == "de")
    }

    /// 🚗️ The document edited by General: the live draft, else the selected custom or builtin.
    pub(crate) fn driver_document(&self) -> OsUiDriver {
        self.driver_draft.clone().unwrap_or_else(|| resolve_ui_driver_document(&self.driver_id, &self.chrome_build.preferences.custom_drivers))
    }

    pub(crate) fn clear_driver_draft(&mut self) {
        self.driver_draft = None;
        self.chrome_build.driver = resolve_ui_driver_chrome(&self.driver_id, &self.chrome_build.preferences.custom_drivers);
    }

    pub(crate) fn set_driver_field(&mut self, key: &str, value: &str) -> bool {
        if !valid_ui_driver_axis_value(key, value) {
            return false;
        }
        let mut draft = self.driver_document();
        if !matches!(draft.config,DslValue::Object(_)) {
            draft.config = builtin_ui_driver_document("default").config;
        }
        if let DslValue::Object(entries)=&mut draft.config {
            if let Some((_,axis))=entries.iter_mut().find(|(name,_)|name==key){*axis=DslValue::String(value.to_owned());}else{entries.push((key.to_owned(),DslValue::String(value.to_owned())));}
        }
        self.chrome_build.driver = ui_driver_document_chrome(&draft);
        self.driver_draft = Some(draft);
        true
    }

    fn build_settings_driver_editor(&self, is_de: bool) -> UiTreeSectionNode {
        let driver = self.driver_document();
        let fallback = builtin_ui_driver_document("default");
        let mut items = Vec::new();
        for (key, label_key, options) in SHELL_DRIVER_AXES {
            let value = ui_driver_axis(&driver, key).or_else(|| ui_driver_axis(&fallback, key)).unwrap_or_default();
            items.push(settings_tree_select_item_with(
                &format!("framework.settings.driver.{key}"),
                shell_chrome_string(*label_key, is_de),
                value,
                options.iter().map(|(value, key)| ((*value).to_string(), shell_chrome_string(*key, is_de).to_string())).collect(),
                "setDriverField",
                crate::action_args_json!({ "key": *key }),
            ));
        }
        items.push(UiTreeItemNode {
            id: "framework.settings.driver.save.label".into(),
            label: Label::data(shell_chrome_string("common.name", is_de)),
            control: Some(UiControlNode::Input(UiInputNode {
                id: "framework.settings.driver.saveLabel".into(),
                input_kind: "text".into(),
                value: self.driver_save_label.clone(),
                placeholder: Some(Label::data(shell_chrome_string("settings.driver.savePlaceholder", is_de))),
                accessibility_label: None,
                commit: Some("blur".into()),
                min: None,
                max: None,
                step: None,
                accept: None,
                precision: None,
                snaps: Vec::new(),
                on_change: ActionDescriptor { controller_id: "framework".into(), action: "setDriverSaveLabel".into(), args: None },
                on_submit: None,
                on_abort: None,
                on_repeat_last: None,
                presence: UiPresence::default(),
                menu: None,
                ..Default::default()
            })),
            ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
        });
        items.push(UiTreeItemNode {
            id: "framework.settings.driver.save.action".into(),
            label: Label::data(shell_chrome_string("settings.driver.save", is_de)),
            control: Some(UiControlNode::Button(UiButtonNode {
                id: Some("framework.settings.driver.save".into()),
                icon_id: IconName::Save,
                label: Label::data(shell_chrome_string("settings.driver.save", is_de)),
                action: ActionDescriptor { controller_id: "framework".into(), action: "saveDriver".into(), args: None },
                style: None,
                presence: UiPresence { state: if self.driver_save_label.trim().is_empty() { ui_wgpu::wgpu::component::ui::UiState::Disabled } else { ui_wgpu::wgpu::component::ui::UiState::Normal }, ..UiPresence::default() },
                menu: None,
            })),
            ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
        });
        if self.driver_id.starts_with("custom.") {
            items.push(UiTreeItemNode {
                id: "framework.settings.driver.delete.action".into(),
                label: Label::data(shell_chrome_string("settings.driver.delete", is_de)),
                control: Some(UiControlNode::Button(UiButtonNode {
                    id: Some("framework.settings.driver.delete".into()),
                    icon_id: IconName::Trash2,
                    label: Label::data(shell_chrome_string("settings.driver.delete", is_de)),
                    action: ActionDescriptor { controller_id: "framework".into(), action: "deleteDriver".into(), args: crate::action_args_json!({ "value": self.driver_id.clone() }) },
                    style: None,
                    presence: UiPresence::default(),
                    menu: None,
                })),
                ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
            });
        }
        let label = if self.driver_draft.is_some() { format!("{} ({})", shell_chrome_string("settings.tab.driver", is_de), shell_chrome_string("settings.driver.dirty", is_de)) } else { shell_chrome_string("settings.tab.driver", is_de).to_string() };
        UiTreeSectionNode { header_toolbar: None, id: "framework.settings.driver.editor".into(), label: Some(Label::data(label)), default_open: Some(false), presence: UiPresence::default(), items, window: None }
    }

    /// ⚙️ The `framework.settings.general` leaf's body — the wgpu twin of React's
    /// `buildSettingsGeneralTree` (`📌️ChromePanels/🟦️.tsx`): the app-identity section, then the
    /// appearance / layout / driver / language / terminology selects and the Reset Dock button, each
    /// row's id and each option's value byte-identical to React's. Rows whose axis is locked by the
    /// boot descriptor are omitted entirely (React's own `locks.*` render gates), never disabled.
    ///
    /// ⚖️ React's `framework.settings.mergePolicy` row (`📌️ChromePanels/🟦️.tsx:467-486`) — the one
    /// General-leaf control with no wgpu twin (audit W14 §B2). Option VALUES are React's
    /// `MERGE_POLICY_OPTIONS` verbatim, which are `protocol::MergePolicy`'s own variant names.
    pub(crate) fn build_settings_general_ui(&self) -> UiNode {
        let is_de = self.locale_id == "de";
        let locks = crate::boot_locks();
        let mut sections = Vec::new();
        if let Some(session) = self.session.as_ref() {
            let app_label = session.app.label.resolve(self.active_terminology(), self.active_locale()).to_string();
            sections.push(UiTreeSectionNode {
                header_toolbar: None,
                id: "framework.settings.app".into(),
                label: Some(Label::data(shell_chrome_string("settings.tab.app", is_de))),
                default_open: Some(true),
                presence: UiPresence::default(),
                items: vec![
                    UiTreeItemNode::base("framework.settings.app.label", Label::data(format!("{}: {app_label}", shell_chrome_string("settings.app.name", is_de)))),
                    UiTreeItemNode::base("framework.settings.app.id", Label::data(format!("{}: {}", shell_chrome_string("settings.app.id", is_de), session.app.id))),
                    UiTreeItemNode::base("framework.settings.app.controller", Label::data(format!("{}: {}", shell_chrome_string("settings.app.controller", is_de), session.app.controller_id))),
                    UiTreeItemNode::base("framework.settings.app.plugin", Label::data(format!("{}: {}", shell_chrome_string("settings.app.plugin", is_de), session.plugin_id))),
                ],
                window: None,
            });
        }
        let mut general = Vec::new();
        if locks.appearance.is_empty() {
            general.push(settings_tree_select_item(
                "framework.settings.appearance",
                shell_chrome_string("settings.tab.appearance", is_de),
                &self.appearance_id,
                vec![
                    ("system".to_string(), shell_chrome_string("settings.appearance.system", is_de).to_string()),
                    ("light".to_string(), shell_chrome_string("settings.appearance.light", is_de).to_string()),
                    ("dark".to_string(), shell_chrome_string("settings.appearance.dark", is_de).to_string()),
                ],
                "setAppearance",
            ));
        }
        general.push(settings_tree_select_item(
            "framework.settings.layout",
            shell_chrome_string("settings.tab.layout", is_de),
            &self.chrome_build.preferences.ui_layout,
            vec![("desktop".to_string(), shell_chrome_string("settings.layout.desktop", is_de).to_string()), ("tablet".to_string(), shell_chrome_string("settings.layout.tablet", is_de).to_string())],
            "setLayout",
        ));
        general.push(settings_tree_select_item("framework.settings.driver", shell_chrome_string("settings.tab.driver", is_de), &self.driver_id, self.driver_rows(), "setDriver"));
        if locks.locale.is_empty() {
            general.push(settings_tree_select_item(
                "framework.settings.language",
                shell_chrome_string("settings.tab.language", is_de),
                &self.locale_id,
                vec![("en".to_string(), shell_chrome_string("settings.language.en", is_de).to_string()), ("de".to_string(), shell_chrome_string("settings.language.de", is_de).to_string())],
                "setLocale",
            ));
        }
        if locks.terminology.is_empty() {
            general.push(settings_tree_select_item(
                "framework.settings.terminology",
                shell_chrome_string("settings.tab.terminology", is_de),
                &self.terminology_id,
                self.active_terminologies().into_iter().map(|id| (id.clone(), id)).collect(),
                "setTerminology",
            ));
        }
        general.push(settings_tree_select_item(
            "framework.settings.mergePolicy",
            shell_chrome_string("settings.mergePolicy", is_de),
            &self.merge_policy,
            SHELL_MERGE_POLICY_OPTIONS.iter().map(|(value, label_key)| ((*value).to_string(), shell_chrome_string(*label_key, is_de).to_string())).collect(),
            "setMergePolicy",
        ));
        general.push(UiTreeItemNode {
            id: "framework.settings.resetDock.action".into(),
            label: Label::data(shell_chrome_string("settings.resetDock", is_de)),
            control: Some(UiControlNode::Button(UiButtonNode {
                id: Some("framework.settings.resetDock".into()),
                icon_id: IconName::RotateCcw,
                label: Label::data(shell_chrome_string("settings.resetDock", is_de)),
                action: ActionDescriptor { controller_id: "framework".into(), action: "runOsCommand".into(), args: crate::action_args_json!({ "value": "os.resetDock" }) },
                style: None,
                presence: UiPresence::default(),
                menu: None,
            })),
            ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
        });
        sections.push(UiTreeSectionNode {
            header_toolbar: None,
            id: "framework.settings.general".into(),
            label: Some(Label::data(shell_chrome_string("settings.tab.general", is_de))),
            default_open: Some(true),
            presence: UiPresence::default(),
            items: general,
            window: None,
        });
        sections.push(self.build_settings_driver_editor(is_de));
        UiNode::Tree(UiTreeNode { presentation: Default::default(), sections, presence: UiPresence::default(), drop_action: None, menu: None, interaction_domain: None })
    }

    /// 🚗️ Every selectable driver id with its display label — the two built-ins plus whatever the
    /// preferences carry, React's `uiDriverList = [...builtinUiDrivers(), ...customDrivers]`.
    pub(crate) fn driver_rows(&self) -> Vec<(String, String)> {
        let is_de = self.locale_id == "de";
        let mut rows = vec![("default".to_string(), shell_chrome_string("settings.driver.default", is_de).to_string()), ("compact".to_string(), shell_chrome_string("settings.driver.compact", is_de).to_string())];
        for (id, driver) in &self.chrome_build.preferences.custom_drivers {
            if !rows.iter().any(|(existing, _)| existing == id) {
                rows.push((id.clone(), driver.label.clone()));
            }
        }
        rows
    }

    /// ⌨️ The `framework.settings.keybindings` leaf's body — the wgpu twin of React's
    /// `buildSettingsKeybindingsTree` (`📌️ChromePanels/🟦️.tsx`): one row per remappable shortcut,
    /// carrying the chord in force (through [`format_keybinding_shortcut`]), a conflict marker when a
    /// later row claims the same first chord, a capture control and a reset control.
    ///
    /// ⌨️ React captures the next chord from a live `keydown` on the capture button. The wgpu chord
    /// capture lives in {@link ShellState::keybinding_capture_control_id}: arming a row makes the next
    /// key event a chord write instead of a shell verb, which is the same one-gesture remap.
    pub(crate) fn build_settings_keybindings_ui(&self) -> UiNode {
        let is_de = self.locale_id == "de";
        let overrides = &self.chrome_build.preferences.keybinding_overrides;
        let mut rows = Vec::new();
        for row in shell_keybinding_rows(overrides) {
            let capturing = self.keybinding_capture_control_id.as_deref() == Some(row.control_id.as_str());
            let label = humanize_control_id(&row.control_id);
            let label = if row.conflicted { format!("{label} ({})", shell_chrome_string("settings.keybindings.conflict", is_de)) } else { label };
            let mut controls = vec![
                settings_text_row(&format!("{label} — {}", format_keybinding_shortcut(&row.keys))),
                UiNode::Button(UiButtonNode {
                    id: Some(format!("framework.settings.keybindings.capture.{}", row.control_id)),
                    icon_id: IconName::CircleDot,
                    label: Label::data(shell_chrome_string(if capturing { "settings.keybindings.pressKeys" } else { "settings.keybindings.capture" }, is_de)),
                    action: ActionDescriptor { controller_id: "framework".into(), action: "captureKeybinding".into(), args: crate::action_args_json!({ "value": row.control_id.clone() }) },
                    style: None,
                    presence: UiPresence { selected: capturing, ..UiPresence::default() },
                    menu: None,
                }),
            ];
            if row.overridden {
                controls.push(UiNode::Button(UiButtonNode {
                    id: Some(format!("framework.settings.keybindings.reset.{}", row.control_id)),
                    icon_id: IconName::RotateCcw,
                    label: Label::data(shell_chrome_string("settings.keybindings.reset", is_de)),
                    action: ActionDescriptor { controller_id: "framework".into(), action: "resetKeybinding".into(), args: crate::action_args_json!({ "value": row.control_id.clone() }) },
                    style: None,
                    presence: UiPresence::default(),
                    menu: None,
                }));
            }
            rows.push(UiNode::Stack(UiStackNode {
                direction: "column".into(),
                gap: None,
                padding: None,
                id: Some(format!("framework.settings.keybindings.{}", row.control_id)),
                children: controls,
                presence: UiPresence::default(),
                activate: None,
                drop_action: None,
                drop_overlay: None,
                menu: None,
            }));
        }
        UiNode::Stack(UiStackNode {
            direction: "column".into(),
            gap: None,
            padding: None,
            id: Some("framework.settings.keybindings.panel".into()),
            children: vec![settings_section("framework.settings.keybindings.list", shell_chrome_string("settings.tab.keybindings", is_de), rows)],
            presence: UiPresence::default(),
            activate: None,
            drop_action: None,
            drop_overlay: None,
            menu: None,
        })
    }

    /// 👁️✏️ The `framework.settings.default-apps` leaf's body — the wgpu twin of React's
    /// `buildSettingsDefaultAppsTree`: one row per (dialect × role) the loaded plugins can open, whose
    /// `Select` pins that pairing's default app (`"none"` clears it, React's `DEFAULT_APP_NONE_VALUE`),
    /// value encoded as `"<pluginId> <appId>"` exactly like React's `encodeDefaultAppValue`.
    pub(crate) fn build_settings_default_apps_ui(&self) -> UiNode {
        let is_de = self.locale_id == "de";
        let rows = self.default_app_rows();
        if rows.is_empty() {
            return UiNode::Stack(UiStackNode {
                direction: "column".into(),
                gap: None,
                padding: None,
                id: Some("framework.settings.defaultApps.empty".into()),
                children: vec![settings_text_row(shell_chrome_string("settings.unavailable", is_de))],
                presence: UiPresence::default(),
                activate: None,
                drop_action: None,
                drop_overlay: None,
                menu: None,
            });
        }
        let items: Vec<UiNode> = rows
            .into_iter()
            .map(|(dialect, role, value, options)| {
                let mut choices = vec![(DEFAULT_APP_NONE_VALUE.to_string(), shell_chrome_string("settings.defaultApps.none", is_de).to_string())];
                choices.extend(options);
                settings_select_row_with(&format!("framework.settings.defaultApps.{dialect}.{role}"), &format!("{dialect} — {role}"), &value, choices, "setDefaultApp", crate::action_args_json!({ "row": format!("{dialect}/{role}") }))
            })
            .collect();
        UiNode::Stack(UiStackNode {
            direction: "column".into(),
            gap: None,
            padding: None,
            id: Some("framework.settings.defaultApps.panel".into()),
            children: vec![settings_section("framework.settings.defaultApps.table", shell_chrome_string("settings.tab.defaultApps", is_de), items)],
            presence: UiPresence::default(),
            activate: None,
            drop_action: None,
            drop_overlay: None,
            menu: None,
        })
    }

    /// 👁️✏️ `(dialect, role, pinned value, options)` per opening pairing the loaded manifests declare —
    /// React's `defaultAppsRows` memo over `AppRouter.entriesFor` plus the persisted `OpeningPreferences`.
    fn default_app_rows(&self) -> Vec<(String, String, String, Vec<(String, String)>)> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

        let mut dialects: Vec<String> = Vec::new();
        for entry in &self.plugins {
            for app in &entry.manifest.apps {
                let dialect = app.dialect.to_coordinate();
                if !dialects.contains(&dialect) {
                    dialects.push(dialect);
                }
            }
        }
        dialects.sort();
        let mut rows = Vec::new();
        for dialect in dialects {
            for role in ["viewer", "editor"] {
                let options: Vec<(String, String)> = self
                    .plugins
                    .iter()
                    .flat_map(|entry| {
                        entry.manifest.apps.iter().filter(|app| app.dialect.to_coordinate() == dialect && app.role.as_str() == role).map(|app| (format!("{} {}", entry.plugin_id, app.id), format!("{} · {}", entry.manifest.label, app.id)))
                    })
                    .collect();
                if options.is_empty() {
                    continue;
                }
                let pinned = self.default_app_pins.get(&format!("{dialect}/{role}")).cloned().unwrap_or_else(|| DEFAULT_APP_NONE_VALUE.to_string());
                rows.push((dialect.clone(), role.to_string(), pinned, options));
            }
        }
        rows
    }

    /// 🖥️ The `framework.display.windows` leaf's body — the wgpu twin of React's
    /// `buildDisplayWindowsTree` (`📌️ChromePanels/🟦️.tsx`): one section per window KIND the app
    /// declares, whose rows are that kind's plain leaf plus — for a `world-3d` surface — the whole
    /// `createWorldProjectionTemplates` taxonomy, each row carrying React's own control id.
    ///
    /// 🔃️ React pre-reverses every template level to cancel the bottom-anchored `Tree`'s own
    /// per-level reversal, so what it RENDERS is: the plain kind leaf, then Parallel, then
    /// Perspective. The retained bottom-anchored tree uses that same up-flow layout and authored order.
    ///
    /// 🖱️ React's rows are drag SOURCES (`COMPOSE_WINDOW_TEMPLATE_MIME`) with no click handler. The
    /// retained Tree carries that same payload into the dock's tab/split/root-split drop resolver.
    pub(crate) fn build_display_windows_ui(&self) -> UiNode {
        let is_de = self.locale_id == "de";
        let Some(session) = self.session.as_ref() else {
            return display_panel_body("framework.display.windows.panel", vec![display_unavailable_section("framework.display.windows.empty", is_de)]);
        };
        let terminology = self.active_terminology();
        let locale = self.active_locale();
        if session.app.window_kinds.iter().next().is_none() {
            return display_panel_body("framework.display.windows.panel", vec![display_unavailable_section("framework.display.windows.empty", is_de)]);
        }
        let mut sections: Vec<UiTreeSectionNode> = Vec::new();
        for kind in session.app.window_kinds.iter() {
            let label = kind.label.resolve(terminology, locale).to_string();
            let mut items = if kind.surface_kind == ui_wgpu::wgpu::SurfaceKind::World3d {
                world_projection_template_rows(&semio_framework::child_element_id(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID, &[kind.id.as_str(), "projection"]), &kind.id, 0, &mut 0)
            } else {
                Vec::new()
            };
            items.push(UiTreeItemNode {
                id: semio_framework::child_element_id(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID, &[kind.id.as_str(), "kind"]),
                label: Label::data(label.clone()),
                icon_id: Some(kind.icon_id.clone()),
                draggable: Some(true),
                drag_data: Some(window_template_drag_data(&kind.id, None)),
                ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
            });
            sections.push(UiTreeSectionNode {
                header_toolbar: None,
                id: semio_framework::child_element_id(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID, &[kind.id.as_str()]),
                label: Some(Label::data(label)),
                default_open: Some(false),
                presence: UiPresence::default(),
                items,
                window: None,
            });
        }
        display_panel_body("framework.display.windows.panel", sections)
    }

    /// 🖥️ The `framework.display.layout` leaf's body — the wgpu twin of React's
    /// `buildDisplayLayoutTree`: a `framework.display.layout.save` section carrying the name box and
    /// the Save button (disabled on a blank name, React's own `!host.layoutSaveLabel.trim()` gate),
    /// then a `framework.display.layout.list` section carrying the app's builtin layouts folded by
    /// their `groupPath` and, when this session saved any, a `Saved` folder whose rows each offer
    /// React's `framework.display.delete.<id>` action.
    pub(crate) fn build_display_layout_ui(&self) -> UiNode {
        let is_de = self.locale_id == "de";
        let builtin: Vec<ui_wgpu::wgpu::NamedLayout> = self.session.as_ref().map(|session| session.app.named_layouts.iter().filter(|entry| entry.origin == "builtin").cloned().collect()).unwrap_or_default();
        let save_section = UiTreeSectionNode {
            header_toolbar: None,
            id: "framework.display.layout.save".into(),
            label: Some(Label::data(shell_chrome_string("display.saveLayout", is_de))),
            default_open: Some(false),
            presence: UiPresence::default(),
            items: vec![
                UiTreeItemNode {
                    id: "framework.display.layout.save.label".into(),
                    label: Label::data(shell_chrome_string("common.name", is_de)),
                    control: Some(ui_wgpu::wgpu::component::ui::UiControlNode::Input(UiInputNode {
                        id: "framework.display.saveLabel".into(),
                        input_kind: "text".into(),
                        value: self.layout_save_label.clone(),
                        placeholder: Some(Label::data(shell_chrome_string("display.saveLayoutPlaceholder", is_de))),
                        accessibility_label: None,
                        commit: None,
                        min: None,
                        max: None,
                        step: None,
                        accept: None,
                        precision: None,
                        snaps: Vec::new(),
                        on_change: ActionDescriptor { controller_id: "framework".into(), action: "setLayoutSaveLabel".into(), args: None },
                        on_submit: None,
                        on_abort: None,
                        on_repeat_last: None,
                        presence: UiPresence::default(),
                        menu: None,
                        ..Default::default()
                    })),
                    ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
                },
                UiTreeItemNode {
                    id: "framework.display.layout.save.action".into(),
                    label: Label::data(shell_chrome_string("common.save", is_de)),
                    control: Some(ui_wgpu::wgpu::component::ui::UiControlNode::Button(UiButtonNode {
                        id: Some("framework.display.save".into()),
                        icon_id: IconName::Save,
                        label: Label::data(shell_chrome_string("display.saveCurrentLayout", is_de)),
                        action: ActionDescriptor { controller_id: "framework".into(), action: "saveCurrentLayout".into(), args: None },
                        style: None,
                        presence: UiPresence { state: if self.layout_save_label.trim().is_empty() { ui_wgpu::wgpu::component::ui::UiState::Disabled } else { ui_wgpu::wgpu::component::ui::UiState::Normal }, ..UiPresence::default() },
                        menu: None,
                    })),
                    ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
                },
            ],
            window: None,
        };
        let mut items = named_layout_rows(&builtin, false, is_de);
        if !self.user_layouts.is_empty() {
            items.push(UiTreeItemNode {
                id: "framework.display.layout.group.saved".into(),
                label: Label::data(shell_chrome_string("display.saved", is_de)),
                default_open: Some(false),
                items: Some(named_layout_rows(&self.user_layouts, true, is_de)),
                ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
            });
        }
        let list_section = UiTreeSectionNode {
            header_toolbar: None,
            id: "framework.display.layout.list".into(),
            label: Some(Label::data(shell_chrome_string("display.layouts", is_de))),
            default_open: Some(true),
            presence: UiPresence::default(),
            items,
            window: None,
        };
        display_panel_body("framework.display.layout.panel", vec![save_section, list_section])
    }

    /// ⚖️ The `framework.settings.conflicts` leaf's body — the wgpu twin of React's
    /// `buildConflictsTree` (`📌️ChromePanels/🟦️.tsx`), now over a REAL projection: one row per OPEN
    /// conflict in [`ShellState::open_conflicts`], labelled `<kind> — <code> — <message>` off the
    /// conflict's own `ConflictKind` tag and worst `MutationMessage` (never parsed from prose), and
    /// React's own unavailable row only when there genuinely is no open conflict.
    ///
    /// ⚖️ React nests Accept/Discard as two inline buttons in the row's single control slot plus a
    /// `ConflictDiffPreview` behind `🔺️DiffViewHost`. Authored here as action-only nested leaves under
    /// React's own `….accept`/`….discard` ids; panel projection lifts that strip into one horizontal
    /// `Toolbar` of `Button`s so schema `rowCount` stays one. The selected row also owns a
    /// semantic DiffView Surface in its explicit detail relation; the empty local snapshot still
    /// produces React's additions-only preview of the incoming/degraded side.
    pub(crate) fn build_settings_conflicts_ui(&self) -> UiNode {
        let is_de = self.locale_id == "de";
        if self.open_conflicts.is_empty() {
            return display_panel_body(
                "framework.settings.conflicts.panel",
                vec![UiTreeSectionNode {
                    header_toolbar: None,
                    id: "framework.settings.conflicts.empty".into(),
                    label: Some(Label::data(shell_chrome_string("conflict.panel", is_de))),
                    default_open: Some(true),
                    presence: UiPresence::default(),
                    items: vec![UiTreeItemNode::base("framework.settings.conflicts.empty.row", Label::data(shell_chrome_string("settings.unavailable", is_de)))],
                    window: None,
                }],
            );
        }
        let items: Vec<UiTreeItemNode> = self
            .open_conflicts
            .iter()
            .map(|conflict| {
                let row_id = format!("framework.settings.conflicts.{}", conflict.id);
                let kind = shell_chrome_string(if conflict.quarantined { "conflict.quarantined" } else { "conflict.degraded" }, is_de);
                let selected = self.selected_conflict_id.as_deref() == Some(conflict.id.as_str());
                let detail = selected.then(|| {
                    let UiNode::ComponentScene(scene) = ui_wgpu::wgpu::build_diff_view_scene(
                        format!("{row_id}.preview"),
                        "framework",
                        ui_wgpu::wgpu::DiffViewScene { before: String::new(), after: conflict.preview_after.clone(), language: Some("json".into()), mode: Some("unified".into()), domain_id: Some(conflict.id.clone()) },
                    ) else {
                        unreachable!("DiffView builder returns a ComponentScene")
                    };
                    scene
                });
                let verb = |suffix: &str, label_key: &'static str, icon: IconName, accept: bool| UiTreeItemNode {
                    id: format!("{row_id}.{suffix}"),
                    label: Label::data(shell_chrome_string(label_key, is_de)),
                    icon_id: Some(icon),
                    action: Some(ActionDescriptor { controller_id: "framework".into(), action: "resolveConflict".into(), args: crate::action_args_json!({ "conflictId": conflict.id.clone(), "accept": accept }) }),
                    ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
                };
                UiTreeItemNode {
                    id: row_id.clone(),
                    label: Label::data(format!("{kind} \u{2014} {} \u{2014} {}", conflict.code, conflict.message)),
                    icon_id: Some(IconName::TriangleAlert),
                    default_open: None,
                    action: Some(ActionDescriptor { controller_id: "framework".into(), action: "selectConflict".into(), args: crate::action_args_json!({ "conflictId": conflict.id.clone() }) }),
                    items: Some(vec![verb("accept", "conflict.accept", IconName::Check, true), verb("discard", "conflict.discard", IconName::X, false)]),
                    detail,
                    presence: UiPresence { selected, ..UiPresence::default() },
                    ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
                }
            })
            .collect();
        display_panel_body(
            "framework.settings.conflicts.panel",
            vec![UiTreeSectionNode {
                header_toolbar: None,
                id: "framework.settings.conflicts.table".into(),
                label: Some(Label::data(shell_chrome_string("conflict.panel", is_de))),
                default_open: Some(true),
                presence: UiPresence::default(),
                items,
                window: None,
            }],
        )
    }

    /// 🛍️ Every plugin id the `framework.marketplace` leaf lists, in its row order: the plugins this shell HOLDS (`loaded`,
    /// extensions excluded — they nest under their host) and every plugin id the generated activation catalogue claims
    /// (`available`) — the same table [`ShellState::install_plugin`]'s lazy install reads, which is what makes `Install` a real
    /// verb rather than a decoration. Sorted and unique.
    pub(crate) fn marketplace_roster(&self) -> Vec<String> {
        let extension_ids: Vec<&str> = self.extensions.iter().map(|entry| entry.record.extension_id.as_str()).collect();
        let mut ids: Vec<String> = self.plugins.iter().filter(|entry| !extension_ids.contains(&entry.plugin_id.as_str())).map(|entry| entry.plugin_id.clone()).collect();
        for (_, plugin_id) in crate::program_bridge::plugin_artifact_kind_activations().iter() {
            if !ids.iter().any(|id| id.as_str() == *plugin_id) {
                ids.push((*plugin_id).to_string());
            }
        }
        ids.sort();
        ids.dedup();
        ids
    }

    /// 🛍️ The `framework.marketplace` leaf's body — the wgpu twin of React's `buildMarketplaceTree`: one section per plugin
    /// SOURCE, a `label · version · status` row per plugin, and this renderer's own install/reload/uninstall lane.
    ///
    /// 🎯️ Every row carries its verbs as ROW ACTIONS on ONE target (U6's row model): a plugin row targets
    /// `framework` with `{ pluginId }` and names `installPlugin` or `reloadPlugin`/`uninstallPlugin`; an extension row targets
    /// `{ extensionId, enabled: !enabled }` and names `setExtensionEnabled`/`uninstallExtension`. A verb the row cannot run
    /// paints and announces DISABLED (React's `disabled` buttons) instead of disappearing, and names why as its localized reason
    /// (the contract's `RowAction.reason`, announced as its description) — the session's own program is never uninstallable
    /// (React's `canUninstall`), a plugin mid-install offers nothing, a failed extension cannot be enabled.
    ///
    /// 🪟️ The roster is ONE windowed Tree section (`framework.marketplace.source.local`): the rows the tree window observer
    /// reported for it ([`tree_windows::TreeWindowScheduler::window_of`]) — one viewport of rows before the first report —
    /// materialise inside [`MARKETPLACE_WINDOW_NODE_BUDGET`], and the retained layout pitches the rest as spacers, so a roster of
    /// any length stays inside the retained node ceiling and scrolls to its last row. Store-owned extensions stay nested under
    /// their declared host, including failed loads, so install failures never erase inventory.
    pub(crate) fn build_marketplace_ui(&self) -> UiNode {
        let is_de = self.locale_id == "de";
        let session_plugin = self.session.as_ref().map(|session| session.plugin_id.clone());
        let ids = self.marketplace_roster();
        let installing = self.plugin_install.as_ref().map(|install| install.plugin_id.clone());
        let verb = |action: &str, args: Option<DslValue>, icon: IconName, label_key: &'static str, disabled_because: Option<&'static str>| ui_wgpu::wgpu::component::ui::UiTreeItemAction {
            icon_id: icon,
            label: Some(Label::data(shell_chrome_string(label_key, is_de))),
            action: ActionDescriptor { controller_id: "framework".into(), action: action.into(), args },
            placement: None,
            disabled: disabled_because.is_some(),
            reason: disabled_because.map(|reason_key| Label::data(shell_chrome_string(reason_key, is_de))),
        };
        let extension_item = |entry: &ShellExtensionProjection| {
            let extension_id = entry.record.extension_id.clone();
            let can_toggle = matches!(entry.load_status, ShellExtensionLoadStatus::Available | ShellExtensionLoadStatus::Loaded);
            let target = crate::action_args_json!({ "extensionId": extension_id.clone(), "enabled": !entry.enabled });
            UiTreeItemNode {
                id: format!("framework.marketplace.plugin.{}.extension.{extension_id}", entry.record.extends_host),
                label: Label::data(format!("{} · {} · {}", entry.record.label, entry.record.version, shell_chrome_string(if entry.enabled { "plugins.extension.enabled" } else { "plugins.extension.disabled" }, is_de))),
                actions: Some(vec![
                    verb("setExtensionEnabled", target.clone(), if entry.enabled { IconName::EyeOff } else { IconName::Eye }, if entry.enabled { "plugins.extension.disable" } else { "plugins.extension.enable" }, (!can_toggle).then_some("plugins.reason.extensionUnavailable")),
                    verb("uninstallExtension", target, IconName::Trash2, "plugins.action.uninstall", None),
                ]),
                ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
            }
        };
        let plugin_item = |plugin_id: &str| {
            let resident = self.plugins.iter().find(|entry| entry.plugin_id == plugin_id);
            let in_flight = installing.as_deref() == Some(plugin_id);
            let status_key = if in_flight {
                "plugins.status.installing"
            } else if resident.is_some() {
                "plugins.status.loaded"
            } else {
                "plugins.status.available"
            };
            let label = resident.map(|entry| entry.manifest.label.clone()).unwrap_or_else(|| plugin_id.to_string());
            let version = resident.map(|entry| entry.manifest.version.clone()).unwrap_or_default();
            let head = if version.is_empty() { format!("{label} \u{b7} {}", shell_chrome_string(status_key, is_de)) } else { format!("{label} \u{b7} {version} \u{b7} {}", shell_chrome_string(status_key, is_de)) };
            let target = crate::action_args_json!({ "pluginId": plugin_id });
            let installing_reason = in_flight.then_some("plugins.reason.installing");
            let uninstall_reason = installing_reason.or_else(|| (session_plugin.as_deref() == Some(plugin_id)).then_some("plugins.reason.inUse"));
            let actions = if resident.is_some() {
                vec![verb("reloadPlugin", target.clone(), IconName::RotateCcw, "plugins.action.reload", installing_reason), verb("uninstallPlugin", target, IconName::Trash2, "plugins.action.uninstall", uninstall_reason)]
            } else {
                vec![verb("installPlugin", target, IconName::Download, "plugins.action.install", installing_reason)]
            };
            let extensions: Vec<UiTreeItemNode> = self.extensions.iter().filter(|entry| entry.record.extends_host == plugin_id).map(&extension_item).collect();
            UiTreeItemNode {
                id: format!("framework.marketplace.plugin.{plugin_id}"),
                label: Label::data(head),
                default_open: Some(!extensions.is_empty()),
                presence: UiPresence { status: if in_flight { ui_wgpu::wgpu::component::ui::UiStatus::Loading } else { ui_wgpu::wgpu::component::ui::UiStatus::Idle }, ..UiPresence::default() },
                actions: Some(actions),
                items: (!extensions.is_empty()).then_some(extensions),
                ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
            }
        };
        let section_key = format!("{FRAMEWORK_MARKETPLACE_TAB_ID}/{MARKETPLACE_LOCAL_SECTION_ID}");
        let (offset, rows) = self.tree_windows.window_of(FRAMEWORK_MARKETPLACE_TAB_ID, &section_key).unwrap_or((0, self.tree_windows.viewport_rows_or_default()));
        let offset = (offset as usize).min(ids.len().saturating_sub(1));
        let mut credit = MARKETPLACE_WINDOW_NODE_BUDGET;
        let materialised: Vec<UiTreeItemNode> = ids
            .iter()
            .skip(offset)
            .take(rows as usize)
            .map(|plugin_id| plugin_item(plugin_id))
            .map_while(|item| {
                let cost = 1 + item.items.as_ref().map_or(0, Vec::len);
                (cost <= credit).then(|| {
                    credit -= cost;
                    item
                })
            })
            .collect();
        let (items, window) = if ids.is_empty() {
            (vec![UiTreeItemNode::base("framework.marketplace.empty", Label::data(shell_chrome_string("marketplace.unavailable", is_de)))], None)
        } else {
            (materialised, Some(UiTreeWindow { row_extent: UiTreeWindowRowExtent::Standard, total: ids.len() as u32, offset: offset as u32 }))
        };
        let mut missing_hosts: Vec<&str> = self.extensions.iter().map(|entry| entry.record.extends_host.as_str()).filter(|host| !ids.iter().any(|id| id.as_str() == *host)).collect();
        missing_hosts.sort();
        missing_hosts.dedup();
        let install_file = UiTreeItemNode {
            id: "framework.marketplace.extensions.install.file".into(),
            label: Label::data(shell_chrome_string("plugins.extension.installFromFile", is_de)),
            icon_id: Some(IconName::FileArchive),
            action: Some(ActionDescriptor { controller_id: "framework".into(), action: "installExtensionFile".into(), args: None }),
            ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
        };
        let install_url = UiTreeItemNode {
            id: "framework.marketplace.extensions.install.url".into(),
            label: Label::data(shell_chrome_string("plugins.extension.installFromUrl", is_de)),
            icon_id: Some(IconName::Download),
            action: Some(ActionDescriptor { controller_id: "framework".into(), action: "installExtensionUrl".into(), args: None }),
            ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
        };
        let mut sections = vec![
            UiTreeSectionNode {
                header_toolbar: None,
                id: "framework.marketplace.extensions.install".into(),
                label: Some(Label::data(shell_chrome_string("plugins.extension.install", is_de))),
                default_open: Some(true),
                presence: UiPresence::default(),
                items: vec![install_url, install_file],
                window: None,
            },
            UiTreeSectionNode {
                header_toolbar: None,
                id: MARKETPLACE_LOCAL_SECTION_ID.into(),
                label: Some(Label::data(format!("{}: local", shell_chrome_string("plugins.source", is_de)))),
                default_open: Some(true),
                presence: UiPresence::default(),
                items,
                window,
            },
        ];
        sections.extend(missing_hosts.into_iter().map(|host| UiTreeSectionNode {
            header_toolbar: None,
            id: format!("framework.marketplace.missing-host.{host}"),
            label: Some(Label::data(host.to_string())),
            default_open: Some(true),
            presence: UiPresence::default(),
            items: self.extensions.iter().filter(|entry| entry.record.extends_host.as_str() == host).map(&extension_item).collect(),
            window: None,
        }));
        display_panel_body("framework.marketplace.panel", sections)
    }

    /// 💬️ Sends the draft to the connected agent — React's `sendAgentMessage`: one
    /// `ShellToGateway::AgentMessage` queued on the bridge outbox plus the identical turn echoed into
    /// the conversation, so the feed is never behind the human's own typing. A blank draft, or a draft
    /// typed while no socket is open, queues nothing and KEEPS the text rather than swallowing it.
    pub(crate) fn send_agent_chat_draft(&mut self) -> bool {
        if !matches!(self.chrome_build.agent.status, crate::agent_bridge::AgentBridgeStatus::Open) {
            return false;
        }
        let trimmed = self.agent_chat_draft.trim().to_string();
        let session = self.shell_session_id.clone();
        if !self.chrome_build.agent.send_agent_message(&session, &trimmed) {
            return false;
        }
        self.agent_chat_draft.clear();
        true
    }

    /// 🛑️ Sends one cooperative cancellation for the gateway-owned invocation id and leaves the
    /// row in `cancelling` until its real result arrives. A disconnected shell changes nothing.
    pub(crate) fn cancel_agent_tool_call(&mut self, invocation_id: &str) -> bool {
        self.chrome_build.agent.cancel_tool_call(invocation_id)
    }

    /// 💬️ The `framework.chat` leaf's BODY — the LIVE agent conversation, the exact twin of React's
    /// `💬️AgentChatPanel/🟦️.tsx` feed: one row per `AgentConversationEntry` carrying a translated role
    /// noun and state chip, the tool name and its real arguments for a tool call, the result summary
    /// once it lands, and the parked approval's summary and decision. Nothing on this surface is
    /// generated locally — with no bridge attached the feed is empty and the composer is disabled,
    /// which is the honest state, not a simulated one. Entry ids mirror React's `entryElementId`.
    ///
    /// 🧾️ This is the packet's content-projection consumer: the panel body is assembled by the SHELL
    /// out of shell state rather than rendered by a guest, published through {@link panel_ui_records}
    /// like every other shell-owned leaf.
    ///
    /// 💬️ The FEED is its own node, under React's own id, rather than rows loose in the panel:
    /// that is what lets the transcript be addressed, labelled and paged as one thing — React's
    /// `<ol id="framework.chat.feed" aria-label={transcriptLabel} aria-live="polite">`.
    ///
    /// 📃️ React's feed SCROLLS and its effect pins it to the bottom on every new entry, so what
    /// the human sees is the newest window of the conversation. The wgpu panel body is a
    /// fixed-credit `MountedLayout` tree with no scroll of its own here, so the same window is
    /// produced by emitting the LAST `AGENT_CHAT_VISIBLE_ENTRIES` rows — the newest ones, in
    /// order. The bridge retains up to `AGENT_CONVERSATION_MAX_ENTRIES` (200) behind them, exactly
    /// as React's array does; nothing is discarded, only unpainted.
    ///
    /// ⌨️ React's composer submits on a bare Enter (Shift+Enter keeps the newline), which the
    /// retained input expresses as `commit: "enter"` plus an `on_submit` verb — the same
    /// `sendChatDraft` the Send button dispatches, so both routes are one code path.
    pub(crate) fn build_agent_chat_ui(&self) -> UiNode {
        let is_de = self.locale_id == "de";
        let locale = self.active_locale();
        let connected = matches!(self.chrome_build.agent.status, crate::agent_bridge::AgentBridgeStatus::Open);
        let mut children = Vec::new();
        let mut feed = Vec::new();
        if self.chrome_build.agent.conversation.is_empty() {
            feed.push(settings_text_row(shell_chrome_string("chat.empty", is_de)));
        }
        let conversation = &self.chrome_build.agent.conversation;
        let window = conversation.len().saturating_sub(AGENT_CHAT_VISIBLE_ENTRIES);
        for entry in conversation.iter().skip(window) {
            feed.push(agent_chat_entry_node(entry, is_de, locale));
        }
        children.push(UiNode::Stack(UiStackNode {
            direction: "column".into(),
            gap: None,
            padding: None,
            id: Some(FRAMEWORK_CHAT_FEED_ID.into()),
            children: feed,
            presence: UiPresence::default(),
            activate: None,
            drop_action: None,
            drop_overlay: None,
            menu: None,
        }));
        if !connected {
            children.push(settings_text_row(shell_chrome_string("chat.disconnected", is_de)));
        }
        children.push(UiNode::Input(UiInputNode {
            id: "framework.chat.draft".into(),
            input_kind: "longText".into(),
            value: self.agent_chat_draft.clone(),
            placeholder: Some(Label::data(shell_chrome_string("chat.placeholder", is_de))),
            accessibility_label: Some(Label::data(shell_chrome_string("chat.draftLabel", is_de))),
            commit: Some("enter".into()),
            min: None,
            max: None,
            step: None,
            accept: None,
            precision: None,
            snaps: Vec::new(),
            on_change: ActionDescriptor { controller_id: "framework".into(), action: "setChatDraft".into(), args: None },
            on_submit: Some(ActionDescriptor { controller_id: "framework".into(), action: "sendChatDraft".into(), args: None }),
            on_abort: None,
            on_repeat_last: None,
            presence: UiPresence { state: if connected { ui_wgpu::wgpu::component::ui::UiState::Normal } else { ui_wgpu::wgpu::component::ui::UiState::Disabled }, ..UiPresence::default() },
            menu: None,
            ..Default::default()
        }));
        children.push(UiNode::Button(UiButtonNode {
            id: Some("framework.chat.send".into()),
            icon_id: IconName::ArrowRight,
            label: Label::data(shell_chrome_string("chat.send", is_de)),
            action: ActionDescriptor { controller_id: "framework".into(), action: "sendChatDraft".into(), args: None },
            style: None,
            presence: UiPresence { state: if connected && !self.agent_chat_draft.trim().is_empty() { ui_wgpu::wgpu::component::ui::UiState::Normal } else { ui_wgpu::wgpu::component::ui::UiState::Disabled }, ..UiPresence::default() },
            menu: None,
        }));
        UiNode::Stack(UiStackNode { direction: "column".into(), gap: None, padding: None, id: Some("framework.chat.panel".into()), children, presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, menu: None })
    }

    /// 🔄️ The retained body behind the always-present Sync leaf. Its three choice actions are the
    /// existing `framework.sync` controller routes; the body only projects that authority into the dock.
    pub(crate) fn build_sync_attach_ui(&self) -> UiNode {
        let is_de = self.locale_id == "de";
        let active_kind = self.sync_backbone_uri.as_deref().and_then(|uri| {
            if uri.starts_with("file://") {
                Some("file")
            } else if uri.starts_with("folder://") {
                Some("folder")
            } else if uri.starts_with("remote://") {
                Some("remote")
            } else {
                None
            }
        });
        let mut children = Vec::new();
        for (kind, icon_id, label_key, action) in [("file", "file-json", "sync.file", "selectFile"), ("folder", "folder", "sync.folder", "selectFolder"), ("remote", "cloud", "sync.remote", "selectRemote")] {
            children.push(UiNode::Button(UiButtonNode {
                id: Some(format!("framework.sync.{kind}")),
                icon_id: IconName::from(icon_id),
                label: Label::data(shell_chrome_string(label_key, is_de)),
                action: ActionDescriptor { controller_id: "framework.sync".into(), action: action.into(), args: None },
                style: None,
                presence: UiPresence { selected: active_kind == Some(kind), ..UiPresence::default() },
                menu: None,
            }));
        }
        if let Some(kind) = self.sync_card_kind.as_deref().filter(|kind| matches!(*kind, "file" | "folder" | "remote")) {
            let placeholder = match kind {
                "remote" => "127.0.0.1:8787/studio-1/demo",
                "folder" => "/absolute/project/folder",
                _ => "/absolute/document.json",
            };
            children.push(UiNode::Input(UiInputNode {
                id: format!("framework.sync.{kind}.path"),
                input_kind: "text".into(),
                value: self.sync_card_draft.clone(),
                placeholder: Some(Label::data(placeholder)),
                accessibility_label: None,
                commit: None,
                min: None,
                max: None,
                step: None,
                accept: None,
                precision: None,
                snaps: Vec::new(),
                on_change: ActionDescriptor { controller_id: "framework.sync".into(), action: "setSyncDraft".into(), args: None },
                on_submit: Some(ActionDescriptor { controller_id: "framework.sync".into(), action: "attach".into(), args: crate::action_args_json!({ "kind": kind }) }),
                on_abort: None,
                on_repeat_last: None,
                presence: UiPresence::default(),
                menu: None,
                ..Default::default()
            }));
            if kind != "remote" {
                children.push(UiNode::Button(UiButtonNode {
                    id: Some("framework.sync.browse".into()),
                    icon_id: IconName::from("folder-open"),
                    label: Label::data(shell_chrome_string("sync.browse", is_de)),
                    action: ActionDescriptor { controller_id: "framework.sync".into(), action: "browse".into(), args: None },
                    style: None,
                    presence: UiPresence::default(),
                    menu: None,
                }));
            }
            children.push(UiNode::Button(UiButtonNode {
                id: Some("framework.sync.attach".into()),
                icon_id: IconName::from("link"),
                label: Label::data(shell_chrome_string("sync.attach", is_de)),
                action: ActionDescriptor { controller_id: "framework.sync".into(), action: "attach".into(), args: crate::action_args_json!({ "kind": kind }) },
                style: None,
                presence: UiPresence { state: if self.sync_card_draft.trim().is_empty() { ui_wgpu::wgpu::component::ui::UiState::Disabled } else { ui_wgpu::wgpu::component::ui::UiState::Normal }, ..UiPresence::default() },
                menu: None,
            }));
        }
        if let Some(uri) = self.sync_backbone_uri.as_deref() {
            children.push(UiNode::Stack(UiStackNode {
                direction: "column".into(),
                gap: None,
                padding: None,
                id: Some("framework.sync.active".into()),
                children: vec![settings_text_row(uri)],
                presence: UiPresence::default(),
                activate: None,
                drop_action: None,
                drop_overlay: None,
                menu: None,
            }));
            children.push(UiNode::Button(UiButtonNode {
                id: Some("framework.sync.detach".into()),
                icon_id: IconName::X,
                label: Label::data(shell_chrome_string("sync.detach", is_de)),
                action: ActionDescriptor { controller_id: "framework.sync".into(), action: "detach".into(), args: None },
                style: None,
                presence: UiPresence::default(),
                menu: None,
            }));
        }
        if let Some(line) = self.sync_link.text(is_de) {
            children.push(UiNode::Stack(UiStackNode {
                direction: "column".into(),
                gap: None,
                padding: None,
                id: Some(format!("framework.sync.link.{}", self.sync_link.code())),
                children: vec![settings_text_row(line)],
                presence: UiPresence::default(),
                activate: None,
                drop_action: None,
                drop_overlay: None,
                menu: None,
            }));
        }
        UiNode::Stack(UiStackNode { direction: "column".into(), gap: None, padding: None, id: Some("framework.sync.panel".into()), children, presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, menu: None })
    }

    /// 🧵️ The truthful Task Manager body for this target — React's `TaskManagerWindow`: the running tasks this shell owns
    /// (picked-file imports, [`ShellState::import_task_nodes`]) with their progress and a Cancel each, above the actors. WGPU
    /// has no actor-registry metrics or dispatch bridge, so the actors read React's distinct `no-runtime` state and publish no
    /// actor verbs.
    pub(crate) fn build_task_manager_ui(&self) -> UiNode {
        let is_de = self.locale_id == "de";
        UiNode::Stack(UiStackNode {
            direction: "column".into(),
            gap: None,
            padding: None,
            id: Some("os.task-manager.panel".into()),
            children: vec![self.import_task_nodes(), UiNode::Stack(UiStackNode {
                direction: "column".into(),
                gap: None,
                padding: None,
                id: Some("os.task-manager.no-runtime".into()),
                children: vec![settings_text_row(shell_chrome_string("taskManager.noRuntime", is_de))],
                presence: UiPresence::default(),
                activate: None,
                drop_action: None,
                drop_overlay: None,
                menu: None,
            })],
            presence: UiPresence::default(),
            activate: None,
            drop_action: None,
            drop_overlay: None,
            menu: None,
        })
    }

    /// 🛠️ One `tool.<id>` leaf's body — the wgpu twin of React's `buildToolTree`: a single headerless
    /// options section over the tool's own published measures. The leaf tab IS the activation control
    /// (see {@link ShellState::arm_tool_for_panel_tab}), so this renders no second activation toggle.
    /// The wgpu program bridge has no `tool_measures` reader yet, so the section carries the tool's
    /// armed state and stays otherwise empty exactly as React's does with no measures in hand.
    pub(crate) fn build_tool_panel_ui(&self, tool_id: &str) -> UiNode {
        let is_de = self.locale_id == "de";
        let armed = self.session.as_ref().and_then(|session| session.view_state.active_tool_id.as_deref()) == Some(tool_id);
        let items = self.tool_measures.get(tool_id).map(|measures| window_measure_tree_rows(measures)).unwrap_or_default();
        display_panel_body(
            &format!("{FRAMEWORK_TOOL_PANEL_TAB_PREFIX}{tool_id}.panel"),
            vec![UiTreeSectionNode {
                header_toolbar: None,
                id: format!("{FRAMEWORK_TOOL_PANEL_TAB_PREFIX}{tool_id}.options"),
                label: Some(Label::data(shell_chrome_string(if armed { "tool.armed" } else { "tool.idle" }, is_de))),
                default_open: Some(true),
                presence: UiPresence::default(),
                items,
                window: None,
            }],
        )
    }

    pub(crate) fn active_terminologies(&self) -> Vec<String> {
        let mut ids = vec!["native".to_string()];
        if let Some(session) = self.session.as_ref() {
            for id in &session.app.terminologies {
                if !ids.contains(id) {
                    ids.push(id.clone());
                }
            }
        }
        ids
    }

    //#region 🧭️PanelAnchorAccessors
    pub fn anchor_state(&self, anchor: PanelAnchor) -> &PanelAnchorState {
        &self.panel_anchors[anchor.index()]
    }

    pub fn anchor_state_mut(&mut self, anchor: PanelAnchor) -> &mut PanelAnchorState {
        &mut self.panel_anchors[anchor.index()]
    }

    /// 🧭️ Is this anchor actually painting — open, and carrying at least one tab to paint.
    pub fn anchor_open(&self, anchor: PanelAnchor) -> bool {
        self.anchor_state(anchor).visible && !self.dock_tabs.tabs(anchor).is_empty()
    }

    /// 🧭️ Every anchor this frame paints a floating panel at, in `PanelAnchor::ALL` order.
    pub fn open_anchors(&self) -> Vec<PanelAnchor> {
        PanelAnchor::ALL.into_iter().filter(|anchor| self.anchor_open(*anchor)).collect()
    }

    /// 🧭️ `⌘️B`'s left column and `⌘️⇧️B`'s right column — the two corner anchors that column carries,
    /// top first, the only place the former hardcoded left/right pair survives.
    pub(crate) fn column_anchors(left: bool) -> [PanelAnchor; 2] {
        if left {
            [PanelAnchor::TopLeft, PanelAnchor::BottomLeft]
        } else {
            [PanelAnchor::TopRight, PanelAnchor::BottomRight]
        }
    }

    /// 🧭️ Projection kept for the shell's column-level reads (`⌘️B`, the overlay safe area, the tutorial
    /// snapshot): the left column is open while either of its corner anchors is.
    pub fn left_panel_open(&self) -> bool {
        Self::column_anchors(true).into_iter().any(|anchor| self.anchor_open(anchor))
    }

    pub fn right_panel_open(&self) -> bool {
        Self::column_anchors(false).into_iter().any(|anchor| self.anchor_open(anchor))
    }

    /// 🧭️ Opens `anchor` at `tab_id`'s path, or folds it shut when that tab is already the open one —
    /// the toggle every panel-toggle control and every tab button shares (React's `PanelChromeTabBar`).
    pub fn toggle_anchor_tab(&mut self, anchor: PanelAnchor, tab_id: &str) {
        let already = self.anchor_open(anchor) && self.anchor_state(anchor).path.iter().any(|id| id == tab_id);
        if already {
            self.anchor_state_mut(anchor).visible = false;
            return;
        }
        let path = self.dock_tabs.locate(tab_id).filter(|(located, _)| *located == anchor).map(|(_, path)| self.dock_tabs.reconcile_path(anchor, &path)).unwrap_or_else(|| self.dock_tabs.default_path(anchor));
        let state = self.anchor_state_mut(anchor);
        state.path = path;
        state.visible = true;
    }

    /// 🧭️ Opens whichever anchor hosts `tab_id` today and selects it — the programmatic reveal every
    /// `setActivePanelTab` outcome, tour step and panel-tab hit funnels through.
    pub fn reveal_dock_tab(&mut self, tab_id: &str) -> Option<PanelAnchor> {
        let (anchor, path) = self.dock_tabs.locate(tab_id)?;
        let path = self.dock_tabs.reconcile_path(anchor, &path);
        let state = self.anchor_state_mut(anchor);
        state.path = path;
        state.visible = true;
        Some(anchor)
    }

    /// 🌳️ Is this anchor's branch folded shut in its tab bar.
    pub fn dock_branch_collapsed(&self, anchor: PanelAnchor, branch_id: &str) -> bool {
        self.dock_collapsed_branches.get(&format!("{}/{branch_id}", anchor.as_str())).copied().unwrap_or(false)
    }

    pub fn toggle_dock_branch(&mut self, anchor: PanelAnchor, branch_id: &str) {
        let key = format!("{}/{branch_id}", anchor.as_str());
        let next = !self.dock_collapsed_branches.get(&key).copied().unwrap_or(false);
        self.dock_collapsed_branches.insert(key, next);
    }

    /// 🧭️ The per-anchor projection the tutorial snapshot and the anchor-model tests read.
    pub fn panel_anchor_snapshot(&self, anchor: PanelAnchor) -> PanelAnchorSnapshot {
        let state = self.anchor_state(anchor);
        if !self.anchor_open(anchor) {
            return PanelAnchorSnapshot::default();
        }
        PanelAnchorSnapshot { visible: true, size: state.size, active_tab: state.active_tab().map(ToOwned::to_owned) }
    }

    /// 🧭️ The dock arrangement this session's sources declare, before any persisted override — the wgpu
    /// twin of React's `defaultDock` (`🏛️ShellHost/🟦️.tsx`'s 🧭️DockAssembly): app panel tabs routed to
    /// their `PanelGroup` corner, the framework Display branch and the sync leaf on bottom-left, the
    /// framework chat leaf on top-right, and the Tool/Command branches on bottom-middle.
    ///
    /// 🧭️ React's own bottom-left order (`🏛️ShellHost/🟦️.tsx`'s 🧭️DockAssembly): the FRAMEWORK
    /// Display branch first — its children are the framework's two display leaves
    /// (`createFrameworkDisplayPanelTabs`, `📌️ChromePanels/🟦️.tsx:249`), never the app's tabs —
    /// then the app's own bottom-left tabs as SIBLINGS of that branch, then the sync leaf. The
    /// branch used to be built out of the app's tabs and was therefore absent altogether for an
    /// app that declares none (puzzle3d), which is why the first real wgpu boot's footer showed no
    /// "Display" chip at all while React always shows one.
    ///
    /// ⚙️ Bottom-right carries ONE Settings branch (the app's own Settings-group tabs first, then the
    /// five framework leaves — React's `integrateAppSettingsPanelTabsIntoFrameworkBranch`, which
    /// re-orders the framework children past the app's), then the Marketplace leaf, then the
    /// framework-owned History leaf — the guest's own `framework.body.history`, which React's dock
    /// partitions into the same leaf (`partitionFrameworkHistoryPanelTab`).
    pub fn default_dock(&self) -> ShellDock {
        let mut dock = ShellDock::default();
        let is_de = self.locale_id == "de";
        let Some(session) = &self.session else { return dock };
        let terminology = self.active_terminology();
        let locale = self.active_locale();
        for tab in &session.app.panel_tabs {
            let anchor = PanelAnchor::from_group(tab.group);
            let order = dock.tabs(anchor).len() as i32;
            dock.tabs_mut(anchor).push(dock_tab_node_of(tab, terminology, locale, order));
        }
        let app_bottom_left: Vec<DockTabNode> = std::mem::take(dock.tabs_mut(PanelAnchor::BottomLeft));
        let mut bottom_left = vec![DockTabNode::branch(FRAMEWORK_CATEGORY_DISPLAY_ID, shell_chrome_string("panelToggle.display", is_de), "layout-grid", 0, Self::framework_display_tabs(is_de))];
        let offset = bottom_left.len() as i32;
        bottom_left.extend(app_bottom_left.into_iter().enumerate().map(|(index, tab)| DockTabNode { order: offset + index as i32, ..tab }));
        let order = bottom_left.len() as i32;
        bottom_left.push(DockTabNode::leaf(FRAMEWORK_SYNC_PANEL_TAB_ID, shell_sync_pill_text(self.sync_pill(), is_de), "refresh-cw", order));
        *dock.tabs_mut(PanelAnchor::BottomLeft) = bottom_left;
        let chat_order = dock.tabs(PanelAnchor::TopRight).len() as i32;
        dock.tabs_mut(PanelAnchor::TopRight).push(DockTabNode::leaf(FRAMEWORK_CHAT_PANEL_ID, shell_chrome_string("panelToggle.chat", is_de), "message-square", chat_order));
        let app_settings: Vec<DockTabNode> = std::mem::take(dock.tabs_mut(PanelAnchor::BottomRight));
        let (history, app_settings): (Vec<DockTabNode>, Vec<DockTabNode>) = app_settings.into_iter().partition(|tab| tab.id == FRAMEWORK_PANEL_TAB_HISTORY_ID);
        let mut settings_children = app_settings;
        let offset = settings_children.len() as i32;
        for (index, child) in self.framework_settings_tabs().into_iter().enumerate() {
            settings_children.push(DockTabNode { order: offset + index as i32, ..child });
        }
        let mut bottom_right = vec![DockTabNode::branch(FRAMEWORK_SETTINGS_PANEL_ID, shell_chrome_string("panelToggle.settings", is_de), "settings", 0, settings_children)];
        bottom_right.push(DockTabNode::leaf(FRAMEWORK_MARKETPLACE_TAB_ID, shell_chrome_string("marketplace.tab", is_de), "circle-dot", 1));
        bottom_right.push(DockTabNode::leaf(FRAMEWORK_TASK_MANAGER_PANEL_ID, shell_chrome_string("panelToggle.taskManager", is_de), "cpu", 2));
        let history_offset = bottom_right.len() as i32;
        bottom_right.extend(history.into_iter().enumerate().map(|(index, tab)| DockTabNode { order: history_offset + index as i32, ..tab }));
        *dock.tabs_mut(PanelAnchor::BottomRight) = bottom_right;
        let tool_tabs = self.tool_panel_tabs();
        if !tool_tabs.is_empty() {
            dock.tabs_mut(PanelAnchor::BottomMiddle).push(DockTabNode::branch(FRAMEWORK_CATEGORY_TOOL_ID, shell_chrome_string("panelToggle.tool", is_de), "hammer", 0, tool_tabs));
        }
        let command_tabs = self.command_category_tabs();
        if !command_tabs.is_empty() {
            dock.tabs_mut(PanelAnchor::BottomMiddle).push(DockTabNode::branch(FRAMEWORK_CATEGORY_COMMAND_ID, shell_chrome_string("panelToggle.command", is_de), "wrench", 1, command_tabs));
        }
        dock
    }

    /// 🖥️ The two framework Display leaves in React's own order — `framework.display.windows` then
    /// `framework.display.layout` (`createFrameworkDisplayPanelTabs`, `📌️ChromePanels/🟦️.tsx:249`,
    /// orders `-100`/`-99` so they always precede whatever an app adds). Framework-owned and
    /// session-independent, exactly like `framework_settings_tabs`, which is why the Display chip is
    /// unconditional chrome in both renderers.
    pub(crate) fn framework_display_tabs(is_de: bool) -> Vec<DockTabNode> {
        vec![DockTabNode::leaf(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID, shell_chrome_string("display.tab.windows", is_de), "layout-grid", 0), DockTabNode::leaf(FRAMEWORK_DISPLAY_LAYOUT_TAB_ID, shell_chrome_string("display.tab.layout", is_de), "layout", 1)]
    }

    /// ⚙️ The five framework Settings leaves in React's own order (`createFrameworkSettingsPanelTab`):
    /// general, theme, keybindings, default-apps, conflicts. The theme leaf is omitted outright when
    /// the boot descriptor locks the theme id — React's `locks.themeId` render gate, which removes the
    /// tab rather than disabling it.
    pub(crate) fn framework_settings_tabs(&self) -> Vec<DockTabNode> {
        let is_de = self.locale_id == "de";
        let mut tabs = vec![DockTabNode::leaf(FRAMEWORK_SETTINGS_GENERAL_TAB_ID, shell_chrome_string("settings.tab.general", is_de), "settings-2", 0)];
        if crate::boot_locks().theme_id.is_empty() {
            tabs.push(DockTabNode::leaf(FRAMEWORK_SETTINGS_THEME_TAB_ID, shell_chrome_string("settings.tab.theme", is_de), "paintbrush", 1));
        }
        tabs.push(DockTabNode::leaf(FRAMEWORK_SETTINGS_KEYBINDINGS_TAB_ID, shell_chrome_string("settings.tab.keybindings", is_de), "circle-dot", 2));
        tabs.push(DockTabNode::leaf(FRAMEWORK_SETTINGS_DEFAULT_APPS_TAB_ID, shell_chrome_string("settings.tab.defaultApps", is_de), "app-window", 3));
        tabs.push(DockTabNode::leaf(FRAMEWORK_SETTINGS_CONFLICTS_TAB_ID, shell_chrome_string("conflict.panel", is_de), "triangle-alert", 4));
        tabs
    }

    /// 🛠️ One `tool.<id>` leaf per tool the ACTIVE MODE declares, in declared order — React's
    /// `buildToolTabs` over `resolveModeTools(app, activeModeId)`. Label and icon come off the tool's
    /// own manifest entry through the live terminology and locale.
    pub(crate) fn tool_panel_tabs(&self) -> Vec<DockTabNode> {
        let Some(session) = self.session.as_ref() else { return Vec::new() };
        let terminology = self.active_terminology();
        let locale = self.active_locale();
        let mode_id = session.view_state.active_mode_id.clone().unwrap_or_else(|| session.app.default_mode_id.clone());
        semio_framework::manifest::resolve_mode_tools(&session.app, &mode_id)
            .into_iter()
            .enumerate()
            .map(|(order, tool)| DockTabNode::leaf(format!("{FRAMEWORK_TOOL_PANEL_TAB_PREFIX}{}", tool.id), tool.label.resolve(terminology, locale).to_string(), tool.icon_id.as_str(), order as i32))
            .collect()
    }

    /// 🎛️ One leaf per command category, the wgpu twin of React's `buildCommandCategoryTabs` — the
    /// bottom-middle Command branch's children, whose bodies `build_command_panel_ui` paints.
    pub(crate) fn command_category_tabs(&self) -> Vec<DockTabNode> {
        let resolved: Vec<_> = self.resolved_commands().into_iter().filter(|entry| entry.definition.in_palette).collect();
        command_categories(&resolved).into_iter().enumerate().map(|(order, (id, label))| DockTabNode::leaf(format!("{FRAMEWORK_COMMAND_CATEGORY_TAB_PREFIX}{id}"), label, "wrench", order as i32)).collect()
    }

    /// 📱️ All eight anchors' root tabs flattened into the single mobile panel's tab list, in
    /// `PanelAnchor::ALL` order, plus the synthetic `framework.mobile.app` leaf React appends when the
    /// navbar's example/mode/role clusters have no room — React's `mobilePanelTabs`
    /// (`🏛️ShellHost/🟦️.tsx`), which flattens `defaultDock`, not the override-applied dock.
    pub fn mobile_panel_tabs(&self) -> Vec<DockTabNode> {
        let is_de = self.locale_id == "de";
        let dock = self.default_dock();
        let mut tabs: Vec<DockTabNode> = PanelAnchor::ALL.into_iter().flat_map(|anchor| dock.tabs(anchor).to_vec()).collect();
        let has_example = !self.session_example_rows().is_empty() && crate::boot_locks().example_id.is_empty();
        let has_modes = self.session.as_ref().is_some_and(|session| session.app.modes.len() > 1);
        if has_example || has_modes {
            tabs.push(DockTabNode::leaf(FRAMEWORK_MOBILE_APP_TAB_ID, shell_chrome_string("mobilePanel.app", is_de), "circle-dot", 99));
        }
        tabs
    }

    /// 📱️ Below React's `UI_MOBILE_MEDIA_QUERY` breakpoint (`max-width: 767px`, the same
    /// `dock::MODE_DOCK_MOBILE_MAX_WIDTH_PX` the window dock reads) the eight anchors do not paint at
    /// all: one mobile panel carries every tab instead. Read off the LOGICAL viewport width W1g
    /// established, never the physical one.
    pub fn mobile_panel_active(&self) -> bool {
        self.screen_w <= crate::dock::MODE_DOCK_MOBILE_MAX_WIDTH_PX
    }

    /// 🧭️ Recomputes `dock_tabs` from the declared sources plus the persisted override, and reconciles
    /// every anchor's remembered path onto what still exists — the wgpu twin of the `applyDockSkeleton`
    /// memo plus React's per-anchor `reconcileActivePath` effects.
    pub fn sync_dock_tabs(&mut self) {
        let default_dock = self.default_dock();
        self.dock_tabs = apply_dock_skeleton(&default_dock, self.dock_override.as_ref());
        for anchor in PanelAnchor::ALL {
            if self.dock_tabs.tabs(anchor).is_empty() {
                let state = self.anchor_state_mut(anchor);
                state.path.clear();
                state.visible = false;
                continue;
            }
            let path = self.dock_tabs.reconcile_path(anchor, &self.anchor_state(anchor).path.clone());
            self.anchor_state_mut(anchor).path = path;
        }
    }

    /// 🎯️ Completes a tab drag onto `target`, persisting the arrangement whenever it stops matching the
    /// computed default — React's `handleTabDockDrop`.
    pub fn move_dock_tab(&mut self, tab_id: &str, target: &DockTabMoveTarget) -> bool {
        let mut dock = self.dock_tabs.clone();
        if !move_tab_in_dock(&mut dock, tab_id, target) {
            return false;
        }
        let next = dock_skeleton_of(&dock);
        let default_skeleton = dock_skeleton_of(&self.default_dock());
        self.dock_override = (!dock_skeletons_equal(Some(&next), Some(&default_skeleton))).then_some(next);
        self.sync_dock_tabs();
        self.reveal_dock_tab(tab_id);
        true
    }

    /// 🗄️ The app id whose per-app dock layer wins over the shared `os` one — React's `new DockLayoutStore(storage, session?.app.id)`.
    pub(crate) fn dock_store_app_id(&self) -> Option<String> {
        self.session.as_ref().map(|session| session.app.id.clone())
    }

    /// 🗄️ Flattens every anchor's chrome into the persisted shape, omitting whatever equals the default.
    ///
    /// 🗄️ Only what DIFFERS from the computed default is stored, React's `dockUiStateStore` rule —
    /// and every anchor's path is reconciled to its own default by `sync_dock_tabs`, so a path
    /// equal to that default is not a user choice and must not turn an untouched anchor into a
    /// persisted entry.
    pub fn dock_ui_snapshot(&self) -> DockUiState {
        let mut anchors = BTreeMap::new();
        for anchor in PanelAnchor::ALL {
            let state = self.anchor_state(anchor);
            let entry = DockUiPanelState {
                visible: state.visible.then_some(true),
                size: (state.size != DEFAULT_PANEL_WIDTH_PX).then_some(state.size),
                path: (!state.path.is_empty() && state.path != self.dock_tabs.default_path(anchor)).then(|| state.path.clone()),
            };
            if entry != DockUiPanelState::default() {
                anchors.insert(anchor.as_str().to_string(), entry);
            }
        }
        DockUiState { version: DOCK_SKELETON_VERSION, anchors }
    }

    /// 🗄️ Applies a persisted per-anchor chrome snapshot — used both on load and directly by tests.
    pub fn apply_dock_ui(&mut self, state: &DockUiState) {
        for anchor in PanelAnchor::ALL {
            let Some(entry) = state.anchors.get(anchor.as_str()) else {
                self.anchor_state_mut(anchor).visible = false;
                continue;
            };
            let target = self.anchor_state_mut(anchor);
            target.visible = entry.visible.unwrap_or(false);
            if let Some(size) = entry.size {
                target.size = size;
            }
            if let Some(path) = &entry.path {
                target.path = path.clone();
            }
        }
    }

    /// 🗄️ Persists the current dock chrome so it survives a reload — the twin of `persist_dock_layout`
    /// for the unrelated `dock`/Mode system.
    pub fn persist_dock_ui(&mut self) {
        let snapshot = self.dock_ui_snapshot();
        let app_id = self.dock_store_app_id();
        save_dock_ui_to_store(app_id.as_deref(), Some(&snapshot));
        self.chrome_present.last_persisted_dock_ui = Some(snapshot);
    }

    /// 🗄️ Persists the dock chrome only when it actually changed since the last persist/load — called
    /// once per frame from `render_chrome` (`ShellChrome`), the same dirty-checked render-loop hook
    /// `persist_ui_prefs_if_changed` (💾️PrefsSync) uses, so no `ui.panelToggle.*` arm has to persist itself.
    pub fn persist_dock_ui_if_changed(&mut self) {
        let current = self.dock_ui_snapshot();
        if self.chrome_present.last_persisted_dock_ui.as_ref() != Some(&current) {
            self.persist_dock_ui();
        }
        let skeleton = self.dock_override.clone();
        if self.chrome_present.last_persisted_dock_skeleton != Some(skeleton.clone()) {
            let app_id = self.dock_store_app_id();
            save_dock_skeleton_to_store(app_id.as_deref(), skeleton.as_ref());
            self.chrome_present.last_persisted_dock_skeleton = Some(skeleton);
        }
    }

    /// 🗄️ Loads the persisted arrangement and chrome and applies both — called once from `ShellState::new()`
    /// and again whenever the session's app changes, since the per-app layer is keyed by app id. Seeds the
    /// owned present-state snapshots so the first frame doesn't re-persist what was just read back.
    pub fn load_persisted_dock(&mut self) {
        let app_id = self.dock_store_app_id();
        self.dock_override = load_dock_skeleton_from_store(app_id.as_deref());
        self.sync_dock_tabs();
        if let Some(state) = load_dock_ui_from_store(app_id.as_deref()) {
            self.apply_dock_ui(&state);
            self.sync_dock_tabs();
        }
        self.chrome_present.last_persisted_dock_ui = Some(self.dock_ui_snapshot());
        self.chrome_present.last_persisted_dock_skeleton = Some(self.dock_override.clone());
        self.load_persisted_named_layouts();
    }

    /// 🖥️ Restores this app's SAVED layouts from the replayed `os.config.ui-preferences` log — React's Display
    /// host projection, re-run from [`Self::load_persisted_dock`] on every app change as React re-memoizes it on
    /// every focused-app change.
    pub fn load_persisted_named_layouts(&mut self) {
        let app_id = self.dock_store_app_id();
        self.user_layouts = load_named_layouts_from_store(app_id.as_deref());
    }

    /// 🖥️ Commits this app's saved-layout roster as `setNamedLayout` events — React's `onSaveUserLayout`/
    /// `onDeleteUserLayout`. Called by whatever mutates [`ShellState::user_layouts`].
    pub fn persist_named_layouts(&self) {
        let app_id = self.dock_store_app_id();
        save_named_layouts_to_store(app_id.as_deref(), &self.user_layouts);
    }

    /// 🖥️ The CURRENT dock as a saveable user layout — React's
    /// `createNamedLayout("user-" + Date.now(), label, currentLayout, "user")`
    /// (`📌️ChromePanels/🟦️.tsx`'s `saveCurrentLayout`), with the same `user-<millis>` id grammar so a
    /// layout saved on either renderer reads as a user layout on the other. `None` with no session,
    /// which is React's own `if (!options.currentLayout) return` gate.
    pub(crate) fn current_named_layout(&self, label: &str) -> Option<ui_wgpu::wgpu::NamedLayout> {
        self.session.as_ref()?;
        let stamp = Self::instant_now_ms().max(0.0) as u64;
        Some(ui_wgpu::wgpu::NamedLayout { id: format!("user-{stamp}"), label: label.to_string(), icon_id: None, layout: self.dock.to_window_layout(), origin: "user".into(), group_path: None })
    }

    /// 🖥️ Applies a builtin or saved layout by id — React's `applyNamedLayout`, which searches
    /// `[...builtinLayouts, ...userLayouts]` and calls `onApplyLayout` with what it finds. `false`
    /// when no layout carries that id, so the caller journals nothing for a stale row.
    pub(crate) fn apply_named_layout(&mut self, layout_id: &str) -> bool {
        let Some(layout) = self.named_layout_by_id(layout_id) else { return false };
        self.layout_override = Some(layout);
        self.sync_dock();
        self.active_window_id = self.dock.active_window_id.clone();
        true
    }

    /// 🖥️ Opens one more instance of `window_kind_id`, split right of the active stack — the same
    /// placement [`ShellState::open_active_window_in_new_window`] uses for `mod+shift+n`, which is
    /// React's `onWindowOpenInNewWindow`. A `template_id` seeds that instance's projection exactly as
    /// React's dropped `{windowKindId, templateId}` payload does.
    pub(crate) fn open_display_window(&mut self, window_kind_id: &str, template_id: Option<&str>) -> bool {
        let Some(session) = self.session.as_ref() else { return false };
        let Some(body_key) = session.app.window_kinds.iter().find(|kind| kind.id == window_kind_id).map(|kind| kind.body_key.clone()) else {
            return false;
        };
        if !self.window_topology_publication_has_credit() {
            return false;
        }
        let instance_id = self.allocate_window_instance_id(window_kind_id);
        let anchor = self.active_window_id.clone().or_else(|| self.dock.collect_window_ids().first().cloned());
        let zone = anchor
            .as_deref()
            .and_then(|window_id| self.dock_stack_path_for(window_id))
            .map(|stack_path| DockDropZone::Split { stack_path, side: crate::dock::DockSide::Right })
            .unwrap_or(DockDropZone::RootSplit { side: crate::dock::DockSide::Right });
        let payload = DockDragPayload {
            kind: DockDragKind::NewWindow,
            window_id: instance_id.clone(),
            window_kind_id: window_kind_id.to_string(),
            template_id: template_id.map(str::to_string),
            source_path: Vec::new(),
            tab_index: 0,
            ghost_label: window_kind_id.to_string(),
        };
        if !self.dock.apply_drop(&payload, &zone) {
            return false;
        }
        self.seed_new_window_projection(&instance_id, template_id);
        self.active_window_id = Some(instance_id.clone());
        self.persist_dock_layout();
        self.commit_window_topology_publication(&instance_id, &body_key, None);
        true
    }

    /// 🏷️ Seeds a newly transferred window's projection chrome, as React's ShellHost drop handler does.
    pub(crate) fn seed_new_window_projection(&mut self, window_id: &str, template_id: Option<&str>) {
        let Some(template_id) = template_id else { return };
        self.world_projection_template.insert(window_id.to_string(), template_id.to_string());
        let Some(selected) = world_projection_template_selection_id(template_id) else { return };
        let effective = match selected { "parallel" => "orthographic", "axonometric" => "axonometric-isometric", "oblique" => "oblique-cavalier", "perspective" => "three-point", other => other };
        let template = world_projection_template(effective);
        let _ = self.apply_window_title_host_command(window_id, template.label);
        let _ = self.apply_window_icon_host_command(window_id, template.icon_id);
    }

    /// 🔌️ Drops a resident plugin — React's `uninstallPlugin`. The session's OWN program is never
    /// dropped (React's `canUninstall`), because a shell may not unload the program it is running.
    pub(crate) fn uninstall_plugin(&mut self, plugin_id: &str) -> bool {
        if self.session.as_ref().is_some_and(|session| session.plugin_id == plugin_id) {
            return false;
        }
        let before = self.plugins.len();
        #[cfg(not(target_arch="wasm32"))]
        if self.inference_port.as_ref().is_some_and(|runner|runner.owner==plugin_id) {self.cancel_inference_port();}
        self.plugins.retain(|entry| entry.plugin_id != plugin_id);
        self.plugins.len() != before
    }

    /// ⚔️ React's `conflictKindLabel`/`conflictMessageText` reduction of one wire conflict into the
    /// row the Conflicts leaf paints — the kind's own tag, and the WORST message's code and text
    /// (`conflict.messages[0]`, React's own pick), never prose-parsed.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn conflict_rows(conflicts: &[protocol::Conflict]) -> Vec<ShellConflictRow> {
        conflicts
            .iter()
            .filter(|conflict| conflict.status == protocol::ConflictStatus::Open)
            .map(|conflict| {
                let worst = conflict.messages.first();
                ShellConflictRow {
                    id: conflict.id.0.clone(),
                    quarantined: matches!(conflict.kind, protocol::ConflictKind::Quarantined { .. }),
                    code: worst.map(|message| message.code.0.clone()).unwrap_or_default(),
                    message: worst.map(|message| message.message.clone()).unwrap_or_default(),
                    preview_after: match &conflict.kind {
                        protocol::ConflictKind::Quarantined { envelopes } => {
                            let compact = semio_framework_pack_json::to_json_string(envelopes);
                            serde_json::from_str::<serde_json::Value>(&compact).ok().and_then(|value| serde_json::to_string_pretty(&value).ok()).unwrap_or(compact)
                        }
                        protocol::ConflictKind::Degraded { edit_ids } => format!("// degraded edits: {}", edit_ids.join(", ")),
                    },
                }
            })
            .collect()
    }

    /// ⚔️ Seeds the Conflicts leaf's roster once per session — React's `AppCommand::ReadConflicts`
    /// bootstrap (`🏛️ShellHost/🟦️.tsx`: "otherwise the panel would only ever show conflicts a later
    /// reply happened to carry"). Best-effort and native-only, the same posture — and the same
    /// backend limit — [`ShellState::refresh_history_snapshot`] already takes.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) async fn seed_open_conflicts(&mut self) {
        if self.conflicts_seeded {
            return;
        }
        let Some(session) = self.session.as_ref() else { return };
        let Some(plugin) = self.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).cloned() else { return };
        let instance_id = session.instance_id;
        self.conflicts_seeded = true;
        match plugin.read_conflicts(instance_id).await {
            Ok(conflicts) => self.open_conflicts = Self::conflict_rows(&conflicts),
            Err(error) => { let detail = self.note_refused_open(error); Self::debug_log(&format!("[TRACE] wgpu shell read_conflicts failed: {detail}")); },
        }
    }

    /// ⚔️ Accept/discard one conflict and adopt the roster the guest answers with — React's
    /// `dispatchResolveConflict`, whose reply carries the surviving conflicts so the row leaves
    /// without a second read.
    pub(crate) async fn resolve_open_conflict(&mut self, conflict_id: &str, accept: bool) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some((plugin, instance_id)) = self.session.as_ref().and_then(|session| self.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).cloned().map(|plugin| (plugin, session.instance_id))) {
                match plugin.resolve_conflict(instance_id, conflict_id, accept).await {
                    Ok(conflicts) => self.open_conflicts = Self::conflict_rows(&conflicts),
                    Err(error) => { let detail = self.note_refused_open(error); Self::debug_log(&format!("[TRACE] wgpu shell resolve_conflict failed: {detail}")); },
                }
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = accept;
            self.open_conflicts.retain(|row| row.id != conflict_id);
        }
        if self.selected_conflict_id.as_deref() == Some(conflict_id) {
            self.selected_conflict_id = None;
        }
    }

    /// 🖥️ The layout `layout_id` names, searching the app's declared layouts and then this app's saved
    /// ones — React's `[...builtinLayouts, ...userLayouts].find(…)`.
    pub(crate) fn named_layout_by_id(&self, layout_id: &str) -> Option<ui_wgpu::wgpu::WindowLayout> {
        if let Some(named) = self.session.as_ref().and_then(|session| session.app.named_layouts.iter().find(|entry| entry.id == layout_id)) {
            return Some(named.layout.clone());
        }
        self.user_layouts.iter().find(|entry| entry.id == layout_id).map(|entry| entry.layout.clone())
    }
    //#endregion 🧭️PanelAnchorAccessors
}

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧪️tests/🖥️wgpu-display-conflicts-marketplace/🦀️.rs"]
mod display_conflicts_marketplace_tests;
//#endregion ShellLifecycle
