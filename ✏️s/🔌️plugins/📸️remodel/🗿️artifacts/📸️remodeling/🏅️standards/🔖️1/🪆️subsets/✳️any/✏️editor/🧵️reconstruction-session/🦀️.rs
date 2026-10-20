//! 🏗️ Remodeling reconstruction tool run (`📋️tool-run-contract.md` §2.4, §3.7): the `ToolRunDefinition`
//! vocabulary and the run and revalidate jobs. The run job ingests the base document's frames, drives the
//! observed [`ReconstructionEngine`] and turns every pipeline decision into trace records and steps:
//! frame verdicts, per-candidate match verdicts (accepted = success, ratio test = warning, cross check =
//! danger), registered cameras and triangulated points as `instance3d` records the moment they appear,
//! pruned points turning danger, depth maps, dense points and the extracted mesh. Its products are
//! provisional `append-content` ops plus one `commit-reconstruction` op; nothing reaches the document before
//! finalize. Source of record: `🔣️.json` beside this file.

use semio_framework_job_publication::{RetainedJobPublication,JobPublicationKind};
use semio_s_artifact_remodel_remodeling_images::editor::remodeling::engine::images::BoundedDecodeProgress;
use semio_s_artifact_remodel_remodeling_images::editor::remodeling::engine::images::BoundedStillDecoder;
use semio_s_artifact_remodel_remodeling_images::editor::remodeling::engine::images::CompressedChunkRope;
use semio_s_artifact_remodel_remodeling_images::editor::remodeling::engine::images::ImageRgba8;
use semio_s_artifact_remodel_remodeling_reconstruct::editor::remodeling::engine::reconstruction::EngineObservation;
use semio_s_artifact_remodel_remodeling_reconstruct::editor::remodeling::engine::reconstruction::EngineStage;
use semio_s_artifact_remodel_remodeling_reconstruct::editor::remodeling::engine::reconstruction::EngineStatus;
use semio_s_artifact_remodel_remodeling_reconstruct::editor::remodeling::engine::reconstruction::FrameAcceptance;
use semio_s_artifact_remodel_remodeling_reconstruct::editor::remodeling::engine::reconstruction::ReconstructionEngine;
use semio_s_artifact_remodel_remodeling_reconstruct::editor::remodeling::engine::reconstruction::TerminalGeoPreparation;
use semio_s_artifact_remodel_remodeling_engine::editor::remodeling::engine::build_engine_params;
use semio_s_artifact_remodel_remodeling_engine::editor::remodeling::engine::camera_pose_preview;
use semio_s_artifact_remodel_remodeling_geo::editor::remodeling::engine::geo as remodeling_geo;
use semio_s_artifact_remodel_remodeling_engine::editor::remodeling::engine::watertight_snapshot;
use semio_s_artifact_remodel_remodeling_engine::editor::remodeling::engine::RasterPngPreparation;
use semio_s_artifact_remodel_remodeling_engine::editor::remodeling::engine::RasterPngProgress;
use semio_s_artifact_remodel_remodeling_mut_media::standards::v1::subsets::any::schema::mutations::append_content;
use semio_s_artifact_remodel_remodeling_mut_results::standards::v1::subsets::any::schema::mutations::commit_reconstruction::commit_reconstruction;
use semio_s_artifact_remodel_remodeling_mutations::standards::v1::subsets::any::schema::mutations::RemodelingMutation;
use semio_s_artifact_remodel_remodeling_mut_media::standards::v1::subsets::any::schema::mutations::append_content::AppendContent;
use semio_s_artifact_remodel_remodeling_mut_results::standards::v1::subsets::any::schema::mutations::commit_reconstruction::CommitReconstruction;
use semio_s_artifact_remodel_remodeling_mut_results::standards::v1::subsets::any::schema::mutations::commit_reconstruction::ReconstructionAssetCommit;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::mesh_is_within_resolution_envelope;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::remodeling_mesh_content_handle;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::CameraPosePreview;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::CameraTrajectory;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::FrameRef;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::GeoProducts;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::MeshSource;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::ByteBuffer;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::Float32Buffer;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::QcReportSnapshot;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingContentDigest;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingContentKind;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingMesh;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::SparseCloud;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::WatertightReportSnapshot;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::REMODELING_DURABLE_CHUNK_RAW_BYTES;
use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, StepContext};
use semio_framework_value::{retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}, ValueError};
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_tool_run::{
    JobKindId, ToolRunCounter, ToolRunCounterDefinition, ToolRunDefinition, ToolRunIdentity, ToolRunProgress, ToolRunReasonDefinition, ToolRunRebasePolicy, ToolRunReconfigurePolicy, ToolRunSettingsReads, ToolRunStageDefinition, ToolRunState, ToolRunStepArg, ToolRunStepKind, ToolRunStepRing,
    ToolRunTickWriter, ToolRunTraceKind, ToolRunTraceSubject, ToolRunVerdict, TOOL_RUN_REASON_CONFLICT,
};
use std::collections::{BTreeSet, VecDeque};
use std::sync::Arc;

#[path = "🧭️run-contract/🦀️.rs"]
mod run_contract;
pub use run_contract::*;

#[path = "🔑️trace-checkpoint-and-ingestion/🦀️.rs"]
mod trace_checkpoint_and_ingestion;
pub use trace_checkpoint_and_ingestion::*;

#[path = "🎁️products-and-outbox/🦀️.rs"]
mod products_and_outbox;
use products_and_outbox::*;

#[path = "🏃️run-job/🦀️.rs"]
mod run_job;
pub use run_job::*;

#[path = "🔍️revalidate-job/🦀️.rs"]
mod revalidate_job;
pub use revalidate_job::*;

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
