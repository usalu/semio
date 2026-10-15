//! 🪣️ Board fill snapshot capture, snapshot ingress and the cooperative fill job.

use super::*;

// #region board_host
impl BoardHost {
    //#region 🪣️Fill
    /// 📸️ Begins a fixed-page snapshot capture without traversing a host collection.
    pub fn begin_board_fill_snapshot(&self) -> BoardFillSnapshotCapture {
        BoardFillSnapshotCapture::new(self.suggestion_offset)
    }

    //#endregion 🪣️Fill

    //#region 🧵️FillJob
}

impl BoardFillSnapshotCapture {
    pub(super) fn new(suggestion_offset: f64) -> Self {
        Self {
            snapshot: Some(BoardFillSnapshot { nodes: BoardFillFixedPages::new(), handles: BoardFillFixedPages::new(), kinds: BoardFillFixedPages::new(), rules: BoardFillFixedPages::new(), suggestion_offset }),
            phase: BoardFillCapturePhase::Nodes,
            last_key: None,
            node: None,
            handle: None,
            kind: None,
            kind_handle_cursor: 0,
            rule: None,
            rule_cursor: 0,
            closing: false,
        }
    }

    fn next_map_entry<'a, T>(&self, values: &'a BTreeMap<String, T>) -> Option<(&'a String, &'a T)> {
        Self::next_map_entry_after(self.last_key, values)
    }

    fn next_map_entry_after<T>(last_key: Option<BoardFillText>, values: &BTreeMap<String, T>) -> Option<(&String, &T)> {
        use std::ops::Bound::{Excluded, Unbounded};
        match last_key {
            Some(last) => values.range::<str, _>((Excluded(last.as_str()), Unbounded)).next(),
            None => values.iter().next(),
        }
    }

    fn capture_node(&mut self, host: &BoardHost) -> Result<(), BoardFillCaptureFault> {
        if let Some(current) = self.node.as_mut() {
            if current.bounds.is_none() {
                let node = host.nodes.get(current.id.as_str()).ok_or(BoardFillCaptureFault::StaleNode)?;
                let bounds = host.node_world_bounds(node, 0.0);
                current.bounds = Some([bounds.min_x, bounds.min_y, bounds.max_x, bounds.max_y]);
                return Ok(());
            }
            let completed = self.node.take().ok_or(BoardFillCaptureFault::NodeCapacity)?;
            let Some(bounds) = completed.bounds else {
                self.node = Some(completed);
                return Err(BoardFillCaptureFault::StaleNode);
            };
            let id = completed.id;
            if let Err(node) = self.snapshot.as_mut().ok_or(BoardFillCaptureFault::NodeCapacity)?.nodes.try_push_owned(BoardFillNodeSnapshot { id, bounds }) {
                self.node = Some(BoardFillNodeCapture { id: node.id, bounds: Some(node.bounds) });
                return Err(BoardFillCaptureFault::NodeCapacity);
            }
            self.last_key = Some(id);
            return Ok(());
        }
        let Some((key, _)) = self.next_map_entry(&host.nodes) else {
            self.phase = BoardFillCapturePhase::Handles;
            self.last_key = None;
            return Ok(());
        };
        let id = BoardFillText::try_from_str(key)?;
        self.node = Some(BoardFillNodeCapture { id, bounds: None });
        Ok(())
    }

    fn capture_handle(&mut self, host: &BoardHost) -> Result<(), BoardFillCaptureFault> {
        if let Some(current) = self.handle.as_mut() {
            match current.stage {
                BoardFillHandleCaptureStage::NodeKind => {
                    let handle = host.handles.get(current.handle.id.as_str()).ok_or(BoardFillCaptureFault::StaleHandle)?;
                    let node = host.nodes.get(&handle.node_id).ok_or(BoardFillCaptureFault::StaleNode)?;
                    current.handle.node_kind = BoardFillText::try_from_str(&node.node_kind)?;
                    current.stage = BoardFillHandleCaptureStage::HandleKind;
                }
                BoardFillHandleCaptureStage::HandleKind => {
                    let handle = host.handles.get(current.handle.id.as_str()).ok_or(BoardFillCaptureFault::StaleHandle)?;
                    current.handle.handle_kind = BoardFillText::try_from_str(&handle.handle_kind)?;
                    current.stage = BoardFillHandleCaptureStage::Slot;
                }
                BoardFillHandleCaptureStage::Slot => {
                    let handle = host.handles.get(current.handle.id.as_str()).ok_or(BoardFillCaptureFault::StaleHandle)?;
                    let anchor = host.brush_handle_anchor_world(handle).ok_or(BoardFillCaptureFault::StaleHandle)?;
                    let slot = host.handle_slot_center_world(&handle.node_id, anchor, host.suggestion_offset).ok_or(BoardFillCaptureFault::StaleHandle)?;
                    current.handle.slot = [slot.x, slot.y];
                    current.stage = BoardFillHandleCaptureStage::WireKind;
                }
                BoardFillHandleCaptureStage::WireKind => {
                    let wire = host.board_fill_default_wire_kind(current.handle.handle_kind.as_str())?;
                    current.handle.wire_kind = BoardFillText::try_from_str(wire)?;
                    current.stage = BoardFillHandleCaptureStage::EdgeKind;
                }
                BoardFillHandleCaptureStage::EdgeKind => {
                    let edge = host.board_fill_default_edge_kind(current.handle.wire_kind.as_str())?;
                    current.handle.edge_kind = BoardFillText::try_from_str(edge)?;
                    current.stage = BoardFillHandleCaptureStage::Visible;
                }
                BoardFillHandleCaptureStage::Visible => {
                    let _ = host.handles.get(current.handle.id.as_str()).ok_or(BoardFillCaptureFault::StaleHandle)?;
                    current.handle.visible = host.handle_effectively_visible(current.handle.id.as_str());
                    current.stage = BoardFillHandleCaptureStage::Connected;
                    current.edge_key = None;
                }
                BoardFillHandleCaptureStage::Connected => {
                    if let Some((edge_key, edge)) = Self::next_map_entry_after(current.edge_key, &host.edges) {
                        current.edge_key = Some(BoardFillText::try_from_str(edge_key)?);
                        if edge.source == current.handle.id.as_str() || edge.target == current.handle.id.as_str() {
                            current.handle.connected = true;
                            current.stage = BoardFillHandleCaptureStage::Weight;
                        }
                    } else {
                        current.stage = BoardFillHandleCaptureStage::Weight;
                    }
                }
                BoardFillHandleCaptureStage::Weight => {
                    current.handle.weight = host.brush_handle_kind_weights.get(current.handle.handle_kind.as_str()).copied().filter(|value| value.is_finite() && *value > 0.0).unwrap_or(1.0);
                    current.stage = BoardFillHandleCaptureStage::Publish;
                }
                BoardFillHandleCaptureStage::Publish => {
                    let completed = self.handle.take().ok_or(BoardFillCaptureFault::HandleCapacity)?;
                    let id = completed.handle.id;
                    if let Err(handle) = self.snapshot.as_mut().ok_or(BoardFillCaptureFault::HandleCapacity)?.handles.try_push_owned(completed.handle) {
                        self.handle = Some(BoardFillHandleCapture { handle, edge_key: completed.edge_key, stage: BoardFillHandleCaptureStage::Publish });
                        return Err(BoardFillCaptureFault::HandleCapacity);
                    }
                    self.last_key = Some(id);
                }
            }
            return Ok(());
        }
        let Some((key, _)) = self.next_map_entry(&host.handles) else {
            self.phase = BoardFillCapturePhase::Kinds;
            self.last_key = None;
            return Ok(());
        };
        let id = BoardFillText::try_from_str(key)?;
        self.handle = Some(BoardFillHandleCapture {
            handle: BoardFillHandleSnapshot {
                id,
                node_kind: BoardFillText::empty(),
                handle_kind: BoardFillText::empty(),
                wire_kind: BoardFillText::empty(),
                edge_kind: BoardFillText::empty(),
                slot: [0.0; 2],
                visible: false,
                connected: false,
                weight: 0.0,
            },
            edge_key: None,
            stage: BoardFillHandleCaptureStage::NodeKind,
        });
        Ok(())
    }

    fn capture_kind(&mut self, host: &BoardHost) -> Result<(), BoardFillCaptureFault> {
        if let Some(current) = self.kind.as_mut() {
            let source = host.node_kinds.get(current.value.id.as_str()).ok_or(BoardFillCaptureFault::StaleKind)?;
            match current.stage {
                BoardFillKindCaptureStage::Icon => {
                    current.value.icon = source.icon.as_deref().map(BoardFillText::try_from_str).transpose()?;
                    current.stage = BoardFillKindCaptureStage::Shape;
                }
                BoardFillKindCaptureStage::Shape => {
                    current.value.shape = if source.shape == NodeShape::Rectangle { BoardFillShape::Rectangle } else { BoardFillShape::Circle };
                    current.stage = BoardFillKindCaptureStage::Radius;
                }
                BoardFillKindCaptureStage::Radius => {
                    current.value.radius = host.brush_node_size * 0.5 * source.scale;
                    current.stage = BoardFillKindCaptureStage::Width;
                }
                BoardFillKindCaptureStage::Width => {
                    current.value.width = if source.shape == NodeShape::Rectangle { host.brush_node_size * source.scale } else { current.value.radius * 2.0 };
                    current.stage = BoardFillKindCaptureStage::Height;
                }
                BoardFillKindCaptureStage::Height => {
                    current.value.height = if source.shape == NodeShape::Rectangle { host.brush_node_size * source.scale } else { current.value.radius * 2.0 };
                    current.stage = BoardFillKindCaptureStage::Weight;
                }
                BoardFillKindCaptureStage::Weight => {
                    current.value.weight = host.brush_node_kind_weights.get(current.value.id.as_str()).copied().filter(|value| value.is_finite() && *value > 0.0).unwrap_or(1.0);
                    current.stage = BoardFillKindCaptureStage::Templates;
                }
                BoardFillKindCaptureStage::Templates => {
                    if let Some(template) = current.template.as_mut() {
                        let source_template = source.handles.get(self.kind_handle_cursor).ok_or(BoardFillCaptureFault::StaleKind)?;
                        match template.stage {
                            BoardFillTemplateCaptureStage::HandleKind => {
                                template.value.handle_kind = BoardFillText::try_from_str(&source_template.handle_kind)?;
                                template.stage = BoardFillTemplateCaptureStage::WireKind;
                            }
                            BoardFillTemplateCaptureStage::WireKind => {
                                let wire = host.board_fill_default_wire_kind(template.value.handle_kind.as_str())?;
                                template.value.wire_kind = BoardFillText::try_from_str(wire)?;
                                template.stage = BoardFillTemplateCaptureStage::EdgeKind;
                            }
                            BoardFillTemplateCaptureStage::EdgeKind => {
                                let edge = host.board_fill_default_edge_kind(template.value.wire_kind.as_str())?;
                                template.value.edge_kind = BoardFillText::try_from_str(edge)?;
                                template.stage = BoardFillTemplateCaptureStage::Angle;
                            }
                            BoardFillTemplateCaptureStage::Angle => {
                                template.value.angle = source_template.angle;
                                template.stage = BoardFillTemplateCaptureStage::Radius;
                            }
                            BoardFillTemplateCaptureStage::Radius => {
                                template.value.radius = source_template.radius;
                                template.stage = BoardFillTemplateCaptureStage::Weight;
                            }
                            BoardFillTemplateCaptureStage::Weight => {
                                template.value.weight = host.brush_handle_kind_weights.get(template.value.handle_kind.as_str()).copied().filter(|value| value.is_finite() && *value > 0.0).unwrap_or(1.0);
                                template.stage = BoardFillTemplateCaptureStage::Publish;
                            }
                            BoardFillTemplateCaptureStage::Publish => {
                                let completed = current.template.take().ok_or(BoardFillCaptureFault::KindHandleCapacity)?;
                                if let Err(value) = current.value.handles.try_push_owned(completed.value) {
                                    current.template = Some(BoardFillTemplateCapture { value, stage: BoardFillTemplateCaptureStage::Publish });
                                    return Err(BoardFillCaptureFault::KindHandleCapacity);
                                }
                                self.kind_handle_cursor += 1;
                            }
                        }
                    } else if source.handles.get(self.kind_handle_cursor).is_some() {
                        current.template = Some(BoardFillTemplateCapture {
                            value: BoardFillTemplateSnapshot { handle_kind: BoardFillText::empty(), wire_kind: BoardFillText::empty(), edge_kind: BoardFillText::empty(), angle: 0.0, radius: None, weight: 0.0 },
                            stage: BoardFillTemplateCaptureStage::HandleKind,
                        });
                    } else {
                        current.stage = BoardFillKindCaptureStage::Publish;
                    }
                }
                BoardFillKindCaptureStage::Publish => {
                    let completed = self.kind.take().ok_or(BoardFillCaptureFault::KindCapacity)?;
                    let id = completed.value.id;
                    let template = completed.template;
                    if let Err(value) = self.snapshot.as_mut().ok_or(BoardFillCaptureFault::KindCapacity)?.kinds.try_push_owned(completed.value) {
                        self.kind = Some(BoardFillKindCapture { value, stage: BoardFillKindCaptureStage::Publish, template });
                        return Err(BoardFillCaptureFault::KindCapacity);
                    }
                    self.last_key = Some(id);
                    self.kind_handle_cursor = 0;
                }
            }
            return Ok(());
        }
        let Some((key, _)) = self.next_map_entry(&host.node_kinds) else {
            self.phase = BoardFillCapturePhase::Rules;
            self.last_key = None;
            return Ok(());
        };
        let id = BoardFillText::try_from_str(key)?;
        self.kind = Some(BoardFillKindCapture {
            value: BoardFillKindSnapshot {
                id,
                shape: BoardFillShape::Circle,
                radius: 0.0,
                width: 0.0,
                height: 0.0,
                icon: None,
                weight: 0.0,
                handles: BoardFillFixedPages::try_new().map_err(|_| BoardFillCaptureFault::KindHandleCapacity)?,
            },
            stage: BoardFillKindCaptureStage::Icon,
            template: None,
        });
        Ok(())
    }

    fn capture_rule(&mut self, host: &BoardHost) -> Result<(), BoardFillCaptureFault> {
        if let Some(current) = self.rule.as_mut() {
            let source = host.link_compat_rules.get(self.rule_cursor).ok_or(BoardFillCaptureFault::StaleRule)?;
            match current.stage {
                BoardFillRuleCaptureStage::Target => {
                    current.value.target = BoardFillText::try_from_str(&source.target)?;
                    current.stage = BoardFillRuleCaptureStage::Bidirectional;
                }
                BoardFillRuleCaptureStage::Bidirectional => {
                    current.value.bidirectional = source.bidirectional;
                    current.stage = BoardFillRuleCaptureStage::Specificity;
                }
                BoardFillRuleCaptureStage::Specificity => {
                    current.value.specificity = source.specificity;
                    current.stage = BoardFillRuleCaptureStage::Publish;
                }
                BoardFillRuleCaptureStage::Publish => {
                    let completed = self.rule.take().ok_or(BoardFillCaptureFault::RuleCapacity)?;
                    if let Err(value) = self.snapshot.as_mut().ok_or(BoardFillCaptureFault::RuleCapacity)?.rules.try_push_owned(completed.value) {
                        self.rule = Some(BoardFillRuleCapture { value, stage: BoardFillRuleCaptureStage::Publish });
                        return Err(BoardFillCaptureFault::RuleCapacity);
                    }
                    self.rule_cursor += 1;
                }
            }
            return Ok(());
        }
        let Some(source) = host.link_compat_rules.get(self.rule_cursor) else {
            self.phase = BoardFillCapturePhase::Complete;
            return Ok(());
        };
        self.rule = Some(BoardFillRuleCapture {
            value: BoardFillRuleSnapshot { source: BoardFillText::try_from_str(&source.source)?, target: BoardFillText::empty(), bidirectional: false, specificity: CompatSpecificity::General },
            stage: BoardFillRuleCaptureStage::Target,
        });
        Ok(())
    }

    pub fn step(&mut self, host: &BoardHost) -> BoardFillCaptureStep {
        if self.closing {
            return BoardFillCaptureStep::Fault(BoardFillCaptureFault::GenerationExhausted);
        }
        let result = match self.phase {
            BoardFillCapturePhase::Nodes => self.capture_node(host),
            BoardFillCapturePhase::Handles => self.capture_handle(host),
            BoardFillCapturePhase::Kinds => self.capture_kind(host),
            BoardFillCapturePhase::Rules => self.capture_rule(host),
            BoardFillCapturePhase::Complete => return BoardFillCaptureStep::Complete,
            BoardFillCapturePhase::Closing => return BoardFillCaptureStep::Fault(BoardFillCaptureFault::GenerationExhausted),
        };
        match result {
            Ok(()) if self.phase == BoardFillCapturePhase::Complete => BoardFillCaptureStep::Complete,
            Ok(()) => BoardFillCaptureStep::Pending,
            Err(fault) => BoardFillCaptureStep::Fault(fault),
        }
    }

    pub fn take_snapshot(&mut self) -> Option<BoardFillSnapshot> {
        if self.phase != BoardFillCapturePhase::Complete || self.node.is_some() || self.handle.is_some() || self.kind.is_some() || self.rule.is_some() {
            return None;
        }
        self.closing = true;
        self.phase = BoardFillCapturePhase::Closing;
        self.snapshot.take()
    }

    pub fn begin_close(&mut self) {
        self.closing = true;
        self.phase = BoardFillCapturePhase::Closing;
    }

    pub fn next_close_release_byte_demand(&self)->usize {
        if self.node.is_some()||self.handle.is_some()||self.rule.is_some(){return 0;}
        if let Some(kind)=self.kind.as_ref(){if kind.template.is_some(){return 0;}return kind.value.handles.next_release_bytes();}
        let Some(snapshot)=self.snapshot.as_ref()else{return 0};
        if let Some(kind)=snapshot.kinds.len.checked_sub(1).and_then(|index|snapshot.kinds.get(index)){if !kind.handles.terminal_is_empty(){return kind.handles.next_release_bytes();}}
        if !snapshot.kinds.is_empty()||!snapshot.rules.is_empty()||!snapshot.handles.is_empty()||!snapshot.nodes.is_empty(){return 0;}
        for bytes in [snapshot.kinds.next_release_bytes(),snapshot.rules.next_release_bytes(),snapshot.handles.next_release_bytes(),snapshot.nodes.next_release_bytes()]{if bytes!=0{return bytes;}}
        0
    }

    pub fn close_step(&mut self, grant: semio_framework_value::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        let maximum_items=grant.maximum_items;
        let maximum_bytes=grant.maximum_release_bytes;
        if grant.maximum_depth==0 {return semio_framework_job::InteractiveJobCloseStep::Pending{progress:Default::default()};}
        self.begin_close();
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() };
        }
        if self.node.take().is_some() || self.handle.take().is_some() || self.rule.take().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,..Default::default()} };
        }
        if let Some(kind) = self.kind.as_mut() {
            if kind.template.take().is_some() {
                return semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,..Default::default()} };
            }
            if kind.value.handles.pop().is_some() {
                return semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,..Default::default()} };
            }
            if let Some(bytes) = kind.value.handles.retained_page_bytes() {
                if bytes > maximum_bytes {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() };
                }
                let Some(released_bytes) = kind.value.handles.retire_empty_page() else {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() };
                };
                return semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,released_bytes,..Default::default()} };
            }
            if let Some(step)=kind.value.handles.retire_descriptor(grant){return step;}
            self.kind = None;
            return semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,..Default::default()} };
        }
        let Some(snapshot) = self.snapshot.as_mut() else { return semio_framework_job::InteractiveJobCloseStep::Complete {progress:Default::default()} };
        if let Some(kind) = snapshot.kinds.len.checked_sub(1).and_then(|index| snapshot.kinds.get_mut(index)) {
            if kind.handles.pop().is_some() {
                return semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,..Default::default()} };
            }
            if let Some(bytes) = kind.handles.retained_page_bytes() {
                if bytes > maximum_bytes {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() };
                }
                let Some(released_bytes) = kind.handles.retire_empty_page() else {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() };
                };
                return semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,released_bytes,..Default::default()} };
            }
        }
        if let Some(kind)=snapshot.kinds.len.checked_sub(1).and_then(|index|snapshot.kinds.get_mut(index)){if let Some(step)=kind.handles.retire_descriptor(grant){return step;}}
        if snapshot.kinds.pop().is_some() || snapshot.rules.pop().is_some() || snapshot.handles.pop().is_some() || snapshot.nodes.pop().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,..Default::default()} };
        }
        macro_rules! retire_snapshot_page {
            ($owners:expr) => {
                if let Some(bytes) = $owners.retained_page_bytes() {
                    if bytes > maximum_bytes {
                        return semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() };
                    }
                    let Some(released_bytes) = $owners.retire_empty_page() else {
                        return semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() };
                    };
                    return semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,released_bytes,..Default::default()} };
                }
                if let Some(step)=$owners.retire_descriptor(grant){return step;}
            };
        }
        retire_snapshot_page!(snapshot.kinds);
        retire_snapshot_page!(snapshot.rules);
        retire_snapshot_page!(snapshot.handles);
        retire_snapshot_page!(snapshot.nodes);
        self.snapshot = None;
        semio_framework_job::InteractiveJobCloseStep::Complete {progress:semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()}}
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing && self.snapshot.is_none() && self.node.is_none() && self.handle.is_none() && self.kind.is_none() && self.rule.is_none()
    }
}

