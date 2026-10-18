//! 🧵️ Job operation and state of the board fill: the operation scope, checkpoint hand over, event takes, seeds and box overlap of the fill job. See [`super`].

use super::*;

// #region board_host
//#region 🪣️Fill
impl BoardFillJob {
    pub fn with_operation(snapshot: BoardFillSnapshot, max_count: u32, operation: semio_framework_job::Operation) -> Self {
        Self {
            operation,
            naming: snapshot.naming,
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
        Ok(Self { operation, naming: checkpoint.naming, state: Some(state), checkpoint: None, preview: None, commit_encoder: None, commit_writer: None, lent: None, commit_output: None, offer: Cell::new(None), admitted: Cell::new(false), fault: None, closing: false })
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

    pub(super) fn kind_event(kind: &BoardFillKindSnapshot, kind_index: usize, verdict: BoardFillCandidateVerdict, [x, y]: [f64; 2]) -> BoardFillCandidateEvent {
        let bounds = match kind.shape {
            BoardFillShape::Rectangle => [x - kind.width / 2.0, y - kind.height / 2.0, x + kind.width / 2.0, y + kind.height / 2.0],
            BoardFillShape::Circle => [x - kind.radius, y - kind.radius, x + kind.radius, y + kind.radius],
        };
        BoardFillCandidateEvent { verdict, kind_index, position: [x, y], bounds }
    }

    pub fn preview_event(preview: &BoardFillCandidatePreview, verdict: BoardFillCandidateVerdict) -> BoardFillCandidateEvent {
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

    pub(super) fn stage_label(&self) -> &'static str {
        (self.naming.fill_stage_label)(self.stage())
    }

    fn next_seed(state: &mut BoardFillJobState) -> u64 {
        state.rng_state = state.rng_state.wrapping_mul(6364136223846793005).wrapping_add(1);
        state.rng_state
    }

    pub(super) fn weighted_rank(state: &mut BoardFillJobState, weight: f64) -> f64 {
        let unit = ((Self::next_seed(state) >> 11) as f64 + 1.0) / ((1_u64 << 53) as f64 + 1.0);
        -unit.ln() / weight.max(f64::EPSILON)
    }

    pub(super) fn boxes_overlap(left: [f64; 4], right: [f64; 4]) -> bool {
        left[0] <= right[2] && left[2] >= right[0] && left[1] <= right[3] && left[3] >= right[1]
    }
}
//#endregion 🧵️FillJob
// #endregion board_host
