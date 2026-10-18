//! 📬️ Owned event impl of the board model: the large owned event method block. See [`super`].

use super::*;

impl BoardOwnedEvent {
    pub fn from_payload(kind: BoardEventKind, payload: &str, key: Option<&str>) -> Result<Self, BoardEventFault> {
        if payload.len() > BOARD_EVENT_PAYLOAD_BYTE_CAPACITY {
            return Err(BoardEventFault::ByteCredits);
        }
        let key = key.unwrap_or_default();
        if key.len() > BOARD_EVENT_KEY_BYTE_CAPACITY {
            return Err(BoardEventFault::KeyCredits);
        }
        let mut owned = Self { kind, payload: Box::new([0; BOARD_EVENT_PAYLOAD_BYTE_CAPACITY]), payload_len: payload.len() as u16, key: [0; BOARD_EVENT_KEY_BYTE_CAPACITY], key_len: key.len() as u16 };
        owned.payload[..payload.len()].copy_from_slice(payload.as_bytes());
        owned.key[..key.len()].copy_from_slice(key.as_bytes());
        Ok(owned)
    }

    pub fn kind(&self) -> BoardEventKind {
        self.kind
    }

    pub fn payload_json(&self) -> &str {
        std::str::from_utf8(&self.payload[..usize::from(self.payload_len)]).expect("board event payload is UTF-8")
    }

    pub fn key(&self) -> Option<&str> {
        (self.key_len > 0).then(|| std::str::from_utf8(&self.key[..usize::from(self.key_len)]).expect("board event key is UTF-8"))
    }

    pub fn owned_bytes(&self) -> usize {
        usize::from(self.payload_len) + usize::from(self.key_len)
    }

    pub fn write_json(&self, output: &mut String) {
        output.push_str("{\"name\":\"");
        output.push_str(self.kind.name());
        output.push_str("\",\"payload\":");
        output.push_str(self.payload_json());
        output.push('}');
    }