impl Drop for BoardFillSnapshotCapture {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "Puzzle2d fill capture must reach exact terminal-empty before Drop");
    }
}

impl BoardFillSnapshotIngress {
    pub fn new(suggestion_offset: f64) -> Self {
        Self {
            snapshot: Some(BoardFillSnapshot { nodes: BoardFillFixedPages::new(), handles: BoardFillFixedPages::new(), kinds: BoardFillFixedPages::new(), rules: BoardFillFixedPages::new(), suggestion_offset }),
            node: None,
            handle: None,
            kind: None,
            template: None,
            rule: None,
            closing: false,
        }
    }

    pub fn begin_node(&mut self) -> Result<(), BoardFillCaptureFault> {
        if self.closing || self.node.is_some() {
            return Err(BoardFillCaptureFault::StaleNode);
        }
        self.node = Some(BoardFillNodeSnapshot { id: BoardFillText::empty(), bounds: [0.0; 4] });
        Ok(())
    }

    pub fn push_node_id_byte(&mut self, byte: u8) -> Result<(), BoardFillCaptureFault> {
        if self.closing {
            return Err(BoardFillCaptureFault::StaleNode);
        }
        self.node.as_mut().ok_or(BoardFillCaptureFault::StaleNode)?.id.try_push_byte(byte)
    }

    pub fn set_node_bound(&mut self, index: usize, value: f64) -> Result<(), BoardFillCaptureFault> {
        if self.closing || !value.is_finite() {
            return Err(BoardFillCaptureFault::StaleNode);
        }
        let bound = self.node.as_mut().and_then(|node| node.bounds.get_mut(index)).ok_or(BoardFillCaptureFault::StaleNode)?;
        *bound = value;
        Ok(())
    }

