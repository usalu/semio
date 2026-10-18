//! 🌐️ World content and pointer planning of the normal board host: descriptor sync, cached world content, the vector scene, hover and the pointer plan, commit, cancel and close steps. See [`super`].

use super::*;

impl BoardHost {
    // #region board_host
    /// 🔗️ Authoring sync: every edge the descriptor adds is announced as an `edgeCreate` event.
    pub fn sync_descriptor(&mut self, desc: &SceneDescriptor) -> Result<(), NormalPortError> {
        sync_descriptor_with(&mut self.scene, &mut self.focus, &mut self.style, &mut self.outbox, desc, true)
    }

    /// 🎯️ Target regions are the backdrop: painted before the first entity layer, always in
    /// world space, so nothing they overlap is ever hidden behind them.
    fn append_cached_world_content(&self, scene: &mut Scene, lod: BoardDrawLod) {
        self.closing.advance_opaque_scene_retirement_step();
        let generation = self.scene.content_scene_generation;
        let cam_aff = self.scene.camera_content_affine();
        let overlay_ids = interaction_overlay_entity_ids(&self.scene, &self.focus);
        if !self.scene.regions.is_empty() || self.gestures.region_paint.is_some() {
            let mut region_layer = Scene::new();
            append_target_region_paint(&self.scene, &self.focus, &self.style, &self.gestures, &mut region_layer, true);
            scene.append(&region_layer, Some(cam_aff));
        }
        let mut fill_layer = Scene::new();
        append_nodes_and_handles_with_overlay_chrome(&self.scene, &self.focus, &self.style, &mut fill_layer, None, lod, true, None, &overlay_ids, NodeHandlePaintLayer::Fill);
        scene.append(&fill_layer, Some(cam_aff));
        let mut cache = self.scene.world_content_cache.borrow_mut();
        let needs_rebuild = cache.as_ref().is_none_or(|c| c.generation != generation || c.lod != lod);
        if needs_rebuild && self.closing.opaque_scene_retirement.get().is_none() {
            if cache.is_some() {
                let Some(token) = semio_framework_canvas::reserve_opaque_scene_retirement() else {
                    self.closing.opaque_scene_fault.set(true);
                    return;
                };
                let WorldContentCache { scene: stale, .. } = cache.take().expect("stale world content cache was witnessed occupied");
                self.closing.publish_opaque_scene_retirement(token, stale);
            }
            let mut content = Scene::new();
            let spans = bake_node_icon_spans(&self.scene, &self.focus, &self.style, &mut content, lod);
            *cache = Some(WorldContentCache { generation, lod, scene: content, spans });
        }
        if let Some(cached) = cache.as_ref() {
            self.scene.append_baked_node_icons(scene, cached, cam_aff);
        }
        let edges_in_world_space = matches!(lod, BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Minimap);
        if edges_in_world_space {
            let mut edge_layer = Scene::new();
            append_edges_wires_and_link(&self.scene, &self.focus, &self.style, &mut edge_layer, None, lod, true, None, Some(&overlay_ids));
            scene.append(&edge_layer, Some(cam_aff));
        } else {
            append_edges_wires_and_link(&self.scene, &self.focus, &self.style, scene, None, lod, false, None, Some(&overlay_ids));
        }
        let mut stroke_layer = Scene::new();
        append_nodes_and_handles_with_overlay_chrome(&self.scene, &self.focus, &self.style, &mut stroke_layer, None, lod, false, None, &overlay_ids, NodeHandlePaintLayer::Stroke);
        scene.append(&stroke_layer, None);
        if let Some(c) = active_link_wire_curve(&self.scene, &self.focus, &self.style) {
            let link_wire_stroke = Stroke::new(ui_styling::strokes::WIRE_HIGHLIGHT);
            let link_wire_color = self.style.canvas_theme.node_stroke;
            let p0 = self.scene.draw_space_point(c.p0(), false);
            let p1 = self.scene.draw_space_point(c.p1(), false);
            let p2 = self.scene.draw_space_point(c.p2(), false);
            let p3 = self.scene.draw_space_point(c.p3(), false);
            let curve = CubicBez::new(p0, p1, p2, p3);
            scene.stroke(&link_wire_stroke, Affine::IDENTITY, link_wire_color, None, &curve);
        }
        if self.style.has_ports() {
            if let Some(node_id) = indirect_ring_node_id(&self.scene, &self.focus, &self.style, lod) {
                append_indirect_handle_ring(&self.scene, &self.focus, &self.style, scene, None, &node_id, StyleChromePass::CachedBase, false);
            }
        }
        let previews_in_world_space = matches!(lod, BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Minimap);
        if previews_in_world_space {
            let mut preview_layer = Scene::new();
            if self.brush.drop_preview.is_some() {
                append_drop_preview_paint(&self.scene, &self.style, &self.brush, &mut preview_layer, lod, true);
            }
            if self.focus.active_utility == ActiveUtility::Brush || self.brush.brush_preview.is_some() {
                append_brush_preview_paint(&self.scene, &self.style, &self.brush, &mut preview_layer, lod, true);
            }
            self.append_transform_gumball_paint(&mut preview_layer, true);
            scene.append(&preview_layer, Some(cam_aff));
        } else {
            if self.brush.drop_preview.is_some() {
                append_drop_preview_paint(&self.scene, &self.style, &self.brush, scene, lod, false);
            }
            if self.focus.active_utility == ActiveUtility::Brush || self.brush.brush_preview.is_some() {
                append_brush_preview_paint(&self.scene, &self.style, &self.brush, scene, lod, false);
            }
            self.append_transform_gumball_paint(scene, false);
        }
    }

