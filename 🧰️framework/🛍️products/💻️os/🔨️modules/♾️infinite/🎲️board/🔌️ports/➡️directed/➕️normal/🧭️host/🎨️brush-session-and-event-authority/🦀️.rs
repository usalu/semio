//! 🎨️ Brush session and event authority of the normal board host: the brush session mirror, active utility, suggestion offset, brush kind weights and slots and the event authority steps. See [`super`].

use super::*;

    // #region board_host
    //#region 🧵️FillJob
impl BoardHost {
    /// 🖌️ Mirrors brush slot + preview from another authoring pane (no pointer input on this host).
    pub fn set_brush_session_mirror_json(&mut self, json: &str) -> Result<(), NormalPortError> {
        if json.trim().is_empty() {
            self.brush.brush_slot_suggestions_active = false;
            self.brush.brush_slot_source_id = None;
            self.brush.brush_candidates.clear();
            self.brush.brush_candidate_index = 0;
            self.brush.brush_preview = None;
            self.brush.brush_preview_emit_key = None;
            self.brush.brush_candidates_emit_key = None;
            self.scene.bump_content_scene_generation();
            return Ok(());
        }
        if json.len() > BOARD_EVENT_BYTE_CAPACITY {
            return Err(NormalPortError::EventCredits);
        }
        let v: serde_json::Value = serde_json::from_str(json).map_err(NormalPortError::BrushSessionJson)?;
        let source = v.get("sourceHandleId").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
        self.brush.brush_slot_source_id = source.clone();
        let mut candidates = BrushCandidatePage::default();
        if let Some(rows) = v.get("candidates").and_then(|x| x.as_array()) {
            if rows.len() > BOARD_POINTER_ITEM_CAPACITY {
                return Err(NormalPortError::EventCredits);
            }
            for row in rows {
                let (node_kind_id, target_handle_index) = if let Some(node_kind_id) = row.as_str().map(str::trim).filter(|value| !value.is_empty()) {
                    (node_kind_id, 0)
                } else {
                    let node_kind_id = row.get("nodeKind").or_else(|| row.get("nodeKindId")).and_then(|value| value.as_str()).map(str::trim).filter(|value| !value.is_empty()).ok_or(NormalPortError::EventCredits)?;
                    let target_handle_index = row.get("targetHandleIndex").and_then(|value| value.as_u64()).unwrap_or(0);
                    let target_handle_index = usize::try_from(target_handle_index).map_err(|_| NormalPortError::EventCredits)?;
                    (node_kind_id, target_handle_index)
                };
                candidates.push(node_kind_id, target_handle_index, 0.0).map_err(|_| NormalPortError::EventCredits)?;
            }
        }
        self.brush.brush_candidates = candidates;
        self.brush.brush_candidate_index = v.get("index").and_then(|x| x.as_u64()).map_or(0, |i| i as usize);
        if self.brush.brush_candidates.is_empty() {
            self.brush.brush_candidate_index = 0;
        } else {
            self.brush.brush_candidate_index %= self.brush.brush_candidates.len();
        }
        self.brush.brush_preview = match (source.as_deref(), v.get("preview")) {
            (Some(source_id), Some(preview)) if !preview.is_null() => {
                let node = preview.get("node").filter(|n| !n.is_null());
                let edge = preview.get("edge").filter(|e| !e.is_null());
                match (node, edge) {
                    (Some(node), Some(edge)) => brush_preview_snapshot_from_session_json(node, edge, source_id),
                    _ => None,
                }
            }
            _ => None,
        };
        self.brush.brush_preview_emit_key = None;
        self.brush.brush_candidates_emit_key = None;
        self.brush.brush_slot_suggestions_active = v.get("suggestionsActive").and_then(|x| x.as_bool()).unwrap_or(false);
        if self.brush.brush_preview.is_none() && !self.brush.brush_candidates.is_empty() {
            brush_rebuild_preview(&mut self.scene, &mut self.style, &mut self.brush, &mut self.outbox);
        } else {
            brush_sync_preview_events(&mut self.brush, &mut self.outbox);
        }
        self.scene.bump_content_scene_generation();
        Ok(())
    }

    pub fn set_active_utility(&mut self, label: &str) {
        let next = match label {
            "brush" => ActiveUtility::Brush,
            "areaBrush" => ActiveUtility::AreaBrush,
            _ => ActiveUtility::Select,
        };
        if self.focus.active_utility == next {
            return;
        }
        if self.focus.active_utility == ActiveUtility::Brush {
            brush_finish_slot(&mut self.scene, &mut self.focus, &mut self.style, &mut self.brush, &mut self.outbox);
        }
        cancel_region_paint(&mut self.scene, &mut self.gestures);
        cancel_region_drag(&mut self.scene, &mut self.focus, &mut self.gestures);
        self.focus.active_utility = next;
        self.focus.interaction = Interaction::None;
        self.scene.bump_content_scene_generation();
    }