    pub fn publish_node(&mut self) -> Result<(), BoardFillCaptureFault> {
        if self.closing {
            return Err(BoardFillCaptureFault::StaleNode);
        }
        let node = self.node.take().ok_or(BoardFillCaptureFault::NodeCapacity)?;
        let Some(snapshot) = self.snapshot.as_mut() else {
            self.node = Some(node);
            return Err(BoardFillCaptureFault::NodeCapacity);
        };
        if let Err(node) = snapshot.nodes.try_push_owned(node) {
            self.node = Some(node);
            return Err(BoardFillCaptureFault::NodeCapacity);
        }
        Ok(())
    }

    pub fn begin_handle(&mut self) -> Result<(), BoardFillCaptureFault> {
        if self.closing || self.handle.is_some() {
            return Err(BoardFillCaptureFault::StaleHandle);
        }
        self.handle = Some(BoardFillHandleSnapshot {
            id: BoardFillText::empty(),
            node_kind: BoardFillText::empty(),
            handle_kind: BoardFillText::empty(),
            wire_kind: BoardFillText::empty(),
            edge_kind: BoardFillText::empty(),
            slot: [0.0; 2],
            visible: false,
            connected: false,
            weight: 0.0,
        });
        Ok(())
    }

    pub fn push_handle_text_byte(&mut self, field: BoardFillIngressHandleText, byte: u8) -> Result<(), BoardFillCaptureFault> {
        if self.closing {
            return Err(BoardFillCaptureFault::StaleHandle);
        }
        let handle = self.handle.as_mut().ok_or(BoardFillCaptureFault::StaleHandle)?;
        match field {
            BoardFillIngressHandleText::Id => handle.id.try_push_byte(byte),
            BoardFillIngressHandleText::NodeKind => handle.node_kind.try_push_byte(byte),
            BoardFillIngressHandleText::HandleKind => handle.handle_kind.try_push_byte(byte),
            BoardFillIngressHandleText::WireKind => handle.wire_kind.try_push_byte(byte),
            BoardFillIngressHandleText::EdgeKind => handle.edge_kind.try_push_byte(byte),
        }
    }

    pub fn set_handle_slot(&mut self, index: usize, value: f64) -> Result<(), BoardFillCaptureFault> {
        if self.closing || !value.is_finite() {
            return Err(BoardFillCaptureFault::StaleHandle);
        }
        let slot = self.handle.as_mut().and_then(|handle| handle.slot.get_mut(index)).ok_or(BoardFillCaptureFault::StaleHandle)?;
        *slot = value;
        Ok(())
    }

    pub fn set_handle_visible(&mut self, visible: bool) -> Result<(), BoardFillCaptureFault> {
        if self.closing {
            return Err(BoardFillCaptureFault::StaleHandle);
        }
        self.handle.as_mut().ok_or(BoardFillCaptureFault::StaleHandle).map(|handle| handle.visible = visible)
    }

    pub fn set_handle_connected(&mut self, connected: bool) -> Result<(), BoardFillCaptureFault> {
        if self.closing {
            return Err(BoardFillCaptureFault::StaleHandle);
        }
        self.handle.as_mut().ok_or(BoardFillCaptureFault::StaleHandle).map(|handle| handle.connected = connected)
    }

    pub fn set_handle_weight(&mut self, weight: f64) -> Result<(), BoardFillCaptureFault> {
        if self.closing || !weight.is_finite() || weight <= 0.0 {
            return Err(BoardFillCaptureFault::StaleHandle);
        }
        self.handle.as_mut().ok_or(BoardFillCaptureFault::StaleHandle).map(|handle| handle.weight = weight)
    }

    pub fn publish_handle(&mut self) -> Result<(), BoardFillCaptureFault> {
        if self.closing {
            return Err(BoardFillCaptureFault::StaleHandle);
        }
        let handle = self.handle.take().ok_or(BoardFillCaptureFault::HandleCapacity)?;
        let Some(snapshot) = self.snapshot.as_mut() else {
            self.handle = Some(handle);
            return Err(BoardFillCaptureFault::HandleCapacity);
        };
        if let Err(handle) = snapshot.handles.try_push_owned(handle) {
            self.handle = Some(handle);
            return Err(BoardFillCaptureFault::HandleCapacity);
        }
        Ok(())
    }

    pub fn begin_kind(&mut self) -> Result<(), BoardFillCaptureFault> {
        if self.closing || self.kind.is_some() {
            return Err(BoardFillCaptureFault::StaleKind);
        }
        self.kind = Some(BoardFillKindSnapshot {
            id: BoardFillText::empty(),
            shape: BoardFillShape::Circle,
            radius: 0.0,
            width: 0.0,
            height: 0.0,
            icon: None,
            weight: 0.0,
            handles: BoardFillFixedPages::try_new().map_err(|_| BoardFillCaptureFault::KindHandleCapacity)?,
        });
        Ok(())
    }

    pub fn begin_kind_icon(&mut self) -> Result<(), BoardFillCaptureFault> {
        let kind = self.kind.as_mut().ok_or(BoardFillCaptureFault::StaleKind)?;
        if self.closing || kind.icon.is_some() {
            return Err(BoardFillCaptureFault::StaleKind);
        }
        kind.icon = Some(BoardFillText::empty());
        Ok(())
    }

    pub fn push_kind_text_byte(&mut self, field: BoardFillIngressKindText, byte: u8) -> Result<(), BoardFillCaptureFault> {
        if self.closing {
            return Err(BoardFillCaptureFault::StaleKind);
        }
        let kind = self.kind.as_mut().ok_or(BoardFillCaptureFault::StaleKind)?;
        match field {
            BoardFillIngressKindText::Id => kind.id.try_push_byte(byte),
            BoardFillIngressKindText::Icon => kind.icon.as_mut().ok_or(BoardFillCaptureFault::StaleKind)?.try_push_byte(byte),
        }
    }

    pub fn set_kind_rectangle(&mut self, rectangle: bool) -> Result<(), BoardFillCaptureFault> {
        if self.closing {
            return Err(BoardFillCaptureFault::StaleKind);
        }
        self.kind.as_mut().ok_or(BoardFillCaptureFault::StaleKind).map(|kind| kind.shape = if rectangle { BoardFillShape::Rectangle } else { BoardFillShape::Circle })
    }

    pub fn set_kind_radius(&mut self, radius: f64) -> Result<(), BoardFillCaptureFault> {
        if self.closing || !radius.is_finite() {
            return Err(BoardFillCaptureFault::StaleKind);
        }
        self.kind.as_mut().ok_or(BoardFillCaptureFault::StaleKind).map(|kind| kind.radius = radius)
    }

    pub fn set_kind_width(&mut self, width: f64) -> Result<(), BoardFillCaptureFault> {
        if self.closing || !width.is_finite() {
            return Err(BoardFillCaptureFault::StaleKind);
        }
        self.kind.as_mut().ok_or(BoardFillCaptureFault::StaleKind).map(|kind| kind.width = width)
    }

    pub fn set_kind_height(&mut self, height: f64) -> Result<(), BoardFillCaptureFault> {
        if self.closing || !height.is_finite() {
            return Err(BoardFillCaptureFault::StaleKind);
        }
        self.kind.as_mut().ok_or(BoardFillCaptureFault::StaleKind).map(|kind| kind.height = height)
    }

    pub fn set_kind_weight(&mut self, weight: f64) -> Result<(), BoardFillCaptureFault> {
        if self.closing || !weight.is_finite() || weight <= 0.0 {
            return Err(BoardFillCaptureFault::StaleKind);
        }
        self.kind.as_mut().ok_or(BoardFillCaptureFault::StaleKind).map(|kind| kind.weight = weight)
    }

    pub fn begin_kind_handle(&mut self) -> Result<(), BoardFillCaptureFault> {
        if self.closing || self.kind.is_none() || self.template.is_some() {
            return Err(BoardFillCaptureFault::StaleKind);
        }
        self.template = Some(BoardFillTemplateSnapshot { handle_kind: BoardFillText::empty(), wire_kind: BoardFillText::empty(), edge_kind: BoardFillText::empty(), angle: 0.0, radius: None, weight: 0.0 });
        Ok(())
    }

    pub fn push_kind_handle_text_byte(&mut self, field: BoardFillIngressTemplateText, byte: u8) -> Result<(), BoardFillCaptureFault> {
        if self.closing {
            return Err(BoardFillCaptureFault::StaleKind);
        }
        let template = self.template.as_mut().ok_or(BoardFillCaptureFault::StaleKind)?;
        match field {
            BoardFillIngressTemplateText::HandleKind => template.handle_kind.try_push_byte(byte),
            BoardFillIngressTemplateText::WireKind => template.wire_kind.try_push_byte(byte),
            BoardFillIngressTemplateText::EdgeKind => template.edge_kind.try_push_byte(byte),
        }
    }

    pub fn set_kind_handle_angle(&mut self, angle: f64) -> Result<(), BoardFillCaptureFault> {
        if self.closing || !angle.is_finite() {
            return Err(BoardFillCaptureFault::StaleKind);
        }
        self.template.as_mut().ok_or(BoardFillCaptureFault::StaleKind).map(|template| template.angle = angle)
    }

    pub fn set_kind_handle_radius(&mut self, radius: Option<f64>) -> Result<(), BoardFillCaptureFault> {
        if self.closing || radius.is_some_and(|radius| !radius.is_finite()) {
            return Err(BoardFillCaptureFault::StaleKind);
        }
        self.template.as_mut().ok_or(BoardFillCaptureFault::StaleKind).map(|template| template.radius = radius)
    }

    pub fn set_kind_handle_weight(&mut self, weight: f64) -> Result<(), BoardFillCaptureFault> {
        if self.closing || !weight.is_finite() || weight <= 0.0 {
            return Err(BoardFillCaptureFault::StaleKind);
        }
        self.template.as_mut().ok_or(BoardFillCaptureFault::StaleKind).map(|template| template.weight = weight)
    }

    pub fn publish_kind_handle(&mut self) -> Result<(), BoardFillCaptureFault> {
        if self.closing {
            return Err(BoardFillCaptureFault::StaleKind);
        }
        let template = self.template.take().ok_or(BoardFillCaptureFault::KindHandleCapacity)?;
        let Some(kind) = self.kind.as_mut() else {
            self.template = Some(template);
            return Err(BoardFillCaptureFault::StaleKind);
        };
        if let Err(template) = kind.handles.try_push_owned(template) {
            self.template = Some(template);
            return Err(BoardFillCaptureFault::KindHandleCapacity);
        }
        Ok(())
    }

    pub fn publish_kind(&mut self) -> Result<(), BoardFillCaptureFault> {
        if self.closing || self.template.is_some() {
            return Err(BoardFillCaptureFault::StaleKind);
        }
        let kind = self.kind.take().ok_or(BoardFillCaptureFault::StaleKind)?;
        let Some(snapshot) = self.snapshot.as_mut() else {
            self.kind = Some(kind);
            return Err(BoardFillCaptureFault::KindCapacity);
        };
        if let Err(kind) = snapshot.kinds.try_push_owned(kind) {
            self.kind = Some(kind);
            return Err(BoardFillCaptureFault::KindCapacity);
        }
        Ok(())
    }

    pub fn begin_rule(&mut self) -> Result<(), BoardFillCaptureFault> {
        if self.closing || self.rule.is_some() {
            return Err(BoardFillCaptureFault::StaleRule);
        }
        self.rule = Some(BoardFillRuleSnapshot { source: BoardFillText::empty(), target: BoardFillText::empty(), bidirectional: false, specificity: CompatSpecificity::Handle });
        Ok(())
    }

