//! 🖱️ Mouse and hover: mouse dispatch, move, down, drag and up handling, chrome presses and the hover target tracking. See [`super`].

use super::*;

impl Tui {
    pub(super) fn dispatch_mouse(&mut self, mouse: &MouseEvent, out: &mut Signals) {
        let mouse = self.count_clicks(mouse);
        let hit = self.scene.hit(mouse.pos);
        match mouse.kind {
            MouseKind::Move => self.mouse_move(&mouse, hit, out),
            MouseKind::Down(button) => self.mouse_down(&mouse, button, hit, out),
            MouseKind::Drag(button) => self.mouse_drag(&mouse, button, hit, out),
            MouseKind::Up(button) => self.mouse_up(&mouse, button, out),
            MouseKind::Scroll { .. } => {
                if !self.inside_modal(hit) {
                    return;
                }
                self.hide_tooltip();
                let target = self.capture.filter(|&id| self.scene.contains(id)).or_else(|| hit.and_then(|hit| self.widget_at(hit)));
                if let Some(id) = target {
                    out.extend(self.route(id, &mouse).map(|signal| (id, signal)));
                }
            }
        }
    }

    fn mouse_move(&mut self, mouse: &MouseEvent, hit: Option<NodeId>, out: &mut Signals) {
        let hit = if self.inside_modal(hit) { hit } else { None };
        self.update_hover(hit, Some(mouse.pos));
        let target = self.capture.filter(|&id| self.scene.contains(id)).or_else(|| hit.and_then(|hit| self.widget_at(hit)));
        if let Some(id) = target {
            out.extend(self.route(id, mouse).map(|signal| (id, signal)));
        }
    }

    fn mouse_down(&mut self, mouse: &MouseEvent, button: MouseButton, hit: Option<NodeId>, out: &mut Signals) {
        if !self.inside_modal(hit) {
            if let Some(modal) = self.modal() {
                let (id, dismiss) = (modal.id, modal.dismiss_outside);
                if dismiss {
                    out.push((id, WidgetSignal::Dismissed));
                    self.close_overlay(id);
                }
            }
            return;
        }
        let Some(hit) = hit else { return };
        self.hide_tooltip();
        if let Some(captured) = self.capture.filter(|&id| self.scene.contains(id)) {
            self.press = Some(Press::Widget { node: captured, button, clicks: mouse.clicks });
            out.extend(self.route(captured, mouse).map(|signal| (captured, signal)));
            return;
        }
        let is_window = matches!(self.scene.node(hit).content, NodeContent::Chrome(ChromeState::Window(_)));
        let is_axis = matches!(self.scene.node(hit).content, NodeContent::Axis(_));
        if is_window && self.chrome_down(hit, mouse, button, out) {
            return;
        }
        if button == MouseButton::Left && is_axis {
            let (rect, children) = (self.scene.node(hit).rect, self.scene.node(hit).children().to_vec());
            if let NodeContent::Axis(axis) = &self.scene.node(hit).content {
                if let Some(index) = gutter_at(&self.scene, axis, rect, &children, mouse.pos) {
                    let last = if axis.horizontal { mouse.pos.x } else { mouse.pos.y };
                    self.press = Some(Press::Splitter { axis: hit, index, last });
                    return;
                }
            }
        }
        let target = self.focus_target(hit);
        if target.is_some() {
            self.focus_node(target, out);
        }
        let widget = self.widget_at(hit);
        let mut signal = None;
        if let Some(id) = widget {
            self.press = Some(Press::Widget { node: id, button, clicks: mouse.clicks });
            signal = self.route(id, mouse);
            out.extend(signal.clone().map(|signal| (id, signal)));
            self.overlay_signals(id, &signal, None, out);
        }
        if button == MouseButton::Right && signal.is_none() {
            out.push((widget.unwrap_or(hit), WidgetSignal::ContextMenu { pos: mouse.pos, item: None }));
        }
    }

    fn chrome_down(&mut self, node: NodeId, mouse: &MouseEvent, button: MouseButton, out: &mut Signals) -> bool {
        let rect = self.scene.node(node).rect;
        let NodeContent::Chrome(chrome) = &self.scene.node(node).content else { return false };
        let Some(target) = chrome.window_target(rect, mouse.pos) else { return false };
        match (button, target) {
            (MouseButton::Left, WindowHit::Close(i)) => out.push((node, WidgetSignal::WindowClose(i))),
            (MouseButton::Left, WindowHit::Maximize(i)) => out.push((node, WidgetSignal::WindowMaximize(i))),
            (MouseButton::Left, WindowHit::NewTab(i)) => out.push((node, WidgetSignal::WindowNewTab(i))),
            (MouseButton::Left, WindowHit::OverflowPrev(i) | WindowHit::OverflowNext(i)) => out.push((node, WidgetSignal::WindowTabActivated(i))),
            (MouseButton::Left, WindowHit::Tab(i)) => {
                let target = self.focus_target(node);
                self.focus_node(target, out);
                out.push((node, WidgetSignal::WindowTabActivated(i)));
                self.press = Some(Press::Tab { chrome: node, from: i, over: i, start: mouse.pos, moved: false });
            }
            (MouseButton::Right, WindowHit::Tab(i)) => out.push((node, WidgetSignal::ContextMenu { pos: mouse.pos, item: Some(i) })),
            _ => {}
        }
        true
    }

