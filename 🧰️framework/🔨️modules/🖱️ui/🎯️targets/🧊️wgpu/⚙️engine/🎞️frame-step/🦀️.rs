//! 🎞️ Frame step: the `frame_step` that paints one surface frame. See [`super`].

use super::*;

// #region engine
//#region 🔖️Ui
impl Ui {
    /// 🖼️ The dirty-gated per-tick pipeline for `window_id`: `flex::LayoutEngine::compute` (itself a
    /// no-operation unless the root carries `DIRTY_LAYOUT`/`SUBTREE_DIRTY`) followed — only if that or the
    /// root's own `DIRTY_PAINT` fired — by `paint::paint_tree`, then handing every
    /// `scene_slots::collect_scene_slots` leaf to `scene_host`, when the caller passed one this tick.
    /// Returns `None` if `window_id` has no tree yet (`apply_tree` never called). A dirty window
    /// always repaints its whole tree — `paint::paint_tree`'s own doc comment: `DrawList` only
    /// supports a full clear-and-rebuild, no incremental dirty-subtree replacement yet.
    ///
    /// 🖋️ `atlas`/`icons` are the CALLER's own `FontAtlas`/`IconAtlas` — `Ui` never owns either (see
    /// this region's top-of-file doc comment): the host must pass the SAME instances it already
    /// `GpuContext::upload_font_atlas`/`upload_icon_atlas`s every frame, exactly like `flex::LayoutEngine::
    /// compute`/`paint::paint_tree` already receive them as parameters rather than fields. This lets
    /// retained-mode content share glyph/icon UVs with the rest of the host's chrome instead of
    /// clobbering (or never populating) a second, independent GPU texture.
    ///
    /// 🎬️ `scene_host` is a PER-FRAME parameter, not a stored field (there used to be a stored
    /// `Option<Box<dyn SceneHost>>` — removed): a caller-owned host typically needs to borrow the
    /// same per-frame state this call site already has in scope (a `GpuContext`, per-surface state
    /// maps, …), which a `Box<dyn SceneHost>` stored on `Ui` itself could never hold, exactly like
    /// `atlas`/`icons` above are parameters rather than fields for the same reason. `paint_tree`
    /// already knows (via `scene_host.is_some()`) whether to paint its own placeholder chrome for
    /// `ComponentScene`/`Image` leaves this tick or leave that rect for the host to fill in below —
    /// see `paint`'s own doc comment on that gate.
    /// 🧵️ Advances one retained paint node, traversal scalar, scene child, publication swap, or
    /// retirement scalar for one mounted window.
    pub fn frame_step<H: SceneHost>(&mut self, window_id: &str, viewport_width: f32, viewport_height: f32, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, mut scene_host: Option<&mut H>) -> UiFrameStep {
        self.set_viewport(window_id, viewport_width, viewport_height);
        self.publish_overlay_origins(window_id);
        let overlay_chrome = self.retained_overlay_chrome(window_id);
        let viewport_rect = semio_framework_ui_theme::geometry::Rect::new(0.0, 0.0, viewport_width, viewport_height);
        let theme = self.theme;
        let Some(surface) = self.windows.token(window_id) else { return UiFrameStep::Missing };
        let Some(window) = self.windows.get_mut(window_id).filter(|window| window.closing.is_none()) else { return UiFrameStep::Missing };
        let Some(root) = window.tree.root else { return UiFrameStep::Missing };
        let reversed = window.router.flow().block == ui_contract::FlowBlock::Up;
        if let Some(retiring) = window.retiring_draw.as_mut() {
            if !retiring.retire_step() {
                return UiFrameStep::Pending;
            }
            window.retiring_draw = None;
            return UiFrameStep::Pending;
        }
        let layout_dirty = window.tree.node(root).is_some_and(|node| node.flags.contains(NodeFlags::DIRTY_LAYOUT) || node.flags.contains(NodeFlags::SUBTREE_DIRTY));
        if layout_dirty {
            return UiFrameStep::Pending;
        }
        if window.paint_frame.is_none() {
            let dirty = window.tree.node(root).is_some_and(|node| node.flags.contains(NodeFlags::DIRTY_PAINT));
            if !dirty {
                return UiFrameStep::Ready;
            }
            let tooltip = retained_tooltip_paint(window, surface, atlas, &theme);
            window.paint_frame = Some(RetainedPaintFrame {
                phase: RetainedPaintPhase::Synchronize,
                walk: RetainedPaintWalk::new(&window.tree, root),
                candidate: DrawList::default(),
                sync_node: None,
                node_sync: RetainedInteractiveSyncCursor::default(),
                paint_node: None,
                paint_overlay: false,
                overlay_chrome,
                overlay_index: 0,
                node_paint: RetainedNodePaintCursor::default(),
                scene_node: None,
                scene_paint: ScenePaintCursor::default(),
                tooltip,
                interaction_epoch: window.presented_interaction_epoch,
                revision: window.revision,
                theme_revision: window.theme_revision,
                viewport_revision: window.viewport_revision,
                viewport_origin: [0.0, 0.0],
                baseline: UiFramePaintCensus::default(),
                hit_candidates: Vec::new(),
                fault_site: None,
                progress: 0,
            });
            return UiFrameStep::Pending;
        }
        let fresh = window
            .paint_frame
            .as_ref()
            .is_some_and(|frame| frame.revision == window.revision && frame.theme_revision == window.theme_revision && frame.viewport_revision == window.viewport_revision && frame.interaction_epoch == window.presented_interaction_epoch);
        if !fresh {
            if let Some(frame) = window.paint_frame.as_mut() {
                frame.node_paint.cancel_draw_route(&mut frame.candidate);
            }
            if !window.paint_frame.as_mut().is_some_and(|frame| {
                let closed = frame.node_sync.close_step();
                frame.note_progress();
                closed
            }) {
                return UiFrameStep::Pending;
            }
            let Some(frame) = window.paint_frame.take() else { return UiFrameStep::Fault };
            window.retiring_draw = Some(frame.candidate);
            return UiFrameStep::Pending;
        }
        let Some(frame) = window.paint_frame.as_mut() else { return UiFrameStep::Fault };
        if matches!(frame.phase, RetainedPaintPhase::Fault) {
            frame.node_paint.cancel_draw_route(&mut frame.candidate);
            if !frame.node_sync.close_step() {
                frame.note_progress();
                return UiFrameStep::Pending;
            }
            frame.sync_node = None;
            return UiFrameStep::Fault;
        }
        if matches!(frame.phase, RetainedPaintPhase::Synchronize) {
            if let Some(node) = frame.sync_node {
                match sync_interactive_state_node_step(&mut window.tree, node, &theme, &mut frame.node_sync) {
                    RetainedInteractiveSyncStep::Pending => {
                        frame.note_progress();
                        return UiFrameStep::Pending;
                    }
                    RetainedInteractiveSyncStep::Complete => {
                        frame.sync_node = None;
                        frame.note_progress();
                        return UiFrameStep::Pending;
                    }
                    RetainedInteractiveSyncStep::Fault => {
                        frame.phase = RetainedPaintPhase::Fault;
                        frame.note_progress();
                        return UiFrameStep::Pending;
                    }
                }
            }
        }
        if matches!(frame.phase, RetainedPaintPhase::Paint) {
            if let Some((node, origin_x, origin_y)) = frame.paint_node {
                let starting = !frame.node_paint.is_active();
                let clip = starting.then(|| retained_node_clip(&window.tree, node)).flatten();
                if starting {
                    if let Some(clip) = clip {
                        frame.candidate.push_scissor(clip);
                    }
                }
                if frame.paint_overlay {
                    frame.candidate.begin_overlay_route();
                }
                let step = paint_node_step_with_driver(
                    &window.tree,
                    node,
                    origin_x,
                    origin_y,
                    &theme,
                    atlas,
                    icons,
                    scene_host.is_some(),
                    self.driver_drag,
                    window.router.flow().block.is_reversed(),
                    window.router.flow().inline,
                    &mut frame.candidate,
                    &mut frame.node_paint,
                );
                if frame.paint_overlay {
                    frame.candidate.end_overlay_route();
                }
                match step {
                    RetainedNodePaintStep::Pending => {
                        frame.note_progress();
                        return UiFrameStep::Pending;
                    }
                    RetainedNodePaintStep::Complete => {
                        if clip.is_some() || (!starting && retained_node_clip(&window.tree, node).is_some()) {
                            frame.candidate.pop_scissor();
                        }
                        frame.paint_node = None;
                        if let Some(node) = window.tree.node_mut(node) {
                            node.flags.set(NodeFlags::DIRTY_PAINT, false);
                        }
                        frame.note_progress();
                        return UiFrameStep::Pending;
                    }
                    RetainedNodePaintStep::Fault => {
                        if clip.is_some() || (!starting && retained_node_clip(&window.tree, node).is_some()) {
                            frame.candidate.pop_scissor();
                        }
                        frame.phase = RetainedPaintPhase::Fault;
                        frame.note_progress();
                        return UiFrameStep::Fault;
                    }
                }
            }
        }
        if matches!(frame.phase, RetainedPaintPhase::Scenes) {
            if let Some((node, origin_x, origin_y)) = frame.scene_node {
                let Some(host) = scene_host.as_deref_mut() else {
                    frame.phase = RetainedPaintPhase::Fault;
                    frame.note_progress();
                    return UiFrameStep::Fault;
                };
                let Some(slot) = scene_slot_for_node(&window.tree, node, origin_x, origin_y) else {
                    frame.phase = RetainedPaintPhase::Fault;
                    frame.note_progress();
                    return UiFrameStep::Fault;
                };
                let progress_before = frame.scene_paint.progress_witness();
                match host.paint_slot_step(&slot, &mut frame.scene_paint, &mut frame.candidate, atlas, icons) {
                    ScenePaintStep::Pending => {
                        if frame.scene_paint.progress_witness() != progress_before {
                            frame.note_progress();
                        }
                        return UiFrameStep::Pending;
                    }
                    ScenePaintStep::Complete => {
                        frame.scene_node = None;
                        frame.note_progress();
                        return UiFrameStep::Pending;
                    }
                    ScenePaintStep::Fault => {
                        frame.phase = RetainedPaintPhase::Fault;
                        frame.note_progress();
                        return UiFrameStep::Fault;
                    }
                }
            }
        }
        frame.note_progress();
        match frame.phase {
            RetainedPaintPhase::Synchronize => match frame.walk.step(&window.tree) {
                RetainedPaintWalkStep::Visit(node, _, _, _) => {
                    frame.sync_node = Some(node);
                    UiFrameStep::Pending
                }
                RetainedPaintWalkStep::Scalar => UiFrameStep::Pending,
                RetainedPaintWalkStep::Complete => {
                    frame.phase = RetainedPaintPhase::Overlays;
                    UiFrameStep::Pending
                }
                RetainedPaintWalkStep::DepthFault => {
                    frame.phase = RetainedPaintPhase::Fault;
                    UiFrameStep::Fault
                }
            },
            RetainedPaintPhase::Overlays => match frame.overlay_chrome.get(frame.overlay_index).copied().flatten() {
                Some(placement) => {
                    frame.overlay_index = frame.overlay_index.saturating_add(1);
                    let bounds = semio_framework_ui_theme::geometry::Rect::new(placement.x, placement.y, placement.width, placement.height);
                    if retained_overlay_chrome_step(&mut frame.candidate, viewport_rect, bounds, placement.backdrop, &theme) {
                        UiFrameStep::Pending
                    } else {
                        frame.phase = RetainedPaintPhase::Fault;
                        UiFrameStep::Fault
                    }
                }
                None => {
                    frame.phase = RetainedPaintPhase::Paint;
                    frame.walk = RetainedPaintWalk::new(&window.tree, root);
                    UiFrameStep::Pending
                }
            },
            RetainedPaintPhase::Paint => match frame.walk.step(&window.tree) {
                RetainedPaintWalkStep::Visit(node, origin_x, origin_y, overlay_root) => {
                    frame.paint_node = Some((node, origin_x, origin_y));
                    frame.paint_overlay = overlay_root.is_some_and(|root| frame.overlay_chrome.iter().flatten().any(|placement| placement.root == root));
                    UiFrameStep::Pending
                }
                RetainedPaintWalkStep::Scalar => UiFrameStep::Pending,
                RetainedPaintWalkStep::Complete => {
                    frame.phase = RetainedPaintPhase::Scenes;
                    frame.walk = RetainedPaintWalk::new(&window.tree, root);
                    UiFrameStep::Pending
                }
                RetainedPaintWalkStep::DepthFault => {
                    frame.phase = RetainedPaintPhase::Fault;
                    UiFrameStep::Fault
                }
            },
            RetainedPaintPhase::Scenes => match frame.walk.step(&window.tree) {
                RetainedPaintWalkStep::Visit(node, origin_x, origin_y, _) => {
                    if scene_host.is_some() && scene_slot_for_node(&window.tree, node, origin_x, origin_y).is_some() {
                        frame.scene_node = Some((node, origin_x, origin_y));
                    }
                    UiFrameStep::Pending
                }
                RetainedPaintWalkStep::Scalar => UiFrameStep::Pending,
                RetainedPaintWalkStep::Complete => {
                    frame.phase = RetainedPaintPhase::Tooltip;
                    UiFrameStep::Pending
                }
                RetainedPaintWalkStep::DepthFault => {
                    frame.phase = RetainedPaintPhase::Fault;
                    UiFrameStep::Fault
                }
            },
            RetainedPaintPhase::Tooltip => {
                if let Some(tooltip) = frame.tooltip.take() {
                    paint_retained_tooltip(&mut frame.candidate, atlas, tooltip, (0.0, 0.0), &theme);
                }
                frame.phase = RetainedPaintPhase::Hits;
                frame.walk = RetainedPaintWalk::new(&window.tree, root);
                frame.hit_candidates.clear();
                frame.overlay_index = 0;
                UiFrameStep::Pending
            }
            RetainedPaintPhase::Hits => match frame.walk.step(&window.tree) {
                RetainedPaintWalkStep::Visit(node, origin_x, origin_y, overlay_root) => {
                    register_retained_hit(
                        &window.tree,
                        &theme,
                        self.driver_drag,
                        reversed,
                        window.router.flow().inline,
                        node,
                        origin_x,
                        origin_y,
                        retained_node_clip(&window.tree, node),
                        overlay_root.is_some(),
                        &mut frame.overlay_index,
                        &mut frame.hit_candidates,
                    );
                    UiFrameStep::Pending
                }
                RetainedPaintWalkStep::Scalar => UiFrameStep::Pending,
                RetainedPaintWalkStep::Complete => {
                    frame.phase = RetainedPaintPhase::Publish;
                    UiFrameStep::Pending
                }
                RetainedPaintWalkStep::DepthFault => {
                    frame.phase = RetainedPaintPhase::Fault;
                    UiFrameStep::Fault
                }
            },
            RetainedPaintPhase::Publish => {
                window.hit_registry = std::mem::take(&mut frame.hit_candidates);
                std::mem::swap(&mut window.draw, &mut frame.candidate);
                window.retiring_draw = Some(std::mem::take(&mut frame.candidate));
                frame.phase = RetainedPaintPhase::Complete;
                window.paint_census = UiFramePaintCensus::of(&window.draw);
                UiFrameStep::Pending
            }
            RetainedPaintPhase::Complete => {
                window.paint_frame = None;
                UiFrameStep::Ready
            }
            RetainedPaintPhase::Fault => UiFrameStep::Fault,
        }
    }

}
//#endregion 🔖️Ui
// #endregion engine
