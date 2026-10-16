//! 🕹️ `shell-input` staging of `session-and-topology`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellLifecycle
impl ShellInputState {
    //#region 🏷️LabelResolution
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

    //#endregion 🏷️LabelResolution
    pub(crate) fn context_window_instance_id(&self, x: f32, y: f32) -> Option<&str> {
        self.presented_input_geometry.dock_drop_bodies.iter().find(|(_, bounds, _)| bounds.contains(x, y)).map(|(_, _, window_id)| window_id.as_str())
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

    pub(crate) fn begin_pending_dock_drag(&mut self, pointer_id: ui_render::PointerId, payload: DockDragPayload, x: f32, y: f32, dock: &mut ShellDockState) -> bool {
        if !self.claim_chrome_pointer(pointer_id) {
            return false;
        }
        dock.pending_dock_drag = Some((payload, (x, y)));
        true
    }

    /// 🧳️ Bridges one semantic Tree transfer into the dock authority before retained routing takes
    /// ownership of the gesture. Both shell-painted and retained Tree hits use this one decoder.
    pub(crate) fn begin_window_template_drag_from_hit(&mut self, pointer_id: ui_render::PointerId, hit: &HitTarget<ActionDescriptor>, x: f32, y: f32, docs: &DocumentState, dock: &mut ShellDockState) -> bool {
        let Some((item_id, drag_data)) = hit.control_id.as_deref().and_then(tree_hit_item_id).zip(hit.drag_data.as_ref()) else {
            return false;
        };
        let Some(payload) = decode_window_template_drag(drag_data) else {
            return false;
        };
        if !docs.session.as_ref().is_some_and(|session| session.app.window_kinds.iter().any(|kind| kind.id == payload.window_kind_id)) {
            return false;
        }
        let window_id = dock.allocate_window_instance_id(&payload.window_kind_id);
        let ghost_label = item_id.rsplit('.').next().unwrap_or(item_id).to_string();
        self.begin_pending_dock_drag(pointer_id, DockDragPayload { kind: DockDragKind::NewWindow, window_id, window_kind_id: payload.window_kind_id, template_id: payload.template_id, source_path: Vec::new(), tab_index: 0, ghost_label }, x, y, dock)
    }
}

//#endregion ShellLifecycle