    fn mouse_drag(&mut self, mouse: &MouseEvent, button: MouseButton, hit: Option<NodeId>, out: &mut Signals) {
        match self.press.take() {
            Some(Press::Widget { node, button: held, clicks }) if held == button => {
                self.press = Some(Press::Widget { node, button: held, clicks });
                out.extend(self.route(node, mouse).map(|signal| (node, signal)));
            }
            Some(Press::Splitter { axis, index, last }) => {
                let mut next = last;
                if let NodeContent::Axis(state) = &self.scene.node(axis).content {
                    let at = if state.horizontal { mouse.pos.x } else { mouse.pos.y };
                    let delta = i32::from(at) - i32::from(last);
                    if delta != 0 {
                        let mut path = state.path.clone();
                        path.push(index);
                        out.push((axis, WidgetSignal::SplitterDragged { path, delta: delta.clamp(i32::from(i16::MIN), i32::from(i16::MAX)) as i16 }));
                        next = at;
                    }
                }
                self.press = Some(Press::Splitter { axis, index, last: next });
            }
            Some(Press::Tab { chrome, from, over, start, moved }) => {
                let rect = self.scene.node(chrome).rect;
                let mut over = over;
                let mut moved = moved || mouse.pos != start;
                if let NodeContent::Chrome(state) = &self.scene.node(chrome).content {
                    if let Some(index) = state.window_drop_index(rect, mouse.pos) {
                        over = index;
                    }
                }
                if moved && over != from {
                    moved = true;
                }
                if let NodeContent::Chrome(state) = &mut self.scene.node_raw_mut(chrome).content {
                    state.set_drop_target((moved && over != from).then_some(over));
                }
                self.scene.mark_dirty(chrome, PAINT_DIRTY);
                self.press = Some(Press::Tab { chrome, from, over, start, moved });
            }
            other => {
                self.press = other;
                self.mouse_move(mouse, hit, out);
            }
        }
    }

    fn mouse_up(&mut self, mouse: &MouseEvent, button: MouseButton, out: &mut Signals) {
        match self.press.take() {
            Some(Press::Widget { node, button: held, clicks }) => {
                if held == button {
                    out.extend(self.route(node, mouse).map(|signal| (node, signal)));
                } else {
                    self.press = Some(Press::Widget { node, button: held, clicks });
                }
            }
            Some(Press::Tab { chrome, from, over, moved, .. }) => {
                if let NodeContent::Chrome(state) = &mut self.scene.node_raw_mut(chrome).content {
                    state.set_drop_target(None);
                }
                self.scene.mark_dirty(chrome, PAINT_DIRTY);
                if moved && over != from {
                    out.push((chrome, WidgetSignal::TabMoved { from, to: over }));
                }
            }
            Some(Press::Splitter { .. }) | None => {}
        }
    }

    fn hover_target(&self, hit: NodeId) -> Option<NodeId> {
        self.scene.lineage(hit).into_iter().find(|&node| matches!(self.scene.node(node).content, NodeContent::Widget(_) | NodeContent::Chrome(_) | NodeContent::Axis(_)))
    }

    fn apply_hover(&mut self, id: NodeId, pos: Option<Pos>) -> bool {
        let rect = self.scene.node(id).rect;
        let children = self.scene.node(id).children().to_vec();
        let changed = match &self.scene.node(id).content {
            NodeContent::Axis(axis) => {
                let hover = pos.and_then(|pos| gutter_at(&self.scene, axis, rect, &children, pos));
                let changed = axis.hover != hover;
                if changed {
                    if let NodeContent::Axis(axis) = &mut self.scene.node_raw_mut(id).content {
                        axis.hover = hover;
                    }
                }
                changed
            }
            NodeContent::Widget(_) => match &mut self.scene.node_raw_mut(id).content {
                NodeContent::Widget(widget) => widget.set_hover(rect, pos),
                _ => false,
            },
            NodeContent::Chrome(_) => match &mut self.scene.node_raw_mut(id).content {
                NodeContent::Chrome(chrome) => chrome.set_hover(rect, pos),
                _ => false,
            },
            _ => false,
        };
        if changed {
            self.scene.mark_dirty(id, PAINT_DIRTY);
        }
        changed
    }

    pub(super) fn set_hover_node(&mut self, node: Option<NodeId>) {
        if self.hovered == node {
            return;
        }
        if let Some(old) = self.hovered.filter(|&id| self.scene.contains(id)) {
            self.apply_hover(old, None);
        }
        self.hovered = node;
        self.hover_pos = None;
        self.hover_tip = None;
        self.hover_since_ms = self.now_ms;
        self.hide_tooltip();
    }

    fn update_hover(&mut self, hit: Option<NodeId>, pos: Option<Pos>) {
        let target = hit.and_then(|hit| self.hover_target(hit));
        self.set_hover_node(target);
        let (Some(id), Some(pos)) = (target, pos) else { return };
        self.apply_hover(id, Some(pos));
        let tip = self.scene.node(id).tooltip.clone().or_else(|| match &self.scene.node(id).content {
            NodeContent::Chrome(chrome) => chrome.tooltip(self.scene.node(id).rect, pos),
            _ => None,
        });
        if tip != self.hover_tip || (self.hover_pos != Some(pos) && self.tooltip.is_some()) {
            self.hide_tooltip();
            self.hover_since_ms = self.now_ms;
        }
        self.hover_tip = tip;
        self.hover_pos = Some(pos);
    }

}