    pub fn set_suggestion_offset(&mut self, distance: f64) {
        let d = if distance.is_finite() && distance >= 0.0 { distance } else { DEFAULT_SUGGESTION_OFFSET };
        if (self.brush.suggestion_offset - d).abs() < 1e-9 {
            return;
        }
        self.brush.suggestion_offset = d;
        if self.focus.active_utility == ActiveUtility::Brush {
            self.brush.brush_preview_emit_key = None;
            brush_rebuild_preview(&mut self.scene, &mut self.style, &mut self.brush, &mut self.outbox);
        }
    }

    pub fn set_brush_kind_weights(&mut self, json: &str) {
        if json.is_empty() {
            return;
        }
        let Ok(v) = serde_json::from_str::<serde_json::Value>(json) else {
            return;
        };
        self.brush.brush_node_kind_weights.clear();
        self.brush.brush_handle_kind_weights.clear();
        if let Some(obj) = v.get("nodeWeights").and_then(|x| x.as_object()) {
            for (k, val) in obj {
                if let Some(w) = val.as_f64() {
                    if w.is_finite() && w >= 0.0 {
                        self.brush.brush_node_kind_weights.insert(k.clone(), w);
                    }
                }
            }
        }
        if let Some(obj) = v.get("handleWeights").and_then(|x| x.as_object()) {
            for (k, val) in obj {
                if let Some(w) = val.as_f64() {
                    if w.is_finite() && w >= 0.0 {
                        self.brush.brush_handle_kind_weights.insert(k.clone(), w);
                    }
                }
            }
        }
        if self.focus.active_utility != ActiveUtility::Brush {
            return;
        }
        if let Some(source) = self.brush.brush_slot_source_id.clone() {
            self.brush.brush_slot_source_id = None;
            brush_enter_slot(&mut self.scene, &mut self.focus, &mut self.style, &mut self.brush, &mut self.outbox, &source);
        } else {
            self.brush.brush_preview_emit_key = None;
            brush_rebuild_preview(&mut self.scene, &mut self.style, &mut self.brush, &mut self.outbox);
        }
    }

    pub fn set_brush_node_size(&mut self, size: f64) {
        let s = if size.is_finite() && size > 0.0 { size } else { DEFAULT_BRUSH_NODE_SIZE };
        if (self.brush.brush_node_size - s).abs() < 1e-9 {
            return;
        }
        self.brush.brush_node_size = s;
        if self.focus.active_utility == ActiveUtility::Brush {
            self.brush.brush_preview_emit_key = None;
            brush_rebuild_preview(&mut self.scene, &mut self.style, &mut self.brush, &mut self.outbox);
        }
    }

    pub fn brush_cycle_candidate(&mut self, forward: bool) {
        if self.brush.brush_candidates.len() < 2 {
            return;
        }
        let len = self.brush.brush_candidates.len();
        self.brush.brush_candidate_index = if forward { (self.brush.brush_candidate_index + 1) % len } else { (self.brush.brush_candidate_index + len - 1) % len };
        brush_rebuild_preview(&mut self.scene, &mut self.style, &mut self.brush, &mut self.outbox);
    }

    pub fn brush_set_candidate_index(&mut self, index: usize) {
        if self.brush.brush_candidates.is_empty() {
            return;
        }
        self.brush.brush_candidate_index = index % self.brush.brush_candidates.len();
        brush_rebuild_preview(&mut self.scene, &mut self.style, &mut self.brush, &mut self.outbox);
    }

    /// 🖌️ Opens a brush slot on a free handle (suggestions menu; works outside brush utility).
    pub fn brush_open_slot(&mut self, handle_id: &str) {
        if !self.scene.handles.contains_key(handle_id) {
            return;
        }
        brush_enter_slot(&mut self.scene, &mut self.focus, &mut self.style, &mut self.brush, &mut self.outbox, handle_id);
        self.brush.brush_slot_suggestions_active = true;
        brush_rebuild_preview(&mut self.scene, &mut self.style, &mut self.brush, &mut self.outbox);
        set_hovered_id(&mut self.scene, &mut self.focus, &mut self.outbox, Some(handle_id.to_string()));
    }