    pub fn push_rule_text_byte(&mut self, field: BoardFillIngressRuleText, byte: u8) -> Result<(), BoardFillCaptureFault> {
        if self.closing {
            return Err(BoardFillCaptureFault::StaleRule);
        }
        let rule = self.rule.as_mut().ok_or(BoardFillCaptureFault::StaleRule)?;
        match field {
            BoardFillIngressRuleText::Source => rule.source.try_push_byte(byte),
            BoardFillIngressRuleText::Target => rule.target.try_push_byte(byte),
        }
    }

    pub fn set_rule_bidirectional(&mut self, bidirectional: bool) -> Result<(), BoardFillCaptureFault> {
        if self.closing {
            return Err(BoardFillCaptureFault::StaleRule);
        }
        self.rule.as_mut().ok_or(BoardFillCaptureFault::StaleRule).map(|rule| rule.bidirectional = bidirectional)
    }

    pub fn set_rule_specificity(&mut self, specificity: &str) -> Result<(), BoardFillCaptureFault> {
        if self.closing {
            return Err(BoardFillCaptureFault::StaleRule);
        }
        let specificity = match specificity {
            "general" => CompatSpecificity::General,
            "node" => CompatSpecificity::Node,
            "edge" => CompatSpecificity::Edge,
            "wire" => CompatSpecificity::Wire,
            "handle" | "vortex" => CompatSpecificity::Handle,
            _ => return Err(BoardFillCaptureFault::StaleRule),
        };
        self.rule.as_mut().ok_or(BoardFillCaptureFault::StaleRule).map(|rule| rule.specificity = specificity)
    }

    pub fn publish_rule(&mut self) -> Result<(), BoardFillCaptureFault> {
        if self.closing {
            return Err(BoardFillCaptureFault::StaleRule);
        }
        let rule = self.rule.take().ok_or(BoardFillCaptureFault::RuleCapacity)?;
        let Some(snapshot) = self.snapshot.as_mut() else {
            self.rule = Some(rule);
            return Err(BoardFillCaptureFault::RuleCapacity);
        };
        if let Err(rule) = snapshot.rules.try_push_owned(rule) {
            self.rule = Some(rule);
            return Err(BoardFillCaptureFault::RuleCapacity);
        }
        Ok(())
    }

    pub fn take_snapshot(&mut self) -> Option<BoardFillSnapshot> {
        if self.closing || self.node.is_some() || self.handle.is_some() || self.kind.is_some() || self.template.is_some() || self.rule.is_some() {
            return None;
        }
        self.closing = true;
        self.snapshot.take()
    }

    pub fn begin_close(&mut self) {
        self.closing = true;
    }

    pub fn next_close_release_byte_demand(&self)->usize {
        if self.node.is_some()||self.handle.is_some()||self.template.is_some()||self.rule.is_some(){return 0;}
        if let Some(kind)=self.kind.as_ref(){return kind.handles.next_release_bytes();}
        let Some(snapshot)=self.snapshot.as_ref()else{return 0};
        if let Some(kind)=snapshot.kinds.len.checked_sub(1).and_then(|index|snapshot.kinds.get(index)){if !kind.handles.terminal_is_empty(){return kind.handles.next_release_bytes();}}
        if !snapshot.kinds.is_empty()||!snapshot.rules.is_empty()||!snapshot.handles.is_empty()||!snapshot.nodes.is_empty(){return 0;}
        for bytes in [snapshot.kinds.next_release_bytes(),snapshot.rules.next_release_bytes(),snapshot.handles.next_release_bytes(),snapshot.nodes.next_release_bytes()]{if bytes!=0{return bytes;}}
        0
    }

    pub fn close_step(&mut self, grant: semio_framework_value::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        let maximum_items=grant.maximum_items;
        let maximum_bytes=grant.maximum_release_bytes;
        if grant.maximum_depth==0 {return semio_framework_job::InteractiveJobCloseStep::Pending{progress:Default::default()};}
        self.begin_close();
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() };
        }
        if self.node.take().is_some() || self.handle.take().is_some() || self.template.take().is_some() || self.rule.take().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,..Default::default()} };
        }
        if let Some(kind) = self.kind.as_mut() {
            if kind.handles.pop().is_some() {
                return semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,..Default::default()} };
            }
            if let Some(bytes) = kind.handles.retained_page_bytes() {
                if bytes > maximum_bytes {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() };
                }
                let Some(released_bytes) = kind.handles.retire_empty_page() else {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() };
                };
                return semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,released_bytes,..Default::default()} };
            }
            if let Some(step)=kind.handles.retire_descriptor(grant){return step;}
            self.kind = None;
            return semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,..Default::default()} };
        }
        let Some(snapshot) = self.snapshot.as_mut() else { return semio_framework_job::InteractiveJobCloseStep::Complete {progress:Default::default()} };
        if let Some(kind) = snapshot.kinds.len.checked_sub(1).and_then(|index| snapshot.kinds.get_mut(index)) {
            if kind.handles.pop().is_some() {
                return semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,..Default::default()} };
            }
            if let Some(bytes) = kind.handles.retained_page_bytes() {
                if bytes > maximum_bytes {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() };
                }
                let Some(released_bytes) = kind.handles.retire_empty_page() else {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() };
                };
                return semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,released_bytes,..Default::default()} };
            }
        }
        if let Some(kind)=snapshot.kinds.len.checked_sub(1).and_then(|index|snapshot.kinds.get_mut(index)){if let Some(step)=kind.handles.retire_descriptor(grant){return step;}}
        if snapshot.kinds.pop().is_some() || snapshot.rules.pop().is_some() || snapshot.handles.pop().is_some() || snapshot.nodes.pop().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,..Default::default()} };
        }
        macro_rules! retire_ingress_page {
            ($owners:expr) => {
                if let Some(bytes) = $owners.retained_page_bytes() {
                    if bytes > maximum_bytes {
                        return semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() };
                    }
                    let Some(released_bytes) = $owners.retire_empty_page() else {
                        return semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() };
                    };
                    return semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,released_bytes,..Default::default()} };
                }
                if let Some(step)=$owners.retire_descriptor(grant){return step;}
            };
        }
        retire_ingress_page!(snapshot.kinds);
        retire_ingress_page!(snapshot.rules);
        retire_ingress_page!(snapshot.handles);
        retire_ingress_page!(snapshot.nodes);
        self.snapshot = None;
        semio_framework_job::InteractiveJobCloseStep::Complete {progress:semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()}}
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing && self.snapshot.is_none() && self.node.is_none() && self.handle.is_none() && self.kind.is_none() && self.template.is_none() && self.rule.is_none()
    }
}

impl Drop for BoardFillSnapshotIngress {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "Puzzle2d artifact fill ingress must reach exact terminal-empty before Drop");
    }
}

impl BoardFillJob {
    pub fn with_operation(snapshot: BoardFillSnapshot, max_count: u32, operation: semio_framework_job::Operation) -> Self {
        Self {
            operation,
            state: Some(BoardFillJobState {
                snapshot,
                stage: if max_count == 0 { BoardFillStage::Complete } else { BoardFillStage::PrepareSources },
                max_count,
                rng_state: operation.seed,
                sources: BoardFillFixedPages::new(),
                source_scan_cursor: 0,
                source_capture: None,
                target_selection_cursor: 0,
                target_best: None,
                current_target: None,
                candidates: BoardFillFixedPages::new(),
                kind_cursor: 0,
                template_cursor: 0,
                compatibility_candidate: None,
                compatibility_cursor: 0,
                compatibility_matched: false,
                candidate_selection_cursor: 0,
                candidate_best: None,
                current_candidate: None,
                current_preview: None,
                host_collision_cursor: 0,
                virtual_collision_cursor: 0,
                virtual_nodes: BoardFillFixedPages::new(),
                virtual_handles: BoardFillFixedPages::new(),
                pending_placement: None,
                accept_pending_virtual_node: None,
                accept_handle_template: None,
                accept_handle_id: None,
                accept_handle_pending_virtual: None,
                accept_handle_pending_placement: None,
                accept_handle_cursor: 0,
                accepted_count: 0,
                next_serial: operation.operation.0,
                stalled: false,
                rejection: None,
                search_count: 0,
                preview_sequence: 0,
                candidate_event: None,
            }),
            checkpoint: None,
            preview: None,
            commit_encoder: None,
            commit_writer: None,
            lent: None,
            commit_output: None,
            offer: Cell::new(None),
            admitted: Cell::new(false),
            fault: None,
            closing: false,
        }
    }

    /// ⏯️ The job resumed from `checkpoint` under exactly `operation`. A checkpoint still carrying the placement it
    /// published is refused: that placement is the owner's hand-off ([`BoardFillCheckpoint::take_pending_placement`]),
    /// and a resumed job must never hold — let alone overwrite — a placement nobody claimed.
    #[expect(clippy::result_large_err, reason = "Refusal returns the exact event, reservation, placement or checkpoint without allocating outside the bounded operation.")]
    pub fn restore(mut checkpoint: BoardFillCheckpoint, operation: semio_framework_job::Operation) -> Result<Self, BoardFillCheckpoint> {
        if checkpoint.operation.operation != operation.operation || checkpoint.operation.base_revision != operation.base_revision || checkpoint.operation.generation != operation.generation || checkpoint.pending_placement().is_some() {
            return Err(checkpoint);
        }
        let Some(state) = checkpoint.state.take() else { return Err(checkpoint) };
        Ok(Self { operation, state: Some(state), checkpoint: None, preview: None, commit_encoder: None, commit_writer: None, lent: None, commit_output: None, offer: Cell::new(None), admitted: Cell::new(false), fault: None, closing: false })
    }

    pub fn operation(&self) -> semio_framework_job::Operation {
        self.operation
    }

    pub fn take_checkpoint(&mut self) -> Option<BoardFillCheckpoint> {
        self.checkpoint.take()
    }

    #[expect(clippy::result_large_err, reason = "Refusal returns the exact event, reservation, placement or checkpoint without allocating outside the bounded operation.")]
    pub fn adopt_checkpoint(&mut self, mut checkpoint: BoardFillCheckpoint) -> Result<(), BoardFillCheckpoint> {
        if self.state.is_some() || checkpoint.pending_placement().is_some() || checkpoint.operation.operation != self.operation.operation || checkpoint.operation.base_revision != self.operation.base_revision || checkpoint.operation.generation != self.operation.generation {
            return Err(checkpoint);
        }
        let Some(state) = checkpoint.state.take() else { return Err(checkpoint) };
        self.state = Some(state);
        Ok(())
    }

    pub fn take_preview(&mut self) -> Option<BoardFillPreview> {
        self.preview.take()
    }