    pub fn build_vector_scene(&self) -> Scene {
        let mut inner = Scene::new();
        let lod = board_draw_lod_for_frame(&self.scene, &self.style);
        if self.style.grid_visible && !self.style.wheel_zoom_active {
            let grid_color = self.style.canvas_theme.grid_minor_stroke;
            if lod != BoardDrawLod::Minimap {
                self.scene.stroke_world_step_grid(&mut inner, grid_color, ui_styling::strokes::GRID_LARGE, self.style.grid_step_large_world(), 0.0);
                match lod {
                    BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro => {
                        self.scene.stroke_world_step_grid(&mut inner, grid_color, ui_styling::strokes::GRID_MEDIUM, self.style.grid_step_medium_world(), 0.0);
                    }
                    BoardDrawLod::Minimap | BoardDrawLod::Overview | BoardDrawLod::Compact => {}
                }
                if matches!(lod, BoardDrawLod::Detail | BoardDrawLod::Micro) {
                    self.scene.stroke_world_step_grid(&mut inner, grid_color, ui_styling::strokes::GRID_SMALL, self.style.grid_step_small_world(), 0.0);
                }
                if lod == BoardDrawLod::Micro {
                    self.scene.stroke_world_step_grid(&mut inner, grid_color, ui_styling::strokes::GRID_MICRO, self.style.grid_step_micro_world(), 0.0);
                }
            }
        }
        if let Some(ref pts) = self.focus.selection_screen_preview {
            if pts.len() >= 2 {
                let mut path = semio_framework_canvas::BezPath::new();
                path.move_to(pts[0]);
                for p in pts.iter().skip(1) {
                    path.line_to(*p);
                }
                path.close_path();
                inner.fill(FillRule::NonZero, Affine::IDENTITY, self.style.canvas_theme.selection_preview_fill, None, &path);
                let mut preview_stroke = Stroke::new(ui_styling::strokes::SELECTION_PREVIEW);
                if self.focus.selection_preview_crossing {
                    preview_stroke.set_dash_pattern(vec![5.0, 4.0]);
                }
                inner.stroke(&preview_stroke, Affine::IDENTITY, self.style.canvas_theme.selection_preview_stroke, None, &path);
            }
        }
        self.append_cached_world_content(&mut inner, lod);
        let scale = self.scene.dpr.max(1.0);
        if (scale - 1.0).abs() < f64::EPSILON {
            inner
        } else {
            let mut scene = Scene::new();
            scene.append(&inner, Some(Affine::IDENTITY.scale(scale)));
            scene
        }
    }

    pub fn encoded_scene_hint(&self) -> usize {
        let s = self.build_vector_scene();
        s.path_count()
    }

    pub fn update_hover_from_world(&mut self, world: Point) {
        let next = resolve_hover_world(&self.scene, &self.focus, &self.style, world);
        set_hovered_id(&mut self.scene, &mut self.focus, &mut self.outbox, next);
    }

