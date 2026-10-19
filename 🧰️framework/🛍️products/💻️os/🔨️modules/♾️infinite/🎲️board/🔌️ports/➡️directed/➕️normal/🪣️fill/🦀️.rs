//! 🪣️ Normal board fill: snapshot capture and ingress, the fill job state machine and the commit encoder.
//!
//! Crate `semio-framework-os-infinite-board-normal-fill`; a downward producer of the infinite board family.

#![allow(clippy::missing_errors_doc, reason = "Graph board host is internal to directed port normal.")]
#![allow(clippy::too_many_arguments, reason = "Immediate-mode paint helpers take one positional arg per geometry/style input; grouping them into structs would obscure the call sites.")]

use semio_framework_os_infinite_board_schema::{absent, array, field};
use semio_framework_os_infinite_board_engine::CompatSpecificity;
use semio_framework_os_infinite_board_normal_model::{BoardFillStage, BoardNaming, handle_port_shapes_compatible, single_letter_port_families_compatible};
use semio_framework_os_infinite_board_normal_scene::{BOARD_FILL_TEXT_BYTES, BoardFillCaptureFault, BoardSceneState, BoardStyleState, brush_handle_anchor_world};
use semio_framework_os_infinite_board_normal_gestures::BoardBrushState;
use std::collections::BTreeMap;
use std::cell::Cell;
use semio_framework_canvas::{Circle, Point};
use graph::{EdgeId, HandleId, NodeId, NodeShape};
use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::{ValueError, ValueRefusalKind};
use semio_framework_graph_drawing::routing::{handle_position_on_circle, handle_position_on_rectangle};
use geometry::normalize_or_zero;
use semio_framework_value::retirement::sequence;

#[path = "🏷️naming-capacities-and-pages/🦀️.rs"]
mod naming_capacities_and_pages;
use naming_capacities_and_pages::BOARD_FILL_CANDIDATE_CAPACITY;
pub use naming_capacities_and_pages::{
    BoardFillFixedPages, BoardFillText, BOARD_FILL_HANDLE_CAPACITY, BOARD_FILL_KIND_CAPACITY, BOARD_FILL_KIND_HANDLE_CAPACITY, BOARD_FILL_NODE_CAPACITY, BOARD_FILL_PAGE_ITEMS, BOARD_FILL_PLACEMENT_CAPACITY, BOARD_FILL_RULE_CAPACITY,
    BOARD_FILL_SOURCE_CAPACITY, NEUTRAL_BOARD_NAMING,
};

#[path = "📸️snapshot-and-capture-stages/🦀️.rs"]
mod snapshot_and_capture_stages;
use snapshot_and_capture_stages::{
    BoardFillCapturePhase, BoardFillHandleCapture, BoardFillHandleCaptureStage, BoardFillHandleSnapshot, BoardFillKindCapture, BoardFillKindCaptureStage, BoardFillKindSnapshot, BoardFillNodeCapture, BoardFillNodeSnapshot, BoardFillRuleCapture,
    BoardFillRuleCaptureStage, BoardFillRuleSnapshot, BoardFillShape, BoardFillTemplateCapture, BoardFillTemplateCaptureStage, BoardFillTemplateSnapshot,
};
pub use snapshot_and_capture_stages::{BoardFillCaptureStep, BoardFillIngressHandleText, BoardFillIngressKindText, BoardFillIngressRuleText, BoardFillIngressTemplateText, BoardFillSnapshot, BoardFillSnapshotCapture, BoardFillSnapshotIngress};

#[path = "🎯️source-candidate-and-preview/🦀️.rs"]
mod source_candidate_and_preview;
use source_candidate_and_preview::{BoardFillCandidate, BoardFillCandidatePreview, BoardFillPlacementHandle, BoardFillSourceCapture, BoardFillSourceCaptureStage, BoardFillSourceOwner, BoardFillVirtualNode};
pub use source_candidate_and_preview::{BoardFillCandidateEvent, BoardFillCandidateVerdict, BoardFillJobState, BoardFillPlacement, BoardFillPreview, BoardFillSource, BoardFillVirtualHandle};

#[path = "📍️placement-checkpoint-and-commit-records/🦀️.rs"]
mod placement_checkpoint_and_commit_records;
pub use placement_checkpoint_and_commit_records::{BoardFillCheckpoint, BoardFillCommitCandidate, BoardFillCommitHandle, BoardFillCommitPlacement, BoardFillCommitShape, BoardFillResult};

#[path = "🧬️commit-encoder-and-payload/🦀️.rs"]
mod commit_encoder_and_payload;
use commit_encoder_and_payload::BoardFillCommitEncodeStage;
pub use commit_encoder_and_payload::{BoardFillCommitEncoder, BOARD_FILL_COMMIT_BYTES, BOARD_FILL_COMMIT_PAGE_COUNT};

#[path = "📥️fill-job-and-snapshot-capture/🦀️.rs"]
mod fill_job_and_snapshot_capture;
use fill_job_and_snapshot_capture::BoardFillOffer;
pub use fill_job_and_snapshot_capture::{begin_board_fill_snapshot, BoardFillJob};

