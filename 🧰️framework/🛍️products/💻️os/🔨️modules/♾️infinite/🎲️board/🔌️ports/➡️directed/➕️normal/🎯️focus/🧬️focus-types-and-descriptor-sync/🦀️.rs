//! 🧬️ Focus types and descriptor sync of the board focus: the focus and outbox state structs, pointer intent, descriptor sync, scene clearing and snapshot loading. See [`super`].

use super::*;

/// 🧩️ BoardHost focus component: selection, preselect, hover and highlight state with the active utility.
pub struct BoardFocusState {
    pub selection: BTreeSet<String>,
    /// 👁️ Live rectangle/lasso preview ids (committed selection stays in `selection` until pointer-up).
    pub preselect: BTreeSet<String>,
    /// 💠️ During preselect: anchor selection \\ `preselect` (secondary chrome while dragging).
    pub preselect_removed: BTreeSet<String>,
    /// 💠️ After commit: ids dropped in the last `select` transition only.
    pub selection_exit_highlight: BTreeSet<String>,
    pub selection_options: SelectionOptions,
    pub hovered_id: Option<String>,
    /// 🖱️ Transitive same-kind hover `(domain, kind_id)` when hovering a kind row or derived from `hovered_id`.
    pub hovered_kind: Option<(String, String)>,
    /// 💠️ Externally driven highlight ids (e.g. cross-panel variable binding); below hover, above neutral.
    pub highlighted_ids: BTreeSet<String>,
    pub interaction: Interaction,
    /// 👉️ Screen-space preview polygon (CSS pixels) while area-selecting; cleared when idle.
    pub selection_screen_preview: Option<Vec<Point>>,
    /// ↔ True when area-select drag is crossing (right-to-left); drives dashed preview stroke.
    pub selection_preview_crossing: bool,
    /// 🖌️ Active viewport utility (`select` suppresses brush slot logic).
    pub active_utility: ActiveUtility,
}

/// 🧩️ BoardHost outbox component: the outgoing board event queue and its emit dedupe signatures.
pub struct BoardOutboxState {
    pub events: BoardEventQueue,
    pub event_overflow: Option<BoardOwnedEvent>,
    pub event_batch_overflow: Option<BoardEventBatchReservation>,
    pub event_schema_fault: bool,
    /// 📡️ Dedupes {@code linkCompatibleNodes} emissions while a link wire is active.
    pub link_compat_nodes_emit_key: Option<String>,
    /// 📡️ Dedupes {@code linkTargetRing} emissions while a link wire is active.
    pub link_target_ring_emit_key: Option<String>,
    /// 📡️ Dedupes `select` emissions when ids are unchanged but modifier merge mode changes mid‑gesture.
    pub last_select_emit_sig: Option<(Vec<String>, Option<String>)>,
    /// 📡️ Dedupes `preselect` emissions during area-select drag.
    pub last_preselect_emit_sig: Option<(Vec<String>, Vec<String>, Option<String>)>,
    /// 🪪️ Serial of the last pointer gesture opened ([`GestureStage::id`] is `gesture-<serial>`).
    pub gesture_serial: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardPointerPhase {
    Move,
    Up,
    Leave,
}

#[derive(Clone, Copy, Debug)]
pub struct BoardPointerIntent {
    pub phase: BoardPointerPhase,
    pub x: f64,
    pub y: f64,
    pub shift: bool,
    pub ctrl_or_meta: bool,
    pub alt: bool,
}

pub(crate) fn board_edge_event_owned_bytes(id: &str, source: &str, target: &str) -> Option<usize> {
    let bytes = 27usize.checked_add(board_json_string_bytes(id))?.checked_add(board_json_string_bytes(source))?.checked_add(board_json_string_bytes(target))?;
    (bytes <= BOARD_EVENT_PAYLOAD_BYTE_CAPACITY).then_some(bytes)
}

impl BoardFocusState {
    /// 🔇️ Updates hover chrome without emitting `hover` (controlled React sync).
    pub fn set_hovered_id_silent(&mut self, id: Option<String>) {
        if self.hovered_id == id && self.hovered_kind.is_none() {
            return;
        }
        self.hovered_id = id;
        self.hovered_kind = None;
    }

