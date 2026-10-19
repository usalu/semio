//! ⏱️ Frame, layout and overlays: the tick, the dirty and deadline queries, the layout and chrome sync, and the menu, dialog, palette and tooltip overlays with their placement. See [`super`].

use super::*;

impl Tui {
    /// ⏰ Advances the clock to `now_ms` (any monotonic millisecond source), ticks timed widgets, opens due tooltips;
    /// true when a repaint is due.
    pub fn tick(&mut self, now_ms: u64) -> bool {
        self.now_ms = now_ms;
        self.ticked = true;
        let mut widgets = Vec::new();
        collect_widgets(&self.scene, self.scene.root(), &mut widgets);
        let mut animated = false;
        for id in widgets {
            let due = match &mut self.scene.node_raw_mut(id).content {
                NodeContent::Widget(widget) => widget.tick(now_ms),
                _ => false,
            };
            if due {
                animated = true;
                self.scene.mark_dirty(id, PAINT_DIRTY);
            }
        }
        let mut windows = Vec::new();
        collect_windows(&self.scene, self.scene.root(), &mut windows);
        let frame = now_ms / SPIN_FRAME_MS;
        for id in windows {
            let spinning = matches!(&self.scene.node(id).content, NodeContent::Chrome(ChromeState::Window(w)) if w.stack_tabs.iter().any(|tab| tab.status == Some(Status::Running)));
            animated |= spinning;
            if let NodeContent::Chrome(ChromeState::Window(w)) = &mut self.scene.node_raw_mut(id).content {
                if spinning && w.spin != frame {
                    w.spin = frame;
                    self.scene.mark_dirty(id, PAINT_DIRTY);
                }
            }
        }
                if let Some(preview) = &mut self.dock_preview { preview.window.spin = frame; }
self.animated = animated;
        if self.tooltip.is_none() && now_ms.saturating_sub(self.hover_since_ms) >= TOOLTIP_DELAY_MS {
            if let (Some(text), Some(pos)) = (self.hover_tip.clone(), self.hover_pos) {
                self.show_tooltip(pos, text);
            }
        }
        self.dirty()
    }

    /// 🛎️ Whether anything changed since the last painted frame.
    pub fn dirty(&self) -> bool {
        self.full_redraw || self.layout_stale || self.scene.dirty_flags() != 0
    }

    /// ⏳ Milliseconds until the engine next needs a tick: a pending tooltip or a deferred frame; `None` when idle.
    pub fn deadline_ms(&self) -> Option<u64> {
        let tooltip = (self.tooltip.is_none() && self.hover_tip.is_some()).then(|| (self.hover_since_ms + TOOLTIP_DELAY_MS).saturating_sub(self.now_ms));
        let frame = self.dirty().then(|| self.last_render_ms.map_or(0, |at| (at + FRAME_BUDGET_MS).saturating_sub(self.now_ms)));
        let animation = self.animated.then(|| SPIN_FRAME_MS - self.now_ms % SPIN_FRAME_MS);
        [tooltip, frame, animation].into_iter().flatten().min()
    }

    /// 🚀 Renders when something changed and the frame budget since the last frame has elapsed.
    pub fn render_due(&mut self, now_ms: u64) -> Option<AnsiPatch> {
        self.now_ms = now_ms;
        if !self.dirty() || self.last_render_ms.is_some_and(|at| now_ms.saturating_sub(at) < FRAME_BUDGET_MS) {
            return None;
        }
        Some(self.render())
    }

    /// 📐️ Solves the scene's layout for the current size without painting.
    pub fn layout(&mut self) {
        self.sync_chrome();
        let viewport = Rect::new(0, 0, self.size.width, self.size.height);
        semio_framework_ui_tui_frame::layout::solve(&mut self.scene, viewport);
        let overlay_root = self.scene.overlay_root();
        self.scene.node_raw_mut(overlay_root).rect = viewport;
        self.place_overlays();
        self.scene.clear_dirty(LAYOUT_DIRTY);
        self.layout_stale = false;
    }

    pub(super) fn ensure_layout(&mut self) {
        if self.layout_stale || self.scene.dirty_flags() & LAYOUT_DIRTY != 0 {
            self.layout();
        }
    }

    pub(super) fn sync_chrome(&mut self) {
        let mut windows = Vec::new();
        collect_windows(&self.scene, self.scene.root(), &mut windows);
        let focused = self.focused_window();
        let compact = self.size.height < COMPACT_ROWS;
        for id in windows {
            let mut flags = None;
            if let NodeContent::Chrome(ChromeState::Window(window)) = &self.scene.node(id).content {
                let focus = if focused.is_some() { Some(focused == Some(id)) } else { None };
                if focus.is_some_and(|f| f != window.focused) || window.compact != compact {
                    flags = Some(focus);
                }
            }
            if let Some(focus) = flags {
                if let NodeContent::Chrome(ChromeState::Window(window)) = &mut self.scene.node_raw_mut(id).content {
                    if let Some(focus) = focus {
                        window.focused = focus;
                    }
                    window.compact = compact;
                }
                self.scene.mark_dirty(id, LAYOUT_DIRTY | PAINT_DIRTY);
            }
        }
    }

