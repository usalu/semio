//! 💨️ transient `Puzzle2dScene` bundle its command/panel/window nodes mutate and render, the shared
//! 🧩️ Puzzle 2d play app — the plugin's 2d play app: its `ArtifactApp` impl (dispatch-only), the
//! snapshot helpers they build on, and the manifest that stitches those nodes together.
//!
//! 🧭️ Every behavioural arm lives in `🎮️commands/<group>/🦀️.rs`; every rendered surface in
//! `📌️panels/<panel>` or `🎭️modes/✏️edit/🪟️windows/<window>`. This file dispatches and stitches.
//!
//! 📸️ The native play root owns a canonical typed snapshot and a derived first-party host view.
//! Commands publish typed deltas; tool runs append their typed mutations to the operation ledger.

pub use semio_s_artifact_puzzle_2d_mut_any_10::editor::puzzle2d::snapshot;

pub use semio_s_artifact_puzzle_2d_text_inverse::editor::puzzle2d::recorder;

#[cfg(any(test, feature = "testkit"))]
#[path = "🧪️tests/🧰️testkit/🦀️.rs"]
pub mod testkit;

#[cfg(any(test, feature = "testkit"))]
pub use testkit::Puzzle2dApp as Puzzle2dTestApp;

use crate::editor::puzzle2d::terminology::puzzle2d_labels;
use crate::editor::puzzle2d::snapshot::puzzle2d_scene_for;
use crate::editor::puzzle2d::snapshot::puzzle2d_scene_with;
use crate::editor::puzzle2d::snapshot::PUZZLE2D_GENERIC_TOOL_IDS;
use crate::editor::puzzle2d::snapshot::PUZZLE2D_RETAINED_TOOL_IDS;
use crate::editor::puzzle2d::snapshot::PUZZLE2D_ADD_NODE_DIALOG_ID;
use crate::editor::puzzle2d::snapshot::PUZZLE2D_BOARD_SNAPSHOT_SCHEMA;
use crate::editor::puzzle2d::snapshot::PUZZLE2D_GRANULARITY_EDGE;
use crate::editor::puzzle2d::snapshot::PUZZLE2D_GRANULARITY_HANDLE;
use crate::editor::puzzle2d::snapshot::PUZZLE2D_GRANULARITY_NODE;
use crate::editor::puzzle2d::snapshot::PUZZLE2D_INTERACTION_DOMAIN;
use crate::editor::puzzle2d::snapshot::PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID;
use crate::editor::puzzle2d::snapshot::PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID;
use crate::editor::puzzle2d::snapshot::Puzzle2dInteractionSnapshot;
use crate::editor::puzzle2d::snapshot::Puzzle2dScene;
use crate::editor::puzzle2d::snapshot::puzzle2d_active_utility;
use crate::editor::puzzle2d::snapshot::puzzle2d_addresses_locked_entity;
use crate::editor::puzzle2d::snapshot::puzzle2d_entity_hidden;
use crate::editor::puzzle2d::snapshot::puzzle2d_entity_label;
use crate::editor::puzzle2d::snapshot::puzzle2d_interaction_definition;
use crate::editor::puzzle2d::snapshot::puzzle2d_node_kind_arg;
use crate::editor::puzzle2d::snapshot::puzzle2d_select_scope;
use crate::editor::puzzle2d::snapshot::puzzle2d_selected_node_ids;
use crate::editor::puzzle2d::snapshot::puzzle2d_selection_is_locked;
use crate::editor::puzzle2d::snapshot::puzzle2d_window_and_engagements_scope;
use crate::editor::puzzle2d::snapshot::puzzle2d_window_only_scope;
use crate::editor::puzzle2d::engine::board_host::{drain_board_events_json};
use crate::editor::puzzle2d::recorder::{puzzle2d_copy_fragment_from, puzzle2d_cut_operations_from, puzzle2d_paste_operations_on};
use crate::editor::puzzle2d::commands::set_active_example;
use crate::editor::puzzle2d::config::{Puzzle2dConfig, Puzzle2dConfigMutation};
use crate::editor::puzzle2d::engine::{BoardHost, Point};
use crate::editor::puzzle2d::modes::edit;
use crate::editor::puzzle2d::modes::edit::tools::fill;
use crate::editor::puzzle2d::precompute::fill as fill_run;
pub use crate::editor::puzzle2d::modes::edit::windows::overview::utilities::select::{puzzle2d_gesture_phase, puzzle2d_selection_pivot, Puzzle2dSelectionMotion, Puzzle2dSelectionRecord, SelectToolRequest};




