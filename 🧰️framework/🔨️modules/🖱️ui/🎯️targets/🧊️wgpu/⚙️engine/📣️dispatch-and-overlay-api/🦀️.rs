//! 📣️ Dispatch and overlay API: the layout queries, the paint census, the event, pointer and accessibility dispatch, the overlay API, the clock and the commands. See [`super`].

use super::*;

// #region engine
//#region 🔖️Ui
impl Ui {
    /// 📤️ Direct access to `window_id`'s last-painted `DrawList` without re-running the pipeline.
    /// 🩺️ Which phase `window_id`'s in-flight retained paint frame is standing on — the only way a
    /// `UiFrameStep::Fault` can name where it came from without widening the step enum. `None` when no
    /// paint frame is in flight.
    /// 📐️ Whether THIS window's root still carries a layout obligation.
    ///
    /// ⚖️ `step_layouts` answers for the layout QUEUE, not for a window: `Idle` means the queue was
    /// empty and `Ready { window_id }` may name a different surface entirely. A caller that reads
    /// either as "my window is laid out" advances to paint against a dirty root, and
    /// `frame_into_step` then answers `Pending` for as long as the shell runs — the wgpu shell burned
    /// its whole 1 048 576-opportunity window-paint budget on `procedural-main` this way, twenty
    /// seconds per chrome walk (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). This is the per-window
    /// predicate that question actually needs.
    pub fn layout_is_dirty(&self, window_id: &str) -> bool {
        let Some(window) = self.windows.get(window_id) else { return false };
        let Some(root) = window.tree.root else { return false };
        window.tree.node(root).is_some_and(|node| node.flags.contains(NodeFlags::DIRTY_LAYOUT) || node.flags.contains(NodeFlags::SUBTREE_DIRTY))
    }

    /// 📏️ The bottom-up intrinsic height measured for one surface's root before the top-down viewport
    /// solve stretches it — the content extent a floating panel needs to HUG its content the way
    /// React's `<Panel>` does (`🖼️Panel/🟦️.tsx`: one bonded edge plus
    /// `maxHeight: calc(100% - 2 * --spacing-single)`, never a `top`+`bottom` pair). A root with no
    /// intrinsic extent, such as an engine surface, keeps its accepted viewport height. `None` while
    /// the surface has no window, no root or no accepted layout yet.
    pub fn surface_content_height(&self, window_id: &str) -> Option<f32> {
        let window = self.windows.get(window_id)?;
        let (tree, intrinsic) = if window.candidate_ready || !window.presented_ready { (&window.tree, window.intrinsic_content_height) } else { (&window.presented_tree, window.presented_intrinsic_content_height) };
        let root = tree.root?;
        let root_layout = tree.accepted_layout(root)?;
        if intrinsic.is_finite() && intrinsic > 0.0 {
            Some(intrinsic)
        } else {
            Some(root_layout.height)
        }
    }

    /// 📐️ Re-arms one window's layout lane. Idempotent — a window already queued is left alone — so a
    /// caller that finds [`Self::layout_is_dirty`] true can always make the obligation reachable
    /// again instead of waiting on a wake-up that was never posted.
    pub fn request_layout(&mut self, window_id: &str) {
        self.enqueue_layout(window_id);
    }

