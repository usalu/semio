//! 🎯️ Construction and focus: the `Tui` constructor, frame size and width mode, the focus model and ring, the focus navigation and the cursor, hover and capture queries. See [`super`].

use super::*;

impl Tui {
    pub fn new(size: Size, theme: Theme) -> Self {
        let blank = Cell::blank([0, 0, 0], [0, 0, 0]);
        let mut front = CellBuffer::new(size, blank);
        let mut back = CellBuffer::new(size, blank);
        front.set_width_mode(WidthMode::Scalar);
        back.set_width_mode(WidthMode::Scalar);
        Self {
            scene: Scene::new(),
            theme,
            size,
            front,
            back,
            focus: None,
            hovered: None,
            capture: None,
            press: None,
            last_click: None,
            overlays: Vec::new(),
            now_ms: 0,
            ticked: false,
            last_render_ms: None,
            hover_pos: None,
            hover_since_ms: 0,
            hover_tip: None,
            tooltip: None,
            layout_stale: true,
            full_redraw: true,
            app_focused: true,
            pending: Vec::new(),
            animated: false,
            width_mode: WidthMode::Scalar,
        }
    }

    /// 🖼️ The last fully-composed frame, for hosts/tests that need to inspect the actual render.
    pub fn frame(&self) -> &CellBuffer {
        &self.front
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn resize(&mut self, size: Size) {
        self.size = size;
        let blank = Cell::blank([0, 0, 0], [0, 0, 0]);
        self.front.resize(size, blank);
        self.back.resize(size, blank);
        self.layout_stale = true;
        self.full_redraw = true;
    }

    /// 🔤️ Chooses how text is measured (clusters or scalars) for every buffer from the terminal's reported Unicode support; repaints in full.
    pub fn set_width_mode(&mut self, mode: WidthMode) {
        if self.width_mode == mode {
            return;
        }
        self.width_mode = mode;
        self.front.set_width_mode(mode);
        self.back.set_width_mode(mode);
        self.layout_stale = true;
        self.full_redraw = true;
    }

    pub fn width_mode(&self) -> WidthMode {
        self.width_mode
    }

    pub fn set_appearance(&mut self, appearance: AppearanceName) {
        self.theme.set_appearance(appearance);
        self.full_redraw = true;
    }

    /// 🎯 The node holding keyboard focus; `None` once it was removed or hidden.
    pub fn focus(&self) -> Option<NodeId> {
        self.focus.filter(|&id| self.scene.contains(id) && self.scene.shown(id))
    }

    /// 🔦 Moves keyboard focus to `id` (any live node); the engine is the only source of focus.
    /// While a modal overlay is open the request waits: the overlay keeps focus and `id` receives it when the overlay closes.
    pub fn set_focus(&mut self, id: Option<NodeId>) {
        let id = id.filter(|&id| self.scene.contains(id));
        if let Some(at) = self.overlays.iter().rposition(|overlay| overlay.modal) {
            let modal = self.overlays[at].id;
            if id.is_none_or(|node| !self.scene.lineage(node).contains(&modal)) {
                if id.is_some() {
                    self.overlays[at].saved_focus = id;
                }
                return;
            }
        }
        self.assign_focus(id);
    }

    pub(super) fn assign_focus(&mut self, id: Option<NodeId>) {
        if self.focus == id {
            return;
        }
        let old = self.focus;
        for node in [old, id].into_iter().flatten() {
            if self.scene.contains(node) {
                self.scene.mark_dirty(node, PAINT_DIRTY);
            }
        }
        self.focus = id;
        if let Some(old) = old {
            self.notify_focus(old, false);
        }
        if let (Some(new), true) = (id, self.app_focused) {
            self.notify_focus(new, true);
        }
    }

    /// 🔔️ Tells a widget it gained or lost keyboard focus; whatever it answers (a terminal's focus report) is delivered with the next dispatch result.
    pub(super) fn notify_focus(&mut self, id: NodeId, gained: bool) {
        if !self.scene.contains(id) {
            return;
        }
        let signal = match &mut self.scene.node_raw_mut(id).content {
            NodeContent::Widget(widget) => widget.on_focus(gained),
            _ => None,
        };
        self.pending.extend(signal.map(|signal| (id, signal)));
    }

    /// 🪟 The window chrome node that contains `id`.
    pub fn window_of(&self, id: NodeId) -> Option<NodeId> {
        self.scene.lineage(id).into_iter().find(|&node| matches!(self.scene.node(node).content, NodeContent::Chrome(ChromeState::Window(_))))
    }

    /// 🥇 The window that holds keyboard focus.
    pub fn focused_window(&self) -> Option<NodeId> {
        self.focus().and_then(|id| self.window_of(id))
    }

    pub(super) fn modal(&self) -> Option<&Overlay> {
        self.overlays.iter().rev().find(|overlay| overlay.modal)
    }

    /// 🔁 The focus ring: interactive, visible widgets in tree order; a modal overlay traps it.
    pub fn focus_ring(&self) -> Vec<NodeId> {
        let mut ring = Vec::new();
        match self.modal() {
            Some(overlay) => collect_focusables(&self.scene, overlay.id, &mut ring),
            None => collect_focusables(&self.scene, self.scene.root(), &mut ring),
        }
        ring
    }

    pub(super) fn step_focus(&mut self, forward: bool, out: &mut Signals) {
        let ring = self.focus_ring();
        if ring.is_empty() {
            return;
        }
        let current = self.focus().and_then(|focus| ring.iter().position(|node| *node == focus));
        let next = match (current, forward) {
            (Some(i), true) => (i + 1) % ring.len(),
            (Some(i), false) => (i + ring.len() - 1) % ring.len(),
            (None, true) => 0,
            (None, false) => ring.len() - 1,
        };
        self.focus_node(Some(ring[next]), out);
    }

    pub fn focus_next(&mut self) {
        self.step_focus(true, &mut Vec::new());
    }

    pub fn focus_prev(&mut self) {
        self.step_focus(false, &mut Vec::new());
    }

    /// ⏭️ Moves focus to the first widget of the next (`delta > 0`) or previous window.
    pub fn focus_window(&mut self, delta: i32) -> Signals {
        let mut windows = Vec::new();
        collect_windows(&self.scene, self.scene.root(), &mut windows);
        windows.retain(|&window| self.scene.shown(window));
        let mut out = Vec::new();
        if windows.is_empty() {
            return out;
        }
        let current = self.focused_window().and_then(|window| windows.iter().position(|w| *w == window));
        let next = match current {
            Some(i) => (i as i32 + delta).rem_euclid(windows.len() as i32) as usize,
            None => 0,
        };
        let mut ring = Vec::new();
        collect_focusables(&self.scene, windows[next], &mut ring);
        if let Some(&target) = ring.first() {
            self.focus_node(Some(target), &mut out);
        }
        out
    }

    pub(super) fn focus_node(&mut self, id: Option<NodeId>, out: &mut Signals) {
        let before = self.focused_window();
        self.set_focus(id);
        let after = self.focused_window();
        if after != before {
            if let Some(window) = after {
                out.push((window, WidgetSignal::WindowFocus));
            }
        }
    }

    pub(super) fn focus_target(&self, hit: NodeId) -> Option<NodeId> {
        let lineage = self.scene.lineage(hit);
        if let Some(&node) = lineage.iter().find(|&&node| focusable(&self.scene, node)) {
            return Some(node);
        }
        let window = lineage.iter().find(|&&node| matches!(self.scene.node(node).content, NodeContent::Chrome(ChromeState::Window(_))))?;
        let mut ring = Vec::new();
        collect_focusables(&self.scene, *window, &mut ring);
        ring.first().copied()
    }

    /// ✏️ The focused widget's terminal cursor, placed by its current rect.
    pub fn cursor(&self) -> Option<CursorSpec> {
        let node = self.scene.node(self.focus()?);
        let NodeContent::Widget(widget) = &node.content else { return None };
        widget.cursor(node.rect).filter(|spec| spec.pos.x < self.size.width && spec.pos.y < self.size.height)
    }

    /// 👆️ The node under the pointer.
    pub fn hovered(&self) -> Option<NodeId> {
        self.hovered.filter(|&id| self.scene.contains(id))
    }

    /// 🪤️ Names the node that owns the pointer until released with `None`.
    pub fn capture(&mut self, node: Option<NodeId>) {
        self.capture = node.filter(|&id| self.scene.contains(id));
    }

}
