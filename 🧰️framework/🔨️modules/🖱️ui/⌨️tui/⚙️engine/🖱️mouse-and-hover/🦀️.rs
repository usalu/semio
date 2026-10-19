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
                self.cancel_pointer();
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
                    let path = axis.path.clone();
                    let root_axis = self.scene.lineage(hit).into_iter().find(|node| matches!(&self.scene.node(*node).content, NodeContent::Axis(axis) if axis.path.is_empty()));
                    let root = root_axis.and_then(|node| self.scene.node(node).parent()).and_then(|node| self.scene.node(node).parent()).unwrap_or(self.scene.root());
                    self.press = Some(Press::Splitter { root, path, index, last });
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
                                let source = match &self.scene.node(node).content {
                    NodeContent::Chrome(ChromeState::Window(window)) => window.tab_ids.get(i).cloned(),
                    _ => None,
                };
let target = self.focus_target(node);
                self.focus_node(target, out);
                out.push((node, WidgetSignal::WindowTabActivated(i)));
                if let Some(source) = source.filter(|source| !source.is_empty()) {
                    self.press = Some(Press::Tab { chrome: node, source, start: mouse.pos, pos: mouse.pos, moved: false });
                }
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
            Some(Press::Splitter { root, path, index, last }) if button == MouseButton::Left => {
                let mut next = last;
                                let Some(axis) = axis_at(&self.scene, root, &path) else { return };
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
                self.press = Some(Press::Splitter { root, path, index, last: next });
            }
            Some(Press::Tab { chrome, source, start, moved, .. }) if button == MouseButton::Left => {
                let moved = moved || u32::from(mouse.pos.x.abs_diff(start.x)) + u32::from(mouse.pos.y.abs_diff(start.y)) >= 2;
                let target = hit.and_then(|node| self.window_of(node));
                self.dock_preview = if moved { target.and_then(|node| self.propose_dock(node, chrome, &source, mouse.pos)) } else { None };
                if let Some(preview) = &self.dock_preview {
                    out.push((chrome, WidgetSignal::WindowDockPreview { source: source.clone(), target: preview.target.clone(), placement: preview.placement, tab_index: preview.tab_index }));
                }
                self.clear_tab_drops();
                self.scene.mark_dirty(self.scene.root(), PAINT_DIRTY);
                self.press = Some(Press::Tab { chrome, source, start, pos: mouse.pos, moved });
            }
            other => {
                self.press = other;
                self.mouse_move(mouse, hit, out);
            }
        }
    }

    fn mouse_up(&mut self, mouse: &MouseEvent, button: MouseButton, out: &mut Signals) {
                if matches!(&self.press, Some(Press::Tab { .. })) && button != MouseButton::Left { return; }
        if matches!(&self.press, Some(Press::Tab { moved: true, .. })) {
            self.mouse_drag(mouse, button, self.scene.hit(mouse.pos), out);
        }
match self.press.take() {
            Some(Press::Widget { node, button: held, clicks }) => {
                if held == button {
                    out.extend(self.route(node, mouse).map(|signal| (node, signal)));
                } else {
                    self.press = Some(Press::Widget { node, button: held, clicks });
                }
            }
            Some(Press::Tab { chrome, source, .. }) => {
                self.clear_tab_drops();
                self.scene.mark_dirty(chrome, PAINT_DIRTY);
                if let Some(preview) = self.dock_preview.take() {
                    out.push((chrome, WidgetSignal::WindowDocked { source, target: preview.target, placement: preview.placement, tab_index: preview.tab_index }));
                }
                self.scene.mark_dirty(self.scene.root(), PAINT_DIRTY);
            }
            Some(Press::Splitter { .. }) | None => {}
        }
        }

    fn propose_dock(&self, chrome: NodeId, held_chrome: NodeId, source: &str, pos: Pos) -> Option<DockPreview> {
        let NodeContent::Chrome(ChromeState::Window(window)) = &self.scene.node(chrome).content else { return None };
        let rect = self.scene.rect(chrome);
        let NodeContent::Chrome(state) = &self.scene.node(chrome).content else { return None };
        let corner = state.window_drop_corner(rect, pos);
        let placement = if let Some(corner) = corner { DockPlacement::Corner(corner) } else { DockPlacement::at(rect, pos)? };
        let target = window.tab_ids.get(window.active_stack_tab).filter(|id| id.as_str() != source).or_else(|| window.tab_ids.iter().find(|id| id.as_str() != source)).or_else(|| matches!(placement, DockPlacement::Corner(_)).then(|| window.tab_ids.iter().find(|id| id.as_str() == source)).flatten())?.clone();
        let tab_index = corner.map(|corner| state.window_corner_insert_index(rect, pos, corner, source));
        let mounted = self.scene.window_layouts.iter().find(|mounted| self.scene.contains(mounted.canvas) && mounted.windows.iter().any(|(_, node)| *node == chrome) && mounted.windows.iter().any(|(_, node)| *node == held_chrome))?;
        let mut projected = mounted.layout.clone();
        if !crate::tui::layout::dock_window(&mut projected, source, &target, placement, tab_index) { return None; }
        let window_rect = crate::tui::layout::solve_window_layout(&projected, self.scene.rect(mounted.canvas)).into_iter().find(|measure| measure.window_kind_id == source)?.rect;
        let source_node = mounted.windows.iter().find(|(id, _)| id == source)?.1;
        let NodeContent::Chrome(ChromeState::Window(source_state)) = &self.scene.try_node(source_node)?.content else { return None };
        let mut window = (**source_state).clone();
        let stack = crate::tui::layout::stack_hosting(&projected, source)?;
        window.stack_tabs = stack.children.iter().map(|child| WindowStackTabState {
            label: child.title.clone().unwrap_or_else(|| child.window_kind_id.clone()),
            corner: child.corner.unwrap_or_default(),
            status: mounted.windows.iter().find(|(id, _)| id == &child.window_kind_id).and_then(|(_, node)| self.scene.try_node(*node)).and_then(|node| match &node.content { NodeContent::Chrome(ChromeState::Window(state)) => state.status, _ => None }),
        }).collect();
        window.tab_ids = stack.children.iter().map(|child| child.window_kind_id.clone()).collect();
        window.active_stack_tab = window.tab_ids.iter().position(|id| id == source)?;
        window.zoomed = projected.zoomed.as_deref() == Some(source);
        window.hover = None;
        window.hover_drag = None;
        window.drop_target = None;
        let rect = window_tab_rect(&window, window_rect, window.active_stack_tab)?;
        Some(DockPreview { target, placement, tab_index, rect, window_rect, window })
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
                if node.is_none() {
            self.emphasize_chrome(None);
            self.hover_pos = None;
        }
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
                self.emphasize_chrome(hit);
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

    pub(super) fn emphasize_chrome(&mut self, hit: Option<NodeId>) {
        let lineage = hit.map(|id| self.scene.lineage(id)).unwrap_or_default();
        let chrome: Vec<_> = lineage.into_iter().filter(|&id| matches!(self.scene.node(id).content, NodeContent::Chrome(ChromeState::Window(_) | ChromeState::Navbar(_) | ChromeState::Footer(_)))).collect();
        if self.hovered_chrome == chrome {
            return;
        }
        for &id in self.hovered_chrome.iter().chain(&chrome) {
            if self.scene.contains(id) && self.hovered_chrome.contains(&id) != chrome.contains(&id) {
                self.scene.mark_dirty(id, PAINT_DIRTY);
            }
        }
        self.hovered_chrome = chrome;
    }

}
