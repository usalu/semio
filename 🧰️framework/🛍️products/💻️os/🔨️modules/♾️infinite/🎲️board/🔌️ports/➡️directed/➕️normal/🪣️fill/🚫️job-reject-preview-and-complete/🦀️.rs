//! 🚫️ Job reject, preview and complete of the board fill: rejecting a candidate or target, the preview, completion, fault outcome, the next offer and its present step. See [`super`].

use super::*;

// #region board_host
//#region 🪣️Fill
impl BoardFillJob {
    pub(super) fn reject_candidate(state: &mut BoardFillJobState, reason: &'static str) -> Result<(), &'static str> {
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

    pub(super) fn reject_target(state: &mut BoardFillJobState, reason: &'static str) -> Result<(), &'static str> {
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

    pub fn preview(state: &BoardFillJobState, operation: semio_framework_job::Operation, sequence: u64) -> BoardFillPreview {
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

    pub fn complete(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> BoardFillOffer {
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
    pub(super) fn next_offer(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> BoardFillOffer {
        if context.is_cancelled() {
            return BoardFillOffer::Cancelled;
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

    pub fn present<'a>(&'a self, context: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
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
//#endregion 🧵️FillJob
// #endregion board_host