#[path = "🚪️snapshot-ingress/🦀️.rs"]
mod snapshot_ingress;

#[path = "🧵️job-operation-and-state/🦀️.rs"]
mod job_operation_and_state;

#[path = "🔍️job-source-and-candidate-scan/🦀️.rs"]
mod job_source_and_candidate_scan;

#[path = "✅️job-accept/🦀️.rs"]
mod job_accept;

#[path = "🚫️job-reject-preview-and-complete/🦀️.rs"]
mod job_reject_preview_and_complete;

#[path = "🏁️job-close-and-interactive/🦀️.rs"]
mod job_close_and_interactive;


impl BoardFillJob {
    pub fn admission_demands() -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        let bytes = BoardFillFixedPages::<BoardFillSource, { BOARD_FILL_SOURCE_CAPACITY.div_ceil(BOARD_FILL_PAGE_ITEMS) }>::descriptor_bytes()
            + BoardFillFixedPages::<BoardFillCandidate, { BOARD_FILL_CANDIDATE_CAPACITY / BOARD_FILL_PAGE_ITEMS }>::descriptor_bytes()
            + BoardFillFixedPages::<BoardFillVirtualNode, { BOARD_FILL_PLACEMENT_CAPACITY.div_ceil(BOARD_FILL_PAGE_ITEMS) }>::descriptor_bytes()
            + BoardFillFixedPages::<BoardFillVirtualHandle, { (BOARD_FILL_PLACEMENT_CAPACITY * BOARD_FILL_KIND_HANDLE_CAPACITY).div_ceil(BOARD_FILL_PAGE_ITEMS) }>::descriptor_bytes();
        Ok(semio_framework_value::RetirementDemand { copy_bytes: bytes + size_of::<Self>(), capacity_bytes: bytes, depth: 1, ..Default::default() })
    }

    pub fn admit_original(snapshot: &mut Option<BoardFillSnapshot>, max_count: u32, semantic_operation: semio_framework_job::Operation, output: &mut Option<Self>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        use semio_framework_action_bus::{admit_original_tool_construction_scoped, ToolJobAdmissionDemand};
        use semio_framework_value::{RetainedCloneProgress, ValueError, ValueRefusalKind};
        if snapshot.is_none() || output.is_some() { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "board fill admission requires its original snapshot and empty output").into()); }
        if snapshot.as_ref().is_some_and(|snapshot| snapshot.original_source.is_some_and(|source| !source.matches(cx))) { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "board fill admission refused a foreign original snapshot").into()); }
        let demand = Self::admission_demands()?;
        let mut allocation_failed = false;
        let mut constructed_progress = RetainedCloneProgress::default();
        let admitted = admit_original_tool_construction_scoped(cx, output, ToolJobAdmissionDemand { items: 1, copy_bytes: demand.copy_bytes, capacity_bytes: demand.capacity_bytes, release_bytes: 0, depth: demand.depth }, |scope| {
            scope.admit_owner(demand)?;
            let binding = scope.identity().source_binding();
            let mut original = snapshot.take().expect("original snapshot retained until admitted construction");
            original.original_source = Some(binding);
            let mut job = Self::from_original_state(original, max_count, semantic_operation, binding);
            let mut progress = RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Self>(), ..Default::default() };
            let state = job.state.as_mut().expect("original board fill construction state");
            macro_rules! descriptor {
                ($field:ident) => {
                    if !allocation_failed {
                        match BoardFillFixedPages::try_new() {
                            Ok(pages) => { let bytes = size_of_val(pages.pages.as_ref()); progress.retained_capacity_bytes += bytes; progress.copied_bytes += bytes; state.$field = pages; }
                            Err(()) => allocation_failed = true,
                        }
                    }
                };
            }
            descriptor!(sources); descriptor!(candidates); descriptor!(virtual_nodes); descriptor!(virtual_handles);
            if allocation_failed { job.fault = Some("board-fill-descriptor-allocation-failed"); }
            constructed_progress = progress;
            Ok((job, progress))
        })?;
        if allocation_failed { return Err(ValueError::literal(ValueRefusalKind::AllocationFailed, "board fill retains its partial descriptor candidate for original close").with_retained_progress(constructed_progress).into()); }
        Ok(admitted)
    }

    fn validate_original_source(&self, cx: &semio_framework_job::StepContext<'_>) -> Result<(), semio_framework_job::OriginalTurnFailure> {
        cx.original_source_parent()?.validate_borrowed_context(cx)?;
        if self.original_source.is_none_or(|original| !original.matches(cx)) { return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "board fill refused a foreign original source").into()); }
        Ok(())
    }
}

