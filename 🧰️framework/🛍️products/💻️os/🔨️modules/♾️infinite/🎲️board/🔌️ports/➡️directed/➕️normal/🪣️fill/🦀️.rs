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