    pub fn take_fault(&mut self) -> Option<&'static str> {
        self.fault.take()
    }

    pub fn take_candidate_event(&mut self) -> Option<BoardFillCandidateEvent> {
        self.state.as_mut()?.candidate_event.take()
    }

    fn kind_event(kind: &BoardFillKindSnapshot, kind_index: usize, verdict: BoardFillCandidateVerdict, [x, y]: [f64; 2]) -> BoardFillCandidateEvent {
        let bounds = match kind.shape {
            BoardFillShape::Rectangle => [x - kind.width / 2.0, y - kind.height / 2.0, x + kind.width / 2.0, y + kind.height / 2.0],
            BoardFillShape::Circle => [x - kind.radius, y - kind.radius, x + kind.radius, y + kind.radius],
        };
        BoardFillCandidateEvent { verdict, kind_index, position: [x, y], bounds }
    }

    fn preview_event(preview: &BoardFillCandidatePreview, verdict: BoardFillCandidateVerdict) -> BoardFillCandidateEvent {
        BoardFillCandidateEvent { verdict, kind_index: preview.kind_index, position: [preview.x, preview.y], bounds: preview.bounds }
    }

    pub fn stage(&self) -> BoardFillStage {
        match self.state.as_ref() {
            Some(state) => state.stage,
            None => match self.checkpoint.as_ref().and_then(|checkpoint| checkpoint.state.as_ref()) {
                Some(state) => state.stage,
                None => BoardFillStage::Complete,
            },
        }
    }

    fn stage_label(&self) -> &'static str {
        match self.stage() {
            BoardFillStage::ResetSources => "puzzle2d-fill-reset-sources",
            BoardFillStage::PrepareSources => "puzzle2d-fill-prepare-sources",
            BoardFillStage::SelectTarget => "puzzle2d-fill-select-target",
            BoardFillStage::ResetCandidates => "puzzle2d-fill-reset-candidates",
            BoardFillStage::PrepareCandidates => "puzzle2d-fill-prepare-candidates",
            BoardFillStage::ScanCompatibility => "puzzle2d-fill-scan-compatibility",
            BoardFillStage::SelectCandidate => "puzzle2d-fill-select-candidate",
            BoardFillStage::ConstructPreview => "puzzle2d-fill-construct-preview",
            BoardFillStage::ScanHostCollision => "puzzle2d-fill-scan-host-collision",
            BoardFillStage::ScanVirtualCollision => "puzzle2d-fill-scan-virtual-collision",
            BoardFillStage::AcceptCandidate => "puzzle2d-fill-accept-candidate",
            BoardFillStage::AcceptNodeId => "puzzle2d-fill-accept-node-id",
            BoardFillStage::AcceptEdgeId => "puzzle2d-fill-accept-edge-id",
            BoardFillStage::AcceptEdgeKind => "puzzle2d-fill-accept-edge-kind",
            BoardFillStage::AcceptNodeKind => "puzzle2d-fill-accept-node-kind",
            BoardFillStage::AcceptSourceHandle => "puzzle2d-fill-accept-source-handle",
            BoardFillStage::AcceptTargetHandle => "puzzle2d-fill-accept-target-handle",
            BoardFillStage::AcceptIcon => "puzzle2d-fill-accept-icon",
            BoardFillStage::AcceptVirtualNode => "puzzle2d-fill-accept-virtual-node",
            BoardFillStage::AcceptSourceConnection => "puzzle2d-fill-accept-source-connection",
            BoardFillStage::AcceptHandles => "puzzle2d-fill-accept-handles",
            BoardFillStage::AcceptHandleId => "puzzle2d-fill-accept-handle-id",
            BoardFillStage::AcceptHandleVirtual => "puzzle2d-fill-accept-handle-virtual",
            BoardFillStage::AcceptHandlePublish => "puzzle2d-fill-accept-handle-publish",
            BoardFillStage::PublishPlanPrefix => "puzzle2d-fill-publish-prefix",
            BoardFillStage::Complete => "puzzle2d-fill-complete",
        }
    }

    fn next_seed(state: &mut BoardFillJobState) -> u64 {
        state.rng_state = state.rng_state.wrapping_mul(6364136223846793005).wrapping_add(1);
        state.rng_state
    }

    fn weighted_rank(state: &mut BoardFillJobState, weight: f64) -> f64 {
        let unit = ((Self::next_seed(state) >> 11) as f64 + 1.0) / ((1_u64 << 53) as f64 + 1.0);
        -unit.ln() / weight.max(f64::EPSILON)
    }

    fn boxes_overlap(left: [f64; 4], right: [f64; 4]) -> bool {
        left[0] <= right[2] && left[2] >= right[0] && left[1] <= right[3] && left[3] >= right[1]
    }

    fn prepare_source(state: &mut BoardFillJobState) -> Result<(), &'static str> {
        let base_len = state.snapshot.handles.len;
        let total_len = base_len.checked_add(state.virtual_handles.len).ok_or("source-capacity")?;
        if let Some(current) = state.source_capture.as_mut() {
            match current.stage {
                BoardFillSourceCaptureStage::Id => {
                    current.value.id = match current.value.owner {
                        BoardFillSourceOwner::Host(index) => state.snapshot.handles.get(index).ok_or("missing-host-handle")?.id,
                        BoardFillSourceOwner::Virtual(index) => state.virtual_handles.get(index).ok_or("missing-virtual-handle")?.id,
                    };
                    current.stage = BoardFillSourceCaptureStage::NodeKind;
                }
                BoardFillSourceCaptureStage::NodeKind => {
                    current.value.node_kind = match current.value.owner {
                        BoardFillSourceOwner::Host(index) => state.snapshot.handles.get(index).ok_or("missing-host-handle")?.node_kind,
                        BoardFillSourceOwner::Virtual(index) => state.virtual_handles.get(index).ok_or("missing-virtual-handle")?.node_kind,
                    };
                    current.stage = BoardFillSourceCaptureStage::HandleKind;
                }
                BoardFillSourceCaptureStage::HandleKind => {
                    current.value.handle_kind = match current.value.owner {
                        BoardFillSourceOwner::Host(index) => state.snapshot.handles.get(index).ok_or("missing-host-handle")?.handle_kind,
                        BoardFillSourceOwner::Virtual(index) => state.virtual_handles.get(index).ok_or("missing-virtual-handle")?.handle_kind,
                    };
                    current.stage = BoardFillSourceCaptureStage::WireKind;
                }
                BoardFillSourceCaptureStage::WireKind => {
                    current.value.wire_kind = match current.value.owner {
                        BoardFillSourceOwner::Host(index) => state.snapshot.handles.get(index).ok_or("missing-host-handle")?.wire_kind,
                        BoardFillSourceOwner::Virtual(index) => state.virtual_handles.get(index).ok_or("missing-virtual-handle")?.wire_kind,
                    };
                    current.stage = BoardFillSourceCaptureStage::EdgeKind;
                }
                BoardFillSourceCaptureStage::EdgeKind => {
                    current.value.edge_kind = match current.value.owner {
                        BoardFillSourceOwner::Host(index) => state.snapshot.handles.get(index).ok_or("missing-host-handle")?.edge_kind,
                        BoardFillSourceOwner::Virtual(index) => state.virtual_handles.get(index).ok_or("missing-virtual-handle")?.edge_kind,
                    };
                    current.stage = BoardFillSourceCaptureStage::Slot;
                }
                BoardFillSourceCaptureStage::Slot => {
                    current.value.slot = match current.value.owner {
                        BoardFillSourceOwner::Host(index) => state.snapshot.handles.get(index).ok_or("missing-host-handle")?.slot,
                        BoardFillSourceOwner::Virtual(index) => state.virtual_handles.get(index).ok_or("missing-virtual-handle")?.slot,
                    };
                    current.stage = BoardFillSourceCaptureStage::Weight;
                }
                BoardFillSourceCaptureStage::Weight => {
                    current.value.weight = match current.value.owner {
                        BoardFillSourceOwner::Host(index) => state.snapshot.handles.get(index).ok_or("missing-host-handle")?.weight,
                        BoardFillSourceOwner::Virtual(index) => state.virtual_handles.get(index).ok_or("missing-virtual-handle")?.weight,
                    };
                    current.stage = BoardFillSourceCaptureStage::Publish;
                }
                BoardFillSourceCaptureStage::Publish => {
                    let completed = state.source_capture.take().ok_or("missing-source-capture")?;
                    if let Err(value) = state.sources.try_push_owned(completed.value) {
                        state.source_capture = Some(BoardFillSourceCapture { value, stage: BoardFillSourceCaptureStage::Publish });
                        return Err("source-capacity");
                    }
                    state.source_scan_cursor += 1;
                }
            }
        } else if state.source_scan_cursor < total_len {
            let owner = if state.source_scan_cursor < base_len {
                let handle = state.snapshot.handles.get(state.source_scan_cursor).ok_or("missing-host-handle")?;
                if handle.connected || !handle.visible {
                    state.source_scan_cursor += 1;
                    return Ok(());
                }
                BoardFillSourceOwner::Host(state.source_scan_cursor)
            } else {
                let index = state.source_scan_cursor - base_len;
                let handle = state.virtual_handles.get(index).ok_or("missing-virtual-handle")?;
                if handle.connected {
                    state.source_scan_cursor += 1;
                    return Ok(());
                }
                BoardFillSourceOwner::Virtual(index)
            };
            state.source_capture = Some(BoardFillSourceCapture {
                value: BoardFillSource {
                    id: BoardFillText::empty(),
                    node_kind: BoardFillText::empty(),
                    handle_kind: BoardFillText::empty(),
                    wire_kind: BoardFillText::empty(),
                    edge_kind: BoardFillText::empty(),
                    slot: [0.0; 2],
                    weight: 0.0,
                    owner,
                    rejected: false,
                },
                stage: BoardFillSourceCaptureStage::Id,
            });
        }
        if state.source_capture.is_none() && state.source_scan_cursor >= total_len {
            if state.sources.is_empty() {
                state.stalled = true;
                state.stage = BoardFillStage::Complete;
            } else {
                state.stage = BoardFillStage::SelectTarget;
                state.target_selection_cursor = 0;
                state.target_best = None;
            }
        }
        Ok(())
    }

    fn select_target(state: &mut BoardFillJobState) -> Result<(), &'static str> {
        if state.target_selection_cursor < state.sources.len {
            let index = state.target_selection_cursor;
            state.target_selection_cursor += 1;
            let source = *state.sources.get(index).ok_or("missing-source")?;
            if !source.rejected {
                let rank = Self::weighted_rank(state, source.weight);
                if state.target_best.is_none_or(|best| rank < best.0 || (rank == best.0 && index < best.1)) {
                    state.target_best = Some((rank, index));
                }
            }
            return Ok(());
        }
        let Some((_, index)) = state.target_best.take() else {
            state.stalled = true;
            state.stage = BoardFillStage::Complete;
            return Ok(());
        };
        state.current_target = Some(index);
        state.kind_cursor = 0;
        state.template_cursor = 0;
        state.compatibility_candidate = None;
        state.compatibility_cursor = 0;
        state.compatibility_matched = false;
        state.rejection = None;
        state.stage = if state.candidates.terminal_is_empty() { BoardFillStage::PrepareCandidates } else { BoardFillStage::ResetCandidates };
        Ok(())
    }

    fn compatibility_rule_matches(rule: &BoardFillRuleSnapshot, source: &BoardFillSource, target_node: BoardFillText, target: &BoardFillTemplateSnapshot) -> bool {
        let (left, right) = match rule.specificity {
            CompatSpecificity::General | CompatSpecificity::Handle => (source.handle_kind, target.handle_kind),
            CompatSpecificity::Node => (source.node_kind, target_node),
            CompatSpecificity::Edge => (source.edge_kind, target.edge_kind),
            CompatSpecificity::Wire => (source.wire_kind, target.handle_kind),
        };
        (rule.source == left && rule.target == right) || (rule.bidirectional && rule.source == right && rule.target == left)
    }

    fn prepare_candidate(state: &mut BoardFillJobState) -> Result<(), &'static str> {
        let Some(target_index) = state.current_target else {
            Self::reject_target(state, "missing-target")?;
            return Ok(());
        };
        let Some(kind) = state.snapshot.kinds.get(state.kind_cursor) else {
            if state.candidates.is_empty() {
                Self::reject_target(state, "no-compatible-candidate")?;
            } else {
                state.candidate_selection_cursor = 0;
                state.candidate_best = None;
                state.stage = BoardFillStage::SelectCandidate;
            }
            return Ok(());
        };
        if let Some(template) = kind.handles.get(state.template_cursor) {
            let target = *state.sources.get(target_index).ok_or("missing-target-source")?;
            if !BoardHost::handle_port_shapes_compatible(target.handle_kind.as_str(), template.handle_kind.as_str()) || !BoardHost::single_letter_port_families_compatible(target.handle_kind.as_str(), template.handle_kind.as_str()) {
                state.candidate_event = Some(Self::kind_event(kind, state.kind_cursor, BoardFillCandidateVerdict::PortIncompatible, target.slot));
                state.template_cursor += 1;
                return Ok(());
            }
            let candidate = BoardFillCandidate { kind_index: state.kind_cursor, target_handle_index: state.template_cursor, weight: kind.weight * template.weight, rejected: false };
            if state.snapshot.rules.is_empty() {
                if let Err(candidate) = state.candidates.try_push_owned(candidate) {
                    state.compatibility_candidate = Some(candidate);
                    return Err("candidate-capacity");
                }
                state.template_cursor += 1;
            } else {
                state.compatibility_candidate = Some(candidate);
                state.compatibility_cursor = 0;
                state.compatibility_matched = false;
                state.stage = BoardFillStage::ScanCompatibility;
            }
        } else {
            state.kind_cursor += 1;
            state.template_cursor = 0;
        }
        Ok(())
    }

    fn scan_compatibility(state: &mut BoardFillJobState) -> Result<(), &'static str> {
        let candidate = state.compatibility_candidate.ok_or("missing-compatibility-candidate")?;
        let target_index = state.current_target.ok_or("missing-target")?;
        let source = *state.sources.get(target_index).ok_or("missing-target-source")?;
        let kind = state.snapshot.kinds.get(candidate.kind_index).ok_or("missing-candidate-kind")?;
        let template = kind.handles.get(candidate.target_handle_index).ok_or("missing-candidate-template")?;
        if let Some(rule) = state.snapshot.rules.get(state.compatibility_cursor) {
            state.compatibility_matched |= Self::compatibility_rule_matches(rule, &source, kind.id, template);
            state.compatibility_cursor += 1;
            return Ok(());
        }
        if state.compatibility_matched {
            if let Err(candidate) = state.candidates.try_push_owned(candidate) {
                state.compatibility_candidate = Some(candidate);
                return Err("candidate-capacity");
            }
        } else {
            state.candidate_event = Some(Self::kind_event(kind, candidate.kind_index, BoardFillCandidateVerdict::RuleIncompatible, source.slot));
        }
        state.compatibility_candidate = None;
        state.template_cursor += 1;
        state.stage = BoardFillStage::PrepareCandidates;
        Ok(())
    }

    fn select_candidate(state: &mut BoardFillJobState) -> Result<(), &'static str> {
        if state.candidate_selection_cursor < state.candidates.len {
            let index = state.candidate_selection_cursor;
            state.candidate_selection_cursor += 1;
            let candidate = *state.candidates.get(index).ok_or("missing-candidate")?;
            if !candidate.rejected {
                let rank = Self::weighted_rank(state, candidate.weight);
                if state.candidate_best.is_none_or(|best| rank < best.0 || (rank == best.0 && index < best.1)) {
                    state.candidate_best = Some((rank, index));
                }
            }
            return Ok(());
        }
        let Some((_, index)) = state.candidate_best.take() else {
            Self::reject_target(state, "candidates-exhausted")?;
            return Ok(());
        };
        state.current_candidate = Some(index);
        state.stage = BoardFillStage::ConstructPreview;
        Ok(())
    }

    fn construct_preview(state: &mut BoardFillJobState) -> Result<(), &'static str> {
        let Some(target_index) = state.current_target else {
            Self::reject_target(state, "missing-target")?;
            return Ok(());
        };
        let Some(candidate_index) = state.current_candidate else {
            Self::reject_target(state, "missing-candidate")?;
            return Ok(());
        };
        let target = *state.sources.get(target_index).ok_or("missing-target-source")?;
        let candidate = *state.candidates.get(candidate_index).ok_or("missing-candidate")?;
        let kind = state.snapshot.kinds.get(candidate.kind_index).ok_or("missing-candidate-kind")?;
        let event = Self::kind_event(kind, candidate.kind_index, BoardFillCandidateVerdict::Testing, target.slot);
        let [x, y] = event.position;
        state.current_preview = Some(BoardFillCandidatePreview { source_id: target.id, kind_index: candidate.kind_index, target_handle_index: candidate.target_handle_index, x, y, bounds: event.bounds });
        state.candidate_event = Some(event);
        state.host_collision_cursor = 0;
        state.virtual_collision_cursor = 0;
        state.search_count = state.search_count.checked_add(1).ok_or("search-sequence-exhausted")?;
        state.stage = BoardFillStage::ScanHostCollision;
        Ok(())
    }

    fn scan_host_collision(state: &mut BoardFillJobState) -> Result<(), &'static str> {
        let Some(preview) = state.current_preview else {
            Self::reject_candidate(state, "missing-preview")?;
            return Ok(());
        };
        if let Some(node) = state.snapshot.nodes.get(state.host_collision_cursor) {
            state.host_collision_cursor += 1;
            if Self::boxes_overlap(preview.bounds, node.bounds) {
                state.candidate_event = Some(Self::preview_event(&preview, BoardFillCandidateVerdict::HostCollision));
                Self::reject_candidate(state, "host-collision")?;
            }
        } else {
            state.stage = BoardFillStage::ScanVirtualCollision;
        }
        Ok(())
    }

    fn scan_virtual_collision(state: &mut BoardFillJobState) -> Result<(), &'static str> {
        let Some(preview) = state.current_preview else {
            Self::reject_candidate(state, "missing-preview")?;
            return Ok(());
        };
        if let Some(node) = state.virtual_nodes.get(state.virtual_collision_cursor) {
            state.virtual_collision_cursor += 1;
            if Self::boxes_overlap(preview.bounds, node.bounds) {
                state.candidate_event = Some(Self::preview_event(&preview, BoardFillCandidateVerdict::VirtualCollision));
                Self::reject_candidate(state, "virtual-collision")?;
            }
        } else {
            state.candidate_event = Some(Self::preview_event(&preview, BoardFillCandidateVerdict::Fits));
            state.stage = BoardFillStage::AcceptCandidate;
        }
        Ok(())
    }

    fn mark_source_connected(state: &mut BoardFillJobState, owner: BoardFillSourceOwner) -> Result<(), &'static str> {
        match owner {
            BoardFillSourceOwner::Host(index) => state.snapshot.handles.get_mut(index).ok_or("missing-host-source").map(|handle| handle.connected = true),
            BoardFillSourceOwner::Virtual(index) => state.virtual_handles.get_mut(index).ok_or("missing-virtual-source").map(|handle| handle.connected = true),
        }
    }

    fn accept_candidate(state: &mut BoardFillJobState) -> Result<(), &'static str> {
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

    fn accept_node_id(state: &mut BoardFillJobState) -> Result<(), &'static str> {
        let placement = state.pending_placement.as_mut().ok_or("missing-placement")?;
        placement.node_id = BoardFillText::try_indexed("puzzle2d.fill.", state.next_serial, "", None).map_err(|_| "placement-id-capacity")?;
        state.stage = BoardFillStage::AcceptEdgeId;
        Ok(())
    }

    fn accept_edge_id(state: &mut BoardFillJobState) -> Result<(), &'static str> {
        let placement = state.pending_placement.as_mut().ok_or("missing-placement")?;
        placement.edge_id = BoardFillText::try_indexed("puzzle2d.fill.edge.", state.next_serial, "", None).map_err(|_| "placement-id-capacity")?;
        state.stage = BoardFillStage::AcceptEdgeKind;
        Ok(())
    }

    fn accept_edge_kind(state: &mut BoardFillJobState) -> Result<(), &'static str> {
        let source_index = state.current_target.ok_or("missing-target")?;
        let edge_kind = state.sources.get(source_index).ok_or("missing-target-source")?.edge_kind;
        state.pending_placement.as_mut().ok_or("missing-placement")?.edge_kind = edge_kind;
        state.stage = BoardFillStage::AcceptNodeKind;
        Ok(())
    }

    fn accept_node_kind(state: &mut BoardFillJobState) -> Result<(), &'static str> {
        let preview = state.current_preview.ok_or("missing-preview")?;
        let kind = state.snapshot.kinds.get(preview.kind_index).ok_or("missing-candidate-kind")?;
        state.pending_placement.as_mut().ok_or("missing-placement")?.node_kind = kind.id;
        state.stage = BoardFillStage::AcceptSourceHandle;
        Ok(())
    }

    fn accept_source_handle(state: &mut BoardFillJobState) -> Result<(), &'static str> {
        let preview = state.current_preview.ok_or("missing-preview")?;
        state.pending_placement.as_mut().ok_or("missing-placement")?.source_handle_id = preview.source_id;
        state.stage = BoardFillStage::AcceptTargetHandle;
        Ok(())
    }

    fn accept_target_handle(state: &mut BoardFillJobState) -> Result<(), &'static str> {
        let preview = state.current_preview.ok_or("missing-preview")?;
        state.pending_placement.as_mut().ok_or("missing-placement")?.target_handle_id = BoardFillText::try_indexed("puzzle2d.fill.", state.next_serial, ":h", Some(preview.target_handle_index)).map_err(|_| "placement-id-capacity")?;
        state.stage = BoardFillStage::AcceptIcon;
        Ok(())
    }

    fn accept_icon(state: &mut BoardFillJobState) -> Result<(), &'static str> {
        let preview = state.current_preview.ok_or("missing-preview")?;
        let kind = state.snapshot.kinds.get(preview.kind_index).ok_or("missing-candidate-kind")?;
        state.pending_placement.as_mut().ok_or("missing-placement")?.icon_kind = kind.icon;
        state.stage = BoardFillStage::AcceptVirtualNode;
        Ok(())
    }

    fn accept_virtual_node(state: &mut BoardFillJobState) -> Result<(), &'static str> {
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

    fn accept_source_connection(state: &mut BoardFillJobState) -> Result<(), &'static str> {
        let source_index = state.current_target.ok_or("missing-target")?;
        let owner = state.sources.get(source_index).ok_or("missing-target-source")?.owner;
        Self::mark_source_connected(state, owner)?;
        state.next_serial = state.next_serial.checked_add(1).ok_or("placement-sequence-exhausted")?;
        state.stage = BoardFillStage::AcceptHandles;
        Ok(())
    }

    fn accept_handle(state: &mut BoardFillJobState) -> Result<(), &'static str> {
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

    fn accept_handle_id(state: &mut BoardFillJobState) -> Result<(), &'static str> {
        let preview = state.current_preview.ok_or("missing-preview")?;
        let template = state.accept_handle_template.ok_or("missing-handle-template")?;
        if state.accept_handle_cursor == preview.target_handle_index {
            let id = state.pending_placement.as_ref().ok_or("missing-placement")?.target_handle_id;
            state.accept_handle_pending_placement = Some(BoardFillPlacementHandle { id, template });
            state.stage = BoardFillStage::AcceptHandlePublish;
            return Ok(());
        }
        let serial = state.next_serial.checked_sub(1).ok_or("placement-sequence-underflow")?;
        state.accept_handle_id = Some(BoardFillText::try_indexed("puzzle2d.fill.", serial, ":h", Some(state.accept_handle_cursor)).map_err(|_| "placement-id-capacity")?);
        state.stage = BoardFillStage::AcceptHandleVirtual;
        Ok(())
    }

    fn accept_handle_virtual(state: &mut BoardFillJobState) -> Result<(), &'static str> {
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

    fn accept_handle_publish(state: &mut BoardFillJobState) -> Result<(), &'static str> {
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

    fn publish_prefix(&mut self) -> Result<BoardFillOffer, &'static str> {
        let state = self.state.as_mut().ok_or("missing-fill-state")?;
        if state.accepted_count >= state.max_count {
            state.stage = BoardFillStage::Complete;
            return Ok(BoardFillOffer::Yield);
        }
        state.stage = BoardFillStage::ResetSources;
        let state = self.state.take().ok_or("missing-fill-state")?;
        self.checkpoint = Some(BoardFillCheckpoint { operation: self.operation, state: Some(state) });
        Ok(BoardFillOffer::Checkpoint(self.checkpoint.as_ref().map_or(0, |checkpoint| u64::from(checkpoint.accepted_count()))))
    }

    fn reject_candidate(state: &mut BoardFillJobState, reason: &'static str) -> Result<(), &'static str> {
        if let Some(index) = state.current_candidate.take() {
            if let Some(candidate) = state.candidates.get_mut(index) {
                candidate.rejected = true;
            }
        }
        state.rejection = Some(BoardFillText::try_from_str(reason).map_err(|_| "rejection-text-capacity")?);
        state.current_preview = None;
        state.host_collision_cursor = 0;
        state.virtual_collision_cursor = 0;
        state.candidate_selection_cursor = 0;
        state.candidate_best = None;
        state.stage = BoardFillStage::SelectCandidate;
        Ok(())
    }

    fn reject_target(state: &mut BoardFillJobState, reason: &'static str) -> Result<(), &'static str> {
        if let Some(index) = state.current_target.take() {
            if let Some(source) = state.sources.get_mut(index) {
                source.rejected = true;
            }
        }
        state.rejection = Some(BoardFillText::try_from_str(reason).map_err(|_| "rejection-text-capacity")?);
        state.current_candidate = None;
        state.current_preview = None;
        state.target_selection_cursor = 0;
        state.target_best = None;
        state.stage = BoardFillStage::ResetCandidates;
        Ok(())
    }

    fn preview(state: &BoardFillJobState, operation: semio_framework_job::Operation, sequence: u64) -> BoardFillPreview {
        let target_handle_id = state.current_target.and_then(|index| state.sources.get(index)).map(|source| source.id);
        let candidate_node_kind_id = state.current_candidate.and_then(|index| state.candidates.get(index)).and_then(|candidate| state.snapshot.kinds.get(candidate.kind_index)).map(|kind| kind.id);
        let tested_collision_id = match state.stage {
            BoardFillStage::ScanHostCollision => state.host_collision_cursor.checked_sub(1).and_then(|index| state.snapshot.nodes.get(index)).map(|node| node.id),
            BoardFillStage::ScanVirtualCollision => state.virtual_collision_cursor.checked_sub(1).and_then(|index| state.virtual_nodes.get(index)).map(|node| node.id),
            _ => None,
        };
        BoardFillPreview {
            sequence,
            generation: operation.generation.0,
            stage: state.stage,
            accepted_count: state.accepted_count,
            target_handle_id,
            candidate_node_kind_id,
            host_collision_cursor: state.host_collision_cursor,
            virtual_collision_cursor: state.virtual_collision_cursor,
            tested_collision_id,
            rejection: state.rejection,
            search_count: state.search_count,
        }
    }

    fn complete(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> BoardFillOffer {
        let Some(state) = self.state.as_ref() else { return self.fault_outcome("missing-fill-state") };
        if self.commit_encoder.is_none() {
            self.commit_encoder = Some(BoardFillCommitEncoder::new());
            context.consume_fuel(1);
            return BoardFillOffer::Yield;
        }
        let encoder_complete = self.commit_encoder.as_ref().is_some_and(|encoder| encoder.stage == BoardFillCommitEncodeStage::Complete);
        if !encoder_complete {
            if let Err(code) = self.commit_encoder.as_mut().ok_or("fill-commit-encoder").and_then(|encoder| encoder.step(state)) {
                return self.fault_outcome(code);
            }
            context.consume_fuel(1);
            return BoardFillOffer::Yield;
        }
        if self.commit_writer.is_none() {
            self.commit_writer = Some(semio_framework_job::RetainedJobPayloadWriter::new(semio_framework_job::JobPayloadStream::CommitOutput));
            context.consume_fuel(1);
            return BoardFillOffer::Yield;
        }
        let Some(encoder) = self.commit_encoder.as_ref() else { return self.fault_outcome("fill-commit-encoder") };
        let start = encoder.output_cursor;
        if start < BOARD_FILL_COMMIT_BYTES {
            let Some(writer) = self.commit_writer.as_mut() else { return self.fault_outcome("fill-commit-writer") };
            let mut page = match writer.admit_page(context) {
                Ok(page) => page,
                Err(_) => return BoardFillOffer::Yield,
            };
            let end = start.saturating_add(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES).min(BOARD_FILL_COMMIT_BYTES);
            if page.write(&encoder.bytes[start..end]).is_err() {
                drop(page);
                return self.fault_outcome("fill-commit-output");
            }
            page.commit();
            let Some(encoder) = self.commit_encoder.as_mut() else { return self.fault_outcome("fill-commit-encoder") };
            encoder.output_cursor = end;
            if end < BOARD_FILL_COMMIT_BYTES {
                context.consume_fuel(1);
                return BoardFillOffer::Yield;
            }
        }
        let Some(writer) = self.commit_writer.take() else { return self.fault_outcome("fill-commit-writer") };
        let output = match writer.finish() {
            Ok(output) => output,
            Err(writer) => {
                self.commit_writer = Some(writer);
                return BoardFillOffer::Yield;
            }
        };
        self.commit_encoder = None;
        self.commit_output = Some(output);
        BoardFillOffer::Complete
    }

    fn fault_outcome(&mut self, code: &'static str) -> BoardFillOffer {
        self.fault = Some(code);
        BoardFillOffer::Fault
    }
}

impl BoardFillJob {
    fn next_offer(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> BoardFillOffer {
        if context.is_cancelled() {
            return BoardFillOffer::Cancelled;
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            return self.fault_outcome("stale-puzzle2d-fill-operation");
        }
        if context.should_yield() {
            return BoardFillOffer::Yield;
        }
        context.set_stage(self.stage_label());
        let Some(stage) = self.state.as_ref().map(|state| state.stage) else {
            return self.fault_outcome("fill-state-not-adopted");
        };
        if stage == BoardFillStage::PublishPlanPrefix {
            let outcome = match self.publish_prefix() {
                Ok(outcome) => outcome,
                Err(code) => self.fault_outcome(code),
            };
            context.consume_fuel(1);
            return outcome;
        }
        if stage == BoardFillStage::Complete {
            return self.complete(context);
        }
        let state = match self.state.as_mut() {
            Some(state) => state,
            None => return self.fault_outcome("missing-fill-state"),
        };
        let result = match stage {
            BoardFillStage::ResetSources => {
                if state.sources.pop().is_none() && state.sources.retire_empty_page().is_none() {
                    state.source_scan_cursor = 0;
                    state.current_target = None;
                    state.current_candidate = None;
                    state.current_preview = None;
                    state.stage = BoardFillStage::PrepareSources;
                }
                Ok(())
            }
            BoardFillStage::PrepareSources => Self::prepare_source(state),
            BoardFillStage::SelectTarget => Self::select_target(state),
            BoardFillStage::ResetCandidates => {
                if state.candidates.pop().is_none() && state.candidates.retire_empty_page().is_none() {
                    state.current_candidate = None;
                    state.compatibility_candidate = None;
                    state.compatibility_cursor = 0;
                    state.compatibility_matched = false;
                    state.stage = if state.current_target.is_some() { BoardFillStage::PrepareCandidates } else { BoardFillStage::SelectTarget };
                }
                Ok(())
            }
            BoardFillStage::PrepareCandidates => Self::prepare_candidate(state),
            BoardFillStage::ScanCompatibility => Self::scan_compatibility(state),
            BoardFillStage::SelectCandidate => Self::select_candidate(state),
            BoardFillStage::ConstructPreview => Self::construct_preview(state),
            BoardFillStage::ScanHostCollision => Self::scan_host_collision(state),
            BoardFillStage::ScanVirtualCollision => Self::scan_virtual_collision(state),
            BoardFillStage::AcceptCandidate => Self::accept_candidate(state),
            BoardFillStage::AcceptNodeId => Self::accept_node_id(state),
            BoardFillStage::AcceptEdgeId => Self::accept_edge_id(state),
            BoardFillStage::AcceptEdgeKind => Self::accept_edge_kind(state),
            BoardFillStage::AcceptNodeKind => Self::accept_node_kind(state),
            BoardFillStage::AcceptSourceHandle => Self::accept_source_handle(state),
            BoardFillStage::AcceptTargetHandle => Self::accept_target_handle(state),
            BoardFillStage::AcceptIcon => Self::accept_icon(state),
            BoardFillStage::AcceptVirtualNode => Self::accept_virtual_node(state),
            BoardFillStage::AcceptSourceConnection => Self::accept_source_connection(state),
            BoardFillStage::AcceptHandles => Self::accept_handle(state),
            BoardFillStage::AcceptHandleId => Self::accept_handle_id(state),
            BoardFillStage::AcceptHandleVirtual => Self::accept_handle_virtual(state),
            BoardFillStage::AcceptHandlePublish => Self::accept_handle_publish(state),
            BoardFillStage::PublishPlanPrefix | BoardFillStage::Complete => Ok(()),
        };
        if let Err(code) = result {
            return self.fault_outcome(code);
        }
        context.consume_fuel(1);
        if context.is_cancelled() {
            return BoardFillOffer::Cancelled;
        }
        let preview_sequence = match context.next_preview_sequence() {
            Ok(sequence) => sequence,
            Err(_) => return BoardFillOffer::Fault,
        };
        let Some(next_preview) = preview_sequence.checked_add(1) else { return self.fault_outcome("preview-sequence-exhausted") };
        self.operation.preview_sequence = next_preview;
        let Some(state) = self.state.as_mut() else { return self.fault_outcome("missing-fill-state") };
        state.preview_sequence = next_preview;
        self.preview = Some(Self::preview(state, self.operation, preview_sequence));
        BoardFillOffer::Preview
    }

    fn present<'a>(&'a self, context: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
        use semio_framework_job::JobOutcomeBorrow as Borrow;
        let absent = || semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "fill outcome payload is absent");
        let admitted = match self.offer.get().ok_or_else(absent)? {
            BoardFillOffer::Yield => Borrow::admit_yield(context)?,
            BoardFillOffer::Cancelled => Borrow::admit_cancelled(context)?,
            BoardFillOffer::Preview => Borrow::admit_preview(context, self.lent.as_ref().ok_or_else(absent)?)?,
            BoardFillOffer::Checkpoint(progress) => Borrow::admit_checkpoint(context, self.lent.as_ref().ok_or_else(absent)?, progress)?,
            BoardFillOffer::Fault => Borrow::admit_fault(context, self.lent.as_ref().ok_or_else(absent)?)?,
            BoardFillOffer::Complete => Borrow::admit_complete(context, None, self.commit_output.as_ref())?,
        };
        self.admitted.set(admitted.is_some());
        Ok(admitted)
    }
}

impl BoardFillJob {
    fn close_granted(&mut self,grant:semio_framework_value::RetainedCloneGrant)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_value::ValueError>{
        let maximum_items=grant.maximum_items;
        let maximum_bytes=grant.maximum_release_bytes;
        if grant.maximum_depth==0 {return Ok(semio_framework_job::InteractiveJobCloseStep::Pending{progress:Default::default()});}
        self.closing = true;
        if maximum_items == 0 {
            return Ok(semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() });
        }
        if self.state.is_none() {
            if let Some(mut checkpoint) = self.checkpoint.take() {
                self.state = checkpoint.state.take();
                return Ok(semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,..Default::default()} });
            }
        }
        if self.commit_encoder.take().is_some() {
            return Ok(semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,..Default::default()} });
        }
        if let Some(writer) = self.commit_writer.as_mut() {
            writer.begin_close();
            let step = writer.close_step(grant);
            if writer.terminal_is_empty() {self.commit_writer = None;}
            return Ok(match step {
                Ok(step)=>semio_framework_job::InteractiveJobCloseStep::Pending{progress:step.progress()},
                Err(error)=>return Err(error),
            });
        }
        for slot in [&mut self.lent, &mut self.commit_output] {
            if let Some(payload) = slot.as_mut() {
                if payload.terminal_is_empty() {
                    *slot = None;
                    return Ok(semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,..Default::default()} });
                }
                return Ok(match payload.close_step(grant) {
                    Ok(step) => semio_framework_job::InteractiveJobCloseStep::Pending { progress: step.progress() },
                    Err(error)=>return Err(error),
                });
            }
        }
        if self.preview.take().is_some() || self.fault.take().is_some() {
            return Ok(semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,..Default::default()} });
        }
        let Some(state) = self.state.as_mut() else { return Ok(semio_framework_job::InteractiveJobCloseStep::Complete {progress:Default::default()}) };
        if let Some(placement) = state.pending_placement.as_mut() {
            if placement.handles.pop().is_some() {
                return Ok(semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,..Default::default()} });
            }
            if let Some(bytes) = placement.handles.retained_page_bytes() {
                if bytes > maximum_bytes {
                    return Ok(semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() });
                }
                let Some(released_bytes) = placement.handles.retire_empty_page() else {
                    return Ok(semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() });
                };
                return Ok(semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,released_bytes,..Default::default()} });
            }
            if let Some(step)=placement.handles.retire_descriptor(grant){return Ok(step);}
            state.pending_placement = None;
            return Ok(semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,..Default::default()} });
        }
        if state.accept_handle_template.take().is_some()
            || state.accept_handle_id.take().is_some()
            || state.accept_handle_pending_virtual.take().is_some()
            || state.accept_handle_pending_placement.take().is_some()
            || state.accept_pending_virtual_node.take().is_some()
            || state.source_capture.take().is_some()
            || state.compatibility_candidate.take().is_some()
            || state.current_preview.take().is_some()
            || state.candidate_event.take().is_some()
            || state.rejection.take().is_some()
        {
            return Ok(semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,..Default::default()} });
        }
        if let Some(kind) = state.snapshot.kinds.len.checked_sub(1).and_then(|index| state.snapshot.kinds.get_mut(index)) {
            if kind.handles.pop().is_some() {
                return Ok(semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,..Default::default()} });
            }
            if let Some(bytes) = kind.handles.retained_page_bytes() {
                if bytes > maximum_bytes {
                    return Ok(semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() });
                }
                let Some(released_bytes) = kind.handles.retire_empty_page() else {
                    return Ok(semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() });
                };
                return Ok(semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,released_bytes,..Default::default()} });
            }
        }
        if let Some(kind)=state.snapshot.kinds.len.checked_sub(1).and_then(|index|state.snapshot.kinds.get_mut(index)){if let Some(step)=kind.handles.retire_descriptor(grant){return Ok(step);}}
        if state.snapshot.kinds.pop().is_some()
            || state.virtual_handles.pop().is_some()
            || state.virtual_nodes.pop().is_some()
            || state.candidates.pop().is_some()
            || state.sources.pop().is_some()
            || state.snapshot.rules.pop().is_some()
            || state.snapshot.handles.pop().is_some()
            || state.snapshot.nodes.pop().is_some()
        {
            return Ok(semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,..Default::default()} });
        }
        macro_rules! retire_page {
            ($owners:expr) => {
                if let Some(bytes) = $owners.retained_page_bytes() {
                    if bytes > maximum_bytes {
                        return Ok(semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() });
                    }
                    let Some(released_bytes) = $owners.retire_empty_page() else {
                        return Ok(semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() });
                    };
                    return Ok(semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,released_bytes,..Default::default()} });
                }
                if let Some(step)=$owners.retire_descriptor(grant){return Ok(step);}
            };
        }
        retire_page!(state.virtual_handles);
        retire_page!(state.virtual_nodes);
        retire_page!(state.candidates);
        retire_page!(state.sources);
        retire_page!(state.snapshot.rules);
        retire_page!(state.snapshot.handles);
        retire_page!(state.snapshot.nodes);
        retire_page!(state.snapshot.kinds);
        self.state = None;
        Ok(semio_framework_job::InteractiveJobCloseStep::Complete {progress:semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()}})
    
    }
}

