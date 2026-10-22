//! 📥️ Fill job and snapshot capture of the board fill: the fill job struct, the offer enum, the snapshot begin function and the snapshot capture impl with its close protocol. See [`super`].

use super::*;

/// 🧵️ Persistent worker-owned fill search over a Send board snapshot.
pub struct BoardFillJob {
        pub(super) original_source: Option<semio_framework_action_bus::OriginalToolSourceBinding>,
pub operation: semio_framework_job::Operation,
    pub naming: &'static BoardNaming,
    pub state: Option<BoardFillJobState>,
    pub checkpoint: Option<BoardFillCheckpoint>,
    pub preview: Option<BoardFillPreview>,
    pub(super) commit_encoder: Option<BoardFillCommitEncoder>,
    pub(super) commit_writer: Option<semio_framework_job_payload::RetainedJobPayloadWriter>,
    pub(super) lent: Option<semio_framework_job_payload::RetainedJobPayload>,
    pub(super) commit_output: Option<semio_framework_job_payload::RetainedJobPayload>,
    pub(super) offer: Cell<Option<BoardFillOffer>>,
    pub admitted: Cell<bool>,
    pub fault: Option<&'static str>,
    pub closing: bool,
}

/// 🎟️ The one outcome a fill turn owes its driver; it stays offered until admission succeeds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum BoardFillOffer { Yield, Preview, Checkpoint(u64), Complete, Fault, Cancelled }

// #region board_host
pub fn begin_board_fill_snapshot(style: &BoardStyleState, brush: &BoardBrushState, output: &mut Option<BoardFillSnapshotCapture>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
    BoardFillSnapshotCapture::admit_original(brush.suggestion_offset, style.naming, output, cx)
}