    /// 💠️ Externally driven highlight set (cross-panel binding); does not emit events.
    pub fn set_highlighted_ids(&mut self, ids: Vec<String>) {
        let next: BTreeSet<String> = ids.into_iter().collect();
        if self.highlighted_ids == next {
            return;
        }
        self.highlighted_ids = next;
    }

    /// 💠️ Current externally driven highlight ids as JSON array.
    pub fn highlighted_ids_json(&self) -> Result<String, NormalPortError> {
        Ok(serde_json::to_string(&self.highlighted_ids.iter().cloned().collect::<Vec<_>>())?)
    }

    /// 🔇️ Mirrors controlled kind hover without emitting `hover`.
    pub fn set_hovered_kind_silent(&mut self, domain: Option<String>, kind_id: Option<String>) {
        let next_kind = domain.zip(kind_id);
        if self.hovered_id.is_none() && self.hovered_kind == next_kind {
            return;
        }
        self.hovered_id = None;
        self.hovered_kind = next_kind;
    }
}

/// 🚚️ One descriptor sync; `announce_new_edges` decides whether edges absent from the current
/// scene emit `edgeCreate`. A snapshot parse passes `false`: it clears the scene first, so every
/// edge is "new" — announcing them echoed the WHOLE document back to the plugin as edge
/// creations after each re-parse (fill placements re-committing themselves) and, undrained,
/// they exhausted the event credits so the next parse of any edged document was refused
/// (three blank panes after one drag, 2026-09-17).
pub fn sync_descriptor_with(scene: &mut BoardSceneState, focus: &mut BoardFocusState, style_state: &mut BoardStyleState, outbox: &mut BoardOutboxState, desc: &SceneDescriptor, announce_new_edges: bool) -> Result<(), NormalPortError> {
    descriptor_admission(desc)?;
    if matches!(focus.interaction, Interaction::LinkAtSourceHandle { .. } | Interaction::LinkDragSnap { .. } | Interaction::LinkTargetNode { .. } | Interaction::ExternalLinkPreview { .. }) {
        focus.interaction = Interaction::None;
        outbox.clear_link_gesture_events();
    }
    if announce_new_edges {
        let mut created_edge_count = 0usize;
        let mut created_edge_bytes = 0usize;
        for edge in desc.edges.iter().filter(|edge| !scene.edges.contains_key(&edge.id)) {
            created_edge_count = created_edge_count.checked_add(1).ok_or(NormalPortError::EventCredits)?;
            let bytes = board_edge_event_owned_bytes(&edge.id, &edge.source, &edge.target).ok_or(NormalPortError::EventCredits)?;
            created_edge_bytes = created_edge_bytes.checked_add(bytes).ok_or(NormalPortError::EventCredits)?;
        }
        outbox.events.reserve(created_edge_count, created_edge_bytes).map_err(|_| NormalPortError::EventCredits)?;
    }
    let want_nodes: BTreeSet<_> = desc.nodes.iter().map(|n| n.id.clone()).collect();
    let want_handles: BTreeSet<_> = desc.handles.iter().map(|h| h.id.clone()).collect();
    let want_edges: BTreeSet<_> = desc.edges.iter().map(|e| e.id.clone()).collect();
    let want_wires: BTreeSet<_> = desc.wires.iter().map(|w| w.id.clone()).collect();
    let want_regions: BTreeSet<_> = desc.regions.iter().map(|r| r.id.clone()).collect();
    scene.regions.retain(|id, _| want_regions.contains(id));
    scene.edges.retain(|id, _| want_edges.contains(id));
    scene.wires.retain(|id, _| want_wires.contains(id));
    scene.handles.retain(|id, _| want_handles.contains(id));
    scene.nodes.retain(|id, _| want_nodes.contains(id));
    for n in &desc.nodes {
        let shape = if n.shape.as_deref() == Some("rectangle") { NodeShape::Rectangle } else { NodeShape::Circle };
        let (radius, width, height) = match shape {
            NodeShape::Circle => (n.radius.unwrap_or(0.0), 0.0, 0.0),
            NodeShape::Rectangle => (0.0, n.width.unwrap_or(0.0), n.height.unwrap_or(0.0)),
        };
        let node_kind = n.node_kind.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).unwrap_or_default();
        let properties = n.user_data.as_ref().map(|v| property_bag_from_value(v)).unwrap_or_default();
        scene.nodes.insert(
            n.id.clone(),
            NodeData {
                id: n.id.clone(),
                x: n.x,
                y: n.y,
                shape,
                radius,
                width,
                height,
                scale: n.scale.filter(|v| v.is_finite() && *v > 0.0).unwrap_or(1.0),
                draggable: n.draggable.unwrap_or(true),
                selected: n.selected.unwrap_or(false),
                visible: n.visible.unwrap_or(true),
                locked: n.locked.unwrap_or(false),
                root: n.root.unwrap_or(false),
                style: n.style.clone(),
                text: n.text.clone(),
                icon_kind: n.icon_kind.clone(),
                node_kind,
                properties,
            },
        );
    }
    for h in &desc.handles {
        let kind = h.handle_kind.as_deref().unwrap_or("").trim().to_string();
        let color_fill = match h.color.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()) {
            None => None,
            Some(s) => Some(parse_css_color(s).ok_or_else(|| NormalPortError::InvalidHandleColor(h.id.clone(), s.to_string()))?),
        };
        let icon_kind = h.icon_kind.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());
        let properties = h.user_data.as_ref().map(|v| property_bag_from_value(v)).unwrap_or_default();
        scene.handles.insert(
            h.id.clone(),
            HandleData {
                id: h.id.clone(),
                node_id: h.node_id.clone(),
                angle: h.angle,
                radius: h.radius.unwrap_or(ui_styling::radii::HANDLE_DEFAULT),
                scale: h.scale.filter(|v| v.is_finite() && *v > 0.0).unwrap_or(1.0),
                selected: h.selected.unwrap_or(false),
                visible: h.visible.unwrap_or(true),
                locked: h.locked.unwrap_or(false),
                style: h.style.clone(),
                handle_kind: kind,
                color_fill,
                icon_kind,
                properties,
            },
        );
    }
    for e in &desc.edges {
        let existed = scene.edges.contains_key(&e.id);
        let edge_kind = e.edge_kind.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).unwrap_or_default();
        let source_tip = parse_catalog_tip_slot(e.source_tip.as_deref());
        let target_tip = parse_catalog_tip_slot(e.target_tip.as_deref());
        let properties = e.user_data.as_ref().map(|v| property_bag_from_value(v)).unwrap_or_default();
        scene.edges.insert(
            e.id.clone(),
            EdgeData {
                id: e.id.clone(),
                source: e.source.clone(),
                target: e.target.clone(),
                selected: e.selected.unwrap_or(false),
                visible: e.visible.unwrap_or(true),
                locked: e.locked.unwrap_or(false),
                style: e.style.clone(),
                edge_kind,
                source_tip,
                target_tip,
                properties,
            },
        );
        if announce_new_edges && !existed {
            let event = BoardOwnedEvent::edge(BoardEventKind::EdgeCreate, &e.id, &e.source, &e.target).expect("new descriptor edges were exactly preflighted");
            outbox.events.push(event).expect("new descriptor edge credits were reserved before mutation");
        }
    }
    for w in &desc.wires {
        let target = w.target.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
        let (end_x, end_y) = match &target {
            Some(_) => (None, None),
            None => {
                let x = match w.end_x {
                    Some(v) if v.is_finite() => Some(v),
                    _ => None,
                };
                let y = match w.end_y {
                    Some(v) if v.is_finite() => Some(v),
                    _ => None,
                };
                if x.is_none() || y.is_none() {
                    continue;
                }
                (x, y)
            }
        };
        let wire_kind = w
            .wire_kind
            .as_ref()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .or_else(|| scene.handles.get(w.source.as_str()).map(|h| style_state.resolve_default_wire_kind_for_handle(h)))
            .unwrap_or_else(|| DEFAULT_WIRE_KIND_ID.to_string());
        let properties = w.user_data.as_ref().map(|v| property_bag_from_value(v)).unwrap_or_default();
        scene.wires.insert(
            w.id.clone(),
            WireData {
                id: w.id.clone(),
                source: w.source.clone(),
                target,
                end_x,
                end_y,
                selected: w.selected.unwrap_or(false),
                visible: w.visible.unwrap_or(true),
                locked: w.locked.unwrap_or(false),
                style: w.style.clone(),
                wire_kind,
                properties,
            },
        );
    }
    for r in &desc.regions {
        scene.regions.insert(
            r.id.clone(),
            RegionData {
                id: r.id.clone(),
                x: r.x,
                y: r.y,
                width: r.width,
                height: r.height,
                label: r.label.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()),
                hidden: r.hidden.unwrap_or(false),
                locked: r.locked.unwrap_or(false),
                selected: r.selected.unwrap_or(false),
            },
        );
    }
    if !focus.is_preselect_active() {
        let mut new_selection = BTreeSet::new();
        for r in &desc.regions {
            if r.selected == Some(true) {
                new_selection.insert(r.id.clone());
            }
        }
        for n in &desc.nodes {
            if n.selected == Some(true) {
                new_selection.insert(n.id.clone());
            }
        }
        for h in &desc.handles {
            if h.selected == Some(true) {
                new_selection.insert(h.id.clone());
            }
        }
        for e in &desc.edges {
            if e.selected == Some(true) {
                new_selection.insert(e.id.clone());
            }
        }
        for w in &desc.wires {
            if w.selected == Some(true) {
                new_selection.insert(w.id.clone());
            }
        }
        let prev_sel = focus.selection.clone();
        if prev_sel != new_selection {
            focus.selection_exit_highlight.clear();
        }
        focus.selection = new_selection;
        if prev_sel != focus.selection {
            push_select_event(focus, outbox);
        }
    }
    sync_selection_flags_to_objects(scene, focus);
    scene.bump_content_scene_generation();
    Ok(())
}

