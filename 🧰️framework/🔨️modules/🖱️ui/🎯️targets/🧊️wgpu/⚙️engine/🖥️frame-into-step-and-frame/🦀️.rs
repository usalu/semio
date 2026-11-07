//! 🖥️ Frame into step and frame: the `frame_into_step` that paints into a caller buffer and the `frame` entry point. See [`super`].

use super::*;

fn append_admitted(grant: semio_framework_job::RetainedCloneGrant, demand: semio_framework_job::RetainedCloneGrant) -> bool {
    grant.maximum_items >= demand.maximum_items && grant.maximum_copy_bytes >= demand.maximum_copy_bytes && grant.maximum_capacity_bytes >= demand.maximum_capacity_bytes && grant.maximum_release_bytes >= demand.maximum_release_bytes && grant.maximum_depth >= demand.maximum_depth
}

// #region engine
//#region 🔖️Ui
impl Ui {
    /// 📋 Quotes retained append work before the caller admits the next frame step.
    pub fn frame_into_next_retained_demand(&self, window_id: &str, viewport: semio_ui_theme::geometry::Rect, target: &DrawList) -> Result<semio_framework_job::RetainedCloneGrant, semio_ui_draw_contract::RetainedOutputError> {
        let Some(window) = self.windows.get(window_id) else { return Ok(Default::default()); };
        let Some(frame) = window.paint_frame.as_ref() else { return Ok(Default::default()); };
        if let Some(token) = frame.append_token {
            if target.active_retained_append_token() != Some(token) { return Ok(Default::default()); }
            let fresh = frame.revision == window.revision && frame.theme_revision == window.theme_revision && frame.viewport_revision == window.viewport_revision && window.viewport == (viewport.w, viewport.h) && frame.viewport_origin == [viewport.x, viewport.y] && frame.interaction_epoch == window.presented_interaction_epoch;
            return if !fresh || matches!(frame.phase, RetainedPaintPhase::Fault) { target.next_retained_append_cancel_demand(token) } else { target.next_retained_append_demand(token) };
        }
        if matches!(frame.phase, RetainedPaintPhase::Publish) { Ok(DrawList::retained_append_begin_demand()) } else { Ok(Default::default()) }
    }

    /// 🧱️ Advances a retained UI node or bounded append into the caller's unpublished frame builder.
    /// The builder remains transaction-owned through `Pending`; only `Ready` permits publication.
    pub fn frame_into_step<H: SceneHost>(&mut self, window_id: &str, viewport: semio_ui_theme::geometry::Rect, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, scene_host: Option<&mut H>, target: &mut DrawList, grant: semio_framework_job::RetainedCloneGrant) -> (UiFrameStep, semio_framework_job::RetainedCloneProgress) {
        let mut progress = semio_framework_job::RetainedCloneProgress::default();
        let step = self.frame_into_step_inner(window_id, viewport, atlas, icons, scene_host, target, grant, &mut progress);
        (step, progress)
    }