    /// 🩺️ Why one window's retained paint is not finishing, in the exact order `frame_into_step`
    /// decides it: a missing window, a rootless tree, a root the layout engine still calls dirty, a
    /// paint frame that has not been opened, or a frame whose revision stamps no longer match the
    /// window's (which retires the frame and starts over). A bare phase could not tell those apart —
    /// `paint_frame_phase` answers `None` for FOUR different stalls
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn paint_stall_census(&self, window_id: &str) -> String {
        let Some(window) = self.windows.get(window_id) else { return "window=absent".to_string() };
        let Some(root) = window.tree.root else { return "root=absent".to_string() };
        let dirty_layout = window.tree.node(root).is_some_and(|node| node.flags.contains(NodeFlags::DIRTY_LAYOUT));
        let subtree_dirty = window.tree.node(root).is_some_and(|node| node.flags.contains(NodeFlags::SUBTREE_DIRTY));
        let frame = window.paint_frame.as_ref();
        let paint_key = frame.and_then(|frame| frame.paint_node).and_then(|(node, _, _)| window.tree.node(node)).map(|node| format!("{:?}", node.key)).unwrap_or_else(|| "none".to_string());
        let paint_cursor = frame.map(|frame| frame.node_paint.census()).unwrap_or_else(|| "none".to_string());
        format!(
            "root=present dirty-layout={dirty_layout} subtree-dirty={subtree_dirty} frame={} revision={}/{} theme={}/{} viewport={}/{} phase={:?} fault-site={:?} paint-key={paint_key} cursor=[{paint_cursor}]",
            frame.is_some(),
            frame.map_or(0, |frame| frame.revision),
            window.revision,
            frame.map_or(0, |frame| frame.theme_revision),
            window.theme_revision,
            frame.map_or(0, |frame| frame.viewport_revision),
            window.viewport_revision,
            self.paint_frame_phase(window_id),
            frame.and_then(|frame| frame.fault_site),
        )
    }

    /// 🔄️ The source line of the synchronize arm that faulted this window's paint frame, `0` when the
    /// walk never took one. See `🖌️paint`'s `retained_sync_fault`.
    pub fn paint_frame_sync_fault_line(&self, window_id: &str) -> u32 {
        self.windows.get(window_id).and_then(|window| window.paint_frame.as_ref()).map_or(0, |frame| frame.node_sync.fault_line)
    }

    /// 🖌️ Monotonic scalar progress of the in-flight retained paint frame.
    pub fn paint_frame_progress(&self, window_id: &str) -> Option<u64> {
        self.windows.get(window_id)?.paint_frame.as_ref().map(|frame| frame.progress)
    }