impl BoardFillSnapshot {
    fn descriptor_demands() -> semio_framework_value::RetirementDemand {
        let bytes = BoardFillFixedPages::<BoardFillNodeSnapshot, { BOARD_FILL_NODE_CAPACITY / BOARD_FILL_PAGE_ITEMS }>::descriptor_bytes()
            + BoardFillFixedPages::<BoardFillHandleSnapshot, { BOARD_FILL_HANDLE_CAPACITY / BOARD_FILL_PAGE_ITEMS }>::descriptor_bytes()
            + BoardFillFixedPages::<BoardFillKindSnapshot, { BOARD_FILL_KIND_CAPACITY / BOARD_FILL_PAGE_ITEMS }>::descriptor_bytes()
            + BoardFillFixedPages::<BoardFillRuleSnapshot, { BOARD_FILL_RULE_CAPACITY / BOARD_FILL_PAGE_ITEMS }>::descriptor_bytes();
        semio_framework_value::RetirementDemand { copy_bytes: bytes, capacity_bytes: bytes, depth: 1, ..Default::default() }
    }

    pub fn next_close_demands(&self, _body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        if self.terminal_is_empty() { return Ok(Default::default()); }
        if let Some(kind) = self.kinds.len.checked_sub(1).and_then(|index| self.kinds.get(index)) {
            if !kind.handles.terminal_is_empty() { return Ok(kind.handles.close_frontier()); }
        }
        if !self.kinds.is_empty() { return Ok(self.kinds.close_frontier()); }
        if !self.rules.is_empty() { return Ok(self.rules.close_frontier()); }
        if !self.handles.is_empty() { return Ok(self.handles.close_frontier()); }
        if !self.nodes.is_empty() { return Ok(self.nodes.close_frontier()); }
        for demand in [self.rules.close_frontier(), self.handles.close_frontier(), self.nodes.close_frontier(), self.kinds.close_frontier()] { if demand.release_bytes != 0 { return Ok(demand); } }
        Ok(Default::default())
    }

    pub fn close_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_job::InteractiveJobCloseStep, semio_framework_job::OriginalTurnFailure> {
        cx.original_source_parent()?.validate_borrowed_context(cx)?;
        let binding = semio_framework_action_bus::OriginalToolSourceBinding::borrow(cx);
        if self.original_source.is_some_and(|source| source != binding) { return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "board fill snapshot refused a foreign original source").into()); }
        self.original_source = Some(binding);
        if self.terminal_is_empty() { return Ok(semio_framework_job::InteractiveJobCloseStep::Complete { progress: Default::default() }); }
        let demand = self.next_close_demands(cx.retained_grant().maximum_copy_bytes)?;
        let step = semio_framework_job::close_original_retained_step(cx, demand, |grant| {
            if let Some(kind) = self.kinds.len.checked_sub(1).and_then(|index| self.kinds.get_mut(index)) {
                if !kind.handles.terminal_is_empty() { return Ok(kind.handles.close_plain_granted(grant)); }
            }
            if !self.kinds.is_empty() { return Ok(self.kinds.close_plain_granted(grant)); }
            if !self.rules.is_empty() { return Ok(self.rules.close_plain_granted(grant)); }
            if !self.handles.is_empty() { return Ok(self.handles.close_plain_granted(grant)); }
            if !self.nodes.is_empty() { return Ok(self.nodes.close_plain_granted(grant)); }
            macro_rules! retire { ($field:ident) => { if !self.$field.terminal_is_empty() { return Ok(self.$field.close_plain_granted(grant)); } }; }
            retire!(rules); retire!(handles); retire!(nodes); retire!(kinds);
            Ok(semio_framework_value::RetainedCloneStep::Complete(Default::default()))
        })?;
        Ok(if self.terminal_is_empty() { semio_framework_job::InteractiveJobCloseStep::Complete { progress: step.progress() } } else { step })
    }

    pub fn terminal_is_empty(&self) -> bool { self.nodes.terminal_is_empty() && self.handles.terminal_is_empty() && self.kinds.terminal_is_empty() && self.rules.terminal_is_empty() }
}

impl<T, const CAPACITY: usize> BoardFillFixedPages<T, CAPACITY> {
    fn close_frontier(&self) -> semio_framework_value::RetirementDemand {
        semio_framework_value::RetirementDemand { copy_bytes: if self.len > 0 { size_of::<T>() } else { 0 }, release_bytes: self.next_release_bytes(), depth: usize::from(!self.terminal_is_empty()), ..Default::default() }
    }
    fn close_plain_granted(&mut self, grant: semio_framework_value::RetainedCloneGrant) -> semio_framework_value::RetainedCloneStep {
        let demand = self.close_frontier();
        if grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_release_bytes < demand.release_bytes || grant.maximum_depth < demand.depth { return semio_framework_value::RetainedCloneStep::Progress(Default::default()); }
        if self.pop().is_some() { return semio_framework_value::RetainedCloneStep::Progress(semio_framework_value::RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }); }
        if let Some(released_bytes) = self.retire_empty_page() { return semio_framework_value::RetainedCloneStep::Progress(semio_framework_value::RetainedCloneProgress { copied_items: 1, released_bytes, ..Default::default() }); }
        self.retire_descriptor(grant).map_or(semio_framework_value::RetainedCloneStep::Complete(Default::default()), |step| semio_framework_value::RetainedCloneStep::Progress(step.progress()))
    }
}