    fn frame_into_step_inner<H: SceneHost>(&mut self, window_id: &str, viewport: semio_ui_theme::geometry::Rect, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, mut scene_host: Option<&mut H>, target: &mut DrawList, grant: semio_framework_job::RetainedCloneGrant, progress: &mut semio_framework_job::RetainedCloneProgress) -> UiFrameStep {
        let semio_ui_theme::geometry::Rect { x: offset_x, y: offset_y, w: viewport_width, h: viewport_height } = viewport;
        self.set_viewport(window_id, viewport_width, viewport_height);
        self.publish_overlay_origins(window_id);
        let overlay_chrome = self.retained_overlay_chrome(window_id);
        let viewport_rect = viewport;
        let theme = self.theme;
        let Some(surface) = self.windows.token(window_id) else { return UiFrameStep::Missing };
        let Some(window) = self.windows.get_mut(window_id).filter(|window| window.closing.is_none()) else { return UiFrameStep::Missing };
        let Some(root) = window.tree.root else { return UiFrameStep::Missing };
        let reversed = window.router.flow().block == ui_contract::FlowBlock::Up;
        let layout_dirty = window.tree.node(root).is_some_and(|node| node.flags.contains(NodeFlags::DIRTY_LAYOUT) || node.flags.contains(NodeFlags::SUBTREE_DIRTY));
        if layout_dirty {
            return UiFrameStep::Pending;
        }
        if window.paint_frame.is_none() {
            let tooltip = retained_tooltip_paint(window, surface, atlas, &theme);
            let mut candidate = DrawList::default();
            candidate.set_screen_height(target.screen_height());
            candidate.set_clock_seconds(target.clock_seconds());
            window.paint_frame = Some(RetainedPaintFrame {
                phase: RetainedPaintPhase::Synchronize,
                walk: RetainedPaintWalk::new(&window.tree, root),
                candidate,
                append_token: None,
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
                viewport_origin: [offset_x, offset_y],
                baseline: UiFramePaintCensus::of(target),
                hit_candidates: Vec::new(),
                fault_site: None,
                progress: 0,
            });
            return UiFrameStep::Pending;
        }
        let fresh = window
            .paint_frame
            .as_ref()
            .is_some_and(|frame| {
                frame.revision == window.revision
                    && frame.theme_revision == window.theme_revision
                    && frame.viewport_revision == window.viewport_revision
                    && frame.viewport_origin == [offset_x, offset_y]
                    && frame.interaction_epoch == window.presented_interaction_epoch
            });
        if window.paint_frame.as_ref().and_then(|frame| frame.append_token).is_some_and(|token| target.active_retained_append_token() != Some(token)) {
            window.paint_frame = None;
            return UiFrameStep::Pending;
        }
        if !fresh {
            if let Some(frame) = window.paint_frame.as_mut() {
                if let Some(token) = frame.append_token {
                    let Ok(demand) = target.next_retained_append_cancel_demand(token) else { frame.phase = RetainedPaintPhase::Fault; return UiFrameStep::Fault; };
                    if !append_admitted(grant, demand) { return UiFrameStep::Pending; }
                    let (result, receipt) = target.cancel_retained_append_step(token); *progress = receipt;
                    frame.note_progress();
                    match result { Ok(semio_ui_draw_contract::RetainedAppendStep::Complete) => frame.append_token = None, Ok(semio_ui_draw_contract::RetainedAppendStep::Pending) => {}, Err(_) => { frame.phase = RetainedPaintPhase::Fault; return UiFrameStep::Fault; } }
                    return UiFrameStep::Pending;
                }
            }
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
            if let Some(token) = frame.append_token {
                let Ok(demand) = target.next_retained_append_cancel_demand(token) else { return UiFrameStep::Fault; };
                if !append_admitted(grant, demand) { return UiFrameStep::Pending; }
                let (result, receipt) = target.cancel_retained_append_step(token); *progress = receipt;
                frame.note_progress();
                match result { Ok(semio_ui_draw_contract::RetainedAppendStep::Complete) => frame.append_token = None, Ok(semio_ui_draw_contract::RetainedAppendStep::Pending) => {}, Err(_) => return UiFrameStep::Fault }
                return UiFrameStep::Pending;
            }
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
                        frame.fault_site = Some("synchronize-node");
                        frame.note_progress();
                        return UiFrameStep::Pending;
                    }
                }
            }
        }
        if matches!(frame.phase, RetainedPaintPhase::Paint) {
            if let Some((node, origin_x, origin_y)) = frame.paint_node {
                let starting = !frame.node_paint.is_active();
                let clip = starting.then(|| retained_node_clip(&window.tree, node).map(|clip| offset_rect(clip, offset_x, offset_y))).flatten();
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
                        frame.fault_site = Some("paint-node");
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
                    frame.fault_site = Some("scenes-no-host");
                    frame.note_progress();
                    return UiFrameStep::Fault;
                };
                let Some(slot) = scene_slot_for_node(&window.tree, node, origin_x, origin_y) else {
                    frame.phase = RetainedPaintPhase::Fault;
                    frame.fault_site = Some("scenes-slot-missing");
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
                        frame.fault_site = Some("scenes-host");
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
                    frame.fault_site = Some("walk-depth-synchronize");
                    UiFrameStep::Fault
                }
            },
            RetainedPaintPhase::Overlays => match frame.overlay_chrome.get(frame.overlay_index).copied().flatten() {
                Some(placement) => {
                    frame.overlay_index = frame.overlay_index.saturating_add(1);
                    let bounds = semio_ui_theme::geometry::Rect::new(offset_x + placement.x, offset_y + placement.y, placement.width, placement.height);
                    if retained_overlay_chrome_step(&mut frame.candidate, viewport_rect, bounds, placement.backdrop, &theme) {
                        UiFrameStep::Pending
                    } else {
                        frame.phase = RetainedPaintPhase::Fault;
                        frame.fault_site = Some("overlay-chrome");
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
                    frame.paint_node = Some((node, origin_x + offset_x, origin_y + offset_y));
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
                    frame.fault_site = Some("walk-depth-paint");
                    UiFrameStep::Fault
                }
            },
            RetainedPaintPhase::Scenes => match frame.walk.step(&window.tree) {
                RetainedPaintWalkStep::Visit(node, origin_x, origin_y, _) => {
                    let origin_x = origin_x + offset_x;
                    let origin_y = origin_y + offset_y;
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
                    frame.fault_site = Some("walk-depth-scenes");
                    UiFrameStep::Fault
                }
            },
            RetainedPaintPhase::Tooltip => {
                if let Some(tooltip) = frame.tooltip.take() {
                    paint_retained_tooltip(&mut frame.candidate, atlas, tooltip, (offset_x, offset_y), &theme);
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
                        origin_x + offset_x,
                        origin_y + offset_y,
                        retained_node_clip(&window.tree, node).map(|clip| offset_rect(clip, offset_x, offset_y)),
                        overlay_root.is_some(),
                        &mut frame.overlay_index,
                        &mut frame.hit_candidates,
                    );
                    UiFrameStep::Pending
                }
                RetainedPaintWalkStep::Scalar => UiFrameStep::Pending,
                RetainedPaintWalkStep::Complete => {
                    frame.phase = RetainedPaintPhase::Borders;
                    UiFrameStep::Pending
                }
                RetainedPaintWalkStep::DepthFault => {
                    frame.phase = RetainedPaintPhase::Fault;
                    frame.fault_site = Some("walk-depth-hits");
                    UiFrameStep::Fault
                }
            },
            RetainedPaintPhase::Borders => {
                frame.phase = RetainedPaintPhase::Publish;
                UiFrameStep::Pending
            },
            RetainedPaintPhase::Publish => {
                if let Some(token) = frame.append_token {
                    let Ok(demand) = target.next_retained_append_demand(token) else { frame.phase = RetainedPaintPhase::Fault; frame.fault_site = Some("publish-candidate"); return UiFrameStep::Pending; };
                    if !append_admitted(grant, demand) { return UiFrameStep::Pending; }
                    let (result, receipt) = target.step_retained_append(token); *progress = receipt;
                    match result {
                        Ok(semio_ui_draw_contract::RetainedAppendStep::Pending) => { frame.note_progress(); return UiFrameStep::Pending; }
                        Ok(semio_ui_draw_contract::RetainedAppendStep::Complete) => frame.append_token = None,
                        Err(_) => { frame.phase = RetainedPaintPhase::Fault; frame.fault_site = Some("publish-candidate"); return UiFrameStep::Pending; }
                    }
                } else {
                    if !append_admitted(grant, DrawList::retained_append_begin_demand()) { return UiFrameStep::Pending; }
                    let (result, receipt) = target.begin_retained_append(&mut frame.candidate); *progress = receipt;
                    match result { Ok(token) => { frame.append_token = Some(token); frame.note_progress(); }, Err(_) => { frame.phase = RetainedPaintPhase::Fault; frame.fault_site = Some("publish-candidate"); } }
                    return UiFrameStep::Pending;
                }
                window.hit_registry = std::mem::take(&mut frame.hit_candidates);
                window.paint_census = UiFramePaintCensus::of(target).since(frame.baseline);
                window.paint_frame = None;
                UiFrameStep::Ready
            }
            RetainedPaintPhase::Complete => {
                window.paint_frame = None;
                UiFrameStep::Ready
            }
            RetainedPaintPhase::Fault => UiFrameStep::Fault,
        }
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn frame<H: SceneHost>(&mut self, window_id: &str, viewport_width: f32, viewport_height: f32, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, scene_host: Option<&mut H>) -> Option<&DrawList> {
        self.set_viewport(window_id, viewport_width, viewport_height);
        self.publish_overlay_origins(window_id);
        let surface = self.windows.token(window_id)?;
        let window = self.windows.get_mut(window_id).filter(|window| window.closing.is_none())?;
        let root = window.tree.root?;
        let layout_dirty = window.tree.node(root).is_some_and(|node| node.flags.contains(NodeFlags::DIRTY_LAYOUT) || node.flags.contains(NodeFlags::SUBTREE_DIRTY));
        if layout_dirty {
            return Some(&window.draw);
        }
        let dirty = window.tree.node(root).is_some_and(|node| node.flags.contains(NodeFlags::DIRTY_PAINT));
        if !dirty {
            return Some(&window.draw);
        }
        window.draw.clear();
        paint_tree(&mut window.tree, root, &self.theme, atlas, icons, scene_host.is_some(), &mut window.draw);
        if let Some(host) = scene_host {
            for slot in collect_scene_slots(&window.tree, root) {
                let mut cursor = ScenePaintCursor::default();
                while matches!(host.paint_slot_step(&slot, &mut cursor, &mut window.draw, atlas, icons), ScenePaintStep::Pending) {}
            }
        }
        if let Some(tooltip) = retained_tooltip_paint(window, surface, atlas, &self.theme) {
            paint_retained_tooltip(&mut window.draw, atlas, tooltip, (0.0, 0.0), &self.theme);
        }
        loop {
            match window.draw.finalize_shared_borders_step() { Ok(true) => break, Ok(false) => {}, Err(_) => return None }
        }
        Some(&window.draw)
    }

}
//#endregion 🔖️Ui
// #endregion engine