    pub fn paint_frame_phase(&self, window_id: &str) -> Option<&'static str> {
        self.windows.get(window_id).and_then(|window| window.paint_frame.as_ref()).map(|frame| match (frame.fault_site, frame.phase) {
            (Some(site), _) => site,
            _ => match frame.phase {
                RetainedPaintPhase::Synchronize => "synchronize",
                RetainedPaintPhase::Overlays => "overlays",
                RetainedPaintPhase::Paint => "paint",
                RetainedPaintPhase::Scenes => "scenes",
                RetainedPaintPhase::Tooltip => "tooltip",
                RetainedPaintPhase::Hits => "hits",
                RetainedPaintPhase::Publish => "publish",
                RetainedPaintPhase::Complete => "complete",
                RetainedPaintPhase::Fault => "fault",
            },
        })
    }

    /// 🎯️ `window_id`'s pointer registry as of its last published paint — one entry per interactive
    /// retained node, at the rect that paint drew it at. A host pushes these into its own
    /// `InputState` every frame build; see [`RetainedHitRegistration`].
    pub fn window_hit_targets(&self, window_id: &str) -> &[RetainedHitRegistration] {
        self.windows.get(window_id).filter(|window| window.closing.is_none()).map_or(&[], |window| window.hit_registry.as_slice())
    }

    /// 📊️ What `window_id`'s own retained paint contributed to the LAST frame it completed — the
    /// only honest answer for the production `frame_into_step` path, whose output lands in the
    /// caller's draw list and never in `draw_list` below. See [`UiFramePaintCensus`].
    pub fn paint_census(&self, window_id: &str) -> Option<UiFramePaintCensus> {
        self.windows.get(window_id).map(|window| window.paint_census)
    }

    pub fn draw_list(&self, window_id: &str) -> Option<&DrawList> {
        self.windows.get(window_id).map(|window| &window.draw)
    }

    /// 🕹️ Routes one event through the exact presented interaction revision. Synchronous commands
    /// have one owner: this return value. `drain_commands` is reserved for asynchronous producers.
    #[allow(clippy::needless_pass_by_value, reason = "changing to &UiEvent is a breaking public API change across ~30 downstream plugins, out of T1 scope")]
    pub fn dispatch_event(&mut self, window_id: &str, event: UiEvent) -> Vec<UiCommand> {
        let reset_caret = matches!(&event, UiEvent::PointerDown { .. } | UiEvent::KeyDown { .. } | UiEvent::TextInput { .. } | UiEvent::Paste { .. } | UiEvent::Ime(_));
        let metrics = TreeRowMetrics::from_theme(&self.theme);
        let driver_drag = self.driver_drag;
        let Some(surface) = self.windows.token(window_id) else { return Vec::new() };
        let Some(window) = self.windows.get_mut(window_id).filter(|window| window.closing.is_none()) else { return Vec::new() };
        let presented = window.presented_ready;
        #[cfg(not(any(test, feature = "testkit")))]
        if !presented {
            return Vec::new();
        }
        let (commands, layout_changed) = {
            let (tree, router) = if presented { (&mut window.presented_tree, &mut window.presented_router) } else { (&mut window.tree, &mut window.router) };
            let Some(root) = tree.root else { return Vec::new() };
            let inline = router.flow().inline;
            router.set_control_border(self.theme.stroke_hairline);
            router.set_control_gap(self.theme.gap_standard);
            let mut commands = router.set_tree_drag_policy(tree, driver_drag, metrics.with_inline(inline));
            commands.extend(router.dispatch(tree, root, &event));
            router.synchronize_retained_caret(tree, reset_caret);
            (commands, tree.take_disclosure_changed())
        };
        if layout_changed {
            let Some(generation) = window.layout_generation.checked_add(1) else { return commands };
            window.layout_generation = generation;
            window.intrinsic_content_height = f32::NAN;
        }
        if presented {
            window.advance_presented_interaction();
            if synchronize_presented_tooltip(window, surface) {
                mark_window_paint_dirty(window);
            }
        }
        if layout_changed {
            self.enqueue_layout(window_id);
        }
        commands
    }

    pub fn dispatch_pointer_event(&mut self, window_id: &str, pointer_id: u64, event: UiEvent) -> Vec<UiCommand> {
        let reset_caret = matches!(&event, UiEvent::PointerDown { .. });
        let metrics = TreeRowMetrics::from_theme(&self.theme);
        let driver_drag = self.driver_drag;
        let Some(surface) = self.windows.token(window_id) else { return Vec::new() };
        let Some(window) = self.windows.get_mut(window_id).filter(|window| window.closing.is_none()) else { return Vec::new() };
        let presented = window.presented_ready;
        #[cfg(not(any(test, feature = "testkit")))]
        if !presented {
            return Vec::new();
        }
        let (commands, layout_changed) = {
            let (tree, router) = if presented { (&mut window.presented_tree, &mut window.presented_router) } else { (&mut window.tree, &mut window.router) };
            let Some(root) = tree.root else { return Vec::new() };
            let inline = router.flow().inline;
            router.set_control_border(self.theme.stroke_hairline);
            router.set_control_gap(self.theme.gap_standard);
            let mut commands = router.set_tree_drag_policy(tree, driver_drag, metrics.with_inline(inline));
            commands.extend(router.dispatch_pointer(tree, root, pointer_id, &event));
            router.synchronize_retained_caret(tree, reset_caret);
            (commands, tree.take_disclosure_changed())
        };
        if layout_changed {
            let Some(generation) = window.layout_generation.checked_add(1) else { return commands };
            window.layout_generation = generation;
            window.intrinsic_content_height = f32::NAN;
        }
        if presented {
            window.advance_presented_interaction();
            if synchronize_presented_tooltip(window, surface) {
                mark_window_paint_dirty(window);
            }
        }
        if layout_changed {
            self.enqueue_layout(window_id);
        }
        commands
    }

    pub fn dispatch_accessibility_event(&mut self, window_id: &str, window_generation: u64, node_id: u64, node_key: &str, event: AccessibilityUiEvent) -> Option<Vec<UiCommand>> {
        let metrics = TreeRowMetrics::from_theme(&self.theme);
        let driver_drag = self.driver_drag;
        let surface = self.windows.token(window_id)?;
        let window = self.windows.get_mut(window_id).filter(|window| window.closing.is_none())?;
        let presented = window.presented_ready;
        #[cfg(not(any(test, feature = "testkit")))]
        if !presented {
            return None;
        }
        let generation = if presented { window.presented_accessibility_generation } else { window.accessibility_generation };
        if generation != window_generation || window_generation == 0 {
            return None;
        }
        let document_id = UiNodeId(node_id);
        let (commands, layout_changed) = {
            let (tree, router) = if presented { (&mut window.presented_tree, &mut window.presented_router) } else { (&mut window.tree, &mut window.router) };
            let record = tree.document()?.record(document_id)?;
            let virtual_select_value = (record.key.as_str() != node_key).then(|| semio_framework_ui_wgpu_hosting::accessibility::select_accessibility_option_value(record, node_key)).flatten();
            let virtual_slider_editor = record.key.as_str() != node_key && semio_framework_ui_wgpu_hosting::accessibility::is_slider_accessibility_editor(record, node_key);
            let segmented_select = matches!(&record.component, ui_contract::Component::Select(select) if select.appearance == ui_contract::SelectAppearance::Segmented);
            let virtual_row_action = (record.key.as_str() != node_key).then(|| semio_framework_ui_wgpu_hosting::accessibility::row_accessibility_action(record, node_key)).flatten();
            if record.key.as_str() != node_key && virtual_select_value.is_none() && !virtual_slider_editor && virtual_row_action.is_none() {
                return None;
            }
            let target = tree.document_node(document_id)?;
            if virtual_select_value.is_some() && !segmented_select && !tree.node(target).is_some_and(|node| node.state.open) {
                return None;
            }
            let inline = router.flow().inline;
            router.set_control_border(self.theme.stroke_hairline);
            router.set_control_gap(self.theme.gap_standard);
            let mut commands = router.set_tree_drag_policy(tree, driver_drag, metrics.with_inline(inline));
            commands.extend(match (virtual_select_value, virtual_row_action) {
                (Some(value), _) => router.dispatch_accessibility_select_option(tree, target, &value, &event),
                (None, _) if virtual_slider_editor => router.dispatch_accessibility_slider_editor(tree, target, &event),
                (None, Some(index)) => router.dispatch_accessibility_row_action(tree, target, index, &event),
                (None, None) => router.dispatch_accessibility(tree, target, &event),
            });
            router.synchronize_retained_caret(tree, matches!(&event, AccessibilityUiEvent::Focus | AccessibilityUiEvent::Activate | AccessibilityUiEvent::Value(_)));
            (commands, tree.take_disclosure_changed())
        };
        if layout_changed {
            let generation = window.layout_generation.checked_add(1)?;
            window.layout_generation = generation;
            window.intrinsic_content_height = f32::NAN;
        }
        if presented {
            window.advance_presented_interaction();
            if synchronize_presented_tooltip(window, surface) {
                mark_window_paint_dirty(window);
            }
        }
        if layout_changed {
            self.enqueue_layout(window_id);
        }
        Some(commands)
    }

    //#region 🪟️OverlayApi
    /// 🔓️ Opens `root`'s subtree as a floating overlay of `kind`, anchored per `anchor` — the
    /// façade-level entry point for React's `Popover`/`Dialog`/`CommandPalette` portals
    /// (`🧱️elements/🗨️Popover/🟦️.tsx`, `🧱️elements/💬️Dialog/🟦️.tsx`). Placement, escape/outside-press
    /// dismissal and the `Dialog`/`CommandPalette` focus trap all come from `events`' own policy.
    pub fn open_overlay(&mut self, window_id: &str, root: NodeId, kind: OverlayKind, anchor: OverlayAnchor) {
        let Some(window) = self.windows.get_mut(window_id).filter(|window| window.closing.is_none()) else { return };
        window.router.open_overlay(&mut window.tree, root, kind, anchor);
    }

    /// 🚪️ Closes one open overlay by its content root.
    pub fn close_overlay(&mut self, window_id: &str, root: NodeId) -> Vec<UiCommand> {
        let Some(window) = self.windows.get_mut(window_id).filter(|window| window.closing.is_none()) else { return Vec::new() };
        window.router.close_overlay(&mut window.tree, root)
    }

    /// 🪜️ The open overlays whose surface chrome the RETAINED LADDER owns, in bottom-to-top order,
    /// capped at [`UI_FRAME_OVERLAY_CHROME`].
    ///
    /// Three of the six kinds are here. A `Dialog`, a `CommandPalette` and a `Popover` are React
    /// portals whose content is an ordinary document subtree: nothing in that subtree draws the
    /// surface it sits on, so this ladder must. The other three already own their own chrome and are
    /// deliberately excluded — a `SelectPopup`'s glass is painted by `paint_select` as part of the
    /// `Select` node itself (whose overlay root is the TRIGGER's rect, not the popup's, so surface
    /// chrome at that placement would paint a menu panel over the closed trigger), a `ContextMenu`'s
    /// by `render_context_menu`, and a `Tooltip`'s by `paint_tooltip`.
    pub(super) fn retained_overlay_chrome(&self, window_id: &str) -> [Option<UiOverlayPlacement>; UI_FRAME_OVERLAY_CHROME] {
        let mut chrome = [None; UI_FRAME_OVERLAY_CHROME];
        let mut next = 0;
        for placement in self.overlay_placements(window_id) {
            if !matches!(placement.kind, OverlayKind::Dialog | OverlayKind::CommandPalette | OverlayKind::Popover) {
                continue;
            }
            let Some(slot) = chrome.get_mut(next) else { break };
            *slot = Some(placement);
            next += 1;
        }
        chrome
    }

    /** 🪟️ Republishes every portal-backed OPEN overlay's resolved placement onto the window's
     * own tree, so this frame's paint walk, scene walk, hit registry and
     * `events::hit_test`/`absolute_rect` all read ONE origin for a floating surface
     * (`UiTree::overlay_origins`). A SelectPopup stays at its in-flow trigger because its Select
     * paints and routes the window-local popup geometry itself.
     *
     * This is what makes an overlay's BODY paint through the normal node pipeline INSIDE the overlay
     * rect: the Group/Section children under the content root are walked, laid out and painted
     * exactly like any other subtree — only the root's walk origin is substituted, so no second paint
     * pass and no parallel geometry exists for overlay content. React gets the same from a portal
     * plus `position: fixed` on `PopoverContent`/`DialogContent`
     * (`🧱️elements/🗨️Popover/🟦️.tsx`, `🧱️elements/💬️Dialog/🟦️.tsx`), where the children are ordinary
     * DOM inside the portalled box. */
    pub(super) fn publish_overlay_origins(&mut self, window_id: &str) {
        let origins: Vec<(NodeId, f32, f32)> = self.overlay_placements(window_id).into_iter().filter(|placement| placement.kind != OverlayKind::SelectPopup).map(|placement| (placement.root, placement.x, placement.y)).collect();
        if let Some(window) = self.windows.get_mut(window_id).filter(|window| window.closing.is_none()) {
            window.tree.set_overlay_origins(origins);
        }
    }

    /// 🥞️ Every open overlay of `window_id`, bottom-to-top, already placed: the resolved top-left the
    /// overlay's own laid-out size wants against this window's viewport, plus the side a collision
    /// flip settled on. A paint pass draws these AFTER the document walk — that ordering, not a flag,
    /// is what puts an overlay above panel content (React gets the same from a portal + `z-menu`).
    pub fn overlay_placements(&self, window_id: &str) -> Vec<UiOverlayPlacement> {
        let Some(window) = self.windows.get(window_id) else { return Vec::new() };
        window
            .router
            .open_overlays()
            .iter()
            .map(|overlay| {
                let size = window.tree.mounted_layout(overlay.root).map_or((0.0, 0.0), |(_, _, width, height)| (width, height));
                let resolved = resolve_overlay_placement_side(&window.tree, overlay.anchor, size, window.viewport, overlay.placement, window.router.flow_inline());
                UiOverlayPlacement { root: overlay.root, kind: overlay.kind, backdrop: overlay.kind.has_backdrop(), width: size.0, height: size.1, x: resolved.x, y: resolved.y, side: resolved.side }
            })
            .collect()
    }

    /// ⏱️ Advances one exact retained window against the process monotonic clock. Pure elapsed time
    /// preserves a sealed candidate; a due repeat or tooltip transition advances interaction once.
    pub fn arm_presented_scene_caret(&mut self, window_id: &str, surface: UiSurfaceToken, document_id: UiNodeId, seconds: f64) -> Option<UiPresentedCaret> {
        let id_matches = self.windows.id(surface).is_some_and(|id| id.as_ref() == window_id);
        if !id_matches {
            return None;
        }
        let window = self.windows.get_token_mut(surface).filter(|window| window.closing.is_none() && window.presented_ready)?;
        let node = window.presented_tree.document_node(document_id)?;
        if !window.presented_tree.node(node).is_some_and(|node| matches!(node.spec.0, UiNode::ComponentScene(_))) {
            return None;
        }
        let step = window.presented_router.arm_scene_caret(&mut window.presented_tree, node, seconds)?;
        window.advance_presented_interaction();
        presented_caret(surface, &window.presented_tree, step)
    }

    pub fn clear_presented_scene_caret(&mut self, window_id: &str, surface: UiSurfaceToken, document_id: UiNodeId) -> Option<UiPresentedCaret> {
        let id_matches = self.windows.id(surface).is_some_and(|id| id.as_ref() == window_id);
        if !id_matches {
            return None;
        }
        let window = self.windows.get_token_mut(surface).filter(|window| window.closing.is_none() && window.presented_ready)?;
        let node = window.presented_tree.document_node(document_id)?;
        let step = window.presented_router.clear_scene_caret(node)?;
        window.advance_presented_interaction();
        presented_caret(surface, &window.presented_tree, step)
    }

    pub fn advance_window_clock(&mut self, window_id: &str, seconds: f64) -> Option<UiWindowClockStep> {
        let surface = self.windows.token(window_id)?;
        let window = self.windows.get_token_mut(surface).filter(|window| window.closing.is_none())?;
        window.draw.set_clock_seconds(seconds as f32);
        let presented = window.presented_ready;
        let step = if presented { window.presented_router.advance_clock(&mut window.presented_tree, seconds) } else { window.router.advance_clock(&mut window.tree, seconds) };
        self.pending_commands.extend(step.commands);
        if presented && step.changed {
            window.advance_presented_interaction();
        }
        let tooltip_changed = if !presented {
            window.presented_tooltip.take().is_some()
        } else {
            match step.tooltip {
                TooltipStep::Reveal(node) if window.presented_router.revealed_row_reason().is_none() => match (window.presented_tree.document_id(node), tooltip_label_in_tree(&window.presented_tree, node)) {
                    (Some(document_id), Some(label)) => {
                        window.presented_tooltip = Some(PresentedTooltip { surface, document_id, label, accessibility_generation: window.presented_accessibility_generation, interaction_epoch: window.presented_interaction_epoch, row_action: None });
                        true
                    }
                    _ => window.presented_tooltip.take().is_some(),
                },
                TooltipStep::Dismissed if window.presented_router.revealed_row_reason().is_none() => window.presented_tooltip.take().is_some(),
                TooltipStep::Reveal(_) | TooltipStep::Dismissed | TooltipStep::Idle => synchronize_presented_tooltip(window, surface),
            }
        };
        if tooltip_changed {
            mark_window_paint_dirty(window);
        }
        let caret = step.caret.and_then(|step| presented_caret(surface, if presented { &window.presented_tree } else { &window.tree }, step));
        Some(UiWindowClockStep { surface, tooltip: step.tooltip, caret, interaction_changed: step.changed, next_deadline: step.next_deadline })
    }

    /// ⏰ Reads one live window's earliest armed deadline after pointer or key dispatch.
    pub fn window_next_clock_deadline(&self, window_id: &str) -> Option<(UiSurfaceToken, Option<f64>)> {
        let surface = self.windows.token(window_id)?;
        let window = self.windows.get_token(surface).filter(|window| window.closing.is_none())?;
        let router = if window.presented_ready { &window.presented_router } else { &window.router };
        Some((surface, router.next_clock_deadline()))
    }

    /// ⏭️ Earliest finite clock deadline owned by an exact accepted surface roster.
    pub fn surfaces_next_clock_deadline(&self, surfaces: &[(String, UiSurfaceToken)]) -> Option<f64> {
        let mut earliest: Option<f64> = None;
        for (window_id, expected) in surfaces {
            let Some((surface, Some(deadline))) = self.window_next_clock_deadline(window_id) else { continue };
            if surface != *expected || !deadline.is_finite() {
                continue;
            }
            earliest = Some(earliest.map_or(deadline, |current| current.min(deadline)));
        }
        earliest
    }

    /// 💡️ The hover text for one arena node, composed exactly like React's
    /// `formatControlTooltipText({ label, hotkey })` (`🔨️modules/💡️control-tooltip-presentation/🟦️.ts:16`):
    /// the record's accessibility label, with its declared shortcut in parentheses when it has one.
    /// A node with no label of its own has no tooltip, matching `useControlTooltipText`'s own
    /// `if (!label) return undefined`.
    pub fn tooltip_label(&self, window_id: &str, node: NodeId) -> Option<String> {
        tooltip_label_in_tree(self.tree(window_id)?, node)
    }
    //#endregion 🪟️OverlayApi

    /// 🗺️ Routes `event` through the shared `🐚️Shell`'s own hit-testing, surfacing chrome-level
    /// `ShellEvent`s (tab activation today; drag/drop is `Shell::dispatch`'s own documented gap).
    pub fn dispatch_shell_event(&mut self, event: &UiEvent) -> Vec<ShellEvent> {
        self.shell.dispatch(event)
    }

    /// 📥️ Drains every `UiCommand` queued by `dispatch_event` calls since the last drain.
    pub fn drain_commands(&mut self) -> Vec<UiCommand> {
        std::mem::take(&mut self.pending_commands)
    }

    /// 📬️ Queues commands produced by an asynchronous host completion for the next host drain.
    pub fn enqueue_async_commands(&mut self, commands: Vec<UiCommand>) {
        self.pending_commands.extend(commands);
    }

    /// 🫳️ Every retained-ui drag in flight (`EventRouter::DragSession`), keyed by window id with pointer in that window's local coordinates.
    pub fn active_drag_sessions(&self) -> Vec<(String, f32, f32, DragPayload)> {
        let mut sessions = Vec::new();
        for window_id in self.window_ids() {
            let Some(window) = self.windows.get(window_id) else { continue };
            if let Some(drag) = window.presented_ready.then_some(&window.presented_router).and_then(EventRouter::drag_session) {
                sessions.push((window_id.to_string(), drag.pointer_x, drag.pointer_y, drag.payload.clone()));
            }
        }
        sessions
    }
}
//#endregion 🔖️Ui
// #endregion engine
