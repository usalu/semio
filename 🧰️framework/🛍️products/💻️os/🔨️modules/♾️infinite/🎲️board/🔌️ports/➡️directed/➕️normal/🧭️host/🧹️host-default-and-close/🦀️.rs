//! 🧹️ Host default and close of the normal board host: the default host, the non-opaque close step and the terminal emptiness check. See [`super`].

use super::*;

// #region board_host
impl Default for BoardHost {
    fn default() -> Self {
        Self {
            scene: BoardSceneState {
                camera: Camera::default(),
                nodes: BTreeMap::new(),
                handles: BTreeMap::new(),
                edges: BTreeMap::new(),
                wires: BTreeMap::new(),
                regions: BTreeMap::new(),
                width: 1,
                height: 1,
                dpr: 1.0,
                content_scene_generation: 0,
                world_content_cache: RefCell::new(None),
                interaction_revision: 0,
            },
            focus: BoardFocusState {
                selection: BTreeSet::new(),
                preselect: BTreeSet::new(),
                preselect_removed: BTreeSet::new(),
                selection_exit_highlight: BTreeSet::new(),
                selection_options: SelectionOptions { method: "rectangle".into(), mode: "replace".into(), select_nodes: true, select_edges: true, select_handles: true },
                hovered_id: None,
                hovered_kind: None,
                highlighted_ids: BTreeSet::new(),
                interaction: Interaction::None,
                selection_screen_preview: None,
                selection_preview_crossing: false,
                active_utility: ActiveUtility::Select,
            },
            style: BoardStyleState {
                handle_kinds: BTreeMap::new(),
                wire_kinds: BTreeMap::new(),
                node_kinds: BTreeMap::new(),
                edge_kinds: BTreeMap::new(),
                edge_tips: builtin_edge_tips(),
                link_compat_rules: Vec::new(),
                world_raster_tiling: "world-clip".into(),
                canvas_theme: CanvasPalette::default(),
                grid_visible: true,
                grid_factor: GRID_FACTOR_DEFAULT,
                grid_snap_enabled: false,
                preserve_original_element_style: false,
                automatic_lod: true,
                forced_draw_lod: None,
                icon_paint_cache: IconPaintCache::new(),
                wheel_zoom_active: false,
                wheel_zoom_render_lod: None,
                naming: &NEUTRAL_BOARD_NAMING,
                placement_naming: None,
                port_mode: GraphPortMode::Ported,
            },
            brush: BoardBrushState {
                suggestion_offset: DEFAULT_SUGGESTION_OFFSET,
                brush_node_size: DEFAULT_BRUSH_NODE_SIZE,
                brush_slot_source_id: None,
                brush_candidates: BrushCandidatePage::default(),
                brush_candidate_index: 0,
                brush_preview: None,
                drop_preview: None,
                brush_candidates_emit_key: None,
                brush_preview_emit_key: None,
                brush_placement_serial: 0,
                brush_node_kind_weights: HashMap::new(),
                brush_handle_kind_weights: HashMap::new(),
                brush_alt_pressed: false,
                brush_slot_suggestions_active: false,
            },
            outbox: BoardOutboxState {
                events: BoardEventQueue::default(),
                event_overflow: None,
                event_batch_overflow: None,
                event_schema_fault: false,
                link_compat_nodes_emit_key: None,
                link_target_ring_emit_key: None,
                last_select_emit_sig: None,
                last_preselect_emit_sig: None,
                gesture_serial: 0,
            },
            gestures: BoardGesturesState {
                link_screen_preview: None,
                transform_flags: TransformGumballFlags::default(),
                transform_drag: None,
                region_drag: None,
                region_paint: None,
                area_brush_extent: (REGION_DEFAULT_BRUSH_EXTENT_WORLD, REGION_DEFAULT_BRUSH_EXTENT_WORLD),
            },
            commit: BoardCommitState {
                pending_delete_planning: None,
                pending_delete_operation: None,
                pending_pointer_commit: None,
                queued_pointer_commit: None,
                cancelled_pointer_plan_retirement: None,
                pointer_publication: None,
            },
            closing: BoardClosingState {
                opaque_scene_retirement: Cell::new(None),
                opaque_scene_fault: Cell::new(false),
                close_phase: BoardHostClosePhase::Events,
                close_entity_retirement: None,
                close_strings: std::array::from_fn(|_| None),
                close_string_len: 0,
                close_node_handles: None,
            },
        }
    }
}

