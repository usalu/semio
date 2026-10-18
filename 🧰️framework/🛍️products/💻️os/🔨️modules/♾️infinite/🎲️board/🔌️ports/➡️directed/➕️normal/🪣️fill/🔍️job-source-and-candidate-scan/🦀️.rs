//! 🔍️ Job source and candidate scan of the board fill: source preparation, target and candidate selection, compatibility, preview construction and collision scans. See [`super`].

use super::*;

// #region board_host
//#region 🪣️Fill
impl BoardFillJob {
    pub(super) fn prepare_source(state: &mut BoardFillJobState) -> Result<(), &'static str> {
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

    pub(super) fn select_target(state: &mut BoardFillJobState) -> Result<(), &'static str> {
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

    pub(super) fn prepare_candidate(state: &mut BoardFillJobState) -> Result<(), &'static str> {
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
            if !handle_port_shapes_compatible(target.handle_kind.as_str(), template.handle_kind.as_str()) || !single_letter_port_families_compatible(target.handle_kind.as_str(), template.handle_kind.as_str()) {
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

    pub(super) fn scan_compatibility(state: &mut BoardFillJobState) -> Result<(), &'static str> {
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

    pub(super) fn select_candidate(state: &mut BoardFillJobState) -> Result<(), &'static str> {
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

    pub(super) fn construct_preview(state: &mut BoardFillJobState) -> Result<(), &'static str> {
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

    pub(super) fn scan_host_collision(state: &mut BoardFillJobState) -> Result<(), &'static str> {
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

    pub(super) fn scan_virtual_collision(state: &mut BoardFillJobState) -> Result<(), &'static str> {
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

    pub(super) fn mark_source_connected(state: &mut BoardFillJobState, owner: BoardFillSourceOwner) -> Result<(), &'static str> {
        match owner {
            BoardFillSourceOwner::Host(index) => state.snapshot.handles.get_mut(index).ok_or("missing-host-source").map(|handle| handle.connected = true),
            BoardFillSourceOwner::Virtual(index) => state.virtual_handles.get_mut(index).ok_or("missing-virtual-source").map(|handle| handle.connected = true),
        }
    }
}
//#endregion 🧵️FillJob
// #endregion board_host