impl semio_framework_job::InteractiveJob for BoardFillJob {
    fn step<'a>(&'a mut self, context: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        if self.admitted.get() || self.offer.get().is_none() {
            self.admitted.set(false);
            let offer = self.next_offer(context);
            self.lent = match offer {
                BoardFillOffer::Preview => Some(semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Preview)),
                BoardFillOffer::Checkpoint(_) => Some(semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CheckpointState)),
                BoardFillOffer::Fault => Some(semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault)),
                BoardFillOffer::Yield | BoardFillOffer::Complete | BoardFillOffer::Cancelled => None,
            };
            self.offer.set(Some(offer));
        }
        self.present(context).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,context))
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        use semio_framework_job::JobOutcomeKind as Kind;
        let lent = || self.lent.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "fill outcome payload is absent"));
        match descriptor.kind() {
            Kind::Yield => descriptor.yielded(),
            Kind::Cancelled => descriptor.cancelled(),
            Kind::PreviewReady => descriptor.preview(lent()?),
            Kind::CheckpointReady { .. } => descriptor.checkpoint(lent()?),
            Kind::Fault => descriptor.fault(lent()?),
            Kind::Complete => descriptor.complete(None, self.commit_output.as_ref()),
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        if self.terminal_is_empty(){return Ok(semio_framework_job::InteractiveJobCloseStep::Complete{progress:Default::default()})}
        let demand=(||->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{Ok(semio_framework_value::RetirementDemand{copy_bytes:self.next_close_copy_byte_demand()?,capacity_bytes:self.next_close_capacity_byte_demand(cx.retained_grant().maximum_copy_bytes)?,release_bytes:self.next_close_release_byte_demand()?,depth:self.next_close_depth_demand()?})})().map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))?;
        let mut blocked=false;
        let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|self.close_granted(grant).map(|step|match step{semio_framework_job::InteractiveJobCloseStep::Pending{progress}=>semio_framework_value::RetainedCloneStep::Progress(progress),semio_framework_job::InteractiveJobCloseStep::Complete{progress}=>semio_framework_value::RetainedCloneStep::Complete(progress),semio_framework_job::InteractiveJobCloseStep::Blocked=>{blocked=true;semio_framework_value::RetainedCloneStep::Progress(Default::default())}}))?;
        if blocked{return Ok(semio_framework_job::InteractiveJobCloseStep::Blocked)}
        if self.terminal_is_empty(){Ok(semio_framework_job::InteractiveJobCloseStep::Complete{progress:step.progress()})}else{Ok(step)}
    }

    fn next_close_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{
        if self.state.is_none()&&self.checkpoint.is_some()||self.commit_encoder.is_some(){return Ok(0);}
        if let Some(payload)=self.lent.as_ref().or(self.commit_output.as_ref()).filter(|_|self.commit_writer.is_none()){return payload.retirement_demands().map(|demand|demand.copy_bytes);}
        self.commit_writer.as_ref().map_or(Ok(0),|writer|writer.retirement_demands().map(|demand|demand.copy_bytes))
    }

    fn next_close_capacity_byte_demand(&self,_:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}

    fn next_close_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{
        if self.terminal_is_empty(){return Ok(0);}
        if self.state.is_none()&&self.checkpoint.is_some()||self.commit_encoder.is_some(){return Ok(1);}
        if let Some(payload)=self.lent.as_ref().or(self.commit_output.as_ref()).filter(|_|self.commit_writer.is_none()){return payload.retirement_demands().map(|demand|demand.depth);}
        self.commit_writer.as_ref().map_or(Ok(1),|writer|writer.retirement_demands().map(|demand|demand.depth))
    }

    fn next_close_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{
        if self.state.is_none()&&self.checkpoint.is_some()||self.commit_encoder.is_some(){return Ok(0);}
        if let Some(writer)=self.commit_writer.as_ref(){return writer.retirement_demands().map(|demand|demand.release_bytes);}
        if let Some(payload)=self.lent.as_ref().or(self.commit_output.as_ref()){return payload.retirement_demands().map(|demand|demand.release_bytes);}
        if self.preview.is_some()||self.fault.is_some(){return Ok(0);}
        let Some(state)=self.state.as_ref()else{return Ok(0)};
        if let Some(placement)=state.pending_placement.as_ref(){return Ok(placement.handles.next_release_bytes());}
        if state.accept_handle_template.is_some()||state.accept_handle_id.is_some()||state.accept_handle_pending_virtual.is_some()||state.accept_handle_pending_placement.is_some()||state.accept_pending_virtual_node.is_some()||state.source_capture.is_some()||state.compatibility_candidate.is_some()||state.current_preview.is_some()||state.candidate_event.is_some()||state.rejection.is_some(){return Ok(0);}
        if let Some(kind)=state.snapshot.kinds.len.checked_sub(1).and_then(|index|state.snapshot.kinds.get(index)){if !kind.handles.terminal_is_empty(){return Ok(kind.handles.next_release_bytes());}}
        if !state.snapshot.kinds.is_empty()||!state.virtual_handles.is_empty()||!state.virtual_nodes.is_empty()||!state.candidates.is_empty()||!state.sources.is_empty()||!state.snapshot.rules.is_empty()||!state.snapshot.handles.is_empty()||!state.snapshot.nodes.is_empty(){return Ok(0);}
        for bytes in [state.virtual_handles.next_release_bytes(),state.virtual_nodes.next_release_bytes(),state.candidates.next_release_bytes(),state.sources.next_release_bytes(),state.snapshot.rules.next_release_bytes(),state.snapshot.handles.next_release_bytes(),state.snapshot.nodes.next_release_bytes(),state.snapshot.kinds.next_release_bytes()]{if bytes!=0{return Ok(bytes);}}
        Ok(0)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.state.is_none() && self.checkpoint.is_none() && self.preview.is_none() && self.commit_encoder.is_none() && self.commit_writer.is_none() && self.lent.is_none() && self.commit_output.is_none() && self.fault.is_none()
    }
}

impl Drop for BoardFillJob {
    fn drop(&mut self) {
        assert!(semio_framework_job::InteractiveJob::terminal_is_empty(self) || std::thread::panicking(), "Puzzle2d fill job must reach exact terminal-empty before Drop");
    }
}

const _: fn() = || {
    fn assert_send<T: Send>() {}
    assert_send::<BoardFillSnapshot>();
    assert_send::<BoardFillSnapshotCapture>();
    assert_send::<BoardFillSnapshotIngress>();
    assert_send::<BoardFillCheckpoint>();
    assert_send::<BoardFillPlacement>();
    assert_send::<BoardFillJob>();
};
    //#endregion 🧵️FillJob
// #endregion board_host