pub use crate::editor::puzzle2d::terminology::puzzle2d_localized;
pub use crate::editor::puzzle2d::terminology::puzzle2d_localized_phrase;
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::ActionRef;
use semio_framework_plugin::AppIo;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactPresentation;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::DialogDefinition;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::InteractionRef;
use semio_framework_plugin::InteractionTarget;
use semio_framework_plugin::InteractionVerb;
use semio_framework_plugin::InteractiveJobClassification;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaPortDirection;
use semio_framework_plugin::MediaPortSpec;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::PortMultiplicity;
use semio_framework_plugin::ToolFactoryKey;
use semio_framework_plugin::ToolJobFactory;
use semio_framework_plugin::ToolJobFactoryError;
use semio_framework_plugin::WindowEngagement;
use semio_framework_plugin::WindowMeasure;
// 🕹️ `InteractionView` — see puzzle3d's identical import comment (missing top-level re-export from
// `semio_framework_plugin`, flagged to the coordinator, not fixed here).
use semio_framework_plugin::app::InteractionView;
use semio_framework_pack_json::{json, Value};
#[cfg(test)] use semio_framework_2d_compute::EngineHandles;
use semio_s_artifact_puzzle_2d_mut_any_10::editor::puzzle2d::snapshot::Puzzle2dCommand;
use semio_s_artifact_puzzle_2d_board_host::editor::puzzle2d::engine::board_host::puzzle_board_host;
//#region 🔖️Constants
//#endregion 🔖️Constants
//#region 🔖️Scene
//#endregion 🔖️Scene
//#region 🔖️BoardReads
//#region 🎯️TargetRegions
//#endregion 🎯️TargetRegions
//#endregion 🔖️BoardReads
//#region 📋️Clipboard
//#endregion 📋️Clipboard
//#region 🔖️BoardHostSync
//#endregion 🔖️BoardHostSync
//#region 🔖️UiScopes
//#endregion 🔖️UiScopes
//#region 🔖️Puzzle2dCommand
//#endregion 🔖️Puzzle2dCommand
//#region 🧵️ReservedJobs
use semio_framework_plugin::{ArtifactReservedToolInput, ArtifactReservedToolJob, ArtifactReservedToolJobRequest};
//#endregion 🧵️ReservedJobs

pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::interaction;
pub use interaction::*;

pub use semio_s_artifact_puzzle_2d_text_inverse::editor::puzzle2d::kit;
use kit::*;

pub use semio_s_artifact_puzzle_2d_retained::editor::puzzle2d::retained;
use retained::*;

#[path = "🖌️session/🦀️.rs"]
pub mod session;
use session::*;

#[path = "🔧️play-app-retained-commands-and-tool-proofs/🦀️.rs"]
mod play_app_retained_commands_and_tool_proofs;
pub use play_app_retained_commands_and_tool_proofs::Puzzle2dPlayApp;
use play_app_retained_commands_and_tool_proofs::{
    Puzzle2dHostConfigurationProofs, Puzzle2dRetainedCommandJobFactory,
    Puzzle2dRetainedCommandProofs, PUZZLE2D_COMMAND_RAW_BYTES, PUZZLE2D_HOST_ONLY_TOOL_IDS,
};

#[path = "🧠️artifact-editor/🦀️.rs"]
mod artifact_editor;

#[path = "🚀️app-assembly/🦀️.rs"]
mod app_assembly;
use app_assembly::{puzzle2d_fault_notices, puzzle2d_flag_value_argument};
pub use app_assembly::create_puzzle2d_app;

//#region 🔖️Manifest

// 🗂️ `Puzzle2dPlaySnapshot`'s pack<->dsl codec (so `framework/sync`'s `FolderEndpoint::Pack` can
// print/parse puzzle-2d play documents without depending on this crate's concrete
// `Projection`/`Mutation` types) is now declared via `.document_codec::<Puzzle2dPlayApp>()` on
// `crate::declaration()` (ticket `26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE`
// M1) — the old side-effecting `register_puzzle2d_exports()` wrapper (this app file's only caller of
// `register_document_codec_for_app`) is gone.
//#endregion 🔖️Manifest

//#region 🧪️UnitTests

//#endregion 🧪️UnitTests
