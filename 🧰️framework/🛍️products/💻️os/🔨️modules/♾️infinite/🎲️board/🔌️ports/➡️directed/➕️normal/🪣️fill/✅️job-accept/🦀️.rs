//! ✅️ Job accept of the board fill: accepting a candidate with its node, edge, kind, handle, icon and virtual node publication. See [`super`].

use super::*;

// #region board_host
//#region 🪣️Fill
impl BoardFillJob {
    pub(super) fn accept_candidate(state: &mut BoardFillJobState) -> Result<(), &'static str> {
        if state.pending_placement.is_some() {
            return Err("placement-unclaimed");
        }
        let preview = state.current_preview.ok_or("missing-preview")?;
        let kind = state.snapshot.kinds.get(preview.kind_index).ok_or("missing-candidate-kind")?;
        let kind_handle_count = kind.handles.len;
        let virtual_handle_count = state.virtual_handles.len.checked_add(kind_handle_count).ok_or("virtual-handle-capacity")?;
        if state.virtual_nodes.len == BOARD_FILL_PLACEMENT_CAPACITY || virtual_handle_count > BOARD_FILL_SOURCE_CAPACITY {
            return Err("placement-capacity");
        }
        state.pending_placement = Some(BoardFillPlacement {
            node_id: BoardFillText::empty(),
            edge_id: BoardFillText::empty(),
            edge_kind: BoardFillText::empty(),
            node_kind: BoardFillText::empty(),
            source_handle_id: BoardFillText::empty(),
            target_handle_id: BoardFillText::empty(),
            target_handle_index: preview.target_handle_index,
            x: preview.x,
            y: preview.y,
            shape: if kind.shape == BoardFillShape::Rectangle { "rectangle" } else { "circle" },
            radius: kind.radius,
            width: kind.width,
            height: kind.height,
            icon_kind: None,
            handles: BoardFillFixedPages::try_new().map_err(|_| "placement-handle-descriptor-capacity")?,
        });
        state.accept_handle_cursor = 0;
        state.accept_handle_template = None;
        state.accept_handle_id = None;
        state.accept_handle_pending_virtual = None;
        state.stage = BoardFillStage::AcceptNodeId;
        Ok(())
    }

    pub(super) fn accept_node_id(state: &mut BoardFillJobState) -> Result<(), &'static str> {
        let placement = state.pending_placement.as_mut().ok_or("missing-placement")?;
        placement.node_id = BoardFillText::try_indexed(state.snapshot.naming.fill_node_prefix, state.next_serial, "", None).map_err(|_| "placement-id-capacity")?;
        state.stage = BoardFillStage::AcceptEdgeId;
        Ok(())
    }

    pub(super) fn accept_edge_id(state: &mut BoardFillJobState) -> Result<(), &'static str> {
        let placement = state.pending_placement.as_mut().ok_or("missing-placement")?;
        placement.edge_id = BoardFillText::try_indexed(state.snapshot.naming.fill_edge_prefix, state.next_serial, "", None).map_err(|_| "placement-id-capacity")?;
        state.stage = BoardFillStage::AcceptEdgeKind;
        Ok(())
    }

    pub(super) fn accept_edge_kind(state: &mut BoardFillJobState) -> Result<(), &'static str> {
        let source_index = state.current_target.ok_or("missing-target")?;
        let edge_kind = state.sources.get(source_index).ok_or("missing-target-source")?.edge_kind;
        state.pending_placement.as_mut().ok_or("missing-placement")?.edge_kind = edge_kind;
        state.stage = BoardFillStage::AcceptNodeKind;
        Ok(())
    }

    pub(super) fn accept_node_kind(state: &mut BoardFillJobState) -> Result<(), &'static str> {
        let preview = state.current_preview.ok_or("missing-preview")?;
        let kind = state.snapshot.kinds.get(preview.kind_index).ok_or("missing-candidate-kind")?;
        state.pending_placement.as_mut().ok_or("missing-placement")?.node_kind = kind.id;
        state.stage = BoardFillStage::AcceptSourceHandle;
        Ok(())
    }

    pub(super) fn accept_source_handle(state: &mut BoardFillJobState) -> Result<(), &'static str> {
        let preview = state.current_preview.ok_or("missing-preview")?;
        state.pending_placement.as_mut().ok_or("missing-placement")?.source_handle_id = preview.source_id;
        state.stage = BoardFillStage::AcceptTargetHandle;
        Ok(())
    }

    pub(super) fn accept_target_handle(state: &mut BoardFillJobState) -> Result<(), &'static str> {
        let preview = state.current_preview.ok_or("missing-preview")?;
        state.pending_placement.as_mut().ok_or("missing-placement")?.target_handle_id = BoardFillText::try_indexed(state.snapshot.naming.fill_node_prefix, state.next_serial, ":h", Some(preview.target_handle_index)).map_err(|_| "placement-id-capacity")?;
        state.stage = BoardFillStage::AcceptIcon;
        Ok(())
    }

    pub(super) fn accept_icon(state: &mut BoardFillJobState) -> Result<(), &'static str> {
        let preview = state.current_preview.ok_or("missing-preview")?;
        let kind = state.snapshot.kinds.get(preview.kind_index).ok_or("missing-candidate-kind")?;
        state.pending_placement.as_mut().ok_or("missing-placement")?.icon_kind = kind.icon;
        state.stage = BoardFillStage::AcceptVirtualNode;
        Ok(())
    }

    pub(super) fn accept_virtual_node(state: &mut BoardFillJobState) -> Result<(), &'static str> {
        if state.accept_pending_virtual_node.is_none() {
            let preview = state.current_preview.ok_or("missing-preview")?;
            let id = state.pending_placement.as_ref().ok_or("missing-placement")?.node_id;
            state.accept_pending_virtual_node = Some(BoardFillVirtualNode { id, bounds: preview.bounds });
            return Ok(());
        }
        let node = state.accept_pending_virtual_node.take().ok_or("missing-virtual-node")?;
        if let Err(node) = state.virtual_nodes.try_push_owned(node) {
            state.accept_pending_virtual_node = Some(node);
            return Err("placement-capacity");
        }
        state.stage = BoardFillStage::AcceptSourceConnection;
        Ok(())
    }

    pub(super) fn accept_source_connection(state: &mut BoardFillJobState) -> Result<(), &'static str> {
        let source_index = state.current_target.ok_or("missing-target")?;
        let owner = state.sources.get(source_index).ok_or("missing-target-source")?.owner;
        Self::mark_source_connected(state, owner)?;
        state.next_serial = state.next_serial.checked_add(1).ok_or("placement-sequence-exhausted")?;
        state.stage = BoardFillStage::AcceptHandles;
        Ok(())
    }

    pub(super) fn accept_handle(state: &mut BoardFillJobState) -> Result<(), &'static str> {
        let preview = state.current_preview.ok_or("missing-preview")?;
        let kind = state.snapshot.kinds.get(preview.kind_index).ok_or("missing-candidate-kind")?;
        let Some(template) = kind.handles.get(state.accept_handle_cursor).copied() else {
            state.accepted_count = state.accepted_count.checked_add(1).ok_or("accepted-count-exhausted")?;
            state.stage = BoardFillStage::PublishPlanPrefix;
            return Ok(());
        };
        state.accept_handle_template = Some(template);
        state.stage = BoardFillStage::AcceptHandleId;
        Ok(())
    }

    pub(super) fn accept_handle_id(state: &mut BoardFillJobState) -> Result<(), &'static str> {
        let preview = state.current_preview.ok_or("missing-preview")?;
        let template = state.accept_handle_template.ok_or("missing-handle-template")?;
        if state.accept_handle_cursor == preview.target_handle_index {
            let id = state.pending_placement.as_ref().ok_or("missing-placement")?.target_handle_id;
            state.accept_handle_pending_placement = Some(BoardFillPlacementHandle { id, template });
            state.stage = BoardFillStage::AcceptHandlePublish;
            return Ok(());
        }
        let serial = state.next_serial.checked_sub(1).ok_or("placement-sequence-underflow")?;
        state.accept_handle_id = Some(BoardFillText::try_indexed(state.snapshot.naming.fill_node_prefix, serial, ":h", Some(state.accept_handle_cursor)).map_err(|_| "placement-id-capacity")?);
        state.stage = BoardFillStage::AcceptHandleVirtual;
        Ok(())
    }

    pub(super) fn accept_handle_virtual(state: &mut BoardFillJobState) -> Result<(), &'static str> {
        let preview = state.current_preview.ok_or("missing-preview")?;
        let kind = state.snapshot.kinds.get(preview.kind_index).ok_or("missing-candidate-kind")?;
        let template = state.accept_handle_template.ok_or("missing-handle-template")?;
        let id = state.accept_handle_id.take().ok_or("missing-handle-id")?;
        let center = Point::new(preview.x, preview.y);
        let anchor = match kind.shape {
            BoardFillShape::Circle => handle_position_on_circle(center, kind.radius, template.angle),
            BoardFillShape::Rectangle => handle_position_on_rectangle(center, kind.width, kind.height, template.angle),
        };
        let normal = normalize_or_zero(anchor - center);
        let slot = anchor + normal * state.snapshot.suggestion_offset;
        state.accept_handle_pending_virtual =
            Some(BoardFillVirtualHandle { id, node_kind: kind.id, handle_kind: template.handle_kind, wire_kind: template.wire_kind, edge_kind: template.edge_kind, slot: [slot.x, slot.y], weight: template.weight, connected: false });
        state.accept_handle_pending_placement = Some(BoardFillPlacementHandle { id, template });
        state.stage = BoardFillStage::AcceptHandlePublish;
        Ok(())
    }

    pub(super) fn accept_handle_publish(state: &mut BoardFillJobState) -> Result<(), &'static str> {
        if let Some(handle) = state.accept_handle_pending_virtual.take() {
            if let Err(handle) = state.virtual_handles.try_push_owned(handle) {
                state.accept_handle_pending_virtual = Some(handle);
                return Err("virtual-handle-capacity");
            }
            return Ok(());
        }
        let handle = state.accept_handle_pending_placement.take().ok_or("missing-placement-handle")?;
        let placement = state.pending_placement.as_mut().ok_or("missing-placement")?;
        if let Err(handle) = placement.handles.try_push_owned(handle) {
            state.accept_handle_pending_placement = Some(handle);
            return Err("placement-handle-capacity");
        }
        state.accept_handle_template = None;
        state.accept_handle_cursor += 1;
        state.stage = BoardFillStage::AcceptHandles;
        Ok(())
    }

    pub(super) fn publish_prefix(&mut self) -> Result<BoardFillOffer, &'static str> {
        let state = self.state.as_mut().ok_or("missing-fill-state")?;
        if state.accepted_count >= state.max_count {
            state.stage = BoardFillStage::Complete;
            return Ok(BoardFillOffer::Yield);
        }
        state.stage = BoardFillStage::ResetSources;
        let state = self.state.take().ok_or("missing-fill-state")?;
        self.checkpoint = Some(BoardFillCheckpoint { operation: self.operation, naming: self.naming, state: Some(state) });
        Ok(BoardFillOffer::Checkpoint(self.checkpoint.as_ref().map_or(0, |checkpoint| u64::from(checkpoint.accepted_count()))))
    }
}
//#endregion 🧵️FillJob
// #endregion board_host