impl BoardHost {
    pub fn close_nonopaque_step(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> bool {
        if context.should_yield() {
            return false;
        }
        if self.closing.close_scratch_step() {
            context.consume_fuel(1);
            return false;
        }
        if let Some(retirement) = self.closing.close_entity_retirement.as_mut() {
            match retirement.step() {
                Ok(true) => self.closing.close_entity_retirement = None,
                Ok(false) => {}
                Err(()) => {
                    self.outbox.event_schema_fault = true;
                    return false;
                }
            }
            context.consume_fuel(1);
            return false;
        }
        match self.closing.close_phase {
            BoardHostClosePhase::Events => {
                if self.close_event_authority_step(context) {
                    self.closing.close_phase = BoardHostClosePhase::Pointer;
                }
                return false;
            }
            BoardHostClosePhase::Pointer => {
                if self.close_pointer_authority_step(context) {
                    self.closing.close_phase = BoardHostClosePhase::WorldScene;
                }
                return false;
            }
            BoardHostClosePhase::WorldScene => {
                if quarantine_world_content_step(&mut self.scene, &mut self.closing) {
                    self.closing.close_phase = BoardHostClosePhase::Icons;
                }
            }
            BoardHostClosePhase::Icons => {
                if self.style.icon_paint_cache.close_step() {
                    self.closing.close_phase = BoardHostClosePhase::Nodes;
                }
            }
            BoardHostClosePhase::Nodes => {
                if let Some((key, value)) = self.scene.nodes.pop_first() {
                    drop(key);
                    self.closing.close_entity_retirement = Some(BoardEntityRetirement::new(BoardRemovedEntity::Node(value)));
                } else {
                    self.closing.close_phase = BoardHostClosePhase::Handles;
                }
            }
            BoardHostClosePhase::Handles => {
                if let Some((key, value)) = self.scene.handles.pop_first() {
                    drop(key);
                    self.closing.close_entity_retirement = Some(BoardEntityRetirement::new(BoardRemovedEntity::Handle(value)));
                } else {
                    self.closing.close_phase = BoardHostClosePhase::Edges;
                }
            }
            BoardHostClosePhase::Edges => {
                if let Some((key, value)) = self.scene.edges.pop_first() {
                    drop(key);
                    self.closing.close_entity_retirement = Some(BoardEntityRetirement::new(BoardRemovedEntity::Edge(value)));
                } else {
                    self.closing.close_phase = BoardHostClosePhase::Wires;
                }
            }
            BoardHostClosePhase::Wires => {
                if let Some((key, value)) = self.scene.wires.pop_first() {
                    drop(key);
                    self.closing.close_entity_retirement = Some(BoardEntityRetirement::new(BoardRemovedEntity::Wire(value)));
                } else {
                    self.closing.close_phase = BoardHostClosePhase::Regions;
                }
            }
            BoardHostClosePhase::Regions => {
                if let Some((key, value)) = self.scene.regions.pop_first() {
                    self.closing.push_close_string(key);
                    self.closing.push_close_string(value.id);
                    self.closing.push_close_optional_string(value.label);
                } else if let Some(id) = self.gestures.region_drag.as_mut().and_then(|drag| drag.gesture.restore.as_mut()).and_then(BTreeSet::pop_first) {
                    self.closing.push_close_string(id);
                } else if let Some(drag) = self.gestures.region_drag.take() {
                    self.closing.push_close_string(drag.id);
                    self.closing.push_close_string(drag.gesture.id);
                } else {
                    self.gestures.region_paint = None;
                    self.closing.close_phase = BoardHostClosePhase::Selection;
                }
            }
            BoardHostClosePhase::Selection => {
                if self.focus.selection.pop_first().is_none() {
                    self.closing.close_phase = BoardHostClosePhase::Preselect;
                }
            }
            BoardHostClosePhase::Preselect => {
                if self.focus.preselect.pop_first().is_none() {
                    self.closing.close_phase = BoardHostClosePhase::PreselectRemoved;
                }
            }
            BoardHostClosePhase::PreselectRemoved => {
                if self.focus.preselect_removed.pop_first().is_none() {
                    self.closing.close_phase = BoardHostClosePhase::SelectionExit;
                }
            }
            BoardHostClosePhase::SelectionExit => {
                if self.focus.selection_exit_highlight.pop_first().is_none() {
                    self.closing.close_phase = BoardHostClosePhase::Highlighted;
                }
            }
            BoardHostClosePhase::Highlighted => {
                if self.focus.highlighted_ids.pop_first().is_none() {
                    self.closing.close_phase = BoardHostClosePhase::Interaction;
                }
            }
            BoardHostClosePhase::Interaction => {
                if self.focus.close_interaction_step() {
                    self.closing.close_phase = BoardHostClosePhase::HandleKinds;
                }
            }
            BoardHostClosePhase::HandleKinds => {
                if let Some((key, value)) = self.style.handle_kinds.pop_first() {
                    self.closing.push_close_string(key);
                    self.closing.push_close_string(value.name);
                    self.closing.push_close_optional_string(value.default_wire_kind);
                } else {
                    self.closing.close_phase = BoardHostClosePhase::WireKinds;
                }
            }
            BoardHostClosePhase::WireKinds => {
                if let Some((key, value)) = self.style.wire_kinds.pop_first() {
                    self.closing.push_close_string(key);
                    self.closing.push_close_string(value.name);
                    self.closing.push_close_optional_string(value.default_edge_kind);
                } else {
                    self.closing.close_phase = BoardHostClosePhase::NodeKinds;
                }
            }
            BoardHostClosePhase::NodeKinds => {
                if let Some((key, value)) = self.style.node_kinds.pop_first() {
                    self.closing.push_close_string(key);
                    self.closing.push_close_string(value.name);
                    self.closing.push_close_optional_string(value.icon);
                    self.closing.close_node_handles = Some(value.handles);
                } else {
                    self.closing.close_phase = BoardHostClosePhase::EdgeKinds;
                }
            }
            BoardHostClosePhase::EdgeKinds => {
                if let Some((key, value)) = self.style.edge_kinds.pop_first() {
                    self.closing.push_close_string(key);
                    self.closing.push_close_string(value.name);
                    self.closing.push_close_optional_string(value.source_tip);
                    self.closing.push_close_optional_string(value.target_tip);
                } else {
                    self.closing.close_phase = BoardHostClosePhase::EdgeTips;
                }
            }
            BoardHostClosePhase::EdgeTips => {
                if self.style.edge_tips.pop_first().is_none() {
                    self.closing.close_phase = BoardHostClosePhase::LinkRules;
                }
            }
            BoardHostClosePhase::LinkRules => {
                if let Some(rule) = self.style.link_compat_rules.pop() {
                    self.closing.push_close_string(rule.source);
                    self.closing.push_close_string(rule.target);
                } else {
                    self.closing.close_phase = BoardHostClosePhase::Previews;
                }
            }
            BoardHostClosePhase::Previews => {
                if self.focus.selection_screen_preview.as_mut().is_some_and(|values| values.pop().is_some()) || self.gestures.link_screen_preview.as_mut().is_some_and(|values| values.pop().is_some()) {
                } else if let Some(preview) = self.brush.brush_preview.take() {
                    self.closing.push_close_string(preview.source_handle_id);
                    self.closing.push_close_string(preview.node_kind_id);
                    self.closing.push_close_optional_string(preview.icon_kind);
                    self.closing.close_node_handles = Some(preview.handles);
                } else if let Some(preview) = self.brush.drop_preview.take() {
                    self.closing.push_close_string(preview.node_kind_id);
                    self.closing.push_close_optional_string(preview.icon_kind);
                } else if self.gestures
                    .transform_drag
                    .as_mut()
                    .is_some_and(|drag| drag.start_positions.pop_first().is_some() || drag.start_handle_angles.pop_first().is_some())
                {
                } else if self.gestures.transform_drag.take().is_some() {
                } else {
                    self.focus.selection_screen_preview = None;
                    self.gestures.link_screen_preview = None;
                    self.closing.close_phase = BoardHostClosePhase::Strings;
                }
            }
            BoardHostClosePhase::Strings => {
                if let Some(value) = self.focus
                    .hovered_id
                    .take()
                    .or_else(|| self.outbox.link_compat_nodes_emit_key.take())
                    .or_else(|| self.outbox.link_target_ring_emit_key.take())
                    .or_else(|| self.brush.brush_slot_source_id.take())
                    .or_else(|| self.brush.brush_candidates_emit_key.take())
                    .or_else(|| self.brush.brush_preview_emit_key.take())
                {
                    drop(value);
                } else if let Some((left, right)) = self.focus.hovered_kind.take() {
                    self.closing.push_close_string(left);
                    self.closing.push_close_string(right);
                } else if let Some(naming) = self.style.placement_naming.take() {
                    self.closing.push_close_string(naming.brush_node_prefix);
                    self.closing.push_close_string(naming.brush_edge_prefix);
                } else if let Some((values, mode)) = self.outbox.last_select_emit_sig.as_mut() {
                    if let Some(value) = values.pop().or_else(|| mode.take()) {
                        drop(value);
                    } else {
                        self.outbox.last_select_emit_sig = None;
                    }
                } else if let Some((left, right, mode)) = self.outbox.last_preselect_emit_sig.as_mut() {
                    if let Some(value) = left.pop().or_else(|| right.pop()).or_else(|| mode.take()) {
                        drop(value);
                    } else {
                        self.outbox.last_preselect_emit_sig = None;
                    }
                } else {
                    drop(std::mem::take(&mut self.style.world_raster_tiling));
                    let method = std::mem::take(&mut self.focus.selection_options.method);
                    let mode = std::mem::take(&mut self.focus.selection_options.mode);
                    self.closing.push_close_string(method);
                    self.closing.push_close_string(mode);
                    self.closing.close_phase = BoardHostClosePhase::Weights;
                }
            }
            BoardHostClosePhase::Weights => {
                let released = self.brush.brush_node_kind_weights.keys().next().cloned().and_then(|key| self.brush.brush_node_kind_weights.remove_entry(&key)).is_some()
                    || self.brush.brush_handle_kind_weights.keys().next().cloned().and_then(|key| self.brush.brush_handle_kind_weights.remove_entry(&key)).is_some();
                if !released {
                    self.closing.close_phase = BoardHostClosePhase::Done;
                }
            }
            BoardHostClosePhase::Done => return self.nonopaque_terminal_is_empty(),
        }
        context.consume_fuel(1);
        false
    }

    pub fn nonopaque_terminal_is_empty(&self) -> bool {
        self.closing.close_phase == BoardHostClosePhase::Done
            && event_authority_terminal_is_empty(&self.outbox, &self.commit)
            && self.commit.pointer_authority_terminal_is_empty()
            && self.closing.close_entity_retirement.is_none()
            && self.closing.close_string_len == 0
            && self.closing.close_node_handles.is_none()
            && self.style.placement_naming.is_none()
            && self.scene.nodes.is_empty()
            && self.scene.handles.is_empty()
            && self.scene.edges.is_empty()
            && self.scene.wires.is_empty()
            && self.scene.regions.is_empty()
            && self.gestures.region_drag.is_none()
            && self.gestures.region_paint.is_none()
            && self.style.handle_kinds.is_empty()
            && self.style.wire_kinds.is_empty()
            && self.style.node_kinds.is_empty()
            && self.style.edge_kinds.is_empty()
            && self.style.edge_tips.is_empty()
            && self.style.link_compat_rules.is_empty()
            && self.focus.selection.is_empty()
            && self.focus.preselect.is_empty()
            && self.focus.preselect_removed.is_empty()
            && self.focus.selection_exit_highlight.is_empty()
            && self.focus.highlighted_ids.is_empty()
            && matches!(self.focus.interaction, Interaction::None)
            && self.gestures.transform_drag.is_none()
            && self.style.icon_paint_cache.terminal_is_empty()
            && self.scene.world_content_cache.borrow().is_none()
            && self.closing.opaque_scene_retirement.get().is_none()
    }
}

impl BoardHost {
    pub fn new() -> Self {
        Self::default()
    }