    fn open_overlay(&mut self, widget: WidgetState, placement: Placement, modal: bool, dismiss_outside: bool) -> NodeId {
        let mut node = Node::new(NodeContent::Widget(widget));
        node.hittable = modal;
        let id = self.scene.add(self.scene.overlay_root(), node);
        let saved_focus = self.focus();
        if modal {
            self.set_hover_node(None);
        }
        self.overlays.push(Overlay { id, placement, modal, dismiss_outside, saved_focus });
        if modal {
            self.assign_focus(Some(id));
        }
        self.layout_stale = true;
        id
    }

    /// 🧾 Opens a context menu at `anchor`; it traps focus until an item is chosen or it is dismissed.
    pub fn open_menu(&mut self, anchor: Pos, items: Vec<MenuItem>) -> NodeId {
        self.open_overlay(WidgetState::Menu(MenuState::new(items)), Placement::Anchored(anchor), true, true)
    }

    /// 💬️ Opens a centred modal dialog; it traps focus until a button is chosen or it is dismissed.
    pub fn open_dialog(&mut self, dialog: DialogState) -> NodeId {
        let dismiss_outside = dialog.dismiss_outside;
        self.open_overlay(WidgetState::Dialog(dialog), Placement::Centered, true, dismiss_outside)
    }

    /// ⌨️ Opens a command palette near the top of the terminal; it traps focus until an entry is chosen or it is dismissed.
    pub fn open_palette(&mut self, palette: PaletteState) -> NodeId {
        self.open_overlay(WidgetState::Palette(palette), Placement::TopCentered, true, true)
    }

    /// 💡️ Shows a tooltip beside `anchor`; it never takes focus or the pointer.
    pub fn show_tooltip(&mut self, anchor: Pos, text: impl Into<String>) -> NodeId {
        self.hide_tooltip();
        let id = self.open_overlay(WidgetState::Tooltip(TooltipState::new(text)), Placement::Tip(anchor), false, false);
        self.tooltip = Some(id);
        id
    }

    pub(super) fn hide_tooltip(&mut self) {
        if let Some(id) = self.tooltip.take() {
            self.close_overlay(id);
        }
    }

    /// 📚 Overlay nodes in z-order, topmost last.
    pub fn overlays(&self) -> Vec<NodeId> {
        self.overlays.iter().map(|overlay| overlay.id).collect()
    }

    /// 🚪 Closes an overlay and gives focus back to what held it before.
    pub fn close_overlay(&mut self, id: NodeId) {
        let Some(at) = self.overlays.iter().position(|overlay| overlay.id == id) else { return };
        let overlay = self.overlays.remove(at);
        if self.tooltip == Some(id) {
            self.tooltip = None;
        }
        for other in &mut self.overlays {
            if other.saved_focus == Some(id) {
                other.saved_focus = overlay.saved_focus;
            }
        }
        if self.focus == Some(id) {
            let restore = overlay.saved_focus.filter(|&saved| self.scene.contains(saved));
            self.focus = restore;
            if let Some(node) = restore {
                self.scene.mark_dirty(node, PAINT_DIRTY);
            }
        }
        self.scene.remove(id);
        self.layout_stale = true;
    }

    fn place_overlays(&mut self) {
        let viewport = self.size;
        for overlay in &self.overlays {
            let Some(node) = self.scene.try_node(overlay.id) else { continue };
            let NodeContent::Widget(widget) = &node.content else { continue };
            let size = widget.overlay_size(viewport);
            let (w, h) = (size.width.min(viewport.width), size.height.min(viewport.height));
            let (x, y) = match overlay.placement {
                Placement::Anchored(anchor) => {
                    let x = if anchor.x + w > viewport.width { anchor.x.saturating_sub(w.saturating_sub(1)) } else { anchor.x };
                    let y = if anchor.y + h > viewport.height { anchor.y.saturating_sub(h.saturating_sub(1)) } else { anchor.y };
                    (x, y)
                }
                Placement::Centered => (viewport.width.saturating_sub(w) / 2, viewport.height.saturating_sub(h) / 2),
                Placement::TopCentered => (viewport.width.saturating_sub(w) / 2, (viewport.height / 6).max(1)),
                Placement::Tip(anchor) => {
                    let x = if anchor.x + 1 + w > viewport.width { anchor.x.saturating_sub(w) } else { anchor.x + 1 };
                    let y = if anchor.y + 1 + h > viewport.height { anchor.y.saturating_sub(h) } else { anchor.y + 1 };
                    (x, y)
                }
            };
            let rect = clamp_to(Rect::new(x, y, w, h), viewport);
            let id = overlay.id;
            self.scene.node_raw_mut(id).rect = rect;
        }
    }

    pub(super) fn overlay_signals(&mut self, node: NodeId, signal: &Option<WidgetSignal>, key: Option<&KeyEvent>, out: &mut Signals) {
        if !self.overlays.iter().any(|overlay| overlay.id == node && overlay.modal) {
            return;
        }
        match (signal, key) {
            (Some(WidgetSignal::Activated(_)), _) => self.close_overlay(node),
            (None, Some(KeyEvent { key: Key::Esc, .. })) => {
                out.push((node, WidgetSignal::Dismissed));
                self.close_overlay(node);
            }
            _ => {}
        }
    }

}
