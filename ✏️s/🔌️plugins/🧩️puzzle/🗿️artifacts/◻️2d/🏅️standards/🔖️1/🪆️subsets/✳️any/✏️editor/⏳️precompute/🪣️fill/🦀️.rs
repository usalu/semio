//! 🪣️ Puzzle 2d fill tool run (`📋️tool-run-contract.md` §2.7, §3.2, §3.3, §3.7): the run job streams the
//! document into the board engine's fill ingress, drives its `BoardFillJob` search and reports every
//! candidate the search decides as a `placement2d` trace record — `testing`, then `danger` (collision),
//! `warning` (port or kind rule) or `success` (placed). Every placement appends its `create_node` +
//! `connect_handles` ops and one entity to the tool run ledger; nothing touches the store before
//! finalize, where the revalidate job re-tests the provisional placements against the head.
//! Vocabulary source of record: `$defs.Puzzle2dFillRun` in `✳️any/🧬️schema/🔣️.json`.

use crate::editor::puzzle2d::engine::{
    BoardFillCandidateEvent, BoardFillCandidateVerdict, BoardFillCaptureFault, BoardFillIngressHandleText, BoardFillIngressKindText, BoardFillIngressRuleText, BoardFillIngressTemplateText, BoardFillJob, BoardFillPlacement, BoardFillSnapshot, BoardFillSnapshotIngress, BoardFillStage,
};
#[cfg(test)] use crate::standards::v1::subsets::any::schema::mutations::{};
use crate::editor::puzzle2d::engine::board_host::PUZZLE_2D_BOARD_NAMING;
use crate::editor::puzzle2d::snapshot::Puzzle2dPlaySnapshot;

use crate::standards::v1::subsets::any::schema::mutations::{Puzzle2dMutation};

use crate::standards::v1::subsets::any::schema::mutations::{connect_handles,create_node};

use crate::puzzle_job::JobTurn;
use semio_framework_job::{Generation, InteractiveJob, InteractiveJobCloseStep, JobOutcomeBorrow, Operation, OperationId, RevisionId, StepBudget, StepContext};
use semio_framework_tool_run::{ToolRunCounter, ToolRunIdentity, ToolRunProgress, ToolRunState, ToolRunStepArg, ToolRunStepKind, ToolRunStepRing, ToolRunTickWriter, ToolRunTraceSubject, ToolRunVerdict, TOOL_RUN_REASON_CONFLICT};
use semio_framework_pack_json::Value;
use std::collections::HashSet;
use std::sync::Arc;

#[path = "📏️limits-vocabulary-and-placement/🦀️.rs"]
mod limits_vocabulary_and_placement;
pub use limits_vocabulary_and_placement::{
    fill_bounds_overlap_with, fill_kind_rows, fill_node_bounds, fill_regions_admit, fill_run_entity,
    fill_run_monotonic_zero, fill_visible_region_bounds, FillRunCheckpoint, FillRunCounter,
    FillRunPlacementKey, FillRunReason, FillRunStage, FILL_RUN_OPS_PER_PLACEMENT,
    FILL_RUN_TARGET_REGION_SLOTS, FILL_RUN_TICK_FLUSH_BYTES,
};
use limits_vocabulary_and_placement::{
    fill_placement_mutations, fill_run_fault, fill_serial_after, FILL_RUN_CAPTURE_UNITS,
    FILL_RUN_DEADLINE_CHECK, FILL_RUN_SEARCH_BATCH,
};

#[path = "🔬️capture-cursor/🦀️.rs"]
mod capture_cursor;
use capture_cursor::{
    capture_fault_code, ArtifactFillCaptureCursor, ArtifactFillCaptureStage,
    ArtifactHandleCaptureField, ArtifactKindCaptureField, ArtifactNodeCaptureField,
    ArtifactRuleCaptureField,
};

#[path = "🧲️fill-capture/🦀️.rs"]
mod fill_capture;
use fill_capture::FillCapture;

#[path = "⏯️run-job-state-and-search/🦀️.rs"]
mod run_job_state_and_search;
use run_job_state_and_search::{FillRunOwed, FillSearch, Puzzle2dFillRunOwners};
pub use run_job_state_and_search::Puzzle2dFillRunJob;

#[path = "🏃️run-job-stepping-and-finalize/🦀️.rs"]
mod run_job_stepping_and_finalize;

#[path = "🔍️revalidate-job/🦀️.rs"]
mod revalidate_job;
pub use revalidate_job::Puzzle2dFillRevalidateJob;

//#region 🧪️Tests

//#endregion 🧪️Tests