    /// 🧠️ Normal directed graph host: no handles, edges reference node ids.
    pub fn new_normal() -> Self {
        let mut host = Self::default();
        host.style.port_mode = GraphPortMode::Normal;
        host.focus.selection_options.select_handles = false;
        host
    }

    /// 🔗️ Applies or clears a host-driven link preview session (cross-surface mirror).
    pub fn set_external_link_preview_json(&mut self, json: &str) -> Result<(), NormalPortError> {
        let v: serde_json::Value = serde_json::from_str(json).map_err(NormalPortError::ExternalLinkPreviewJson)?;
        let source = v.get("source").and_then(|s| s.as_str()).unwrap_or("").trim().to_string();
        if source.is_empty() {
            if matches!(self.focus.interaction, Interaction::ExternalLinkPreview { .. }) {
                self.focus.interaction = Interaction::None;
                self.outbox.clear_link_gesture_events();
            }
            return Ok(());
        }
        let end_x = v.get("endX").and_then(|x| x.as_f64()).unwrap_or(0.0);
        let end_y = v.get("endY").and_then(|y| y.as_f64()).unwrap_or(0.0);
        let compatible_node_ids: Vec<String> = v.get("compatiblePartIds").and_then(|a| a.as_array()).map(|arr| arr.iter().filter_map(|x| x.as_str().map(str::to_string)).collect()).unwrap_or_default();
        let ring_node_id = v.get("ringPartId").and_then(|n| n.as_str()).map(str::to_string);
        let ring_handle_ids: Vec<String> = v.get("ringAnchorIds").and_then(|a| a.as_array()).map(|arr| arr.iter().filter_map(|x| x.as_str().map(str::to_string)).collect()).unwrap_or_default();
        self.focus.interaction = Interaction::ExternalLinkPreview { source_id: source, end_world: Point::new(end_x, end_y), compatible_node_ids, ring_node_id, ring_handle_ids };
        sync_link_gesture_events(&mut self.scene, &mut self.focus, &mut self.style, &mut self.outbox);
        Ok(())
    }
}
