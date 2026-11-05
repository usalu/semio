//! 📸️ Remodeling editor — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch. `RemodelingPlayApp` is a unit struct; each former `RemodelingPlayRuntime` field
//! lives in the exact Model, Frames, or Report window config; every action dispatches
//! through the single typed `RemodelingCommand` channel via `ArtifactEditor::handle`.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window scenes in
//! `🎭️modes/*/🪟️windows/*`, panel trees in `📌️panels/*`, labels in `🦀️terminology.rs`, view state in
//! `🦀️config.rs`, the photogrammetry stack in this app's own `⚙️engine` topic files (relocated from the
//! artifact tree, 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES, #2553). This file is a
//! routing table: `handle` → `RemodelingCommand::dispatch`, `render` → body-key → node, and a
//! `🔖️Manifest` region that calls one `definition()` per node.

use semio_s_artifact_remodel_remodeling_images::editor::remodeling::engine::images as remodeling_image;
use semio_s_artifact_remodel_remodeling_editor_kit::editor::remodeling::kit::decode_still_image;
use semio_s_artifact_remodel_remodeling_modes::editor::remodeling::modes::analyze;
use semio_s_artifact_remodel_remodeling_modes::editor::remodeling::modes::capture;
use semio_s_artifact_remodel_remodeling_modes::editor::remodeling::modes::model;
use semio_s_artifact_remodel_remodeling_panels::editor::remodeling::panels::calibration as calibration_panel;
use semio_s_artifact_remodel_remodeling_panels::editor::remodeling::panels::document;
use semio_s_artifact_remodel_remodeling_panels::editor::remodeling::panels::media;
use semio_s_artifact_remodel_remodeling_panels::editor::remodeling::panels::parameters;
use semio_s_artifact_remodel_remodeling_panels::editor::remodeling::panels::quality;
use semio_s_artifact_remodel_remodeling_panels::editor::remodeling::panels::results;
use semio_s_artifact_remodel_remodeling_panels::editor::remodeling::panels::tracks;
use semio_s_artifact_remodel_remodeling_session::editor::remodeling::reconstruction_session::ReconstructionRevalidateJob;
use semio_s_artifact_remodel_remodeling_session::editor::remodeling::reconstruction_session::ReconstructionRunJob;
use semio_s_artifact_remodel_remodeling_editor_kit::editor::remodeling::terminology::remodeling_labels;
use semio_s_artifact_remodel_remodeling_modes::editor::remodeling::transient;
use semio_s_artifact_remodel_remodeling_modes::editor::remodeling::transient::RemodelingWindowTransient;
use semio_s_artifact_remodel_remodeling_mutations::standards::v1::subsets::any::schema::mutations::RemodelingMutation;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::FrameRef;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::ImageAsset;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::MediaKind;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::MediaStream;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::REMODELING_DOCUMENT_SCHEMA;
use semio_framework::{ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError};
use semio_plugin::app::InteractionView;
use semio_plugin::retained_command::{ArtifactCommandWork, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
use semio_plugin::ActionArgDef;
use semio_plugin::ActionArgOption;
use semio_plugin::ActionDefinition;
use semio_plugin::ActionKind;
use semio_plugin::AppDefinition;
use semio_plugin::AppIo;
use semio_plugin::AppOperationContext;
use semio_plugin::ArtifactEditor;
use semio_plugin::ArtifactOwnedToolJobRequest;
use semio_plugin::ArtifactToolFactoryRegistry;
use semio_plugin::ArtifactView;
use semio_plugin::ConfigView;
use {semio_framework_artifact_reference::Dialect};
use semio_plugin::DraftView;
use semio_plugin::Editor;
use semio_plugin::EditorApp;
use semio_plugin::Emit;
use semio_plugin::EphemeralEmit;
use semio_plugin::Fault;
use semio_plugin::FaultCode;
use semio_plugin::FaultOrigin;
use semio_plugin::mesh_io::GlbExporter;
use semio_plugin::GranularityDefinition;
use semio_plugin::HierarchyProvider;
use semio_plugin::HoverSpec;
use semio_plugin::InteractionDefinition;
use semio_plugin::InteractionRef;
use semio_plugin::InteractiveJobClassification;
use semio_ui_locale::Label;
use semio_ui_locale::LocalizedLabel;
use semio_plugin::Media;
use semio_plugin::MediaClass;
use semio_plugin::MediaError;
use semio_plugin::MediaForm;
use semio_plugin::MediaPayload;
use semio_plugin::MediaPortDirection;
use semio_plugin::MediaPortSpec;
use semio_plugin::MediaType;
use semio_plugin::MergeMode;
use semio_plugin::mesh_io::MeshExporter;
use semio_plugin::NoConfig;
use semio_plugin::NoConfigMutation;
use semio_plugin::NoDraft;
use semio_plugin::NoDraftMutation;
use semio_plugin::SelectionMethod;
use semio_plugin::SelectionMode;
use semio_plugin::SelectionSpec;
use semio_plugin::ToolRunJob;
use semio_plugin::ToolRunJobPurpose;
use semio_plugin::ToolRunJobRequest;
use semio_plugin::UtilityCategory;
use semio_plugin::UtilityDefinition;
use semio_plugin::WindowMeasure;
use std::collections::HashMap;
use pack::snapshot::ArtifactPack;
use semio_framework_2d_compute::EngineHandles;

#[path = "🔌️io-and-commands/🦀️.rs"]
mod io_and_commands;
pub use io_and_commands::*;

#[path = "🌉️action-bridge/🦀️.rs"]
mod action_bridge;
use action_bridge::*;

#[path = "🚧️retained-commands/🦀️.rs"]
mod retained_commands;
use retained_commands::*;

#[path = "🎛️artifact-editor/🦀️.rs"]
mod artifact_editor;

#[path = "📜️manifest/🦀️.rs"]
mod manifest;
pub use manifest::*;

//#region 🔖️Commands

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported here under its own flat name.
use semio_s_artifact_remodel_remodeling_cmd_workflow::editor::remodeling::commands::add_gcp;
use semio_s_artifact_remodel_remodeling_cmd_workflow::editor::remodeling::commands::calibrate_cameras;
use semio_s_artifact_remodel_remodeling_cmd_workflow::editor::remodeling::commands::edit_calibration;
use semio_s_artifact_remodel_remodeling_cmd_workflow::editor::remodeling::commands::place_gcp_observation;
use semio_s_artifact_remodel_remodeling_cmd_workflow::editor::remodeling::commands::remove_gcp;
use semio_s_artifact_remodel_remodeling_cmd_ingest::editor::remodeling::commands::add_stream;
use semio_s_artifact_remodel_remodeling_cmd_ingest::editor::remodeling::commands::import_abort;
use semio_s_artifact_remodel_remodeling_cmd_ingest::editor::remodeling::commands::import_frame_payload;
use semio_s_artifact_remodel_remodeling_cmd_ingest::editor::remodeling::commands::import_video_bytes_payload;
use semio_s_artifact_remodel_remodeling_cmd_ingest::editor::remodeling::commands::import_video_done;
use semio_s_artifact_remodel_remodeling_cmd_ingest::editor::remodeling::commands::import_video_frame_payload;
use semio_s_artifact_remodel_remodeling_cmd_ingest::editor::remodeling::commands::remove_stream;
use semio_s_artifact_remodel_remodeling_cmd_ingest::editor::remodeling::commands::set_stream_sync;
use semio_s_artifact_remodel_remodeling_cmd_workflow::editor::remodeling::commands::clear_dense;
use semio_s_artifact_remodel_remodeling_cmd_workflow::editor::remodeling::commands::clear_geo_products;
use semio_s_artifact_remodel_remodeling_cmd_workflow::editor::remodeling::commands::clear_mesh_result;
use semio_s_artifact_remodel_remodeling_cmd_workflow::editor::remodeling::commands::clear_result;
use semio_s_artifact_remodel_remodeling_cmd_workflow::editor::remodeling::commands::clear_sparse;
use semio_s_artifact_remodel_remodeling_cmd_workflow::editor::remodeling::commands::clear_tracks;
use semio_s_artifact_remodel_remodeling_cmd_workflow::editor::remodeling::commands::reset_placeholder_mesh;
use semio_s_artifact_remodel_remodeling_cmd_workflow::editor::remodeling::commands::export_qc_report;
use semio_s_artifact_remodel_remodeling_cmd_ingest::editor::remodeling::commands::import_frames;
use semio_s_artifact_remodel_remodeling_cmd_ingest::editor::remodeling::commands::import_video;
use semio_s_artifact_remodel_remodeling_cmd_ingest::editor::remodeling::commands::set_active_example;
use semio_s_artifact_remodel_remodeling_cmd_workflow::editor::remodeling::commands::set_camera;
use semio_s_artifact_remodel_remodeling_cmd_workflow::editor::remodeling::commands::set_frame_cursor;
use semio_s_artifact_remodel_remodeling_cmd_workflow::editor::remodeling::commands::set_layer_visibility;
use semio_s_artifact_remodel_remodeling_cmd_workflow::editor::remodeling::commands::set_report_table;
use semio_s_artifact_remodel_remodeling_cmd_workflow::editor::remodeling::commands::set_dense_params;
use semio_s_artifact_remodel_remodeling_cmd_workflow::editor::remodeling::commands::set_feature_params;
use semio_s_artifact_remodel_remodeling_cmd_workflow::editor::remodeling::commands::set_geo_params;
use semio_s_artifact_remodel_remodeling_cmd_workflow::editor::remodeling::commands::set_ingest_params;
use semio_s_artifact_remodel_remodeling_cmd_workflow::editor::remodeling::commands::set_match_params;
use semio_s_artifact_remodel_remodeling_cmd_workflow::editor::remodeling::commands::set_mesh_params;
use semio_s_artifact_remodel_remodeling_cmd_workflow::editor::remodeling::commands::set_motion_params;
use semio_s_artifact_remodel_remodeling_cmd_workflow::editor::remodeling::commands::set_sfm_params;
//#endregion 🔖️Commands

//#region 🔖️RemodelingPlayApp
/// 🧪️ Unit struct — concrete Model, Frames, and Report windows own their local view state.
#[derive(Default)]
pub struct RemodelingPlayApp;

//#endregion 🔖️RemodelingPlayApp

//#region 🧪️UnitTests
// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
// able to drive the whole app without re-deriving the harness.
//#endregion 🧪️UnitTests