    pub fn selection(kind: BoardEventKind, ids: &[String], anchor_ids: Option<&[String]>, removed_ids: Option<&[String]>, gesture: Option<&str>) -> Result<Self, BoardEventFault> {
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"ids\":")?;
        payload.string_array(ids.iter().map(String::as_str))?;
        if let Some(anchor_ids) = anchor_ids {
            payload.raw(",\"anchorIds\":")?;
            payload.string_array(anchor_ids.iter().map(String::as_str))?;
        }
        if let Some(removed_ids) = removed_ids {
            payload.raw(",\"removedIds\":")?;
            payload.string_array(removed_ids.iter().map(String::as_str))?;
        }
        if kind == BoardEventKind::Select {
            payload.raw(",\"exitHighlightIds\":[]")?;
        }
        if let Some(gesture) = gesture {
            payload.raw(",\"gestureMergeMode\":")?;
            payload.string(gesture)?;
        }
        payload.raw("}")?;
        payload.finish(kind, None)
    }

    pub fn camera(x: f64, y: f64, zoom: f64) -> Result<Self, BoardEventFault> {
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"x\":")?;
        payload.number(x)?;
        payload.raw(",\"y\":")?;
        payload.number(y)?;
        payload.raw(",\"zoom\":")?;
        payload.number(zoom)?;
        payload.raw("}")?;
        payload.finish(BoardEventKind::Camera, None)
    }

    pub fn id(kind: BoardEventKind, id: &str) -> Result<Self, BoardEventFault> {
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"id\":")?;
        payload.string(id)?;
        payload.raw("}")?;
        payload.finish(kind, None)
    }

    pub fn id_list<'a>(kind: BoardEventKind, field: &str, ids: impl IntoIterator<Item = &'a str>) -> Result<Self, BoardEventFault> {
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{")?;
        payload.string(field)?;
        payload.raw(":[")?;
        for (count, id) in ids.into_iter().enumerate() {
            if count == BOARD_POINTER_ITEM_CAPACITY {
                return Err(BoardEventFault::ItemCredits);
            }
            if count > 0 {
                payload.raw(",")?;
            }
            payload.string(id)?;
        }
        payload.raw("]}")?;
        payload.finish(kind, None)
    }

    pub fn preselect_sets(ids: &BTreeSet<String>, removed_ids: &BTreeSet<String>, gesture: Option<&str>) -> Result<Self, BoardEventFault> {
        if ids.len() > BOARD_POINTER_ITEM_CAPACITY || removed_ids.len() > BOARD_POINTER_ITEM_CAPACITY {
            return Err(BoardEventFault::ItemCredits);
        }
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"ids\":")?;
        payload.string_array(ids.iter().map(String::as_str))?;
        payload.raw(",\"removedIds\":")?;
        payload.string_array(removed_ids.iter().map(String::as_str))?;
        if let Some(gesture) = gesture {
            payload.raw(",\"gestureMergeMode\":")?;
            payload.string(gesture)?;
        }
        payload.raw("}")?;
        payload.finish(BoardEventKind::Preselect, None)
    }

    pub fn select_set(ids: &BTreeSet<String>, anchor_ids: Option<&BTreeSet<String>>, gesture: Option<&str>) -> Result<Self, BoardEventFault> {
        if ids.len() > BOARD_POINTER_ITEM_CAPACITY || anchor_ids.is_some_and(|anchor_ids| anchor_ids.len() > BOARD_POINTER_ITEM_CAPACITY) {
            return Err(BoardEventFault::ItemCredits);
        }
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"ids\":")?;
        payload.string_array(ids.iter().map(String::as_str))?;
        if let Some(anchor_ids) = anchor_ids {
            payload.raw(",\"anchorIds\":")?;
            payload.string_array(anchor_ids.iter().map(String::as_str))?;
        }
        payload.raw(",\"exitHighlightIds\":[]")?;
        if let Some(gesture) = gesture {
            payload.raw(",\"gestureMergeMode\":")?;
            payload.string(gesture)?;
        }
        payload.raw("}")?;
        payload.finish(BoardEventKind::Select, None)
    }

    pub fn edge(kind: BoardEventKind, id: &str, source: &str, target: &str) -> Result<Self, BoardEventFault> {
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"id\":")?;
        payload.string(id)?;
        payload.raw(",\"source\":")?;
        payload.string(source)?;
        payload.raw(",\"target\":")?;
        payload.string(target)?;
        payload.raw("}")?;
        payload.finish(kind, None)
    }

    pub fn node_move(id: &str, x: f64, y: f64) -> Result<Self, BoardEventFault> {
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"id\":")?;
        payload.string(id)?;
        payload.raw(",\"x\":")?;
        payload.number(x)?;
        payload.raw(",\"y\":")?;
        payload.number(y)?;
        payload.raw("}")?;
        payload.finish(BoardEventKind::NodeMove, Some(id))
    }

    /// 🔄️ The live rotate-ring frame as resolved positions — a TRANSIENT row hosts mirror into
    /// sibling panes and drop before dispatch, so a drag previews everywhere and still commits once.
    pub fn transform_preview<'a>(moves: impl IntoIterator<Item = (&'a str, f64, f64)>) -> Result<Self, BoardEventFault> {
        Self::moves_row(BoardEventKind::TransformPreview, moves)
    }

    /// 🕹️ The selection a gesture's press staged, published at its release. `gesture_id` tags it when the
    /// release also carries the gesture record, so both leave as one dispatch.
    pub(super) fn staged_select(ids: &[String], merge: Option<&str>, gesture_id: Option<&str>) -> Result<Self, BoardEventFault> {
        if ids.len() > BOARD_POINTER_ITEM_CAPACITY {
            return Err(BoardEventFault::ItemCredits);
        }
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"ids\":")?;
        payload.string_array(ids.iter().map(String::as_str))?;
        payload.raw(",\"exitHighlightIds\":[]")?;
        if let Some(merge) = merge {
            payload.raw(",\"gestureMergeMode\":")?;
            payload.string(merge)?;
        }
        if let Some(gesture_id) = gesture_id {
            payload.raw(",\"gestureId\":")?;
            payload.string(gesture_id)?;
        }
        payload.raw("}")?;
        payload.finish(BoardEventKind::Select, None)
    }

    /// 🎬️ ONE finished pointer gesture: its id, kind, the ids it acted on, its motion and the proximity
    /// handle pairs it recorded — never positions, so the record is a parametric input a guest can yield
    /// as one editable mutation.
    pub fn gesture<'a>(gesture_id: &str, targets: impl IntoIterator<Item = &'a str>, motion: BoardGestureMotion, proximity: &[(String, String)]) -> Result<Self, BoardEventFault> {
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"gestureId\":")?;
        payload.string(gesture_id)?;
        payload.raw(",\"kind\":")?;
        payload.string(match motion {
            BoardGestureMotion::Drag { .. } => "drag",
            BoardGestureMotion::Rotate { .. } => "rotate",
        })?;
        payload.raw(",\"targets\":[")?;
        let mut count = 0usize;
        for id in targets {
            if count == BOARD_POINTER_ITEM_CAPACITY {
                return Err(BoardEventFault::ItemCredits);
            }
            if count > 0 {
                payload.raw(",")?;
            }
            payload.string(id)?;
            count += 1;
        }
        if count == 0 {
            return Err(BoardEventFault::Schema);
        }
        payload.raw("]")?;
        match motion {
            BoardGestureMotion::Drag { dx, dy } => {
                payload.raw(",\"dx\":")?;
                payload.number(board_pointer_offset(dx))?;
                payload.raw(",\"dy\":")?;
                payload.number(board_pointer_offset(dy))?;
            }
            BoardGestureMotion::Rotate { pivot, angle } => {
                payload.raw(",\"pivotX\":")?;
                payload.number(pivot.x)?;
                payload.raw(",\"pivotY\":")?;
                payload.number(pivot.y)?;
                payload.raw(",\"angle\":")?;
                payload.number(angle)?;
            }
        }
        payload.raw(",\"proximity\":[")?;
        for (index, (source, target)) in proximity.iter().enumerate() {
            if index > 0 {
                payload.raw(",")?;
            }
            payload.raw("{\"source\":")?;
            payload.string(source)?;
            payload.raw(",\"target\":")?;
            payload.string(target)?;
            payload.raw("}")?;
        }
        payload.raw("]}")?;
        payload.finish(BoardEventKind::Gesture, None)
    }

    fn moves_row<'a>(kind: BoardEventKind, moves: impl IntoIterator<Item = (&'a str, f64, f64)>) -> Result<Self, BoardEventFault> {
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"moves\":[")?;
        let mut count = 0usize;
        for (id, x, y) in moves {
            if count == BOARD_POINTER_ITEM_CAPACITY {
                return Err(BoardEventFault::ItemCredits);
            }
            if count > 0 {
                payload.raw(",")?;
            }
            payload.raw("{\"id\":")?;
            payload.string(id)?;
            payload.raw(",\"x\":")?;
            payload.number(x)?;
            payload.raw(",\"y\":")?;
            payload.number(y)?;
            payload.raw("}")?;
            count += 1;
        }
        if count == 0 {
            return Err(BoardEventFault::Schema);
        }
        payload.raw("]}")?;
        payload.finish(kind, None)
    }

    /// 🎯️ One painted target region, as the normalized rectangle the brush released on. The id is
    /// minted by the document, never here — the engine paints geometry, the guest owns identity.
    pub fn region_create(bounds: [f64; 4]) -> Result<Self, BoardEventFault> {
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"x\":")?;
        payload.number(bounds[0])?;
        payload.raw(",\"y\":")?;
        payload.number(bounds[1])?;
        payload.raw(",\"width\":")?;
        payload.number(bounds[2] - bounds[0])?;
        payload.raw(",\"height\":")?;
        payload.number(bounds[3] - bounds[1])?;
        payload.raw("}")?;
        payload.finish(BoardEventKind::RegionCreate, None)
    }

    /// 📐️ A region grip drag committed on release: corner AND extent, because a west/north grip
    /// moves the minimum corner as well as the size.
    pub fn region_resize(id: &str, bounds: [f64; 4]) -> Result<Self, BoardEventFault> {
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"id\":")?;
        payload.string(id)?;
        payload.raw(",\"x\":")?;
        payload.number(bounds[0])?;
        payload.raw(",\"y\":")?;
        payload.number(bounds[1])?;
        payload.raw(",\"width\":")?;
        payload.number(bounds[2] - bounds[0])?;
        payload.raw(",\"height\":")?;
        payload.number(bounds[3] - bounds[1])?;
        payload.raw("}")?;
        payload.finish(BoardEventKind::RegionResize, Some(id))
    }

    pub fn link_compatible(source: &str, node_ids: &[String]) -> Result<Self, BoardEventFault> {
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"source\":")?;
        payload.string(source)?;
        payload.raw(",\"nodeIds\":")?;
        payload.string_array(node_ids.iter().map(String::as_str))?;
        payload.raw("}")?;
        payload.finish(BoardEventKind::LinkCompatibleNodes, None)
    }

    pub fn link_ring(source: &str, node_id: Option<&str>, handle_ids: &[String]) -> Result<Self, BoardEventFault> {
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"source\":")?;
        payload.string(source)?;
        payload.raw(",\"nodeId\":")?;
        if let Some(node_id) = node_id {
            payload.string(node_id)?;
        } else {
            payload.raw("null")?;
        }
        payload.raw(",\"handleIds\":")?;
        payload.string_array(handle_ids.iter().map(String::as_str))?;
        payload.raw("}")?;
        payload.finish(BoardEventKind::LinkTargetRing, None)
    }

    pub fn hover(id: Option<&str>, kind: Option<(&str, &str)>) -> Result<Self, BoardEventFault> {
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"id\":")?;
        if let Some(id) = id {
            payload.string(id)?;
        } else {
            payload.raw("null")?;
        }
        payload.raw(",\"kind\":")?;
        if let Some((domain, kind_id)) = kind {
            payload.raw("{\"domain\":")?;
            payload.string(domain)?;
            payload.raw(",\"kindId\":")?;
            payload.string(kind_id)?;
            payload.raw("}")?;
        } else {
            payload.raw("null")?;
        }
        payload.raw("}")?;
        payload.finish(BoardEventKind::Hover, None)
    }

    fn brush_handles(payload: &mut BoardPayloadBuilder, handles: &[NodeKindHandleTemplate]) -> Result<(), BoardEventFault> {
        if handles.len() > BOARD_POINTER_ITEM_CAPACITY {
            return Err(BoardEventFault::ItemCredits);
        }
        payload.raw("[")?;
        for (index, handle) in handles.iter().enumerate() {
            if index > 0 {
                payload.raw(",")?;
            }
            payload.raw("{\"angle\":")?;
            payload.number(handle.angle)?;
            payload.raw(",\"handleKind\":")?;
            payload.string(&handle.handle_kind)?;
            if let Some(radius) = handle.radius {
                payload.raw(",\"radius\":")?;
                payload.number(radius)?;
            }
            payload.raw("}")?;
        }
        payload.raw("]")
    }

    pub fn brush_preview(preview: Option<&BrushPreviewSnapshot>) -> Result<Self, BoardEventFault> {
        let mut payload = BoardPayloadBuilder::new();
        let Some(preview) = preview else {
            payload.raw("{\"node\":null,\"edge\":null}")?;
            return payload.finish(BoardEventKind::BrushPreview, None);
        };
        payload.raw("{\"node\":{\"nodeKind\":")?;
        payload.string(&preview.node_kind_id)?;
        payload.raw(",\"x\":")?;
        payload.number(preview.x)?;
        payload.raw(",\"y\":")?;
        payload.number(preview.y)?;
        payload.raw(",\"shape\":")?;
        payload.string(if preview.shape == NodeShape::Rectangle { "rectangle" } else { "circle" })?;
        if preview.shape == NodeShape::Rectangle {
            payload.raw(",\"width\":")?;
            payload.number(preview.width)?;
            payload.raw(",\"height\":")?;
            payload.number(preview.height)?;
        } else {
            payload.raw(",\"radius\":")?;
            payload.number(preview.radius)?;
        }
        if let Some(icon) = &preview.icon_kind {
            payload.raw(",\"iconKind\":")?;
            payload.string(icon)?;
        }
        payload.raw(",\"handles\":")?;
        Self::brush_handles(&mut payload, &preview.handles)?;
        payload.raw("},\"edge\":{\"sourceHandleId\":")?;
        payload.string(&preview.source_handle_id)?;
        payload.raw(",\"targetHandleIndex\":")?;
        payload.usize(preview.target_handle_index)?;
        payload.raw("}}")?;
        payload.finish(BoardEventKind::BrushPreview, None)
    }

    pub fn brush_candidates(source: &str, candidates: &BrushCandidatePage, index: usize, suggestions_active: bool) -> Result<Self, BoardEventFault> {
        if candidates.len() > BOARD_POINTER_ITEM_CAPACITY {
            return Err(BoardEventFault::ItemCredits);
        }
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"sourceHandleId\":")?;
        payload.string(source)?;
        payload.raw(",\"candidates\":[")?;
        for (candidate_index, candidate) in candidates.iter().enumerate() {
            if candidate_index > 0 {
                payload.raw(",")?;
            }
            payload.raw("{\"nodeKind\":")?;
            payload.string(candidate.node_kind_id)?;
            payload.raw(",\"targetHandleIndex\":")?;
            payload.usize(candidate.target_handle_index)?;
            payload.raw("}")?;
        }
        payload.raw("],\"index\":")?;
        payload.usize(index)?;
        payload.raw(",\"suggestionsActive\":")?;
        payload.boolean(suggestions_active)?;
        payload.raw("}")?;
        payload.finish(BoardEventKind::BrushCandidates, None)
    }

    pub fn brush_place(preview: &BrushPreviewSnapshot, node_id: &str, edge_id: &str) -> Result<Self, BoardEventFault> {
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"nodeId\":")?;
        payload.string(node_id)?;
        payload.raw(",\"edgeId\":")?;
        payload.string(edge_id)?;
        payload.raw(",\"nodeKind\":")?;
        payload.string(&preview.node_kind_id)?;
        payload.raw(",\"sourceHandleId\":")?;
        payload.string(&preview.source_handle_id)?;
        payload.raw(",\"targetHandleIndex\":")?;
        payload.usize(preview.target_handle_index)?;
        payload.raw(",\"x\":")?;
        payload.number(preview.x)?;
        payload.raw(",\"y\":")?;
        payload.number(preview.y)?;
        payload.raw(",\"shape\":")?;
        payload.string(if preview.shape == NodeShape::Rectangle { "rectangle" } else { "circle" })?;
        if preview.shape == NodeShape::Rectangle {
            payload.raw(",\"width\":")?;
            payload.number(preview.width)?;
            payload.raw(",\"height\":")?;
            payload.number(preview.height)?;
        } else {
            payload.raw(",\"radius\":")?;
            payload.number(preview.radius)?;
        }
        if let Some(icon) = &preview.icon_kind {
            payload.raw(",\"iconKind\":")?;
            payload.string(icon)?;
        }
        payload.raw(",\"handles\":")?;
        Self::brush_handles(&mut payload, &preview.handles)?;
        payload.raw("}")?;
        payload.finish(BoardEventKind::BrushPlace, None)
    }
}