    pub fn plan_pointer(&self, intent: BoardPointerIntent) -> Result<BoardPointerPlan, BoardPointerPlanFault> {
        if self.scene.interaction_revision == u64::MAX || (matches!(intent.phase, BoardPointerPhase::Move | BoardPointerPhase::Up) && pointer_lane_is_direct(&self.focus, &self.gestures)) {
            return Err(BoardPointerPlanFault::Unsupported);
        }
        let screen = Point::new(intent.x, intent.y);
        let world = self.scene.screen_to_world(screen);
        if self.focus.active_utility == ActiveUtility::Brush && !matches!(self.focus.interaction, Interaction::Pan { .. }) {
            return plan_brush_pointer(&self.scene, &self.focus, &self.style, &self.brush, intent, world);
        }
        match (intent.phase, &self.focus.interaction) {
            (BoardPointerPhase::Move, Interaction::Pan { origin, start_screen }) => {
                let delta = screen - *start_screen;
                let camera = [origin.x - delta.x / origin.zoom, origin.y - delta.y / origin.zoom, origin.zoom];
                Ok(BoardPointerPlan::empty(self.scene.interaction_revision, BoardPointerPlanKind::Pan { camera }))
            }
            (BoardPointerPhase::Move, Interaction::DragNodes { primary_id, offset, start_positions, .. }) => plan_drag_pointer(&self.scene, &self.style, world, (primary_id, *offset, start_positions), None),
            (_, Interaction::SelectionPending { initial_ids, start, start_screen }) => plan_selection_pending_pointer(&self.scene, &self.focus, &self.style, intent, initial_ids, *start, *start_screen, screen, world),
            (_, Interaction::Selection { initial_ids, points, screen_points, start, start_screen }) => plan_selection_pointer(&self.scene, &self.focus, &self.style, intent, initial_ids, points, screen_points, *start, *start_screen, screen, world),
            (BoardPointerPhase::Move, Interaction::LinkAtSourceHandle { source_id, start_screen }) => plan_link_move_pointer(&self.scene, &self.focus, &self.style, &self.outbox, source_id, Some(*start_screen), screen, world),
            (BoardPointerPhase::Move, Interaction::LinkDragSnap { source_id, .. }) => plan_link_move_pointer(&self.scene, &self.focus, &self.style, &self.outbox, source_id, None, screen, world),
            (BoardPointerPhase::Up, Interaction::LinkDragSnap { source_id, target_id, .. }) => plan_link_finish_pointer(&self.scene, &self.focus, &self.style, &self.outbox, source_id, target_id.as_deref(), world),
            (BoardPointerPhase::Up, Interaction::LinkAtSourceHandle { source_id, .. }) => plan_link_finish_pointer(&self.scene, &self.focus, &self.style, &self.outbox, source_id, None, world),
            (BoardPointerPhase::Move, Interaction::LinkTargetNode { .. } | Interaction::ExternalLinkPreview { .. }) => plan_link_retain_pointer(&self.scene, &self.focus, &self.style, world),
            (BoardPointerPhase::Up, Interaction::LinkTargetNode { source_id, .. }) => plan_link_clear_pointer(&self.scene, &self.focus, &self.style, &self.outbox, source_id, world),
            (BoardPointerPhase::Up, Interaction::ExternalLinkPreview { .. }) => plan_link_retain_pointer(&self.scene, &self.focus, &self.style, world),
            (BoardPointerPhase::Move, Interaction::None) => plan_hover_pointer(&self.scene, &self.focus, resolve_hover_world(&self.scene, &self.focus, &self.style, world).as_deref()),
            (BoardPointerPhase::Up, Interaction::Pan { origin, start_screen }) => {
                let delta = screen - *start_screen;
                let camera = [origin.x - delta.x / origin.zoom, origin.y - delta.y / origin.zoom, origin.zoom];
                Ok(BoardPointerPlan::empty(self.scene.interaction_revision, BoardPointerPlanKind::FinishPan { camera }))
            }
            (BoardPointerPhase::Up, Interaction::DragNodes { primary_id, offset, start_positions, proximity_pair, gesture, .. }) => plan_drag_pointer(&self.scene, &self.style, world, (primary_id, *offset, start_positions), Some((gesture, proximity_pair.as_ref()))),
            (BoardPointerPhase::Up, Interaction::None) => Ok(BoardPointerPlan::empty(self.scene.interaction_revision, BoardPointerPlanKind::Idle)),
            (BoardPointerPhase::Leave, Interaction::None) => plan_hover_pointer(&self.scene, &self.focus, None),
            (BoardPointerPhase::Leave, _) => Ok(BoardPointerPlan::empty(self.scene.interaction_revision, BoardPointerPlanKind::Idle)),
        }
    }