impl BoardFillSnapshotCapture {
    pub fn admission_demands() -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        let mut demand = BoardFillSnapshot::descriptor_demands(); demand.copy_bytes += size_of::<Self>(); Ok(demand)
    }
    pub fn admit_original(suggestion_offset: f64, naming: &'static BoardNaming, output: &mut Option<Self>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        use semio_framework_action_bus::{admit_original_tool_construction_scoped, ToolJobAdmissionDemand};
        use semio_framework_value::{RetainedCloneProgress, ValueError, ValueRefusalKind};
        let demand = Self::admission_demands()?;
        let mut allocation_failed = false;
        let mut constructed_progress = RetainedCloneProgress::default();
        let admitted = admit_original_tool_construction_scoped(cx, output, ToolJobAdmissionDemand { items: 1, copy_bytes: demand.copy_bytes, capacity_bytes: demand.capacity_bytes, release_bytes: 0, depth: demand.depth }, |scope| {
            scope.admit_owner(demand)?;
            let mut candidate = Self::unallocated(suggestion_offset, naming);
            let snapshot = candidate.snapshot.as_mut().expect("original snapshot construction carrier");
            snapshot.original_source = Some(scope.identity().source_binding());
            let mut progress = RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Self>(), ..Default::default() };
            macro_rules! descriptor {
                ($field:ident) => { if !allocation_failed { match BoardFillFixedPages::try_new() {
                    Ok(pages) => { let bytes = size_of_val(pages.pages.as_ref()); progress.retained_capacity_bytes += bytes; progress.copied_bytes += bytes; snapshot.$field = pages; },
                    Err(()) => allocation_failed = true,
                } } };
            }
            descriptor!(nodes); descriptor!(handles); descriptor!(kinds); descriptor!(rules);
            constructed_progress = progress;
            Ok((candidate, progress))
        })?;
        if allocation_failed { return Err(ValueError::literal(ValueRefusalKind::AllocationFailed, "board fill retains its partial snapshot owner for original close").with_retained_progress(constructed_progress).into()); }
        Ok(admitted)
    }

    fn unallocated(suggestion_offset: f64, naming: &'static BoardNaming) -> Self {
        Self {
            snapshot: Some(BoardFillSnapshot { original_source: None, nodes: BoardFillFixedPages::unallocated(), handles: BoardFillFixedPages::unallocated(), kinds: BoardFillFixedPages::unallocated(), rules: BoardFillFixedPages::unallocated(), suggestion_offset, naming }),
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

    fn capture_node(&mut self, scene: &BoardSceneState, style: &BoardStyleState) -> Result<(), BoardFillCaptureFault> {
        if let Some(current) = self.node.as_mut() {
            if current.bounds.is_none() {
                let node = scene.nodes.get(current.id.as_str()).ok_or(BoardFillCaptureFault::StaleNode)?;
                let bounds = style.node_world_bounds(node, 0.0);
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
        let Some((key, _)) = self.next_map_entry(&scene.nodes) else {
            self.phase = BoardFillCapturePhase::Handles;
            self.last_key = None;
            return Ok(());
        };
        let id = BoardFillText::try_from_str(key)?;
        self.node = Some(BoardFillNodeCapture { id, bounds: None });
        Ok(())
    }

    fn capture_handle(&mut self, scene: &BoardSceneState, style: &BoardStyleState, brush: &BoardBrushState) -> Result<(), BoardFillCaptureFault> {
        if let Some(current) = self.handle.as_mut() {
            match current.stage {
                BoardFillHandleCaptureStage::NodeKind => {
                    let handle = scene.handles.get(current.handle.id.as_str()).ok_or(BoardFillCaptureFault::StaleHandle)?;
                    let node = scene.nodes.get(&handle.node_id).ok_or(BoardFillCaptureFault::StaleNode)?;
                    current.handle.node_kind = BoardFillText::try_from_str(&node.node_kind)?;
                    current.stage = BoardFillHandleCaptureStage::HandleKind;
                }
                BoardFillHandleCaptureStage::HandleKind => {
                    let handle = scene.handles.get(current.handle.id.as_str()).ok_or(BoardFillCaptureFault::StaleHandle)?;
                    current.handle.handle_kind = BoardFillText::try_from_str(&handle.handle_kind)?;
                    current.stage = BoardFillHandleCaptureStage::Slot;
                }
                BoardFillHandleCaptureStage::Slot => {
                    let handle = scene.handles.get(current.handle.id.as_str()).ok_or(BoardFillCaptureFault::StaleHandle)?;
                    let anchor = brush_handle_anchor_world(scene, style, handle).ok_or(BoardFillCaptureFault::StaleHandle)?;
                    let slot = scene.handle_slot_center_world(&handle.node_id, anchor, brush.suggestion_offset).ok_or(BoardFillCaptureFault::StaleHandle)?;
                    current.handle.slot = [slot.x, slot.y];
                    current.stage = BoardFillHandleCaptureStage::WireKind;
                }
                BoardFillHandleCaptureStage::WireKind => {
                    let wire = style.board_fill_default_wire_kind(current.handle.handle_kind.as_str())?;
                    current.handle.wire_kind = BoardFillText::try_from_str(wire)?;
                    current.stage = BoardFillHandleCaptureStage::EdgeKind;
                }
                BoardFillHandleCaptureStage::EdgeKind => {
                    let edge = style.board_fill_default_edge_kind(current.handle.wire_kind.as_str())?;
                    current.handle.edge_kind = BoardFillText::try_from_str(edge)?;
                    current.stage = BoardFillHandleCaptureStage::Visible;
                }
                BoardFillHandleCaptureStage::Visible => {
                    let _ = scene.handles.get(current.handle.id.as_str()).ok_or(BoardFillCaptureFault::StaleHandle)?;
                    current.handle.visible = scene.handle_effectively_visible(current.handle.id.as_str());
                    current.stage = BoardFillHandleCaptureStage::Connected;
                    current.edge_key = None;
                }
                BoardFillHandleCaptureStage::Connected => {
                    if let Some((edge_key, edge)) = Self::next_map_entry_after(current.edge_key, &scene.edges) {
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
                    current.handle.weight = brush.brush_handle_kind_weights.get(current.handle.handle_kind.as_str()).copied().filter(|value| value.is_finite() && *value > 0.0).unwrap_or(1.0);
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
        let Some((key, _)) = self.next_map_entry(&scene.handles) else {
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

    fn capture_kind(&mut self, style: &BoardStyleState, brush: &BoardBrushState) -> Result<(), BoardFillCaptureFault> {
        if let Some(current) = self.kind.as_mut() {
            let source = style.node_kinds.get(current.value.id.as_str()).ok_or(BoardFillCaptureFault::StaleKind)?;
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
                    current.value.radius = brush.brush_node_size * 0.5 * source.scale;
                    current.stage = BoardFillKindCaptureStage::Width;
                }
                BoardFillKindCaptureStage::Width => {
                    current.value.width = if source.shape == NodeShape::Rectangle { brush.brush_node_size * source.scale } else { current.value.radius * 2.0 };
                    current.stage = BoardFillKindCaptureStage::Height;
                }
                BoardFillKindCaptureStage::Height => {
                    current.value.height = if source.shape == NodeShape::Rectangle { brush.brush_node_size * source.scale } else { current.value.radius * 2.0 };
                    current.stage = BoardFillKindCaptureStage::Weight;
                }
                BoardFillKindCaptureStage::Weight => {
                    current.value.weight = brush.brush_node_kind_weights.get(current.value.id.as_str()).copied().filter(|value| value.is_finite() && *value > 0.0).unwrap_or(1.0);
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
                                let wire = style.board_fill_default_wire_kind(template.value.handle_kind.as_str())?;
                                template.value.wire_kind = BoardFillText::try_from_str(wire)?;
                                template.stage = BoardFillTemplateCaptureStage::EdgeKind;
                            }
                            BoardFillTemplateCaptureStage::EdgeKind => {
                                let edge = style.board_fill_default_edge_kind(template.value.wire_kind.as_str())?;
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
                                template.value.weight = brush.brush_handle_kind_weights.get(template.value.handle_kind.as_str()).copied().filter(|value| value.is_finite() && *value > 0.0).unwrap_or(1.0);
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
        let Some((key, _)) = self.next_map_entry(&style.node_kinds) else {
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

    fn capture_rule(&mut self, style: &BoardStyleState) -> Result<(), BoardFillCaptureFault> {
        if let Some(current) = self.rule.as_mut() {
            let source = style.link_compat_rules.get(self.rule_cursor).ok_or(BoardFillCaptureFault::StaleRule)?;
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
        let Some(source) = style.link_compat_rules.get(self.rule_cursor) else {
            self.phase = BoardFillCapturePhase::Complete;
            return Ok(());
        };
        self.rule = Some(BoardFillRuleCapture {
            value: BoardFillRuleSnapshot { source: BoardFillText::try_from_str(&source.source)?, target: BoardFillText::empty(), bidirectional: false, specificity: CompatSpecificity::General },
            stage: BoardFillRuleCaptureStage::Target,
        });
        Ok(())
    }

    pub fn next_step_demands(&self, style: &BoardStyleState) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        fn page<T, const N: usize>(pages: &BoardFillFixedPages<T, N>) -> usize {
            if pages.len < BoardFillFixedPages::<T, N>::item_capacity() && pages.pages.get(pages.len / BOARD_FILL_PAGE_ITEMS).is_some_and(Option::is_none) { BOARD_FILL_PAGE_ITEMS * size_of::<T>() } else { 0 }
        }
        let capacity_bytes = match self.phase {
            BoardFillCapturePhase::Nodes if self.node.as_ref().is_some_and(|node| node.bounds.is_some()) => self.snapshot.as_ref().map_or(0, |snapshot| page(&snapshot.nodes)),
            BoardFillCapturePhase::Handles if self.handle.as_ref().is_some_and(|handle| matches!(handle.stage, BoardFillHandleCaptureStage::Publish)) => self.snapshot.as_ref().map_or(0, |snapshot| page(&snapshot.handles)),
            BoardFillCapturePhase::Kinds => match self.kind.as_ref() {
                None if self.next_map_entry(&style.node_kinds).is_some() => BoardFillFixedPages::<BoardFillTemplateSnapshot, { BOARD_FILL_KIND_HANDLE_CAPACITY / BOARD_FILL_PAGE_ITEMS }>::descriptor_bytes(),
                None => 0,
                Some(kind) if matches!(kind.stage, BoardFillKindCaptureStage::Publish) => self.snapshot.as_ref().map_or(0, |snapshot| page(&snapshot.kinds)),
                Some(kind) if matches!(kind.stage, BoardFillKindCaptureStage::Templates) && kind.template.as_ref().is_some_and(|template| matches!(template.stage, BoardFillTemplateCaptureStage::Publish)) => page(&kind.value.handles),
                _ => 0,
            },
            BoardFillCapturePhase::Rules if self.rule.as_ref().is_some_and(|rule| matches!(rule.stage, BoardFillRuleCaptureStage::Publish)) => self.snapshot.as_ref().map_or(0, |snapshot| page(&snapshot.rules)),
            _ => 0,
        };
        Ok(semio_framework_value::RetirementDemand { copy_bytes: size_of::<Self>() + capacity_bytes, capacity_bytes, release_bytes: 0, depth: 2 })
    }

    pub fn step(&mut self, scene: &BoardSceneState, style: &BoardStyleState, brush: &BoardBrushState, cx: &mut semio_framework_job::StepContext<'_>) -> Result<BoardFillCaptureStep, semio_framework_job::OriginalTurnFailure> {
        use semio_framework_action_bus::{admit_original_tool_construction_scoped, ToolJobAdmissionDemand};
        use semio_framework_value::{RetainedCloneProgress, ValueError, ValueRefusalKind};
        cx.original_source_parent()?.validate_borrowed_context(cx)?;
        let snapshot = self.snapshot.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "board capture original snapshot is absent"))?;
        if !snapshot.original_source.is_some_and(|source| source.matches(cx)) { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "board capture refused a foreign original source").into()); }
        if self.closing { return Ok(BoardFillCaptureStep::Fault(BoardFillCaptureFault::GenerationExhausted)); }
        if matches!(self.phase, BoardFillCapturePhase::Complete) { return Ok(BoardFillCaptureStep::Complete); }
        let demand = self.next_step_demands(style)?;
        let mut output = None;
        admit_original_tool_construction_scoped(cx, &mut output, ToolJobAdmissionDemand { items: 1, copy_bytes: demand.copy_bytes, capacity_bytes: demand.capacity_bytes, release_bytes: 0, depth: demand.depth }, |scope| {
            scope.admit_owner(demand)?;
            let result = match self.phase {
                BoardFillCapturePhase::Nodes => self.capture_node(scene, style),
                BoardFillCapturePhase::Handles => self.capture_handle(scene, style, brush),
                BoardFillCapturePhase::Kinds => self.capture_kind(style, brush),
                BoardFillCapturePhase::Rules => self.capture_rule(style),
                BoardFillCapturePhase::Complete => Ok(()),
                BoardFillCapturePhase::Closing => Err(BoardFillCaptureFault::GenerationExhausted),
            };
            let capacity_bytes = if result.is_ok() { demand.capacity_bytes } else { 0 };
            let outcome = match result {
                Ok(()) if self.phase == BoardFillCapturePhase::Complete => BoardFillCaptureStep::Complete,
                Ok(()) => BoardFillCaptureStep::Pending,
                Err(fault) => BoardFillCaptureStep::Fault(fault),
            };
            Ok((outcome, RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Self>() + capacity_bytes, retained_capacity_bytes: capacity_bytes, released_bytes: 0 }))
        })?;
        Ok(output.unwrap_or(BoardFillCaptureStep::Pending))
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
        assert!(self.terminal_is_empty() || std::thread::panicking(), "Board fill capture must reach exact terminal-empty before Drop");
    }
}
//#endregion 🧵️FillJob
// #endregion board_host
