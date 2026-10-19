//! ⌨️ Input dispatch: event dispatch and routing, key sanitising and dispatch, the modal gate and the click counting. See [`super`].

use super::*;

impl Tui {
    /// 🎹 Routes one input event: keys to the focused widget, pointer events through hit-test, hover, capture and drag state.
    pub fn dispatch(&mut self, event: &Event) -> Signals {
        self.dispatch_at(event, self.now_ms)
    }

    /// 🕰️ Like `dispatch` with an explicit clock reading, so click counting and tooltip dwell are deterministic.
    pub fn dispatch_at(&mut self, event: &Event, now_ms: u64) -> Signals {
        self.now_ms = self.now_ms.max(now_ms);
        self.ticked |= now_ms > 0;
        self.sanitize();
        let mut out = Vec::new();
        match event {
            Event::Resize(size) => {
                self.resize(*size);
                self.set_hover_node(None);
            }
            Event::Key(key) => self.dispatch_key(key, &mut out),
            Event::Paste(text) => {
                if let Some(id) = self.focus() {
                    let signal = match &mut self.scene.node_raw_mut(id).content {
                        NodeContent::Widget(widget) => widget.on_paste(text),
                        _ => None,
                    };
                    self.scene.mark_dirty(id, PAINT_DIRTY);
                    out.extend(signal.map(|signal| (id, signal)));
                }
            }
            Event::Mouse(mouse) => {
                self.ensure_layout();
                self.dispatch_mouse(mouse, &mut out);
            }
            Event::FocusLost => {
                self.set_hover_node(None);
                self.cancel_pointer();
                self.app_focused = false;
                if let Some(id) = self.focus() {
                    self.notify_focus(id, false);
                }
            }
            Event::FocusGained => {
                self.app_focused = true;
                if let Some(id) = self.focus() {
                    self.notify_focus(id, true);
                }
            }
            Event::Wake => {}
        }
        let mut delivered = std::mem::take(&mut self.pending);
        delivered.extend(out);
        delivered
    }

    pub(super) fn sanitize(&mut self) {
        if self.focus.is_some_and(|id| !self.scene.contains(id)) {
            self.focus = None;
        }
        if self.hovered.is_some_and(|id| !self.scene.contains(id)) {
            self.hovered = None;
        }
        if self.capture.is_some_and(|id| !self.scene.contains(id)) {
            self.capture = None;
        }
        let stale = match &self.press {
            Some(Press::Widget { node, .. }) => !self.scene.contains(*node),
            Some(Press::Splitter { root, path, index, .. }) => axis_at(&self.scene, *root, path).is_none_or(|axis| self.scene.node(axis).children().len() <= index + 1),
            Some(Press::Tab { chrome, source, .. }) => !self.scene.contains(*chrome) || !matches!(&self.scene.node(*chrome).content, NodeContent::Chrome(ChromeState::Window(window)) if window.tab_ids.contains(source)),
            None => false,
        };
        if stale {
            self.cancel_pointer();
        }
        let gone: Vec<NodeId> = self.overlays.iter().filter(|overlay| !self.scene.contains(overlay.id)).map(|overlay| overlay.id).collect();
        for id in gone {
            self.overlays.retain(|overlay| overlay.id != id);
            if self.tooltip == Some(id) {
                self.tooltip = None;
            }
        }
    }

    fn dispatch_key(&mut self, key: &KeyEvent, out: &mut Signals) {
                if key.key == Key::Esc && self.dragging_window() && self.cancel_pointer() { return; }
self.hide_tooltip();
        let target = self.focus();
        let claims = target.is_some_and(|id| matches!(&self.scene.node(id).content, NodeContent::Widget(widget) if widget.claims_tab()));
        if matches!(key.key, Key::Tab | Key::BackTab) && !claims {
            self.step_focus(key.key == Key::Tab, out);
            return;
        }
        let Some(id) = target else { return };
        let signal = match &mut self.scene.node_raw_mut(id).content {
            NodeContent::Widget(widget) => widget.on_key(key),
            _ => None,
        };
        self.scene.mark_dirty(id, PAINT_DIRTY);
        self.overlay_signals(id, &signal, Some(key), out);
        out.extend(signal.map(|signal| (id, signal)));
    }

    pub(super) fn widget_at(&self, hit: NodeId) -> Option<NodeId> {
        self.scene.lineage(hit).into_iter().find(|&node| matches!(self.scene.node(node).content, NodeContent::Widget(_)))
    }

    pub(super) fn route(&mut self, id: NodeId, mouse: &MouseEvent) -> Option<WidgetSignal> {
        let rect = self.scene.try_node(id)?.rect;
        let signal = match &mut self.scene.node_raw_mut(id).content {
            NodeContent::Widget(widget) => widget.on_mouse(rect, mouse),
            _ => None,
        };
        self.scene.mark_dirty(id, PAINT_DIRTY);
        signal
    }

    pub(super) fn inside_modal(&self, hit: Option<NodeId>) -> bool {
        match (self.modal(), hit) {
            (None, _) => true,
            (Some(modal), Some(hit)) => self.scene.lineage(hit).contains(&modal.id),
            (Some(_), None) => false,
        }
    }

    pub(super) fn count_clicks(&mut self, mouse: &MouseEvent) -> MouseEvent {
        let MouseKind::Down(button) = mouse.kind else {
            if let Some(Press::Widget { clicks, .. }) = &self.press {
                return MouseEvent { clicks: *clicks, ..*mouse };
            }
            return *mouse;
        };
        let count = if mouse.clicks > 1 {
            mouse.clicks
        } else if let (true, Some(last)) = (self.ticked, &self.last_click) {
            if last.button == button && last.pos == mouse.pos && self.now_ms.saturating_sub(last.at_ms) <= CLICK_WINDOW_MS {
                last.count % 3 + 1
            } else {
                1
            }
        } else {
            1
        };
        self.last_click = Some(Click { button, pos: mouse.pos, at_ms: self.now_ms, count });
        MouseEvent { clicks: count, ..*mouse }
    }

}