    pub fn step_pointer_commit(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> BoardAuthorityStep {
        let Some(mut operation) = self.commit.pending_pointer_commit.take() else {
            return BoardAuthorityStep::Complete;
        };
        if context.should_yield() {
            self.commit.pending_pointer_commit = Some(operation);
            return BoardAuthorityStep::Pending;
        }
        if context.is_cancelled() && operation.phase == 0 {
            operation.cancelling = true;
        }
        if self.scene.interaction_revision != operation.plan.revision && operation.phase == 0 {
            operation.cancelling = true;
            operation.faulted = true;
        }
        if operation.cancelling {
            if operation.cursor < operation.plan.delta_len {
                operation.plan.deltas[usize::from(operation.cursor)] = None;
                operation.cursor += 1;
                context.consume_fuel(1);
                self.commit.pending_pointer_commit = Some(operation);
                return BoardAuthorityStep::Pending;
            }
            if operation.retire_one() || operation.points.pop().is_some() || operation.screen_points.pop().is_some() || operation.overlay_points.pop().is_some() {
                context.consume_fuel(1);
                self.commit.pending_pointer_commit = Some(operation);
                return BoardAuthorityStep::Pending;
            }
            if operation.scan_after.take().is_some() {
                context.consume_fuel(1);
                self.commit.pending_pointer_commit = Some(operation);
                return BoardAuthorityStep::Pending;
            }
            return if operation.faulted { BoardAuthorityStep::Fault } else { BoardAuthorityStep::Cancelled };
        }
        let complete = step_pointer_commit_operation(&mut self.scene, &mut self.focus, &mut self.style, &mut self.brush, &mut self.outbox, &mut operation);
        context.consume_fuel(1);
        if !complete {
            self.commit.pending_pointer_commit = Some(operation);
            return BoardAuthorityStep::Pending;
        }
        if operation.faulted {
            return BoardAuthorityStep::Fault;
        }
        let completed_revision = operation.plan.revision;
        if self.scene.interaction_revision != completed_revision {
            return BoardAuthorityStep::Fault;
        }
        self.scene.interaction_revision = completed_revision.wrapping_add(1);
        if operation.plan.output_len > 2 {
            self.commit.pointer_publication = Some(BoardPointerPublication { bytes: operation.plan.output, len: operation.plan.output_len });
        }
        if let Some(mut queued) = self.commit.queued_pointer_commit.take() {
            if queued.revision != completed_revision {
                return BoardAuthorityStep::Fault;
            }
            queued.revision = self.scene.interaction_revision;
            self.commit.pending_pointer_commit = Some(BoardPointerCommitOperation::new(queued));
            return BoardAuthorityStep::Pending;
        }
        BoardAuthorityStep::Complete
    }

    pub fn step_pointer_cancel(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> bool {
        if self.commit.pending_pointer_commit.is_some() {
            let _ = self.step_pointer_commit(context);
            return false;
        }
        let Some(retirement) = self.commit.cancelled_pointer_plan_retirement.as_mut() else {
            return true;
        };
        if retirement.close_step() {
            self.commit.cancelled_pointer_plan_retirement = None;
        }
        context.consume_fuel(1);
        false
    }

    pub fn close_pointer_authority_step(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> bool {
        if let Some(operation) = self.commit.pending_pointer_commit.as_mut() {
            if operation.cursor == 0 {
                operation.cancelling = true;
            }
            let _ = self.step_pointer_commit(context);
            return false;
        }
        if self.commit.queued_pointer_commit.take().is_some() {
            context.consume_fuel(1);
            return false;
        }
        if let Some(retirement) = self.commit.cancelled_pointer_plan_retirement.as_mut() {
            if retirement.close_step() {
                self.commit.cancelled_pointer_plan_retirement = None;
            }
            context.consume_fuel(1);
            return false;
        }
        if let Some(publication) = self.commit.pointer_publication.as_mut() {
            if publication.close_step() {
                self.commit.pointer_publication = None;
            }
            return false;
        }
        true
    }
}