pub fn clear_scene(scene: &mut BoardSceneState, focus: &mut BoardFocusState) {
    scene.regions.clear();
    scene.edges.clear();
    scene.wires.clear();
    scene.handles.clear();
    scene.nodes.clear();
    focus.selection.clear();
    focus.preselect.clear();
    focus.preselect_removed.clear();
    focus.selection_exit_highlight.clear();
}

/// 🎲️ Replaces the whole board with owned snapshot facts, ALL OR NOTHING: the descriptor is built and admitted
/// first, and only a snapshot that will really paint clears the live scene. Clearing before the
/// refusal left the panes blank until the next parse — the refusal and "the board went empty"
/// were the same event (2026-09-17 battery, `16-inspection/inspector-fresh-on-open`).
pub fn load_board_snapshot(scene: &mut BoardSceneState, focus: &mut BoardFocusState, style: &mut BoardStyleState, outbox: &mut BoardOutboxState, f: BoardSnapshot) -> bool {
    let port_mode = match f.schema.as_str() {
        "board.normal.undirected.v1" => GraphPortMode::Normal,
        "board.ports.directed.v1" => GraphPortMode::Ported,
        _ => return false,
    };
    let has_ports = port_mode.has_ports();
    let camera = f.camera.as_ref().map(|camera| (camera.x, camera.y, camera.zoom));
    let Some(desc) = board_snapshot_scene_descriptor(f, has_ports) else {
        return false;
    };
    if descriptor_admission(&desc).is_err() {
        return false;
    }
    style.port_mode = port_mode;
    if !has_ports {
        focus.selection_options.select_handles = false;
    }
    if let Some((x, y, zoom)) = camera {
        board_set_camera(scene, outbox, x, y, zoom);
    }
    clear_scene(scene, focus);
    sync_descriptor_with(scene, focus, style, outbox, &desc, false).is_ok()
}