    /// 🎣️ Points the brush slot at a handle, or at nothing, WITHOUT opening the suggestions
    /// popup — the armed brush's own targeting channel, so a program driving the brush headlessly
    /// resolves the same candidate page a pointer hover would without claiming the popup's hover.
    pub fn brush_target_slot(&mut self, handle_id: Option<&str>) {
        match handle_id.filter(|handle_id| self.scene.handles.contains_key(*handle_id)) {
            Some(handle_id) => {
                brush_enter_slot(&mut self.scene, &mut self.focus, &mut self.style, &mut self.brush, &mut self.outbox, handle_id);
                brush_rebuild_preview(&mut self.scene, &mut self.style, &mut self.brush, &mut self.outbox);
            }
            None => brush_clear_slot(&mut self.scene, &mut self.focus, &mut self.brush, &mut self.outbox),
        }
    }

    /// 🖌️ Commits the active brush preview and clears the slot.
    pub fn brush_commit_slot(&mut self) {
        brush_commit_preview(&mut self.scene, &mut self.style, &mut self.brush, &mut self.outbox);
        brush_clear_slot(&mut self.scene, &mut self.focus, &mut self.brush, &mut self.outbox);
    }

    /// 🖌️ Discards the active brush slot without placing.
    pub fn brush_cancel_slot(&mut self) {
        brush_clear_slot(&mut self.scene, &mut self.focus, &mut self.brush, &mut self.outbox);
    }

    pub fn step_event_authority(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> BoardAuthorityStep {
        if context.should_yield() {
            return BoardAuthorityStep::Pending;
        }
        let faulted_before = self.outbox.event_schema_fault;
        if self.commit.pending_delete_planning.is_some() {
            if context.is_cancelled() {
                if let Some(planning) = self.commit.pending_delete_planning.as_mut() {
                    planning.cancelling = true;
                }
            }
            let result = step_delete_planning(&mut self.scene, &mut self.focus, &mut self.style, &mut self.outbox, &mut self.commit);
            context.consume_fuel(1);
            return result;
        }
        if self.commit.pending_delete_operation.is_some() {
            if context.is_cancelled() {
                if let Some(operation) = self.commit.pending_delete_operation.as_mut() {
                    if !operation.claimed {
                        operation.cancelling = true;
                    }
                }
            }
            if self.commit.pending_delete_operation.as_ref().is_some_and(|operation| operation.cancelling && !operation.claimed) {
                self.commit.pending_delete_operation = None;
                context.consume_fuel(1);
                return BoardAuthorityStep::Cancelled;
            }
            step_pending_delete_operation(&mut self.scene, &mut self.focus, &mut self.outbox, &mut self.commit);
            context.consume_fuel(1);
            if self.outbox.event_schema_fault && !faulted_before {
                return BoardAuthorityStep::Fault;
            }
            return if self.commit.pending_delete_operation.is_some() { BoardAuthorityStep::Pending } else { BoardAuthorityStep::Complete };
        }
        BoardAuthorityStep::Complete
    }

    pub fn close_event_authority_step(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> bool {
        if context.should_yield() {
            return false;
        }
        if let Some(planning) = self.commit.pending_delete_planning.as_mut() {
            planning.cancelling = true;
        }
        if self.commit.pending_delete_planning.is_some() {
            let _ = step_delete_planning(&mut self.scene, &mut self.focus, &mut self.style, &mut self.outbox, &mut self.commit);
            context.consume_fuel(1);
            return false;
        }
        if let Some(operation) = self.commit.pending_delete_operation.as_mut() {
            if !operation.claimed {
                operation.cancelling = true;
            }
        }
        if self.commit.pending_delete_operation.is_some() {
            let _ = self.step_event_authority(context);
            return false;
        }
        if self.outbox.event_overflow.take().is_some() {
            context.consume_fuel(1);
            return false;
        }
        if let Some(batch) = self.outbox.event_batch_overflow.as_mut() {
            let _ = batch.pop();
            if batch.is_empty() {
                self.outbox.event_batch_overflow = None;
            }
            context.consume_fuel(1);
            return false;
        }
        let closed = self.outbox.events.close_step();
        context.consume_fuel(1);
        closed
    }

    pub(crate) fn apply_link_drag_snap_hover(&mut self, _source_handle_id: &str, world: Point, target_handle_id: Option<&str>) {
        if let Some(tid) = target_handle_id {
            set_hovered_id(&mut self.scene, &mut self.focus, &mut self.outbox, Some(tid.to_string()));
        } else {
            self.update_hover_from_world(world);
        }
    }

    /// 🎯️ All pick targets under a screen point as JSON (`domain`, `id`, `generality`).
    pub fn pick_targets_at_screen_json(&self, sx: f64, sy: f64) -> String {
        let world = self.scene.screen_to_world(Point::new(sx, sy));
        serde_json::to_string(&resolve_pick_targets_world(&self.scene, &self.focus, &self.style, &self.gestures, world)).unwrap_or_else(|_| "[]".into())
    }
}
